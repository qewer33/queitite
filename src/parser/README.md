# Parser

### Grammar

```js
program        → statement* EOF ;

declaration    → funDecl
               | varDecl
               | statement ;

funDecl        → "fn" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
varDecl        → IDENTIFIER ( "=" expression )? EOL ;

statement      → exprStmt
               | ifStmt
               | forStmt
               | printStmt
               | returnStmt
               | whileStmt
               | block ;

exprStmt       → expression EOL ;
ifStmt         → "if" expression statement
               ( "else" statement )? ;
forStmt        → "for" ( varDecl | exprStmt | "and" )
                 expression? "and"
                 expression? statement ;
printStmt      → "print" expression EOL ;
returnStmt     → "return" expression EOL ;
whileStmt      → "while" "(" expression ")" statement ;
block          → "do" declaration "end" ;

expression     → assignment ;
assignment     → IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" | "**" | "??" ) unary )* ;
unary          → ( "!" | "-" ) unary | call ;
arguments      → expression ( "," expression )* ;
call           → primary ( "(" arguments? ")" )* ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")"
               | IDENTIFIER ;
```