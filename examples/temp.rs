extern crate rustproof;

fn main() {
    rustproof::control::demo();
    rustproof::reporting::demo();
    rustproof::z3_interface::demo();
    rustproof::weakest_precondition::demo();
    rustproof::parser::demo();
}