#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigInt {
    num: Vec<u8>,
    sign: isize,
}

impl BigInt {
    fn from(num: Vec<u8>, sign: isize) -> BigInt {
        if sign != -1 && sign != 1 {
            panic!("sign can only be -1 or 1 (initialized with \"{sign}\" instead)");
        }
        BigInt { num, sign }
    }
}

#[cfg(test)]
mod tests {}
