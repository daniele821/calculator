#![allow(dead_code, unused)]

use fraction::Fraction;
use std::{env, str::FromStr};

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

pub fn run() -> Result<Fraction, String> {
    let args = collect_args();
    let tokens = parse_tokens(&args)?;
    all_checks_on_tokens(&tokens)?;
    todo!();
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

pub fn all_checks_on_tokens(tokens: &[Token]) -> Result<(), String> {
    check_blocks(tokens)?;
    Ok(())
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

fn check_blocks(tokens: &[Token]) -> Result<(), String> {
    let err_msg = String::from("blocks are not balanced");
    let mut stack = Vec::<Token>::new();
    for token in tokens {
        match token {
            Token::StartPriorityBlock => stack.push(Token::StartPriorityBlock),
            Token::EndPriorityBlock => {
                stack.pop().ok_or(err_msg.clone())?;
            }
            _ => (),
        };
    }
    if !stack.is_empty() {
        return Err(err_msg);
    }
    Ok(())
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
            EndPriorityBlock,
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
        let tokens_valid_2: &[Token] = &[Token::StartPriorityBlock, Token::EndPriorityBlock];
        let tokens_valid_3: &[Token] = &[
            Token::StartPriorityBlock,
            Token::EndPriorityBlock,
            Token::StartPriorityBlock,
            Token::EndPriorityBlock,
        ];
        let tokens_invalid_1: &[Token] = &[
            Token::StartPriorityBlock,
            Token::EndPriorityBlock,
            Token::EndPriorityBlock,
        ];
        let tokens_invalid_2: &[Token] = &[
            Token::StartPriorityBlock,
            Token::StartPriorityBlock,
            Token::EndPriorityBlock,
        ];
        let tokens_invalid_3: &[Token] = &[Token::EndPriorityBlock, Token::StartPriorityBlock];

        assert!(super::check_blocks(tokens_valid_1).is_ok());
        assert!(super::check_blocks(tokens_valid_2).is_ok());
        assert!(super::check_blocks(tokens_valid_3).is_ok());
        assert!(super::check_blocks(tokens_invalid_1).is_err());
        assert!(super::check_blocks(tokens_invalid_2).is_err());
        assert!(super::check_blocks(tokens_invalid_3).is_err());
    }
}
