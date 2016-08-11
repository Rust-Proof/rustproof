// The Rust-Proof Project is copyright 2016, Sami Sahli,
// Michael Salter, Matthew Slocum, Vincent Schuster,
// Bradley Rasmussen, Drew Gohman, and Matthew O'Brien.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter;
use std::env;
use std::path;
use std::process::{Command, Stdio};
use std::process::ExitStatus;
use std::thread::spawn;

// skeleton for running rustproof as a test
// Commented out until I figure out static strings
//static valid_return : String = "\nVerification Condition is valid.\nUnsat(\"unsat\")\n".to_string();
//static unsat_return : String = "\nVerification Condition is invalid.\nSat(\"sat\")\n".to_string();

#[test]
fn demo_ssahli_demo() {
    //Creates a new thread to run plugin-examples
//    let thread = spawn(|| {
        //
    let output = Command::new("cargo")
        .arg("build")
        .arg("--example")
        .arg("ssahli")
        //.stdout(Stdio::piped())
        .output()
        .unwrap_or_else(|e| {panic!(1)});
    let stdout_result = String::from_utf8_lossy(&output.stdout);
    //println!("HEY MATT LOOK AT ME !: {:?}", stdout_result);
    //println!("{:?}", stdout_result);
    let correct_result = "\nVerification Condition is valid.\nUnsat(\"unsat\")\n".to_string();
    //println!("{:?}", correct_result);
    assert_eq!(stdout_result, correct_result);


//    });
    //let result = output.status;
    //println!("This is the exit result: {:?}", result);
//new stuff
/*    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    //Added but still in prototype phase.
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    //end of new stuff


    //May not need was used for previous version for taking in a integer.
    match result {
        Ok(_) => { println!("thread join result ok");}
        Err(e) => {
            let new_e = e.downcast::<String>();
            match new_e {
                Ok(e2) => {
                    println!("Got an error integer {:?}", e2);
                }
                Err(e3) => { println!("Got unknown error: {:?}", e3); }
            }
        }
    }

*/

}
