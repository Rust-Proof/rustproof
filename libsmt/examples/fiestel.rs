// Four rounds of custom fiestel cipher (taken from bkp2016 fiestel challenge).
// NOT production grade crypto ;)
// Though Z3 was unable to solve the 17 round fiestel network that was a part of the
// challenge, this serves as a good example for the usage of this library.

#[macro_use] extern crate libsmt;

use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;
use libsmt::theories::{bitvec, core};
use libsmt::logics::qf_abv::QF_ABV;
use libsmt::logics::qf_abv;

fn main() {

    let mut z3: z3::Z3 = Default::default();
    // Create a new instance of a solver.
    let mut solver = SMTLib2::new(Some(QF_ABV));

    // We need to find the values of the left and right keys. Set these as symbolic.
    let lk = solver.new_var(Some("LK"), qf_abv::bv_sort(24));
    let rk = solver.new_var(Some("RK"), qf_abv::bv_sort(24));

    // An input of 6 A's. Split as 'rt' and 'lt' (0x41 = ord('A'))
    let mut rt = bv_const!(solver, 0x414141, 24);
    let mut lt = bv_const!(solver, 0x414141, 24);

    // Four round fiestel network, iterate and add the corresponding constraints.
    for _ in 0..4 {
        lt = {
            let leftk_p_rt = solver.assert(bitvec::OpCodes::BvAdd, &[lk, rt]);
            let rot_res_l = solver.assert(bitvec::OpCodes::RotateLeft(11), &[leftk_p_rt]);
            solver.assert(bitvec::OpCodes::BvXor, &[lt, rot_res_l])
        };

        rt = {
            let rightk_p_lt = solver.assert(bitvec::OpCodes::BvAdd, &[rk, lt]);
            let rot_res_l = solver.assert(bitvec::OpCodes::RotateLeft(11), &[rightk_p_lt]);
            solver.assert(bitvec::OpCodes::BvXor, &[rt, rot_res_l])
        };
    }

    // fd0893dfe111: The value of the output from the encryption service.
    let lt_const = bv_const!(solver, 0xfd0893, 24);
    let rt_const = bv_const!(solver, 0xdfe111, 24);

    // Assert that our final result should correspond to these values.
    let _  = solver.assert(core::OpCodes::Cmp, &[lt, lt_const]);
    let _ = solver.assert(core::OpCodes::Cmp, &[rt, rt_const]);

    // Print the required keys.
    if let Ok(result) = solver.solve(&mut z3) {
        println!("LK: {:x}; RK: {:x}", result[&lk], result[&rk]);
    } else {
        println!("No Solution.");
    }
}
