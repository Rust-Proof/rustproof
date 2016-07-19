// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// trash code [demo]
pub fn demo() {
    println!("weakest precondition - reporting in");
}


//FIXME: wp is a predicate but is just a place holder for now. Will need appropriate type in
//       function arguments
pub fn stmt_substitution(wp: predicate, stmt: StatementKind)  {
    let var_to_sub = stmt.Lvalue;
    let sub_with = stmt.Rvalue;
    match var_to_sub {
        StatementKind::Lvalue::Var => {unimplemented!();}
        StatementKind::Lvalue::Temp => {unimplemented!();}
        StatementKind::Lvalue::Arg => {unimplemented!();}
        StatementKind::Lvalue::Static => {unimplemented!();}
        StatementKind::Lvalue::ReturnPointer => {unimplemented!();}
        StatementKind::Lvalue::Projection => {unimplemented!();}
    }
    match sub_with {
        StatementKind::Rvalue::CheckedBinaryOp => {unimplemented!();}
        StatementKind::Rvalue::BinaryOp => {unimplemented!();}
        StatementKind::Rvalue::UnaryOp => {unimplemented!();}
        _=> {unimplemented!();}
    }
}
