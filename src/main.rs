use lox_rs::{interpreter::*, lexer::*, parser::*};

use colored::Colorize;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{env, io::Result};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        let _ = run_prompt();
    } else {
        let files: Vec<&String> = args.iter().filter(|arg| arg.ends_with("lox")).collect();
        if files.len() == 0 {
            let _ = run_prompt();
        } else if files.len() == 1 {
            let _ = run_file(files[0]);
        } else {
            unreachable!("Please enter single file only!")
        }
    }
}

pub fn run_prompt() -> Result<()> {
    let mut reader = DefaultEditor::new().unwrap();
    let mut interpreter = Interpreter::new();
    loop {
        let line = reader.readline_with_initial("> ", ("", ""));
        match line {
            Ok(line) => {
                run(&line, &mut interpreter, true);
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
    let mut interpreter = Interpreter::new();
    run(&source, &mut interpreter, false);
    Ok(())
}

pub fn run(source: &str, interpreter: &mut Interpreter, is_repl: bool) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let parser = Parser::new(tokens, is_repl);
    let stmts = parser.parse();

    match stmts {
        Ok(ref stmts) => {
            interpreter.interpret(stmts);
        }
        Err(e) => eprintln!("{}", e),
    };
}
