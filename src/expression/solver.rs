#![allow(dead_code, unused)]

use super::types::{
    error::{CheckErr, Error, ParseErr},
    token::{BinaryOp, EndBlock, StartBlock, Token, TokenType, UnaryOp},
};
use fraction::{Fraction, Zero};
use std::{io::Write, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FixRules {
    BlockProduct,
}

impl FixRules {
    pub fn all() -> Vec<Self> {
        vec![FixRules::BlockProduct]
    }

    pub fn none() -> Vec<Self> {
        vec![]
    }
}

pub fn parse_tokens(str: &str) -> Result<Vec<Token>, Error> {
    let mut acc_num = String::new();
    let mut stack = Vec::<StartBlock>::new();
    let mut res = Vec::new();

    for c in str.chars() {
        if !acc_num.is_empty() && !c.is_ascii_digit() && c != '.' {
            res.push(Token::parse_num(&acc_num)?);
            acc_num.clear();
        }

        if c.is_whitespace() {
            continue;
        }

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
            '(' => {
                stack.push(StartBlock::Bracket);
                res.push(Token::from(StartBlock::Bracket));
            }
            ')' => match stack.last() {
                Some(StartBlock::Bracket) => {
                    stack.pop();
                    res.push(Token::from(EndBlock::Bracket));
                }
                _ => Err(CheckErr::UnbalancedBlocks(vec![Token::from(
                    EndBlock::Bracket,
                )]))?,
            },
            '|' => match stack.last() {
                Some(StartBlock::Abs) => {
                    stack.pop();
                    res.push(Token::from(EndBlock::Abs));
                }
                _ => {
                    stack.push(StartBlock::Abs);
                    res.push(Token::from(StartBlock::Abs))
                }
            },
            '0'..='9' | '.' => acc_num += c.to_string().as_str(),
            _ => Err(ParseErr::InvalidToken(c.to_string()))?,
        }
    }

    if !acc_num.is_empty() {
        res.push(Token::parse_num(&acc_num)?);
    }

    if !stack.is_empty() {
        Err(CheckErr::UnbalancedBlocks(
            stack.iter().map(|t| Token::StartBlock(t.clone())).collect(),
        ))?;
    }

    Ok(res)
}

pub fn fix_tokens(tokens: &mut Vec<Token>, rules: &[FixRules]) {
    if rules.contains(&FixRules::BlockProduct) {
        let rule1_pos = tokens
            .iter()
            .enumerate()
            .filter(|(_, t)| t.eq_tokentype(&TokenType::EndBlock))
            .filter(|(i, _)| {
                tokens
                    .get(i + 1)
                    .map(|t| t.eq_tokentype(&TokenType::StartBlock))
                    .unwrap_or(false)
            })
            .map(|(i, _)| i)
            .rev()
            .collect::<Vec<_>>();
        for pos in rule1_pos {
            tokens.insert(pos + 1, Token::BinaryOperator(BinaryOp::Mul));
        }
    }
}

pub fn check_tokens(tokens: &[Token]) -> Result<(), Error> {
    let mut stack = Vec::<TokenType>::new();
    const NUM: &TokenType = &TokenType::Number;
    const STA: &TokenType = &TokenType::StartBlock;
    const END: &TokenType = &TokenType::EndBlock;
    const BIN: &TokenType = &TokenType::BinaryOperator;
    const UNA: &TokenType = &TokenType::UnaryOperator;

    for token in tokens {
        stack.push(TokenType::from(token));
        loop {
            let len = stack.len();
            if check_token(&mut stack, &[NUM, BIN, NUM], false) {
                stack.drain(len - 2..=len - 1);
                continue;
            }
            if check_token(&mut stack, &[UNA, NUM], true) {
                stack.remove(len - 2);
                continue;
            }
            if check_token(&mut stack, &[STA, UNA, NUM], false) {
                stack.remove(len - 2);
                continue;
            }
            if check_token(&mut stack, &[NUM, BIN, UNA, NUM], false) {
                stack.drain(len - 3..=len - 1);
                continue;
            }
            if check_token(&mut stack, &[STA, NUM, END], false) {
                stack.remove(len - 1);
                stack.remove(len - 3);
                continue;
            }
            break;
        }
    }

    if stack.len() != 1 || stack.first() != Some(&TokenType::Number) {
        Err(CheckErr::ExprWithNoResult(stack))?
    }
    Ok(())
}

