use std::ops::Add;

use fraction::{BigUint, GenericFraction, Integer};

pub fn disp(first: &BigUint, last: &BigUint) -> BigUint {
    if first == last {
        return first.clone();
    }
    let mid: BigUint = first + (last - first) / 2u64;
    disp(first, &mid) * disp(&mid.add(1u64), last)
}

pub fn disp_small(first: u64, last: u64) -> BigUint {
    if first == last {
        return BigUint::from(first);
    }
    let mid = first + (last - first) / 2;
    disp_small(first, mid) * disp_small(mid + 1, last)
}

pub fn is_integer<T: Integer + Clone>(num: &GenericFraction<T>) -> bool {
    match num {
        fraction::GenericFraction::Rational(_, ratio) => ratio.is_integer(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
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
}
