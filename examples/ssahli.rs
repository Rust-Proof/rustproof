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
#[condition(pre="x: int > 0", post="x: int >= 5")]
fn add_five_or_three(x: i32) -> i32 {
    if x > 3 {
        x + 5
    }
    else {
        x + 3
    }
}
/*
fn add_three(mut x: i32) -> i32 {
    x = x + 3;
    return x;
}
*/
// This should error/warn: it isn't placed over a function
/*
#[condition(pre="x: int > 0", post="x: int >= 5")]
struct Foo {
    x: i32,
}

struct Bar;
*/
