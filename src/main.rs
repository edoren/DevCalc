mod token;
use token::Token;

use std::collections::VecDeque;
use std::env;
use std::fmt;

//////////////////////////////////////////////////////////////////////
/// Utils Common
//////////////////////////////////////////////////////////////////////

fn is_valid_binary_number(ch: u8) -> bool {
    ch == 48 || ch == 49
}
fn is_valid_octal_number(ch: u8) -> bool {
    ch >= 48 && ch <= 55
}
fn is_valid_decimal_number(ch: u8) -> bool {
    ch >= 48 && ch <= 57
}
fn is_valid_hexadecimal_number(ch: u8) -> bool {
    is_valid_decimal_number(ch) || (ch >= 65 && ch <= 70) || (ch >= 97 && ch <= 102)
}
fn is_an_operator(ch: u8) -> bool {
    // match + - & | < > ( ) SPACE
    match ch {
        43 | 45 | 38 | 124 | 60 | 62 | 40 | 41 | 32 => return true,
        _ => return false
    };
}
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

//////////////////////////////////////////////////////////////////////
/// Operator
//////////////////////////////////////////////////////////////////////

enum Operator {
    SUM,
    SUB,
    AND,
    OR,
    SHIFTL,
    SHIFTR,
    UNKNOWN,
}

impl Operator {
    fn from_bytes(val: &[u8]) -> Operator {
        let mut op = Self::UNKNOWN;
        if val.len() == 1 {
            match val {
                [43] => op = Self::SUM,
                [45] => op = Self::SUB,
                [38] => op = Self::AND,
                [124] => op = Self::OR,
                _ => {}
            };
        } else if val.len() == 2 {
            match val {
                [60, 60] => op = Self::SHIFTL,
                [62, 62] => op = Self::SHIFTR,
                _ => {}
            };
        }
        return op;
    }
}

impl Token for Operator {
    fn to_string(&self) -> String {
        match self {
            Self::SUM => String::from("+"),
            Self::SUB => String::from("-"),
            Self::AND => String::from("&"),
            Self::OR => String::from("|"),
            Self::SHIFTL => String::from("<<"),
            Self::SHIFTR => String::from(">>"),
            _ => String::from("UNKNOWN"),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Token::to_string(self))
    }
}

//////////////////////////////////////////////////////////////////////
/// Number
//////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq, Eq)]
enum NumberBase {
    BIN = 0,
    OCT = 8,
    DEC = 10,
    HEX = 16,
}

impl NumberBase {
    fn to_string(&self) -> &str {
        match self {
            Self::BIN => return "BIN",
            Self::OCT => return "OCT",
            Self::DEC => return "DEC",
            Self::HEX => return "HEX"
        }
    }
}

impl fmt::Debug for NumberBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", NumberBase::to_string(self))
    }
}

pub struct Number {
    value: u64,
    base: NumberBase
}

impl Number {
    fn from_slice(slice: &[u8], base: &NumberBase) -> Number {
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
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", Token::to_string(self), self.base.to_string())
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Token::to_string(self))
    }
}

//////////////////////////////////////////////////////////////////////
/// Shunting Yard
//////////////////////////////////////////////////////////////////////

fn shunting_yard(input: Vec<u8>) -> bool {
    let mut error_msg: String = String::new();
    let mut start = 0;
    let mut end;
    loop {
        if start >= input.len() { break; }
        end = start + 1;

        // Check for number
        if is_valid_decimal_number(input[start]) {
            let mut num_base: NumberBase = NumberBase::DEC;
            // Check if input[start] == "0"
            if input[start] == 48 && end < input.len() {
                let next: u8 = input[end];
                // Check for b and B
                if next == 98 || next == 66 {
                    num_base = NumberBase::BIN;
                }
                // Check for o and O
                else if next == 111 || next == 79 {
                    num_base = NumberBase::OCT;
                }
                // Check for x and X
                else if next == 120 || next == 88 {
                    num_base = NumberBase::HEX;
                }
                if num_base != NumberBase::DEC {
                    end += 1;
                }
            }
            loop {
                if end >= input.len() {
                    break;
                }
                let next: u8 = input[end];
                let is_valid;
                match num_base {
                    NumberBase::BIN => is_valid = is_valid_binary_number(next),
                    NumberBase::OCT => is_valid = is_valid_octal_number(next),
                    NumberBase::DEC => is_valid = is_valid_decimal_number(next),
                    NumberBase::HEX => is_valid = is_valid_hexadecimal_number(next),
                }
                if !is_valid {
                    if !is_an_operator(next) {
                        error_msg = "Invalid ".to_owned() + num_base.to_string() + " number";
                    }
                    break
                }
                end += 1;
            }
            if error_msg.is_empty() {
                let num_start = if num_base != NumberBase::DEC { start + 2 } else { start };
                let number = Number::from_slice(&input[num_start..end], &num_base);
                println!("Number: {:?}", number);
            }
        }

        if !error_msg.is_empty() {
            println!();
            println!("Error parsing in column {}: {}", end, error_msg);
            println!("{}", String::from_utf8(input).unwrap());
            println!("{0}^\n{0}Error here", " ".repeat(end));
            return false;
        }

        start = end;
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation: String = args[1].clone();
    println!("Operation: {}", operation);
    shunting_yard(operation.into_bytes());
}
