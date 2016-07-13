#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let x = add_five(1);
}

#[condition(pre="x   > 0", post="x  >= 5")]
fn add_five(x: i32) -> i32 {
    x+5
}
