#![feature(plugin, custom_attribute)]
#![plugin(rustproof)]
#![allow(dead_code)]
fn main() {
//    let x = add_five(5);
    let y = add_two(5);
}

//  (declare-fun x () (_ BitVec 64))
//  (assert (not (=> (and (bvslt x (_ bv2147483642 64))
//                        (bvsgt x (bvneg (_ bv2147483648 64))))
//                   (and (and (= (bvadd x (_ bv2 64))
//                                (bvadd x (_ bv5 64)))
//                             (bvsgt (bvadd x (_ bv2 64)) (_ bv18446744071562067968 64)))
//                        (bvslt (bvadd x (_ bv2 64)) (_ bv2147483647 64))))))
//  Verification Condition is not valid.
//
//  #[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32", post="return:i32 == x:i32 + 5:i32")]
//  fn add_five(x: i32) -> i32 {
//      if false {
//          x+2
//      } else {
//          x+2
//      }
//  }

//  (declare-fun x () (_ BitVec 64))
//  (assert (not (=> (and (bvslt x (_ bv2147483642 64)) (bvsgt x (bvneg (_ bv2147483648 64))))
//                   (and (=> (false)
//                            (and (and (= (bvadd x (_ bv2 64))
//                                         (bvadd x (_ bv5 64)))
//                                      (bvsgt (bvadd x (_ bv2 64))
//                                             (_ bv18446744071562067968 64)))
//                                 (bvslt (bvadd x (_ bv2 64))
//                                        (_ bv2147483647 64))))
//                        (=> (not (false))
//                            (and (and (= (bvadd x (_ bv2 64))
//                                         (bvadd x (_ bv5 64)))
//                                      (bvsgt (bvadd x (_ bv2 64))
//                                             (_ bv18446744071562067968 64)))
//                                 (bvslt (bvadd x (_ bv2 64)) (_ bv2147483647 64))))))))
//  
//  Verification Condition is valid!
#[condition(pre="x:i32 < 2147483642:i32 && x:i32 > -2147483648:i32", post="return:i32 == x:i32 + 5:i32")]
fn add_two(x: i32) -> i32 {
    x+5
}
