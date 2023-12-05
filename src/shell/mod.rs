use rustyline::{error::ReadlineError, DefaultEditor};

use crate::expression::solver;

pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match &line[..] {
                    "exit" => break,
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
