#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Int,
    Return,
    
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    
    Constant, // a constant like the 2 in `int 2`

    Identifier,
    ToDetermine,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub litteral: String,
}

impl Token {
    pub fn new() -> Self {
        Self{kind:TokenKind::ToDetermine, litteral:String::new()}
    }
    pub fn from_char(kind:TokenKind, litteral_char:&char) -> Self {
        Self{kind, litteral:litteral_char.to_string()}
    }
    pub fn from_string(kind: TokenKind, litteral_string: String) -> Self {
        Self{kind, litteral:litteral_string}
    }
}