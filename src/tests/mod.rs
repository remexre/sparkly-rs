use {Doc, DocInner, Sparkly};

macro_rules! tests {
    ($([$name:ident, $n:expr, $c:expr] $l:expr => $r:expr),*) => {
        $(#[test] fn $name() {
            let l = format!("{}", $l.to_doc().display_opts($n, $c));
            assert_eq!(&l, $r)
        })*
    };
}

mod impl_bounds;
mod sexpr;

tests! {
    [misc_1, 80, false]
        Doc::from("asdf")
            .append(Doc::from(DocInner::Nest(2, Box::new(DocInner::Line(None)))))
            .append(Doc::from("qwerty"))
            .append(Doc::from(DocInner::Nest(4, Box::new(DocInner::Line(None)))))
            .append(Doc::from("zxcvbn")) =>
        "asdf\n  qwerty\n    zxcvbn"
}
