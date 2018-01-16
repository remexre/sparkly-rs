use std::fmt::{Display as DisplayTrait, Formatter, Result as FmtResult};
#[cfg(feature = "termion")]
use std::io::{stdout, Result as IoResult, Write};
#[cfg(feature = "termion")]
use std::os::unix::io::AsRawFd;

use ansi_term::Style;
#[cfg(feature = "termion")]
use termion::{get_tty, is_tty, terminal_size};

use {Doc, DocInner};

impl Doc {
    /// Returns an object that implements `Display` for the current size of the
    /// terminal, or 80 columns wide if it cannot be detected. Color is enabled
    /// if `stdout` is a TTY.
    ///
    /// Terminal width detection requires the `termion` feature.
    pub fn display(&self) -> Display {
        #[cfg(feature = "termion")]
        let width = terminal_size().map(|(w, _)| w).unwrap_or(80);
        #[cfg(not(feature = "termion"))]
        let width = 80;

        #[cfg(feature = "termion")]
        let color = is_tty(&stdout());
        #[cfg(not(feature = "termion"))]
        let color = false;

        self.display_opts(width as usize, color)
    }

    /// Returns an object that implements `Display` for the given options.
    pub fn display_opts(&self, width: usize, color: bool) -> Display {
        Display {
            color,
            doc: &self.inner,
            width,
        }
    }

    /// Returns an object that implements `Display` for the current size of the
    /// terminal. Color support is assumed if `stdout` is a TTY.
    #[cfg(feature = "termion")]
    pub fn display_term(&self) -> IoResult<Display> {
        let (width, _) = terminal_size()?;
        Ok(self.display_opts(width as usize, is_tty(&stdout())))
    }

    /// Writes the `Doc` to a `Write` for the current size of the terminal.
    /// Color support is assumed if the `Write` is a TTY.
    #[cfg(feature = "termion")]
    pub fn write_to<W: AsRawFd + Write>(&self, mut w: W) -> IoResult<()> {
        let (width, _) = terminal_size()?;
        let disp = self.display_opts(width as usize, is_tty(&w));
        write!(w, "{}", disp)
    }

    /// Writes the `Doc` to a `Write` for the current size of the terminal,
    /// followed by a newline. Color support is assumed if the `Write` is a
    /// TTY.
    #[cfg(feature = "termion")]
    pub fn writeln_to<W: AsRawFd + Write>(&self, mut w: W) -> IoResult<()> {
        let (width, _) = terminal_size()?;
        let disp = self.display_opts(width as usize, is_tty(&w));
        writeln!(w, "{}", disp)
    }

    /// Writes the `Doc` to the terminal. Color support is assumed.
    #[cfg(feature = "termion")]
    pub fn write_to_tty(&self) -> IoResult<()> {
        let tty = get_tty()?;
        self.write_to(tty)
    }

    /// Writes the `Doc` to the terminal, followed by a newline.
    #[cfg(feature = "termion")]
    pub fn writeln_to_tty(&self) -> IoResult<()> {
        let tty = get_tty()?;
        self.writeln_to(tty)
    }
}

/// A helper struct for rendering `Doc`s to an `std::fmt::Write`.
#[derive(Debug)]
pub struct Display<'doc> {
    color: bool,
    doc: &'doc DocInner,
    width: usize,
}

impl<'doc> DisplayTrait for Display<'doc> {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        for r in best(self.width, 0, vec![(0, Style::new(), &self.doc)]) {
            match r {
                RenderDoc::Line(i) => {
                    fmt.write_str("\n")?;
                    for _ in 0..i {
                        fmt.write_str(" ")?;
                    }
                }
                RenderDoc::Text(s, sty) => if self.color {
                    write!(fmt, "{}", sty.paint(s))?;
                } else {
                    fmt.write_str(s)?;
                },
            }
        }
        Ok(())
    }
}

enum RenderDoc<'doc> {
    Line(usize),
    Text(&'doc str, Style),
}

// `w` is the width of the terminal, `k` is the number of characters already
// placed.
fn best<'doc>(
    w: usize,
    mut k: usize,
    mut stack: Vec<(usize, Style, &'doc DocInner)>,
) -> Vec<RenderDoc<'doc>> {
    let mut rendered = Vec::new();
    while let Some((i, sty, doc)) = stack.pop() {
        match *doc {
            DocInner::Append(ref x, ref y) => {
                stack.push((i, sty, y));
                stack.push((i, sty, x));
            }
            DocInner::Nest(j, ref x) => {
                stack.push((i + j, sty, x));
            }
            DocInner::Text(ref s) => {
                k += s.len();
                rendered.push(RenderDoc::Text(s, sty));
            }
            DocInner::Line(_) => {
                k = i;
                rendered.push(RenderDoc::Line(i));
            }
            DocInner::Alt(ref x, ref y) => {
                // TODO: There ought to be a better way to do this.
                let mut l = stack.clone();
                l.push((i, sty, x));
                let mut r = stack;
                r.push((i, sty, y));
                rendered.extend(better(w, k, best(w, k, l), best(w, k, r)));
                return rendered;
            }
            DocInner::Style(sty, ref x) => {
                stack.push((i, sty, x));
            }
        }
    }
    rendered
}

fn better<'doc>(
    w: usize,
    k: usize,
    l: Vec<RenderDoc<'doc>>,
    r: Vec<RenderDoc<'doc>>,
) -> Vec<RenderDoc<'doc>> {
    if fits(w, k, &l) {
        l
    } else {
        r
    }
}

fn fits<'a, 'b>(w: usize, mut k: usize, docs: &'a [RenderDoc<'b>]) -> bool {
    for doc in docs {
        if k > w {
            return false;
        } else {
            match doc {
                &RenderDoc::Line(_) => return true,
                &RenderDoc::Text(s, _) => {
                    k += s.len();
                }
            }
        }
    }
    true
}
