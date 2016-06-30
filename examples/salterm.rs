#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

extern crate rustproof;

use rustproof::expression::VariableMappingData;

fn main() {
	let x = 7u32;
	let z = foo(x);

	let var1: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string()};
	let var2: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "i32".to_string()};
	let var3: VariableMappingData = VariableMappingData { name: "y".to_string(), var_type: "i32".to_string()};
	let var4: VariableMappingData = VariableMappingData { name: "x".to_string(), var_type: "u32".to_string()};
	let var5: VariableMappingData = VariableMappingData { name: "y".to_string(), var_type: "u32".to_string()};

	assert!(var1 == var1);
	assert!(var1 == var2);
	assert!(var1 != var3);
	assert!(var1 != var4);
	assert!(var1 != var5);
}

#[condition(pre="b", post="c")]
fn foo(x: u32) -> u32 {
	let y = 5u32;
	x + y
}