use std::iter::once;

use {Doc, Pretty};

#[derive(Clone, Debug)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

impl<'a> From<&'a str> for SExpr {
    fn from(s: &'a str) -> SExpr {
        SExpr::Atom(s.to_string())
    }
}

impl From<Vec<SExpr>> for SExpr {
    fn from(v: Vec<SExpr>) -> SExpr {
        SExpr::List(v)
    }
}

impl Pretty for SExpr {
    fn as_pretty(&self) -> Doc {
        match *self {
            SExpr::Atom(ref s) => s.into(),
            SExpr::List(ref l) => once(Doc::from("("))
                .chain(l.iter().map(SExpr::as_pretty))
                .chain(once(Doc::from(")")))
                .collect(),
        }
    }
}

#[test]
fn atom() {
    assert_pretty_eq! {
        [80, false] SExpr::from("foo") => "foo"
    }
}
