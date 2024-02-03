pub mod token;

use std::iter::Peekable;

use self::token::{Token, TokenKind};

pub struct Lexer {}

impl Lexer {
    pub fn lex(file: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut file_iter = file.chars().peekable();
        while file_iter.peek().is_some() {
            let mut current_token = Token::new();
            let current_char = file_iter.next().unwrap();
            match current_char {
                '{' => tokens.push(Token::from_char(TokenKind::OpenBrace, &current_char)),
                '}' => tokens.push(Token::from_char(TokenKind::CloseBrace, &current_char)),
                '(' => tokens.push(Token::from_char(TokenKind::OpenParen, &current_char)),
                ')' => tokens.push(Token::from_char(TokenKind::CloseParen, &current_char)),
                ';' => tokens.push(Token::from_char(TokenKind::Semicolon, &current_char)),
                'A'..='Z' | 'a'..='z' | '_' => {
                    Self::get_fullword(current_char, &mut file_iter, &mut current_token);
                    tokens.push(current_token);
                },
                '0'..='9' => {
                    Self::get_const(current_char, &mut file_iter, &mut current_token);
                    tokens.push(current_token);
                }
                _ => {}
            }
        }
        tokens
    }

    fn get_fullword(
        current_char: char,
        iter: &mut Peekable<impl Iterator<Item = char>>,
        current_token: &mut Token,
    ) {
        current_token.litteral.push(current_char);
        while iter.peek().is_some() {
            if iter.peek().unwrap().is_alphanumeric() || *iter.peek().unwrap() == '_' {
                let current_char = iter.next().unwrap();
                current_token.litteral.push(current_char);
                match current_token.litteral.as_str() {
                    "int" => {
                        current_token.kind = TokenKind::Int;
                        return;
                    }
                    "return" => {
                        current_token.kind = TokenKind::Return;
                        return;
                    }
                    _ => {}
                }
            } else {
                current_token.kind = TokenKind::Identifier;
                return;
            }
        }
    }

    fn get_const(
        current_char: char,
        iter: &mut Peekable<impl Iterator<Item = char>>,
        current_token: &mut Token,
    ) {
        current_token.kind = TokenKind::Constant;
        current_token.litteral.push(current_char);
        while iter.peek().is_some() {
            if iter.peek().unwrap().is_ascii_digit() {
                let current_char = iter.next().unwrap();
                current_token.litteral.push(current_char);
            } else {
                return
            }
        }
    }
}
