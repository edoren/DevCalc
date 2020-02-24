extern crate num_bigint;
extern crate num_traits;

use std::any;
use std::fmt;
use std::ops;

use crate::token::Token;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

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
            Self::HEX => return "HEX",
        }
    }
}

impl std::str::FromStr for NumberBase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" | "2" => Ok(Self::BIN),
            "oct" | "8" => Ok(Self::OCT),
            "dec" | "10" => Ok(Self::DEC),
            "hex" | "16" => Ok(Self::HEX),
            _ => Err("Error parsing".to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct Number {
    value: BigInt,
    base: NumberBase,
}

impl Number {
    pub fn from_slice(slice: &[u8], base: &NumberBase) -> Result<Number, String> {
        if let Some(result) = BigInt::parse_bytes(slice, *base as u32) {
            Ok(Number {
                value: result,
                base: *base,
            })
        } else {
            Err("Error parsing number from slice".to_owned())
        }
    }

    pub fn set_base(&mut self, base: &NumberBase) -> &Number {
        self.base = *base;
        self
    }
}

impl Token for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Number {
        Number {
            value: self.value + rhs.value,
            base: self.base,
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        Number {
            value: self.value - rhs.value,
            base: self.base,
        }
    }
}

impl ops::Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Number {
        Number {
            value: self.value * rhs.value,
            base: self.base,
        }
    }
}

impl ops::Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Number {
        Number {
            value: self.value / rhs.value,
            base: self.base,
        }
    }
}

impl ops::Shl<Number> for Number {
    type Output = Number;

    fn shl(self, rhs: Number) -> Number {
        Number {
            value: self.value << rhs.value.to_usize().unwrap(),
            base: self.base,
        }
    }
}

impl ops::Shr<Number> for Number {
    type Output = Number;

    fn shr(self, rhs: Number) -> Number {
        Number {
            value: self.value >> rhs.value.to_usize().unwrap(),
            base: self.base,
        }
    }
}

impl ops::BitAnd<Number> for Number {
    type Output = Number;

    fn bitand(self, rhs: Number) -> Number {
        Number {
            value: self.value & rhs.value,
            base: self.base,
        }
    }
}

impl ops::BitXor<Number> for Number {
    type Output = Number;

    fn bitxor(self, rhs: Number) -> Number {
        Number {
            value: self.value ^ rhs.value,
            base: self.base,
        }
    }
}

impl ops::BitOr<Number> for Number {
    type Output = Number;

    fn bitor(self, rhs: Number) -> Number {
        Number {
            value: self.value | rhs.value,
            base: self.base,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.base {
            NumberBase::BIN => fmt::Binary::fmt(&self.value, f),
            NumberBase::OCT => fmt::Octal::fmt(&self.value, f),
            NumberBase::DEC => fmt::Display::fmt(&self.value, f),
            NumberBase::HEX => fmt::UpperHex::fmt(&self.value, f),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
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
