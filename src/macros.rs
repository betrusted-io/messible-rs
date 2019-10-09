/// Macro for sending a formatted string through a Messible
#[macro_export]
macro_rules! mprint {
    ($s:expr) => {
        $crate::write_str($s);
    };
    ($($arg:tt)*) => {
        $crate::write_fmt(format_args!($($arg)*));
    };
}

/// Macro for sending a formatted string through a Messible, with a newline.
#[macro_export]
macro_rules! mprintln {
    () => {
        mprint!("\n");
    };
    ($fmt:expr) => {
        mprint!(concat!($fmt, "\n"));
    };
    ($fmt:expr, $($arg:tt)*) => {
        mprint!(concat!($fmt, "\n"), $($arg)*);
    };
}