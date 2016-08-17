//! Macro helpers for defining theories

use backends::backend::SMTNode;

#[macro_export]
macro_rules! impl_smt_node {
    ($fns: ty, define vars [$($variant: pat),*], define consts [$($c: pat),*]) => {
        impl SMTNode for $fns {
            fn is_var(&self) -> bool {
                match *self {
                $(
                    $variant => true,
                 )*
                    _ => false,
                }
            }

            fn is_const(&self) -> bool {
                match *self {
                $(
                    $c => true,
                )*
                    _ => false,
                }
            }
        }
    }
}
