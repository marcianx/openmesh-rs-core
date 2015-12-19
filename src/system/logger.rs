
/// Writes to stdout.
#[macro_export]
macro_rules! omout {
    ($fmt:expr) => (println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}

/// Writes to non-error log.
#[macro_export]
macro_rules! omlog {
    ($fmt:expr) => (println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}

/// Writes to error log.
#[macro_export]
macro_rules! omerr {
    ($fmt:expr) => (::std::result::Result::ok(writeln!(::std::io::stderr(), $fmt)));
    ($fmt:expr, $($arg:tt)*) => (::std::result::Result::ok(writeln!(::std::io::stderr(), $fmt, $($arg)*)));
}
