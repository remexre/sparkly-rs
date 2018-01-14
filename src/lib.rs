//! A pretty-printing library based on Wadler's "A Prettier Printer" with support for ANSI styling.
#![warn(missing_docs)]

extern crate ansi_term;

#[cfg(test)]
extern crate itertools;

#[cfg(feature = "termion")]
extern crate termion;

mod ctors;
mod render;
#[cfg(test)]
mod tests;

use std::borrow::Cow;

pub use ansi_term::{Colour, Style};

pub use render::Display;

/// The type of pretty-printed text.
#[derive(Clone, Debug, PartialEq)]
pub struct Doc {
    inner: DocInner,
}

impl From<DocInner> for Doc {
    fn from(inner: DocInner) -> Doc {
        Doc { inner }
    }
}

/// The actual enum for `Doc`.
#[derive(Clone, Debug, PartialEq)]
enum DocInner {
    /// An alternation between two possible ways to format the same content.
    ///
    /// The two `Doc`s must flatten to the same `Doc`.
    /// No first line of the left `Doc` may be shorter than any first line of
    /// the right `Doc`.
    Alt(Box<DocInner>, Box<DocInner>),

    /// A concatenation of two documents, without inserting a break between
    /// them.
    Append(Box<DocInner>, Box<DocInner>),

    /// A newline. The parameter is the string the newline collapses to; the
    /// newline is uncollapsible if it is `None`. The string must not contain
    /// `"\n"`.
    Line(Option<&'static str>),

    /// Changes the indentation *by* (not to) the given amount, measured in
    /// spaces. Indentation is inserted _after_ a newline.
    Nest(usize, Box<DocInner>),

    /// Applies a style to the `Doc`.
    Style(Style, Box<DocInner>),

    /// A string, which must not contain `"\n"`. This is a `Cow` to avoid
    /// allocating a bunch of `" "` and `""` on the heap.
    ///
    /// The `From<&str>` impl for `Doc` does handle newlines properly.
    Text(Cow<'static, str>),
}

/// A trait for values that are pretty-printable.
trait Sparkly {
    fn to_doc(&self) -> Doc;
}

impl Sparkly for Doc {
    fn to_doc(&self) -> Doc {
        self.clone()
    }
}

impl<T: AsRef<Sparkly>> Sparkly for T {
    fn to_doc(&self) -> Doc {
        self.as_ref().to_doc()
    }
}
