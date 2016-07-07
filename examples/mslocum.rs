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
fn main() {
    let x = add_five(1);
}

#[condition(pre="x > 0", post="x >= 5")]
fn add_five(x: i32) -> i32 {
    x+5
}
