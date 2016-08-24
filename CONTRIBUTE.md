# Contributing to rustproof

If you're looking to contribute to rustproof, this document should help you understand the design structure of the project.

Last updated Aug 24

##  Overview
A brief overview of the rustproof directory. More details on each module are in the next section.

`src`
: where the rustproof internals live.

`tests`
: system tests are located here. Testing is covered in more detail in the rustproof internals section.

`examples`
: examples that a new rustproof user can examine and run using `cargo build --example <example_file>`

`scripts`
: currently the only script here is for Travis CI

## rustproof internals
Within `src`, you'll find the following modules:

### `src/expression`
This module is what creates rustproof's internal representations of logical expressions. Its functions are used in multiple places to create expressions from pre/post conditions, and from user written code (ultimately, from rust's MIR statements).

Files: `mod.rs`

### `src/parser`
The parser that uses generates `expression`s from a user's pre/post conditions. The `mod.rs` file has a function for checking the `#[condition]` attribute for errors, and a function for calling the parser.

Some more details on the parser: `expression_parser.rs` is not intended to be modified manually. It is a LR(1) parser auto generated using the [LALRPOP](https://github.com/nikomatsakis/lalrpop) library. In order to modify the parser, you can modify the grammar rules in `expression_parser.lalrpop`.

Files: `mod.rs`, `expression_parser.rs`, `expression_parser.lalrpop`

### `src/weakest_precondition`
The brains behind generating a weakest precondition. This file generates the weakest precondition `wp` in `expression` format from MIR statements of the user's code. The `gen()` function performs a recursive depth-first search on the MIR control-flow graph, performing necessary replacements in `wp`'s expression in reverse order. For example, a post-condition `return: i32 == (x: i32 + 5i32)` would be the `wp` when `gen()` returns from the exit point of a function. If the next MIR statement prior to the exit point of the function is `return = (tmp1: i32)`, then a replacement occurs with the result being `tmp1: i32 == (x: i32 + 5i32)`. This continues until all MIR statements are read, ending with the first MIR statement of the function.

Additionally there is the file `overflow.rs`: this file contains functions for overflow checking. If an expression contains the binary operator `signed add`, then an additional set of expressions is added onto `wp` to check for overflow.

Files: `mod.rs`, `overflow.rs`

### `src/smt_output`
This module translates `wp` from the internal `expression` format to SMT-lib format that is used by the SMT solver Z3. It then sends the final expression to [libsmt.rs](https://github.com/Rust-Proof/libsmt.rs), which runs Z3 in a child process, and returns any relevant information to the user.

Files: `mod.rs`

### `src/reporting`
rustproof's internal reporting module. This allows developers to throw meaningful rustproof warnings and errors to the user without exposing them to unhelpful rustproof internals. The macros are `rp_warn!()` and `rp_error!()`.

Files: `mod.rs`

### `src/tests`
This is where unit tests live, as well as where system tests are called from. Unit tests for most modules, namely `weakest_precondition`, are not only difficult to do (such as creating MIR stubs) but are also redundant with thorough system tests. `system_tests.rs` creates child processes that run the system tests located in the root directory's `tests` folder. Each function in a system test must begin with `valid` or `invalid` to correspond with their expected return, and is compared with their actual output to see if the test fails or passes. While this method currently works, it does not work with versions of rust nightly after 2016-08-11. Ultimately this method of system testing must change.

Files: `mod.rs`, `system_tests.rs`, `test_expression.rs`

### `lib.rs`
This file is the first point of entry for rustproof. It contains the `#[plugin_registrar]` attribute that rustc uses to identify compiler plugins. The `fn registrar()` is called once during compilation to register the used plugins, which in this case is the `MirPass`. `MirPass` is called once per user function (which may need to change in order to implement verification of function calls in user code). `MirPass` collects information about the function and its MIR statements, calls `gen()` from `weakest_preconditon`, and finishes with calling `gen_smtlib()` from `smt_output`. 
