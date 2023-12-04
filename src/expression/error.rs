#![allow(dead_code, unused)]

use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Parse(ParseErr),
    Check(CheckErr),
    Solve(SolveErr),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParseErr {
    InvalidNumber(String),
    InvalidToken(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CheckErr {
    UnbalancedBlocks(Vec<Token>),
    ExprWithNoResult(Vec<TokenType>),
    InvalidAdiacents(Vec<Token>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SolveErr {}

impl From<ParseErr> for Error {
    fn from(value: ParseErr) -> Self {
        Self::Parse(value)
    }
}

impl From<CheckErr> for Error {
    fn from(value: CheckErr) -> Self {
        Self::Check(value)
    }
}

impl From<SolveErr> for Error {
    fn from(value: SolveErr) -> Self {
        Self::Solve(value)
    }
}
