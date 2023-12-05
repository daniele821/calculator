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
    ExprWithNoResult(Vec<TokenType>),
    BrokenCheckRule(Vec<Token>, CheckRules),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SolveErr {
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
            CheckErr::ExprWithNoResult(res) => {
                format!("expression has no result '{}'", common::fmt(res, None))
            }
            CheckErr::BrokenCheckRule(a, r) => {
                format!("broken check rule '{r:?}', with '{}'", common::fmt(a, None))
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
