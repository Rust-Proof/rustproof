// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Internal data structure and related functions to represent logical expressions.

//#[macro_use]
extern crate term;
use std::fmt;
use std::process;

use errors::{ColorConfig, Handler};
use syntax::codemap::CodeMap;
use std::rc::Rc;

// Boolean Expression type
#[derive(Clone, PartialEq)]
pub enum Expression {
    // Two sub-expressions joined by an operator
    BinaryExpression(BinaryExpressionData),
    // A sub-expression acted upon by an operator
    UnaryExpression(UnaryExpressionData),
    // A variable; should be either one of a function's formal arguments,
    // a special "return" variable, or something from an encapsulating scope.
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
        match *self {
            Expression::BinaryExpression (ref b) => {
                write!(f, "({} {} {})", *b.left, b.op, *b.right)
            },
            Expression::UnaryExpression (ref u) => write!(f, "({} {})", u.op, *u.e),
            Expression::VariableMapping (ref v) => write!(f, "({}: {})", v.name, v.var_type),
            Expression::BooleanLiteral (ref b) => write!(f, "({})", b),
            Expression::UnsignedBitVector(ref u) => {
                write!(f, "({}u{})", u.value, u.size.to_string())
            },
            Expression::SignedBitVector(ref s) => {
                write!(f, "({}i{})", s.value, s.size.to_string())
            }
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, PartialEq)]
pub struct BinaryExpressionData {
    pub op: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}

#[derive(Clone, PartialEq)]
pub struct UnaryExpressionData {
    pub op: UnaryOperator,
    pub e: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct VariableMappingData { pub name: String, pub var_type: Types }

// Check equality for VariableMappingData types.
// Should return true if the name and type of the variables are the same.
impl PartialEq for VariableMappingData {
    fn eq(&self, _rhs: &VariableMappingData) -> bool {
        return (self.name == _rhs.name) && (self.var_type == _rhs.var_type);
    }
}

// Ensures it is clear that VariableMappingData has full equality.
impl Eq for VariableMappingData {}

impl fmt::Display for VariableMappingData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}: {})", self.name, self.var_type)
    }
}

#[derive(Clone, PartialEq)]
pub struct UnsignedBitVectorData {
    pub size: u8,
    pub value: u64,
}

#[derive(Clone, PartialEq)]
pub struct SignedBitVectorData {
    pub size: u8,
    pub value: i64,
}

