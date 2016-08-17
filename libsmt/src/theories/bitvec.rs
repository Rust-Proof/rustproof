//! Defines basic operations defined under FixedSizeBitVectors theory in SMTLIB2.

use std::fmt::Debug;
use std::fmt;

use backends::backend::SMTNode;

#[macro_export]
macro_rules! bv_const {
    ($solver: ident, $i: expr, $n: expr) => { $solver.new_const(bitvec::OpCodes::Const($i, $n)) }
}

#[derive(Clone, Debug)]
pub enum OpCodes {
    Concat,
    Extract(u64, u64),
    BvNot,
    BvAnd,
    BvOr,
    BvNeg,
    BvAdd,
    BvMul,
    BvSMulDoesNotOverflow,
    BvSMulDoesNotUnderflow,
    BvUMulDoesNotOverflow,
    BvUDiv,
    BvURem,
    BvShl,
    BvLShr,
    BvULt,
    BvNand,
    BvNor,
    BvXor,
    BvXnor,
    BvComp,
    BvSub,
    BvSDiv,
    BvSRem,
    BvSMod,
    BvAShr,
    // parameterized functions
    Repeat(u64),
    ZeroExtend(u64),
    SignExtend(u64),
    RotateLeft(u64),
    RotateRight(u64),
    // logical functions
    BvULe,
    BvUGt,
    BvUGe,
    BvSLt,
    BvSLe,
    BvSGt,
    BvSGe,
    Const(u64, usize),
    FreeVar(String),
}

impl fmt::Display for OpCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            OpCodes::Concat => "concat".to_owned(),
            OpCodes::Extract(i, j) => format!("(_ extract {} {})", i, j),
            OpCodes::BvNot => "bvnot".to_owned(),
            OpCodes::BvAnd => "bvand".to_owned(),
            OpCodes::BvOr => "bvor".to_owned(),
            OpCodes::BvNeg => "bvneg".to_owned(),
            OpCodes::BvAdd => "bvadd".to_owned(),
            OpCodes::BvMul => "bvmul".to_owned(),
            OpCodes::BvSMulDoesNotOverflow => "bvsmul_noovfl".to_owned(),
            OpCodes::BvSMulDoesNotUnderflow => "bvsmul_noudfl".to_owned(),
            OpCodes::BvUMulDoesNotOverflow => "bvumul_noovfl".to_owned(),
            OpCodes::BvUDiv => "bvudiv".to_owned(),
            OpCodes::BvURem => "bvurem".to_owned(),
            OpCodes::BvShl => "bvshl".to_owned(),
            OpCodes::BvLShr => "bvlshr".to_owned(),
            OpCodes::BvULt => "bvult".to_owned(),
            OpCodes::BvNand => "bvnand".to_owned(),
            OpCodes::BvNor => "bvnor".to_owned(),
            OpCodes::BvXor => "bvxor".to_owned(),
            OpCodes::BvXnor => "bvxnor".to_owned(),
            OpCodes::BvComp => "bvcomp".to_owned(),
            OpCodes::BvSub => "bvsub".to_owned(),
            OpCodes::BvSDiv => "bvsdiv".to_owned(),
            OpCodes::BvSRem => "bvsrem".to_owned(),
            OpCodes::BvSMod => "bvsmod".to_owned(),
            OpCodes::BvAShr => "bvashr".to_owned(),
            OpCodes::Repeat(i) => format!("(_ repeat {})", i),
            OpCodes::ZeroExtend(i) => format!("(_ zero_extend {})", i),
            OpCodes::SignExtend(i) => format!("(_ sign_extend {})", i),
            OpCodes::RotateLeft(i) => format!("(_ rotate_left {})", i),
            OpCodes::RotateRight(i) => format!("(_ rotate_right {})", i),
            OpCodes::BvULe => "bvule".to_owned(),
            OpCodes::BvUGt => "bvugt".to_owned(),
            OpCodes::BvUGe => "bvuge".to_owned(),
            OpCodes::BvSLt => "bvslt".to_owned(),
            OpCodes::BvSLe => "bvsle".to_owned(),
            OpCodes::BvSGt => "bvsgt".to_owned(),
            OpCodes::BvSGe => "bvsge".to_owned(),
            OpCodes::Const(val, n) => format!("(_ bv{} {})", val, n),
            OpCodes::FreeVar(ref name) => format!("{}", name),
        };
        write!(f, "{}", s)
    }
}

impl_smt_node!(OpCodes, define vars [OpCodes::FreeVar(_)], define consts [OpCodes::Const(_, _)]);

#[derive(Clone, Debug)]
pub enum Sorts {
    BitVector(usize),
    Bool,
}

impl fmt::Display for Sorts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Sorts::BitVector(ref n) => format!("(_ BitVec {})", n),
            Sorts::Bool => format!("{}", "bool"),
        };
        write!(f, "{}", s)
    }
}
