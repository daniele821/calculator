use fraction::Fraction;
use std::{env, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number(Fraction),
    /// - Operator between n expressions, and returns a Number.
    /// - Example: '-' 12 '+' (12 '/' 56)
    Operator(String),
    /// - Start of block defined by two characters.
    /// - Example: '(' expr )
    StartBlock(String),
    /// - End of block defined by two characters.
    /// - Example: ( expr ')'
    EndBlock(String),
    /// - Start/end of block defined by a single character
    /// - Example: '|' expr '|'
    UnaryBlock(String),
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
                _ => return Err(err_msg),
            },
            Token::EndBlock(str) => match &str[..] {
                ")" => {
                    let expected_token = &Token::StartBlock(String::from("("));
                    let actual_token = stack.pop().ok_or(err_msg.clone())?;
                    (expected_token == actual_token).then_some(err_msg.clone());
                }
                _ => return Err(err_msg),
            },
            Token::UnaryBlock(str) => match &str[..] {
                "|" => {
                    let stack_top = stack.last();
                    if stack_top == Some(&&Token::UnaryBlock(String::from(str))) {
                        stack.pop();
                    } else {
                        stack.push(token);
                    }
                }
                _ => return Err(err_msg),
            },
            _ => (),
        };
    }
    stack.is_empty().then_some(()).ok_or(err_msg)
}

fn parse_token(chars: &[char]) -> Result<(Token, &[char]), String> {
    let char = chars.first().ok_or("cannot parse empty string!")?;
    match char {
        '*' | '+' | '%' | '/' | '-' | '^' => Ok((Token::Operator(char.to_string()), &chars[1..])),
        '(' => Ok((Token::StartBlock(char.to_string()), &chars[1..])),
        ')' => Ok((Token::EndBlock(char.to_string()), &chars[1..])),
        '|' => Ok((Token::UnaryBlock(char.to_string()), &chars[1..])),
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

fn token_priority(token: Token) -> Option<usize> {
    match token {
        Token::StartBlock(_) | Token::EndBlock(_) | Token::UnaryBlock(_) => Some(0),
        Token::Operator(str) => match &str[..] {
            "^" => Some(1),
            "*" | "/" | "%" => Some(2),
            "+" | "-" => Some(3),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let expr = String::from("(|-12 ^ 2 * (34 + 69)| - (6 / 2)) % 2");
        let expected_expr_tokens = vec![
            Token::StartBlock(String::from("(")),
            Token::UnaryBlock(String::from("|")),
            Token::Operator(String::from("-")),
            Token::Number(Fraction::from(12)),
            Token::Operator(String::from("^")),
            Token::Number(Fraction::from(2)),
            Token::Operator(String::from("*")),
            Token::StartBlock(String::from("(")),
            Token::Number(Fraction::from(34)),
            Token::Operator(String::from("+")),
            Token::Number(Fraction::from(69)),
            Token::EndBlock(String::from(")")),
            Token::UnaryBlock(String::from("|")),
            Token::Operator(String::from("-")),
            Token::StartBlock(String::from("(")),
            Token::Number(Fraction::from(6)),
            Token::Operator(String::from("/")),
            Token::Number(Fraction::from(2)),
            Token::EndBlock(String::from(")")),
            Token::EndBlock(String::from(")")),
            Token::Operator(String::from("%")),
            Token::Number(Fraction::from(2)),
        ];
        let actual_expr_tokens = super::parse_tokens(&expr).unwrap();
        expected_expr_tokens
            .iter()
            .enumerate()
            .for_each(|(i, t)| assert_eq!(t, actual_expr_tokens.get(i).unwrap(), "at index {i}"));
        assert_eq!(expected_expr_tokens, actual_expr_tokens);

        let wrong_expr1 = String::from("(12.34.54 + 12)");
        let wrong_expr2 = String::from("(12.34'54 + 12)");
        assert!(super::parse_tokens(&wrong_expr1).is_err());
        assert!(super::parse_tokens(&wrong_expr2).is_err());
    }

    #[test]
    fn check_blocks() {
        let tokens_valid_1 = &super::parse_tokens("12.56").unwrap();
        let tokens_valid_2 = &super::parse_tokens("(12 + 34)").unwrap();
        let tokens_valid_3 = &super::parse_tokens("|(|-4| * |5|)(6)|").unwrap();
        let tokens_invalid_1 = &super::parse_tokens("(()").unwrap();
        let tokens_invalid_2 = &super::parse_tokens("())").unwrap();
        let tokens_invalid_3 = &super::parse_tokens(")(").unwrap();
        let tokens_invalid_4 = &super::parse_tokens("(|)|()").unwrap();

        assert!(super::check_blocks(tokens_valid_1).is_ok());
        assert!(super::check_blocks(tokens_valid_2).is_ok());
        assert!(super::check_blocks(tokens_valid_3).is_ok());
        assert!(super::check_blocks(tokens_invalid_1).is_err());
        assert!(super::check_blocks(tokens_invalid_2).is_err());
        assert!(super::check_blocks(tokens_invalid_3).is_err());
        assert!(super::check_blocks(tokens_invalid_4).is_err());
    }
}
