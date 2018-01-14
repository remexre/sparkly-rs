# sparkly-rs

A pretty-printing library based on Wadler's "A Prettier Printer" with support for ANSI styling.

I'm using this to walk myself through the paper and understand it well enough to implement; plus it doesn't look like there's an existing implementation of this.

## Features and Optional Dependencies

[`termion`](https://github.com/ticki/termion): Allows the width and color support of the terminal to be autodetected.

## TODOs

 - When `impl Trait` is stabilized, use `impl Display` instead of the `Display` struct.
 - `no_std` support.
 - Opt-in [`unicode-width`](https://github.com/unicode-rs/unicode-width)
 - Windows (before 10) support? A good project might be a `(&mut Write, &str, Style) -> IoResult<()>` function that uses Windows syscalls there and ANSI everywhere else.
