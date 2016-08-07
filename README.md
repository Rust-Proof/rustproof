# Rustproof

[![Build Status](https://travis-ci.org/Rust-Proof/rustproof.svg?branch=master)](https://travis-ci.org/Rust-Proof/rustproof)

Rustproof is a compiler plugin for the Rust programming language. It is designed
to generate verification conditions for their code. It will ensure that the program can be formally verified, thereby reducing the potential of bugs in the code and provide a level of guarantee about the behavior of the software.


# How to

To call rustproof:  

`#[condition(pre="", post="")]`

Where the "pre" and "post" conditions are logical expressions.

For example:

`#[condition(pre=" x:i32 > 0:i32 ", post=" x:i32 >= 5:i32 ")]`

A complete example of how to format:

    #[condition(pre=" x:i32 < 10:i32 && x:i32 > 0:i32", post=" return:i32 < 5:i32 ")]
	fn add_five_or_three(x: i32)-> i32  {
        if x > 3 {  
            x + 5  
        }  
        else {  
            x + 3  
        }  
    }

To use rustproof and ensure a clean build on your code:

`cargo build`

# Dependencies

Add the following lines to your `Cargo.toml` file:

```toml
[dependencies]
rustproof = { git = "https://github.com/Rust-Proof/rustproof.git"}
```

The following are outside dependencies required for rustproof. Please refer to the following links for proper build instructions.

[Z3Prover][z3]
[z3]:https://github.com/Z3Prover/z3

Your installation of Z3 needs to be set in your PATH for proper usage.



# Motivation
The purpose of this compiler plugin is to prove program correctness under specific conditions. This will allow for the programmer to ensure greater control over possible bugs within their code.

# Contributors
[Matthew Slocum][acro]  
[Sami Sahli][sahli]  
[Vincent Schuster][schuster]  
[Michael Salter][salter]  
[Bradley Rasmussen][rasmussen]  
[Drew Gohman][gohman]  
[Matthew O'Brien][obrien]  

[acro]:https://github.com/arc3x
[sahli]:https://github.com/ssahli
[schuster]:https://github.com/VSchuster
[salter]:https://github.com/salterm
[rasmussen]:https://github.com/bajr
[gohman]:https://github.com/found101
[obrien]:https://github.com/obriematt

# Reporting Issues

Please report all issues on the github [issue tracker][issues].

[issues]:https://github.com/Rust-Proof/rustproof/issues


# License

Rustproof is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE][1], [LICENSE-MIT][2], and [COPYRIGHT][3] for details.

[1]:https://github.com/Rust-Proof/rustproof/blob/master/LICENSE-APACHE
[2]:https://github.com/Rust-Proof/rustproof/blob/master/LICENSE-MIT
[3]:https://github.com/Rust-Proof/rustproof/blob/master/COPYRIGHT
