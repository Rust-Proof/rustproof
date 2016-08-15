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
fn determine_evaluation_type_comparison_unary(){
    let u: Expression = Expression::VariableMapping( VariableMappingData{
        name: "x".to_string(),
        var_type:"bool".to_string()
    });
    let to_test: Expression = Expression::UnaryExpression( UnaryExpressionData{
        op: UnaryOperator::Not,
        e: Box::new(u.clone()),
    });
    let returned_string = determine_evaluation_type(&to_test);
    let correct_result = "bool";
    assert_eq!(returned_string, correct_result);
}


#[test]
fn determine_evaluation_type_comparison_binary(){
    let left_side: Expression = Expression::VariableMapping( VariableMappingData{
        name:"x".to_string(),
        var_type: "i32".to_string()
    });
    let right_side: Expression = Expression::VariableMapping( VariableMappingData{
        name:"y".to_string(),
        var_type: "i32".to_string()
    });
    let to_test: Expression = Expression::BinaryExpression(BinaryExpressionData{
        op: BinaryOperator::Addition,
        left: Box::new(left_side.clone()),
        right: Box::new(right_side.clone()),
    });

    let returned_string = determine_evaluation_type(&to_test);
    let correct_result = "i32";
    assert_eq!(returned_string,correct_result);
}

#[test]
fn test_all_substitute_unary_operators(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()};
    let target: Expression = Expression::VariableMapping(target_var.clone() );
    let replacement: Expression = Expression::VariableMapping( VariableMappingData{
        name: "y".to_string(),
        var_type: "i32".to_string()
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

//#[test]
fn substitute_variable_with_expression_greater_than_or_equal(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: "i32".to_string()
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: "i32".to_string()
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

//#[test]
fn substitute_variable_with_expression_less_than() {
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: "i32".to_string()
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: "i32".to_string()
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

//#[test]
fn substitute_variable_with_expression_less_than_or_equal() {
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: "i32".to_string()
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: "i32".to_string()
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


//#[test]
fn substitute_variable_with_expression_greater_than(){
    let target_var : VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()
    };
    let target: Expression = Expression::VariableMapping( target_var.clone() );
    let superfluous: Expression = Expression::VariableMapping( VariableMappingData {
        name: "z".to_string(),
        var_type: "i32".to_string()
    });
    let replacement: Expression = Expression::VariableMapping( VariableMappingData {
        name: "y".to_string(),
        var_type: "i32".to_string()
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
        var_type: "i32".to_string()
    };
    let var2: VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "i32".to_string()
    };
    let var3: VariableMappingData = VariableMappingData {
        name: "y".to_string(),
        var_type: "i32".to_string()
    };
    let var4: VariableMappingData = VariableMappingData {
        name: "x".to_string(),
        var_type: "u32".to_string()
    };
    let var5: VariableMappingData = VariableMappingData {
        name: "y".to_string(),
        var_type: "u32".to_string()
    };
    let var6: VariableMappingData = VariableMappingData {
        name: "".to_string(),
        var_type: "".to_string()
    };

    assert!(var1 == var1);
    assert!(var1 == var2);
    assert!(var1 != var3);
    assert!(var1 != var4);
    assert!(var1 != var5);
    assert!(var1 != var6);
    assert!(var6 == var6);
}
