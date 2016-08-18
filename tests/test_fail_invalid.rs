#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// Used to test the tester

// Condition is valid, and function name indicates "invalid", so should be detected as a mismatch
#[condition(pre="x: i32 <= i32::MAX - 5i32", post="return: i32 == (x: i32 + 5i32)")]
fn invalid_add_five_i32(x: i32) -> i32 {
    x+5
}