#[derive(Clone, PartialEq)]
pub enum BinaryOperator {
    // Normal operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    // Overflow operators
    SignedMultiplicationDoesNotOverflow,
    SignedMultiplicationDoesNotUnderflow,
    UnsignedMultiplicationDoesNotOverflow,
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
    BiImplication,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryOperator::Addition => write!(f, "+"),
            BinaryOperator::Subtraction => write!(f, "-"),
            BinaryOperator::Multiplication => write!(f, "*"),
            BinaryOperator::Division => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::SignedMultiplicationDoesNotOverflow => {
                write!(f, "s_mul_no_overflow")
            },
            BinaryOperator::SignedMultiplicationDoesNotUnderflow => {
                write!(f, "s_mul_no_underflow")
            },
            BinaryOperator::UnsignedMultiplicationDoesNotOverflow => {
                write!(f, "u_mul_no_overflow")
            },
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseXor => write!(f, "^"),
            BinaryOperator::BitwiseLeftShift => write!(f, "<<"),
            BinaryOperator::BitwiseRightShift => write!(f, ">>"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::And => write!(f, "AND"),
            BinaryOperator::Or => write!(f, "OR"),
            BinaryOperator::Xor => write!(f, "XOR"),
            BinaryOperator::Implication => write!(f, "IMPLIES"),
            BinaryOperator::BiImplication => write!(f, "EQUIV"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UnaryOperator {
    Negation,
    BitwiseNot,
    // Boolean logical operator
    Not,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnaryOperator::Negation => { write!(f, "-") },
            UnaryOperator::BitwiseNot => { write!(f, "!") },
            UnaryOperator::Not => { write!(f, "NOT") }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
	Bool,
	I8,
	I16,
	I32,
	I64,
	U8,
	U16,
	U32,
	U64,
    Void
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Types::Bool => { write!(f, "bool") },
            Types::I8 => { write!(f, "i8") },
            Types::I16 => { write!(f, "i16") },
            Types::I32 => { write!(f, "i32") },
            Types::I64 => { write!(f, "i64") },
            Types::U8 => { write!(f, "u8") },
            Types::U16 => { write!(f, "u16") },
            Types::U32 => { write!(f, "u32") },
            Types::U64 => { write!(f, "u64") },
            Types::Void => { write!(f, "()") },
        }
    }
}

/// Recurses through an Expression and replaces any instance of a variable with a given Expression.
///
/// # Arguments:
/// * `source_expression` - The Expression to be recursed through.
/// * `target` - The variable to be replaced.
/// * `replacement` - The Expression to replace the target, if found.
///
/// # Remarks:
///
pub fn substitute_variable_with_expression (source_expression: &mut Expression,
                                            target: &VariableMappingData,
                                            replacement: &Expression) {
    let mut replace: bool = false;
    match source_expression {
        &mut Expression::BinaryExpression(ref mut b) => {
            // Recurisvely call the sub-expressions
            substitute_variable_with_expression(&mut(*b.left), target, replacement);
            substitute_variable_with_expression(&mut(*b.right), target, replacement);
        },
        &mut Expression::UnaryExpression(ref mut u) => {
            // Recurisvely call the sub-expression
            substitute_variable_with_expression(&mut(*u.e), target, replacement);
        },
        &mut Expression::VariableMapping(ref mut v) => {
            // Substitute the variable if it matches the target
            if v == target {
                replace = true;
            }
        },
        _ => {
            // No substitution should be done
        }
    }

    // Substitute the variable after the match to avoid scope issues
    if replace {
        *source_expression = replacement.clone();
    }
}

/// Recurses through an Expression and returns the type it would evaluate to.
///
/// # Arguments:
/// * `expression` - An Expression whose evaluation type will be returned.
///
/// # Return:
/// * A String representation of the type that should return from the top level of the Expression.
///
/// # Remarks:
///
pub fn determine_evaluation_type ( expression: &Expression ) -> Types {
    match ty_check(expression) {
        Ok(_) => {
            match *expression {
                Expression::BinaryExpression(ref b) => {
                    match b.op {
                        BinaryOperator::Addition
                        | BinaryOperator::Subtraction
                        | BinaryOperator::Multiplication
                        | BinaryOperator::Division
                        | BinaryOperator::Modulo
                        | BinaryOperator::BitwiseLeftShift
                        | BinaryOperator::BitwiseRightShift
                        | BinaryOperator::BitwiseOr
                        | BinaryOperator::BitwiseAnd
                        | BinaryOperator::BitwiseXor => determine_evaluation_type(&*b.left),
                        BinaryOperator::LessThan
                        | BinaryOperator::LessThanOrEqual
                        | BinaryOperator::GreaterThan
                        | BinaryOperator::GreaterThanOrEqual
                        | BinaryOperator::SignedMultiplicationDoesNotOverflow
                        | BinaryOperator::SignedMultiplicationDoesNotUnderflow
                        | BinaryOperator::UnsignedMultiplicationDoesNotOverflow
                        | BinaryOperator::Equal
                        | BinaryOperator::NotEqual
                        | BinaryOperator::And
                        | BinaryOperator::Or
                        | BinaryOperator::Xor
                        | BinaryOperator::Implication
                        | BinaryOperator::BiImplication => Types::Bool,
                    }
                },
                Expression::UnaryExpression(ref u) => {
                    match u.op {
                        UnaryOperator::Negation
                        | UnaryOperator::Not => determine_evaluation_type(&*u.e),
                        UnaryOperator::BitwiseNot => determine_evaluation_type(&*u.e),
                    }
                },
                Expression::VariableMapping(ref v) => v.var_type.clone(),
                Expression::BooleanLiteral(_) => Types::Bool,
                Expression::UnsignedBitVector(ref u) => {
                    match u.size {
                        8 => Types::U8,
                        16 => Types::U16,
                        32 => Types::U32,
                        64 => Types::U64,
                        _ => {
                            rp_error!(
                                "Invalid or Unsupported integer type: \"u{}\"",
                                u.size.to_string()
                            );
                        }
                    }
                },
                Expression::SignedBitVector(ref s) => {
                    match s.size {
                        8 => Types::I8,
                        16 => Types::I16,
                        32 => Types::I32,
                        64 => Types::I64,
                        _ => {
                            rp_error!(
                                "Invalid or Unsupported integer type: \"i{}\"",
                                s.size.to_string()
                            );
                        }
                    }
                }
            }
        },
        Err(e) => rp_error!("{}", e),
    }
}

/// Recurses through an Expression and checks for validity of types, operands, and integer bounds.
///
/// # Arguments:
/// * `expression` - An Expression whose type will be checked.
///
/// # Return:
/// * Ok(true) if all seems valid.
/// * Err(String) otherwise, the String containing a message about the first problem encountered.
///
/// # Remarks:
///
pub fn ty_check( expression: &Expression ) -> Result<bool, String> {
    match *expression {
        Expression::BinaryExpression(ref b) => {
            match b.op {
                BinaryOperator::Addition
                | BinaryOperator::Subtraction
                | BinaryOperator::Multiplication
                | BinaryOperator::Division
                | BinaryOperator::Modulo
                | BinaryOperator::SignedMultiplicationDoesNotOverflow
                | BinaryOperator::SignedMultiplicationDoesNotUnderflow
                | BinaryOperator::UnsignedMultiplicationDoesNotOverflow => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == Types::Bool) || (r_type == Types::Bool) {
                                        Err(
                                            format!(
                                                "Invalid use of binary operator {} on boolean \
                                                value(s)",
                                                b.op
                                            )
                                        )
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                BinaryOperator::BitwiseLeftShift | BinaryOperator::BitwiseRightShift => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == Types::Bool) || (r_type == Types::Bool) {
                                        Err(
                                            format!(
                                                "Invalid use of binary operator {} on boolean \
                                                value(s)",
                                                b.op
                                            )
                                        )
                                    //Ensure both operand types are of same signedness
                                    } else if (is_valid_signed(&*b.left)
                                    			&& !is_valid_signed(&*b.right))
                                            || (is_valid_unsigned(&*b.left)
                                            	&& !is_valid_unsigned(&*b.right)) {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                BinaryOperator::BitwiseOr
                | BinaryOperator::BitwiseAnd
                | BinaryOperator::BitwiseXor => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operand types match
                                    if l_type != r_type {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                BinaryOperator::LessThan
                | BinaryOperator::LessThanOrEqual
                | BinaryOperator::GreaterThan
                | BinaryOperator::GreaterThanOrEqual => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are numeric types
                                    if (l_type == Types::Bool) || (r_type == Types::Bool) {
                                        Err(
                                            format!(
                                                "Invalid use of binary operator {} on boolean \
                                                value(s)",
                                                b.op
                                            )
                                        )
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                BinaryOperator::Equal | BinaryOperator::NotEqual => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operand types match
                                    if l_type != r_type {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor
                | BinaryOperator::Implication | BinaryOperator::BiImplication => {
                    match ty_check(&*b.left) {
                        Ok(_) => {
                            match ty_check(&*b.right) {
                                Ok(_) => {
                                    let l_type: Types = determine_evaluation_type(&*b.left);
                                    let r_type: Types = determine_evaluation_type(&*b.right);
                                    // Ensure both operands are boolean types
                                    if (l_type != Types::Bool) || (r_type != Types::Bool) {
                                        Err(
                                            format!(
                                                "Invalid use of binary operator {} on numeric value(s)",
                                                b.op
                                            )
                                        )
                                    // Ensure both operand types match
                                    } else if l_type != r_type {
                                        Err(
                                            format!(
                                                "Binary operand types do not match: {} {} {}",
                                                l_type,
                                                b.op,
                                                r_type
                                            )
                                        )
                                    } else {
                                        Ok(true)
                                    }
                                },
                                Err(e) => Err(e)
                            }
                        },
                        Err(e) => Err(e)
                    }
                }
            }
        },
        Expression::UnaryExpression(ref u) => {
            match u.op {
                UnaryOperator::Negation => {
                    match ty_check(&*u.e) {
                        Ok(_) => {
                            let e_type: Types = determine_evaluation_type(&*u.e);

                            // Ensure operand is a numeric type
                            if e_type == Types::Bool {
                                Err(
                                    format!(
                                        "Invalid use of operator {} on boolean value {}",
                                        u.op,
                                        *u.e
                                    )
                                )
                            // Ensure operand is not an unsigned type
                            } else if is_valid_unsigned(&*u.e) {
                                Err(
                                    format!(
                                        "Invalid use of operator {} on unsigned value {}",
                                        u.op,
                                        *u.e
                                    )
                                )
                            } else {
                                Ok(true)
                            }
                        },
                        Err(e) => Err(e)
                    }
                },
                UnaryOperator::BitwiseNot => {
                    match ty_check(&*u.e) {
                        Ok(_) => Ok(true),
                        Err(e) => Err(e)
                    }
                },
                UnaryOperator::Not => {
                    let e_type: Types = determine_evaluation_type(&*u.e);
                    // Ensure operand is a boolean type
                    if e_type != Types::Bool {
                        Err(
                            format!(
                                "Invalid use of operator {} on non-boolean value {}",
                                u.op,
                                *u.e
                            )
                        )
                    } else {
                        Ok(true)
                    }
                },
            }
        },
        Expression::VariableMapping(ref v) => {
            if let Types::Void = v.var_type {
                Err(format!("Variable {} has void type!", v.name))
            } else {
                Ok(true)
            }
        },
        Expression::BooleanLiteral(_) => {
            Ok(true)
        },
        Expression::UnsignedBitVector(ref u) => {
            match u.size {
                8 => {
                    if (u.value >= u8::min_value() as u64) && (u.value <= u8::max_value() as u64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for u8 type: {}", u.value.to_string()))
                    }
                },
                16 => {
                    if (u.value >= u16::min_value() as u64)
                       && (u.value <= u16::max_value() as u64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for u16 type: {}", u.value.to_string()))
                    }
                },
                32 => {
                    if (u.value >= u32::min_value() as u64)
                       && (u.value <= u32::max_value() as u64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for u32 type: {}", u.value.to_string()))
                    }
                },
                64 => {
                    if (u.value >= u64::min_value() as u64)
                       && (u.value <= u64::max_value() as u64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for u64 type: {}", u.value.to_string()))
                    }
                },
                _ => {
                   Err(format!("Invalid or unsupported integer type: \"u{}\"", u.size.to_string()))
                }
            }
        },
        Expression::SignedBitVector(ref s) => {
            match s.size {
                8 => {
                    if (s.value >= i8::min_value() as i64) && (s.value <= i8::max_value() as i64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for i8 type: {}", s.value.to_string()))
                    }
                },
                16 => {
                    if (s.value >= i16::min_value() as i64)
                       && (s.value <= i16::max_value() as i64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for i16 type: {}", s.value.to_string()))
                    }
                },
                32 => {
                    if (s.value >= i32::min_value() as i64)
                       && (s.value <= i32::max_value() as i64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for i32 type: {}", s.value.to_string()))
                    }
                },
                64 => {
                    if (s.value >= i64::min_value() as i64)
                       && (s.value <= i64::max_value() as i64) {
                        Ok(true)
                    } else {
                        Err(format!("Out of range value for i64 type: {}", s.value.to_string()))
                    }
                },
                _ => {
                    Err(
                        format!(
                            "Invalid or unsupported integer type: \"i{}\"",
                            s.size.to_string()
                        )
                    )
                }
            }
        }
    }
}

/// Checks if an Expression matches one of the supported unsigned integer types
///
/// # Arguments:
/// * `e` - An Expression
///
/// # Return:
/// * `true` if it matches, `false` otherwise
///
/// # Remarks:
/// * Current supported types: u8, u16, u32, u64
///
pub fn is_valid_unsigned(e: &Expression) -> bool {
    match *e {
    	Expression::VariableMapping(ref v) => {
    		is_unsigned_type(v.var_type.clone())
    	}
    	Expression::UnsignedBitVector(ref u) => {
    		match u.size {
    			8u8 | 16u8 | 32u8 | 64u8 => true,
    			_ => false,
    		}
    	}
    	_ => false,
    }
}

/// Checks if an Expression matches one of the supported signed integer types
///
/// # Arguments:
/// * `e` - An Expression
///
/// # Return:
/// * `true` if it matches, `false` otherwise
///
/// # Remarks:
/// * Current supported types: i8, i16, i32, i64
///
pub fn is_valid_signed(e: &Expression) -> bool {
    match *e {
    	Expression::VariableMapping(ref v) => {
    		is_signed_type(v.var_type.clone())
    	}
    	Expression::SignedBitVector(ref s) => {
    		match s.size {
    			8u8 | 16u8 | 32u8 | 64u8 => true,
    			_ => false,
    		}
    	}
    	_ => false,
    }
}

/// Checks if a Types, like the return from determine_evaluation_type(), matches one of the
/// supported unsigned integer types
///
/// # Arguments:
/// * `t` - A Types
///
/// # Return:
/// * `true` if it matches, `false` otherwise
///
/// # Remarks:
/// * Current supported types: u8, u16, u32, u64
///
pub fn is_unsigned_type(t: Types) -> bool {
	match t {
		Types::U8 | Types::U16 | Types::U32 | Types::U64 => true,
		_ => false,
	}
}

/// Checks if a Types, like the return from determine_evaluation_type(), matches one of the
/// supported signed integer types
///
/// # Arguments:
/// * `t` - A Types
///
/// # Return:
/// * `true` if it matches, `false` otherwise
///
/// # Remarks:
/// * Current supported types: i8, i16, i32, i64
///
pub fn is_signed_type(t: Types) -> bool {
	match t {
		Types::I8 | Types::I16 | Types::I32 | Types::I64 => true,
		_ => false,
	}
}

/// Returns a `Types` interpreted from a `String` representing a variable or literal type
///
/// # Arguments:
/// * `s` - A String
///
/// # Return:
/// * A Types
///
/// # Remarks:
/// * Current supported types: bool, i8, i16, i32, i64, u8, u16, u32, u64
pub fn string_to_type(s: String) -> Types {
	match s.as_str() {
        "bool" => Types::Bool,
        "i8" => Types::I8,
        "i16" => Types::I16,
        "i32" => Types::I32,
        "i64" => Types::I64,
        "u8" => Types::U8,
        "u16" => Types::U16,
        "u32" => Types::U32,
        "u64" => Types::U64,
        "()" => Types::Void,
        _ => unimplemented!(),
	}
}