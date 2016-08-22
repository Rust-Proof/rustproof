#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: i32 = 5;
	invalid_bit_shift_left_two_i32(x);
}

// invalid
#[condition(pre="(x: i32 > 0i32) && (x:i32 < 10i32)", post="return: i32 < 0i32")]
fn invalid_bit_shift_left_two_i32(x: i32) -> i32 {
		x << 2i32
}
