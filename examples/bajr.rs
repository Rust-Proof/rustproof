#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
fn main() {
    let x = add_five(5);
    let y = add_two(5);
}

#[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32", post="return:i32 == x:i32 + 5:i32")]
fn add_five(x: i32) -> i32 {
    if false {
        x+2
    } else {
        x+2
    }
}

#[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32", post="return:i32 == (x:i32 + 5:i32)")]
fn add_two(x: i32) -> i32 {
    x+5
}
