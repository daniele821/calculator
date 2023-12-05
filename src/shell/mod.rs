use crate::expression::solver;
use std::io::{stdin, stdout, Write};

pub fn run() {
    let mut input = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).unwrap();
        if !input.contains('\n') {
            println!();
        }

        let mut args = input.split_whitespace();
        if let Some(str) = args.next() {
            match str {
                "exit" => break,
                _ => {
                    println!();
                    match solver::resolve(&input, &[], &[], true) {
                        Ok(res) => println!("\nSolution: {res}"),
                        Err(err) => println!("\n{err}"),
                    };
                    println!();
                }
            }
        };

        input.clear();
    }
}
