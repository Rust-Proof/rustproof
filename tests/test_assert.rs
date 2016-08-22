#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// Should be valid
#[condition(pre="x: i32 == 10i32", post="true")]
fn valid_simple_assertion(x: i32) {
    assert!(x > 0);
}

// Should be invalid
#[condition(pre="x: i32 == 10i32", post="true")]
fn invalid_simple_assertion(x: i32) {
    assert!(x < 10);
}


//FIXME: This portion doesn't work currently.
/*
// Should be valid
#[condition(pre="x: i32 == 10i32", post="true")]
fn valid_simple_assertion_eq(x: i32) {
    assert_eq!(x, 10);
}

// Should be invalid
#[condition(pre="x: i32 == 10i32", post="true")]
fn invalid_simple_assertion_eq(x: i32) {
    assert_eq!(x, 0);
}
*/
