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
