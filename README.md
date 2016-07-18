# Rustproof

Rustproof is a compiler plugin for the Rust programming language. It is designed
to generate verification conditions for their code. It will ensure that the program can be formally verified, thereby reducing the potential of bugs in the code and provide a level of guarantee about the behavior of the software.


# How to

To use rustproof:  

`#[condition(pre="", post="")]`

Where the "pre" and "post" conditions are logical expressions.

For example:

`#[condition(pre="x>0", post="x>=5")]`

A complete example of how to format:

    #[condition(pre="x > 0", post="x >= 5")]
	fn add_five_or_three(x: i32)-> i32  {
        if x > 3 {  
            x + 5  
        }  
        else {  
            x + 3  
        }  
    }

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
