#![allow(dead_code, unused)]

use super::error::{Error, ParseErr};
use fraction::{BigFraction, BigUint, GenericFraction};
use std::{fmt::Display, mem, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    StartBlock(StartBlock),
    EndBlock(EndBlock),
    UnaryOperatorLeft(UnaryOpLeft),
    UnaryOperatorRight(UnaryOpRight),
    BinaryOperator(BinaryOp),
    Number(BigFraction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    StartBlock,
    EndBlock,
    UnaryOperatorLeft,
    UnaryOperatorRight,
    BinaryOperator,
    Number,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartBlock {
    Bracket,
    Abs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndBlock {
    Bracket,
    Abs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpLeft {
    Neg,
    Pos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpRight {
    Fact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    Exp,
}

impl From<StartBlock> for Token {
    fn from(value: StartBlock) -> Self {
        Token::StartBlock(value)
    }
}

impl From<EndBlock> for Token {
    fn from(value: EndBlock) -> Self {
        Token::EndBlock(value)
    }
}

impl From<UnaryOpLeft> for Token {
    fn from(value: UnaryOpLeft) -> Self {
        Token::UnaryOperatorLeft(value)
    }
}

impl From<BinaryOp> for Token {
    fn from(value: BinaryOp) -> Self {
        Token::BinaryOperator(value)
    }
}

impl From<BigFraction> for Token {
    fn from(value: BigFraction) -> Self {
        Token::Number(value)
    }
}

impl From<&Token> for TokenType {
    fn from(value: &Token) -> Self {
        match value {
            Token::StartBlock(_) => Self::StartBlock,
            Token::EndBlock(_) => Self::EndBlock,
            Token::UnaryOperatorLeft(_) => Self::UnaryOperatorLeft,
            Token::BinaryOperator(_) => Self::BinaryOperator,
            Token::Number(_) => Self::Number,
            Token::UnaryOperatorRight(_) => Self::UnaryOperatorRight,
        }
    }
}

impl Display for StartBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StartBlock::Abs => "|",
            StartBlock::Bracket => "(",
        };
        write!(f, "{str}")
    }
}

impl Display for EndBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EndBlock::Abs => "|",
            EndBlock::Bracket => ")",
        };
        write!(f, "{str}")
    }
}

impl Display for UnaryOpLeft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UnaryOpLeft::Neg => "-",
            UnaryOpLeft::Pos => "+",
        };
        write!(f, "{str}")
    }
}

impl Display for UnaryOpRight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UnaryOpRight::Fact => "!",
        };
        write!(f, "{str}")
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Mod => "%",
            BinaryOp::Div => "/",
            BinaryOp::Exp => "^",
        };
        write!(f, "{str}")
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::StartBlock(str) => str.to_string(),
            Token::EndBlock(str) => str.to_string(),
            Token::UnaryOperatorLeft(str) => str.to_string(),
            Token::UnaryOperatorRight(str) => str.to_string(),
            Token::BinaryOperator(str) => str.to_string(),
            Token::Number(str) => str.to_string(),
        };
        write!(f, "{str}")
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl Token {
    pub fn parse_num(str: &str) -> Result<Self, Error> {
        let num = BigFraction::from_str(str).or(Err(ParseErr::InvalidNumber(str.to_string())))?;
        Ok(Token::Number(num))
    }

    pub fn num(&self) -> Option<&BigFraction> {
        match self {
            Token::Number(num) => Some(num),
            _ => None,
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            Token::StartBlock(_) => 0,
            Token::UnaryOperatorRight(_) => 1,
            Token::UnaryOperatorLeft(_) => 2,
            Token::BinaryOperator(op) => match op {
                BinaryOp::Exp => 3,
                BinaryOp::Mul | BinaryOp::Mod | BinaryOp::Div => 4,
                BinaryOp::Add | BinaryOp::Sub => 5,
            },
            _ => usize::MAX,
        }
    }

    pub fn eq_type(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn eq_tokentype(&self, other: &TokenType) -> bool {
        mem::discriminant(&TokenType::from(self)) == mem::discriminant(other)
    }
}

impl TokenType {
    pub fn eq_tokentype(&self, other: &Token) -> bool {
        other.eq_tokentype(self)
    }
}

impl StartBlock {
    pub fn corrisp(&self) -> EndBlock {
        match self {
            StartBlock::Bracket => EndBlock::Bracket,
            StartBlock::Abs => EndBlock::Abs,
        }
    }

    pub fn is_corrisp(&self, other: EndBlock) -> bool {
        self.corrisp() == other
    }
}

impl EndBlock {
    pub fn corrisp(&self) -> StartBlock {
        match self {
            EndBlock::Bracket => StartBlock::Bracket,
            EndBlock::Abs => StartBlock::Abs,
        }
    }

    pub fn is_corrisp(&self, other: StartBlock) -> bool {
        self.corrisp() == other
    }
}
