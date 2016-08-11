#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// * * *
// Integer Add Tests
// * * *

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= i32::MAX - 4:i32) && (x: i32 >= i32::MIN + 5:i32)", post="return: i32 == (x: i32 +5:i32)")]
fn add_five_i32_invalid(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= i32::MAX - 5:i32) && (x: i32 >= i32::MIN + 5:i32)", post="return: i32 == (x: i32 +5:i32)")]
fn add_five_i32_valid(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 64 bit integers
// Should be invalid
#[condition(pre="(x: i64 <= i64::MAX - 4:i64) && (x: i64 >= i64::MIN + 5:i64)", post="return: i64 == (x: i64 +5:i64)")]
fn add_five_i64_invalid(x: i64) -> i64 {
    x+5
}

// Tests signed integer overflow for 64 bit integers
// Should be invalid
#[condition(pre="(x: i64 <= i64::MAX - 5:i64) && (x: i64 >= i64::MIN + 5:i64)", post="return: i64 == (x: i64 +5:i64)")]
fn add_five_i64_valid(x: i64) -> i64 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="x: u32 <= u32::MAX - 4:u32", post="return: u32 == (x: u32 + 5:u32)")]
fn add_five_u32_invalid(x: u32) -> u32 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="x: u32 <= u32::MAX - 5:u32", post="return: u32 == (x: u32 + 5:u32)")]
fn add_five_u32_valid(x: u32) -> u32 {
    x+5
}

// * * *
// Assertion Tests
// * * *

// Should be valid
#[condition(pre="true", post="true")]
fn asrt_basic_valid() {
    assert!(1 > 0)
}

// Should be invalid
#[condition(pre="true", post="true")]
fn asrt_basic_invalid() {
    assert!(1 < 0)
}

// FIXME ERROR This test fails to compile.
// Should be valid
//#[condition(pre="x:bool == true", post="true")]
fn asrt_argument_valid(x: bool) {
    assert!(x)
}

// * * *
// Conditional Branch Tests
// * * *

// Should be valid
#[condition(pre="true", post="true")]
fn basic_literal_if_valid() {
    if true {
        assert!(1 > 0)
    } else {
        assert!(1 < 0)
    }
}

// Should be invalid
#[condition(pre="true", post="true")]
fn basic_literal_if_invalid() {
    if true {
        assert!(1 < 0)
    } else {
        assert!(1 > 0)
    }
}

// Should be valid
#[condition(pre="true", post="true")]
fn large_branch_literal_if_valid() {
    if false {
        assert!(1 < 0)
    } else if true {
        assert!(1 > 0)
    } else {
        assert!(1 < 0)
    }
}

// Should be invalid
#[condition(pre="true", post="true")]
fn large_branch_literal_if_invalid() {
    if false {
        assert!(1 < 0)
    } else if false {
        assert!(1 > 0)
    } else {
        assert!(1 < 0)
    }
}
