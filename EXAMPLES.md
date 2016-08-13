# Example Usages

## Integer Arithmetic with overflow checking
```
// Proven to never overflow
#[condition(pre="(x:i32 <= i32::MAX - 5:i32)", post="return:i32 == (x:i32 + 5:i32)")]
fn add_five(x:i32) -> i32 {
    x+5
}
```
