#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let x = add_five(5);
}

// WP=      x+5 = x+5 & x < MAX-5
#[condition(pre="(x: i32 < 2147483642) && (x: i32 > -2147483653)", post="return: i32 == (x: i32 +5)")]
fn add_five(x: i32) -> i32 {
    x+5
    //assert!(1 > 0)
}
