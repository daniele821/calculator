#![allow(dead_code, unused)]

use crate::{
    common,
    expression::{
        error::{CheckErr, Error, ParseErr, SolveErr},
        token::{BinaryOp, EndBlock, StartBlock, Token, TokenType, UnaryOp},
    },
};
use fraction::Fraction;
use std::ops::{Neg, Range, RangeBounds, RangeInclusive};

const STA: TokenType = TokenType::StartBlock;
const END: TokenType = TokenType::EndBlock;
const UNA: TokenType = TokenType::UnaryOperator;
const BIN: TokenType = TokenType::BinaryOperator;
const NUM: TokenType = TokenType::Number;
const POS: Token = Token::UnaryOperator(UnaryOp::Pos);
const NEG: Token = Token::UnaryOperator(UnaryOp::Neg);
const ADD: Token = Token::BinaryOperator(BinaryOp::Add);
const SUB: Token = Token::BinaryOperator(BinaryOp::Sub);

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
    /// deny: "++expr", "-+-+expr++expr", allow: "expr+-expr", "expr--expr"
    DenyMultipleSign,
    /// deny: "++expr", "-+-+expr++expr", "expr+-expr", "expr--expr"
    DenyAllMultipleSign,
}
impl CheckRules {
    pub fn all() -> Vec<Self> {
        vec![
            CheckRules::DenyMultipleSign,
            CheckRules::DenyAllMultipleSign,
        ]
    }
}

pub fn parse(str: &str, fixes: &[FixRules], checks: &[CheckRules]) -> Result<Vec<Token>, Error> {
    let mut tokens = parse_tokens(str)?;
    fix_tokens(&mut tokens, fixes);
    check_tokens(&tokens, checks)?;
    Ok(tokens)
}

fn parse_tokens(str: &str) -> Result<Vec<Token>, Error> {
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
            '0'..='9' | '.' => acc_num.push(c),
            _ => Err(ParseErr::InvalidToken(c.to_string()))?,
        }
    }

    if !acc_num.is_empty() {
        res.push(Token::parse_num(&acc_num)?);
    }

    Ok(res)
}

