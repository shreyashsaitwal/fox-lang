pub mod scanner;

use std::{
    fs,
    io::Result,
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
};

use scanner::Scanner;

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
        run(line);
    }
}

fn run(source: String) -> Result<()> {
    let scanner = Scanner::new(&source);
    for i in scanner {
        dbg!(i);
    }
    Ok(())
}

fn error(line: u32, message: String) {
    report(line, "".to_owned(), message);
}

fn report(line: u32, place: String, message: String) {
    eprintln!("[line {line}] Error {place}: {message}");
}
