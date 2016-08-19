#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![deny(warnings)]

fn main() {
    let _ = add_one(1);
}

#[condition(pre="x: i32 <= i32::MAX",
            post="return : i32 == (x : i32 + 1 : i32)")]
fn add_one(x : i32) -> i32 {
    x+1
}
