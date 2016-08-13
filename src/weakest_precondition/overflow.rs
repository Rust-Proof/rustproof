// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_const_math;

use std::process;
use expression::*;
use rustc::mir::repr::*;
use rustc::middle::const_val::ConstVal;
use rustc_const_math::ConstInt;
use rustc_data_structures::indexed_vec::Idx;
use rustc::ty::{Ty, TypeVariants};
use std::rt::begin_panic_fmt;

use errors::{ColorConfig, Handler};
use syntax::codemap::CodeMap;
use std::rc::Rc;

// One catch-all function for overflow checking.
pub fn overflow_check(wp: &Expression, var: &VariableMappingData, binop: &BinOp, lvalue: &Expression, rvalue: &Expression) -> Expression {
    let v = var.clone();

    Expression::BinaryExpression( BinaryExpressionData {
        op: BinaryOperator::And,
        left: Box::new(wp.clone()),
        right: Box::new(
            match v.var_type.as_str() {
                "i8" => { signed_overflow(binop, 8u8, lvalue, rvalue) },
                "i16" => { signed_overflow(binop, 16u8, lvalue, rvalue) },
                "i32" => { signed_overflow(binop, 32u8, lvalue, rvalue) },
                "i64" => { signed_overflow(binop, 64u8, lvalue, rvalue) },
                "u8" => { unsigned_overflow(binop, 8u8, lvalue, rvalue) },
                "u16" => { unsigned_overflow(binop, 16u8, lvalue, rvalue) },
                "u32" => { unsigned_overflow(binop, 32u8, lvalue, rvalue) },
                "u64" => { unsigned_overflow(binop, 64u8, lvalue, rvalue) },
                _ => { panic!("Unsupported return type of binary operation: {}", v.var_type); }
            }
        ),
    })
}

// Signed: Match on the type of BinOp and call the correct function
fn signed_overflow(binop: &BinOp, size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    match binop {
        &BinOp::Add => { signed_add(size, lvalue, rvalue) },
        &BinOp::Mul => { signed_mul(size, lvalue, rvalue) },
        &BinOp::Sub => { signed_sub(size, lvalue, rvalue) },
        &BinOp::Div => { signed_div(size, lvalue, rvalue) },
        &BinOp::Rem => { unimplemented!() },
        &BinOp::Shl => { unimplemented!() },
        &BinOp::Shr => { unimplemented!() },
        &BinOp::BitOr => { unimplemented!() },
        &BinOp::BitAnd => { unimplemented!() },
        &BinOp::BitXor => { unimplemented!() },
        &BinOp::Lt => { unimplemented!() },
        &BinOp::Le => { unimplemented!() },
        &BinOp::Gt => { unimplemented!() },
        &BinOp::Ge => { unimplemented!() },
        &BinOp::Eq => { unimplemented!() },
        &BinOp::Ne => { unimplemented!() },
    }
}

// Creates an Expression containing overflow and underflow checks for lvalue + rvalue, assuming they are bitvectors of length "size"
//
// The following psuedocode provides a logically equivalent version of what is produced
// (false is returned if overflow/underflow has occurred, true otherwise)
//
// If lvalue >= 0 && rvalue >= 0
//   If lvalue + rvalue < 0
//     false
//   Else
//     true
// Else
//   If lvalue < 0 && rvalue < 0
//     If lvalue + rvalue >= 0
//       false
//     Else
//       true
//   Else
//     true
//
fn signed_add(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::And,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::GreaterThanOrEqual,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::GreaterThanOrEqual,
                                left: Box::new(rvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
                right: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::GreaterThanOrEqual,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::Addition,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(rvalue.clone()),
                            })
                        ),
                        right: Box::new(
                            Expression::SignedBitVector( SignedBitVectorData {
                                size: size,
                                value: 0i64,
                            })
                        ),
                    })
                ),
            })
        ),
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::Or,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(rvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
                right: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::Implication,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::And,
                                left: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::LessThan,
                                        left: Box::new(lvalue.clone()),
                                        right: Box::new(
                                            Expression::SignedBitVector( SignedBitVectorData {
                                                size: size,
                                                value: 0i64,
                                            })
                                        ),
                                    })
                                ),
                                right: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::LessThan,
                                        left: Box::new(rvalue.clone()),
                                        right: Box::new(
                                            Expression::SignedBitVector( SignedBitVectorData {
                                                size: size,
                                                value: 0i64,
                                            })
                                        ),
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::Addition,
                                        left: Box::new(lvalue.clone()),
                                        right: Box::new(rvalue.clone()),
                                    })
                                ),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
            })
        ),
    })
}

