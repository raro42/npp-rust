# Overnight product gaps

## GitHub Issues
- **Issue:** https://github.com/raro42/npp-rust/issues/6

## Problem / goal

Implement real behaviour for gaps in `docs/whats-missing.md`.
Do not invent Coming Soon stubs. Prefer small real UX over status-only notes.
Commit and push to `origin/dev` each batch. Bump version per `.cursor/rules/bump-version.mdc`.

## Priority

1. Dual-view: menu Edit uses focused pane tab — **done**
2. Encoding ANSI / Windows-1252 save — **done**
3. Change-history line remap + saved-vs-unsaved colours — **done**
4. Compare re-diff after edit — **done**
5. Preferences depth — **partial** (tab width, word wrap, status toggles in Preferences)
6. Project panels
7. CLI flags — **done** (`-V`, `-n`/`--line`, `-ro`, `--`)
8. Theme apply MVP

## Progress (2026-08-29)

Batch for tester (v0.2.3 + v0.2.4):

- Dual view: `dispatch_menu_cmd` retargets Edit/Format to `focused_edit_tab`
- Encoding: per-tab `FileEncoding`; ANSI save → Windows-1252 via `fs::write_file_with_encoding`
- Change history: amber unsaved / green saved; `LineEditSnap` remap; save promotes marks
- Compare: re-diff after edit (~200 ms debounce)
- CLI: `--version`, `--line`/`-n`, `--read-only`, `--`
- Preferences: tab width, word wrap, status lang/chars toggles

Hand off: this file is `TEST-…` (issue stays open).

## Privacy
- Do not paste private paths into GitHub.


## Handoff

- Status: complete (v0.3.0)
- Handoff: complete
