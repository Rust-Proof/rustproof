// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use expression::*;

#[test]
fn test_all_substitute_binary_operators() {
    substitute_variable_with_expression_less_than();
    substitute_variable_with_expression_less_than_or_equal();
    substitute_variable_with_expression_greater_than();
    substitute_variable_with_expression_greater_than_or_equal();
}

#[test]
fn test_check_signedness() {
    check_signedness_bool();
    check_signedness_i8();
    check_signedness_i16();
    check_signedness_i32();
    check_signedness_i64();
    check_signedness_u8();
    check_signedness_u16();
    check_signedness_u32();
    check_signedness_u64();
}

#[test]
fn determine_evaluation_type_comparison_unary(){
    let u: Expression = Expression::VariableMapping( VariableMappingData{
        name: "x".to_string(),
        var_type: Types::Bool
    });
    let to_test: Expression = Expression::UnaryExpression( UnaryExpressionData{
        op: UnaryOperator::Not,
        e: Box::new(u.clone()),
    });
    let determined_type = determine_evaluation_type(&to_test);
    let correct_result = Types::Bool;
    assert_eq!(determined_type, correct_result);
}

#[test]
fn determine_evaluation_type_comparison_binary(){
    let left_side: Expression = Expression::VariableMapping( VariableMappingData{
        name:"x".to_string(),
        var_type: Types::I32
    });
    let right_side: Expression = Expression::VariableMapping( VariableMappingData{
        name:"y".to_string(),
        var_type: Types::I32
    });
    let to_test: Expression = Expression::BinaryExpression(BinaryExpressionData{
        op: BinaryOperator::Addition,
        left: Box::new(left_side.clone()),
        right: Box::new(right_side.clone()),
    });

    let returned_string = determine_evaluation_type(&to_test);
    let correct_result = Types::I32;
    assert_eq!(returned_string, correct_result);
}

#[test]
fn substitute_unary_operator_not(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let target: Expression = Expression::VariableMapping(target_var.clone());
    let replacement: Expression = Expression::VariableMapping( VariableMappingData{
        name: "y".to_string(),
        var_type: Types::I32
    });
    let mut p: Expression = Expression::UnaryExpression(UnaryExpressionData{
        op: UnaryOperator::Not,
        e: Box::new(target.clone()),
    });

    let correct_result: Expression = Expression::UnaryExpression(UnaryExpressionData{
        op: UnaryOperator::Not,
        e: Box::new(replacement.clone() ),
    });
    substitute_variable_with_expression(&mut p, &target_var, &replacement);
    assert_eq!(p, correct_result);
}

fn substitute_variable_with_expression_greater_than_or_equal(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: Types::I32
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: Types::I32
    });
    let mut p: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::GreaterThanOrEqual,
        left: Box::new( target.clone() ),
        right: Box::new( superfluous.clone() ),
    });

    let correct_result: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::GreaterThanOrEqual,
        left: Box::new( replacement.clone() ),
        right: Box::new( superfluous.clone() ),
    });
    substitute_variable_with_expression(&mut p, &target_var, &replacement);
    assert_eq!(p, correct_result);
}

fn substitute_variable_with_expression_less_than() {
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: Types::I32
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: Types::I32
    });
    let mut p: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::LessThan,
        left: Box::new( target.clone() ),
        right: Box::new( superfluous.clone() ),
    });

    let correct_result: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::LessThan,
        left: Box::new( replacement.clone() ),
        right: Box::new( superfluous.clone() ),
    });
    substitute_variable_with_expression(&mut p, &target_var, &replacement);
    assert_eq!(p, correct_result);
}

fn substitute_variable_with_expression_less_than_or_equal() {
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: Types::I32
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: Types::I32
    });
    let mut p: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::LessThanOrEqual,
        left: Box::new( target.clone() ),
        right: Box::new( superfluous.clone() ),
    });

    let correct_result: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::LessThanOrEqual,
        left: Box::new( replacement.clone() ),
        right: Box::new( superfluous.clone() ),
    });
    substitute_variable_with_expression(&mut p, &target_var, &replacement);
    assert_eq!(p, correct_result);
}

