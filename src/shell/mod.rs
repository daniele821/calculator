use std::process::Command;

use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{
    common::{self, Color},
    expression::solver::{self, FixRules},
};

pub fn run() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(&common::color(&Color::OTH, ">>> "));
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match &line[..] {
                    "" => continue,
                    "exit" => break,
                    "clear" => {
                        Command::new("clear").spawn().unwrap().wait().unwrap();
                    }
                    "help" => println!("{}", help()),
                    _ => match solver::resolve(&line, &FixRules::all(), &[], true) {
                        Ok(res) => {
                            let title = common::color(&Color::TIT, "Solution:");
                            let res = common::color(&Color::SUC, &res);
                            println!("{title} {res}\n");
                        }
                        Err(err) => {
                            let title = common::color(&Color::TIT, "Error:");
                            let err = common::color(&Color::FAI, &err);
                            println!("{title} {err}\n");
                        }
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
    format!(
        "{}
  - {}  => close shell
  - {} => clear terminal
  - {}  => show this help message
  - {}     => parse as an expression
",
        common::color(&Color::TIT, "Commands:"),
        common::color(&Color::SUB, "exit"),
        common::color(&Color::SUB, "clear"),
        common::color(&Color::SUB, "help"),
        common::color(&Color::SUB, "*")
    )
}
