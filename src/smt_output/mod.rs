// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

use expression::*;

pub fn gen_smtlib (vc: &Predicate) {
    // Define an instance of Z3
    let mut z3: z3::Z3 = Default::default();

    // Declare a logic to use
    let mut solver = SMTLib2::new(Some(QF_ABV));

    // Apply logic to Z3 instance
    solver.set_logic(&mut z3);

    // DEBUG
    println!("Verification Condition is: ``{}''", vc);

    // Traverse the Predicate graph and build the solver
    let _ = solver.pred2smtlib(&vc);

    // Check the satisfiability of the solver
    if let Ok(result) = solver.solve(&mut z3) {
        println!("Satisfiable");
    } else {
        println!("Unsatisfiable");
    }
}

pub trait Pred2SMT {
    type Idx: Debug + Clone;
    type Logic: Logic;

    //fn gen_smtlib(&Predicate);
    fn pred2smtlib (&mut self, &Predicate) -> Self::Idx;
    //fn pred2smtlib (&mut self, &Predicate) -> NodeIndex;
    fn term2smtlib (&mut self, &Term) -> Self::Idx;
    //fn term2smtlib (&mut self, &Term) -> NodeIndex;
}

// bajr is keeping this here for posterity... and misplaced pride
//  impl<L: Logic> Pred2SMT for SMTLib2<L>
//      where <L as Logic>::Sorts: From<array_ex::Sorts<QF_ABV_Sorts,QF_ABV_Sorts>> + From<bitvec::Sorts> + From<core::Sorts>,
//            <L as Logic>::Fns: From<array_ex::OpCodes<QF_ABV_Sorts,QF_ABV_Sorts,QF_ABV_Fn>> + From<bitvec::OpCodes> + From<core::OpCodes>
impl Pred2SMT for SMTLib2<QF_ABV>
{
    type Idx = NodeIndex;
    type Logic = QF_ABV;


    fn pred2smtlib (&mut self, vc: &Predicate) -> Self::Idx {
        match vc {
            &Predicate::VariableMapping (ref v) => {
                match v.var_type.as_ref() {
                    "bool" => return self.new_var(Some(&v.name), core::Sorts::Bool),
                    _ => return self.new_var(Some(&v.name), core::Sorts::Bool),
                }
            },
            &Predicate::BooleanLiteral (ref b) => {
                if *b == true {
                    return self.new_const(core::OpCodes::True);
                } else {
                    return self.new_const(core::OpCodes::False);
                }
            },
            &Predicate::BinaryExpression (ref b) => {
                match b.op {
                    BooleanBinaryOperator::And => {
                        let l = self.pred2smtlib(b.p1.as_ref());
                        let r = self.pred2smtlib(b.p2.as_ref());
                        return self.assert(core::OpCodes::And, &[l,r]);
                    },
                    BooleanBinaryOperator::Or => {
                        let l = self.pred2smtlib(b.p1.as_ref());
                        let r = self.pred2smtlib(b.p2.as_ref());
                        return self.assert(core::OpCodes::Or, &[l,r]);
                    },
                    BooleanBinaryOperator::Implies => {
                        let l = self.pred2smtlib(b.p1.as_ref());
                        let r = self.pred2smtlib(b.p2.as_ref());
                        return self.assert(core::OpCodes::Imply, &[l,r]);
                    },
                }
            },
            &Predicate::UnaryExpression (ref u) => {
                match u.op {
                    BooleanUnaryOperator::Not => {
                        let n = self.pred2smtlib(u.p.as_ref());
                        return self.assert(core::OpCodes::Not, &[n]);
                    },
                }
            },
            &Predicate::IntegerComparison (ref i) => {
                match i.op {
                    IntegerComparisonOperator::LessThan => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSLt, &[l,r]);
                    },
                    IntegerComparisonOperator::LessThanOrEqual => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSLe, &[l,r]);
                    },
                    IntegerComparisonOperator::GreaterThan => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSGt, &[l,r]);
                    },
                    IntegerComparisonOperator::GreaterThanOrEqual => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSGe, &[l,r]);
                    },
                    IntegerComparisonOperator::Equal => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        return self.assert(core::OpCodes::Cmp, &[l,r]);
                    },
                    IntegerComparisonOperator::NotEqual => {
                        let l = self.term2smtlib(i.t1.as_ref());
                        let r = self.term2smtlib(i.t2.as_ref());
                        let eq = self.assert(core::OpCodes::Cmp, &[l,r]);
                        return self.assert(core::OpCodes::Not, &[eq]);
                    },
                }
            }
        }
    }
    
    fn term2smtlib (&mut self, term: &Term) -> Self::Idx {
        match term {
            &Term::VariableMapping (ref v) => {
                match v.var_type.as_ref() {
                    "int" => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                    "i32" => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                    "i64" => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                    "u32" => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                    "u64" => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                    _ => return self.new_var(Some(&v.name), bitvec::Sorts::BitVector(64)),
                }
            },
            &Term::BinaryExpression (ref b) => {
                match b.op {
                    IntegerBinaryOperator::Addition => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvAdd, &[l,r]);
                    },
                    IntegerBinaryOperator::Subtraction => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSub, &[l,r]);
                    },
                    IntegerBinaryOperator::Multiplication => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvMul, &[l,r]);
                    },
                    IntegerBinaryOperator::Division => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSDiv, &[l,r]);
                    },
                    IntegerBinaryOperator::Modulo => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvSMod, &[l,r]);
                    },
                    IntegerBinaryOperator::BitwiseOr => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvOr, &[l,r]);
                    },
                    IntegerBinaryOperator::BitwiseAnd => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvAnd, &[l,r]);
                    },
                    IntegerBinaryOperator::BitwiseXor => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvXor, &[l,r]);
                    },
                    IntegerBinaryOperator::BitwiseLeftShift => {
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvShl, &[l,r]);
                    },
                    IntegerBinaryOperator::BitwiseRightShift => { // AShr or LShr?
                        let l = self.term2smtlib(b.t1.as_ref());
                        let r = self.term2smtlib(b.t2.as_ref());
                        return self.assert(bitvec::OpCodes::BvLShr, &[l,r]);
                    },
                    IntegerBinaryOperator::ArrayLookup => {
                        // FIXME: This arm is unimplemented!
                        // FIXME: But it must exist and it must return an index
                        return self.new_const(core::OpCodes::True);
//                          return self.assert(array_ex::OpCodes::Select, &[.., ..]);
                    },
                    IntegerBinaryOperator::ArrayUpdate => {
                        // FIXME: This arm is unimplemented!
                        // FIXME: But it must exist and it must return an index
                        return self.new_const(core::OpCodes::True);
//                          return self.assert(array_ex::OpCodes::Store, &[.., ..]);
                    },
                }
            },
            &Term::UnaryExpression (ref u) => {
                match u.op {
                    IntegerUnaryOperator::Negation => {
                        let n = self.term2smtlib(u.t.as_ref());
                        return self.assert(bitvec::OpCodes::BvNeg, &[n]);
                    },
                    IntegerUnaryOperator::BitwiseNot => {
                        let n = self.term2smtlib(u.t.as_ref());
                        return self.assert(bitvec::OpCodes::BvNot, &[n]);
                    },
                }
            },
            &Term::UnsignedBitVector (ref u) => {
                return bv_const!(self, u.value, 64);
            },
            &Term::SignedBitVector (ref s) => {
                return bv_const!(self, s.value as u64, 64);
            }
        }
    }
}
