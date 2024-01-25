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
}

pub struct Token<'a> {
    pub kind: TokenKind,
    pub litteral: &'a str,
}

impl<'a> Token<'a> {
    fn new(kind:TokenKind, litteral:&'a str) -> Self {
        Self{kind, litteral}
    }
}