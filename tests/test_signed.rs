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
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 30)")]
fn invalid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 3)")]
fn valid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 10)")]
fn invalid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > -10)")]
fn valid_add_five_i32(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < -30)")]
fn invalid_subtract_five_i32(x: i32) -> i32 {
    x-5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 20)")]
fn valid_subtract_five_i32(x: i32) -> i32 {
    x-5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 0)")]
fn invalid_subtract_five_i32(x: i32) -> i32 {
    x-5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 0)")]
fn valid_subtract_five_i32(x: i32) -> i32 {
    x-5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 1")]
fn invalid_multiply_five_i32(x: i32) -> i32 {
    x*5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 1)")]
fn valid_multiply_five_i32(x: i32) -> i32 {
    x*5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 1)")]
fn invalid_multiply_five_i32(x: i32) -> i32 {
    x*5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 0)")]
fn valid_multiply_five_i32(x: i32) -> i32 {
    x*5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 1")]
fn invalid_divide_five_i32(x: i32) -> i32 {
    x/5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 1)")]
fn valid_divide_five_i32(x: i32) -> i32 {
    x/5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 1)")]
fn invalid_divide_five_i32(x: i32) -> i32 {
    x/5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 0)")]
fn valid_divide_five_i32(x: i32) -> i32 {
    x/5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 0")]
fn invalid_mod_five_i32(x: i32) -> i32 {
    x%5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > -1)")]
fn valid_mod_five_i32(x: i32) -> i32 {
    x%5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 20)")]
fn invalid_mod_five_i32(x: i32) -> i32 {
    x%5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 20)")]
fn valid_mod_five_i32(x: i32) -> i32 {
    x%5
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 30")]
fn invalid_unary_minus_five_i32(x: i32) -> i32 {
    x-(-5)
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 0)")]
fn valid_unary_minus_five_i32(x: i32) -> i32 {
    x-(-5)
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 30)")]
fn invalid_unary_minus_five_i32(x: i32) -> i32 {
    x-(-5)
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 30)")]
fn valid_unary_minus_five_i32(x: i32) -> i32 {
    x-(-5)
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 0")]
fn invalid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > 0)")]
fn valid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 0)")]
fn invalid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 0)")]
fn valid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 < 0")]
fn invalid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0) && (x: i32 < 10)", post="return: (x: i32 > -1)")]
fn valid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2
}

// Tests signed integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 > 30)")]
fn invalid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0) && (x: i32 > -10)", post="return: (x: i32 < 10)")]
fn valid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2
}


// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 >= 0) && (x: i32 <= 10)", post="return: (x: i32 < 30)")]
fn valid_equal_test(x: i32) -> i32 {
    x+5
}

// Tests signed integer overflow for 16 bit integers
// Should be valid
#[condition(pre="(x: i16 == 1)", post="return: (x: i16 == 6)")]
fn valid_equal_only_test(x: i16) -> i16 {
    x+5
}

// Tests signed integer overflow for 64 bit integers
// Should be valid
#[condition(pre="(x: i64 >= 0) && (x: i64 <= 10)", post="return: (x: i64 != 30)")]
fn valid_not_equal_test(x: i64) -> i64 {
    x+5
}

// Tests signed integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 == 0) ^ (x: i32 == 1)", post="return: (x: i32 > 0)")]
fn valid_xor_test(x: i32) -> i32 {
    x+5
}

