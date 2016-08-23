#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]


// WP=      x+5 = x+5 & x < MAX-5
// #[condition(pre="x < MAX-5", post="return = x+5")]
fn main() {}
// Tests signed integer for 32 bit integers

// Should be invalid and return model

#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 > 0i32")]

fn multiply_5(x: i32) -> i32 {

    x * 5i32

}
