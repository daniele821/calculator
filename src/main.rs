use std::env;

use calculator::{
    common,
    expression::{
        error::Error,
        solver::{self, FixRules},
    },
};

fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let mut tokens = solver::parse(&args, &FixRules::all(), &[])?;
    println!("{}", common::fmt(&tokens, None));
    while solver::solve_one_op(&mut tokens)? {
        println!("{}", common::fmt(&tokens, None));
    }
    solver::get_result(&tokens)?;
    Ok(())
}
