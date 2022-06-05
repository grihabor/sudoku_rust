use std::fmt::{Display, Formatter};
use std::convert::TryFrom;
use std::ops;

pub const ONE: Digit = Digit(0);
pub const TWO: Digit = Digit(1);
pub const THREE: Digit = Digit(2);
pub const FOUR: Digit = Digit(3);
pub const FIVE: Digit = Digit(4);
pub const SIX: Digit = Digit(5);
pub const SEVEN: Digit = Digit(6);
pub const EIGHT: Digit = Digit(7);
pub const NINE: Digit = Digit(8);

#[derive(Copy, Clone, Debug)]
pub struct Digit(u8);

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Digit, Self::Error> {
        let d = value as u8;
        if d >= b'1' && d <= b'9' {
            Ok(Digit((d as u8) - b'1'))
        } else {
            Err("invalid digit")
        }
    }
}

impl TryFrom<u8> for Digit {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 8 {
            Ok(Digit(value))
        } else {
            Err("invalid digit")
        }
    }
}

impl TryFrom<u32> for Digit {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Digit::try_from(value as u8)
    }
}

impl TryFrom<i32> for Digit {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Digit::try_from(value as u8)
    }
}

impl From<Digit> for char {
    fn from(d: Digit) -> char {
        (b'1' + d.0) as char
    }
}

impl From<Digit> for u8 {
    fn from(d: Digit) -> u8 {
        d.0
    }
}

impl From<Digit> for u16 {
    fn from(d: Digit) -> u16 {
        d.0 as u16
    }
}

impl From<Digit> for u64 {
    fn from(d: Digit) -> u64 {
        d.0 as u64
    }
}

impl From<Digit> for i32 {
    fn from(d: Digit) -> i32 {
        d.0 as i32
    }
}

impl From<Digit> for usize {
    fn from(d: Digit) -> Self {
        d.0 as usize
    }
}
