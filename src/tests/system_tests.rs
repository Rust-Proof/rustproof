// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//use std::iter;
//use std::env;
//use std::path;
use std::process::Command;
//use std::process::ExitStatus;
//use std::thread::spawn;

// skeleton for running rustproof as a test
// Commented out until I figure out static strings
//static valid_return : String = "\nVerification Condition is valid.\nUnsat(\"unsat\")\n".to_string();
//static unsat_return : String = "\nVerification Condition is invalid.\nSat(\"sat\")\n".to_string();

#[test]
fn demo_ssahli_demo() {
    //Creates a new thread to run plugin-examples

    let output = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg("ssahli")
        .output()
        .unwrap();
    let stdout_result = String::from_utf8_lossy(&output.stdout);
    let correct_result = "\nfn add_five_or_three(..)\tVerification Condition is valid.\n\n".to_string();
    println!("The output is: {:?}", stdout_result);
    println!("The correct output is: {:?}", correct_result);
    assert_eq!(stdout_result, correct_result);

}


#[test]
fn test_many_instances(){

    //let mut flag = true;
    let mut checker = true;

    let output = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg("test_conditions")
        .output()
        .unwrap();

    let stdout_result = String::from_utf8_lossy(&output.stdout);
    let split = stdout_result.split("\n\n");
    for s in split{
        //println!("Is this string empty? {:?}", s.is_empty());
        if s.is_empty() == false {
            println!("{:?}", s);
            println!("Starts boolean: {:?}", s.ends_with("not valid."));
            println!("Ends boolean: {:?}", s.starts_with("\nfn invalid"));
            //assert_eq!(s.starts_with("\nfn invalid"), s.ends_with("not valid."));
            if s.ends_with("not valid.") != s.starts_with("\nfn invalid"){
                checker = false;
            }
        }
    }
    println!("The flag check is: {:?}", checker);
    assert!(checker);
}
