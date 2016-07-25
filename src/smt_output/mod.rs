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
use libsmt::theories::{array_ex, bitvec, core};
use libsmt::logics::qf_abv::*;
use libsmt::logics::qf_abv;
use libsmt::logics::lia::*;
use libsmt::logics::lia;
use petgraph::graph::NodeIndex;

use expression::*;

pub fn gen_smtlib (vc: Predicate) {
    let mut z3: z3::Z3 = Default::default();

    //let mut solver = SMTLib2::new(Some(QF_ABV));
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

pub trait Pred2SMT {
    type Idx: Debug + Clone;
    type Logic: Logic;

    fn pred2smtlib (&mut self, &Predicate) -> Self::Idx;
    //fn pred2smtlib (&mut self, &Predicate) -> NodeIndex;
    fn term2smtlib (&mut self, &Term) -> Self::Idx;
    //fn term2smtlib (&mut self, &Term) -> NodeIndex;
}

impl<L: Logic> Pred2SMT for SMTLib2<L>
    where <L as Logic>::Sorts: From<array_ex::Sorts<QF_ABV_Sorts,QF_ABV_Sorts>> + From<bitvec::Sorts> + From<core::Sorts>,
          <L as Logic>::Fns: From<array_ex::OpCodes<QF_ABV_Sorts,QF_ABV_Sorts,QF_ABV_Fn>> + From<bitvec::OpCodes> + From<core::OpCodes>
{
    type Idx = NodeIndex;
    type Logic = L;

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
                        return self.assert(core::OpCodes::And,
                                             &[self.pred2smtlib(b.p1.as_ref()),
                                               self.pred2smtlib(b.p2.as_ref())]);
                    },
                    BooleanBinaryOperator::Or => {
                        return self.assert(core::OpCodes::Or,
                                             &[self.pred2smtlib(b.p1.as_ref()),
                                               self.pred2smtlib(b.p2.as_ref())]);
                    },
                    BooleanBinaryOperator::Implies => {
                        return self.assert(core::OpCodes::Imply,
                                             &[self.pred2smtlib(b.p1.as_ref()),
                                               self.pred2smtlib(b.p2.as_ref())]);
                    },
                }
            },
            &Predicate::UnaryExpression (ref u) => {
                match u.op {
                    BooleanUnaryOperator::Not => {
                        return self.assert(core::OpCodes::Not,
                                             &[self.pred2smtlib(u.p.as_ref())]);
                    },
                }
            },
            &Predicate::IntegerComparison (ref i) => {
                match i.op {
                    IntegerComparisonOperator::LessThan => {
                        return self.assert(bitvec::OpCodes::BvSLt,
                                             &[self.term2smtlib(i.t1.as_ref()),
                                               self.term2smtlib(i.t2.as_ref())]);
                    },
                    IntegerComparisonOperator::LessThanOrEqual => {
                        return self.assert(bitvec::OpCodes::BvSLe,
                                             &[self.term2smtlib(i.t1.as_ref()),
                                               self.term2smtlib(i.t2.as_ref())]);
                    },
                    IntegerComparisonOperator::GreaterThan => {
                        return self.assert(bitvec::OpCodes::BvSGt,
                                             &[self.term2smtlib(i.t1.as_ref()),
                                               self.term2smtlib(i.t2.as_ref())]);
                    },
                    IntegerComparisonOperator::GreaterThanOrEqual => {
                        return self.assert(bitvec::OpCodes::BvSGe,
                                             &[self.term2smtlib(i.t1.as_ref()),
                                               self.term2smtlib(i.t2.as_ref())]);
                    },
                    IntegerComparisonOperator::Equal => {
                        return self.assert(bitvec::OpCodes::BvComp,
                                             &[self.term2smtlib(i.t1.as_ref()),
                                               self.term2smtlib(i.t2.as_ref())]);
                    },
                    IntegerComparisonOperator::NotEqual => {
                        return self.assert(core::OpCodes::Not,
                                             &[self.assert(bitvec::OpCodes::BvComp,
                                                             &[self.term2smtlib(i.t1.as_ref()),
                                                               self.term2smtlib(i.t2.as_ref())])]);
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
                        return self.assert(bitvec::OpCodes::BvAdd,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::Subtraction => {
                        return self.assert(bitvec::OpCodes::BvSub,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::Multiplication => {
                        return self.assert(bitvec::OpCodes::BvMul,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::Division => {
                        return self.assert(bitvec::OpCodes::BvSDiv,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::Modulo => {
                        return self.assert(bitvec::OpCodes::BvSMod,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::BitwiseOr => {
                        return self.assert(bitvec::OpCodes::BvOr,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::BitwiseAnd => {
                        return self.assert(bitvec::OpCodes::BvAnd,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::BitwiseXor => {
                        return self.assert(bitvec::OpCodes::BvXor,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::BitwiseLeftShift => {
                        return self.assert(bitvec::OpCodes::BvShl,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::BitwiseRightShift => { // AShr or LShr?
                        return self.assert(bitvec::OpCodes::BvLShr,
                                             &[self.term2smtlib(b.t1.as_ref()),
                                               self.term2smtlib(b.t2.as_ref())]);
                    },
                    IntegerBinaryOperator::ArrayLookup => {
                        // FIXME: This arm is unimplemented!
                        // FIXME: But it must exist and it must return an index
                        return self.new_const(core::OpCodes::True);
//                          return self.assert(array_ex::OpCodes::Select,
//                                             &[.., ..]);
                    },
                    IntegerBinaryOperator::ArrayUpdate => {
                        // FIXME: This arm is unimplemented!
                        // FIXME: But it must exist and it must return an index
                        return self.new_const(core::OpCodes::True);
//                          return self.assert(array_ex::OpCodes::Store,
//                                             &[.., ..]);
                    },
                }
            },
            &Term::UnaryExpression (ref u) => {
                match u.op {
                    IntegerUnaryOperator::Negation => {
                        return self.assert(bitvec::OpCodes::BvNeg,
                                             &[self.term2smtlib(u.t.as_ref())]);
                    },
                    IntegerUnaryOperator::BitwiseNot => {
                        return self.assert(bitvec::OpCodes::BvNot,
                                             &[self.term2smtlib(u.t.as_ref())]);
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
