use {Doc, Pretty};

macro_rules! assert_pretty_eq {
    ($([$n:expr, $c:expr] $l:expr => $r:expr),*) => {
        $({
            let l = format!("{}", $l.as_pretty().display($n, $c));
            assert_eq!(&l, $r)
        })*
    };
}

mod sexpr;

#[test]
fn misc() {
    assert_pretty_eq! {
        [80, false]
            Doc::from("asdf")
                .append(Doc::Nest(2, Box::new(Doc::Line)))
                .append(Doc::from("qwerty"))
                .append(Doc::Nest(4, Box::new(Doc::Line)))
                .append(Doc::from("zxcvbn")) =>
            "asdf\n  qwerty\n    zxcvbn"
    }
}
