#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() {}

// Used to test the tester

// Should be invalid but function name specifies valid
#[condition(pre="x: i32 <= i32::MAX - 4:i32", post="return: i32 == (x: i32 + 5:i32)")]
fn valid_add_five_i32(x: i32) -> i32 {
    x+5
}
