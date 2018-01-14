use std::iter::FromIterator;

use ansi_term::Style;
use itertools::Itertools;

use {Doc, DocInner, Sparkly};

// The basic constructors, i.e. functions that don't take self and return Doc.
impl Doc {
    /// Returns an empty `Doc`.
    pub fn empty() -> Doc {
        Doc::from(DocInner::Text("".into()))
    }

    /// Expands to a newline, which will never be shortened.
    pub fn line() -> Doc {
        Doc::from(DocInner::Line(None))
    }

    /// Expands to the given string, if it will fit, or a newline if it won't.
    pub fn line_or(s: &'static str) -> Doc {
        Doc::from(DocInner::Line(Some(s)))
    }

    /// Concatenates `Doc`s, putting newlines between them.
    pub fn lines<I: IntoIterator<Item = T>, T: Sparkly>(iter: I) -> Doc {
        Doc::from(DocInner::Line(None)).join(iter)
    }
}

// Constructing combinators, i.e. functions that take self and return Doc.
impl Doc {
    /// Appends one `Doc` to another. Equivalent to the `Append` constructor.
    pub fn append(self, right: Doc) -> Doc {
        Doc::from(DocInner::Append(
            Box::new(self.inner),
            Box::new(right.inner),
        ))
    }

    /// Brackets a `Doc` between two constant strings.
    pub fn bracket(self, l: &'static str, r: &'static str) -> Doc {
        Doc::from(l)
            .maybe_split(self)
            .nest(4)
            .maybe_split(Doc::from(r))
            .group()
    }

    /// Groups the contents of a `Doc`.
    pub fn group(self) -> Doc {
        let inner = self.inner;
        Doc::from(DocInner::Alt(
            Box::new(inner.clone().flatten()),
            Box::new(inner),
        ))
    }

    /// Joins `Doc`s, placing the `self` `Doc` between them.
    pub fn join<I: IntoIterator<Item = T>, T: Sparkly>(self, iter: I) -> Doc {
        iter.into_iter()
            .map(|t| t.to_doc())
            .fold1(|l, r| l.append(self.clone()).append(r))
            .unwrap_or_else(Doc::empty)
    }

    /// Concatenates two `Doc`s, putting a newline between them.
    pub fn with_line(self, right: Doc) -> Doc {
        self.append(Doc::from(DocInner::Line(None))).append(right)
    }

    /// Concatenates two `Doc`s. If the line would overflow, a newline is
    /// inserted between them.
    pub fn maybe_split(self, right: Doc) -> Doc {
        self.append(Doc::from(DocInner::Line(Some(""))))
            .append(right)
    }

    /// Concatenates two `Doc`s, putting a space between them that will never
    /// be split to a newline.
    pub fn nbsp(self, right: Doc) -> Doc {
        self.append(Doc::from(DocInner::Text(" ".into())))
            .append(right)
    }

    /// Nests the `Doc` with the given amount of indentation.
    pub fn nest(self, n: usize) -> Doc {
        Doc::from(DocInner::Nest(n, Box::new(self.inner)))
    }

    /// Concatenates two `Doc`s, putting a space between them if it will fit,
    /// and a newline if it will not.
    pub fn space(self, right: Doc) -> Doc {
        self.append(Doc::from(DocInner::Line(Some(" "))))
            .append(right)
    }

    /// Applies a style to a `Doc`.
    pub fn style(self, style: Style) -> Doc {
        Doc::from(DocInner::Style(style, Box::new(self.inner)))
    }
}

impl DocInner {
    /// Flattens a document, converting every collapsible line break to the
    /// appropriate string.
    fn flatten(self) -> DocInner {
        match self {
            DocInner::Append(l, r) => {
                let l = l.flatten();
                let r = r.flatten();
                DocInner::Append(Box::new(l), Box::new(r))
            }
            DocInner::Nest(_, x) => x.flatten(),
            DocInner::Text(s) => DocInner::Text(s),
            DocInner::Line(Some(s)) => DocInner::Text(s.into()),
            DocInner::Line(None) => DocInner::Line(None),
            DocInner::Alt(x, _) => x.flatten(),
            DocInner::Style(s, d) => DocInner::Style(s, Box::new(d.flatten())),
        }
    }
}

impl From<&'static str> for Doc {
    fn from(s: &'static str) -> Doc {
        let inner = if s.contains('\n') {
            let parts = s.rsplit('\n');
            let mut doc = Doc::empty().inner;
            for s in parts {
                doc = DocInner::Append(
                    Box::new(DocInner::Append(
                        Box::new(DocInner::Text(s.into())),
                        Box::new(DocInner::Line(None)),
                    )),
                    Box::new(doc),
                );
            }
            doc
        } else {
            DocInner::Text(s.into())
        };
        Doc { inner }
    }
}

// TODO: From<AsRef<str>> once specialization is stable
impl From<String> for Doc {
    fn from(s: String) -> Doc {
        let inner = if s.contains('\n') {
            let parts = s.rsplit('\n');
            let mut doc = Doc::empty().inner;
            for s in parts {
                doc = DocInner::Append(
                    Box::new(DocInner::Append(
                        Box::new(DocInner::Text(s.to_string().into())),
                        Box::new(DocInner::Line(None)),
                    )),
                    Box::new(doc),
                );
            }
            doc
        } else {
            DocInner::Text(s.to_string().into())
        };
        Doc { inner }
    }
}

impl FromIterator<Doc> for Doc {
    fn from_iter<T: IntoIterator<Item = Doc>>(iter: T) -> Doc {
        let mut inner = Doc::empty().inner;
        for d in iter {
            inner = DocInner::Append(Box::new(inner), Box::new(d.inner));
        }
        Doc::from(inner)
    }
}
