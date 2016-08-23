#![feature(plugin, custom_attribute)]
#![plugin(rustproof(debug))]
#![allow(dead_code)]
fn main() {}

#[condition(pre="true", post="true")]
fn test() {
    assert_eq!(true, true);
}
