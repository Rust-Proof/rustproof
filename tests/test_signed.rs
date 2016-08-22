#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// FIXME: some preconditions are unnecisarilly restricitve ie add_five_i32_invalid does not need x: i32 >= i32::MIN + 5i32

// * * *
// Integer Add Tests
// * * *

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 > 5i32")]
fn invalid_add_five_i32(x: i32) -> i32 {
    x + 5
}


// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 > 4i32")]
fn valid_add_five_i32(x: i32) -> i32 {
    x + 5
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 > 0i32")]
fn invalid_add_five_negative_i32(x: i32) -> i32 {
    x + 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 > -6i32")]
fn valid_add_five_negative_i32(x: i32) -> i32 {
    x + 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 > 4i32")]
fn invalid_subtract_five_i32(x: i32) -> i32 {
    x - 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 < 6i32")]
fn valid_subtract_five_i32(x: i32) -> i32 {
    x - 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 > 0i32")]
fn invalid_subtract_five_negative_i32(x: i32) -> i32 {
    x - 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 < -2i32")]
fn valid_subtract_five_negative_i32(x: i32) -> i32 {
    x - 5
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 < 0i32")]
fn invalid_multiply_five_i32(x: i32) -> i32 {
    x * 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 <= 10i32) && (x: i32 >= 0i32)", post="return: i32 >= 0i32")]
fn valid_multiply_five_i32(x: i32) -> i32 {
    x * 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 != -2i32")]
fn invalid_multiply_five_negative_i32(x: i32) -> i32 {
    x * 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 >= -10i32)", post="return: i32 > 0i32")]
fn valid_multiply_five_negative_i32(x: i32) -> i32 {
    x * 5
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 >= 0i32) && (x: i32 <= 10i32)", post="return: i32 >= 0i32")]
fn invalid_divide_five_i32(x: i32) -> i32 {
    5 / x
}


// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0i32) && (x: i32 <= 10i32)", post="return: i32 >= 0i32")]
fn valid_divide_five_i32(x: i32) -> i32 {
    2 / x
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 > 1i32")]
fn invalid_divide_five_negative_i32(x: i32) -> i32 {
    x / 2
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < 1i32")]
fn valid_divide_five_negative_i32(x: i32) -> i32 {
    x / 2
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 >= 0i32) && (x: i32 <= 10i32)", post="return: i32 < 0i32")]
fn invalid_mod_five_i32(x: i32) -> i32 {
    x % 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0i32) && (x: i32 <= 10i32)", post="return: i32 >= 0i32")]
fn valid_mod_five_i32(x: i32) -> i32 {
    x % 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 > 1i32")]
fn invalid_mod_five_negative_i32(x: i32) -> i32 {
    x % 5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < 1i32")]
fn valid_mod_five_negative_i32(x: i32) -> i32 {
    x % 5
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 > 10i32")]
fn invalid_unary_minus_five_i32(x: i32) -> i32 {
    x + (-(5))
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 < 10i32")]
fn valid_unary_minus_five_i32(x: i32) -> i32 {
    x + (-(5))
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 > 0i32")]
fn invalid_unary_minus_negative_five_i32(x: i32) -> i32 {
    x + (-(5))
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < 0i32")]
fn valid_unary_minus_five_negative_i32(x: i32) -> i32 {
    x + (-(5))
}

/*
// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 < 0i32")]
fn invalid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2i32
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 > 0i32")]
fn valid_bit_shift_left_two_i32(x: i32) -> i32 {
    x << 2i32
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 > 0i32")]
fn invalid_bit_shift_left_two_negative_i32(x: i32) -> i32 {
    x << 2i32
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < 2i32")]
fn valid_bit_shift_left_two_negative_i32(x: i32) -> i32 {
    x << 2i32
}


// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 > 0i32")]
fn invalid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2i32
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 > 0i32) && (x: i32 < 10i32)", post="return: i32 > -10i32")]
fn valid_bit_shift_right_two_i32(x: i32) -> i32 {
    x >> 2i32
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < 0i32")]
fn invalid_bit_shift_right_negative_two_i32(x: i32) -> i32 {
    x >> 2i32
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 < 0i32) && (x: i32 > -10i32)", post="return: i32 < -20i32")]
fn valid_bit_shift_right_two_negative_i32(x: i32) -> i32 {
    x >> 2i32
}
*/

// Tests signed integer for 16 bit integers
// Should be valid
#[condition(pre="(x: i16 < 10i16) && (x: i16 > 0i16)", post="return: i16 > 3i16")]
fn valid_equal_only_test(x: i16) -> i16 {
    x+5
}

// Tests signed integer for 64 bit integers
// Should be valid
#[condition(pre="(x: i64 < 10i64) && (x: i64 > 0i64)", post="return: i64 > 3i64")]
fn valid_not_equal_test(x: i64) -> i64 {
    x+5
}

// Tests signed integer for 32 bit integers
// Should be valid
#[condition(pre="(x: i32 == 0i32)", post="return: i32 == 10i32")]
fn valid_xor_test(x: i32) -> i32 {
    x+5
}

