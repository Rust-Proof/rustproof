// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#[macro_use]
//extern crate rustc;
//extern crate syntax;
//extern crate rustc_plugin;
extern crate term;
use rustc_plugin::Registry;
use std::fmt;
use std::process;

#[derive(Clone, PartialEq)]
pub struct BinaryExpressionData { pub op: BinaryOperator, pub left: Box<Expression>, pub right: Box<Expression> }

#[derive(Clone, PartialEq)]
pub struct UnaryExpressionData { pub op: UnaryOperator, pub e: Box<Expression> }

// Boolean Expression type
#[derive(Clone, PartialEq)]
pub enum Expression {
    // Boolean expressions
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    // A variable; should be either one of a function's formal arguments, a special "return" variable, or something from an encapsulating scope.
    VariableMapping(VariableMappingData),
    // Boolean literals
    BooleanLiteral(bool),
    // Integer literals
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

// Used for representing Expression types as strings, recursively.
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::BinaryExpression (ref b) => {
                write!(f, "({} {} {})", *b.left, b.op, *b.right)
            },
            &Expression::UnaryExpression (ref u) => {
                write!(f, "({} {})", u.op, *u.e)
            },
            &Expression::VariableMapping (ref v) => {
                write!(f, "({} : {})", v.name, v.var_type)
            }
            &Expression::BooleanLiteral (ref b) => {
                write!(f, "({})", b)
            },
            &Expression::UnsignedBitVector(ref u) => {
                write!(f, "({} : u{})", u.value, u.size.to_string())
            },
            &Expression::SignedBitVector(ref s) => {
                write!(f, "({} : i{})", s.value, s.size.to_string())
            }
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug)]
pub struct VariableMappingData { pub name: String, pub var_type: String}

// FIXME: Type checking removed, should be returned later!
// Check equality for VariableMappingData types. Should return true if the name and type of the variables are the same.
impl PartialEq for VariableMappingData {
    fn eq(&self, _rhs: &VariableMappingData) -> bool {
        if (self.name == _rhs.name)  && (self.var_type == _rhs.var_type) {
            true
        } else {
            false
        }
    }
}

// Ensures it is clear that VariableMappingData has full equality.
impl Eq for VariableMappingData {}

impl fmt::Display for VariableMappingData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} : {})", self.name, self.var_type)
    }
}

#[derive(Clone, PartialEq)]
pub struct UnsignedBitVectorData { pub size: u8, pub value: u64 }

#[derive(Clone, PartialEq)]
pub struct SignedBitVectorData { pub size: u8, pub value: i64 }

#[derive(Clone, PartialEq)]
pub enum BinaryOperator {
    // Normal operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    // Bitwise operators
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    // Comparison operators
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    // Boolean logical operators
    And,
    Or,
    Xor,
    Implication,
    BiImplication
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &BinaryOperator::Addition => { write!(f, "+") },
            &BinaryOperator::Subtraction => { write!(f, "-") },
            &BinaryOperator::Multiplication => { write!(f, "*") },
            &BinaryOperator::Division => { write!(f, "/") },
            &BinaryOperator::Modulo => { write!(f, "%") },
            &BinaryOperator::BitwiseOr => { write!(f, "|") },
            &BinaryOperator::BitwiseAnd => { write!(f, "&") },
            &BinaryOperator::BitwiseXor => { write!(f, "^") },
            &BinaryOperator::BitwiseLeftShift => { write!(f, "<<") },
            &BinaryOperator::BitwiseRightShift => { write!(f, ">>") },
            &BinaryOperator::LessThan => { write!(f, "<") },
            &BinaryOperator::LessThanOrEqual => { write!(f, "<=") },
            &BinaryOperator::GreaterThan => { write!(f, ">") },
            &BinaryOperator::GreaterThanOrEqual => { write!(f, ">=") },
            &BinaryOperator::Equal => { write!(f, "==") },
            &BinaryOperator::NotEqual => { write!(f, "!=") },
            &BinaryOperator::And => { write!(f, "AND") },
            &BinaryOperator::Or => { write!(f, "OR") },
            &BinaryOperator::Xor => { write!(f, "XOR") },
            &BinaryOperator::Implication => { write!(f, "IMPLIES") },
            &BinaryOperator::BiImplication => { write!(f, "EQUIV") }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UnaryOperator {
    Negation,
    BitwiseNot,
    // Boolean logical operators
    Not,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &UnaryOperator::Negation => { write!(f, "-") },
            &UnaryOperator::BitwiseNot => { write!(f, "!") },
            &UnaryOperator::Not => { write!(f, "NOT") }
        }
    }
}

