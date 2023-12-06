#![allow(dead_code)]

use std::{
    fmt::{self, Display},
    io::{self, IsTerminal},
};

use fraction::{BigUint, Fraction};

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

pub fn disp(first: &BigUint, last: &BigUint) -> BigUint {
    if first == last {
        return first.clone();
    }
    let mid: BigUint = first + (last - first) / 2u64;
    disp(first, &mid) * disp(&(mid + 1u64), last)
}

pub fn disp_small(first: u64, last: u64) -> BigUint {
    if first == last {
        return BigUint::from(first);
    }
    let mid = first + (last - first) / 2;
    disp_small(first, mid) * disp_small(mid + 1, last)
}

pub fn is_integer(num: &Fraction) -> bool {
    match num {
        fraction::GenericFraction::Rational(_, ratio) => ratio.is_integer(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_product() {
        let actual1 = disp(&BigUint::from(4u64), &BigUint::from(6u64));
        let expected1 = BigUint::from(120u64);
        assert_eq!(actual1, expected1);
    }

    #[test]
    fn test_product_small() {
        let actual1 = disp_small(4u64, 6u64);
        let expected1 = BigUint::from(120u64);
        assert_eq!(actual1, expected1);
    }

    #[test]
    fn test_is_integer() {
        let int = Fraction::from_str("34/2").unwrap();
        let rat = Fraction::from_str("34.3").unwrap();
        let nan = Fraction::NaN;
        let inf = Fraction::infinity();
        assert!(is_integer(&int));
        assert!(!is_integer(&rat));
        assert!(!is_integer(&nan));
        assert!(!is_integer(&inf));
    }
}
