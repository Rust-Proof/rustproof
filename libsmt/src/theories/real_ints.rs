//! Defines basic operations defined under Real_Ints theory in SMTLIB2.

use std::fmt;

#[macro_use]
use theories::utils;
use backends::backend::SMTNode;

#[derive(Clone, Debug)]
pub enum OpCodes {
    Neg,
    Sub,
    Add,
    Mul,
    Div,
    Lte,
    Lt,
    Gte,
    Gt,
    ToReal,
    ToInt,
    IsInt,
    ConstInt(u64),
    ConstReal(f64),
    FreeVar(String),
}

impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCodes::Neg => "-".to_owned(),
            OpCodes::Sub => "-".to_owned(),
            OpCodes::Add => "+".to_owned(),
            OpCodes::Mul => "*".to_owned(),
            OpCodes::Div => "/".to_owned(),
            OpCodes::Lte => "<=".to_owned(),
            OpCodes::Lt => "<".to_owned(),
            OpCodes::Gte => ">=".to_owned(),
            OpCodes::Gt => ">".to_owned(),
            OpCodes::ToReal => "to_real".to_owned(),
            OpCodes::ToInt => "to_int".to_owned(),
            OpCodes::IsInt => "is_int".to_owned(),
            OpCodes::ConstInt(ref val) => format!("{}", val),
            OpCodes::ConstReal(ref val) => format!("{}", val),
            OpCodes::FreeVar(ref name) => format!("{}", name),
        };
        write!(f, "{}", s)
    }
}

impl_smt_node!(OpCodes, define vars [OpCodes::FreeVar(_)], define consts [OpCodes::ConstInt(_), OpCodes::ConstReal(_)]);

#[derive(Clone,Debug)]
pub enum Sorts {
    Real,
    Int
}

impl fmt::Display for Sorts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Sorts::Real => "Real",
            Sorts::Int => "Int"
        };
        write!(f, "{}", "s")
    }
}
