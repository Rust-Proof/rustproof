#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// FIXME: some preconditions are unnecisarilly restricitve ie add_five_i32_invalid does not need x: i32 >= i32::MIN + 5i32

// * * *
// Integer Add Tests
// * * *

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= i32::MAX - 4i32) && (x: i32 >= i32::MIN + 5i32)", post="return: i32 == (x: i32 + 5i32)")]
fn invalid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= i32::MAX - 5i32) && (x: i32 >= i32::MIN + 5i32)", post="return: i32 == (x: i32 + 5i32)")]
fn valid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 64 bit integers
// Should be invalid
#[condition(pre="(x: i64 <= i64::MAX - 4i64) && (x: i64 >= i64::MIN + 5i64)", post="return: i64 == (x: i64 + 5i64)")]
fn invalid_add_five_i64(x: i64) -> i64 {
    x+5
}

// Tests signed integer overflow for 64 bit integers
// Should be valid
#[condition(pre="(x: i64 <= i64::MAX - 5i64) && (x: i64 >= i64::MIN + 5i64)", post="return: i64 == (x: i64 + 5i64)")]
fn valid_add_five_i64(x: i64) -> i64 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="x: u32 <= u32::MAX - 4u32", post="return: u32 == (x: u32 + 5u32)")]
fn invalid_add_five_u32(x: u32) -> u32 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="x: u32 <= u32::MAX - 5u32", post="return: u32 == (x: u32 + 5u32)")]
fn valid_add_five_u32(x: u32) -> u32 {
    x+5
}

// * * *
// Assertion Tests
// * * *

// Should be valid
#[condition(pre="true", post="true")]
fn valid_asrt_basic() {
    assert!(1 > 0)
}

// Should be invalid
#[condition(pre="true", post="true")]
fn invalid_asrt_basic() {
    assert!(1 < 0)
}

// FIXME ERROR This test fails to compile.
// Should be valid
//#[condition(pre="x:bool == true", post="true")]
fn valid_asrt_argument(x: bool) {
    assert!(x)
}

// * * *
// Conditional Branch Tests
// * * *

// Should be valid
#[condition(pre="true", post="true")]
fn valid_basic_literal_if() {
    if true {
        assert!(1 > 0)
    } else {
        assert!(1 < 0)
    }
}

//Matt's example. Should be invalid. Calling it valid
#[condition(pre="true",post="true")]
fn invalid_fake_function(){
    if true {
        assert!(1<0)
    } else {
        assert!(1>0)
    }
}

// Should be invalid
#[condition(pre="true", post="true")]
fn invalid_basic_literal_if() {
    if true {
        assert!(1 < 0)
    } else {
        assert!(1 > 0)
    }
}

// Should be valid
#[condition(pre="true", post="true")]
fn valid_large_branch_literal_if() {
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
fn invalid_large_branch_literal_if() {
    if false {
        assert!(1 < 0)
    } else if false {
        assert!(1 > 0)
    } else {
        assert!(1 < 0)
    }
}

// * * *
// Boolean Tests
// * * *

// Should be valid
#[condition(pre="x:bool == true", post="true")]
fn valid_boolean_comparison_in_condition(x:bool) -> bool {
    x
}

// Should be invalid
#[condition(pre="x:bool == true", post="false")]
fn invalid_boolean_comparison_in_condition(x:bool) -> bool {
    x
}

// Should be valid
#[condition(pre="true", post="return:bool == true")]
fn valid_simple_bool(x:bool) -> bool {
    x || true
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_condition(x:bool) -> bool {
    if x {
        true
    } else {
        false
    }
}

// Should be invalid
#[condition(pre="x:bool == false", post="return:bool == true")]
fn invalid_boolean_condition(x:bool) -> bool {
    if x {
        true
    } else {
        false
    }
}

