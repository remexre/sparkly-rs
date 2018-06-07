/// Creates a Display impl based on the Sparkly impl for the type.
///
/// The impl will assume a width of 80 characters.
///
/// Generic bounds are supported with the following syntax:
///
/// ```rust
/// # #[macro_use]
/// # extern crate sparkly;
/// # use std::fmt::Display;
/// # use std::marker::PhantomData;
/// # struct Type<T, U>(PhantomData<fn(T) -> U>);
/// impl_Display_for_Sparkly!((T, U: Display) Type<T, U>);
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! impl_Display_for_Sparkly {
    (($($bound:tt)*) $ty:ty) => {
        impl<$($bound)*> ::std::fmt::Display for $ty
        where
            $ty: $crate::Sparkly,
        {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(fmt, "{}", $crate::Sparkly::to_doc(self).display_opts(80, false))
            }
        }
    };
    ($ty:ty) => {
        impl_Display_for_Sparkly!(() $ty);
    };
}