fn fix_tokens(tokens: &mut Vec<Token>, rules: &[FixRules]) {
    if rules.contains(&FixRules::BlockProduct) {
        let rule1_pos = tokens
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| (w[0].eq_tokentype(&END) && w[1].eq_tokentype(&STA)).then_some(i))
            .rev()
            .collect::<Vec<_>>();
        for pos in rule1_pos {
            tokens.insert(pos + 1, Token::BinaryOperator(BinaryOp::Mul));
        }
    }

    if rules.contains(&FixRules::CloseBlocks) {
        let mut stack = Vec::<StartBlock>::new();
        for token in tokens.iter() {
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

fn check_tokens(tokens: &[Token], checks: &[CheckRules]) -> Result<(), Error> {
    let mut block_stack = Vec::<StartBlock>::new();
    let mut token_stack = Vec::<TokenType>::new();

    // check rules are respected
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
            if check_token(&mut token_stack, &[&NUM, &BIN, &NUM], false) {
                token_stack.drain(len - 2..=len - 1);
                continue;
            }
            if check_token(&mut token_stack, &[&UNA, &NUM], false) {
                token_stack.remove(len - 2);
                continue;
            }
            if check_token(&mut token_stack, &[&STA, &NUM, &END], false) {
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
    if token_stack.len() != 1 || token_stack.first() != Some(&NUM) {
        Err(CheckErr::ExprWithNoResult(token_stack))?
    }

    Ok(())
}

fn check_token(stack: &mut Vec<TokenType>, elems: &[&TokenType], strictly_eq: bool) -> bool {
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

fn check_rules(tokens: &[Token], checks: &[CheckRules]) -> Result<(), Error> {
    let mul_sign = checks.contains(&CheckRules::DenyMultipleSign);
    let all_sign = checks.contains(&CheckRules::DenyAllMultipleSign);

    for pair in tokens.windows(2) {
        if all_sign && [POS, NEG, ADD, SUB].contains(&pair[0]) && [POS, NEG].contains(&pair[1]) {
            Err(CheckErr::InvalidAdiacents(pair.to_vec()))?;
        }
        if mul_sign && [POS, NEG].contains(&pair[0]) && [POS, NEG].contains(&pair[1]) {
            Err(CheckErr::InvalidAdiacents(pair.to_vec()))?;
        }
    }

    Ok(())
}

pub fn solve(tokens: &mut Vec<Token>) -> Result<Fraction, Error> {
    while solve_one_op(tokens) {}
    get_result(tokens)
}

pub fn solve_one_op(tokens: &mut Vec<Token>) -> bool {
    let index = next_operation(tokens);
    if let Some(index) = index {
        let token = &tokens[index];
        let mut nums = Vec::<&Fraction>::new();
        let from: usize;
        let to: usize;
        match TokenType::from(token) {
            STA => {
                nums.push(tokens[index + 1].num().unwrap());
                from = index;
                to = index + 2;
            }
            UNA => {
                nums.push(tokens[index + 1].num().unwrap());
                from = index;
                to = index + 1;
            }
            BIN => {
                nums.push(tokens[index - 1].num().unwrap());
                nums.push(tokens[index + 1].num().unwrap());
                from = index - 1;
                to = index + 1;
            }
            _ => unreachable!(),
        }
        let num: Fraction = match token {
            Token::StartBlock(start) => match start {
                StartBlock::Bracket => *nums[0],
                StartBlock::Abs => nums[0].abs(),
            },
            Token::UnaryOperator(unary) => match unary {
                UnaryOp::Neg => nums[0].neg(),
                UnaryOp::Pos => *nums[0],
            },
            Token::BinaryOperator(bin) => match bin {
                BinaryOp::Add => nums[0] + nums[1],
                BinaryOp::Sub => nums[0] - nums[1],
                BinaryOp::Mul => nums[0] * nums[1],
                BinaryOp::Mod => nums[0] % nums[1],
                BinaryOp::Div => nums[0] / nums[1],
            },
            _ => unreachable!(),
        };
        tokens.drain(from..=to);
        tokens.insert(from, Token::Number(num));
        return true;
    }
    false
}

pub fn get_result(tokens: &[Token]) -> Result<Fraction, Error> {
    assert_eq!(tokens.len(), 1);
    let res = tokens.first().unwrap().num().unwrap();
    if !res.is_finite() {
        Err(SolveErr::NotRationalNumber(*res))?;
    }
    Ok(*res)
}

fn next_operation(tokens: &[Token]) -> Option<usize> {
    let mut op_index = None::<usize>;
    let mut op_priority = usize::MAX;
    for (index, token) in tokens.iter().enumerate() {
        if token.priority() < op_priority {
            let before1 = tokens.get(index.saturating_sub(1)).map(TokenType::from);
            let current = tokens.get(index).map(TokenType::from);
            let after1 = tokens.get(index + 1).map(TokenType::from);
            let after2 = tokens.get(index + 2).map(TokenType::from);
            match (before1, current, after1, after2) {
                (Some(NUM), Some(BIN), Some(NUM), _)
                | (_, Some(STA), Some(NUM), Some(END))
                | (_, Some(UNA), Some(NUM), _) => {
                    op_index = Some(index);
                    op_priority = token.priority();
                }
                _ => (),
            }
        }
        if op_priority == 0 {
            break;
        }
    }
    op_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() -> Result<(), Error> {
        let actual_res1 = parse_tokens("(||||)()")?;
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
        let actual_res2 = parse_tokens("1 -5 *-(|-37|*4.8)+5 %99/7")?;
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
        let mut actual_tokens_rule1 = parse_tokens("()(())||")?;
        fix_tokens(&mut actual_tokens_rule1, rule1);
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
        let mut actual_tokens_rule2 = parse_tokens("(|(")?;
        fix_tokens(&mut actual_tokens_rule2, rule2);
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
        let rule1 = &[CheckRules::DenyMultipleSign];
        let rule2 = &[CheckRules::DenyAllMultipleSign];
        let valid1 = check_tokens(&parse_tokens("-+3.8*(1+|7|*-(5+|-1|))")?, &[]);
        let invalid1 = check_tokens(&parse_tokens("13*()+9")?, &[]);
        let test_rule1_ok = check_tokens(&parse_tokens("-3++5+-1-+4--2")?, rule1);
        let test_rule1_err = check_tokens(&parse_tokens("--5")?, rule1);
        let test_rule2_ok = check_tokens(&parse_tokens("-3+5")?, rule2);
        let test_rule2_err = check_tokens(&parse_tokens("4--5")?, rule2);
        assert!(valid1.is_ok());
        assert!(invalid1.is_err());
        assert!(test_rule1_ok.is_ok());
        assert!(test_rule1_err.is_err());
        assert!(test_rule2_ok.is_ok());
        assert!(test_rule2_err.is_err());
        Ok(())
    }

    #[test]
    fn test_next_op() -> Result<(), Error> {
        let expr1 = parse("12+34*45", &FixRules::all(), &CheckRules::all())?;
        let expr2 = parse("12+(12)", &FixRules::all(), &CheckRules::all())?;
        let expr3 = parse("12+(12/34)", &FixRules::all(), &CheckRules::all())?;
        assert_eq!(next_operation(&expr1), Some(3));
        assert_eq!(next_operation(&expr2), Some(2));
        assert_eq!(next_operation(&expr3), Some(4));
        Ok(())
    }

    #[test]
    fn test_solve() -> Result<(), Error> {
        let mut expr1 = parse("12+34*45", &FixRules::all(), &CheckRules::all())?;
        let mut expr2 = parse("-|-12|+34*45", &FixRules::all(), &CheckRules::all())?;
        assert_eq!(solve(&mut expr1)?, Fraction::from(1542));
        assert_eq!(solve(&mut expr2)?, Fraction::from(1518));
        Ok(())
    }
}
