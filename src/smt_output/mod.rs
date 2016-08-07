// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate term;

use super::DEBUG;

use std::convert::From;
use std::fmt;
use std::fmt::Debug;

use libsmt;
use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{array_ex, bitvec, core};
use libsmt::logics::qf_abv::*;
use libsmt::logics::qf_abv;
use libsmt::logics::lia::*;
use libsmt::logics::lia;
use petgraph::graph::NodeIndex;
use std::process;

use expression::*;

// Now that we have a verification condition, we need to verify that it is always true.
// Simply satisfying P->WP isn't enough. We need to verify that !(P->WP) is *unsatisfiable*
pub fn gen_smtlib (vc: &Expression) {
    // Define an instance of Z3
    let mut z3: z3::Z3 = Default::default();

    // Declare a logic to use
    let mut solver = SMTLib2::new(Some(QF_ABV));

    // Apply logic to Z3 instance
    solver.set_logic(&mut z3);

    // Check the satisfiability of the solver
    let vcon = solver.expr2smtlib(&vc);
    let _ = solver.assert(core::OpCodes::Not, &[vcon]);

    let (res, check) = solver.solve(&mut z3);
    match res {
        Ok(..) => {
            match check {
                SMTRes::Sat(..) => { println!("\nVerification Condition is not valid.\n{:?}", check); },
                SMTRes::Unsat(..) => { println!("\nVerification Condition is valid.\n{:?}", check); },
                _ => { unimplemented!() }
            }
        },
        Err(..) => { println!("\nError in Verification Condition Generation.\n{:?}", check); }
    }

    /*
    // Check the satisfiability of the solver
    if let Ok(result) = solver.solve(&mut z3) {
        // If the assertion is satisfiable, then the VC is not valid (not always true)
        // FIXME This should probably warn
        println!("\nVerification Condition is not valid.");
    } else {
        // If the assertion is unsatisfiable, then the VC is valid (always true)
        // FIXME Do we want to output if things are good?
        println!("\nVerification Condition is valid!");
    }
    */
}

pub trait Pred2SMT {
    type Idx: Debug + Clone;
    type Logic: Logic;

    fn expr2smtlib (&mut self, &Expression) -> Self::Idx;
}

// bajr is keeping this here for posterity... and misplaced pride
//  impl<L: Logic> Pred2SMT for SMTLib2<L>
//      where <L as Logic>::Sorts: From<array_ex::Sorts<QF_ABV_Sorts,QF_ABV_Sorts>> + From<bitvec::Sorts> + From<core::Sorts>,
//            <L as Logic>::Fns: From<array_ex::OpCodes<QF_ABV_Sorts,QF_ABV_Sorts,QF_ABV_Fn>> + From<bitvec::OpCodes> + From<core::OpCodes>
impl Pred2SMT for SMTLib2<QF_ABV> {
    type Idx = NodeIndex;
    type Logic = QF_ABV;

    fn expr2smtlib (&mut self, vc: &Expression) -> Self::Idx {
        match vc {
            &Expression::BinaryExpression (ref b) => {
                match b.op {
                    BinaryOperator::Addition => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvAdd, &[l,r]);
                    },
                    BinaryOperator::Subtraction => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvSub, &[l,r]);
                    },
                    BinaryOperator::Multiplication => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvMul, &[l,r]);
                    },
                    BinaryOperator::Division => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(vc).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSDiv, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUDiv, &[l,r]);
                        }
                    },
                    BinaryOperator::Modulo => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(vc).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSMod, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvURem, &[l,r]);
                        }
                    },
                    BinaryOperator::BitwiseOr => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvOr, &[l,r]);
                    },
                    BinaryOperator::BitwiseAnd => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvAnd, &[l,r]);
                    },
                    BinaryOperator::BitwiseXor => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvXor, &[l,r]);
                    },
                    BinaryOperator::BitwiseLeftShift => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(bitvec::OpCodes::BvShl, &[l,r]);
                    },
                    BinaryOperator::BitwiseRightShift => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(vc).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvAShr, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvLShr, &[l,r]);
                        }
                    },
                    BinaryOperator::LessThan => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(b.left.as_ref()).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSLt, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvULt, &[l,r]);
                        }
                    },
                    BinaryOperator::LessThanOrEqual => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(b.left.as_ref()).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSLe, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvULe, &[l,r]);
                        }
                    },
                    BinaryOperator::GreaterThan => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(b.left.as_ref()).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSGt, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUGt, &[l,r]);
                        }
                    },
                    BinaryOperator::GreaterThanOrEqual => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(b.left.as_ref()).starts_with("i") {
                            return self.assert(bitvec::OpCodes::BvSGe, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUGe, &[l,r]);
                        }
                    },
                    BinaryOperator::Equal => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::Cmp, &[l,r]);
                    },
                    BinaryOperator::NotEqual => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        let eq = self.assert(core::OpCodes::Cmp, &[l,r]);
                        return self.assert(core::OpCodes::Not, &[eq]);
                    },
                    BinaryOperator::And => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::And, &[l,r]);
                    },
                    BinaryOperator::Or => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::Or, &[l,r]);
                    },
                    BinaryOperator::Xor => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::Xor, &[l,r]);
                    },
                    BinaryOperator::Implication => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::Imply, &[l,r]);
                    },

                    BinaryOperator::BiImplication => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        return self.assert(core::OpCodes::Cmp, &[l,r]);
                    }
                }
            },
            &Expression::UnaryExpression (ref u) => {
                match u.op {
                    UnaryOperator::Negation => {
                        let n = self.expr2smtlib(u.e.as_ref());
                        return self.assert(bitvec::OpCodes::BvNeg, &[n]);
                    },
                    UnaryOperator::BitwiseNot => {
                        let n = self.expr2smtlib(u.e.as_ref());
                        return self.assert(bitvec::OpCodes::BvNot, &[n]);
                    },
                    UnaryOperator::Not => {
                        let n = self.expr2smtlib(u.e.as_ref());
                        return self.assert(core::OpCodes::Not, &[n]);
                    },
                }
            },
            &Expression::VariableMapping (ref v) => {
                match v.var_type.as_ref() {
                    "bool" => { return self.new_var(Some(&v.name), core::Sorts::Bool); },
                    "i8" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(8)); },
                    "i16" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(16)); },
                    "i32" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(32)); },
                    "i64" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)); },
                    "u8" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(8)); },
                    "u16" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(16)); },
                    "u32" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(32)); },
                    "u64" => { return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)); },
                    _ => { rp_error!("Invalid or Unsupported type for variable: \"{}\" : \"{}\"", v.name, v.var_type); },
                }
            },
            &Expression::BooleanLiteral (ref b) => {
                return self.new_const(core::OpCodes::Const(*b));
            },
            &Expression::UnsignedBitVector (ref u) => {
                return bv_const!(self, u.value, u.size as usize);
            },
            &Expression::SignedBitVector (ref s) => {
                return bv_const!(self, s.value as u64, s.size as usize);
            }
        }
    }
}
