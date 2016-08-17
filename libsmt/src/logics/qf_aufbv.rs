use std::fmt::{Display, Debug};
use std::fmt;

use theories::{array_ex, bitvec, core};
use backends::backend::{Logic, SMTNode};

define_sorts_for_logic!(QF_AUFBV_Sorts,
                        BV -> bitvec::Sorts,
                        Core -> core::Sorts,
                        ArrayEx -> array_ex::Sorts<QF_AUFBV_Sorts, QF_AUFBV_Sorts>
                        );

define_fns_for_logic!(QF_AUFBV_Fn,
                      BVOps -> bitvec::OpCodes,
                      CoreOps -> core::OpCodes,
                      ArrayOps -> array_ex::OpCodes<QF_AUFBV_Sorts, QF_AUFBV_Sorts, QF_AUFBV_Fn>
                      );

define_logic!(QF_AUFBV,
              QF_AUFBV_Fn,
              QF_AUFBV_Sorts,
              map { QF_AUFBV_Sorts::BV(_) => bitvec::OpCodes::FreeVar,
                  QF_AUFBV_Sorts::ArrayEx(_) => array_ex::OpCodes::FreeVar
              }
              );
