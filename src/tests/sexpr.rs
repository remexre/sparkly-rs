use proptest::prelude::*;

use {Doc, Sparkly};

#[derive(Clone, Debug)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

impl_Display_for_Sparkly!(SExpr);

impl Sparkly for SExpr {
    fn to_doc(&self) -> Doc {
        match *self {
            SExpr::Atom(ref s) => s.to_string().into(),
            SExpr::List(ref l) => Doc::space().join(l).bracket("(", ")"),
        }
    }
}

macro_rules! sexpr {
    ($atom:ident) => {
        SExpr::Atom(stringify!($atom).to_string())
    };
    (($($atom:ident)*)) => {
        SExpr::List(vec![
            $(sexpr!($atom)),*
        ])
    };
}

fn arb_sexpr() -> BoxedStrategy<SExpr> {
    let leaf = prop_oneof!["[a-zA-Z]+".prop_map(SExpr::Atom)];
    leaf.prop_recursive(4, 256, 16, |inner| {
        prop_oneof![prop::collection::vec(inner, 0..8).prop_map(SExpr::List)].boxed()
    }).boxed()
}

proptest! {
    #[test]
    fn display_works(ref sexpr in arb_sexpr()) {
        let doc = sexpr.to_doc();
        let p = doc.display_opts(80, false);
        assert_eq!(p.to_string(), sexpr.to_string());
    }
}

tests! {
    [atom, 80, false] sexpr![foo] => "foo",
    [empty, 80, false] sexpr![()] => "()",
    [not_wrapping, 80, false]
        sexpr![(foo bar baz quux spam eggs)] =>
        "(foo bar baz quux spam eggs)",
    [wrapping, 10, false]
        sexpr![(foo bar baz quux spam eggs)] =>
        "(\n    foo\n    bar\n    baz\n    quux\n    spam\n    eggs\n)"
}
