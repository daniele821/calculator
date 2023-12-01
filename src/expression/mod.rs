use fraction::Fraction;
use std::{env, str::FromStr};

// ---------- PUBLIC ITEMS ----------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Err {
    IllegalState,
    InvalidToken(String),
    InvalidNumber(String),
    InvalidExpression,
    UnbalancedBlocks,
}

pub fn run() -> Result<Fraction, Err> {
    run_args(&collect_args())
}

pub fn run_args(expr: &str) -> Result<Fraction, Err> {
    let tokens = parse_tokens(expr)?;
    check_expressions(&tokens)?;
    solve_expr(tokens)
}

// ---------- PRIVATE ITEMS ----------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number,
    UnaryOperator,
    BinaryOperator,
    StartBlock,
    EndBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenValue {
    token: Token,
    value: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenNum {
    token: TokenValue,
    num: Option<Fraction>,
}

impl From<(Token, &str)> for TokenValue {
    fn from(value: (Token, &str)) -> Self {
        Self {
            token: value.0,
            value: String::from(value.1),
        }
    }
}

impl TokenNum {
    fn from_token(token: &TokenValue) -> Result<Self, Err> {
        let token = token.clone();
        let num: Option<Fraction> = if token.token == Token::Number {
            let err = Err(Err::InvalidNumber(token.value.clone()));
            Some(Fraction::from_str(&token.value).or(err)?)
        } else {
            None
        };
        Ok(Self { token, num })
    }
}

fn collect_args() -> String {
    env::args().skip(1).map(|i| i + " ").collect::<String>()
}

fn parse_tokens(expr: &str) -> Result<Vec<TokenValue>, Err> {
    let mut res = Vec::new();
    let chars_slice = &(expr.chars().collect::<Vec<char>>())[..];
    let mut chars_slice = skip_whitespaces(chars_slice);
    let mut block_stack = Vec::new();
    while !chars_slice.is_empty() {
        let (token, char_slice_tmp) = parse_token(chars_slice, &res, &mut block_stack)?;
        chars_slice = skip_whitespaces(char_slice_tmp);
        res.push(token);
    }
    Ok(res)
}

fn parse_token<'a>(
    chars: &'a [char],
    prev: &[TokenValue],
    block_stack: &mut Vec<String>,
) -> Result<(TokenValue, &'a [char]), Err> {
    let char = chars.first().ok_or(Err::IllegalState)?;
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
        '(' => {
            block_stack.push(char.to_string());
            (TokenValue::from((Token::StartBlock, str)), &chars[1..])
        }
        ')' => {
            if let Some(str) = block_stack.last() {
                if str != "(" {
                    return Err(Err::UnbalancedBlocks);
                }
            }
            block_stack.pop();
            (TokenValue::from((Token::EndBlock, str)), &chars[1..])
        }
        '|' => match block_stack.last().map(|s| s.as_str()) {
            Some("|") => {
                block_stack.pop();
                (TokenValue::from((Token::EndBlock, str)), &chars[1..])
            }
            _ => {
                block_stack.push(char.to_string());
                (TokenValue::from((Token::StartBlock, str)), &chars[1..])
            }
        },
        '0'..='9' => parse_number(chars)?,
        _ => Err(Err::InvalidToken(str.to_string()))?,
    })
}

fn parse_number(chars: &[char]) -> Result<(TokenValue, &[char]), Err> {
    let num_str = chars
        .iter()
        .take_while(|c| c.is_ascii_digit() || c == &&'.')
        .collect::<String>();
    Fraction::from_str(&num_str).or(Err(Err::InvalidNumber(num_str.clone())))?;
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
        while collapse_expression(&mut expr_stack) {}
        check_block(&mut block_stack, token)?;
    }
    (expr_stack.len() == 1 && expr_stack.get(0) == Some(&&Token::Number))
        .then_some(())
        .ok_or(Err::InvalidExpression)
}