fn substitute_variable_with_expression_greater_than(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: Types::I32
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: Types::I32
    });
    let mut p: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::GreaterThan,
        left: Box::new( target.clone() ),
        right: Box::new( superfluous.clone() ),
    });

    let correct_result: Expression = Expression::BinaryExpression( BinaryExpressionData{
        op: BinaryOperator::GreaterThan,
        left: Box::new( replacement.clone() ),
        right: Box::new( superfluous.clone() ),
    });
    substitute_variable_with_expression(&mut p, &target_var, &replacement);
    assert_eq!(p, correct_result);

}

#[test]
fn variable_mapping_data_equality() {
    let var1: VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let var2: VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::I32
    };
    let var3: VariableMappingData = VariableMappingData {
        name: "y".to_string(),
        var_type: Types::I32
    };
    let var4: VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: Types::U32
    };
    let var5: VariableMappingData = VariableMappingData {
        name: "y".to_string(),
        var_type: Types::U32
    };

    assert!(var1 == var1);
    assert!(var1 == var2);
    assert!(var1 != var3);
    assert!(var1 != var4);
    assert!(var1 != var5);
}

fn check_signedness_bool() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::Bool,
    });
    let boolean: Expression = Expression::BooleanLiteral(true);

    assert!(!is_valid_signed(&var));
    assert!(!is_valid_unsigned(&var));
    assert!(!is_valid_signed(&boolean));
    assert!(!is_valid_unsigned(&boolean));
}

fn check_signedness_i8() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::I8,
    });
    let num: Expression = Expression::SignedBitVector(SignedBitVectorData {
        size: 8u8,
        value: 1i64,
    });

    assert!(is_valid_signed(&var));
    assert!(is_valid_signed(&num));
    assert!(!is_valid_unsigned(&var));
    assert!(!is_valid_unsigned(&num));
}

fn check_signedness_i16() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::I16,
    });
    let num: Expression = Expression::SignedBitVector(SignedBitVectorData {
        size: 16u8,
        value: 1i64,
    });

    assert!(is_valid_signed(&var));
    assert!(is_valid_signed(&num));
    assert!(!is_valid_unsigned(&var));
    assert!(!is_valid_unsigned(&num));
}

fn check_signedness_i32() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::I32,
    });
    let num: Expression = Expression::SignedBitVector(SignedBitVectorData {
        size: 32u8,
        value: 1i64,
    });

    assert!(is_valid_signed(&var));
    assert!(is_valid_signed(&num));
    assert!(!is_valid_unsigned(&var));
    assert!(!is_valid_unsigned(&num));
}

fn check_signedness_i64() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::I64,
    });
    let num: Expression = Expression::SignedBitVector(SignedBitVectorData {
        size: 64u8,
        value: 1i64,
    });

    assert!(is_valid_signed(&var));
    assert!(is_valid_signed(&num));
    assert!(!is_valid_unsigned(&var));
    assert!(!is_valid_unsigned(&num));
}

fn check_signedness_u8() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::U8,
    });
    let num: Expression = Expression::UnsignedBitVector(UnsignedBitVectorData {
        size: 8u8,
        value: 1u64,
    });

    assert!(!is_valid_signed(&var));
    assert!(!is_valid_signed(&num));
    assert!(is_valid_unsigned(&var));
    assert!(is_valid_unsigned(&num));
}

fn check_signedness_u16() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::U16,
    });
    let num: Expression = Expression::UnsignedBitVector(UnsignedBitVectorData {
        size: 16u8,
        value: 1u64,
    });

    assert!(!is_valid_signed(&var));
    assert!(!is_valid_signed(&num));
    assert!(is_valid_unsigned(&var));
    assert!(is_valid_unsigned(&num));
}

fn check_signedness_u32() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::U32,
    });
    let num: Expression = Expression::UnsignedBitVector(UnsignedBitVectorData {
        size: 32u8,
        value: 1u64,
    });

    assert!(!is_valid_signed(&var));
    assert!(!is_valid_signed(&num));
    assert!(is_valid_unsigned(&var));
    assert!(is_valid_unsigned(&num));
}

fn check_signedness_u64() {
    let var: Expression = Expression::VariableMapping(VariableMappingData {
        name: "v".to_string(),
        var_type: Types::U64,
    });
    let num: Expression = Expression::UnsignedBitVector(UnsignedBitVectorData {
        size: 64u8,
        value: 1u64,
    });

    assert!(!is_valid_signed(&var));
    assert!(!is_valid_signed(&num));
    assert!(is_valid_unsigned(&var));
    assert!(is_valid_unsigned(&num));
}