#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
#![allow(unused_attributes)]
fn main() {}

// * * *
// Boolean and Boolean Operator Tests
// * * *

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
fn valid_attr_condition_and_and()-> bool {
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
fn valid_attr_condition_or_or()-> bool {
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
fn valid_attr_condition_xor_xor()-> bool {
    true
}


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
    if x {
        x | false
    } else {
        x | true
    }
}

// Should be valid
#[condition(pre="(x: bool == true) || (x: bool == false)", post="return:bool == true")]
fn valid_simple_bool_pre_or(x:bool) -> bool {
    if x {
        true
    } else {
        true
    }
}

// Should be invalid
#[condition(pre="(x: bool == true) && (y: bool == false)", post="return:bool == false")]
fn invalid_bool_pre_and(x:bool, y:bool) -> bool {
    if x | y {
        true
    } else {
        false
    }
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

    if z {
        a = a + 5;
    } else {
        a = a + 10;
    }

    if a > 9 && y {
        return true;
    } else {
        if x {
            return true;
        }
        else {
            return false;
        }
    }
}

// Should be invalid
#[condition(pre="(w: i32 >= 6i32) && (x: bool == false) && (y: bool == true) && (z: bool == true)", post="return:bool == true")]
fn invalid_bool_complex(x:bool, y:bool, z:bool, w:i32) -> bool {
    let mut a = 4;
    if w > 5 {
        return false;
    }
    if z {
        a = a + 5;
    } else {
        a = a + 10;
    }

    if a > 9 && y {
        return true;
    } else {
        if x {
            return true;
        } else {
            return false;
        }
    }
}
