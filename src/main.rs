#![allow(dead_code, unused)]

use calculator::{expression, expression::Token};

fn main() {
    let i = [' ', '\t', '\n', '1', ' '];
    dbg!(&expression::skip_whitespaces(&i)[1..]);
    dbg!(expression::skip_whitespaces(
        &expression::skip_whitespaces(&i)[1..]
    ));
}
