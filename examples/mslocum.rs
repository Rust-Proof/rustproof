#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() { }

// Should be invalid
#[condition(pre="(x: i32 <= i32::MAX - 4:i32) && (x: i32 >= i32::MIN + 5:i32)", post="return: i32 == (x: i32 +5:i32)")]
fn add_five(x: i32) -> i32 {
    x+5
}

// Should be valid
#[condition(pre="(x: i32 <= i32::MAX - 5:i32) && (x: i32 >= i32::MIN + 5:i32)", post="return: i32 == (x: i32 +5:i32)")]
fn add_five_again(x: i32) -> i32 {
    x+5
}

// Should be valid
#[condition(pre="true", post="true")]
fn asrt() {
    assert!(1 > 0)
}