fn collapse_expression(stack: &mut Vec<&Token>) -> bool {
    let len = stack.len();
    if len >= 3
        && stack.get(len - 3) == Some(&&Token::Number)
        && stack.get(len - 2) == Some(&&Token::BinaryOperator)
        && stack.get(len - 1) == Some(&&Token::Number)
    {
        stack.drain(len - 2..=len - 1);
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
        stack.drain(len - 3..=len - 1);
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
            "(" | "|" => stack.push(token.clone()),
            _ => Err(Err::InvalidToken(token.value.clone()))?,
        },
        Token::EndBlock => match token.value.as_str() {
            ")" | "|" => {
                let expected_block = matching_block(token.value.as_str())?;
                let expected_token = &TokenValue::from((Token::StartBlock, expected_block));
                let actual_token = &stack.pop().ok_or(Err::UnbalancedBlocks)?;
                let equals = expected_token == actual_token;
                equals.then_some(()).ok_or(Err::UnbalancedBlocks)?;
            }
            _ => Err(Err::InvalidToken(token.value.clone()))?,
        },
        _ => (),
    };
    Ok(())
}

fn matching_block(block: &str) -> Result<&str, Err> {
    Ok(match block {
        "(" => ")",
        ")" => "(",
        "|" => "|",
        _ => Err(Err::IllegalState)?,
    })
}

fn solve_expr(tokens: Vec<TokenValue>) -> Result<Fraction, Err> {
    let mut tokens = convert_token(tokens)?;
    while next_op(&mut tokens)? {}
    if tokens.len() != 1 {
        return Err(Err::IllegalState);
    }
    let token = tokens.get(0).ok_or(Err::IllegalState)?;
    Ok(token.num.ok_or(Err::IllegalState))?
}

fn convert_token(tokens: Vec<TokenValue>) -> Result<Vec<TokenNum>, Err> {
    let mut buffer = Vec::<TokenNum>::with_capacity(tokens.len());
    for i in tokens {
        buffer.push(TokenNum::from_token(&i)?);
    }
    Ok(buffer)
}

fn priority(token: &TokenNum) -> Option<usize> {
    let str = token.token.value.as_str();
    let token = &token.token.token;
    match token {
        Token::StartBlock | Token::EndBlock => Some(0),
        Token::UnaryOperator => Some(1),
        Token::BinaryOperator => match str {
            "/" | "*" | "%" => Some(2),
            "+" | "-" => Some(3),
            _ => None,
        },
        _ => None,
    }
}

fn next_op_index(tokens: &[TokenNum]) -> Option<usize> {
    let mut next_index = None::<usize>;
    let mut next_priority = usize::MAX;
    for (index, token) in tokens.iter().enumerate() {
        if let Some(priority) = priority(token) {
            if priority < next_priority {
                next_index = Some(index);
                next_priority = priority;
            }
        }
    }
    next_index
}

