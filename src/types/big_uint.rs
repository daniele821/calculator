#![allow(dead_code, unused)]

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug, Clone)]
pub struct BigUInt {
    num: Vec<u8>,
}

impl BigUInt {
    pub fn from(num: Vec<u8>) -> Self {
        if num.is_empty() {
            return Self { num: vec![0] };
        }
        let mut res = Self { num };
        res.strip_leading_zeros();
        res
    }

    pub fn compare(&self, compared: &BigUInt) -> Ordering {
        let this = &self.num;
        let other = &compared.num;
        let max_len: usize = usize::max(this.len(), other.len());
        for i in (0..max_len).rev() {
            match this.get(i).unwrap_or(&0).cmp(other.get(i).unwrap_or(&0)) {
                Ordering::Equal => (),
                order => return order,
            }
        }
        Ordering::Equal
    }

    pub fn sum(&self, adder: &BigUInt) -> BigUInt {
        let this = &self.num;
        let other = &adder.num;
        let max_len: usize = usize::max(this.len(), other.len());
        let mut res = Vec::with_capacity(max_len);
        let mut carry = false;
        for i in 0..max_len {
            let val_1 = this.get(i).unwrap_or(&0);
            let val_2 = other.get(i).unwrap_or(&0);
            let (sum, overflowed) = val_1.overflowing_add(*val_2);
            res.push(sum + if carry { 1 } else { 0 });
            carry = overflowed;
        }
        if carry {
            res.push(1);
        }
        BigUInt::from(res)
    }

    pub fn abs_sub(&self, sub: &BigUInt) -> BigUInt {
        match self.compare(sub) {
            Ordering::Equal => BigUInt::from(vec![0]),
            Ordering::Less => sub.unsafe_sub(self),
            Ordering::Greater => self.unsafe_sub(sub),
        }
    }

    pub fn safe_sub(&self, sub: &BigUInt) -> Option<BigUInt> {
        match self.compare(sub) {
            Ordering::Equal => Some(BigUInt::from(vec![0])),
            Ordering::Less => None,
            Ordering::Greater => Some(self.unsafe_sub(sub)),
        }
    }
}

impl Add for BigUInt {
    type Output = BigUInt;
    fn add(self, rhs: Self) -> Self::Output {
        self.sum(&rhs)
    }
}

impl AddAssign for BigUInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.sum(&rhs);
    }
}

impl Sub for BigUInt {
    type Output = BigUInt;
    fn sub(self, rhs: Self) -> Self::Output {
        self.unsafe_sub(&rhs)
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.unsafe_sub(&rhs)
    }
}

impl Add for &BigUInt {
    type Output = BigUInt;
    fn add(self, rhs: Self) -> Self::Output {
        self.sum(rhs)
    }
}

impl AddAssign for &mut BigUInt {
    fn add_assign(&mut self, rhs: Self) {
        **self = self.sum(rhs);
    }
}

impl Sub for &BigUInt {
    type Output = BigUInt;
    fn sub(self, rhs: Self) -> Self::Output {
        self.safe_sub(rhs).expect("result is negative")
    }
}

impl SubAssign for &mut BigUInt {
    fn sub_assign(&mut self, rhs: Self) {
        **self = self.safe_sub(rhs).expect("result is negative");
    }
}

impl Default for BigUInt {
    fn default() -> Self {
        BigUInt::from(vec![0])
    }
}

impl PartialEq for BigUInt {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl BigUInt {
    fn strip_leading_zeros(&mut self) {
        let mut i = self.num.len();
        loop {
            if i <= 1 || self.num.get(i - 1).unwrap_or(&0) != &0u8 {
                break;
            }
            i -= 1;
        }
        self.num.truncate(i);
    }

    fn unsafe_sub(&self, sub: &BigUInt) -> BigUInt {
        let max = &self.num;
        let min = &sub.num;
        let max_len: usize = usize::max(max.len(), min.len());
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
        let mut res = BigUInt::from(res);
        res.strip_leading_zeros();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let num3 = BigUInt::from(vec![12, 34]);
        assert!(num1 < num2);
        assert!(num1 > num3);
        assert!(num2 > num3);
        assert!(num1 == num1);
    }

    #[test]
    fn sum() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let sum = BigUInt::from(vec![24, 68, 126, 1]);
        assert!(num1 + num2 == sum);
    }

    #[test]
    #[should_panic]
    fn sub() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let sum = BigUInt::from(vec![24, 68, 126, 1]);
        assert!(&sum - &num1 == num2);
        assert!(&sum - &num2 == num1);
        let should_panic = &num1 - &sum;
    }
}
