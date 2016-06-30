#![feature(plugin, custom_attribute)]
#![plugin(mirtest)]

fn main() {
    let mut x = 3;
    x = add_five(x);
    println!("{:#?}", x);
}

fn add_five(mut x: i32) -> i32 {
    x = x + 5;
    return x;
}
