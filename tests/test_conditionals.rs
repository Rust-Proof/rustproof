#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
fn main() {}

// Tests that should return 'valid'
#[condition(pre="(x: i32 < i32::MAX - 6i32) && (y: bool == false)", post="return: i32 == (x: i32 + 6i32)")]
fn valid_variable_conditional(x: i32, y: bool) -> i32 {
    if y {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="x: i32 < i32::MAX - 6i32", post="return: i32 == (x: i32 + 6i32)")]
fn valid_literal_conditional(x: i32) -> i32 {
    if false {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="(x: i32 < i32::MAX - 6i32) && (x: i32 < 5i32)", post="return: i32 == (x: i32 + 6i32)")]
fn valid_expression_conditional(x: i32) -> i32 {
    if x > 5 {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="x: i32 == 5i32", post="return: i32 == (x: i32 + 6i32)")]
fn valid_triple_branch_conditional(x: i32) -> i32 {
    if x > 5 {
        x + 5
    } else if x < 5 {
        x + 7
    } else {
        x + 6
    }
}

#[condition(pre="(x: i32 == 4i32) && (y: bool == false)", post="return: i32 == (x: i32 + 6i32)")]
fn valid_nested_conditionals(x: i32, y: bool) -> i32 {
    if y {
        x + 5
    } else {
        if x > 5 {
            x + 7
        } else {
            x + 6
        }
    }
}

// Tests that should return 'invalid'
#[condition(pre="(x: i32 < i32::MAX - 6i32) && (y: bool == true)", post="return: i32 == (x: i32 + 6i32)")]
fn invalid_variable_conditional(x: i32, y: bool) -> i32 {
    if y {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="x: i32 < i32::MAX - 6i32", post="return: i32 == (x: i32 + 6i32)")]
fn invalid_literal_conditional(x: i32) -> i32 {
    if true {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="(x: i32 < i32::MAX - 6i32) && (x: i32 < 5i32)", post="return: i32 == (x: i32 + 6i32)")]
fn invalid_expression_conditional(x: i32) -> i32 {
    if x < 5 {
        x + 5
    } else {
        x + 6
    }
}

#[condition(pre="x: i32 == 5i32", post="return: i32 == (x: i32 + 6i32)")]
fn invalid_triple_branch_conditional(x: i32) -> i32 {
    if x > 5 {
        x + 5
    } else if x == 5 {
        x + 7
    } else {
        x + 6
    }
}

#[condition(pre="(x: i32 == 4i32) && (y: bool == false)", post="return: i32 == (x: i32 + 6i32)")]
fn invalid_nested_conditionals(x: i32, y: bool) -> i32 {
    if y {
        x + 5
    } else {
        if x < 5 {
            x + 7
        } else {
            x + 6
        }
    }
}
