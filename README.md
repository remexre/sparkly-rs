# sparkly-rs

A pretty-printing library based on Wadler's "A Prettier Printer" with support for ANSI styling.

I'm using this to walk myself through the paper and understand it well enough to implement; plus it doesn't look like there's an existing implementation of this.

With the [`termion`](https://github.com/ticki/termion) optional dependency, the width and color support of the terminal can be autodetected.

## TODOs

 - When `impl Trait` is stabilized, use `impl Display` instead of the `Display` struct.
 - `no_std` support.

