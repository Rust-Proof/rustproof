#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]

fn main() {
    let w = test_slt(5);
//      let x = if_add_five(5);
//      let y = add_five(5);
//      let z = sub_five(5);
}

#[condition(pre="(x: i32 <= i32::MAX) && (x: i32 >= i32::MIN)",
            post="return: i32 == (x: i32 + 5:i32)")]
fn test_slt(x: i32) -> i32 {
    x+5
}

//  // This should be valid
//  #[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32",
//              post="return:i32 == (x:i32 - 5:i32)")]
//  fn sub_five(x: i32) -> i32 {
//      x-5
//  }
//  
//  // This should be valid
//  #[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32",
//              post="return:i32 == (x:i32 + 5:i32)")]
//  fn if_add_five(x: i32) -> i32 {
//      if false {
//          x+5
//      } else {
//          x+5
//      }
//  }
//  
//  // This should be valid
//  #[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32",
//              post="return:i32 == (x:i32 + 5:i32)")]
//  fn add_five(x: i32) -> i32 {
//      x+5
//  }


