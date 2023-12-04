#![allow(dead_code, unused)]

use super::{
    error::{CheckErr, Error, ParseErr},
    token::{BinaryOp, EndBlock, StartBlock, Token, TokenType, UnaryOp},
};
use crate::common;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FixRules {
    /// fixes: "(expr) (expr)" => "(expr) * (expr)"
    BlockProduct,
    /// fixes: "(|(" => "(|()|)"
    CloseBlocks,
}
impl FixRules {
    pub fn all() -> Vec<Self> {
        vec![FixRules::BlockProduct, FixRules::CloseBlocks]
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CheckRules {
    /// allows: "expr+-expr", "++expr", "-+-+expr++expr", ...
    AllowSignMul,
}
impl CheckRules {
    pub fn all() -> Vec<Self> {
        vec![CheckRules::AllowSignMul]
    }
}

pub fn parse(str: &str, fixes: &[FixRules], checks: &[CheckRules]) -> Result<Vec<Token>, Error> {
    let mut tokens = parse_tokens(str)?;
    fix_tokens(&mut tokens, fixes);
    check_tokens(&tokens, checks)?;
    Ok(tokens)
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
            '+' => match res.last() {
                Some(Token::Number(_)) | Some(Token::EndBlock(_)) => {
                    res.push(Token::from(BinaryOp::Add))
                }
                _ => res.push(Token::from(UnaryOp::Pos)),
            },
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

    if rules.contains(&FixRules::CloseBlocks) {
        let mut stack = Vec::<StartBlock>::new();
        for token in tokens.iter().filter(|t| {
            t.eq_tokentype(&TokenType::StartBlock) || t.eq_tokentype(&TokenType::EndBlock)
        }) {
            match token {
                Token::StartBlock(start) => stack.push(start.clone()),
                Token::EndBlock(end) => assert_eq!(stack.pop(), Some(end.corrisp())),
                _ => (),
            }
        }
        for block in stack.iter().rev() {
            tokens.push(Token::from(block.corrisp()));
        }
    }
}

pub fn check_tokens(tokens: &[Token], checks: &[CheckRules]) -> Result<(), Error> {
    let mut block_stack = Vec::<StartBlock>::new();
    let mut token_stack = Vec::<TokenType>::new();
    const NUM: &TokenType = &TokenType::Number;
    const STA: &TokenType = &TokenType::StartBlock;
    const END: &TokenType = &TokenType::EndBlock;
    const BIN: &TokenType = &TokenType::BinaryOperator;
    const UNA: &TokenType = &TokenType::UnaryOperator;

    check_rules(tokens, checks)?;

    for token in tokens {
        // check blocks
        match token {
            Token::StartBlock(start) => block_stack.push(start.clone()),
            Token::EndBlock(end) => assert_eq!(block_stack.pop(), Some(end.corrisp())),
            _ => (),
        }
        // collapse expression
        token_stack.push(TokenType::from(token));
        loop {
            let len = token_stack.len();
            if check_token(&mut token_stack, &[NUM, BIN, NUM], false) {
                token_stack.drain(len - 2..=len - 1);
                continue;
            }
            if check_token(&mut token_stack, &[UNA, NUM], false) {
                token_stack.remove(len - 2);
                continue;
            }
            if check_token(&mut token_stack, &[STA, NUM, END], false) {
                token_stack.remove(len - 1);
                token_stack.remove(len - 3);
                continue;
            }
            break;
        }
    }

    // errors for unbalanced blocks
    if !block_stack.is_empty() {
        let block_stack: Vec<Token> = common::convert(&block_stack);
        Err(CheckErr::UnbalancedBlocks(block_stack))?
    }
    // errors for expression collapsion
    if token_stack.len() != 1 || token_stack.first() != Some(&TokenType::Number) {
        Err(CheckErr::ExprWithNoResult(token_stack))?
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

pub fn check_rules(tokens: &[Token], checks: &[CheckRules]) -> Result<(), Error> {
    const POS: &Token = &Token::UnaryOperator(UnaryOp::Pos);
    const NEG: &Token = &Token::UnaryOperator(UnaryOp::Neg);
    const ADD: &Token = &Token::BinaryOperator(BinaryOp::Add);
    const SUB: &Token = &Token::BinaryOperator(BinaryOp::Sub);

    let sign_mul = checks.contains(&CheckRules::AllowSignMul);

    for pair in tokens.windows(2) {
        if sign_mul && [POS, NEG, ADD, SUB].contains(&&pair[0]) && [POS, NEG].contains(&&pair[1]) {
            Err(CheckErr::InvalidAdiacents(pair.to_vec()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Token::parse_num("1")?,
            Token::from(BinaryOp::Sub),
            Token::parse_num("5")?,
            Token::from(BinaryOp::Mul),
            Token::from(UnaryOp::Neg),
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(UnaryOp::Neg),
            Token::parse_num("37")?,
            Token::from(EndBlock::Abs),
            Token::from(BinaryOp::Mul),
            Token::parse_num("4.8")?,
            Token::from(EndBlock::Bracket),
            Token::from(BinaryOp::Add),
            Token::parse_num("5")?,
            Token::from(BinaryOp::Mod),
            Token::parse_num("99")?,
            Token::from(BinaryOp::Div),
            Token::parse_num("7")?,
        ];
        assert_eq!(expected_res1, actual_res1);
        assert_eq!(expected_res2, actual_res2);
        Ok(())
    }

    #[test]
    fn test_fix() -> Result<(), Error> {
        let rule1 = &[FixRules::BlockProduct];
        let rule2 = &[FixRules::CloseBlocks];
        let mut actual_tokens_rule1 = super::parse_tokens("()(())||")?;
        super::fix_tokens(&mut actual_tokens_rule1, rule1);
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
        let mut actual_tokens_rule2 = super::parse_tokens("(|(")?;
        super::fix_tokens(&mut actual_tokens_rule2, rule2);
        let expected_tokens_rule2 = vec![
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(StartBlock::Bracket),
            Token::from(EndBlock::Bracket),
            Token::from(EndBlock::Abs),
            Token::from(EndBlock::Bracket),
        ];
        assert_eq!(actual_tokens_rule1, expected_tokens_rule1);
        assert_eq!(actual_tokens_rule2, expected_tokens_rule2);
        assert_ne!(actual_tokens_rule1, expected_tokens_rule2);
        assert_ne!(actual_tokens_rule2, expected_tokens_rule1);
        Ok(())
    }

    #[test]
    fn test_check() -> Result<(), Error> {
        let rule1 = &[CheckRules::AllowSignMul];
        let valid1 = super::check_tokens(&super::parse_tokens("-+3.8*(1+|7|*-(5+|-1|))")?, &[]);
        let invalid1 = super::check_tokens(&super::parse_tokens("13*()+9")?, &[]);
        let test_rule1_ok1 = super::check_tokens(&super::parse_tokens("1--3++1-+-+5")?, &[]);
        let test_rule1_err1 = super::check_tokens(&super::parse_tokens("1--3++1-+-+5")?, rule1);
        let test_rule1_err2 = super::check_tokens(&super::parse_tokens("1--3++1-+5")?, rule1);
        assert!(valid1.is_ok());
        assert!(invalid1.is_err());
        assert!(test_rule1_ok1.is_ok());
        assert!(test_rule1_err1.is_err());
        assert!(test_rule1_err2.is_err());
        Ok(())
    }
}
