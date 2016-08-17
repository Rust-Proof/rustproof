// A simple example to highlight the application of Z3 and interacting with it through Rust.

// Consider following set of constraints :-
// x + y > 5
// x > 1
// y > 1

// To find a satisfiable solution of x & y for above equations we can write the following SMTLIB2 syntax in Z3 :-

// ; SMTLIB2 code for above constraints
// (set-logic LIA)
// (declare-fun x () Int)
// (declare-fun y () Int)
// (assert (> (+ x y) 5))
// (assert (> x 1))
// (assert (> y 1))

// The libsmt library is designed to simplify this process of interacting with Z3 and enable to do so programmatically through Rust


// Import the libsmt library
extern crate libsmt;

use libsmt::backends::smtlib2::*;
use libsmt::backends::backend::*;
use libsmt::backends::z3;

// Include the Int theory and its functions
use libsmt::theories::{integer};

// Include the LIA logic
use libsmt::logics::lia::LIA;

fn main() {

    let mut z3: z3::Z3 = Default::default();
    // Defining an instance of Z3 solver
    let mut solver = SMTLib2::new(Some(LIA));
    solver.set_logic(&mut z3);

    // Defining the symbolic vars x & y
    let x = solver.new_var(Some("x"),integer::Sorts::Int);
    let y = solver.new_var(Some("y"),integer::Sorts::Int);

    // Defining the integer constants
    let int5 = solver.new_const(integer::OpCodes::Const(5));
    let int1 = solver.new_const(integer::OpCodes::Const(1));

    // Defining the assert conditions
    let cond1 = solver.assert(integer::OpCodes::Add, &[x, y]);
    let _  = solver.assert(integer::OpCodes::Gt, &[cond1, int5]);
    let _  = solver.assert(integer::OpCodes::Gt, &[x, int1]); 
    let _  = solver.assert(integer::OpCodes::Gt, &[y, int1]);

    if let Ok(result) = solver.solve(&mut z3) {
        println!("x: {}; y: {}", result[&x], result[&y]);
    } else {
        println!("No solutions for x and y found for given set of constraints");
    }

}

