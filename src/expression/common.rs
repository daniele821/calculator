#![allow(dead_code, unused)]

use fraction::Fraction;

// ------------------------------ TOKEN ------------------------------

pub enum Token {
    StartBlock(StartBlock),
    EndBlock(EndBlock),
    UnaryOperator(UnaryOp),
    BinaryOperator(BinaryOp),
    Number(Fraction),
}

pub enum StartBlock {
    Bracket,
    Abs,
}

pub enum EndBlock {
    Bracket,
    Abs,
}

pub enum UnaryOp {
    Neg,
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Mod,
    Div,
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

impl From<UnaryOp> for Token {
    fn from(value: UnaryOp) -> Self {
        Token::UnaryOperator(value)
    }
}

impl From<BinaryOp> for Token {
    fn from(value: BinaryOp) -> Self {
        Token::BinaryOperator(value)
    }
}

impl From<Fraction> for Token {
    fn from(value: Fraction) -> Self {
        Token::Number(value)
    }
}
