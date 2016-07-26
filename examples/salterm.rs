#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

use rustproof::expression::Predicate;
use rustproof::expression::BinaryPredicateData;
use rustproof::expression::BooleanBinaryOperator;

fn main() {
	let x = 7i32;
	let z = foo(x);
	println!("foo({}) = {}", x, z);
	let p = Predicate::BinaryExpression( BinaryPredicateData { op: BooleanBinaryOperator::And, p1: Box::new(Predicate::BooleanLiteral(true)), p2: Box::new(Predicate::BooleanLiteral(false)) } );
	println!("p: {}", p);
	let q = rustproof::parser::parse_condition("(3 + 1 & 2 <= (74 + var: int))");
	println!("q: {}", q);
	let r = rustproof::parser::parse_condition("5 - 2 + (8 / 5) << 6 > 5");
	println!("r: {}", r);
}

#[condition(pre="x: int < 10 && x: int > 0", post="return: int > 5")]
fn foo(x: i32) -> i32 {
	let y: i32 = 7i32;
	1i32 + y
}
