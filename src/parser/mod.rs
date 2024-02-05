use std::iter::Peekable;

use crate::lexer::token;

use super::lexer::token::{Token, TokenKind};

pub trait Parsed<'a> {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Self;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Program {
    pub function_declaration: Function,
}

impl<'a> Parsed<'a> for Program {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        let mut prog = Program::default();
        prog.function_declaration = Function::parse(tokens);
        prog
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
}

impl<'a> Parsed<'a> for Function {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        let mut function = Function::default();
        match tokens.next() {
            None => {
                panic!("Cannot parse function : no more tokens")
            }
            Some(token) => {
                Self::parse_tokens(token, tokens, &mut function);
            }
        }
        function
    }
}

impl Function {
    fn parse_tokens<'a>(
        current_token: &Token,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        function: &mut Self,
    ) {
        match current_token.kind {
            TokenKind::Int => {}
            _ => {
                panic!("function return type is wrong")
            }
        }
        
        if tokens.peek().is_some() && tokens.peek().unwrap().kind == TokenKind::Identifier {
            function.name = tokens.next().unwrap().litteral.clone();
        } else {
            panic!("function name expected")
        }

        // handle correctly those ones
        assert_eq!(tokens.next().unwrap(), &Token{kind:TokenKind::OpenParen, litteral:String::from("(")});
        assert_eq!(tokens.next().unwrap(), &Token{kind:TokenKind::CloseParen, litteral:String::from(")")});

        if let Some(token) = tokens.next() {
            if token.kind != TokenKind::OpenBrace {
                panic!("Open brace expected before the function body")
            }
        } else {
            panic!("No more token, expected an open brace.")
        }
        while tokens.peek().is_some() && tokens.peek().unwrap().kind != TokenKind::CloseBrace {
            function.statements.push(Statement::parse(tokens));
        }

        match tokens.next() {
            None => {
                panic!("curly braces expected at the end of the function, no more tokens")
            },
            Some(token) => {
                if token.kind != TokenKind::CloseBrace {panic!("curly braces expected at the end of the function")}
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Statement {
    pub _return: Option<Expr>,
}

impl<'a> Parsed<'a> for Statement {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        let mut statement = Self::default();
        match tokens.next() {
            None => {
                panic!("Cannot parse statement : no more tokens")
            }
            Some(token) => {
                Self::parse_tokens(token, tokens, &mut statement);
            }
        }
        statement
    }
}

impl Statement {
    fn parse_tokens<'a>(
        current_token: &Token,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        statement: &mut Self,
    ) {
        match current_token.kind {
            TokenKind::Return => {
                statement._return = Some(Expr::parse(tokens));
                assert!(tokens.peek().is_some());
                assert_eq!(tokens.next().unwrap().kind, TokenKind::Semicolon);
            }
            _ => panic!("the statement is anything other than return"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Expr {
    pub constant: Option<i32>,
    pub string: Option<String>,
}

impl<'a> Parsed<'a> for Expr {
    fn parse(tokens: &mut Peekable<impl Iterator<Item = &'a Token>>) -> Self {
        let mut expression = Self::default();
        match tokens.next() {
            None => {
                panic!("Cannot parse expression : no more tokens")
            }
            Some(token) => {
                Self::parse_tokens(token, tokens, &mut expression);
            }
        }
        expression
    }
}

impl Expr {
    fn parse_tokens<'a>(
        current_token: &Token,
        tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
        expression: &mut Self,
    ) {
        match current_token.kind {
            TokenKind::Constant => {
                expression.constant = Some(current_token.litteral.parse().unwrap());
            }
            _ => panic!("the expression is anything other than a contant"),
        }
    }
}
