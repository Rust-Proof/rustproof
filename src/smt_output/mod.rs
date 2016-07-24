// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//  ( set-logic LIA )
//  ( declare-fun x () Int )
//  ( declare-fun y () Int )
//  ( declare-fun z () Int ) ; This is an example
//  ( declare-fun u () Int )
//  ( declare-fun v () Int )
//  ( assert ( and ( or ( <= (+ x 3) (* 2 y ) ) ( >= (+ x 4) z )) ) )
//  ( declare-fun u () Bool )
//  ( declare-fun v () Bool )
//  ( assert ( = u v ) )
//  ( assert ( not ( = x y ) ) )
//  ( check-sat )
//  ( get-model )

// ( ( ((x+3) <= (2*u)) OR ((v+4) >= y) OR (((x + y) + z) >= 2)
//   ) AND (
//     7 == ( (
//          if ((x <= 2) AND (2 <= ((x + 3) + -1))) then 3 else 0
//       ) + (
//          if ((u <=2) AND (2 <= ((u + 3) + -1))) then 4 else 0
// ) ) ) )

use std::convert::From;
use std::fmt;
use std::fmt::Debug;

use libsmt;
use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{bitvec, core};
use libsmt::logics::qf_abv::*;
use petgraph::graph::{Graph, NodeIndex};

use expression::*;

//  pub trait Pred2SMT {
//      type Idx: Debug + Clone;
//      type Logic: Logic;
//  
//      fn pred2smtlib (&mut self, &Predicate) -> Self::Idx;
//      fn term2smtlib (&mut self, &Term) -> Self::Idx;
//  }

pub fn gen_smtlib (vc: Predicate) {
    let mut z3: z3::Z3 = Default::default();

    let mut solver = SMTLib2::new(Some(QF_ABV));
    solver.set_logic(&mut z3);

    //_ = solver.pred2smtlib(&vc);

    // DEBUG
    println!("Verification Condition is: ``{}''", vc);

    let x = solver.new_var(Some("x"), core::Sorts::Bool);

    if let Ok(result) = solver.solve(&mut z3) {
        println!("Satisfiable");
    } else {
        println!("Unsatisfiable");
    }
}

// The following needs to be done recursively...
// For all terms: Define the symbolic variables in predicate
//                Define the integer constants used
// Define the assert conditions

fn pred2smtlib (solver: &SMTLib2<Logic>, vc: &Predicate) -> NodeIndex {
    match vc {
        &Predicate::VariableMapping (ref v) => {
            match v.var_type.as_ref() {
                "bool" => return solver.new_var(Some(&v.name), core::Sorts::Bool),
                _ => return solver.new_var(Some(&v.name), core::Sorts::Bool),
            }
        },
        &Predicate::BooleanLiteral (ref b) => {
            if b == true {
                return solver.new_const(core::OpCodes::True);
            } else {
                return solver.new_const(core::OpCodes::False);
            }
        },
        &Predicate::BinaryExpression (ref b) => {
            match b.op {
                BooleanBinaryOperator::And => {
                    return solver.assert(core::OpCodes::And,
                                         &[solver.pred2smtlib(*b.p1),
                                           solver.pred2smtlib(*b.p2)]);
                },
                BooleanBinaryOperator::Or => {
                    return solver.assert(core::OpCodes::Or,
                                         &[solver.pred2smtlib(*b.p1),
                                           solver.pred2smtlib(*b.p2)]);
                },
                BooleanBinaryOperator::Implies => {
                    return solver.assert(core::OpCodes::Imply,
                                         &[solver.pred2smtlib(*b.p1),
                                           solver.pred2smtlib(*b.p2)]);
                },
            }
        },
        &Predicate::UnaryExpression (ref u) => {
            match u.op {
                BooleanUnaryOperator::Not => {
                    return solver.assert(core::OpCodes::Not,
                                         &[solver.pred2smtlib(*u.p)]);
                },
            }
        },
        &Predicate::IntegerComparison (ref i) => {
            match i.op {
                IntegerComparisonOperator::LessThan => {
                    return solver.assert(bitvec::OpCodes::BvSLt,
                                         &[solver.term2smtlib(*i.t1),
                                           solver.term2smtlib(*i.t2)]);
                },
                IntegerComparisonOperator::LessThanOrEqual => {
                    return solver.assert(bitvec::OpCodes::BvSLe,
                                         &[solver.term2smtlib(*i.t1),
                                           solver.term2smtlib(*i.t2)]);
                },
                IntegerComparisonOperator::GreaterThan => {
                    return solver.assert(bitvec::OpCodes::BvSGt,
                                         &[solver.term2smtlib(*i.t1),
                                           solver.term2smtlib(*i.t2)]);
                },
                IntegerComparisonOperator::GreaterThanOrEqual => {
                    return solver.assert(bitvec::OpCodes::BvSGe,
                                         &[solver.term2smtlib(*i.t1),
                                           solver.term2smtlib(*i.t2)]);
                },
                IntegerComparisonOperator::Equal => {
                    return solver.assert(bitvec::OpCodes::BvComp,
                                         &[solver.term2smtlib(*i.t1),
                                           solver.term2smtlib(*i.t2)]);
                },
                IntegerComparisonOperator::NotEqual => {
                    return solver.assert(core::OpCodes::Not,
                                         &[solver.assert(bitvec::OpCodes::Cmp,
                                                         &[solver.term2smtlib(*i.t1),
                                                           solver.term2smtlib(*i.t2)])]);
                },
            }
        }
    }
}

