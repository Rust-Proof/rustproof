#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]

//extern crate rustproof;

fn main() {
    let mut x = 3;
    x = add_five(x);
    println!("{:?}", x);
}

#[condition(pre="x: i32 > 0: i32", post="return: i32 >= 5: i32")]
fn add_five(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}

/*struct Foo {
    x: i32,
}
*/
