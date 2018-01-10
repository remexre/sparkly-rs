//! A pretty-printing library based on Wadler's "A Prettier Printer" with support for ANSI styling.
#![warn(missing_docs)]

extern crate ansi_term;

#[cfg(feature = "termion")]
extern crate termion;

mod ctors;
mod render;
#[cfg(test)]
mod tests;

pub use render::Display;

/// The type of pretty-printed text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Doc {
    /// A Doc which contains no text.
    Nil,

    /// A string, which must not contain `"\n"`.
    ///
    /// The `From<&str>` impl for `Doc` does handle newlines properly.
    Text(String),

    /// A newline.
    Line,

    /// A concatenation of two documents, without inserting a break between
    /// them.
    Append(Box<Doc>, Box<Doc>),

    /// Changes the indentation *by* (not to) the given amount, measured in
    /// spaces. Indentation is inserted _after_ a newline.
    Nest(isize, Box<Doc>),
}

/// A trait for values that are pretty-printable.
trait Pretty {
    fn as_pretty(&self) -> Doc;
}

impl Pretty for Doc {
    fn as_pretty(&self) -> Doc {
        self.clone()
    }
}

impl<T: AsRef<Pretty>> Pretty for T {
    fn as_pretty(&self) -> Doc {
        self.as_ref().as_pretty()
    }
}
