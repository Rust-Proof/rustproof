// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//use libsmt::backends::smtlib2::*;
//use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{integer};
use libsmt::logics::lia::LIA;

use expression::Predicate;

pub mod smt_output;

pub fn gen_smtlib (wp: Predicate) {
//    let mut z3: z3::Z3 = Default::default();
//
//    let mut solver = SMTLib2::new(Some(QF_LIA));
//    solver.set_logic(&mut z3);

    // DEBUG
    println!("Weakest Precondition is: ``{}''", wp);

//    if let Ok(result) = solver.solve(&mut z3) {
//        println!("Satisfiable");
//    } else {
//        println!("Unsatisfiable");
//    }
}



// ( ( ((x+3) <= (2*u))
//     OR ((v+4) >= y)
//     OR (((x + y) + z) >= 2)
//   ) AND (
//     7 == ( (
//          if ((x <= 2) AND (2 <= ((x + 3) + -1)))
//          then 3
//          else 0
//       ) + (
//          if ((u <=2) AND (2 <= ((u + 3) + -1)))
//          then 4
//          else 0
// ) ) ) )

//  fn gen_solver (wp: Predicate, solver: &mut SMTLib2) {
//      match wp {
//          &Predicate::VariableMapping (ref v) => {
//              // declare var
//          },
//          &Predicate::BooleanLiteral (ref b) => {
//              // declare bool
//          },
//          &Predicate::BinaryExpression (ref b) => {
//              match b.op {
//                  BooleanBinaryOperator::And => {
//                      write!(f, "({} && {})", *b.p1, *b.p2)
//                  },
//                  BooleanBinaryOperator::Or => {
//                      write!(f, "({} || {})", *b.p1, *b.p2)
//                  },
//                  BooleanBinaryOperator::Implies => {
//                      write!(f, "({} -> {})", *b.p1, *b.p2)
//                  }
//              }
//          },
//          &Predicate::UnaryExpression (ref u) => {
    //          match u.op {
        //          BooleanUnaryOperator::Not => {
            //          write!(f, "(!!{})", *u.p)
        //          }
    //          }
//          },
//             &Predicate::IntegerComparison (ref i) => {
//                 match i.op {
//                     IntegerComparisonOperator::LessThan => {
//                         write!(f, "({} < {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::LessThanOrEqual => {
//                         write!(f, "({} <= {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::GreaterThan => {
//                         write!(f, "({} > {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::GreaterThanOrEqual => {
//                         write!(f, "({} >= {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::Equal => {
//                         write!(f, "({} == {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::NotEqual => {
//                         write!(f, "({} != {})", *i.t1, *i.t2)
//                     }
//                 }
//             }
//         }
//     }
// }
        // The following needs to be done recursively...
        // For all terms: Define the symbolic variables in predicate
        //                  let var = 
        //                Define the integer constants used
        // Define the assert conditions


// // Used for representing Predicate types as strings, recursively.
// impl fmt::Display for Predicate {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // FIXME: this commented line below is the format to use
//         // write!(f, "predicate")
//         match self {
//             &Predicate::VariableMapping (ref v) => {
//                 write!(f, "({} : {})", v.name, v.var_type)
//             }
//             &Predicate::BooleanLiteral (ref b) => {
//                 write!(f, "({})", b)
//             },
//             &Predicate::BinaryExpression (ref b) => {
//                 match b.op {
//                     BooleanBinaryOperator::And => {
//                         write!(f, "({} && {})", *b.p1, *b.p2)
//                     },
//                     BooleanBinaryOperator::Or => {
//                         write!(f, "({} || {})", *b.p1, *b.p2)
//                     },
//                     BooleanBinaryOperator::Implies => {
//                         write!(f, "({} -> {})", *b.p1, *b.p2)
//                     }
//                 }
// 
//             },
//             &Predicate::UnaryExpression (ref u) => {
//                 match u.op {
//                     BooleanUnaryOperator::Not => {
//                         write!(f, "(!!{})", *u.p)
//                     }
//                 }
//             }
//             &Predicate::IntegerComparison (ref i) => {
//                 match i.op {
//                     IntegerComparisonOperator::LessThan => {
//                         write!(f, "({} < {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::LessThanOrEqual => {
//                         write!(f, "({} <= {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::GreaterThan => {
//                         write!(f, "({} > {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::GreaterThanOrEqual => {
//                         write!(f, "({} >= {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::Equal => {
//                         write!(f, "({} == {})", *i.t1, *i.t2)
//                     },
//                     IntegerComparisonOperator::NotEqual => {
//                         write!(f, "({} != {})", *i.t1, *i.t2)
//                     }
//                 }
//             }
//         }
//     }
// }
