//! Formatting of a simple ML-like language.

#![feature(test)]

#[macro_use]
extern crate sparkly;
extern crate test;

use std::iter::once;

use sparkly::{Doc, Sparkly};

#[derive(Clone, Debug)]
enum Expr {
    Call(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Lam(Vec<&'static str>, Box<Expr>),
    Let(bool, &'static str, Box<Expr>, Box<Expr>),
    Variable(&'static str),
}

impl_Display_for_Sparkly!(Expr);

impl Sparkly for Expr {
    fn to_doc(&self) -> Doc {
        match *self {
            Expr::Call(ref func, ref args) => Doc::join(
                Doc::space(),
                once(func.to_doc()).chain(args.iter().map(|arg| arg.to_doc())),
            ).bracket("(", ")"),
            Expr::If(ref c, ref t, ref e) => Doc::from("if")
                .append(Doc::space())
                .append(c.to_doc())
                .append(Doc::space())
                .append("then".into())
                .append(Doc::space())
                .append(t.to_doc())
                .append(Doc::space())
                .append("else".into())
                .append(Doc::space())
                .append(e.to_doc()),
            Expr::Lam(ref args, ref body) => Doc::from("\\")
                .append(Doc::space().join(args.iter().map(|&arg| Doc::from(arg))))
                .append(".".into())
                .append(Doc::space())
                .append(body.to_doc())
                .bracket("(", ")"),
            Expr::Let(rec, name, ref bound, ref body) => if rec {
                Doc::from("let").append(Doc::nbsp()).append("rec".into())
            } else {
                Doc::from("let")
            }.append(
                Doc::space()
                    .append(name.into())
                    .append(Doc::space())
                    .group(),
            )
                .append("=".into())
                .append(
                    Doc::nest(Doc::space().append(bound.to_doc()), 2)
                        .append(Doc::space())
                        .append("in".into())
                        .group(),
                )
                .append(Doc::nest(Doc::line().append(body.to_doc()), 2)),
            Expr::Variable(s) => s.into(),
        }
    }
}

macro_rules! expr {
    ((call $f:tt $($a:tt)*)) => {
        Expr::Call(Box::new(expr!($f)), vec![$(expr!($a)),*])
    };
    ((if $c:tt $t:tt)) => {
        Expr::If(Box::new(expr!($c)), Box::new(expr!($t)), None)
    };
    ((if $c:tt $t:tt $e:tt)) => {
        Expr::If(
            Box::new(expr!($c)),
            Box::new(expr!($t)),
            Box::new(expr!($e)),
        )
    };
    ((lam $($n:ident)* . $e:tt)) => {
        Expr::Lam(vec![$(stringify!($n)),*], Box::new(expr!($e)))
    };
    ((let $n:ident $v:tt $e:tt)) => {
        Expr::Let(false, stringify!($n), Box::new(expr!($v)), Box::new(expr!($e)))
    };
    ((letrec $n:ident $v:tt $e:tt)) => {
        Expr::Let(true, stringify!($n), Box::new(expr!($v)), Box::new(expr!($e)))
    };
    ($id:ident) => {
        Expr::Variable(stringify!($id))
    };
}

mod small_ast {
    use super::*;
    use test::Bencher;

    fn make() -> Expr {
        expr! {
            (let s (lam x y z . (call x y (call y z)))
              (let k (lam x y . x)
                (let i (lam x . x)
                  (call i s k k i))))
        }
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let ex = make();
        b.iter(|| ex.to_doc().display_opts(80, false).to_string())
    }

    #[test]
    fn correctness() {
        let ex = make();
        assert_eq!(
            ex.to_doc().display_opts(80, false).to_string(),
            concat!(
                "let s = (\\x y z. (x y (y z))) in\n",
                "  let k = (\\x y. x) in\n",
                "    let i = (\\x. x) in\n",
                "      (i s k k i)"
            )
        );
    }
}

mod medium_ast {
    use super::*;
    use test::Bencher;

    fn make() -> Expr {
        expr! {
            (let s (lam x y z . (call x y (call y z)))
              (let k (lam x y . x)
                (let i (lam x . x)
                  (letrec even (lam x . (if (call eq x zero)
                                            true
                                            (call not (call even (call minus x one)))))
                    (call i (call even five))))))
        }
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let ex = make();
        b.iter(|| ex.to_doc().display_opts(80, false).to_string())
    }

    #[test]
    fn correctness() {
        let ex = make();
        assert_eq!(
            ex.to_doc().display_opts(80, false).to_string(),
            concat!(
                "let s = (\\x y z. (x y (y z))) in\n",
                "  let k = (\\x y. x) in\n",
                "    let i = (\\x. x) in\n",
                "      let rec even =\n",
                "        (\\x. if (eq x zero) then true else (not (even (minus x one))))\n",
                "      in\n",
                "        (i (even five))"
            )
        );
    }
}
