# sparkly-rs

[![Build Status](https://travis-ci.org/remexre/sparkly-rs.svg?branch=master)](https://travis-ci.org/remexre/sparkly-rs)
[![Crates.io](https://img.shields.io/crates/v/sparkly.svg)](https://crates.io/crates/sparkly)
[![Documentation](https://docs.rs/sparkly/badge.svg)](https://docs.rs/sparkly/*/sparkly/)
![License](https://img.shields.io/crates/l/sparkly.svg)

A pretty-printing library based on Wadler's "A Prettier Printer" with support for ANSI styling.

I'm using this to walk myself through the paper and understand it well enough to implement; plus it doesn't look like there's an existing implementation of this.

## Features and Optional Dependencies

[`termion`](https://crates.io/crates/termion): Allows the width and color support of the terminal to be autodetected.

## TODOs

 - When `impl Trait` is stabilized, use `impl Display` instead of the `Display` struct.
 - `no_std` support.
 - Opt-in [`unicode-width`](https://crates.io/crates/unicode-width)
 - Windows (before 10) support? A good project might be a `(&mut Write, &str, Style) -> IoResult<()>` function that uses Windows syscalls there and ANSI everywhere else.

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license (http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
