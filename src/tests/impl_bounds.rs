use std::fmt::Display;

use proptest::prelude::*;

use {Doc, Sparkly};

#[derive(Clone, Debug)]
enum Foo<T> {
    Yup(T),
    Nope,
}

impl<T: Display> Sparkly for Foo<T> {
    fn to_doc(&self) -> Doc {
        match *self {
            Foo::Yup(ref t) => Doc::from(t.to_string()),
            Foo::Nope => Doc::from("nope."),
        }
    }
}

impl_Display_for_Sparkly!((T: Display) Foo<T>);

fn arb_foo() -> BoxedStrategy<Foo<i32>> {
    prop_oneof![(0..1000).prop_map(Foo::Yup), Just(Foo::Nope)].boxed()
}

proptest! {
    #[test]
    fn display_works(ref foo in arb_foo()) {
        let doc = foo.to_doc();
        let p = doc.display_opts(80, false);
        assert_eq!(p.to_string(), foo.to_string());
    }
}
