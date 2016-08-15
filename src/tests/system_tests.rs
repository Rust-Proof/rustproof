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
fn test_example_file(file: String) -> bool {
    // Clean rustproof to ensure this test runs
    // Note: this does not increase test time
    Command::new("cargo")
        .arg("clean")
        .arg("-p")
        .arg("rustproof")
        .output()
        .unwrap();

    // Flag to set false when a test fails
    let mut no_failure = true;

    // Compile the example
    let output = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(file.clone())
        .output()
        .unwrap();

    // Process the output
    let stdout_result = String::from_utf8_lossy(&output.stdout);
    let split = stdout_result.split("\n\n");

    // For each function
    for s in split{
        if !s.is_empty() {
            // If the output line's starting symbol doesnt match the ending symbol, set failure flag
            if  !((s.starts_with("\nfn valid") && s.ends_with("valid.")) ||
                 (s.starts_with("\nfn invalid") && s.ends_with("not valid."))) {
                     println!("{:?} caused failure", s);
                     no_failure = false;
            }
        }
    }

    return no_failure;
}

#[test]
fn test_examples() {
    assert!(test_example_file("test_conditions".to_string()));
}
