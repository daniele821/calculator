use fraction::Fraction;
use std::{env, str::FromStr};

pub fn run() -> Result<Fraction, Err> {
    let args = collect_args();
    let tokens = parse_tokens(&args)?;
    check_expressions(&tokens)?;
    todo!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Err {
    Parsing(ParseErr),
    Expression(ExprErr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErr {
    InvalidToken(String),
    InvalidNumber(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprErr {
    NoResult,
    InvalidToken(TokenValue),
    UnbalancedBlocks,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenValue {
    token: Token,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Number,
    UnaryOperator,
    BinaryOperator,
    StartBlock,
    EndBlock,
}

impl From<ParseErr> for Err {
    fn from(value: ParseErr) -> Self {
        Self::Parsing(value)
    }
}

impl From<ExprErr> for Err {
    fn from(value: ExprErr) -> Self {
        Self::Expression(value)
    }
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

fn parse_tokens(expr: &str) -> Result<Vec<TokenValue>, Err> {
    let mut res = Vec::new();
    let mut chars_slice = &(expr.chars().collect::<Vec<char>>())[..];
    while !chars_slice.is_empty() {
        let (token, char_slice_tmp) = parse_token(chars_slice, &res)?;
        chars_slice = skip_whitespaces(char_slice_tmp);
        res.push(token);
    }
    Ok(res)
}

fn parse_token<'a>(
    chars: &'a [char],
    prev: &[TokenValue],
) -> Result<(TokenValue, &'a [char]), Err> {
    let char = chars.first().expect("cannot parse an empty string!");
    let str = &char.to_string()[..];
    let last = prev.last();
    Ok(match char {
        '-' => match last.map(|t| &t.token) {
            Some(Token::Number) => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
            Some(Token::EndBlock) => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
            _ => (TokenValue::from((Token::UnaryOperator, str)), &chars[1..]),
        },
        '+' => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
        '*' => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
        '/' => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
        '%' => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
        '^' => (TokenValue::from((Token::BinaryOperator, str)), &chars[1..]),
        '(' => (TokenValue::from((Token::StartBlock, str)), &chars[1..]),
        ')' => (TokenValue::from((Token::EndBlock, str)), &chars[1..]),
        '0'..='9' => parse_number(chars)?,
        _ => Err(ParseErr::InvalidToken(str.to_string()))?,
    })
}

fn parse_number(chars: &[char]) -> Result<(TokenValue, &[char]), Err> {
    let num_str = chars
        .iter()
        .take_while(|c| c.is_ascii_digit() || c == &&'.')
        .collect::<String>();
    Fraction::from_str(&num_str).or(Err(ParseErr::InvalidNumber(num_str.clone())))?;
    let num_len = num_str.chars().count();
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

fn check_expressions(tokens: &[TokenValue]) -> Result<(), Err> {
    let mut expr_stack = Vec::<&Token>::new();
    let mut block_stack = Vec::<TokenValue>::new();
    for token in tokens {
        expr_stack.push(&token.token);
        while simplify_expression(&mut expr_stack) {}
        check_block(&mut block_stack, token)?;
    }
    (expr_stack.len() == 1 && expr_stack.get(0) == Some(&&Token::Number))
        .then_some(())
        .ok_or(Err::Expression(ExprErr::NoResult))
}

fn simplify_expression(stack: &mut Vec<&Token>) -> bool {
    let len = stack.len();
    if len >= 3
        && stack.get(len - 3) == Some(&&Token::Number)
        && stack.get(len - 2) == Some(&&Token::BinaryOperator)
        && stack.get(len - 1) == Some(&&Token::Number)
    {
        stack.remove(len - 1);
        stack.remove(len - 2);
        return true;
    }
    if len == 2
        && stack.get(len - 2) == Some(&&Token::UnaryOperator)
        && stack.get(len - 1) == Some(&&Token::Number)
    {
        stack.remove(len - 2);
        return true;
    }
    if len >= 3
        && stack.get(len - 3) == Some(&&Token::StartBlock)
        && stack.get(len - 2) == Some(&&Token::UnaryOperator)
        && stack.get(len - 1) == Some(&&Token::Number)
    {
        stack.remove(len - 2);
        return true;
    }
    if len >= 4
        && stack.get(len - 4) == Some(&&Token::Number)
        && stack.get(len - 3) == Some(&&Token::BinaryOperator)
        && stack.get(len - 2) == Some(&&Token::UnaryOperator)
        && stack.get(len - 1) == Some(&&Token::Number)
    {
        stack.remove(len - 1);
        stack.remove(len - 2);
        stack.remove(len - 3);
        return true;
    }
    if len >= 3
        && stack.get(len - 3) == Some(&&Token::StartBlock)
        && stack.get(len - 2) == Some(&&Token::Number)
        && stack.get(len - 1) == Some(&&Token::EndBlock)
    {
        stack.remove(len - 1);
        stack.remove(len - 3);
        return true;
    }
    false
}

fn check_block(stack: &mut Vec<TokenValue>, token: &TokenValue) -> Result<(), Err> {
    match token.token {
        Token::StartBlock => match token.value.as_str() {
            "(" => stack.push(token.clone()),
            _ => Err(ExprErr::InvalidToken(token.clone()))?,
        },
        Token::EndBlock => match token.value.as_str() {
            ")" => {
                let expected_token = &TokenValue::from((Token::StartBlock, "("));
                let actual_token = &stack.pop().ok_or(ExprErr::UnbalancedBlocks)?;
                let equals = expected_token == actual_token;
                equals.then_some(()).ok_or(ExprErr::UnbalancedBlocks)?;
            }
            _ => Err(ExprErr::InvalidToken(token.clone()))?,
        },
        _ => (),
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tokens() {
        let expr = "12*3-(-7)";
        let expected_expr_tokens = vec![
            TokenValue::from((Token::Number, "12")),
            TokenValue::from((Token::BinaryOperator, "*")),
            TokenValue::from((Token::Number, "3")),
            TokenValue::from((Token::BinaryOperator, "-")),
            TokenValue::from((Token::StartBlock, "(")),
            TokenValue::from((Token::UnaryOperator, "-")),
            TokenValue::from((Token::Number, "7")),
            TokenValue::from((Token::EndBlock, ")")),
        ];
        let actual_expr_tokens = super::parse_tokens(expr).unwrap();
        assert_eq!(expected_expr_tokens, actual_expr_tokens);
    }

    #[test]
    fn check_expressions() {
        let expression_valid1 = &super::parse_tokens("1 * 2 + 4").unwrap();
        let expression_valid2 = &super::parse_tokens("-1 * 2 + -4").unwrap();
        let expression_valid3 = &super::parse_tokens("1 + (12 * (2 + 3) + (3 + 4) + 3)").unwrap();
        let expression_invalid1 = &super::parse_tokens("-1 * 2 + --4").unwrap();
        let expression_invalid2 = &super::parse_tokens("( ) * )").unwrap();
        assert!(super::check_expressions(expression_valid1).is_ok());
        assert!(super::check_expressions(expression_valid2).is_ok());
        assert!(super::check_expressions(expression_valid3).is_ok());
        assert!(super::check_expressions(expression_invalid1).is_err());
        assert!(super::check_expressions(expression_invalid2).is_err());
    }
}
