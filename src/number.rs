use std::any;
use std::fmt;
use std::ops;

use crate::token::Token;

//////////////////////////////////////////////////////////////////////
/// Number
//////////////////////////////////////////////////////////////////////

fn get_number_from_ascii(ch: u8) -> u8 {
    if ch >= 48 && ch <= 57 {
        return ch - 48;
    }
    if ch == 65 || ch == 97 { return 10 } // A
    if ch == 66 || ch == 98 { return 11 } // B
    if ch == 67 || ch == 99 { return 12 } // C
    if ch == 68 || ch == 100 { return 13 } // D
    if ch == 69 || ch == 101 { return 14 } // E
    if ch == 70 || ch == 102 { return 15 } // F
    return 255;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberBase {
    BIN = 0,
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

#[derive(Copy, Clone)]
pub struct Number {
    value: u64,
    base: NumberBase
}

impl Number {
    fn from(value: u64) -> Number {
        Number { value: value, base: NumberBase::DEC }
    }
    pub fn from_slice(slice: &[u8], base: &NumberBase) -> Number {
        let mut result: u64 = 0;
        let mut power = slice.len();
        for val in slice {
            power -= 1;
            let ch = get_number_from_ascii(*val) as u64;
            match base {
                NumberBase::BIN => result += ch * 2u64.pow(power as u32),
                NumberBase::OCT => result += ch * 8u64.pow(power as u32),
                NumberBase::DEC => result += ch * 10u64.pow(power as u32),
                NumberBase::HEX => result += ch * 16u64.pow(power as u32),
            }
        }
        Number { value: result, base: *base }
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
        Number::from(ops::Add::add(self.value, rhs.value))
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Number {
        Number::from(ops::Sub::sub(self.value, rhs.value))
    }
}

impl ops::Shl<Number> for Number {
    type Output = Number;

    fn shl(self, rhs: Number) -> Number {
        Number::from(ops::Shl::shl(self.value, rhs.value))
    }
}

impl ops::Shr<Number> for Number {
    type Output = Number;

    fn shr(self, rhs: Number) -> Number {
        Number::from(ops::Shr::shr(self.value, rhs.value))
    }
}

impl ops::BitAnd<Number> for Number {
    type Output = Number;

    fn bitand(self, rhs: Number) -> Number {
        Number::from(ops::BitAnd::bitand(self.value, rhs.value))
    }
}

impl ops::BitOr<Number> for Number {
    type Output = Number;

    fn bitor(self, rhs: Number) -> Number {
        Number::from(ops::BitOr::bitor(self.value, rhs.value))
    }
}

impl ops::BitXor<Number> for Number {
    type Output = Number;

    fn bitxor(self, rhs: Number) -> Number {
        Number::from(ops::BitXor::bitxor(self.value, rhs.value))
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
            NumberBase::HEX => fmt::LowerHex::fmt(&self.value, f),
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
