#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() { }

// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 30)")]
fn invalid_add_five_u32(x: u32) -> u32 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 3)")]
fn valid_add_five_u32(x: u32) -> u32 {
    x+5
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < -30)")]
fn invalid_subtract_five_u32(x: u32) -> u32 {
    x-5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 20)")]
fn valid_subtract_five_u32(x: u32) -> u32 {
    x-5
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 1")]
fn invalid_multiply_five_u32(x: u32) -> u32 {
    x*5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 1)")]
fn valid_multiply_five_u32(x: u32) -> u32 {
    x*5
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 1")]
fn invalid_divide_five_u32(x: u32) -> u32 {
    x/5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 1)")]
fn valid_divide_five_u32(x: u32) -> u32 {
    x/5
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 0")]
fn invalid_mod_five_u32(x: u32) -> u32 {
    x%5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > -1)")]
fn valid_mod_five_u32(x: u32) -> u32 {
    x%5
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 30")]
fn invalid_unary_minus_five_u32(x: u32) -> u32 {
    x-(-5)
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 0)")]
fn valid_unary_minus_five_u32(x: u32) -> u32 {
    x-(-5)
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 0")]
fn invalid_bit_shift_left_two_u32(x: u32) -> u32 {
    x << 2
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > 0)")]
fn valid_bit_shift_left_two_u32(x: u32) -> u32 {
    x << 2
}


// Tests unsigned integer overflow for 32 bit integers
// Should be invalid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 < 0")]
fn invalid_bit_shift_right_two_u32(x: u32) -> u32 {
    x >> 2
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 > 0) && (x: u32 < 10)", post="return: (x: u32 > -1)")]
fn valid_bit_shift_right_two_u32(x: u32) -> u32 {
    x >> 2
}



// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 >= 0) && (x: u32 <= 10)", post="return: (x: u32 < 30)")]
fn valid_equal_test(x: u32) -> u32 {
    x+5
}

// Tests unsigned integer overflow for 16 bit integers
// Should be valid
#[condition(pre="(x: u16 == 1)", post="return: (x: u16 == 6)")]
fn valid_equal_only_test(x: u16) -> u16 {
    x+5
}

// Tests unsigned integer overflow for 64 bit integers
// Should be valid
#[condition(pre="(x: u64 >= 0) && (x: u64 <= 10)", post="return: (x: u64 != 30)")]
fn valid_not_equal_test(x: u64) -> u64 {
    x+5
}

// Tests unsigned integer overflow for 32 bit integers
// Should be valid
#[condition(pre="(x: u32 == 0) ^ (x: u32 == 1)", post="return: (x: u32 > 0)")]
fn valid_xor_test(x: u32) -> u32 {
    x+5
}

