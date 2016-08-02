// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use expression::{Expression, BinaryOperator, BinaryExpressionData, VariableMappingData, substitute_variable_with_expression};

#[test]
fn simple_replacement() {
	let target_var : VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() };
	let target: Expression = Expression::VariableMapping( target_var.clone() );
	let superfluous: Expression = Expression::VariableMapping( VariableMappingData { name: "z".to_string(), var_type: "i32".to_string() } );
	let replacement: Expression = Expression::VariableMapping( VariableMappingData { name: "y".to_string(), var_type: "i32".to_string() } );
	let mut p: Expression = Expression::BinaryExpression( BinaryExpressionData{
		op: BinaryOperator::LessThan, 
		left: Box::new( target.clone() ),
		right: Box::new( superfluous.clone() ),
	} );

	let correct_result: Expression = Expression::BinaryExpression( BinaryExpressionData{
		op: BinaryOperator::LessThan, 
		left: Box::new( replacement.clone() ),
		right: Box::new( superfluous.clone() ),
	} );
	substitute_variable_with_expression(&mut p, &target_var, &replacement);
	println!("result {:?}", p);
	println!("correct_result {:?}", correct_result);
	assert_eq!(p, correct_result);
}

#[test]
fn variable_mapping_data_equality() {
	let var1: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() };
	let var2: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() };
	let var3: VariableMappingData = VariableMappingData { name: "y".to_string(), var_type: "i32".to_string() };
	let var4: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "u32".to_string() };
	let var5: VariableMappingData = VariableMappingData { name: "y".to_string(), var_type: "u32".to_string() };
	let var6: VariableMappingData = VariableMappingData { name: "".to_string(), var_type: "".to_string() };

	assert!(var1 == var1);
	assert!(var1 == var2);
	assert!(var1 != var3);
	//FIXME: Following line should be included once type information is included in equality checking
	//assert!(var1 != var4); 
	assert!(var1 != var5);
	assert!(var1 != var6);
	assert!(var6 == var6);
}