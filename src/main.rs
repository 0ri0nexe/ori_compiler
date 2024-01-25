mod lexer;

use std::{env::{self}, fs::canonicalize, path::PathBuf, process::exit};

struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Cli {
        pattern: args[1],
        path: PathBuf::from(args[2]), 
    };

    let file_path = canonicalize(&args[1]).unwrap();
    dbg!(&file_path);
    
}
