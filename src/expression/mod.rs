// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//#[macro_use]
//extern crate rustc;
//extern crate syntax;
//extern crate rustc_plugin;

use rustc_plugin::Registry;
use std::fmt;

pub struct AndData { pub p1: Box<Predicate>, pub p2: Box<Predicate> }
pub struct OrData { pub p1: Box<Predicate>, pub p2: Box<Predicate> }
pub struct NotData { pub p: Box<Predicate> }
pub struct ImpliesData { pub p1: Box<Predicate>, pub p2: Box<Predicate> }
pub struct IntegerComparisonData { pub op: IntegerComparisonOperator, pub t1: Term, pub t2: Term }

//Boolean Expression type
pub enum Predicate {
    //Boolean literals
    BooleanLiteral(bool),
    //Boolean operations
    And(AndData),
    Or(OrData),
    Not(NotData),
    Implies(ImpliesData),
    //Integer comparison, which yields boolean
    IntegerComparison(IntegerComparisonData),
}

pub struct VariableMappingData { pub name: String, pub var_type: String}
pub struct BinaryExpressionData { pub op: IntegerBinaryOperator, pub t1: Box<Term>, pub t2: Box<Term> }
pub struct UnaryExpressionData { pub op: IntegerUnaryOperator, pub t: Box<Term> }
pub struct UnsignedBitVectorData { pub size: u8, pub value: u64 }
pub struct SignedBitVectorData { pub size: u8, pub value: i64 }

//A literal, variable, or expression involving either
pub enum Term {
    VariableMapping(VariableMappingData),
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

pub enum IntegerBinaryOperator {
    //Normal operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    //Bitwise operators
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    //Array operators
    ArrayLookup,
    ArrayUpdate
}

pub enum IntegerUnaryOperator {
    Negation,
    BitwiseNot
}

pub enum IntegerComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreatherThan,
    GreatherThanOrEqual,
    Equal,
    NotEqual
}

//Recurses through a Predicate and replaces any Variable Mapping with the given Term
pub fn substitute_variable_in_predicate_with_term ( p: Predicate, x: VariableMappingData, e: Term ) -> Predicate {
    match p {
        Predicate::And(a) => {
            unimplemented!()
        },
        Predicate::Or(o) => {
            unimplemented!()
        },
        Predicate::Not(n) => {
            unimplemented!()
        },
        Predicate::Implies(i) => {
            unimplemented!()
        },
        Predicate::IntegerComparison(ic) => {
            unimplemented!()
        },
        _ => {
            unimplemented!()
        }
    };
}

//Recurses through a Term and replaces any Variable Mapping with the given Term
pub fn substitute_varible_in_term_with_term ( t1: Term, x: VariableMappingData, t2: Term ) -> Term {

    unimplemented!();
}

//Used for representing Predicate types as strings, recursively.
impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "predicate")
    }
}

//Check equality for VariableMappingData types. Should return true if the name and type of the variables are the same.
impl PartialEq for VariableMappingData {
    fn eq(&self, _rhs: &VariableMappingData) -> bool {
        if (self.name == _rhs.name) && (self.var_type == _rhs.var_type) {
            true
        } else {
            false
        }
    }
}

//Ensures it is clear that VariableMappingData has full equality
impl Eq for VariableMappingData {}