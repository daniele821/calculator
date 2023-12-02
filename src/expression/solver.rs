use crate::{BinaryOp, Token, UnaryOp};

pub fn parse_tokens(str: &str) -> Vec<Token> {
    let mut res = Vec::new();

    for c in str.chars() {
        match c {
            '+' => res.push(Token::from(BinaryOp::Add)),
            '-' => match res.last() {
                Some(Token::Number(_)) | Some(Token::EndBlock(_)) => {
                    res.push(Token::from(BinaryOp::Sub))
                }
                _ => res.push(Token::from(UnaryOp::Neg)),
            },
            '*' => res.push(Token::from(BinaryOp::Mul)),
            '/' => res.push(Token::from(BinaryOp::Div)),
            '%' => res.push(Token::from(BinaryOp::Mod)),
            '(' => todo!(),
            ')' => todo!(),
            '|' => todo!(),
            '0'..='9' | '.' => todo!(),
            _ => panic!("invalid token!"),
        }
    }

    res
}
