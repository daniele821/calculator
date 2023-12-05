use std::env;

use calculator::{
    common,
    expression::{
        error::Error,
        solver::{self, CheckRules, FixRules},
    },
};
use fraction::Fraction;

fn main() {
    let res = run();
    match res {
        Ok(res) => println!("Solution: {res}"),
        Err(err) => println!("{err}"),
    }
}

fn run() -> Result<Fraction, Error> {
    println!("\nExplanation:");
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut tokens = solver::parse(&args, &FixRules::all(), &CheckRules::all())?;
    println!("{}", common::fmt(&tokens, None));
    while solver::solve_one_op(&mut tokens)? {
        println!("{}\n", common::fmt(&tokens, None));
    }
    solver::get_result(&tokens)
}
