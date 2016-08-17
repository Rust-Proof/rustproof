//! Struct and methods to interact with the Z3 solver.


//extern crate native;
//extern crate rustrt;

//use std::io::{process, Command};
//use native::io::file;
///use rustrt::rtio;

// for file
/*
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::os::unix::io::IntoRawFd;
use std::path::Path;
*/
//f2

use backends::smtlib2::SMTProc;
use std::process::{Child, Command, Stdio};


#[derive(Default)]
pub struct Z3 {
    fd: Option<Child>,
}

impl SMTProc for Z3 {
    fn init(&mut self) {
        let (cmd, args) = ("z3", &["-in", "-smt2"]);
        let child = Command::new(cmd)
                        .args(args)
                        .stdout(Stdio::piped())
                        .stdin(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to spawn process");

        self.fd = Some(child);
    }

    fn pipe<'a>(&'a mut self) -> &'a mut Child {
        if self.fd.is_none() {
            self.init();
        }
        self.fd.as_mut().unwrap()
    }
}

#[cfg(test)]
mod test {
    use backends::backend::*;
    use backends::smtlib2::*;
    use super::*;
    use theories::bitvec;
    use theories::{core, integer};
    use logics::{lia, qf_bv};

    #[test]
    fn test_z3_int() {
        let mut z3: Z3 = Default::default();
        let mut solver = SMTLib2::new(Some(lia::LIA));
        let x = solver.new_var(Some("X"), integer::Sorts::Int);
        let y = solver.new_var(Some("Y"), integer::Sorts::Int);
        let c = solver.new_const(integer::OpCodes::Const(10));
        solver.assert(core::OpCodes::Cmp, &[x, c]);
        solver.assert(integer::OpCodes::Gt, &[x, y]);
        let result = solver.solve(&mut z3).unwrap();
        assert_eq!(result[&x], 10);
        assert_eq!(result[&y], 9);
    }

    #[test]
    fn test_z3_bitvec() {
        let mut z3: Z3 = Default::default();
        let mut solver = SMTLib2::new(Some(qf_bv::QF_BV));
        let x = solver.new_var(Some("X"), bitvec::Sorts::BitVector(32));
        let c = solver.new_const(bitvec::OpCodes::Const(10, 32));
        let c8 = solver.new_const(bitvec::OpCodes::Const(8, 32));
        let y = solver.new_var(Some("Y"), bitvec::Sorts::BitVector(32));
        solver.assert(core::OpCodes::Cmp, &[x, c]);
        let x_xor_y = solver.assert(bitvec::OpCodes::BvXor, &[x, y]);
        solver.assert(core::OpCodes::Cmp, &[x_xor_y, c8]);
        let result = solver.solve(&mut z3).unwrap();
        assert_eq!(result[&x], 10);
        assert_eq!(result[&y], 2);
    }

    #[test]
    fn test_z3_extract() {
        let mut z3: Z3 = Default::default();
        let mut solver = SMTLib2::new(Some(qf_bv::QF_BV));
        let x = solver.new_var(Some("X"), bitvec::Sorts::BitVector(32));
        let c4 = solver.new_const(bitvec::OpCodes::Const(0b100, 4));
        let x_31_28 = solver.assert(bitvec::OpCodes::Extract(31, 28), &[x]);
        solver.assert(core::OpCodes::Cmp, &[x_31_28, c4]);
        let result = solver.solve(&mut z3).unwrap();
        assert_eq!(result[&x], (0b100 << 28));
    }
}
