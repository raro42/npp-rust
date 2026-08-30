//! Embed short git revision at compile time for the status bar.

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

fn main() {
    // Workspace root is two levels above crates/app.
    let short =
        git(&["-C", "../..", "rev-parse", "--short=7", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let full = git(&["-C", "../..", "rev-parse", "HEAD"]).unwrap_or_else(|| short.clone());
    println!("cargo:rustc-env=NPP_GIT_HASH={short}");
    println!("cargo:rustc-env=NPP_GIT_HASH_FULL={full}");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/heads/main");
}
