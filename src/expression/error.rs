#![allow(dead_code, unused)]

use super::{
    solver::CheckRules,
    token::{Token, TokenType},
};
use crate::common;
use std::fmt::Display;

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
    BrokenCheckRule(CheckRules),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SolveErr {
    ExprWithNoResult(Vec<Token>),
    OperIllegalValues(Vec<Token>),
}

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

impl Display for SolveErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            SolveErr::OperIllegalValues(tokens) => {
                format!("invalid operation '{}'", common::fmt(tokens, None))
            }
            SolveErr::ExprWithNoResult(tokens) => {
                format!("expression has no results '{}'", common::fmt(tokens, None))
            }
        };
        write!(f, "{err}")
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            ParseErr::InvalidNumber(num) => format!("invalid number '{num}'"),
            ParseErr::InvalidToken(tok) => format!("invalid token '{tok}'"),
        };
        write!(f, "{err}")
    }
}

impl Display for CheckErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            CheckErr::UnbalancedBlocks(blocks) => {
                format!("unbalanced blocks '{}'", common::fmt(blocks, None))
            }
            CheckErr::BrokenCheckRule(rule) => {
                format!("broken check rule '{rule:?}'")
            }
        };
        write!(f, "{err}")
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Error::Parse(err) => format!("ParseErr: {err}"),
            Error::Check(err) => format!("CheckErr: {err}"),
            Error::Solve(err) => format!("SolveErr: {err}"),
        };
        write!(f, "{err}")
    }
}
