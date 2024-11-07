#[macro_export]
macro_rules! fatal_assert {
    ($($arg:tt)+) => {{
        error!($($arg)+);
        panic!($($arg)+);

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
macro_rules! fatal_unwrap__ {
    ($e:expr, $str:expr) => {
        $e.unwrap_or_else(|_| {
            fatal_assert!($str);
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