// Recurses through an Expression and replaces any Variable Mapping with the given Expression.
pub fn substitute_variable_with_expression ( source_expression: &mut Expression, target: &VariableMappingData, replacement: &Expression ) {
    let mut replace: bool = false;
    match source_expression {
        &mut Expression::BinaryExpression(ref mut b) => {
            // Recurisvely call the sub-expressions
            substitute_variable_with_expression(&mut(*b.left), &target, &replacement);
            substitute_variable_with_expression(&mut(*b.right), &target, &replacement);
        },
        &mut Expression::UnaryExpression(ref mut u) => {
            // Recurisvely call the sub-expression
            substitute_variable_with_expression(&mut(*u.e), &target, &replacement);
        },
        &mut Expression::VariableMapping(ref mut v) => {
            // Substitute the variable if it matches the target
            // FIXME: hotfix inplace here to allow type inference for return blocks
            if v == target {
                replace = true;
            }
        },
        _ => {
            // No substitution should be done
        }
    }

    if replace {
        *source_expression = replacement.clone();
    }
}

// Recurses through an Expression and returns the type it would evaluate to.
pub fn determine_evaluation_type ( expression: &Expression ) -> String {
    match expression {
        &Expression::BinaryExpression(ref b) => {
            match b.op {
                BinaryOperator::Addition | BinaryOperator::Subtraction | BinaryOperator::Multiplication | BinaryOperator::Division | BinaryOperator::Modulo => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);
                    // Ensure both operands are numeric types
                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                        rp_error!("Invalid use of binary operator {} on boolean value(s)", b.op);
                    // Ensure both operand types match
                    } else if l_type != r_type {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        l_type
                    }
                },
                BinaryOperator::BitwiseLeftShift | BinaryOperator::BitwiseRightShift => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);

                    // Ensure both operands are numeric types
                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                        rp_error!("Invalid use of binary operator {} on boolean value(s)", b.op);
                    //Ensure both operand types are of same signedness
                    } else if (l_type.starts_with("i") && r_type.starts_with("i")) || (l_type.starts_with("u") && r_type.starts_with("u")) {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        l_type
                    }
                },
                BinaryOperator::BitwiseOr | BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseXor => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);
                    // Ensure both operand types match
                    if l_type != r_type {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        l_type
                    }
                },
                BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual | BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);
                    // Ensure both operands are numeric types
                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                        rp_error!("Invalid use of binary operator {} on boolean value(s)", b.op);
                    // Ensure both operand types match
                    } else if l_type != r_type {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        "bool".to_string()
                    }
                },
                BinaryOperator::Equal | BinaryOperator::NotEqual => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);
                    // Ensure both operand types match
                    if l_type != r_type {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        "bool".to_string()
                    }
                },
                BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor | BinaryOperator::Implication | BinaryOperator::BiImplication => {
                    let l_type: String = determine_evaluation_type(&*b.left);
                    let r_type: String = determine_evaluation_type(&*b.right);
                    // Ensure both operands are boolean types
                    if (l_type != "bool".to_string()) || (r_type != "bool".to_string()) {
                        rp_error!("Invalid use of binary operator {} on numeric value(s)", b.op);
                    // Ensure both operand types match
                    } else if l_type != r_type {
                        rp_error!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type);
                    } else {
                        "bool".to_string()
                    }
                }
            }
        },
        &Expression::UnaryExpression(ref u) => {
            match u.op {
                UnaryOperator::Negation => {
                    let e_type: String = determine_evaluation_type(&*u.e);
                    // Ensure operand is a numeric type
                    if e_type == "bool".to_string() {
                        rp_error!("Invalid use of operator {} on boolean value {}", u.op, *u.e);
                    // Ensure operand is not an unsigned type
                    } else if (e_type == "u8".to_string()) || (e_type == "u16".to_string()) || (e_type == "u32".to_string()) || (e_type == "u64".to_string()) {
                        rp_error!("Invalid use of operator {} on unsigned value {}", u.op, *u.e);
                    } else {
                        e_type
                    }
                },
                UnaryOperator::BitwiseNot => {
                    determine_evaluation_type(&*u.e)
                },
                UnaryOperator::Not => {
                    let e_type: String = determine_evaluation_type(&*u.e);
                    // Ensure operand is a boolean type
                    if e_type != "bool".to_string() {
                        rp_error!("Invalid use of operator {} on non-boolean value {}", u.op, *u.e);
                    } else {
                        e_type
                    }
                }
            }
        },
        &Expression::VariableMapping(ref v) => {
            v.var_type.clone()
        },
        &Expression::BooleanLiteral(ref b) => {
            "bool".to_string()
        },
        &Expression::UnsignedBitVector(ref u) => {
            match u.size {
                8 => {
                    "u8".to_string()
                },
                16 => {
                    "u16".to_string()
                },
                32 => {
                    "u32".to_string()
                },
                64 => {
                    "u64".to_string()
                },
                _ => {
                    rp_error!("Invalid or Unsupported integer type: \"u{}\"", u.size.to_string());
                }
            }
        },
        &Expression::SignedBitVector(ref s) => {
            match s.size {
                8 => {
                    "i8".to_string()
                },
                16 => {
                    "i16".to_string()
                },
                32 => {
                    "i32".to_string()
                },
                64 => {
                    "i64".to_string()
                },
                _ => {
                    rp_error!("Invalid or Unsupported integer type: \"i{}\"", s.size.to_string());
                }
            }
        }
    }
}

