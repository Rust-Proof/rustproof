#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let x = add_five(5);
}

// WP=      x+5 = x+5 & x < MAX-5
#[condition(pre="(x: int < 2147483642) && (x: int > -2147483653)", post="return: int == (x: int +5)")]
fn add_five(x: i32) {
    assert!(1 > 0)
    //x+5
}
