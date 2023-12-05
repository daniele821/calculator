use std::process::Command;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::expression::solver;

pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("\x1b[92m>>>\x1b[0m ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match &line[..] {
                    "exit" => break,
                    "clear" => {
                        Command::new("clear").spawn().unwrap().wait().unwrap();
                    }
                    "help" => println!("{}", help()),
                    _ => match solver::resolve(&line, &[], &[], true) {
                        Ok(res) => println!("\nSolution: {res}\n"),
                        Err(err) => println!("\n{err}\n"),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn help() -> String {
    String::from(
        "
    Commands:
    - exit      => close shell
    - clear     => clear terminal
    - help      => show this help message
    - *         => parse as an expression
",
    )
}
