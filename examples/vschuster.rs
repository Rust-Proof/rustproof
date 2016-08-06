#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let x = add_five(5);
}

// WP=      x+5 = x+5 & x < MAX-5
// #[condition(pre="x < MAX-5", post="return = x+5")]

#[condition(pre="x:i32 < 2147483642:i32", post="return:i32 == x:i32 + 5:i32")]
fn add_five(x: i32) -> i32 {
    if true
        {5 % x}
    else
        {x}
}
