#![allow(unused, dead_code)]

use super::big_uint::BigUInt;

#[derive(Debug)]
pub struct BigInt {
    num: BigUInt,
    sign: isize,
}

impl BigInt {
    pub fn from(num: BigUInt, sign: isize) -> BigInt {
        if sign != -1 && sign != 1 {
            panic!("sign can only be -1 or 1 (initialized with \"{sign}\" instead)");
        }
        BigInt { num, sign }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
