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

pub struct AndData { p1: Box<Predicate>, p2: Box<Predicate> }
pub struct OrData { p1: Box<Predicate>, p2: Box<Predicate> }
pub struct NotData { p: Box<Predicate> }
pub struct ImpliesData { p1: Box<Predicate>, p2: Box<Predicate> }
pub struct IntegerComparisonData { op: IntegerComparisonOperand, t1: Term, t2: Term }

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

pub struct VariableMappingData { name: String, var_type: String}
pub struct BinaryExpressionData { op: IntegerBinaryOperand, t1: Box<Term>, t2: Box<Term> }
pub struct UnaryExpressionData { op: IntegerUnaryOperand, t: Box<Term> }
pub struct UnsignedBitVectorData { size: u8, value: u64 }
pub struct SignedBitVectorData { size: u8, value: i64 }

//A literal, variable, or expression involving either
pub enum Term {
    VariableMapping(VariableMappingData),
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

pub enum IntegerBinaryOperand {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    ArrayLookup,
    ArrayUpdate
}

pub enum IntegerUnaryOperand {
    Negation
}

pub enum IntegerComparisonOperand {
    LessThan,
    LessThanOrEqual,
    GreatherThan,
    GreatherThanOrEqual,
    Equal,
    NotEqual
}

//Recurses through a Predicate and replaces any Variable Mapping with the given Term
pub fn substitute_variable_in_predicate_with_term ( p: Predicate, x: VariableMappingData, e: Term ) -> Predicate {

    // FIXME: shouldn't always return "true"
    Predicate::BooleanLiteral(true)
}

//Recurses through a Term and replaces any Variable Mapping with the given Term
pub fn substitute_varible_in_term_with_term ( t1: Term, x: VariableMappingData, t2: Term ) -> Term {

    // FIXME: shouldn't always return a variable mapping for an i64 whose value is 1
    Term::SignedBitVector( SignedBitVectorData {size: 64u8, value: 1i64})
}

// FIXME: Needs to properly represent the contents as a string, recursively, for all cases
impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "predicate")
    }
}