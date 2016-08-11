
extern crate rustc_const_math;

use std::process;
use expression::*;
use rustc::mir::repr::*;
use rustc::middle::const_val::ConstVal;
use rustc_const_math::ConstInt;
use rustc_data_structures::indexed_vec::Idx;
use rustc::ty::{Ty, TypeVariants};
use std::rt::begin_panic_fmt;
use term;

pub fn overflow_check(wp: &Expression, var: &VariableMappingData, binop: &BinOp, lvalue: &Expression, rvalue: &Expression) -> Expression {
    let mut v = var.clone();
    v.name = v.name + ".0";

    Expression::BinaryExpresson( BinaryExpressionData {
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
            }
        ),
    })
}



fn signed_overflow(binop: &Binop, size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    match binop {
        &BinOp::Add => { },
        &BinOp::Sub => { },
        &BinOp::Mul => { },
        &BinOp::Div => { },
        &BinOp::Rem => { },
        &BinOp::Shl => { },
        &BinOp::Shr => { },
        &BinOp::BitOr => { },
        &BinOp::BitAnd => { },
        &BinOp::BitXor => { },
        &BinOp::Lt => { },
        &BinOp::Le => { },
        &BinOp::Gt => { },
        &BinOp::Ge => { },
        &BinOp::Eq => { },
        &BinOp::Ne => { },
    }
}

fn unsigned_overflow(binop: &Binop, size: u8, lvalue: &Expression, rvalue: &Expression) -> Expression {
    match binop {
        &BinOp::Add => { },
        &BinOp::Sub => { },
        &BinOp::Mul => { },
        &BinOp::Div => { },
        &BinOp::Rem => { },
        &BinOp::Shl => { },
        &BinOp::Shr => { },
        &BinOp::BitOr => { },
        &BinOp::BitAnd => { },
        &BinOp::BitXor => { },
        &BinOp::Lt => { },
        &BinOp::Le => { },
        &BinOp::Gt => { },
        &BinOp::Ge => { },
        &BinOp::Eq => { },
        &BinOp::Ne => { },
    }
}


/*
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
    let mut v = var.clone();
    v.name = v.name + ".0";
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
    let mut v = var.clone();
    v.name = v.name + ".0";
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
*/
