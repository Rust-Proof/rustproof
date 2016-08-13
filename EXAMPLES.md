# Example Usages

## Integer Arithmetic with Overflow Checking
```
// Proven to never overflow
#[condition(pre="(x:i32 <= i32::MAX - 5:i32)", post="return:i32 == (x:i32 + 5:i32)")]
fn add_five(x:i32) -> i32 {
    assert!(x <= 2147483647-5);
    x+5
}
```

## Boolean Conditional
```
// Proven to always return the NOT of the provided argument
#[condition(pre="true", post="(x:bool==true IMPLIES return:bool==false) && (x:bool==false IMPLIES return:bool==true)")]
fn boolean_not(x:bool) -> bool {
    if x == true {
        false
    } else {
        true
    }
}
```
