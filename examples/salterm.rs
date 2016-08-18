#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(plugin_as_library)]

extern crate rustproof;

fn main() {
	let x: u64 = 4u64;
	let y: i64 = 4i64;
	foo(x);
	bar(x);
	baz(y);

	let z: bool = true;
	complex_if(z);
}

// Invalid
#[condition(pre="x: u64 <= u64::MAX AND x: u64 >= u64::MIN", post="return: u64 == x: u64 + 5u64")]
fn foo(x: u64) -> u64 {
		x + 5u64
}

// Valid
#[condition(pre="x: u64 <= u64::MAX - 5u64 AND x: u64 >= u64::MIN", post="return: u64 == x: u64 + 5u64")]
fn bar(x: u64) -> u64 {
		x + 5u64
}

// Invalid
#[condition(pre="y: i64 <= i64::MAX AND y: i64 >= i64::MIN", post="return: i64 == y: i64 + 5i64")]
fn baz(y: i64) -> i64 {
		y + 5i64
}

// Invalid
#[condition(pre="z: bool == true", post="return: i32 != 1i32")]
fn complex_if(z: bool) -> i32 {
    if z {
    	1
    } else {
    	2
    }
}
