//! Crate that allows rust programs to interact with SMT (Satisfiability Modulo Theory) solvers.

extern crate petgraph;
extern crate regex;

pub mod theories {
    #[macro_use] pub mod utils;
    #[macro_use] pub mod bitvec;
    pub mod integer;
    pub mod core;
    pub mod array_ex;
    pub mod real;
    pub mod real_ints;
}

pub mod logics {
    #[macro_use] pub mod utils;
    pub mod qf_bv;
    pub mod qf_aufbv;
    pub mod qf_abv;
    pub mod lia;
}

pub mod backends {
    pub mod backend;
    pub mod smtlib2;
    pub mod z3;
}
