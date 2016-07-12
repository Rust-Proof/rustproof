# Rustproof
A Rust compiler plugin to verify correctness of functions.

To run example code:  
    `cargo build --example <example>`

To ensure a clean build on the code:  
    `cargo clean && cargo build --example <example>`

# Code usage example

To use rustproof:  

    #[condition(pre="x > 0", post="x >= 5")]
      fn add_five_or_three(x: i32) -> i32 {`
        if x > 3 {  
            x + 5  
        }  
        else {  
            x + 3  
        }  
    }
    

# Motivation


# Todo


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
