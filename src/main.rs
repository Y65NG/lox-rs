use lox_rs::{ast::*, interpreter::*, lexer::*, parser::*};

use colored::Colorize;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{env, fs::*, io::Result};
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

pub fn run_prompt() -> Result<()> {
    let mut reader = DefaultEditor::new().unwrap();
    let interpreter = Interpreter::new();
    loop {
        let line = reader.readline_with_initial("> ", ("", ""));
        match line {
            Ok(line) => {
                run(&line, &interpreter, true);
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
    let interpreter = Interpreter::new();
    run(&source, &interpreter, false);
    Ok(())
}

pub fn run(source: &str, interpreter: &Interpreter, is_repl: bool) {

    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens, is_repl);
    // let expr = parser.parse();
    let stmts = parser.parse();

    match stmts {
        Ok(ref stmts) => {
            interpreter.interpret(stmts);
        }
        Err(e) => eprintln!("{}", e),
    };
}
