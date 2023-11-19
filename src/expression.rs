// #![allow(dead_code, unused)]

use fraction::Fraction;
use std::{env, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(Fraction),
    BinaryOperator(String),
    StartBlock(String),
    EndBlock(String),
}

pub fn run() -> Result<Fraction, String> {
    let args = collect_args();
    let tokens = parse_tokens(&args)?;
    check_blocks(&tokens)?;
    Err(String::from("TODO!"))
}

pub fn collect_args() -> String {
    env::args().skip(1).map(|i| i + " ").collect::<String>()
}

pub fn parse_tokens(expr: &str) -> Result<Vec<Token>, String> {
    let mut res = Vec::new();
    let mut chars_slice = &(expr.chars().collect::<Vec<char>>())[..];
    while !chars_slice.is_empty() {
        let (token, char_slice_tmp) = parse_token(chars_slice)?;
        chars_slice = skip_whitespaces(char_slice_tmp);
        res.push(token);
    }
    Ok(res)
}

pub fn check_blocks(tokens: &[Token]) -> Result<(), String> {
    let err_msg = String::from("blocks are not balanced");
    let mut stack = Vec::<&Token>::new();
    for token in tokens {
        match token {
            Token::StartBlock(str) => match &str[..] {
                "(" => stack.push(token),
                _ => Err(err_msg.clone())?,
            },
            Token::EndBlock(str) => match &str[..] {
                ")" => {
                    let expected_token = &Token::StartBlock(String::from("("));
                    let actual_token = stack.pop().ok_or(err_msg.clone())?;
                    (expected_token == actual_token).then_some(err_msg.clone());
                }
                _ => Err(err_msg.clone())?,
            },
            _ => Err(err_msg.clone())?,
        };
    }
    stack.is_empty().then_some(()).ok_or(err_msg)
}

fn parse_token(chars: &[char]) -> Result<(Token, &[char]), String> {
    let char = chars.first().ok_or("cannot parse empty string!")?;
    match char {
        '*' | '+' | '%' | '/' | '-' | '^' => {
            Ok((Token::BinaryOperator(char.to_string()), &chars[1..]))
        }
        '(' => Ok((Token::StartBlock(char.to_string()), &chars[1..])),
        ')' => Ok((Token::EndBlock(char.to_string()), &chars[1..])),
        '0'..='9' => parse_number(chars),
        char => Err(format!("'{char}' is not a valid token!")),
    }
}

fn parse_number(chars: &[char]) -> Result<(Token, &[char]), String> {
    let num_str = chars
        .iter()
        .take_while(|c| c.is_ascii_digit() || c == &&'.')
        .collect::<String>();
    let num_len = num_str.chars().count();
    let err_msg = format!("{num_str} in not a valid number!");
    let number = Fraction::from_str(&num_str).or(Err(err_msg))?;
    Ok((Token::Number(number), &chars[num_len..]))
}

fn skip_whitespaces(chars: &[char]) -> &[char] {
    for i in 0..chars.len() {
        if !chars[i].is_whitespace() {
            return &chars[i..];
        }
    }
    &[]
}

#[cfg(test)]
mod tests {
    use super::{Token::*, *};
    use std::str::FromStr;

    #[test]
    fn parse_tokens() {
        let expr = String::from("(+-12) (%10.10) ^^/*()    12.12)");
        let expected_expr_tokens = Ok(vec![
            StartBlock(String::from("(")),
            BinaryOperator(String::from("+")),
            BinaryOperator(String::from("-")),
            Number(Fraction::from(12)),
            EndBlock(String::from(")")),
            StartBlock(String::from("(")),
            BinaryOperator(String::from("%")),
            Number(Fraction::from_str("10.10").unwrap()),
            EndBlock(String::from(")")),
            BinaryOperator(String::from("^")),
            BinaryOperator(String::from("^")),
            BinaryOperator(String::from("/")),
            BinaryOperator(String::from("*")),
            StartBlock(String::from("(")),
            EndBlock(String::from(")")),
            Number(Fraction::from_str("12.12").unwrap()),
            EndBlock(String::from(")")),
        ]);
        let actual_expr_tokens = super::parse_tokens(&expr);
        assert_eq!(actual_expr_tokens, expected_expr_tokens);

        let wrong_expr1 = String::from("(12.34.54 + 12)");
        let wrong_expr2 = String::from("(12.34'54 + 12)");
        assert!(super::parse_tokens(&wrong_expr1).is_err());
        assert!(super::parse_tokens(&wrong_expr2).is_err());
    }

    #[test]
    fn check_blocks() {
        let tokens_valid_1: &[Token] = &[];
        let tokens_valid_2: &[Token] = &[
            Token::StartBlock(String::from("(")),
            Token::EndBlock(String::from(")")),
        ];
        let tokens_valid_3: &[Token] = &[
            Token::StartBlock(String::from("(")),
            Token::EndBlock(String::from(")")),
            Token::StartBlock(String::from("(")),
            Token::EndBlock(String::from(")")),
        ];
        let tokens_invalid_1: &[Token] = &[
            Token::StartBlock(String::from("(")),
            Token::EndBlock(String::from(")")),
            Token::EndBlock(String::from(")")),
        ];
        let tokens_invalid_2: &[Token] = &[
            Token::StartBlock(String::from("(")),
            Token::StartBlock(String::from("(")),
            Token::EndBlock(String::from(")")),
        ];
        let tokens_invalid_3: &[Token] = &[
            Token::EndBlock(String::from(")")),
            Token::StartBlock(String::from("(")),
        ];

        assert!(super::check_blocks(tokens_valid_1).is_ok());
        assert!(super::check_blocks(tokens_valid_2).is_ok());
        assert!(super::check_blocks(tokens_valid_3).is_ok());
        assert!(super::check_blocks(tokens_invalid_1).is_err());
        assert!(super::check_blocks(tokens_invalid_2).is_err());
        assert!(super::check_blocks(tokens_invalid_3).is_err());
    }
}
