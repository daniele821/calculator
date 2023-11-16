#![allow(dead_code, unused)]

use std::cmp::Ordering;

#[derive(Debug)]
pub struct BigUInt {
    num: Vec<u8>,
}

impl BigUInt {
    pub fn from(num: Vec<u8>) -> Self {
        if num.is_empty() {
            return Self { num: vec![0] };
        }
        Self { num }
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

    pub fn add(&self, adder: &BigUInt) -> BigUInt {
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

    pub fn sub(&self, sub: &BigUInt) -> BigUInt {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let num3 = BigUInt::from(vec![12, 34]);
        assert_eq!(num1.compare(&num2), Ordering::Less);
        assert_eq!(num1.compare(&num3), Ordering::Greater);
        assert_eq!(num2.compare(&num3), Ordering::Greater);
        assert_eq!(num1.compare(&num1), Ordering::Equal);
    }

    #[test]
    fn add() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let sum = BigUInt::from(vec![24, 68, 126, 1]);
        assert_eq!(num1.add(&num2).compare(&sum), Ordering::Equal);
    }

    #[test]
    fn sub() {
        let num1 = BigUInt::from(vec![12, 34, 127]);
        let num2 = BigUInt::from(vec![12, 34, 255]);
        let sum = BigUInt::from(vec![24, 68, 126, 1]);
        assert_eq!(sum.sub(&num1).compare(&num2), Ordering::Equal);
        assert_eq!(sum.sub(&num2).compare(&num1), Ordering::Equal);
    }

    #[test]
    fn strip_leading_zeros() {
        let mut num1 = BigUInt::from(vec![12, 34, 64, 0, 0]);
        let mut num2 = BigUInt::from(vec![12, 34, 64]);
        let res = vec![12, 34, 64];
        num1.strip_leading_zeros();
        num2.strip_leading_zeros();
        assert_eq!(res, num1.num);
        assert_eq!(res, num2.num);
    }
}
