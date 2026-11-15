//! Embed short git revision at compile time for the status bar.
//! Copy Function List classifier bodies into OUT_DIR so `cargo fmt` never
//! parses the multi-MB `.inc` sources (Windows rustfmt stack overflow).

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8(out.stdout).ok()?;
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn copy_inc(manifest_dir: &str, out_dir: &str, src_name: &str, dst_name: &str) {
    let src = PathBuf::from(manifest_dir).join("src").join(src_name);
    let dst = PathBuf::from(out_dir).join(dst_name);
    fs::copy(&src, &dst).unwrap_or_else(|e| {
        panic!("copy {} -> {}: {e}", src.display(), dst.display());
    });
    println!("cargo:rerun-if-changed={}", src.display());
}

fn main() {
    // Workspace root is two levels above crates/app.
    let short =
        git(&["-C", "../..", "rev-parse", "--short=7", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let full = git(&["-C", "../..", "rev-parse", "HEAD"]).unwrap_or_else(|| short.clone());
    println!("cargo:rustc-env=NPP_GIT_HASH={short}");
    println!("cargo:rustc-env=NPP_GIT_HASH_FULL={full}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads/main");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    copy_inc(
        &manifest_dir,
        &out_dir,
        "func_like.body.inc",
        "func_like_body.rs",
    );
    copy_inc(
        &manifest_dir,
        &out_dir,
        "func_like_line_tests.body.inc",
        "func_like_line_tests_body.rs",
    );
}