pub fn check_token(stack: &mut Vec<TokenType>, elems: &[&TokenType], strictly_eq: bool) -> bool {
    let stack_len = stack.len();
    let elems_len = elems.len();
    if (stack_len < elems_len) || (stack_len > elems_len && strictly_eq) {
        return false;
    }
    !stack
        .iter()
        .rev()
        .take(elems_len)
        .enumerate()
        .any(|(i, t)| &t != elems.get(elems_len - i - 1).unwrap())
}

#[cfg(test)]
mod tests {
    use crate::expression::{
        solver::FixRules,
        types::{
            error::Error,
            token::{BinaryOp, EndBlock, StartBlock, Token, UnaryOp},
        },
    };
    use fraction::Fraction;
    use std::str::FromStr;

    use super::{check_tokens, parse_tokens};

    #[test]
    fn test_parsing() -> Result<(), Error> {
        let actual_res1 = super::parse_tokens("(||||)()")?;
        let expected_res1 = vec![
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(EndBlock::Abs),
            Token::from(StartBlock::Abs),
            Token::from(EndBlock::Abs),
            Token::from(EndBlock::Bracket),
            Token::from(StartBlock::Bracket),
            Token::from(EndBlock::Bracket),
        ];
        let actual_res2 = super::parse_tokens("1 -5 *-(|-37|*4.8)+5 %99/7")?;
        let expected_res2 = vec![
            Token::from(Fraction::from(1)),
            Token::from(BinaryOp::Sub),
            Token::from(Fraction::from(5)),
            Token::from(BinaryOp::Mul),
            Token::from(UnaryOp::Neg),
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(UnaryOp::Neg),
            Token::from(Fraction::from(37)),
            Token::from(EndBlock::Abs),
            Token::from(BinaryOp::Mul),
            Token::from(Fraction::from_str("4.8").unwrap()),
            Token::from(EndBlock::Bracket),
            Token::from(BinaryOp::Add),
            Token::from(Fraction::from(5)),
            Token::from(BinaryOp::Mod),
            Token::from(Fraction::from(99)),
            Token::from(BinaryOp::Div),
            Token::from(Fraction::from(7)),
        ];
        assert_eq!(expected_res1, actual_res1);
        assert_eq!(expected_res2, actual_res2);
        Ok(())
    }

    #[test]
    fn test_fix() -> Result<(), Error> {
        let mut actual_tokens_rule1 = super::parse_tokens("()(())||")?;
        super::fix_tokens(&mut actual_tokens_rule1, &FixRules::all());
        let expected_tokens_rule1 = vec![
            Token::from(StartBlock::Bracket),
            Token::from(EndBlock::Bracket),
            Token::from(BinaryOp::Mul),
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Bracket),
            Token::from(EndBlock::Bracket),
            Token::from(EndBlock::Bracket),
            Token::from(BinaryOp::Mul),
            Token::from(StartBlock::Abs),
            Token::from(EndBlock::Abs),
        ];
        assert_eq!(actual_tokens_rule1, expected_tokens_rule1);
        Ok(())
    }

    #[test]
    fn test_check() -> Result<(), Error> {
        let valid1 = super::check_tokens(&super::parse_tokens("-(-12)+34.8*(12+|7|*-(5+|-1|))")?);
        let invalid1 = super::check_tokens(&super::parse_tokens("12---12")?);
        let invalid2 = super::check_tokens(&super::parse_tokens("13*()+9")?);
        assert!(valid1.is_ok());
        assert!(invalid1.is_err());
        assert!(invalid2.is_err());
        Ok(())
    }
}
