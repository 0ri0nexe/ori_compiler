use ori::lexer::Lexer;

use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path).expect("could not read the file");
    let tokens = Lexer::lex(content);
    
    dbg!(&tokens);
    
}
