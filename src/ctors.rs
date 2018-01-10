use std::iter::FromIterator;

use Doc;

impl Doc {
    /// Appends one `Doc` to another. Equivalent to the `Append` constructor.
    pub fn append(self, right: Doc) -> Doc {
        Doc::Append(Box::new(self), Box::new(right))
    }
}

impl<S: AsRef<str>> From<S> for Doc {
    fn from(s: S) -> Doc {
        let s = s.as_ref();
        if s.contains('\n') {
            let parts = s.rsplit('\n');
            let mut doc = Doc::Nil;
            for s in parts {
                doc = Doc::Append(
                    Box::new(Doc::Append(
                        Box::new(Doc::Text(s.to_string())),
                        Box::new(Doc::Line),
                    )),
                    Box::new(doc),
                );
            }
            doc
        } else {
            Doc::Text(s.to_string())
        }
    }
}

impl FromIterator<Doc> for Doc {
    fn from_iter<T: IntoIterator<Item = Doc>>(iter: T) -> Doc {
        let mut doc = Doc::Nil;
        for d in iter {
            doc = Doc::Append(Box::new(doc), Box::new(d));
        }
        doc
    }
}

#[test]
fn from_str() {
    let d = Doc::from("asdf");
    assert_eq!(d, Doc::Text("asdf".to_string()));
}

#[test]
fn from_str_newlines() {
    let d1 = Doc::from("asdf\nqwerty\n\nzxcvbn");
    let d2 = Doc::Append(
        Box::new(Doc::Append(
            Box::new(Doc::Text("asdf".to_string())),
            Box::new(Doc::Line),
        )),
        Box::new(Doc::Append(
            Box::new(Doc::Append(
                Box::new(Doc::Text("qwerty".to_string())),
                Box::new(Doc::Line),
            )),
            Box::new(Doc::Append(
                Box::new(Doc::Append(
                    Box::new(Doc::Text("".to_string())),
                    Box::new(Doc::Line),
                )),
                Box::new(Doc::Append(
                    Box::new(Doc::Append(
                        Box::new(Doc::Text("zxcvbn".to_string())),
                        Box::new(Doc::Line),
                    )),
                    Box::new(Doc::Nil),
                )),
            )),
        )),
    );
    assert_eq!(d1, d2);
}