// Creates an Expression containing overflow and underflow checks for lvalue - rvalue, assuming they are bitvectors of length "size"
//
// The following psuedocode provides a logically equivalent version of what is produced
// (false is returned if overflow/underflow has occurred, true otherwise)
//
// If lvalue >= 0 && rvalue < 0
//   If lvalue - rvalue < 0
//     false
//   Else
//     true
// Else
//   If lvalue < 0 && rvalue >= 0
//     If lvalue - rvalue >= 0
//       false
//     Else
//       true
//   Else
//     true
//
fn signed_sub(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::And,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::GreaterThanOrEqual,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(rvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
                right: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::GreaterThanOrEqual,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::Subtraction,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(rvalue.clone()),
                            })
                        ),
                        right: Box::new(
                            Expression::SignedBitVector( SignedBitVectorData {
                                size: size,
                                value: 0i64,
                            })
                        ),
                    })
                ),
            })
        ),
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::Or,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(lvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::GreaterThanOrEqual,
                                left: Box::new(rvalue.clone()),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
                right: Box::new(
                    Expression::BinaryExpression( BinaryExpressionData{
                        op: BinaryOperator::Implication,
                        left: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::And,
                                left: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::LessThan,
                                        left: Box::new(lvalue.clone()),
                                        right: Box::new(
                                            Expression::SignedBitVector( SignedBitVectorData {
                                                size: size,
                                                value: 0i64,
                                            })
                                        ),
                                    })
                                ),
                                right: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::GreaterThanOrEqual,
                                        left: Box::new(rvalue.clone()),
                                        right: Box::new(
                                            Expression::SignedBitVector( SignedBitVectorData {
                                                size: size,
                                                value: 0i64,
                                            })
                                        ),
                                    })
                                ),
                            })
                        ),
                        right: Box::new(
                            Expression::BinaryExpression( BinaryExpressionData{
                                op: BinaryOperator::LessThan,
                                left: Box::new(
                                    Expression::BinaryExpression( BinaryExpressionData{
                                        op: BinaryOperator::Subtraction,
                                        left: Box::new(lvalue.clone()),
                                        right: Box::new(rvalue.clone()),
                                    })
                                ),
                                right: Box::new(
                                    Expression::SignedBitVector( SignedBitVectorData {
                                        size: size,
                                        value: 0i64,
                                    })
                                ),
                            })
                        ),
                    })
                ),
            })
        ),
    })
}

fn signed_mul(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    let overflow: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::SignedMultiplicationDoesNotOverflow,
        left: Box::new(lvalue.clone()),
        right: Box::new(rvalue.clone()),
    });

    let underflow: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::SignedMultiplicationDoesNotUnderflow,
        left: Box::new(lvalue.clone()),
        right: Box::new(rvalue.clone()),
    });

    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(overflow),
        right: Box::new(underflow),
    })
}

fn signed_div(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    let condition = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Equal,
                left: Box::new(lvalue.clone()),
                right: Box::new(
                    Expression::SignedBitVector( SignedBitVectorData{
                        size: size,
                        value: match size {
                            8u8 => { i8::min_value() as i64 },
                            16u8 => { i16::min_value() as i64 },
                            32u8 => { i32::min_value() as i64 },
                            64u8 => { i64::min_value() as i64 },
                            _ => { panic!("unsupported integer type") },
                        },
                    })
                ),
            })
        ),
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Equal,
                left: Box::new(rvalue.clone()),
                right: Box::new(
                    Expression::SignedBitVector( SignedBitVectorData{
                        size: size,
                        value: -1i64,
                    })
                )
            })
        ),
    });

    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(condition.clone()),
                right: Box::new(
                    Expression::BooleanLiteral(false)
                ),
            })
        ),
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Implication,
                left: Box::new(
                    Expression::UnaryExpression( UnaryExpressionData{
                        op: UnaryOperator::Not,
                        e: Box::new(condition.clone()),
                    })
                ),
                right: Box::new(
                    Expression::BooleanLiteral(true)
                ),
            })
        ),
    })
}

// Unsigned: Match on the type of BinOp and call the correct function
fn unsigned_overflow(binop: &BinOp, size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    match binop {
        &BinOp::Add => { unsigned_add(size, lvalue, rvalue) },
        &BinOp::Sub => { unsigned_sub(size, lvalue, rvalue) },
        &BinOp::Mul => { unsigned_mul(size, lvalue, rvalue) },
        &BinOp::Div => { unimplemented!() },
        &BinOp::Rem => { unimplemented!() },
        &BinOp::Shl => { unimplemented!() },
        &BinOp::Shr => { unimplemented!() },
        &BinOp::BitOr => { unimplemented!() },
        &BinOp::BitAnd => { unimplemented!() },
        &BinOp::BitXor => { unimplemented!() },
        &BinOp::Lt => { unimplemented!() },
        &BinOp::Le => { unimplemented!() },
        &BinOp::Gt => { unimplemented!() },
        &BinOp::Ge => { unimplemented!() },
        &BinOp::Eq => { unimplemented!() },
        &BinOp::Ne => { unimplemented!() },
    }
}

fn unsigned_mul(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::UnsignedMultiplicationDoesNotOverflow,
        left: Box::new(lvalue.clone()),
        right: Box::new(rvalue.clone()),
    })
}

