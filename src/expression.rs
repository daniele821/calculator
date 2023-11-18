#![allow(dead_code, unused)]

use fraction::Fraction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Module,
    Elevation,
    StartPriorityBlock,
    EndPriorityBlock,
    Number(Fraction),
}

pub fn parse_tokens(expr: &str) -> Result<Vec<Token>, &'static str> {
    let mut res = Vec::new();
    let mut str = expr;
    while !str.is_empty() {
        res.push(parse_token(str)?);
    }
    Ok(res)
}

fn parse_token(mut str: &str) -> Result<Token, &'static str> {
    let mut chars = str.chars().enumerate();
    match chars.next().map(|(i, c)| c) {
        None => panic!("string to be parsed is empty!"),
        Some(char) => match char {
            '*' => Token::Multiplication,
            '+' => Token::Addition,
            '0'..='9' => todo!(),
            _ => return Err("\"{c}\" is not a valid token!"),
        },
    };
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{Token::*, *};
    use std::str::FromStr;

    #[test]
    fn parse_tokens() {
        let expr = String::from("(+-12) (%10.10) ^^/*()    12.12)");
        let expected_expr_tokens = Ok(vec![
            StartPriorityBlock,
            Addition,
            Subtraction,
            Number(Fraction::from(12)),
            EndPriorityBlock,
            StartPriorityBlock,
            Module,
            Number(Fraction::from_str("10.10").unwrap()),
            EndPriorityBlock,
            Elevation,
            Elevation,
            Division,
            Multiplication,
            StartPriorityBlock,
            EndPriorityBlock,
            Number(Fraction::from_str("12.12").unwrap()),
        ]);
        let actual_expr_tokens = super::parse_tokens(&expr);
        assert_eq!(actual_expr_tokens, expected_expr_tokens);

        let wrong_expr1 = String::from("(12.34.54 + 12)");
        let wrong_expr2 = String::from("(12.34'54 + 12)");
        assert!(super::parse_tokens(&wrong_expr1).is_err());
        assert!(super::parse_tokens(&wrong_expr2).is_err());
    }
}
