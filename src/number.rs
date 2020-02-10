extern crate num_bigint;
extern crate num_traits;

use std::any;
use std::fmt;
use std::ops;

use num_bigint::{BigUint};
use num_traits::{Zero, ToPrimitive};
use crate::token::Token;

//////////////////////////////////////////////////////////////////////
/// Number
//////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberBase {
    BIN = 2,
    OCT = 8,
    DEC = 10,
    HEX = 16,
}

impl NumberBase {
    pub fn to_string(&self) -> &str {
        match self {
            Self::BIN => return "BIN",
            Self::OCT => return "OCT",
            Self::DEC => return "DEC",
            Self::HEX => return "HEX"
        }
    }
}

#[derive(Clone)]
pub struct Number {
    value: BigUint,
    base: NumberBase
}

impl Number {
    pub fn from_slice(slice: &[u8], base: &NumberBase) -> Number {
        if let Some(result) = BigUint::parse_bytes(slice, *base as u32) {
            Number { value: result, base: *base }
        } else {
            println!("Error parsing number from slice");
            Number { value: BigUint::zero(), base: *base }
        }
    }
}

impl Token for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn as_any(&self) -> &dyn any::Any { self }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        Number { value: self.value + rhs.value, base: self.base }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        Number { value: self.value - rhs.value, base: self.base }
    }
}

impl ops::Shl<Number> for Number {
    type Output = Number;

    fn shl(self, rhs: Number) -> Number {
        Number { value: self.value << rhs.value.to_usize().unwrap(), base: self.base }
    }
}

impl ops::Shr<Number> for Number {
    type Output = Number;

    fn shr(self, rhs: Number) -> Number {
        Number { value: self.value >> rhs.value.to_usize().unwrap(), base: self.base }
    }
}

impl ops::BitAnd<Number> for Number {
    type Output = Number;

    fn bitand(self, rhs: Number) -> Number {
        Number { value: self.value & rhs.value, base: self.base }
    }
}

impl ops::BitXor<Number> for Number {
    type Output = Number;

    fn bitxor(self, rhs: Number) -> Number {
        Number { value: self.value ^ rhs.value, base: self.base }
    }
}

impl ops::BitOr<Number> for Number {
    type Output = Number;

    fn bitor(self, rhs: Number) -> Number {
        Number { value: self.value | rhs.value, base: self.base }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.base {
            NumberBase::BIN => fmt::Binary::fmt(&self.value, f),
            NumberBase::OCT => fmt::Octal::fmt(&self.value, f),
            NumberBase::DEC => fmt::Display::fmt(&self.value, f),
            NumberBase::HEX => fmt::UpperHex::fmt(&self.value, f),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.value, f)
    }
}

impl fmt::Octal for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}

impl fmt::LowerHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}

impl fmt::UpperHex for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.value, f)
    }
}
