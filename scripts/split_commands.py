#!/usr/bin/env python3
"""Split crates/app/src/commands.rs into commands/*.rs domain modules."""
from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] if False else Path(".")
src_path = Path("crates/app/src/commands.rs")
text = src_path.read_text()
lines = text.splitlines(keepends=True)

out = Path("crates/app/src/commands")
out.mkdir(exist_ok=True)

helpers = "".join(lines[1716:2210])
helpers = re.sub(r"^fn ", "pub(crate) fn ", helpers, flags=re.M)
helpers = re.sub(r"^enum ", "pub(crate) enum ", helpers, flags=re.M)
helpers = helpers.replace("enum NumSort", "pub(crate) enum NumSort")
# Avoid double pub(crate) pub
helpers = helpers.replace("pub(crate) pub fn", "pub fn")
helpers = helpers.replace("pub(crate) pub(crate)", "pub(crate)")

common = (
    "//! Shared helpers for menu command modules.\n"
    "#![allow(dead_code)]\n\n"
    "use super::{CmdResult, UiFlags};\n"
    "use crate::editor::EditorState;\n"
    "use std::path::{Path, PathBuf};\n\n"
    + helpers
)
(out / "common.rs").write_text(common)


def slice_arms(start_1: int, end_1_inclusive: int) -> str:
    return "".join(lines[start_1 - 1 : end_1_inclusive])


def make_domain(name: str, prefixes: list[str], arms: str) -> str:
    cover = " || ".join(f'cmd.starts_with("{p}")' for p in prefixes)
    # Exact IDs for help-style cmds that are not prefixes
    exact = [p for p in prefixes if not p.endswith("_") and p.startswith("IDM_")]
    if exact and all(not p.endswith("_") for p in prefixes):
        cover = " || ".join(f'cmd == "{p}" or cmd.starts_with("{p}")' for p in prefixes)
    return f"""//! {name} menu commands.
use super::common::*;
use super::{{CmdResult, UiFlags}};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {{
    {cover}
}}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {{
    if !covers(cmd) {{
        return None;
    }}
    Some(match cmd {{
{arms}
        _ => CmdResult::Stub,
    }})
}}
"""


file_arms = slice_arms(281, 365)
edit_arms = slice_arms(368, 1021)
format_arms = slice_arms(1023, 1046)
search_arms = slice_arms(1049, 1281)
view_arms = slice_arms(1283, 1489)
# Move EDIT_RTL/LTR from view into edit if present
rtl = []
view_clean = []
for block in re.split(r'(?=^\s+"IDM_)', view_arms, flags=re.M):
    if "IDM_EDIT_RTL" in block or "IDM_EDIT_LTR" in block:
        rtl.append(block)
    else:
        view_clean.append(block)
view_arms = "".join(view_clean)
if rtl:
    edit_arms = edit_arms + "".join(rtl)

lang_arms = slice_arms(1492, 1540)
misc_arms = slice_arms(1542, 1642)
help_arms = slice_arms(1645, 1676)

(out / "file.rs").write_text(make_domain("File", ["IDM_FILE_"], file_arms))
(out / "edit.rs").write_text(make_domain("Edit", ["IDM_EDIT_"], edit_arms))
(out / "format.rs").write_text(make_domain("Format", ["IDM_FORMAT_"], format_arms))
(out / "search.rs").write_text(
    make_domain("Search", ["IDM_SEARCH_", "IDM_FOCUS_ON_"], search_arms)
)
(out / "view.rs").write_text(make_domain("View", ["IDM_VIEW_"], view_arms))
(out / "lang.rs").write_text(make_domain("Language", ["IDM_LANG_"], lang_arms))
(out / "misc.rs").write_text(
    make_domain(
        "Tools / Window / Settings",
        ["IDM_TOOL_", "IDM_WINDOW_", "IDM_SETTING_", "IDM_CMDLINE"],
        misc_arms,
    )
)
(out / "help.rs").write_text(
    """//! Help (?) menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;

pub fn covers(cmd: &str) -> bool {
    matches!(
        cmd,
        "IDM_ABOUT"
            | "IDM_HOMESWEETHOME"
            | "IDM_PROJECTPAGE"
            | "IDM_FORUM"
            | "IDM_ONLINEDOCUMENT"
            | "IDM_UPDATE_NPP"
            | "IDM_DEBUGINFO"
            | "IDM_CMDLINEARGUMENTS"
    )
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
"""
    + help_arms
    + """
        _ => CmdResult::Stub,
    })
}
"""
)

impl_fn = "".join(lines[49:276])
default_arm = "".join(lines[1677:1715])

mod_rs = (
    """//! Command dispatch for Notepad++ menu IDs (`IDM_*`).
//!
//! Split by menu area so agents can edit in parallel:
//! `file`, `edit`, `search`, `view`, `format`, `lang`, `misc`, `help`.

mod common;
mod file;
mod edit;
mod format;
mod search;
mod view;
mod lang;
mod misc;
mod help;

use crate::editor::EditorState;

"""
    + "".join(lines[5:47])
    + "\n\n"
    + impl_fn
    + """

/// Run a menu command. Returns whether it was implemented or only stubbed.
pub fn dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> CmdResult {
    if let Some(r) = file::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = edit::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = format::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = search::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = view::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = lang::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = misc::try_dispatch(cmd, state, ui) {
        return r;
    }
    if let Some(r) = help::try_dispatch(cmd, state, ui) {
        return r;
    }
    // Fallback: language / encoding catch-alls (same as before).
    match cmd {
"""
    + default_arm
    + """
    }
}

pub use common::{
    coming_soon_blurb, coming_soon_for, feature_name_from_cmd, paste_over_bookmarked_lines,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_ready_commands() {
        assert!(is_implemented("IDM_FILE_NEW"));
        assert!(is_implemented("IDM_ABOUT"));
        assert!(is_implemented("IDM_LANG_RUST"));
        assert!(is_implemented("IDM_LANG_FOO"));
        assert!(!is_implemented("IDM_SETTING_PLUGINADM"));
    }
}
"""
)

(out / "mod.rs").write_text(mod_rs)
src_path.unlink()
print("ok", sorted(p.name for p in out.iterdir()))
for p in sorted(out.iterdir()):
    print(f"  {p.name}: {len(p.read_text().splitlines())} lines")
