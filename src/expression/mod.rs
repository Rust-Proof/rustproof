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
pub struct IntegerComparisonData { pub op: IntegerComparisonOperator, pub t1: Box<Term>, pub t2: Box<Term> }

// Boolean Expression type
pub enum Predicate {
    // Boolean literals
    BooleanLiteral(bool),
    // Boolean operations
    And(AndData),
    Or(OrData),
    Not(NotData),
    Implies(ImpliesData),
    // Integer comparison, which yields boolean
    IntegerComparison(IntegerComparisonData),
}

impl fmt::Debug for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct VariableMappingData { pub name: String, pub var_type: String}
pub struct BinaryExpressionData { pub op: IntegerBinaryOperator, pub t1: Box<Term>, pub t2: Box<Term> }
pub struct UnaryExpressionData { pub op: IntegerUnaryOperator, pub t: Box<Term> }
pub struct UnsignedBitVectorData { pub size: u8, pub value: u64 }
pub struct SignedBitVectorData { pub size: u8, pub value: i64 }

// A literal, variable, or expression involving either.
pub enum Term {
    VariableMapping(VariableMappingData),
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

#[derive(Clone)]
pub enum IntegerBinaryOperator {
    // Normal operators
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    // Bitwise operators
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    // Array operators
    ArrayLookup,
    ArrayUpdate
}

#[derive(Clone)]
pub enum IntegerUnaryOperator {
    Negation,
    BitwiseNot
}

#[derive(Clone)]
pub enum IntegerComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual
}

