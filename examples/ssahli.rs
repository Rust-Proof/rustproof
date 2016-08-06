#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
/*
fn main() {
    let mut x = 3;
    x = add_five(x);
    x = add_three(x);
    println!("{:?}", x);
}
*/
fn main() {}
// This is acceptable: it is placed over a function
#[condition(pre="(x:i32 < 2147483642:i32) && (x:i32 > -2147483653:i32)", post="return:i32 == (x:i32 + 5:i32)")]
fn add_five_or_three(x: i32) -> i32 {
    let y = (3, x, 5, 7);
    y.1 + 5
}
