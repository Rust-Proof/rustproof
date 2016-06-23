#![feature(plugin, custom_attribute)]
#![plugin(mir_dump2)]

extern crate rustproof;

#[precondition]
fn main() {
    let mut x = 3;
    x = add_five(x);
    println!("{:?}", x);
}

#[precondition(cond="x > 0")]
fn add_five(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}

#[precondition]
struct Foo {
    x: i32,
}
