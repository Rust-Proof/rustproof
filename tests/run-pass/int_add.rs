#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

#![allow(dead_code)]
#![allow(unused_attributes)]
#![deny(warnings)]

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= i32::MAX - 5:i32) && (x: i32 >= i32::MIN + 5:i32)", post="return:
i32 == (x: i32 +5:i32)")]
fn add_five_i32(x: i32) -> i32 {
    x+5
}


fn main() {
    let _ = add_five_i32(5);
}
