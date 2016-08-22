#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() {}



// Should be valid
#[condition(pre="true", post="true")]
fn valid_basic_boolean() -> bool {
    true
}

// Should be invalid
#[condition(pre="true", post="false")]
fn invalid_basic_boolean()-> bool {
    true
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_and_logical(x:bool) -> bool {
    x && true
}

// Should be invalid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn invalid_boolean_and_logical(x:bool) -> bool {
    x && false
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_and_bitwise(x:bool) -> bool {
    x & true
}

// Should be invalid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn invalid_boolean_and_bitwise(x:bool) -> bool {
    x & false
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_or_logical(x:bool) -> bool {
    x || true
}
// Should be invalid
#[condition(pre="x:bool == true", post="return:bool == false")]
fn invalid_boolean_or_logical(x:bool) -> bool {
    x || true
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn valid_boolean_or_bitwise(x:bool) -> bool {
    x | true
}
// Should be invalid
#[condition(pre="x:bool == true", post="return:bool == false")]
fn invalid_boolean_or_bitwise(x:bool) -> bool {
    x | true
}

// Should be valid
#[condition(pre="x:bool == true", post="return:bool == false")]
fn valid_boolean_not_variable(x:bool) -> bool {
    !x
}

// Should be invalid
#[condition(pre="x:bool == true", post="return:bool == true")]
fn invalid_boolean_not_variable(x:bool) -> bool {
    !x
}

// Should be valid
#[condition(pre="x:bool == true", post="return: bool == false")]
fn valid_boolean_not_literal() -> bool {
    !true
}

// Should be invalid
#[condition(pre="x:bool == true", post="return: bool == true")]
fn invalid_boolean_not_literal() -> bool {
    !true
}

// Should be valid
#[condition(pre="x:bool == true", post="return: bool == false")]
fn valid_boolean_xor_bitwise(x: bool) -> bool {
    x ^ true
}

// Should be invalid
#[condition(pre="x:bool == true", post="return: bool == true")]
fn invalid_boolean_xor_bitwise(x : bool) -> bool {
    x ^ true
}

// Should be valid
#[condition(pre="true",post="true AND true")]
fn valid_attr_condition_and_AND()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true && true")]
fn valid_attr_condition_and_logical()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true & true")]
fn valid_attr_condition_and_bitwise()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true OR true")]
fn valid_attr_condition_or_OR()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true || true")]
fn valid_attr_condition_or_logical()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true | true")]
fn valid_attr_condition_or_bitwise()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true ^ false")]
fn valid_attr_condition_xor_bitwise()-> bool {
    true
}

// Should be valid
#[condition(pre="true",post="true XOR false")]
fn valid_attr_condition_xor_XOR()-> bool {
    true
}
