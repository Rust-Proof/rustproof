#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let mut x = 3;
    x = add_five(x);
    x = add_three(x);
    println!("{:?}", x);
}

// This is acceptable: it is placed over a function
#[condition(pre="x > 0", post="x >= 5")]
fn add_five(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}

fn add_three(mut x: i32) -> i32 {
    x = x + 3;
    return x;
}

// This should error/warn: it isn't placed over a function
#[condition(pre="x > 0", post="x >= 5")]
struct Foo {
    x: i32,
}

struct Bar;
