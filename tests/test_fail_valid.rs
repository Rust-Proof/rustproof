#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// Used to test the tester

// Condition is invalid, and function name indicates "valid", so should be detected as a mismatch
#[condition(pre="x: i32 <= i32::MAX - 4i32", post="return: i32 == (x: i32 + 5i32)")]
fn valid_add_five_i32(x: i32) -> i32 {
    x+5
}
