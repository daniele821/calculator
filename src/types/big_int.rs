#![allow(dead_code, unused_variables)]
use std::cmp::Ordering;

/// - numbers are stored in base 256 as a vector of u8 values
/// - vector is read left to right, is the rightmost value is in position 0
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigInt {
    num: Vec<u8>,
    sign: isize,
}

impl BigInt {
    pub fn from(num: Vec<u8>, sign: isize) -> BigInt {
        if sign != -1 && sign != 1 {
            panic!("sign can only be -1 or 1 (initialized with \"{sign}\" instead)");
        }
        if num.is_empty() {
            return BigInt { num: vec![0], sign };
        }
        BigInt { num, sign }
    }
}

impl BigInt {
    fn add(num1: &[u8], num2: &[u8]) -> Vec<u8> {
        let max_len: usize = usize::max(num1.len(), num2.len());
        let mut res = Vec::with_capacity(max_len);
        let mut carry = false;

        for i in 0..max_len {
            let val_1 = num1.get(i).unwrap_or(&0);
            let val_2 = num2.get(i).unwrap_or(&0);
            let (sum, overflowed) = val_1.overflowing_add(*val_2);
            if carry {
                // should not overflow, but be careful, as it may be a cause of problems!
                res.push(sum + 1);
            } else {
                res.push(sum);
            }
            carry = overflowed;
        }
        if carry {
            res.push(1u8);
        }
        res
    }

    /// if max isn't the actual bigger value, result is incorrect
    /// make sure max is the bigger value, min is the smallest value
    fn sub(max: &[u8], min: &[u8]) -> Vec<u8> {
        let max_len: usize = usize::max(min.len(), max.len());
        let mut res = Vec::with_capacity(max_len);
        let mut carry = false;

        for i in 0..max_len {
            let val_max = max.get(i).unwrap_or(&0);
            let val_min = min.get(i).unwrap_or(&0);
            let val_min = val_min.wrapping_add(if carry { 1 } else { 0 });
            let (sub, underflowed) = val_max.overflowing_sub(val_min);
            res.push(sub);
            carry = underflowed;
        }
        res
    }

    fn compare(num1: &[u8], num2: &[u8]) -> Ordering {
        let max_len: usize = usize::max(num1.len(), num2.len());
        for i in (0..max_len).rev() {
            match num1.get(i).unwrap_or(&0).cmp(num2.get(i).unwrap_or(&0)) {
                Ordering::Equal => (),
                order => return order,
            }
        }
        Ordering::Equal
    }

    fn strip_leading_zeros(num: &mut Vec<u8>) {
        let mut i = num.len();
        loop {
            if i <= 1 || num.get(i - 1).unwrap_or(&0) != &0u8 {
                break;
            }
            i -= 1;
        }
        num.truncate(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp() {
        let num1: &[u8] = &[12, 34, 127];
        let num2: &[u8] = &[12, 34, 255];
        let num3: &[u8] = &[12, 34];
        assert_eq!(BigInt::compare(num1, num2), Ordering::Less);
        assert_eq!(BigInt::compare(num1, num3), Ordering::Greater);
        assert_eq!(BigInt::compare(num2, num3), Ordering::Greater);
        assert_eq!(BigInt::compare(num1, num1), Ordering::Equal);
    }

    #[test]
    fn add() {
        let num1 = &[12, 34, 127];
        let num2 = &[12, 34, 255];
        let sum = &[24, 68, 126, 1];
        assert_eq!(
            BigInt::compare(&BigInt::add(num1, num2), sum),
            Ordering::Equal
        );
    }

    #[test]
    fn sub() {
        let num1 = &[12, 34, 127];
        let num2 = &[12, 34, 255];
        let sum = &[24, 68, 126, 1];
        assert_eq!(
            BigInt::compare(&BigInt::sub(sum, num1), num2),
            Ordering::Equal
        );
        assert_eq!(
            BigInt::compare(&BigInt::sub(sum, num2), num1),
            Ordering::Equal
        );
    }

    #[test]
    fn strip_leading_zeros() {
        let num1 = vec![12, 34, 64, 0, 0];
        let num2 = vec![12, 34, 64];
        let res = vec![12, 34, 64];
        let mut strip1 = num1.clone();
        let mut strip2 = num2.clone();
        BigInt::strip_leading_zeros(&mut strip1);
        BigInt::strip_leading_zeros(&mut strip2);
        assert_eq!(res, strip1);
        assert_eq!(res, strip2);
    }
}
