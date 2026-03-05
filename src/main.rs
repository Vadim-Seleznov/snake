mod scanner;
use crate::scanner::*;

use std::env;
use std::process::exit;
use std::fs;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String> {
    if let Ok(contents) = fs::read_to_string(path) {
        run(&contents)
    } else {
        Err("Could not read a file!".to_string())
    }
}

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{token:?}");
    }

    Ok(())
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("Could not flush stdout!".to_string())
        }
    
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
    
        match handle.read_line(&mut buffer) {
                Ok(_) => (),
                Err(_) => return Err("Could not parse line!".to_string()),
        };

        if buffer == "exit\n" || buffer == "exit" {
            return Ok(());
        }
    
    
        println!("ECHO: {buffer}");
        if let Err(msg) = run(&buffer) {
            return Err(msg);
        } else {
            continue;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: snake [script]");
        exit(64);
    } else if args_len == 2 {

        if let Err(msg) = run_file(&args[1]) {
            println!("ERROR: {msg}");
            exit(1);
        } else {
            ()
        }

    } else {
        if let Ok(_) = run_prompt() {
            exit(0);
        } else {
            println!("Error while running prompt!");
            exit(1);
        }
    }

    dbg!(args);
}
