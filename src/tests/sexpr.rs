use {Doc, Sparkly};

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

impl Sparkly for SExpr {
    fn to_doc(&self) -> Doc {
        match *self {
            SExpr::Atom(ref s) => s.to_string().into(),
            SExpr::List(ref l) => Doc::space().join(l).bracket("(", ")"),
        }
    }
}

tests! {
    [atom, 80, false] SExpr::from("foo") => "foo",
    [empty, 80, false] SExpr::from(vec![]) => "()",
    [not_wrapping, 80, false]
        SExpr::from(vec![
            SExpr::from("foo"),
            SExpr::from("bar"),
            SExpr::from("baz"),
            SExpr::from("quux"),
            SExpr::from("spam"),
            SExpr::from("eggs"),
        ]) => "(foo bar baz quux spam eggs)",
    [wrapping, 10, false]
        SExpr::from(vec![
            SExpr::from("foo"),
            SExpr::from("bar"),
            SExpr::from("baz"),
            SExpr::from("quux"),
            SExpr::from("spam"),
            SExpr::from("eggs"),
        ]) => "(\n    foo\n    bar\n    baz\n    quux\n    spam\n    eggs\n)"
}
