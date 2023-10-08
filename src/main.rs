#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub mod ast;
pub mod ast_impl;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod stmt;
pub mod tests;

use std::path::Path;

use parser::Parser;

use crate::{ast::LoxError, interpreter::Interpreter};

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("Usage: rlox [file.lox]");
        std::process::exit(0);
    }

    let path = args.nth(1).expect("Failed to get script file");

    if path == "-" {
        run_repl().expect("REPL Crashed");
        return;
    }

    if !Path::new(&path).exists() {
        eprintln!("Cannot find {}\nexiting.", &path);
        return;
    }

    if let Err(LoxError::ExitCode(n)) = run_file(path) {
        std::process::exit(n);
    }
}
fn run_repl() -> Result<(), LoxError> {
    let mut parser = Parser::new();
    let mut interpreter = Interpreter::new();

    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line from stdin");
        line = line.trim().to_string();
        match parser.load(line) {
            Ok(expr) => {
                interpreter.interpret(expr);
            }
            Err(err) => {
                eprintln!("> {}", err);
                break;
            }
        }
    }
    Ok(())
}

fn run_file(path: String) -> Result<(), LoxError> {
    let mut interpreter = Interpreter::new();
    let mut parser = Parser::new();

    if parser.scanner.had_error {
        return Err(LoxError::ExitCode(65));
    }

    match parser.load_file(path) {
        Ok(expr) => {
            interpreter.interpret(expr);
        }
        Err(err) => {
            eprintln!("[line: {}] Error while parsing: {:#?}", parser.line, &err);
            return Err(err);
        }
    }

    return Ok(());
}
