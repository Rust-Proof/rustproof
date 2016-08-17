use std::fmt::{Display, Debug};
use std::fmt;

use theories::{array_ex, bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QF_ABV_Sorts,
                        BV -> bitvec::Sorts,
                        Core -> core::Sorts,
                        ArrayEx -> array_ex::Sorts<QF_ABV_Sorts, QF_ABV_Sorts>
                        );

define_fns_for_logic!(QF_ABV_Fn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes,
                      ArrayOps -> array_ex::OpCodes<QF_ABV_Sorts, QF_ABV_Sorts, QF_ABV_Fn>
                      );

define_logic!(QF_ABV,
              QF_ABV_Fn,
              QF_ABV_Sorts,
              map { QF_ABV_Sorts::BV(_) => bitvec::OpCodes::FreeVar,
              QF_ABV_Sorts::ArrayEx(_) => array_ex::OpCodes::FreeVar
              }
              );


// Helper methods that help in contruction of array and bitvector types.
pub fn array_sort<T: Into<QF_ABV_Sorts>, P: Into<QF_ABV_Sorts>>(idx: T, data: P) -> QF_ABV_Sorts {
    QF_ABV_Sorts::ArrayEx(array_ex::Sorts::new(idx.into(), data.into()))
}

pub fn bv_sort(size: usize) -> QF_ABV_Sorts {
    QF_ABV_Sorts::BV(bitvec::Sorts::BitVector(size))
}

pub fn bv_const(val: u64, size: usize) -> QF_ABV_Fn {
    QF_ABV_Fn::BVOps(bitvec::OpCodes::Const(val, size))
}

pub fn array_const<T, P, Q>(idx_ty: T, val_ty: P, val: Q) -> QF_ABV_Fn
    where T: Into<QF_ABV_Sorts>,
          P: Into<QF_ABV_Sorts>,
          Q: Into<QF_ABV_Fn>
{
    let arr_ty = array_ex::Sorts::Array(Box::new(idx_ty.into()), Box::new(val_ty.into()));
    let arr_const = array_ex::OpCodes::Const(arr_ty, Box::new(val.into()));
    QF_ABV_Fn::ArrayOps(arr_const)
}
