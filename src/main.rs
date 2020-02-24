mod number;
mod operator;
mod token;
extern crate clap;

use std::collections::VecDeque;

use clap::{App, Arg, value_t};

use number::{Number, NumberBase};
use operator::Operator;
use token::Token;

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

//////////////////////////////////////////////////////////////////////
/// Shunting Yard
//////////////////////////////////////////////////////////////////////

fn shunting_yard(input: Vec<u8>) -> Result<VecDeque<Box<dyn Token>>, String> {
    let mut error_msg = String::new();
    let mut start;
    let mut end = 0;

    let mut tokens = VecDeque::<Box<dyn Token>>::new();
    let mut operator_stack = Vec::<Operator>::new();

    loop {
        start = end;

        if !error_msg.is_empty() {
            println!("Error parsing in column {}: {}", end, error_msg);
            println!("{}", String::from_utf8(input).unwrap());
            println!("{0}^\n{0}Error here", " ".repeat(end - 1));
            return Err("Parsing Error".to_owned());
        }

        if start >= input.len() {
            break;
        }
        end = start + 1;

        // Check for space
        if input[start] == 32 {
            continue;
        }

        // Check for UTF-8 non breaking space
        if input[start] == 194 && end < input.len() && input[end] == 160 {
            end += 1;
            continue;
        }

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
                let next = input[end];
                let is_valid;
                match num_base {
                    NumberBase::BIN => is_valid = is_valid_binary_number(next),
                    NumberBase::OCT => is_valid = is_valid_octal_number(next),
                    NumberBase::DEC => is_valid = is_valid_decimal_number(next),
                    NumberBase::HEX => is_valid = is_valid_hexadecimal_number(next),
                }
                if !is_valid {
                    if is_valid_hexadecimal_number(next) {
                        error_msg = "Invalid ".to_owned() + num_base.to_string() + " number";
                        end += 1;
                    }
                    break;
                }
                end += 1;
            }
            if error_msg.is_empty() {
                let num_start = if num_base != NumberBase::DEC {
                    start + 2
                } else {
                    start
                };
                if let Ok(number) = Number::from_slice(&input[num_start..end], &num_base) {
                    tokens.push_back(Box::new(number));
                }
            }
            continue;
        }

        // Check function: TODO

        // Get the next operator
        let op = Operator::from_bytes(&input[start..input.len()]);
        if op == Some(Operator::SHIFTL) || op == Some(Operator::SHIFTR) {
            end += 1;
        }

        // Check operator
        if let Some(operator) = op {
            if operator == Operator::LPARENTHESIS {
                operator_stack.push(operator);
                continue;
            } else if operator == Operator::RPARENTHESIS {
                let mut lparenth_found = false;
                while let Some(top_operator) = operator_stack.last() {
                    if *top_operator != Operator::LPARENTHESIS {
                        tokens.push_back(Box::new(operator_stack.pop().unwrap()));
                    } else {
                        operator_stack.pop();
                        lparenth_found = true;
                        break;
                    }
                }
                if !lparenth_found {
                    error_msg = "Error mismatched parenthesis".to_owned();
                }
                continue;
            } else {
                while let Some(top_operator) = operator_stack.last() {
                    if (
                        // there is a function at the top of the operator stack: TODO

                        // there is an operator at the top of the operator stack with greater precedence
                        (top_operator.get_precedence() > operator.get_precedence()) ||
                        // the operator at the top of the operator stack has equal precedence and the token is left associative
                        (top_operator.get_precedence() == operator.get_precedence() && operator.is_left_associative())
                    ) && *top_operator != Operator::LPARENTHESIS
                    {
                        tokens.push_back(Box::new(operator_stack.pop().unwrap()));
                    } else {
                        break;
                    }
                }
                operator_stack.push(operator);
                continue;
            }
        } else {
            error_msg = "Invalid operator".to_owned();
            continue;
        }
    }

    while operator_stack.len() > 0 {
        // Check for mismatched parenthesis
        let op = operator_stack.pop().unwrap();
        if op == Operator::LPARENTHESIS || op == Operator::RPARENTHESIS {
            println!("Error mismatched parenthesis");
            return Err("Parsing Error".to_owned());
        }
        tokens.push_back(Box::new(op));
    }

    return Ok(tokens);
}

fn postfix_eval(tokens: &VecDeque<Box<dyn Token>>) -> Option<Vec::<(Number, Operator, Number, Number)>> {
    let mut stack = Vec::<Number>::new();
    let mut operations = Vec::<(Number, Operator, Number, Number)>::new();

    for i in 0..tokens.len() {
        let token = tokens[i].as_any();
        if let Some(number) = token.downcast_ref::<Number>() {
            stack.push(number.clone());
        }
        if let Some(operator) = token.downcast_ref::<Operator>() {
            if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                if let Some(result) = operator.operate(a.clone(), b.clone()) {
                    operations.push((a, *operator, b, result.clone()));
                    stack.push(result);
                } else {
                    println!(
                        "Error evaluating expression: {0:#b} ({0:#?}) {1} {2:#b} ({2:#?})",
                        a, operator, b
                    );
                    return None;
                }
            } else {
                println!("Error malformed expression");
                return None;
            }
        }
    }

    if !operations.is_empty() {
        return Some(operations);
    }

    return None;
}

fn main() {
    let matches = App::new("Developer Calculator")
        .version("1.0")
        .author("Manuel Sabogal <mfer32@gmail.com>")
        .about("Developer step by step calculator")
        .arg(
            Arg::with_name("EXPRESSION")
                .help("The expression to evaluate")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("base")
                .short("b")
                .long("base")
                .value_name("BASE")
                .help("Set the base for the output")
                .possible_values(&["bin", "2", "oct", "8", "dec", "10", "hex", "16"])
                .default_value("dec")
                .takes_value(true),
        )
        .get_matches();

    let operation = value_t!(matches.value_of("EXPRESSION"), String).unwrap_or_else(|e| e.exit());
    let output_base = value_t!(matches.value_of("base"), NumberBase).unwrap_or_else(|e| e.exit());

    if let Ok(tokens) = shunting_yard(operation.clone().into_bytes()) {
        if tokens.len() == 1 {
            if let Some(number) = tokens[0].as_any().downcast_ref::<Number>() {
                println!("\nResult: {:#}", number.clone().set_base(&output_base));
                return;
            }
        }

        println!("Executing: {}\n", operation);
        if let Some(mut operations) = postfix_eval(&tokens) {
            for operation in &mut operations {
                let (a, operator, b, result) = operation;
                let line = format!(
                    "{0:#b} {1} {2:#b} = {3:#b} ({0:#} {1} {2:#} = {3:#})",
                    a, operator, b, result.set_base(&output_base)
                );
                println!("{}", line);
            }
            println!("\nResult: {:#}", operations.last().unwrap().3);
        }
    }
}
