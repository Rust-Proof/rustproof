#![feature(plugin, custom_attribute)]
#![plugin(rustproof(debug))]
#![allow(dead_code)]
fn main() {}


fn test() {
    #![condition(pre="true", post="true")]
    assert_eq!(true, true);
}
