use ori::{
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
    parser::*,
};

#[test]
fn lexing() {
    let mut tokens = Lexer::lex(String::from("int main() {\nreturn 5 \n}"));
    assert_eq!(
        tokens,
        vec![
            Token {
                kind: TokenKind::Int,
                litteral: String::from("int")
            },
            Token {
                kind: TokenKind::Identifier,
                litteral: String::from("main")
            },
            Token {
                kind: TokenKind::OpenParen,
                litteral: String::from("(")
            },
            Token {
                kind: TokenKind::CloseParen,
                litteral: String::from(")")
            },
            Token {
                kind: TokenKind::OpenBrace,
                litteral: String::from("{")
            },
            Token {
                kind: TokenKind::Return,
                litteral: String::from("return")
            },
            Token {
                kind: TokenKind::Constant,
                litteral: String::from("5")
            },
            Token {
                kind: TokenKind::CloseBrace,
                litteral: String::from("}")
            },
        ]
    );
}

#[test]
fn parsing() {
    let tokens = Lexer::lex(String::from("int main() {\nreturn 5; \n}"));
    let program = Program::parse(&mut tokens.iter().peekable());
    
    let statements = vec![
        Statement{
            _return: Some(Expr { constant: Some(5), string: None })
        }
    ];

    let main_func = Function {
        name: String::from("main"),
        statements
    };

    assert_eq!(program, 
        Program {
            function_declaration: main_func,
        }
    )
}