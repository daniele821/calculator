// #![allow(dead_code, unused)]

use crate::{
    common::{self, algs, Color},
    expression::{
        error::{CheckErr, Error, ParseErr, SolveErr},
        token::{BinaryOp, EndBlock, StartBlock, Token, TokenType, UnaryOpLeft},
    },
};
use fraction::{BigFraction, Zero};
use std::ops::Neg;

use super::token::UnaryOpRight;

const STA: TokenType = TokenType::StartBlock;
const END: TokenType = TokenType::EndBlock;
const UNL: TokenType = TokenType::UnaryOperatorLeft;
const UNR: TokenType = TokenType::UnaryOperatorRight;
const BIN: TokenType = TokenType::BinaryOperator;
const NUM: TokenType = TokenType::Number;
const POS: Token = Token::UnaryOperatorLeft(UnaryOpLeft::Pos);
const NEG: Token = Token::UnaryOperatorLeft(UnaryOpLeft::Neg);
const ADD: Token = Token::BinaryOperator(BinaryOp::Add);
const SUB: Token = Token::BinaryOperator(BinaryOp::Sub);
const DENY_DIV: CheckRules = CheckRules::DenyDivision;
const DENY_MOD: CheckRules = CheckRules::DenyModule;
const DENY_MLS: CheckRules = CheckRules::DenyMultipleSign;
const DENY_AMS: CheckRules = CheckRules::DenyAllMultipleSign;
const DENY_EXP: CheckRules = CheckRules::DenyExponent;
const DENY_FAC: CheckRules = CheckRules::DenyFactorial;
const DENY_DERANG: CheckRules = CheckRules::DenyDerangement;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FixRules {
    /// fixes: "(expr) (expr)" => "(expr) * (expr)"
    BlockProduct,
    /// fixes: "(|(" => "(|()|)"
    CloseBlocks,
}
impl FixRules {
    pub const ALL: [Self; 2] = [Self::BlockProduct, Self::CloseBlocks];
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CheckRules {
    /// deny: "++expr", "-+-+expr++expr", allow: "expr+-expr", "expr--expr"
    DenyMultipleSign,
    /// deny: "++expr", "-+-+expr++expr", "expr+-expr", "expr--expr"
    DenyAllMultipleSign,
    /// deny: "expr / expr"
    DenyDivision,
    /// deny: "expr % expr"
    DenyModule,
    /// deny: "expr ^ int_expr"
    DenyExponent,
    /// deny: "expr !"
    DenyFactorial,
    /// deny: "! expr"
    DenyDerangement,
}
impl CheckRules {
    pub const ALL: [Self; 7] = [
        DENY_MLS,
        DENY_AMS,
        DENY_DIV,
        DENY_MOD,
        DENY_EXP,
        DENY_FAC,
        DENY_DERANG,
    ];
    pub const DENY_OP: [Self; 5] = [DENY_DIV, DENY_MOD, DENY_EXP, DENY_FAC, DENY_DERANG];
    pub const DENY_SIGN: [Self; 2] = [DENY_MLS, DENY_AMS];
}

pub fn resolve(
    str: &str,
    fixes: &[FixRules],
    checks: &[CheckRules],
    explain: bool,
) -> Result<BigFraction, Error> {
    let mut tokens = parse(str, fixes, checks)?;
    if explain {
        let title = common::color(&Color::TIT, "Explanation:");
        println!("{title}\n{}", common::fmt(&tokens, None));
    }
    while solve_next(&mut tokens)? {
        if explain {
            println!("{}", common::fmt(&tokens, None));
        }
    }
    let e = || SolveErr::ExprWithNoResult(tokens.clone());
    if tokens.len() != 1 {
        return Err(Error::Solve(SolveErr::ExprWithNoResult(tokens)));
    }
    Ok(tokens.first().ok_or_else(e)?.num().ok_or_else(e)?.clone())
}

pub fn parse(str: &str, fixes: &[FixRules], checks: &[CheckRules]) -> Result<Vec<Token>, Error> {
    let mut tokens = parse_tokens(str)?;
    fix_tokens(&mut tokens, fixes);
    check_rules(&tokens, checks)?;
    Ok(tokens)
}

fn parse_tokens(str: &str) -> Result<Vec<Token>, Error> {
    let mut acc_num = String::new();
    let mut stack = Vec::<StartBlock>::new();
    let mut res = Vec::new();

    for c in str.chars() {
        if !acc_num.is_empty() && !c.is_ascii_digit() && c != '.' && c != '_' {
            res.push(Token::parse_num(&acc_num)?);
            acc_num.clear();
        }
        if c.is_whitespace() {
            continue;
        }
        match c {
            '+' => match res.last() {
                Some(Token::Number(_))
                | Some(Token::EndBlock(_))
                | Some(Token::UnaryOperatorRight(_)) => res.push(Token::from(BinaryOp::Add)),
                _ => res.push(Token::from(UnaryOpLeft::Pos)),
            },
            '-' => match res.last() {
                Some(Token::Number(_))
                | Some(Token::EndBlock(_))
                | Some(Token::UnaryOperatorRight(_)) => res.push(Token::from(BinaryOp::Sub)),
                _ => res.push(Token::from(UnaryOpLeft::Neg)),
            },
            '!' => res.push(Token::from(UnaryOpRight::Fact)),
            '^' => res.push(Token::from(BinaryOp::Exp)),
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
            '0'..='9' | '.' | '_' => acc_num.push(c),
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

fn check_rules(tokens: &[Token], checks: &[CheckRules]) -> Result<(), Error> {
    let mut block_stack = Vec::<StartBlock>::new();
    let mul_sign = checks.contains(&DENY_MLS);
    let all_sign = checks.contains(&DENY_AMS);
    let deny_div = checks.contains(&DENY_DIV);
    let deny_mod = checks.contains(&DENY_MOD);
    let deny_exp = checks.contains(&DENY_EXP);
    let deny_fac = checks.contains(&DENY_FAC);

    // check rules are respected
    for token in tokens {
        if deny_div && token == &Token::from(BinaryOp::Div) {
            Err(CheckErr::BrokenCheckRule(DENY_DIV))?;
        }
        if deny_mod && token == &Token::from(BinaryOp::Mod) {
            Err(CheckErr::BrokenCheckRule(DENY_MOD))?;
        }
        if deny_exp && token == &Token::from(BinaryOp::Exp) {
            Err(CheckErr::BrokenCheckRule(DENY_EXP))?;
        }
        if deny_fac && token == &Token::from(UnaryOpRight::Fact) {
            Err(CheckErr::BrokenCheckRule(DENY_FAC))?;
        }
    }
    for pair in tokens.windows(2) {
        if mul_sign && [POS, NEG].contains(&pair[0]) && [POS, NEG].contains(&pair[1]) {
            Err(CheckErr::BrokenCheckRule(DENY_MLS))?;
        }
        if all_sign && [POS, NEG, ADD, SUB].contains(&pair[0]) && [POS, NEG].contains(&pair[1]) {
            Err(CheckErr::BrokenCheckRule(DENY_AMS))?;
        }
    }

    // check blocks are balanced
    for token in tokens {
        match token {
            Token::StartBlock(start) => block_stack.push(start.clone()),
            Token::EndBlock(end) => assert_eq!(block_stack.pop(), Some(end.corrisp())),
            _ => (),
        }
    }
    if !block_stack.is_empty() {
        let block_stack: Vec<Token> = common::convert(&block_stack);
        Err(CheckErr::UnbalancedBlocks(block_stack))?
    }

    Ok(())
}

pub fn solve_next(tokens: &mut Vec<Token>) -> Result<bool, Error> {
    if let Some(index) = next_operation(tokens) {
        let token = &tokens[index];
        let err = || Error::Solve(SolveErr::ExprWithNoResult(tokens.to_vec()));
        let mut nums = Vec::<&BigFraction>::new();
        let from: usize;
        let to: usize;
        match TokenType::from(token) {
            STA => {
                nums.push(tokens[index + 1].num().ok_or_else(err)?);
                from = index;
                to = index + 2;
            }
            UNL => {
                nums.push(tokens[index + 1].num().ok_or_else(err)?);
                from = index;
                to = index + 1;
            }
            UNR => {
                nums.push(tokens[index - 1].num().ok_or_else(err)?);
                from = index - 1;
                to = index;
            }
            BIN => {
                nums.push(tokens[index - 1].num().ok_or_else(err)?);
                nums.push(tokens[index + 1].num().ok_or_else(err)?);
                from = index - 1;
                to = index + 1;
            }
            _ => unreachable!(),
        }
        let num = match token {
            Token::StartBlock(start) => match start {
                StartBlock::Bracket => nums[0].clone(),
                StartBlock::Abs => nums[0].abs(),
            },
            Token::UnaryOperatorLeft(unary) => match unary {
                UnaryOpLeft::Neg => nums[0].neg(),
                UnaryOpLeft::Pos => nums[0].clone(),
                UnaryOpLeft::Derang => todo!("derangement algorithm"),
            },
            Token::UnaryOperatorRight(unary) => match unary {
                UnaryOpRight::Fact => algs::fact(nums[0])?,
            },
            Token::BinaryOperator(bin) => match bin {
                BinaryOp::Add => nums[0] + nums[1],
                BinaryOp::Sub => nums[0] - nums[1],
                BinaryOp::Mul => nums[0] * nums[1],
                BinaryOp::Mod => calculate(&nums, bin)?,
                BinaryOp::Div => calculate(&nums, bin)?,
                BinaryOp::Exp => algs::exp(nums[0], nums[1])?,
            },
            _ => unreachable!(),
        };
        tokens.drain(from..=to);
        tokens.insert(from, Token::Number(num));
        return Ok(true);
    } else if tokens.len() != 1 {
        Err(SolveErr::ExprWithNoResult(tokens.clone()))?;
    }
    Ok(false)
}

fn next_operation(tokens: &[Token]) -> Option<usize> {
    let mut op_index = None::<usize>;
    let mut op_priority = usize::MAX;
    for (index, token) in tokens.iter().enumerate() {
        if token.priority() < op_priority {
            let before1 = tokens.get(index.saturating_sub(1)).map(TokenType::from);
            let before1 = if index == 0 { None } else { before1 };
            let current = tokens.get(index).map(TokenType::from);
            let after1 = tokens.get(index + 1).map(TokenType::from);
            let after2 = tokens.get(index + 2).map(TokenType::from);
            match (before1, current, after1, after2) {
                (Some(NUM), Some(BIN), Some(NUM), _)
                | (_, Some(STA), Some(NUM), Some(END))
                | (_, Some(UNL), Some(NUM), _)
                | (Some(NUM), Some(UNR), _, _) => {
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

fn calculate(nums: &[&BigFraction], op: &BinaryOp) -> Result<BigFraction, Error> {
    match op {
        BinaryOp::Mod | BinaryOp::Div => {
            if nums[1].is_zero() {
                let vec = vec![
                    Token::from(nums[0].clone()),
                    Token::from(op.clone()),
                    Token::from(nums[1].clone()),
                ];
                Err(SolveErr::OperIllegalValues(vec))?;
            }
        }
        _ => unreachable!(),
    }
    match op {
        BinaryOp::Mod => Ok(nums[0] % nums[1]),
        BinaryOp::Div => Ok(nums[0] / nums[1]),
        _ => unreachable!(),
    }
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
            Token::from(UnaryOpLeft::Neg),
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(UnaryOpLeft::Neg),
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
        let valid1 = check_rules(&parse_tokens("-+3.8*(1+|7|*-(5+|-1|))")?, &[]);
        let test_rule1_ok = check_rules(&parse_tokens("-3++5+-1-+4--2")?, rule1);
        let test_rule1_err = check_rules(&parse_tokens("--5")?, rule1);
        let test_rule2_ok = check_rules(&parse_tokens("-3+5")?, rule2);
        let test_rule2_err = check_rules(&parse_tokens("4--5")?, rule2);
        assert!(valid1.is_ok());
        assert!(test_rule1_ok.is_ok());
        assert!(test_rule1_err.is_err());
        assert!(test_rule2_ok.is_ok());
        assert!(test_rule2_err.is_err());
        Ok(())
    }

    #[test]
    fn test_next_op() -> Result<(), Error> {
        let expr1 = parse("12+34*45", &FixRules::ALL, &[])?;
        let expr2 = parse("12+(12)", &FixRules::ALL, &[])?;
        let expr3 = parse("12+(12/34)", &FixRules::ALL, &[])?;
        assert_eq!(next_operation(&expr1), Some(3));
        assert_eq!(next_operation(&expr2), Some(2));
        assert_eq!(next_operation(&expr3), Some(4));
        Ok(())
    }
}
