#![allow(dead_code)]

use crate::{BinaryOp, EndBlock, StartBlock, Token, UnaryOp};
use fraction::Fraction;
use std::str::FromStr;

pub fn parse_tokens(str: &str) -> Vec<Token> {
    let mut acc_num = String::new();
    let mut stack = Vec::<StartBlock>::new();
    let mut res = Vec::new();

    for c in str.chars() {
        if !acc_num.is_empty() {
            match c {
                '0'..='9' | '.' => (),
                _ => {
                    res.push(Token::Number(Fraction::from_str(&acc_num).unwrap()));
                    acc_num = String::new();
                }
            }
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
                _ => panic!("unbalanced blocks!"),
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
            _ => panic!("invalid token!"),
        }
    }

    if !acc_num.is_empty() {
        res.push(Token::Number(Fraction::from_str(&acc_num).unwrap()));
    }

    if !stack.is_empty() {
        panic!("unbalanced blocks!")
    }

    res
}

pub fn fix_tokens(tokens: &mut Vec<Token>) {
    let str = Token::StartBlock(StartBlock::Abs);
    let end = Token::EndBlock(EndBlock::Abs);

    // RULE1: "(expr)(expr)" ---> "(expr)*(expr)"
    let rule1_pos = tokens
        .iter()
        .enumerate()
        .filter(|(_, t)| t.eq_type(&end))
        .filter(|(i, _)| tokens.get(i + 1).map(|t| t.eq_type(&str)).unwrap_or(false))
        .map(|(i, _)| i)
        .rev()
        .collect::<Vec<_>>();
    for pos in rule1_pos {
        tokens.insert(pos + 1, Token::BinaryOperator(BinaryOp::Mul));
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryOp, EndBlock, StartBlock, Token, UnaryOp};
    use fraction::Fraction;
    use std::str::FromStr;

    #[test]
    fn test_parsing() {
        let actual_res1 = super::parse_tokens("(||||)()");
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
        let actual_res2 = super::parse_tokens("1 -5 *-(|-37|*4.8)+5 %99/7");
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
    }

    #[test]
    fn test_fix() {
        let mut actual_tokens_rule1 = super::parse_tokens("()(())||");
        super::fix_tokens(&mut actual_tokens_rule1);
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
    }
}
