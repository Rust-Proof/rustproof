#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// FIXME: some preconditions are unnecisarilly restricitve ie add_five_u32_invalid does not need x: u32 >= u32::MIN + 5u32

// * * *
// Integer Add Tests
// * * *


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 >= 0u32)", post="return: u32 > 5u32")]
fn invalid_add_five_u32(x: u32) -> u32 {
    x + 5
}


// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 >= 0u32)", post="return: u32 > 4u32")]
fn valid_add_five_u32(x: u32) -> u32 {
    x + 5
}


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 >= 0u32)", post="return: u32 > 10u32")]
fn invalid_subtract_five_u32(x: u32) -> u32 {
    x - 5
}

// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 > 5u32)", post="return: u32 < 6u32")]
fn valid_subtract_five_u32(x: u32) -> u32 {
    x - 5
}

// Test to check unsigned subtration underflow
// Should be valid
#[condition(pre="x:u64 == 7u64", post="return:u64 == 5u64")]
fn valid_subtraction_overflow(x:u64) -> u64 {
    return x-2;
}


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 >= 0u32)", post="return: u32 < 0u32")]
fn invalid_multiply_five_u32(x: u32) -> u32 {
    x * 5u32
}

// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 <= 10u32) && (x: u32 >= 0u32)", post="return: u32 >= 0u32")]
fn valid_multiply_five_u32(x: u32) -> u32 {
    x * 5
}

// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 >= 0u32) && (x: u32 <= 10u32)", post="return: u32 >= 0u32")]
fn invalid_divide_five_u32(x: u32) -> u32 {
    5 / x
}


// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0u32) && (x: u32 <= 10u32)", post="return: u32 >= 0u32")]
fn valid_divide_five_u32(x: u32) -> u32 {
    2 / x
}


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 >= 0u32) && (x: u32 <= 10u32)", post="return: u32 < 0u32")]
fn invalid_mod_five_u32(x: u32) -> u32 {
    x % 5
}

// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0u32) && (x: u32 <= 10u32)", post="return: u32 >= 0u32")]
fn valid_mod_five_u32(x: u32) -> u32 {
    x % 5
}


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0u32) && (x: u32 < 10u32)", post="return: u32 < 0u32")]
fn invalid_bit_shift_left_two_u32(x: u32) -> u32 {
    x << 2u32
}

// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0u32) && (x: u32 < 10u32)", post="return: u32 > 0u32")]
fn valid_bit_shift_left_two_u32(x: u32) -> u32 {
    x << 2u32
}


// Tests unsigned integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0u32) && (x: u32 < 10u32)", post="return: u32 > 0u32")]
fn invalid_bit_shift_right_two_u32(x: u32) -> u32 {
    x >> 2u32
}

// Tests unsigned integer for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0u32) && (x: u32 < 10u32)", post="return: u32 >= 0u32")]
fn valid_bit_shift_right_two_u32(x: u32) -> u32 {
    x >> 2u32
}

// Tests unsigned integer for 16 bit integers
// Should be valid
#[condition(pre="(x: u8 < 10u8) && (x: u8 > 0u8)", post="return: u8 > 3u8")]
fn valid_add_5_u8(x: u8) -> u8 {
    x+5u8
}


// Tests unsigned integer for 16 bit integers
// Should be valid
#[condition(pre="(x: u16 < 10u16) && (x: u16 > 0u16)", post="return: u16 > 3u16")]
fn valid_add_5_u16(x: u16) -> u16 {
    x+5
}

// Tests unsigned integer for 64 bit integers
// Should be valid
#[condition(pre="(x: u64 < 10u64) && (x: u64 > 0u64)", post="return: u64 > 3u64")]
fn valid_add_5_u64(x: u64) -> u64 {
    x+5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 == u32::MAX)", post="return: u32 > 20u32")]
fn invalid_add_five_overflow_u32(x: u32) -> u32 {
    x + 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 == u32::MIN)", post="return: u32 < 20u32")]
fn invalid_subtract_five_overflow_u32(x: u32) -> u32 {
    x - 5
}

// Tests signed integer for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 == u32::MAX)", post="true")]
fn invalid_multiply_two_overflow_u32(x: u32) -> u32 {
    x * 2
}

// Tests signed integer for 16 bit integers
// Should be invalid
#[condition(pre="(x: u16 == u16::MAX)", post="return: u16 > 20u16")]
fn invalid_add_five_overflow_u16(x: u16) -> u16 {
    x + 5
}

// Tests signed integer for 16 bit integers
// Should be invalid
#[condition(pre="(x: u16 == u16::MIN)", post="return: u16 < 20u16")]
fn invalid_subtract_five_overflow_u16(x: u16) -> u16 {
    x - 5
}

// Tests signed integer for 8 bit integers
// Should be invalid
#[condition(pre="(x: u8 == u8::MAX)", post="return: u8 > 20u8")]
fn invalid_add_five_overflow_u8(x: u8) -> u8 {
    x + 5
}

// Tests signed integer for 8 bit integers
// Should be invalid
#[condition(pre="(x: u8 == u8::MIN)", post="return: u8 < 20u8")]
fn invalid_subtract_five_overflow_u8(x: u8) -> u8 {
    x - 5
}

// Tests signed integer for 64 bit integers
// Should be invalid
#[condition(pre="(x: u64 == u64::MAX)", post="return: u64 > 20u64")]
fn invalid_add_five_overflow_u64(x: u64) -> u64 {
    x + 5
}

// Tests signed integer for 64 bit integers
// Should be invalid
#[condition(pre="(x: u64 == u64::MIN)", post="return: u64 < 20u64")]
fn invalid_subtract_five_overflow_u64(x: u64) -> u64 {
    x - 5
}
