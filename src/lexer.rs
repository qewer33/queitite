use std::str::Chars;

use crate::token::Token;

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
            let token = self.scan_char();
            if let Some(token) = token {
                dbg!(&token);
                tokens.push(token);
            }

            if self.is_at_end() {
                break
            }

            self.next();
        }

        tokens
    }

    fn scan_char(&mut self) -> Option<Token> {
        let c = self.current();

        let token = match c {
            // types
            '"' => {
                let str = self.consume_until('"');
                Some(Token::Str(str))
            }
            // assign
            '=' => {
                if self.consume('=') {
                    return Some(Token::Equals);
                }

                Some(Token::Assign)
            },
            // arithmetic
            '+' => {
                if self.consume('=') {
                    return Some(Token::AddAssign);
                } else if self.consume('+') {
                    return Some(Token::Incr);
                }

                Some(Token::Add)
            },
            '-' => {
                if self.consume('=') {
                    return Some(Token::SubAssign);
                } else if self.consume('-') {
                    return Some(Token::Decr);
                }

                Some(Token::Sub)
            },
            '*' => {
                if self.consume('*') {
                    return Some(Token::Pow);
                }

                Some(Token::Mult)
            },
            '/' => {
                Some(Token::Div)
            },
            // bool ops
            '<' => {
                if self.consume('=') {
                    return Some(Token::LesserEquals);
                }

                Some(Token::Lesser)
            },
            '>' => {
                if self.consume('=') {
                    return Some(Token::GreaterEquals);
                }

                Some(Token::Greater)
            },
            '!' => {
                if self.consume('=') {
                    return Some(Token::NotEquals);
                }

                Some(Token::Not)
            },
            // other
            '#' => {
                let comment = self.consume_until('\n');
                Some(Token::Comment(comment))
            }
            ' ' => None,
            _ => {
                // check types
                if let Some(bool) = self.check_bool() {
                    return Some(Token::Bool(bool));
                }

                if let Some(num) = self.check_num() {
                    return Some(Token::Num(num));
                }

                // assume keyword
                let keyword = self.consume_until(' ');
                Some(Token::Keyword(keyword))
            }
        };

        token
    }

    // type checks

    fn check_bool(&mut self) -> Option<bool> {
        if self.consume_str("true".to_string()) {
            return Some(true);
        } else if self.consume_str("false".to_string()) {
            return Some(false);
        }
        None
    }

    fn check_num(&mut self) -> Option<String> {
        if self.current().is_numeric() {
            let num = self.consume_until(' ');
            return Some(num);
        }
        None
    }

    // iter utils

    fn current(&self) -> char {
        self.src[self.curr]
    }

    fn next(&mut self) -> char {
        if self.is_at_end() {
            return ' ';
        }

        self.curr += 1;
        self.src[self.curr]
    }

    fn peek(&self) -> char {
         if self.is_at_end() {
            return ' ';
        }

        self.src[self.curr+1]
    }

    fn check(&self, c: char) -> bool {
        c == self.src[self.curr]
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

    fn consume_str(&mut self, str: String) -> bool {
        let slice: &String = &self.src[self.curr..(self.curr+str.len())].iter().clone().collect();

        if str.eq(slice) {
            self.curr += str.len();
            return true;
        }
        false
    } 

    fn consume_until(&mut self, c: char) -> String {
        let mut out = String::new();

        while self.current() != c {
            out.push(self.current());
            self.next();
        }

        out
    }

    fn is_at_end(&self) -> bool {
        self.curr+1 == self.src.len()
    }
}