// l + r >= l
fn unsigned_add(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::GreaterThanOrEqual,
        //l + r
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Addition,
                left: Box::new(lvalue.clone()),
                right: Box::new(rvalue.clone()),
            })
        ),
        // l
        right: Box::new(rvalue.clone()),
    })
}

// l - r <= l
fn unsigned_sub(size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::LessThanOrEqual,
        //l - r
        left: Box::new(
            Expression::BinaryExpression( BinaryExpressionData{
                op: BinaryOperator::Subtraction,
                left: Box::new(lvalue.clone()),
                right: Box::new(rvalue.clone()),
            })
        ),
        // l
        right: Box::new(rvalue.clone()),
    })
}


// --------------------------------------------
// FIXME: These functions below will eventually
// be deprecated by the functions above.
// --------------------------------------------

/// Generates a version of wp "And"ed together with a conditional expression that mimics a check for overflow for the type of var.
///
/// # Arguments:
/// * `wp` - The current weakest precondition that the overflow is to be "And"ed to
/// * `var` - VariableMappingData that determines size and value of the overflow value as well as
///           The left hand operand information of the overflow check
///
/// # Return Value:
/// * Returns the modified weakest precondition with overflow check
///
/// # Remarks:
/// * Current supported ConstInt: I8, I16, I32, I64, U8, U16, U32, U64
/// * WARNING: If var.clone() does not happen, it will break tuple support within the current code
///
pub fn add_overflow(wp: &Expression, var: &VariableMappingData) -> Expression {
    let v = var.clone();

    // "And" together the current wp to the overflowcheck
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(wp.clone()),
        // Creates the righthand side of the "And" Expression which is the overflow check
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData {
                op: BinaryOperator::LessThanOrEqual,
                // left hand side is the VariableMapping v data
                left: Box::new(Expression::VariableMapping(v.clone())),
                // Right hand side is the max value allowed by the VariableMapping v type
                right: Box::new(match v.var_type.as_str() {
                    "i8" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 8u8,
                            value: i8::max_value() as i64
                        })
                    },
                    "i16" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 16u8,
                            value: i16::max_value() as i64
                        })
                    },
                    "i32" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 32u8,
                            value: i32::max_value() as i64
                        })
                    },
                    "i64" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 64u8,
                            value: i64::max_value() as i64
                        })
                    },
                    "u8" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 8u8,
                            value: u8::max_value() as u64
                        })
                    },
                    "u16" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 16u8,
                            value: u16::max_value() as u64
                        })
                    },
                    "u32" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 32u8,
                            value: u32::max_value() as u64
                        })
                    },
                    "u64" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 64u8,
                            value: u64::max_value() as u64
                        })
                    },
                    _ => { panic!("Unsupported return type of binary operation: {}", v.var_type); }
                })
            })
        )
    })
}


/// Generates a version of wp "And"ed together with a conditional expression that mimics a check for underflow for the type of var.
///
/// # Arguments:
/// * `wp` - The current weakest precondition that the underflow is to be "And"ed to
/// * `var` - VariableMappingData that determines size and value of the underflow value as well as
///           The left hand operand information of the overflow check
///
/// # Return Value:
/// * Returns the modified weakest precondition with underflow check
///
/// # Remarks:
/// * Current supported ConstInt: I8, I16, I32, I64, U8, U16, U32, U64
/// * WARNING: If var.clone() does not happen, it will break tuple support within the current code
///
pub fn add_underflow(wp: &Expression, var: &VariableMappingData) -> Expression {
    let v = var.clone();

    // "And" together the current wp to the underflow check
    Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::And,
        left: Box::new(wp.clone()),
        // Creates the righthand side of the "And" Expression which is the overflow check
        right: Box::new(
            Expression::BinaryExpression( BinaryExpressionData {
                op: BinaryOperator::GreaterThanOrEqual,
                // left hand side is the VariableMapping v data
                left: Box::new(Expression::VariableMapping(v.clone())),
                // Right hand side is the max value allowed by the VariableMapping v type
                right: Box::new(match v.var_type.as_str() {
                    "i8" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 8u8,
                            value: i8::min_value() as i64
                        })
                    },
                    "i16" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 16u8,
                            value: i16::min_value() as i64
                        })
                    },
                    "i32" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 32u8,
                            value: i32::min_value() as i64
                        })
                    },
                    "i64" => {
                        Expression::SignedBitVector( SignedBitVectorData{
                            size: 64u8,
                            value: i64::min_value() as i64
                        })
                    },
                    "u8" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 8u8,
                            value: u8::min_value() as u64
                        })
                    },
                    "u16" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 16u8,
                            value: u16::min_value() as u64
                        })
                    },
                    "u32" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 32u8,
                            value: u32::min_value() as u64
                        })
                    },
                    "u64" => {
                        Expression::UnsignedBitVector( UnsignedBitVectorData{
                            size: 64u8,
                            value: u64::min_value() as u64
                        })
                    },
                    _ => { panic!("Unsupported return type of binary operation: {}", v.var_type); }
                })
            })
        )
    })
}
