const BIT_PER_ELEM: usize = 8;

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

    pub fn nth_bit(&self, nth: usize) -> Option<u8> {
        let nth_elem = nth / BIT_PER_ELEM;
        let nth_bit = nth % BIT_PER_ELEM;
        let nth_bit_map = 0x01 << nth_bit;
        self.num
            .get(nth_elem)
            .map(|elem| (elem & nth_bit_map) >> nth_bit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_bit() {
        let num1 = BigInt::from(vec![0b01010101, 0b11001100, 0b11110000], 1);
        assert_eq!(num1.nth_bit(0), Some(1), "bit 0 should be 1");
        assert_eq!(num1.nth_bit(10), Some(1), "bit 10 should be 1");
        assert_eq!(num1.nth_bit(16), Some(0), "bit 16 should be 0");
        assert_eq!(num1.nth_bit(24), None, "bit 24 is out of bound");
    }
}
