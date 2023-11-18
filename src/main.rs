#![allow(dead_code, unused)]

use std::str::FromStr;

use calculator::{expression, expression::Token};
use fraction::{error::ParseError, Fraction};

fn main() {
    let i = [' ', '\t', '\n', '1', ' '];
    dbg!(&expression::skip_whitespaces(&i)[1..]);
    dbg!(expression::skip_whitespaces(
        &expression::skip_whitespaces(&i)[1..]
    ));
    dbg!(Fraction::from_str(&String::from(".12.2")).or(Err("")));
}
