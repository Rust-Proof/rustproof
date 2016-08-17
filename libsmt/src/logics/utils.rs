//! Defines macros to build Logics on top of theories.

use std::convert::Into;
use std::fmt;

use backends::backend::{Logic, SMTNode};

#[macro_export]
macro_rules! define_sorts_for_logic {
    ($logic: ident, $($variant: ident -> $sort: ty),*) => {
        #[derive(Clone, Debug)]
        pub enum $logic {
            $(
                $variant($sort),
            )*
        }
        
        $(
            impl Into<$logic> for $sort {
                fn into(self) -> $logic {
                    $logic::$variant(self)
                }
            }
        )*

            impl fmt::Display for $logic {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let s = match *self {
                        $(
                            $logic::$variant(ref op) => op.to_string(),
                         )*
                    };
                    write!(f, "{}", s)
                }
            }
    }
}

#[macro_export]
macro_rules! define_fns_for_logic {
    ($logic: ident, $($variant: ident -> $sort: ty),*) => {
        #[derive(Clone, Debug)]
        pub enum $logic {
            $(
                $variant($sort),
            )*
        }
        
        $(
            impl Into<$logic> for $sort {
                fn into(self) -> $logic {
                    $logic::$variant(self)
                }
            }
        )*

            impl fmt::Display for $logic {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    let s = match *self {
                        $(
                            $logic::$variant(ref op) => op.to_string(),
                         )*
                    };
                    write!(f, "{}", s)
                }
            }

        impl SMTNode for $logic {
            fn is_var(&self) -> bool {
                match *self {
                    $(
                        $logic::$variant(ref inner) => inner.is_var(),
                    )*
                }
            }

            fn is_const(&self) -> bool {
                match *self {
                    $(
                        $logic::$variant(ref inner) => inner.is_const(),
                    )*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_logic {
    ($logic: ident, $op: ident, $sorts: ty, map { $($fv: pat => $rt: path),* }) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $logic;

        impl fmt::Display for $logic {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", stringify!($logic))
            }
        }

        impl Logic for $logic {
            type Fns = $op;
            type Sorts = $sorts;

            fn free_var<T: AsRef<str>>(name: T, ty: $sorts) -> Self::Fns {
                match ty {
                    $(
                        $fv => $rt(name.as_ref().to_owned()).into(),
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}
