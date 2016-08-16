// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::Debug;
use std::process;

use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{bitvec, core};
use libsmt::logics::qf_abv::*;
use petgraph::graph::NodeIndex;

use errors::{ColorConfig, Handler};
use syntax::codemap::CodeMap;
use std::rc::Rc;

use expression::*;

/// Invokes Z3 to check the satisfiability of a verification condition.
///
/// # Arguments:
/// * `vc` - A verification condition as an Expression.
/// * `name` - The name of the function whose verification condition is being checked.
/// * `debug` - A flag to enable/disable debug printing.
///
/// # Remarks:
/// * Simply satisfying P->WP isn't enough; that will only tell us if P->WP is _sometimes true_. We
/// * need to verify that !(P->WP) is *unsatisfiable* to determine that P->WP is _always true_.
///
pub fn gen_smtlib (vc: &Expression, name: String, debug: bool) {
    // Define an instance of Z3
    let mut z3: z3::Z3 = Default::default();

    // Declare a logic to use
    let mut solver = SMTLib2::new(Some(QF_ABV));

    // Check the satisfiability of the solver
    let vcon = solver.expr2smtlib(vc);
    let _ = solver.assert(core::OpCodes::Not, &[vcon]);

    let (res, check) = solver.solve(&mut z3, debug);
    match res {
        Ok(..) => {
            match check {
                SMTRes::Sat(_, ref model) => {
                    println!(
                        "\nfn {}(..)\tVerification Condition is not valid.\n\n{}\n",
                        name,
                        model.clone().unwrap()
                    );
                },
                SMTRes::Unsat(..) => {
                    println!("\nfn {}(..)\tVerification Condition is valid.\n", name);
                },
                _ => { unimplemented!() }
            }
        },
        Err(..) => {
            println!("\nfn {}(..)g\tError in Verification Condition Generation.\n", name);
        },
    }
}

pub trait Pred2SMT {
    type Idx: Debug + Clone;
    type Logic: Logic;

    fn expr2smtlib (&mut self, &Expression) -> Self::Idx;
}

impl Pred2SMT for SMTLib2<QF_ABV> {
    type Idx = NodeIndex;
    type Logic = QF_ABV;

