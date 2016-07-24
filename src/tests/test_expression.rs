// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use expression::{Term, Predicate, IntegerComparisonOperator, BinaryPredicateData, VariableMappingData, IntegerComparisonData};
use expression::{substitute_variable_in_predicate_with_term, substitute_variable_in_term_with_term};

#[test]
fn simple_replacement() {
	let target_var : VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() };
	let target: Term = Term::VariableMapping( target_var.clone() );
	let superfluous: Term = Term::VariableMapping( VariableMappingData { name: "z".to_string(), var_type: "i32".to_string() } );
	let replacement: Term = Term::VariableMapping( VariableMappingData { name: "y".to_string(), var_type: "i32".to_string() } );
	let p: Predicate = Predicate::IntegerComparison( IntegerComparisonData{
		op: IntegerComparisonOperator::LessThan, 
		t1: Box::new( target.clone() ),
		t2: Box::new( superfluous.clone() ),
	} );

	let correct_result: Predicate = Predicate::IntegerComparison( IntegerComparisonData{
		op: IntegerComparisonOperator::LessThan, 
		t1: Box::new( replacement.clone() ),
		t2: Box::new( superfluous.clone() ),
	} );
	let result = substitute_variable_in_predicate_with_term(p, target_var, replacement);
	println!("result {:?}", result);
	println!("correct_result {:?}", correct_result);
	assert_eq!(result, correct_result);
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

//substitute_variable_in_predicate_with_term()

#[test]
fn substitute_variable_in_predicate_with_term_test() {

}


//substitute_variable_in_term_with_term()

#[test]
fn variable_mapping_data_replaced_with_variable_mapping_data() {
	let var1: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() };
	let term1: Term = Term::VariableMapping(VariableMappingData { name: "x".to_string(), var_type: "i32".to_string() });
	let term2: Term = Term::VariableMapping(VariableMappingData { name: "y".to_string(), var_type: "u32".to_string() });
	let term3: Term = Term::VariableMapping(VariableMappingData { name: "y".to_string(), var_type: "u32".to_string() });

	let term4: Term = substitute_variable_in_term_with_term(term1, var1, term2);
	match term4 {
		Term::VariableMapping(v) => {
			match term3 {
				Term::VariableMapping(u) => {
					assert!(v == u);
				},
				_ => {}
			}
		},
		_ => {}
	}
}