// Recurses through a Predicate and replaces any Variable Mapping with the given Term.
pub fn substitute_variable_in_predicate_with_term ( source_predicate: Predicate, target: VariableMappingData, replacement_term: Term ) -> Predicate {
    match source_predicate {
        Predicate::BooleanLiteral(b) => {
            // Return a copy.
            Predicate::BooleanLiteral ( b )
        },
        Predicate::And(a) => {
            // Recurisvely call the sub-predicates and return a new And.
            Predicate::And ( AndData {
                p1: Box::new(substitute_variable_in_predicate_with_term(
                    *a.p1,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                )),
                p2: Box::new(substitute_variable_in_predicate_with_term(
                    *a.p2,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Predicate::Or(o) => {
            // Recurisvely call the sub-predicates and return a new Or.
            Predicate::Or ( OrData {
                p1: Box::new(substitute_variable_in_predicate_with_term(
                    *o.p1,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                )),
                p2: Box::new(substitute_variable_in_predicate_with_term(
                    *o.p2,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Predicate::Not(n) => {
            // Recurisvely call the sub-predicate and return a new Not.
            Predicate::Not ( NotData {
                p: Box::new(substitute_variable_in_predicate_with_term(
                    *n.p,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Predicate::Implies(i) => {
            // Recurisvely call the sub-predicates and return a new Implies.
            Predicate::Implies ( ImpliesData {
                p1: Box::new(substitute_variable_in_predicate_with_term(
                    *i.p1,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                )),
                p2: Box::new(substitute_variable_in_predicate_with_term(
                    *i.p2,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Predicate::IntegerComparison(i) => {
            // Recurisvely call the sub-terms and return a new IntegerComparison.
            Predicate::IntegerComparison( IntegerComparisonData {
                op: i.op,
                t1: Box::new(substitute_variable_in_term_with_term(
                    *i.t1,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                )),
                t2: Box::new(substitute_variable_in_term_with_term(
                    *i.t2,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        }
    }
}

// Recurses through a Term and replaces any matching Variable Mapping with the given Term. Returns a copy of the source Term with replacements.
pub fn substitute_variable_in_term_with_term ( source_term: Term, target: VariableMappingData, replacement_term: Term ) -> Term {
    match source_term {
        Term::VariableMapping(v) => {
            // Replace the VariableMapping with replacement_term if it matches target, otherwise return a copy.
            if v == target {
                return_term_copy(&replacement_term)
            } else {
                Term::VariableMapping( VariableMappingData { name: v.name.clone(), var_type: v.var_type.clone() } )
            }
        },
        Term::BinaryExpression(b) => {
            // Recursively call the sub-terms and return new BinaryExpression.
            Term::BinaryExpression( BinaryExpressionData {
                op: b.op,
                t1: Box::new(substitute_variable_in_term_with_term(
                    *b.t1,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                )),
                t2: Box::new(substitute_variable_in_term_with_term(
                    *b.t2,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Term::UnaryExpression(u) => {
            // Recusrively call the sub-term and return new UnaryExpression.
            Term::UnaryExpression( UnaryExpressionData {
                op: u.op,
                t: Box::new(substitute_variable_in_term_with_term(
                    *u.t,
                    VariableMappingData { name: target.name.clone(), var_type: target.var_type.clone() },
                    return_term_copy(&replacement_term)
                ))
            } )
        },
        Term::UnsignedBitVector(u) => {
            // Return a copy.
            return_term_copy(&replacement_term)
        },
        Term::SignedBitVector(s) => {
            // Return a copy
            return_term_copy(&replacement_term)
        }
    }
}

// Returns a Term identical to the one that was submitted.
pub fn return_term_copy( original: &Term ) -> Term {
    match original {
        &Term::VariableMapping(ref v) => {
            Term::VariableMapping( VariableMappingData { name: v.name.clone(), var_type: v.var_type.clone() } )
        }
        &Term::BinaryExpression(ref b) => {
            Term::BinaryExpression(
                BinaryExpressionData {
                    op: b.op.clone(),
                    t1: Box::new(return_term_copy(&(*b.t1))),
                    t2: Box::new(return_term_copy(&(*b.t2)))
                }
            )
        },
        &Term::UnaryExpression(ref u) => {
            Term::UnaryExpression(
                UnaryExpressionData {
                    op: u.op.clone(),
                    t: Box::new(return_term_copy(&(*u.t)))
                }
            )
        },
        &Term::UnsignedBitVector(ref u) => {
            Term::UnsignedBitVector(
                UnsignedBitVectorData {
                    size: u.size,
                    value: u.value
                }
            )
        },
        &Term::SignedBitVector(ref s) => {
            Term::SignedBitVector(
                SignedBitVectorData {
                    size: s.size,
                    value: s.value
                }
            )
        }
    }
}

// Used for representing Predicate types as strings, recursively.
impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: this commented line below is the format to use
        // write!(f, "predicate")
        match self {
            &Predicate::BooleanLiteral (ref b) => {
                write!(f, "({})", b)
            },
            &Predicate::And (ref a) => {
                write!(f, "({} && {})", *a.p1, *a.p2)
            },
            &Predicate::Or (ref o) => {
                write!(f, "({} || {})", *o.p1, *o.p2)
            },
            &Predicate::Not (ref n) => {
                write!(f, "(!! {})", *n.p)
            },
            &Predicate::Implies (ref i) => {
                write!(f, "({} -> {})", *i.p1, *i.p2)
            },
            &Predicate::IntegerComparison (ref i) => {
                match i.op {
                    IntegerComparisonOperator::LessThan => {
                        write!(f, "({} < {})", *i.t1, *i.t2)
                    },
                    IntegerComparisonOperator::LessThanOrEqual => {
                        write!(f, "({} <= {})", *i.t1, *i.t2)
                    },
                    IntegerComparisonOperator::GreaterThan => {
                        write!(f, "({} > {})", *i.t1, *i.t2)
                    },
                    IntegerComparisonOperator::GreaterThanOrEqual => {
                        write!(f, "({} >= {})", *i.t1, *i.t2)
                    },
                    IntegerComparisonOperator::Equal => {
                        write!(f, "({} == {})", *i.t1, *i.t2)
                    },
                    IntegerComparisonOperator::NotEqual => {
                        write!(f, "({} != {})", *i.t1, *i.t2)
                    }
                }
            }
        }
    }
}

// Used for representing Term types as strings, recursively. Expressions are surrounded by parentheses to help visualize branching structure.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: this commented line below is the format to use
        // write!(f, "predicate")
        match self {
            &Term::VariableMapping(ref v) => {
                write!(f, "({})", v.name)
            },
            &Term::BinaryExpression(ref b) => {
                match b.op {
                    IntegerBinaryOperator::Addition => {
                        write!(f, "({} + {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::Subtraction => {
                        write!(f, "({} - {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::Multiplication => {
                        write!(f, "({} * {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::Division => {
                        write!(f, "({} / {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::Modulo => {
                        write!(f, "({} % {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::BitwiseOr => {
                        write!(f, "({} | {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::BitwiseAnd => {
                        write!(f, "({} & {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::BitwiseXor => {
                        write!(f, "({} ^ {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::BitwiseLeftShift => {
                        write!(f, "({} << {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::BitwiseRightShift => {
                        write!(f, "({} >> {})", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::ArrayLookup => {
                        write!(f, "({} [{}])", *b.t1, *b.t2)
                    },
                    IntegerBinaryOperator::ArrayUpdate => {
                        write!(f, "({} [{}])", *b.t1, *b.t2)
                    }
                }
            },
            &Term::UnaryExpression(ref u) => {
                match u.op {
                    IntegerUnaryOperator::Negation => {
                        write!(f, "(- {})", *u.t)
                    },
                    IntegerUnaryOperator::BitwiseNot => {
                        write!(f, "(! {})", *u.t)
                    }
                }
            },
            &Term::UnsignedBitVector(ref u) => {
                write!(f, "({})", u.value)
            },
            &Term::SignedBitVector(ref s) => {
                write!(f, "({})", s.value)
            }
        }
    }
}


// Check equality for VariableMappingData types. Should return true if the name and type of the variables are the same.
impl PartialEq for VariableMappingData {
    fn eq(&self, _rhs: &VariableMappingData) -> bool {
        if (self.name == _rhs.name) && (self.var_type == _rhs.var_type) {
            true
        } else {
            false
        }
    }
}

// Ensures it is clear that VariableMappingData has full equality.
impl Eq for VariableMappingData {}