fn term2smtlib<L: Logic> (solver: &SMTLib2<L>, term: &Term) -> NodeIndex {
    match term {
        &Term::VariableMapping (ref v) => {
            match v.var_type.as_ref() {
                "int" => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                "i32" => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                "i64" => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                "u32" => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                "u64" => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                _ => return solver.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
            }
        },
        &Term::BinaryExpression (ref b) => {
            match b.op {
                IntegerBinaryOperator::Addition => {
                    return solver.assert(bitvec::OpCodes::BvAdd,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::Subtraction => {
                    return solver.assert(bitvec::OpCodes::BvSub,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::Multiplication => {
                    return solver.assert(bitvec::OpCodes::BvMul,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::Division => {
                    return solver.assert(bitvec::OpCodes::BvSDiv,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::Modulo => {
                    return solver.assert(bitvec::OpCodes::BvSMod,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::BitwiseOr => {
                    return solver.assert(bitvec::OpCodes::BvOr,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::BitwiseAnd => {
                    return solver.assert(bitvec::OpCodes::BvAnd,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::BitwiseXor => {
                    return solver.assert(bitvec::OpCodes::BvXor,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::BitwiseLeftShift => {
                    return solver.assert(bitvec::OpCodes::BvShl,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::BitwiseRightShift => { // AShr or LShr?
                    return solver.assert(bitvec::OpCodes::BvLShr,
                                         &[solver.term2smtlib(*b.t1),
                                           solver.term2smtlib(*b.t2)]);
                },
                IntegerBinaryOperator::ArrayLookup => {
                    // NOT SURE HOW TO HANDLE THIS YET
                },
                IntegerBinaryOperator::ArrayUpdate => {
                    // NOT SURE HOW TO HANDLE THIS YET
                },
            }
        },
        &Term::UnaryExpression (ref u) => {
            match u.op {
                IntegerUnaryOperator::Negation => {
                    return solver.assert(bitvec::OpCodes::BvNeg,
                                         &[solver.term2smtlib(*u.t)]);
                },
                IntegerUnaryOperator::BitwiseNot => {
                    return solver.assert(bitvec::OpCodes::BvNot,
                                         &[solver.term2smtlib(*u.t)]);
                },
            }
        },
        &Term::UnsignedBitVector (ref u) => {
            return bv_const!(solver, u.value, 64);
        },
        &Term::SignedBitVector (ref s) => {
            return bv_const!(solver, s.value, 64);
        }
    }
}
