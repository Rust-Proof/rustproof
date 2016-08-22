// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::process::Command;

// Uses a /example file as a system test for rustproof
// returns false on verification condition mismatch from function name prefix
fn test_example_file(file: &str) -> bool {

    // Clean rustproof to ensure this test runs
    // Note: this does not increase test time
    Command::new("cargo").args(&["clean","-p", "rustproof"]).output()
        .expect("failed to execute child process: cargo clean -p rustproof");

    // Compile the example
    let output = Command::new("cargo").args(&["build", "--test", file]).output()
        .expect(format!("failed to execute child process: cargo build --test {}", file).as_str());

    // Read strerr for any rustproof errors. if found, return false
    let stderr_result = String::from_utf8_lossy(&output.stderr);
    let split_err = stderr_result.split("\n");
    for s in split_err {
        if s.starts_with("error") {
            return false;
        }
    }

    // Process the output
    let stdout_result = String::from_utf8_lossy(&output.stdout);
    let split = stdout_result.split("\n");

    // For each function
    for s in split{
        if s.starts_with("fn") {
            // If the output line starts with "invalid" it must end with "not valid"
            // If the output line starts with "valid" it must end with "valid"
            // If there is a mismatch, we have a test failure.
            // Lines beginning with anything else should be ignored
            if !((s.starts_with("fn invalid") && s.ends_with("not valid."))
               || (s.starts_with("fn valid") && s.ends_with("valid.") && !s.ends_with("not valid."))) {
                return false;
            }
        }
    }

    return true;
}

#[test]
fn test_examples() {
    assert!(test_example_file("test_conditions"));
}

// Test examples for unsigned examples
//#[test]
//fn test_unsigned_examples(){
//    assert!(test_example_file(""));
//}

// Test examples for signed examples
//#[test]
//fn test_signed_examples(){
//    assert!(test_example_file(""));
//}

// Test example for boolean examples
//#[test]
//fn test_boolean_examples(){
//    assert!(test_example_file(""))
//}

// Test example for conditional examples
//#[test]
//fn test_conditional_exampels(){
//    assert!(test_example_file(""));
//}

// Test example for assertion examples
//#[test]
//fn test_assertion_examples(){
//    assert!(test_example_file(""));
//}

#[test]
#[should_panic]
fn test_system_test_validity() {

    assert!(test_example_file("test_fail_valid"));
    assert!(test_example_file("test_fail_invalid"));
}