    fn expr2smtlib (&mut self, vc: &Expression) -> Self::Idx {
        match *vc {
            Expression::BinaryExpression (ref b) => {
                let l = self.expr2smtlib(b.left.as_ref());
                let r = self.expr2smtlib(b.right.as_ref());
                match b.op {
                    BinaryOperator::Addition => {
                        return self.assert(bitvec::OpCodes::BvAdd, &[l,r]);
                    },
                    BinaryOperator::Subtraction => {
                        return self.assert(bitvec::OpCodes::BvSub, &[l,r]);
                    },
                    BinaryOperator::Multiplication => {
                        return self.assert(bitvec::OpCodes::BvMul, &[l,r]);
                    },
                    BinaryOperator::Division => {
                        // Check for signedness
                        if determine_evaluation_type(vc).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSDiv, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUDiv, &[l,r]);
                        }
                    },
                    BinaryOperator::Modulo => {
                        // Check for signedness
                        if determine_evaluation_type(vc).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSMod, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvURem, &[l,r]);
                        }
                    },
                    BinaryOperator::SignedMultiplicationDoesNotOverflow => {
                        return self.assert(bitvec::OpCodes::BvSMulDoesNotOverflow, &[l,r]);
                    },
                    BinaryOperator::SignedMultiplicationDoesNotUnderflow => {
                        return self.assert(bitvec::OpCodes::BvSMulDoesNotUnderflow, &[l,r]);
                    },
                    BinaryOperator::UnsignedMultiplicationDoesNotOverflow => {
                        return self.assert(bitvec::OpCodes::BvUMulDoesNotOverflow, &[l,r]);
                    },
                    BinaryOperator::BitwiseOr => {
                        return self.assert(bitvec::OpCodes::BvOr, &[l,r]);
                    },
                    BinaryOperator::BitwiseAnd => {
                        return self.assert(bitvec::OpCodes::BvAnd, &[l,r]);
                    },
                    BinaryOperator::BitwiseXor => {
                        return self.assert(bitvec::OpCodes::BvXor, &[l,r]);
                    },
                    BinaryOperator::BitwiseLeftShift => {
                        return self.assert(bitvec::OpCodes::BvShl, &[l,r]);
                    },
                    BinaryOperator::BitwiseRightShift => {
                        // Check for signedness
                        if determine_evaluation_type(vc).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvAShr, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvLShr, &[l,r]);
                        }
                    },
                    BinaryOperator::LessThan => {
                        let l = self.expr2smtlib(b.left.as_ref());
                        let r = self.expr2smtlib(b.right.as_ref());
                        if determine_evaluation_type(b.left.as_ref()).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSLt, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvULt, &[l,r]);
                        }
                    },
                    BinaryOperator::LessThanOrEqual => {
                        // Check for signedness
                        if determine_evaluation_type(b.left.as_ref()).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSLe, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvULe, &[l,r]);
                        }
                    },
                    BinaryOperator::GreaterThan => {
                        // Check for signedness
                        if determine_evaluation_type(b.left.as_ref()).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSGt, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUGt, &[l,r]);
                        }
                    },
                    BinaryOperator::GreaterThanOrEqual => {
                        // Check for signedness
                        if determine_evaluation_type(b.left.as_ref()).starts_with('i') {
                            return self.assert(bitvec::OpCodes::BvSGe, &[l,r]);
                        } else {
                            return self.assert(bitvec::OpCodes::BvUGe, &[l,r]);
                        }
                    },
                    BinaryOperator::Equal
                    | BinaryOperator::BiImplication => {
                        return self.assert(core::OpCodes::Cmp, &[l,r]);
                    }
                    BinaryOperator::NotEqual => {
                        let eq = self.assert(core::OpCodes::Cmp, &[l,r]);
                        return self.assert(core::OpCodes::Not, &[eq]);
                    },
                    BinaryOperator::And => {
                        return self.assert(core::OpCodes::And, &[l,r]);
                    },
                    BinaryOperator::Or => {
                        return self.assert(core::OpCodes::Or, &[l,r]);
                    },
                    BinaryOperator::Xor => {
                        return self.assert(core::OpCodes::Xor, &[l,r]);
                    },
                    BinaryOperator::Implication => {
                        return self.assert(core::OpCodes::Imply, &[l,r]);
                    },
                }
            },
            Expression::UnaryExpression (ref u) => {
                let n = self.expr2smtlib(u.e.as_ref());
                match u.op {
                    UnaryOperator::Negation => {
                        return self.assert(bitvec::OpCodes::BvNeg, &[n]);
                    },
                    UnaryOperator::BitwiseNot => {
                        return self.assert(bitvec::OpCodes::BvNot, &[n]);
                    },
                    UnaryOperator::Not => {
                        return self.assert(core::OpCodes::Not, &[n]);
                    },
                }
            },
            Expression::VariableMapping (ref v) => {
                let sort = match v.var_type.as_ref() {
                    "bool" => bitvec::Sorts::Bool,
                    "i8" | "u8" => bitvec::Sorts::BitVector(8),
                    "i16" | "u16" => bitvec::Sorts::BitVector(16),
                    "i32" | "u32" => bitvec::Sorts::BitVector(32),
                    "i64" | "u64" => bitvec::Sorts::BitVector(64),
                    _ => {
                        rp_error!(
                            "Invalid or Unsupported type for variable: \"{}\" : \"{}\"",
                            v.name,
                            v.var_type
                        );
                    },
                };
                return self.new_var(Some(&v.name), sort);
            },
            Expression::BooleanLiteral (ref b) => {
                return self.new_const(core::OpCodes::Const(*b));
            },
            Expression::UnsignedBitVector (ref u) => {
                return bv_const!(self, u.value, u.size as usize);
            },
            Expression::SignedBitVector (ref s) => {
                return bv_const!(self, s.value as u64, s.size as usize);
            }
        }
    }
}
