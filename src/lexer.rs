use crate::token::{Token, KEYWORDS};

pub struct Lexer {
    src: Vec<char>,
    curr: usize
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.chars().collect(),
            curr: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if self.is_at_end() {
                break
            }
            
            let token = self.scan_char();

            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens.push(Token::EOF);
        tokens
    }

    fn scan_char(&mut self) -> Option<Token> {
        let c = self.current();

        let token = match c {
            // types
            '"' => {
                self.next();
                let str = self.consume_until('"');
                self.next();
                self.next();
                Some(Token::Str(str))
            }
            // assign
            '=' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::Equals);
                }

                self.next();
                Some(Token::Assign)
            },
            // arithmetic
            '+' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::AddAssign);
                } else if self.consume('+') {
                    self.next();
                    return Some(Token::Incr);
                }

                self.next();
                Some(Token::Add)
            },
            '-' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::SubAssign);
                } else if self.consume('-') {
                    self.next();
                    return Some(Token::Decr);
                }

                self.next();
                Some(Token::Sub)
            },
            '*' => {
                if self.consume('*') {
                    self.next();
                    return Some(Token::Pow);
                }

                self.next();
                Some(Token::Mult)
            },

            '/' => {
                self.next();
                Some(Token::Div)
            },
            // bool ops
            '<' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::LesserEquals);
                }
                
                self.next();
                Some(Token::Lesser)
            },
            '>' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::GreaterEquals);
                }

                self.next();
                Some(Token::Greater)
            },
            '!' => {
                if self.consume('=') {
                    self.next();
                    return Some(Token::NotEquals);
                }

                self.next();
                Some(Token::Not)
            },
            // other
            '\n' => {
                self.next();
                Some(Token::EOL)
            },
            '(' => {
                self.next();
                Some(Token::LParen)
            },
            ')' => {
                self.next();
                Some(Token::RParen)
            },
            '#' => {
                self.next();
                let comment = self.consume_until('\n');
                self.next();
                None
            },
            ',' => {
                self.next();
                Some(Token::Comma)
            },
            ' ' => {
                self.next();
                None
            },
            _ => {
                // check types
                if let Some(bool) = self.check_bool() {
                    self.next();
                    return Some(Token::Bool(bool));
                }

                if let Some(num) = self.check_num() {
                    self.next();
                    return Some(Token::Num(num));
                }

                // checks keywords, assume identifiers if it doesn't match any
                let mut str = String::new();
                
                loop {
                    str.push(self.current());

                    if !self.peek().is_alphanumeric() {
                        break;
                    }
                    self.next();
                }

                self.next();
                if KEYWORDS.contains(&str.as_str()) {
                    return Some(Token::Keyword(str));
                }
                Some(Token::Identifier(str))
            }
        };

        token
    }

    // type checks

    fn check_bool(&mut self) -> Option<bool> {
        if self.consume_str("true") {
            return Some(true);
        } else if self.consume_str("false") {
            return Some(false);
        }
        None
    }

    fn check_num(&mut self) -> Option<String> {
        if self.current().is_numeric() {
            let mut num = String::new();

            loop {
                num.push(self.current());

                if !self.peek().is_numeric() {
                    break;
                }
                self.next();
            }

            return Some(num);
        }
        None
    }

    // iter utils

    fn current(&self) -> char {
        self.src[self.curr]
    }

    fn next(&mut self) -> char {
        self.curr += 1;

        if self.is_at_end() {
            return ' ';
        }

        self.src[self.curr]
    }

    fn peek(&self) -> char {
         if self.is_at_end() {
            return ' ';
        }

        self.src[self.curr+1]
    }

    fn consume(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if c == self.src[self.curr+1] {
            self.next();
            return true;
        }
        false
    }

    fn consume_str(&mut self, s: &str) -> bool {
        let len = s.len();

        if self.curr + len > self.src.len() {
            return false;
        }

        let slice: String = self.src[self.curr..self.curr + len].iter().collect();
        if slice == s {
            self.curr += len;
            return true;
        }

        false
    }

    fn consume_until(&mut self, c: char) -> String {
        let mut out = String::new();

        loop {
            out.push(self.current());

            if self.peek() == c {
                break;
            }
            self.next();
        }

        out
    }

    fn is_at_end(&self) -> bool {
        self.curr == self.src.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokens(src: &str) -> Vec<Token> {
        let mut lx = Lexer::new(src.to_string());
        lx.tokenize()
    }

    #[test]
    fn empty_input() {
        assert_eq!(tokens(""), vec![Token::EOF]);
    }

    #[test]
    fn simple_assign() {
        assert_eq!(
            tokens("a = 10\n"),
            vec![
                Token::Identifier("a".into()),
                Token::Assign,
                Token::Num("10".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn call_with_arg() {
        assert_eq!(
            tokens("print(a)\n"),
            vec![
                Token::Identifier("print".into()),
                Token::LParen,
                Token::Identifier("a".into()),
                Token::RParen,
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn function_def_line() {
        assert_eq!(
            tokens("sq(n) = n*n\n"),
            vec![
                Token::Identifier("sq".into()),
                Token::LParen,
                Token::Identifier("n".into()),
                Token::RParen,
                Token::Assign,
                Token::Identifier("n".into()),
                Token::Mult,
                Token::Identifier("n".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn if_block_skeleton() {
        assert_eq!(
            tokens("if a == 100 do\nend\n"),
            vec![
                Token::Keyword("if".into()),
                Token::Identifier("a".into()),
                Token::Equals,
                Token::Num("100".into()),
                Token::Keyword("do".into()),
                Token::EOL,
                Token::Keyword("end".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn string_literal() {
        assert_eq!(
            tokens("print(\"poggers\")\n"),
            vec![
                Token::Identifier("print".into()),
                Token::LParen,
                Token::Str("poggers".into()),
                Token::RParen,
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn two_char_ops() {
        assert_eq!(
            tokens("a!=b\nc>=d\ne<=f\n"),
            vec![
                Token::Identifier("a".into()),
                Token::NotEquals,
                Token::Identifier("b".into()),
                Token::EOL,
                Token::Identifier("c".into()),
                Token::GreaterEquals,
                Token::Identifier("d".into()),
                Token::EOL,
                Token::Identifier("e".into()),
                Token::LesserEquals,
                Token::Identifier("f".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn blank_lines() {
        assert_eq!(tokens("\n\n"), vec![Token::EOL, Token::EOL, Token::EOF]);
    }

    #[test]
    fn string_at_eof() {
        assert_eq!(
            tokens("print(\"x\")"),
            vec![
                Token::Identifier("print".into()),
                Token::LParen,
                Token::Str("x".into()),
                Token::RParen,
                Token::EOF
            ]
        );
    }

    #[test]
    fn comment_then_identifier() {
        // Assumes you EMIT a Comment token and then an EOL after it.
        assert_eq!(
            tokens("# this is a comment\nx\n"),
            vec![
                Token::EOL,
                Token::Identifier("x".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }

    #[test]
    fn keywords_vs_identifiers() {
        assert_eq!(
            tokens("do end if print dox\n"),
            vec![
                Token::Keyword("do".into()),
                Token::Keyword("end".into()),
                Token::Keyword("if".into()),
                Token::Identifier("print".into()),
                Token::Identifier("dox".into()),
                Token::EOL,
                Token::EOF
            ]
        );
    }
}