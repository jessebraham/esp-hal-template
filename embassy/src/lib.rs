#![no_std]

/// Convert a `T` to a `&'static mut T`.
///
/// The macro declares a `static StaticCell` and then initializes it when run,
/// returning the `&'static mut`. Therefore, each instance can only be run once.
/// Next runs will panic. The `static` can additionally be decorated with
/// attributes, such as `#[link_section]`, `#[used]`, et al.
#[macro_export]
macro_rules! make_static {
    ( $t:ty, $val:expr) => ($crate::make_static!($t, $val, ));
    ( $t:ty, $val:expr, $(#[$m:meta])* ) => {{
        $(#[$m])*
        static STATIC_CELL: ::static_cell::StaticCell<$t> = ::static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));

        x
    }};
}
