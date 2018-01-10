use std::fmt::{Display as DisplayTrait, Formatter, Result as FmtResult};
#[cfg(feature = "termion")]
use std::io::{stdout, Result as IoResult, Write};
#[cfg(feature = "termion")]
use std::os::unix::io::AsRawFd;

#[cfg(feature = "termion")]
use termion::{get_tty, is_tty, terminal_size};

use Doc;

impl Doc {
    /// Returns an object that implements `Display` for the given settings.
    pub fn display(&self, width: usize, color: bool) -> Display {
        Display {
            color,
            doc: self,
            width,
        }
    }

    #[cfg(feature = "termion")]
    /// Returns an object that implements `Display` for the current size of the
    /// terminal. Color support is assumed if `stdout` is a TTY.
    pub fn display_term(&self) -> IoResult<Display> {
        let (width, _) = terminal_size()?;
        Ok(self.display(width as usize, is_tty(&stdout())))
    }

    #[cfg(feature = "termion")]
    /// Writes the `Doc` to a `Write` for the current size of the terminal.
    /// Color support is assumed if the `Write` is a TTY.
    pub fn write_to<W: AsRawFd + Write>(&self, mut w: W) -> IoResult<()> {
        let (width, _) = terminal_size()?;
        let disp = self.display(width as usize, is_tty(&w));
        write!(w, "{}", disp)
    }

    #[cfg(feature = "termion")]
    /// Writes the `Doc` to the terminal. Color support is assumed.
    pub fn write_to_tty(&self) -> IoResult<()> {
        let tty = get_tty()?;
        self.write_to(tty)
    }
}

/// A helper struct for rendering `Doc`s to an `std::fmt::Write`.
#[derive(Debug)]
pub struct Display<'doc> {
    color: bool,
    doc: &'doc Doc,
    width: usize,
}

impl<'doc> Display<'doc> {
    fn fmt_doc(&self, doc: &Doc, fmt: &mut Formatter) -> FmtResult {
        match *doc {
            Doc::Nil => Ok(()),
            Doc::Text(ref s) => fmt.write_str(s),
            Doc::Line => fmt.write_str("\n"),
            Doc::Append(ref l, ref r) => {
                self.fmt_doc(l, fmt)?;
                self.fmt_doc(r, fmt)
            }
            Doc::Nest(n, ref doc) => match **doc {
                Doc::Line => {
                    fmt.write_str("\n")?;
                    for _ in 0..n {
                        fmt.write_str(" ")?;
                    }
                    Ok(())
                }
                Doc::Append(ref l, ref r) => {
                    // TODO This is _awful_.
                    self.fmt_doc(&Doc::Nest(n, l.clone()), fmt)?;
                    self.fmt_doc(&Doc::Nest(n, r.clone()), fmt)
                }
                ref doc => self.fmt_doc(doc, fmt),
            },
        }
    }
}

impl<'doc> DisplayTrait for Display<'doc> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        self.fmt_doc(self.doc, fmt)
    }
}
