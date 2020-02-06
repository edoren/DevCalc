use std::any;
use std::fmt;

use crate::token::Token;
use crate::number::Number;

//////////////////////////////////////////////////////////////////////
/// Operator
//////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum Operator {
    SUM,
    SUB,
    AND,
    XOR,
    OR,
    SHIFTL,
    SHIFTR,
    LPARENTHESIS,
    RPARENTHESIS
}

impl Operator {
    pub fn from_bytes(input: &[u8]) -> Option<Operator> {
        let mut op: Option<Operator> = None;
        match input[0] {
            43 => op = Some(Operator::SUM),
            45 => op = Some(Operator::SUB),
            38 => op = Some(Operator::AND),
            94 => op = Some(Operator::XOR),
            124 => op = Some(Operator::OR),
            60 | 62 => {
                if input.len() > 1 {
                    if input[1] == input[0] {
                        op = if input[1] == 60 { Some(Operator::SHIFTL) } else { Some(Operator::SHIFTR) };
                    }
                }
            }
            40 => op = Some(Operator::LPARENTHESIS),
            41 => op = Some(Operator::RPARENTHESIS),
            _ => {},
        };
        return op;
    }
    pub fn get_precedence(&self) -> u32 {
        match self {
            Self::SUM => 9,
            Self::SUB => 9,
            Self::AND => 7,
            Self::XOR => 6,
            Self::OR => 5,
            Self::SHIFTL => 8,
            Self::SHIFTR => 8,
            Self::LPARENTHESIS => 0,
            Self::RPARENTHESIS => 0,
        }
    }
    pub fn is_left_associative(&self) -> bool {
        match self {
            Self::SUM | Self::SUB | Self::AND | Self::XOR | Self::OR | Self::SHIFTL | Self::SHIFTR => true,
            Self::LPARENTHESIS | Self::RPARENTHESIS => false
        }
    }
    pub fn operate(&self, a: Number, b: Number) -> Option<Number> {
        match self {
            Self::SUM => Some(a + b),
            Self::SUB => Some(a - b),
            Self::AND => Some(a & b),
            Self::XOR => Some(a ^ b),
            Self::OR => Some(a | b),
            Self::SHIFTL => Some(a << b),
            Self::SHIFTR => Some(a >> b),
            _ => None
        }
    }
}

impl Token for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::SUM => String::from("+"),
            Self::SUB => String::from("-"),
            Self::AND => String::from("&"),
            Self::XOR => String::from("^"),
            Self::OR => String::from("|"),
            Self::SHIFTL => String::from("<<"),
            Self::SHIFTR => String::from(">>"),
            Self::LPARENTHESIS => String::from("("),
            Self::RPARENTHESIS => String::from(")"),
        }
    }
    fn as_any(&self) -> &dyn any::Any { self }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Token::to_string(self))
    }
}
