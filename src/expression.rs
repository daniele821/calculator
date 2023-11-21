use fraction::Fraction;
use std::{env, str::FromStr};

pub fn run() -> Result<Fraction, String> {
    let args = collect_args();
    let tokens = parse_tokens(&args)?;
    check_blocks(&tokens)?;
    Err(String::from("TODO!"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number,
    Operator,
    StartBlock,
    EndBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenValue {
    token: Token,
    value: String,
}

impl From<(Token, &str)> for TokenValue {
    fn from(value: (Token, &str)) -> Self {
        Self {
            token: value.0,
            value: String::from(value.1),
        }
    }
}

fn collect_args() -> String {
    env::args().skip(1).map(|i| i + " ").collect::<String>()
}

fn parse_tokens(expr: &str) -> Result<Vec<TokenValue>, String> {
    let mut res = Vec::new();
    let mut chars_slice = &(expr.chars().collect::<Vec<char>>())[..];
    while !chars_slice.is_empty() {
        let (token, char_slice_tmp) = parse_token(chars_slice)?;
        chars_slice = skip_whitespaces(char_slice_tmp);
        res.push(token);
    }
    Ok(res)
}

fn check_blocks(tokens: &[TokenValue]) -> Result<(), String> {
    let err_msg = String::from("blocks are not balanced");
    let mut stack = Vec::<&TokenValue>::new();
    for token in tokens {
        match token.token {
            Token::StartBlock => match token.value.as_str() {
                "(" => stack.push(token),
                _ => return Err(err_msg),
            },
            Token::EndBlock => match token.value.as_str() {
                ")" => {
                    let expected_token = &TokenValue::from((Token::StartBlock, ")"));
                    let actual_token = stack.pop().ok_or(err_msg.clone())?;
                    (expected_token == actual_token).then_some(err_msg.clone());
                }
                _ => return Err(err_msg),
            },
            _ => (),
        };
    }
    stack.is_empty().then_some(()).ok_or(err_msg)
}

fn parse_token(chars: &[char]) -> Result<(TokenValue, &[char]), String> {
    let char = chars.first().ok_or("cannot parse empty string!")?;
    let str = &char.to_string()[..];
    match char {
        '*' | '+' | '%' | '/' | '-' | '^' => {
            Ok((TokenValue::from((Token::Operator, str)), &chars[1..]))
        }
        '(' => Ok((TokenValue::from((Token::StartBlock, str)), &chars[1..])),
        ')' => Ok((TokenValue::from((Token::EndBlock, str)), &chars[1..])),
        '0'..='9' => parse_number(chars),
        char => Err(format!("'{char}' is not a valid token!")),
    }
}

fn parse_number(chars: &[char]) -> Result<(TokenValue, &[char]), String> {
    let num_str = chars
        .iter()
        .take_while(|c| c.is_ascii_digit() || c == &&'.')
        .collect::<String>();
    let num_len = num_str.chars().count();
    let err_msg = format!("{num_str} in not a valid number!");
    let _ = Fraction::from_str(&num_str).or(Err(err_msg))?;
    let token = TokenValue::from((Token::Number, num_str.as_str()));
    Ok((token, &chars[num_len..]))
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
    use super::*;

    #[test]
    fn parse_tokens() {
        let expr = "12*3-(7)";
        let expected_expr_tokens = vec![
            TokenValue::from((Token::Number, "12")),
            TokenValue::from((Token::Operator, "*")),
            TokenValue::from((Token::Number, "3")),
            TokenValue::from((Token::Operator, "-")),
            TokenValue::from((Token::StartBlock, "(")),
            TokenValue::from((Token::Number, "7")),
            TokenValue::from((Token::EndBlock, ")")),
        ];
        let actual_expr_tokens = super::parse_tokens(expr).unwrap();
        assert_eq!(expected_expr_tokens, actual_expr_tokens);
    }

    #[test]
    fn check_blocks() {
        let tokens_valid = &super::parse_tokens("(()())").unwrap();
        let tokens_invalid = &super::parse_tokens(")()()").unwrap();
        assert!(super::check_blocks(tokens_valid).is_ok());
        assert!(super::check_blocks(tokens_invalid).is_err());
    }
}
