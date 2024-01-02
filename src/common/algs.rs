use crate::expression::{
    error::{Error, SolveErr},
    token::{BinaryOp, Token, UnaryOpRight},
};
use fraction::{BigFraction, BigUint, GenericFraction, Ratio, Sign, Zero};

pub fn disp(first: u64, last: u64) -> BigUint {
    if first == last {
        return BigUint::from(first);
    }
    let mid = first + (last - first) / 2;
    disp(first, mid) * disp(mid + 1, last)
}

pub fn to_ratio(num: &BigFraction) -> Option<Ratio<BigUint>> {
    match num {
        fraction::GenericFraction::Rational(_, ratio) => Some(ratio.clone()),
        _ => None,
    }
}

pub fn to_i32(num: &BigFraction) -> Option<i32> {
    match num {
        fraction::GenericFraction::Rational(sign, ratio) => {
            if !ratio.is_integer() {
                return None;
            }
            let digits = ratio.numer().clone().to_u32_digits();
            let first = digits.first().unwrap_or(&0);
            if digits.len() > 1 || first > &(i32::MAX as u32) {
                return None;
            }
            let sign = if sign == &Sign::Plus { 1 } else { -1 };
            Some(*first as i32 * sign)
        }
        _ => None,
    }
}

pub fn to_u64(num: &BigFraction) -> Option<u64> {
    match num {
        fraction::GenericFraction::Rational(sign, ratio) => {
            if !ratio.is_integer() {
                return None;
            }
            if sign == &Sign::Minus {
                return None;
            }
            if ratio.numer() > &BigUint::from(u64::MAX) {
                return None;
            }
            let digits = ratio.numer().to_u64_digits();
            let first = digits.first().unwrap_or(&0);
            Some(*first)
        }
        _ => None,
    }
}

pub fn exp(base: &BigFraction, exp: &BigFraction) -> Result<BigFraction, Error> {
    let err = || {
        SolveErr::OperIllegalValues(vec![
            Token::Number(base.clone()),
            Token::from(BinaryOp::Exp),
            Token::Number(exp.clone()),
        ])
    };
    if base.is_nan() || base.is_infinite() || exp.is_nan() || exp.is_infinite() {
        None.ok_or_else(err)?;
    }
    let exp_ = to_i32(exp).ok_or_else(err)?;
    let base_ = to_ratio(base).ok_or_else(err)?;
    let res = base_.pow(exp_);
    let sign = match (
        base.is_sign_positive(),
        (exp % BigUint::from(2u32)).is_zero(),
    ) {
        (true, true) => Sign::Plus,
        (true, false) => Sign::Plus,
        (false, true) => Sign::Plus,
        (false, false) => Sign::Minus,
    };
    Ok(BigFraction::Rational(sign, res))
}

pub fn fact(num: &BigFraction) -> Result<BigFraction, Error> {
    if num.is_zero() {
        return Ok(BigFraction::from(1));
    }
    let err = || {
        SolveErr::OperIllegalValues(vec![
            Token::Number(num.clone()),
            Token::from(UnaryOpRight::Fact),
        ])
    };
    match num {
        GenericFraction::Rational(_, _) => (),
        _ => None.ok_or_else(err)?,
    }
    let num_ = to_u64(num).ok_or_else(err)?;
    Ok(BigFraction::from(disp(1u64, num_)))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_disp() {
        let actual1 = disp(4u64, 6u64);
        let expected1 = BigUint::from(120u64);
        assert_eq!(actual1, expected1);
    }

    #[test]
    fn test_exp() -> Result<(), Error> {
        let actual1 = exp(
            &BigFraction::from_str("2.5").unwrap(),
            &BigFraction::from(4),
        )?;
        let expected1 = BigFraction::from_str("39.0625").unwrap();
        assert_eq!(actual1, expected1);
        Ok(())
    }

    #[test]
    fn test_fact() -> Result<(), Error> {
        let actual1 = fact(&BigFraction::from(10))?;
        let expected1 = BigFraction::from(3628800);
        assert_eq!(actual1, expected1);
        Ok(())
    }
}
