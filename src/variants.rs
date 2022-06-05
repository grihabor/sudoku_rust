use std::convert::{TryFrom, TryInto};
use crate::digit::Digit;
use std::fmt::Formatter;
use std::ops::{BitOr, BitXor};
use std::{cmp, fmt, ops};

pub const NUM_BITS: usize = 9;
pub const MASK: usize = (1 << NUM_BITS) - 1;
pub const ANY: Variants = Variants(MASK as u16);

#[derive(Copy, Clone)]
pub struct Variants(u16);

impl fmt::Debug for Variants {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl cmp::PartialEq for Variants {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Variants {
    fn new() -> Variants {
        Variants(0)
    }
    fn count(self) -> u32 {
        self.0.count_ones()
    }
    fn has_digit(self, d: Digit) -> bool {
        self.0 & Variants::from(d).0 != 0
    }
}

impl From<Digit> for Variants {
    fn from(d: Digit) -> Self {
        Variants(0x1 << u8::from(d))
    }
}

impl TryInto<Digit> for Variants {
    type Error = &'static str;

    fn try_into(self) -> Result<Digit, Self::Error> {
        match self.count() {
            0 => Err("no variants"),
            1 => Ok(Digit::try_from(self.0.trailing_zeros()).unwrap()),
            _ => Err("multiple variants"),
        }
    }
}

impl TryFrom<u64> for Variants {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value & (MASK as u64) == value {
            Ok(Variants(value as u16))
        } else {
            Err("value too large")
        }
    }
}

impl From<Variants> for u64 {
    fn from(v: Variants) -> Self {
        v.0 as u64
    }
}

impl ops::BitOr<Variants> for Variants {
    type Output = Variants;

    fn bitor(self, rhs: Variants) -> Self::Output {
        Variants(self.0 | rhs.0)
    }
}

impl ops::BitOr<Digit> for Variants {
    type Output = Variants;

    fn bitor(self, rhs: Digit) -> Self::Output {
        self | Variants::from(rhs)
    }
}

impl ops::BitOrAssign<Digit> for Variants {
    fn bitor_assign(&mut self, rhs: Digit) {
        *self = *self | rhs
    }
}

impl ops::BitXor<Variants> for Variants {
    type Output = Variants;

    fn bitxor(self, rhs: Variants) -> Self::Output {
        Variants(self.0 ^ rhs.0)
    }
}

impl ops::BitXorAssign<Variants> for Variants {
    fn bitxor_assign(&mut self, rhs: Variants) {
        *self = *self ^ rhs
    }
}

impl ops::BitXor<Digit> for Variants {
    type Output = Variants;

    fn bitxor(self, rhs: Digit) -> Self::Output {
        self.bitxor(Variants::from(rhs))
    }
}

impl ops::BitXorAssign<Digit> for Variants {
    fn bitxor_assign(&mut self, rhs: Digit) {
        *self = *self ^ rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::digit;
    use crate::digit::Digit;
    use crate::variants::Variants;

    #[test]
    fn test_count() {
        let mut v = Variants::new();
        assert_eq!(v.count(), 0);
        v |= digit::ONE;
        assert_eq!(v.count(), 1);
        v |= digit::NINE;
        assert_eq!(v.count(), 2);
        v |= digit::ONE; // already there
        assert_eq!(v.count(), 2);
    }
}
