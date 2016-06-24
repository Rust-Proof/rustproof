// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, and Drew Gohman.
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

//Boolean Expression type
enum Predicate {
    //Boolean literals
    True,
    False,
    //Boolean operations
    And { p1: Predicate, p2: Predicate },
    Or { p1: Predicate, p2: Predicate },
    Not { p: Predicate },
    Implies { p1: Predicate, p2: Predicate }
    //Integer comparison, which yields boolean
    IntegerComparison { op: IntegerComparisonOperand, t1: Term, t2: Term },
}

//A literal, variable, or expression involving either
enum Term {
    VariableMapping { name: String, type: String}
    BinaryExpression { op: IntegerBinaryOperand, t1: Term, t2: Term },
    UnaryExpression { op: IntegerUnaryExpression, t: Term },
    UnsignedBitVector { size: u8, value: u64},
    SignedBitVector { size: u8, value: i64}
}

enum IntegerBinaryOperand {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    ArrayLookup,
    ArrayUpdate
}

enum IntegerUnaryOperand {
    Negation
}

enum IntegerComparisonOperand {
    LessThan,
    LessThanOrEqual,
    GreatherThan,
    GreatherThanOrEqual,
    Equal,
    NotEqual
}

//Master function for the parser module. Checks pre- and post-conditions and builds them into expressions.
pub fn parse() {

}

//Recurses through a Predicate and replaces any Variable Mapping with the given Term
pub fn substitute_variable_in_predicate_with_term ( p: Predicate, x: Term::VariableMapping, e: Term ) -> P {

}


pub fn substitute_varible_in_term_with_term ( t1: Term, x: Term::VariableMapping, t2: Term ) -> Term {
    
}