#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {}
// This is acceptable: it is placed over a function
#[condition(pre="x: i32 > 0", post="return: i32 >= 5")]
fn add_five_or_three(x: i32) -> i32 {
    if x > 3 {
        x + 5
    }
    else {
        x + 3
    }
}
