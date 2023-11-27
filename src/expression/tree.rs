#![allow(dead_code, unused)]

use super::{Token, TokenValue};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
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
impl From<(Token, &str)> for AST {
    fn from(value: (Token, &str)) -> Self {
        Self::from(TokenValue::from(value))
    }
}

impl From<(TokenValue, AST, AST)> for AST {
    fn from(value: (TokenValue, AST, AST)) -> Self {
        let mut res = Self::from(value.0);
        res.add_left(value.1);
        res.add_left(value.2);
        res
    }
}

impl AST {
    pub fn value(&self) -> &TokenValue {
        &self.value
    }
    pub fn change_value(&mut self, value: TokenValue) {
        self.value = value
    }
    pub fn left(&self) -> Option<&AST> {
        self.left.as_deref()
    }
    pub fn right(&self) -> Option<&AST> {
        self.right.as_deref()
    }
    pub fn add_left(&mut self, left: AST) {
        self.left = Some(Box::new(left));
    }
    pub fn add_right(&mut self, right: AST) {
        self.right = Some(Box::new(right));
    }
    pub fn add_left_token(&mut self, left: TokenValue) {
        self.add_left(AST::from(left));
    }
    pub fn add_right_token(&mut self, right: TokenValue) {
        self.add_right(AST::from(right));
    }
    pub fn remove_left(&mut self) {
        self.left = None;
    }
    pub fn remove_right(&mut self) {
        self.right = None;
    }
    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }
    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }
    pub fn is_leaf(&self) -> bool {
        !self.has_left() && !self.has_right()
    }
}
