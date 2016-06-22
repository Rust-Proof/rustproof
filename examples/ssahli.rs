#![feature(plugin)]
#![plugin(mir_dump)]

extern crate rustproof;

fn main() {
    hello_world();
    println!("Goodbye, world!");
}

fn hello_world() {
    println!("Hello, world!");
}
