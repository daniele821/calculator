#![allow(dead_code, unused)]

use calculator::expression;
use fraction::Fraction;

fn main() {
    let result = expression::run();
    match result {
        Ok(res) => println!("result of calculation is: '{res}'"),
        Err(msg) => println!("calculation failed: '{msg:?}'"),
    }
}
