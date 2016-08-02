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
extern crate term;
use rustc_plugin::Registry;
use std::fmt;
use std::process;

#[derive(Clone, PartialEq)]
pub struct BinaryExpressionData { pub op: BinaryOperator, pub left: Box<Expression>, pub right: Box<Expression> }

#[derive(Clone, PartialEq)]
pub struct UnaryExpressionData { pub op: UnaryOperator, pub e: Box<Expression> }

// Boolean Expression type
#[derive(Clone, PartialEq)]
pub enum Expression {
    // Boolean expressions
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    // A variable; should be either one of a function's formal arguments, a special "return" variable, or something from an encapsulating scope.
    VariableMapping(VariableMappingData),
    // Boolean literals
    BooleanLiteral(bool)
    // Integer literals
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

// Used for representing Expression types as strings, recursively.
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::BinaryExpression (ref b) => {
                match b.op {
                    BinaryOperator::And => {
                        write!(f, "({} && {})", *b.left, *b.right)
                    },
                    BinaryOperator::Or => {
                        write!(f, "({} || {})", *b.left, *b.right)
                    },
                    BinaryOperator::Xor => {
                        write!(f, "({} XOR {})", *b.left, *b.right)
                    },
                    BinaryOperator::Implication => {
                        write!(f, "({} => {})", *b.left, *b.right)
                    },
                    BinaryOperator::BiImplication => {
                        write!(f, "({} <=> {})", *b.left, *b.right)
                    },
                    BinaryOperator::Addition => {
                        write!(f, "({} + {})", *b.left, *b.right)
                    },
                    BinaryOperator::Subtraction => {
                        write!(f, "({} - {})", *b.left, *b.right)
                    },
                    BinaryOperator::Multiplication => {
                        write!(f, "({} * {})", *b.left, *b.right)
                    },
                    BinaryOperator::Division => {
                        write!(f, "({} / {})", *b.left, *b.right)
                    },
                    BinaryOperator::Modulo => {
                        write!(f, "({} % {})", *b.left, *b.right)
                    },
                    BinaryOperator::BitwiseOr => {
                        write!(f, "({} | {})", *b.left, *b.right)
                    },
                    BinaryOperator::BitwiseAnd => {
                        write!(f, "({} & {})", *b.left, *b.right)
                    },
                    BinaryOperator::BitwiseXor => {
                        write!(f, "({} ^ {})", *b.left, *b.right)
                    },
                    BinaryOperator::BitwiseLeftShift => {
                        write!(f, "({} << {})", *b.left, *b.right)
                    },
                    BinaryOperator::BitwiseRightShift => {
                        write!(f, "({} >> {})", *b.left, *b.right)
                    },
                    IntegerComparisonOperator::LessThan => {
                        write!(f, "({} < {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::LessThanOrEqual => {
                        write!(f, "({} <= {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::GreaterThan => {
                        write!(f, "({} > {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::GreaterThanOrEqual => {
                        write!(f, "({} >= {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::Equal => {
                        write!(f, "({} == {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::NotEqual => {
                        write!(f, "({} != {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::LessThan => {
                        write!(f, "({} < {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::LessThanOrEqual => {
                        write!(f, "({} <= {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::GreaterThan => {
                        write!(f, "({} > {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::GreaterThanOrEqual => {
                        write!(f, "({} >= {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::Equal => {
                        write!(f, "({} == {})", *i.left, *i.right)
                    },
                    IntegerComparisonOperator::NotEqual => {
                        write!(f, "({} != {})", *i.left, *i.right)
                    }
                }

            },
            &Expression::UnaryExpression (ref u) => {
                match u.op {
                    UnaryOperator::Not => {
                        write!(f, "(!!{})", *u.e)
                    },
                    UnaryOperator::Negation => {
                        write!(f, "(- {})", *u.e)
                    },
                    UnaryOperator::BitwiseNot => {
                        write!(f, "(! {})", *u.e)
                    }
                }
            },
            &Expression::VariableMapping (ref v) => {
                write!(f, "({} : {})", v.name, v.var_type)
            }
            &Expression::BooleanLiteral (ref b) => {
                write!(f, "({})", b)
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

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug)]
pub struct VariableMappingData { pub name: String, pub var_type: String}

// FIXME: Type checking removed, should be returned later!
// Check equality for VariableMappingData types. Should return true if the name and type of the variables are the same.
impl PartialEq for VariableMappingData {
    fn eq(&self, _rhs: &VariableMappingData) -> bool {
        if self.name == _rhs.name  { //&& (self.var_type == _rhs.var_type)
            true
        } else {
            false
        }
    }
}

// Ensures it is clear that VariableMappingData has full equality.
impl Eq for VariableMappingData {}

impl fmt::Display for VariableMappingData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} : {})", self.name, self.var_type)
    }
}

#[derive(Clone, PartialEq)]
pub struct BinaryExpressionData { pub op: BinaryOperator, pub left: Box<Term>, pub right: Box<Term> }

#[derive(Clone, PartialEq)]
pub struct UnaryExpressionData { pub op: UnaryOperator, pub t: Box<Term> }

#[derive(Clone, PartialEq)]
pub struct UnsignedBitVectorData { pub size: u8, pub value: u64 }

#[derive(Clone, PartialEq)]
pub struct SignedBitVectorData { pub size: u8, pub value: i64 }

// A literal, variable, or expression involving either.
#[derive(Clone, PartialEq)]
pub enum Term {
    // Integer expressions
    BinaryExpression(BinaryExpressionData),
    UnaryExpression(UnaryExpressionData),
    // An integer variable; should be either one of a function's formal arguments, a special "return" variable, or something from an encapsulating scope.
    VariableMapping(VariableMappingData),
    // Integer literals
    UnsignedBitVector(UnsignedBitVectorData),
    SignedBitVector(SignedBitVectorData)
}

#[derive(Clone, PartialEq)]
pub enum BinaryOperator {
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
    // Comparison operators
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    // Boolean logical operators
    And,
    Or,
    Xor,
    Implication,
    BiImplication
}

#[derive(Clone, PartialEq)]
pub enum UnaryOperator {
    Negation,
    BitwiseNot,
    // Boolean logical operators
    Not,
}

// Recurses through a Expression and replaces any Variable Mapping with the given Term.
pub fn substitute_variable_with_expression ( mut source_expression: Expression, target: VariableMappingData, replacement: Term ) {
    match source_expression {
        Expression::BinaryExpression(b) => {
            // Recurisvely call the sub-expressions
            substitute_variable_with_expression(b.left, target, replacement);
            substitute_variable_with_expression(b.right, target, replacement);
        },
        Expression::UnaryExpression(u) => {
            // Recurisvely call the sub-expression
            substitute_variable_with_expression(b.e, target, replacement);
        },
        Expression::VariableMapping(v) => {
            // Substitute the variable if it matches the target
            if v == target {
                source_expression = replacement;
            }
        },
        _ => {
            // No substitution should be done
        }
    }
}