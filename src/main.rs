mod lexer;

use lexer::Lexer;
use lox_rs::{lexer::*, ast::*};

use std::{env, io::Result, fs::*};
use rustyline::{error::ReadlineError, DefaultEditor};
use colored::Colorize;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        run_prompt();
    } else {
        let files: Vec<&String> = args.iter().filter(|arg| arg.ends_with("lox")).collect();
        if files.len() == 0 {
            // unreachable!("Please enter valid path!")
            run_prompt();
        } else if files.len() == 1 {
            run_file(files[0]);
        } else {
            unreachable!("Please enter single file only!")
        }
    }
}

pub fn run_prompt() -> Result<()>  {
    let mut reader = DefaultEditor::new().unwrap();
    loop {
        let line = reader.readline_with_initial("> ", ("", ""));
        match line {
            Ok(line) => {
                run(&line);
                
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "CTRL-C".cyan().dimmed());
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

pub fn run_file(path: &str) -> Result<()> {
    let source = std::fs::read_to_string(path).unwrap();
    run(&source);
    Ok(())
}

pub fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    println!("{:?}", tokens);
}
