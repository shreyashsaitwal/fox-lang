#![feature(if_let_guard)]

mod errors;
mod lexer;
mod expr;

use miette::Report;
use std::{
    fs,
    io::Result,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
};

use lexer::Lexer;

pub fn run_file(path: PathBuf) {
    let source = fs::read_to_string(path).unwrap();
    if let Err(_) = run(source) {
        exit(64);
    }
}

pub fn run_prompt() {
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        let _ = stdin().read_line(&mut line);
        let _ = run(line);
    }
}

fn run(source: String) -> Result<()> {
    let lexer = Lexer::new(&source);
    for i in lexer {
        if let Err(e) = i {
            eprintln!("{:?}", Report::new(e));
        } else if let Ok(t) = i {
            println!("{t:?}");
        }
    }
    Ok(())
}
