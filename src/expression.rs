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

pub fn parse_tokens(expr: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::new();
    let mut chars: Vec<char> = expr.chars().collect();
    let chars_slice = &chars[..];
    while chars_slice.is_empty() {
        let (token, chars_slice) = parse_token(chars_slice)?;
        res.push(token);
    }
    Ok(res)
}

fn parse_token(chars: &[char]) -> Result<(Token, &[char]), String> {
    let char = chars.first().ok_or("cannot parse empty string!")?;
    match char {
        '*' => Ok((Token::Multiplication, &chars[1..])),
        '+' => Ok((Token::Addition, &chars[1..])),
        '%' => Ok((Token::Module, &chars[1..])),
        '/' => Ok((Token::Division, &chars[1..])),
        '-' => Ok((Token::Subtraction, &chars[1..])),
        '^' => Ok((Token::Elevation, &chars[1..])),
        '(' => Ok((Token::StartPriorityBlock, &chars[1..])),
        ')' => Ok((Token::EndPriorityBlock, &chars[1..])),
        char => Err(format!("\"{char}\" is not a valid token!")),
    }
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
