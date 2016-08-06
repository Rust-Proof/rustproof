#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    add_five(0);
}

#[condition(pre="true", post="true")]
fn add_five(x: i32) {
    assert!(1 < 2);
}
