#![allow(dead_code, unused_variables)]
use std::cmp::Ordering;

/// numbers are stored in base 256 as a vector of u8 values
/// vector is read left to right, is the rightmost value is in position 0
/// # Example
/// vec[1,10,100] represents the number 100-10-1 in base 256
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
        todo!();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let num1: &[u8] = &[12, 34, 127];
        let num2: &[u8] = &[12, 34, 255];
        assert_eq!(BigInt::add(num1, num2), vec![24, 68, 126, 1]);
    }

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
}
