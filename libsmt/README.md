# libsmt.rs

[![Build Status](https://travis-ci.org/sushant94/libsmt.rs.svg?branch=master)](https://travis-ci.org/sushant94/libsmt.rs)
[![Coverage Status](https://coveralls.io/repos/github/sushant94/libsmt.rs/badge.svg?branch=enable_coveralls)](https://coveralls.io/github/sushant94/libsmt.rs?branch=master)

Rust bindings for SMTLIB2

libsmt.rs allows developers to write and interact with SMT Solvers that
conform to the SMTLIB2 Standard.

The current implementation supports [Z3](http://rise4fun.com/Z3) as the default solver, however, it
is trivial to use it with other solvers that accept input in the SMTLIB2
format.

## Documentation

Some documentation exists [here](http://sushant94.me/libsmt.rs/)

Documentation will be written lazily. This will remain a task of least
priority until the library is stable and robust. If you are interested in using this
library and need some documentation, feel free to raise an issue and I'd be
more than happy to help you out.

## Usage Example

Usage examples are available in
[examples/](https://github.com/sushant94/libsmt.rs/tree/master/examples)

## Todo

The current implementation does not expose the full power of SMT Solvers. Here
are some items on our roadmap to complete this library:

- Implement solver features like:
 - declare-const
 - define-fun
 - define-sort
 - declare-sort

- Allow users to pass options to SMT Solvers
- Improved Type Checking


However, features unexposed by the API can still be used through
`raw_read()` and `raw_write()` that exposes the underlying solver directly.

## Contributing

Contributions in form of suggestions, issues and (especially) pull requests
are most welcome! Please follow the Rust Style of coding and ensure that
tests are written while submitting a Pull Request.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution Notice

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
