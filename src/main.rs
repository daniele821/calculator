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
    let mut expl = String::new();
    let res = run(&mut expl);
    println!("\nExplanation:\n{expl}");
    match res {
        Ok(res) => println!("Solution: {res}"),
        Err(err) => println!("{err}"),
    }
}

fn run(expl: &mut String) -> Result<Fraction, Error> {
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut tokens = solver::parse(&args, &FixRules::all(), &CheckRules::all())?;
    expl.push_str(&format!("{}\n", common::fmt(&tokens, None)));
    while solver::solve_one_op(&mut tokens)? {
        expl.push_str(&format!("{}\n", common::fmt(&tokens, None)));
    }
    solver::get_result(&tokens)
}
