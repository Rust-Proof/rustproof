//! Defines basic operations defined under Int theory in SMTLIB2.

use std::fmt;
use std::fmt::Debug;

#[macro_use]
use theories::utils;
use backends::backend::SMTNode;

#[derive(Clone, Debug)]
pub enum OpCodes {
    Cmp,
    Lt,
    Gt,
    Lte,
    Gte,
    Mod,
    Div,
    Abs,
    Mul,
    Add,
    Sub,
    Neg,
    Const(u64),
    FreeVar(String),
}


impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCodes::Cmp => "=".to_owned(),
            OpCodes::Lt => "<".to_owned(),
            OpCodes::Gt => ">".to_owned(),
            OpCodes::Lte => "<=".to_owned(),
            OpCodes::Gte => ">=".to_owned(),
            OpCodes::Mod => "mod".to_owned(),
            OpCodes::Div => "div".to_owned(),
            OpCodes::Abs => "abs".to_owned(),
            OpCodes::Mul => "*".to_owned(),
            OpCodes::Add => "+".to_owned(),
            OpCodes::Sub => "-".to_owned(),
            OpCodes::Neg => "-".to_owned(),
            OpCodes::Const(ref val) => format!("{}", val),
            OpCodes::FreeVar(ref name) => format!("{}", name),
        };
        write!(f, "{}", s)
    }
}

impl_smt_node!(OpCodes, define vars [OpCodes::FreeVar(_)], define consts [OpCodes::Const(_)]);

#[derive(Clone, Debug)]
pub enum Sorts {
    Int
}

impl fmt::Display for Sorts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Int")
    }
}
