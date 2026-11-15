//! Function List line classifiers (language / config key matchers).
//!
//! Heavy matchers live in `func_like.body.inc` and are copied into OUT_DIR by
//! `build.rs`, then `include!`d here so `cargo fmt` never parses the multi-MB
//! bodies (Windows rustfmt overflows its ~1 MiB main stack on those files).

include!(concat!(env!("OUT_DIR"), "/func_like_body.rs"));

#[cfg(test)]
mod func_like_line_tests {
    include!(concat!(env!("OUT_DIR"), "/func_like_line_tests_body.rs"));
}
