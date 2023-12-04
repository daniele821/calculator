use std::env;

use calculator::expression::{
    error::Error,
    solver::{self, FixRules},
};

fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut tokens = solver::parse(&args, &FixRules::all(), &[])?;
    let result = solver::solve(&mut tokens)?;
    println!("Solution: {result}");
    Ok(())
}
