use ori::{
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
    parser::*,
};

#[test]
fn lexing() {
    let tokens = Lexer::lex(String::from("int main() {\nreturn 5 \n}"));
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
    dbg!(&program);
    let statements = vec![Statement {
        _return: Some(Expr {
            constant: Some(5),
            string: None,
        }),
    }];

    let main_func = Function {
        name: String::from("main"),
        statements,
    };

    assert_eq!(
        program,
        Program {
            function_declaration: main_func,
        }
    )
}

fn parse(string: &str) {
    Program::parse(&mut Lexer::lex(String::from(string)).iter().peekable());
}

#[test]
fn valid_parsing() {
    parse(
        r#"
    int main() {
        return 100;
    }
    "#,
    );

    parse(
        r#"
    int 
    main
    (   
    )
    {
    return
    0
    ;
    }
    "#,
    );

    parse(
        r#"
    int main(){return 0;}
    "#,
    );

    parse(
        r#"
    int main() {
        return 0;
    }
    "#,
    );

    parse(
        r#"
    int main() {
        return 2;
    }
    "#,
    );

    parse(
        r#"
            int   main    (  )  {   return  0 ; }
    "#,
    );
}

#[test]
#[should_panic]
fn invalid_parsing() {
    parse(
        "int main( {
        return 0;
    }",
    )
}

#[test]
#[should_panic]
fn invalid_parsing2() {
    parse(
        "int main() {
        return;
    }",
    )
}

#[test]
#[should_panic]
fn invalid_parsing3() {
    parse(
        "int main() {
        return 0;",
    )
}

#[test]
#[should_panic]
fn invalid_parsing4() {
    parse(
        "int main() {
        return 0
    }",
    )
}

#[test]
#[should_panic]
fn invalid_parsing5() {
    parse(
        "int main() {
        return0;
    }",
    )
}

#[test]
#[should_panic]
fn invalid_parsing6() {
    parse(
        "int main() {
            RETURN 0;
        }",
    )
}
