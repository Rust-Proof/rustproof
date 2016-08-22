#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

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
fn main() {}
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



// Should be invalid
#[condition(pre="x: bool == false", post="return:bool == false")]
fn invalid_simple_bool_bitwise_or(x:bool) -> bool {
    x | true
}

// Should be invalid

#[condition(pre="x: bool == true", post="return:bool == true")]
fn invalid_simple_bool_bitwise_and_2(x:bool) -> bool {
    x & false
}

// Should be valid
#[condition(pre="x: bool == true", post="return:bool == true")]
fn valid_simple_bool_bitwise_or_2(x:bool) -> bool {
    x | false
}

// * * *
// Additional Tests
// * * *

// Should be valid
#[condition(pre="x: bool == false", post="return:bool == true")]
fn valid_simple_bool_bitwise_or_if_else(x:bool) -> bool {
    if x
        { x | false }
    else
        { x | true }
}

// Should be valid
#[condition(pre="(x: bool == true) || (x: bool == false)", post="return:bool == true")]
fn valid_simple_bool_pre_or(x:bool) -> bool {
    if x
        {true}
    else
        {true}
}

// Should be invalid
#[condition(pre="(x: bool == true) && (y: bool == false)", post="return:bool == false")]
fn invalid_bool_pre_and(x:bool, y:bool) -> bool {
    if x | y
        { true }
    else
        { false }
}

// Should be valid
#[condition(pre="x: bool == true", post="(return:bool == true) || (return:bool == false)")]
fn valid_bool_post_or(x:bool) -> bool {
    x | false
}

// Should be valid
#[condition(pre="(x: bool == true) && (y: bool == false) && (z: bool == true)", post="return:bool == true")]
fn valid_bool_complex(x:bool, y:bool, z:bool) -> bool {
    let mut a = 4;

    if z
        { a = a + 5; }
    else
        { a = a + 10; }

    if a > 9 && y
        { return true; }
    else
        {
            if x
                {return true;}
            else
                {return false;}
        }
}

// Should be invalid
#[condition(pre="(x: bool == false) && (y: bool == true) && (z: bool == true)", post="return:bool == true")]
fn invalid_bool_complex(x:bool, y:bool, z:bool) -> bool {
    let mut a = 4;

    if z
        { a = a + 5; }
    else
        { a = a + 10; }

    if a > 9 && y
        { return true; }
    else
        {
            if x
                {return true;}
            else
                {return false;}
        }
}
