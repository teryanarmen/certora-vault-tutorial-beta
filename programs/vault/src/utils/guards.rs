#[cfg(not(feature = "certora"))]
mod inner {
    #[rustfmt::skip]
    macro_rules! impl_bin_require {
        ($name: ident, $pred: tt, $dollar: tt) => {
            #[macro_export]
            macro_rules! $name {
                    ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
                        if $lhs $pred $rhs { } else { return Err(VaultError::GuardFail); }
                    }};
                }
            pub use $name;
        };
    }
    pub(crate) use impl_bin_require;
}

#[cfg(feature = "certora")]
mod inner {
    #[rustfmt::skip]
    macro_rules! impl_bin_require {
        ($name: ident, $pred: tt, $dollar: tt) => {
            #[macro_export]
            macro_rules! $name {
                    ($lhs: expr, $rhs: expr $dollar(, $desc: literal)? ) => {{
                        if $lhs $pred $rhs { } else { panic!(); }
                    }};
                }
            pub use $name;
        };
    }
    pub(crate) use impl_bin_require;
}

pub(crate) use inner::*;
impl_bin_require!(require_gt, >, $);
impl_bin_require!(require_ge, >=, $);
impl_bin_require!(require_eq, ==, $);
impl_bin_require!(require_lt, <, $);
impl_bin_require!(require_le, <=, $);
