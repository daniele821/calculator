use std::fmt;

use super::TokenValue;

#[derive(Debug)]
pub struct AST {
    value: TokenValue,
    left: Option<Box<AST>>,
    right: Option<Box<AST>>,
}

impl From<TokenValue> for AST {
    fn from(value: TokenValue) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
    }
}
