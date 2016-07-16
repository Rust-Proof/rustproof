#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

extern crate rustproof;

use rustproof::expression::Predicate;
use rustproof::expression::BinaryPredicateData;
use rustproof::expression::BooleanBinaryOperator;

fn main() {
	let x = 7u32;
	let z = foo(x);
	println!("foo({}) = {}", x, z);
	let p = Predicate::BinaryExpression( BinaryPredicateData { op: BooleanBinaryOperator::And, p1: Box::new(Predicate::BooleanLiteral(true)), p2: Box::new(Predicate::BooleanLiteral(false)) } );
	println!("p: {}", p);
	let q = rustproof::parser::parse_condition("(3 + 1 & 2 <= (74 + var : i))");
	println!("q: {}", q);
	let r = rustproof::parser::parse_condition("5 - 2 + (8 / 5) << 6 > 5");
	println!("r: {}", r);
}

#[condition(pre="1 > 3", post="true && var : b")]
fn foo(x: u32) -> u32 {
	let y = 5u32;
	x + y
}