#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: i32 = 5;
	invalid_multiply_five_i32(x);
	valid_multiply_five_i32(x);
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 < 0i32")]
fn invalid_multiply_five_i32(x: i32) -> i32 {
    x * 5i32
}
// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 >= 0i32")]
fn valid_multiply_five_i32(x: i32) -> i32 {
    x * 5i32
}