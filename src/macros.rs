#[macro_export]
macro_rules! fatal_assert {
    ($($arg:tt)+) => {{
        error!($($arg)+);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! fatal_unwrap_e {
    ($e:expr, $str:expr) => {
        $e.unwrap_or_else(|e| {
            fatal_assert!($str, e);
        })
    };
}
#[macro_export]
macro_rules! fatal_unwrap {
    ($e:expr, $str:expr) => {
        $e.unwrap_or_else(|| {
            fatal_assert!($str);
        })
    };
}
