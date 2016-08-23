#![feature(plugin, custom_attribute)]
#![plugin(rustproof(debug))]
#![allow(dead_code)]


// WP=      x+5 = x+5 & x < MAX-5
// #[condition(pre="x < MAX-5", post="return = x+5")]
fn main() {}
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

    x * 5

}
