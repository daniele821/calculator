#![allow(dead_code)]

use std::{
    fmt::{self, Display},
    io::{self, IsTerminal},
    ops::Add,
};

use fraction::BigUint;

pub enum Color {
    /// success
    SUC,
    /// failure
    FAI,
    /// title
    TIT,
    /// sub-title
    SUB,
    /// other
    OTH,
}

pub fn color<T: Display + ?Sized>(color: &Color, str: &T) -> String {
    let str = str.to_string();
    if !io::stdout().is_terminal() {
        return str;
    }
    match color {
        Color::SUC => format!("\x1b[1;32m{str}\x1b[0m"),
        Color::FAI => format!("\x1b[1;31m{str}\x1b[0m"),
        Color::TIT => format!("\x1b[1;34m{str}\x1b[0m"),
        Color::SUB => format!("\x1b[1;36m{str}\x1b[0m"),
        Color::OTH => format!("\x1b[1;33m{str}\x1b[0m"),
    }
}

pub fn fmt<T: fmt::Display>(items: &[T], sep: Option<&str>) -> String {
    items
        .iter()
        .map(T::to_string)
        .collect::<Vec<_>>()
        .join(sep.unwrap_or(" "))
}

pub fn convert<T: Clone, F: From<T>>(items: &[T]) -> Vec<F> {
    items.iter().map(|t| F::from(t.clone())).collect()
}

pub fn product_split_halves(first: &BigUint, last: &BigUint) -> BigUint {
    if first == last {
        return first.clone();
    }
    let mid: BigUint = first + (last - first) / 2u64;
    product_split_halves(first, &mid) * product_split_halves(&mid.add(1u64), last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let actual1 = product_split_halves(&BigUint::from(4u64), &BigUint::from(6u64));
        let expected1 = BigUint::from(120u64);
        assert_eq!(actual1, expected1);
    }
}
