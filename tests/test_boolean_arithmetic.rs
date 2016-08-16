#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]

// * * *
// Boolean and Boolean Operator Tests
// * * *

//  &&, ||
// !, &, |, ^
// literals
// variables

// Simple boolean Tests
// && and || overators Tests
//
// Should be valid

#[condition(pre="x: bool == true", post="return:bool == true")]
fn valid_simple_bool_and(x:bool) -> bool {
    x && true
}

// Should be valid
#[condition(pre="x: bool == false", post="return:bool == false")]
fn valid_simple_bool_or(x:bool) -> bool {
    x || false
}

// Should be invalid
#[condition(pre="x: bool == true", post="return:bool == false")]
fn invalid_simple_bool_and_2(x:bool) -> bool {
    x && true
}

// Should be invalid
#[condition(pre="x: bool == false", post="return:bool == true")]
fn invalid_simple_bool_or_2(x:bool) -> bool {
    x || false
}

// * * *
// test ! Operator
// * * *

//should be invalid
#[condition(pre="x: bool == true", post="return:bool == true")]
fn invalid_simple_bool_not(x:bool) -> bool {
    !x && true
}

// Should be invalid
#[condition(pre="x: bool == false", post="return:bool == false")]
fn invalid_simple_bool_not_2(x:bool) -> bool {
    !x || false
}


// * * *
// test & and | operator
// * * *

#[condition(pre="x: bool == true", post="return:bool == true")]
fn valid_simple_bool_bitwise_and(x:bool) -> bool {
    x & true
}


/*
// Should be valid
#[condition(pre="x: bool == false", post="return:bool == false")]
fn valid_simple_bool_bitwise_or(x:bool) -> bool {
    x | true
}

// Should be invalid
#[condition(pre="x: bool == true", post="return:bool == false")]
fn invalid_simple_bool_bitwise_and_2(x:bool) -> bool {
    x & false
}

// Should be invalid
#[condition(pre="x: bool == false", post="return:bool == true")]
fn invalid_simple_bool_bitwise_or_2(x:bool) -> bool {
    x & false
}
*/