fn next_op(tokens: &mut Vec<TokenNum>) -> Result<bool, Err> {
    let index = match next_op_index(tokens) {
        Some(index) => index,
        None => return Ok(false),
    };
    let op = match tokens.get(index) {
        Some(token) => token,
        None => return Err(Err::IllegalState),
    };
    match &op.token.token {
        Token::StartBlock | Token::EndBlock => todo!("blocks not implemented!"),
        Token::BinaryOperator => {
            let bef_token = tokens
                .get(index.saturating_sub(1))
                .ok_or(Err::IllegalState)?;
            let aft_token = tokens.get(index + 1).ok_or(Err::IllegalState)?;
            let bef = &bef_token.num.ok_or(Err::IllegalState)?;
            let aft = &aft_token.num.ok_or(Err::IllegalState)?;
            let res = match op.token.value.as_str() {
                "-" => bef - aft,
                "+" => bef + aft,
                "*" => bef * aft,
                "/" => bef / aft,
                "%" => bef % aft,
                _ => Err(Err::IllegalState)?,
            };
            let res = res.to_string();
            let token_res = TokenNum::from_token(&TokenValue::from((Token::Number, &res[..])))?;
            tokens.drain(index - 1..=index + 1);
            tokens.insert(index - 1, token_res);
        }
        Token::UnaryOperator => {
            let num_token = tokens.get(index + 1).ok_or(Err::IllegalState)?;
            let num = &num_token.num.ok_or(Err::IllegalState)?;
            let res = match op.token.value.as_str() {
                "-" => -num,
                _ => Err(Err::IllegalState)?,
            };
            let res = res.to_string();
            let token_res = TokenNum::from_token(&TokenValue::from((Token::Number, &res[..])))?;
            tokens.drain(index..=index + 1);
            tokens.insert(index, token_res);
        }
        Token::Number => Err(Err::IllegalState)?,
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- TEST IMPORTANT FUNCTIONS ----------

    #[test]
    fn test_parse_tokens() -> Result<(), Err> {
        let expr1 = "12*3-(-7)";
        let expected_expr_tokens1 = vec![
            TokenValue::from((Token::Number, "12")),
            TokenValue::from((Token::BinaryOperator, "*")),
            TokenValue::from((Token::Number, "3")),
            TokenValue::from((Token::BinaryOperator, "-")),
            TokenValue::from((Token::StartBlock, "(")),
            TokenValue::from((Token::UnaryOperator, "-")),
            TokenValue::from((Token::Number, "7")),
            TokenValue::from((Token::EndBlock, ")")),
        ];
        let actual_expr_tokens1 = parse_tokens(expr1)?;
        let expr2 = "|(||)|||";
        let expected_expr_tokens2 = vec![
            TokenValue::from((Token::StartBlock, "|")),
            TokenValue::from((Token::StartBlock, "(")),
            TokenValue::from((Token::StartBlock, "|")),
            TokenValue::from((Token::EndBlock, "|")),
            TokenValue::from((Token::EndBlock, ")")),
            TokenValue::from((Token::EndBlock, "|")),
            TokenValue::from((Token::StartBlock, "|")),
            TokenValue::from((Token::EndBlock, "|")),
        ];
        let actual_expr_tokens2 = parse_tokens(expr2)?;
        assert_eq!(expected_expr_tokens1, actual_expr_tokens1);
        assert_eq!(expected_expr_tokens2, actual_expr_tokens2);
        Ok(())
    }

    #[test]
    fn test_check_expressions() -> Result<(), Err> {
        let expression_valid1 = &parse_tokens("1 * 2 + 4")?;
        let expression_valid2 = &parse_tokens("-1 * 2 + -4")?;
        let expression_valid3 = &parse_tokens("1 + (12 * (2 + 3) + (3 + 4) + 3)")?;
        let expression_invalid1 = &parse_tokens("-1 * 2 + --4")?;
        let expression_invalid2 = &parse_tokens("( ) * )")?;
        assert!(check_expressions(expression_valid1).is_ok());
        assert!(check_expressions(expression_valid2).is_ok());
        assert!(check_expressions(expression_valid3).is_ok());
        assert!(check_expressions(expression_invalid1).is_err());
        assert!(check_expressions(expression_invalid2).is_err());
        Ok(())
    }

    #[test]
    fn test_solve_expr() -> Result<(), Err> {
        let actual_result1 = solve_expr(parse_tokens("10 % 9 + 3 * 5 * 2 / 5 - -6")?)?;
        let actual_result2 = solve_expr(parse_tokens("10 % (6 / (9 - 3) * 3) * 5 * 3 / 5 - -6")?)?;
        let actual_result3 = solve_expr(parse_tokens("10 % (6 / |9 - 15| * 3) * 5 * 3 / 5 - -6")?)?;
        let expected_result1 = Fraction::from(13);
        let expected_result2 = Fraction::from_str("6.9").or(Err(Err::IllegalState))?;
        let expected_result3 = Fraction::from_str("6.9").or(Err(Err::IllegalState))?;
        assert_eq!(actual_result1, expected_result1);
        assert_eq!(actual_result2, expected_result2);
        assert_eq!(actual_result3, expected_result3);
        Ok(())
    }

    // ---------- TEST UTILITY FUNCTIONS ----------
    #[test]
    fn test_next_op_index() -> Result<(), Err> {
        let expr1 = convert_token(parse_tokens("1 + 2 * 4")?)?;
        let expr2 = convert_token(parse_tokens("1")?)?;
        assert_eq!(next_op_index(&expr1), Some(3));
        assert_eq!(next_op_index(&expr2), None);
        Ok(())
    }
}