// Recurses through an Expression and returns Ok(true) if all the types seem valid; returns Err(some message) if a type seems invalid. 
pub fn ty_check( expression: &Expression ) -> Result<bool, String> {
    match expression {
        &Expression::BinaryExpression(ref b) => {
            match b.op {
                BinaryOperator::Addition | BinaryOperator::Subtraction | BinaryOperator::Multiplication | BinaryOperator::Division | BinaryOperator::Modulo => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                                        Err(format!("Invalid use of binary operator {} on boolean value(s)", b.op))
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                BinaryOperator::BitwiseLeftShift | BinaryOperator::BitwiseRightShift => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                                        Err(format!("Invalid use of binary operator {} on boolean value(s)", b.op))
                                    //Ensure both operand types are of same signedness
                                    } else if (l_type.starts_with("i") && r_type.starts_with("i")) || (l_type.starts_with("u") && r_type.starts_with("u")) {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                BinaryOperator::BitwiseOr | BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseXor => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operand types match
                                    if l_type != r_type {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                BinaryOperator::LessThan | BinaryOperator::LessThanOrEqual | BinaryOperator::GreaterThan | BinaryOperator::GreaterThanOrEqual => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == "bool".to_string()) || (r_type == "bool".to_string()) {
                                        Err(format!("Invalid use of binary operator {} on boolean value(s)", b.op))
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                BinaryOperator::Equal | BinaryOperator::NotEqual => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operand types match
                                    if l_type != r_type {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor | BinaryOperator::Implication | BinaryOperator::BiImplication => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: String = determine_evaluation_type(&*b.left);
                                    let r_type: String = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are boolean types
                                    if (l_type != "bool".to_string()) || (r_type != "bool".to_string()) {
                                        Err(format!("Invalid use of binary operator {} on numeric value(s)", b.op))
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(format!("Binary operand types do not match: {} {} {}", l_type, b.op, r_type))
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => {
                                    Err(e)
                                }
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
            }
        },
        &Expression::UnaryExpression(ref u) => {
            match u.op {
                UnaryOperator::Negation => {
                    match ty_check(&*u.e) {
                        Ok(_) => {
                            let e_type: String = determine_evaluation_type(&*u.e);

                            // Ensure operand is a numeric type
                            if e_type == "bool".to_string() {
                                Err(format!("Invalid use of operator {} on boolean value {}", u.op, *u.e))
                            // Ensure operand is not an unsigned type
                            } else if (e_type == "u8".to_string()) || (e_type == "u16".to_string()) || (e_type == "u32".to_string()) || (e_type == "u64".to_string()) {
                                Err(format!("Invalid use of operator {} on unsigned value {}", u.op, *u.e))
                            } else {
                                Ok(true)
                            }
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                UnaryOperator::BitwiseNot => {
                    match ty_check(&*u.e) {
                        Ok(_) => {
                            Ok(true)
                        },
                        Err(e) => {
                            Err(e)
                        }
                    }
                },
                UnaryOperator::Not => {
                    let e_type: String = determine_evaluation_type(&*u.e);
                    // Ensure operand is a boolean type
                    if e_type != "bool".to_string() {
                        Err(format!("Invalid use of operator {} on non-boolean value {}", u.op, *u.e))
                    } else {
                        Ok(true)
                    }
                },
            }
        },
        &Expression::VariableMapping(ref v) => {
            match v.var_type.as_str() {
                "bool" | "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => {
                    Ok(true)
                },
                _ => {
                    Err(format!("Invalid or unsupported variable type: \"{}\"", v.var_type))
                }
            }
        },
        &Expression::BooleanLiteral(ref b) => {
            Ok(true)
        },
        &Expression::UnsignedBitVector(ref u) => {
            match u.size {
                8 | 16 | 32 | 64 => {
                    Ok(true)
                },
                _ => {
                   Err(format!("Invalid or Unsupported integer type: \"u{}\"", u.size.to_string()))
                }
            }
        },
        &Expression::SignedBitVector(ref s) => {
            match s.size {
                8 | 16 | 32 | 64 => {
                    Ok(true)
                },
                _ => {
                    Err(format!("Invalid or Unsupported integer type: \"i{}\"", s.size.to_string()))
                }
            }
        }
    }
}
