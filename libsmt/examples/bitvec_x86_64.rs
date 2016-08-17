//! Example showing bitvector and array usage examples

// Example ASM:
// -----------------
// global _main
// section .text
// main:
//   ; Instructions, not for our case.
//
// write:
//   ; Funtion epilogue and prologue skipped.
//   ; buf is located at rbp - 0xa.
//   ; rdi and rsi are symbolic, user controlled inputs.
//   lea rax, [rbp - 0xa]
//   add rax, rdi
//   mov [rax], rsi
//   ret

extern crate libsmt;

use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{array_ex, bitvec, core};
use libsmt::logics::qf_abv::QF_ABV;
use libsmt::logics::qf_abv;

macro_rules! bv_const {
    ($solver: ident, $i: expr, $n: expr) => { $solver.new_const(bitvec::OpCodes::Const($i, $n)) }
}

fn main() {
    let mut z3: z3::Z3 = Default::default();
    let mut solver = SMTLib2::new(Some(QF_ABV));

    // First we declare all the symbolic vars and constants that are needed in order to check if
    // the program is vulnerable.

    // Two symbolic vars corresponding to the user inputs.
    let rdi = solver.new_var(Some("rdi"), qf_abv::bv_sort(64));
    let rsi = solver.new_var(Some("rsi"), qf_abv::bv_sort(64));

    // Memory is defined to be constant of type Array of BitVectors.
    let mem = {
        let bit_vec_arr = qf_abv::array_sort(qf_abv::bv_sort(64),
                                             qf_abv::bv_sort(64));
        solver.new_var(Some("mem"), bit_vec_arr)
    };

    // rbp is a constant in this case (and equal to 0x9000).
    let rbp = bv_const!(solver, 0x9000, 64);

    let const_a = bv_const!(solver, 0xA, 64);
    let const_4 = bv_const!(solver, 0x4, 64);
    let const_badcafe = bv_const!(solver, 0xbadcafe, 64);
    // Constant array (of bitvectors) with all 0's. Used to reset memory.
    let const_mem_0 = {
        let arr_const = qf_abv::array_const(qf_abv::bv_sort(64),
                                            qf_abv::bv_sort(64),
                                            bitvec::OpCodes::Const(0, 64));
        solver.new_const(arr_const)
    };

    // Adding constraints based on the above asm.
    
    // Reset memory to 0
    solver.assert(core::OpCodes::Cmp, &[mem, const_mem_0]);
    // We know that the return address is at rbp + 0x4
    let ret_addr = solver.assert(bitvec::OpCodes::BvAdd, &[rbp, const_4]);

    // buf = rbp - 0xa
    let buf = solver.assert(bitvec::OpCodes::BvSub, &[rbp, const_a]);
    // rax = rax + rdi
    let rax = solver.assert(bitvec::OpCodes::BvAdd, &[buf, rdi]);

    // [rax] = rsi
    let new_mem = solver.assert(array_ex::OpCodes::Store, &[mem, rax, rsi]);

    // Assert for safety constaint i.e. Add a constraint to check if mem[ret_addr] is equal to
    // 0xbadcafe. If there is a satisfying solution, then the return address can be overwritten and
    // the solver returns the value of rdi and rsi that achieves this goal.
    let sel = solver.assert(array_ex::OpCodes::Select, &[new_mem, ret_addr]);
    solver.assert(core::OpCodes::Cmp, &[sel, const_badcafe]);

    // Check if we have a satisfying solution.
    if let Ok(result) = solver.solve(&mut z3) {
        println!("Out-Of-Bounds Write detected!");
        println!("rdi: 0x{:x}; rsi: 0x{:x};", result[&rdi], result[&rsi]);
    } else {
        println!("This program is not vulnerable to Out-Of-Bounds Write.");
    }
}
