# What’s missing (after stub sweep)

Date: 2026-08-29

## Stubs (Coming Soon dialog)

**None.** All 478 export menu IDs are marked implemented (teal). No grey “Coming Soon” stubs remain.

## Honest partials (work, but not full Notepad++)

About **10** handlers still say the limit in the status line or use a stand-in, including:

| Area | Gap |
|------|-----|
| View | Dual view: both panes writable; click focuses a pane; Menu Edit uses the focused pane. Project panel lists workspace files (MVP, not N++ projects) |
| Edit | RTL/LTR mirrors editor line anchors + status cue; full bidi / UI chrome mirror still open |
| Encoding | ANSI / UTF-8 / UTF-8-BOM set per-tab save encoding (Windows-1252 on ANSI). Unmapped chars become `?` on ANSI save |
| Search | Change History: amber unsaved / green saved ticks; line remap on insert/delete (MVP) |
| Settings | Themes: MVP apply (egui + editor colours); N++ XML / token colours still open; plugins: builtins listed, drop-in not loaded |
| File | Pin works from tab chrome (`[P]` / context / button) and `IDM_PINTAB` — Close All but Pinned keeps pinned tabs |
| View | Tab drag-reorder on the tab bar; Move Tab Forward / Backward still available |

## Larger product gaps (not menu stubs)

- Full Preferences (tabs, margins, multi-language UI, …). Partial: log-tail, font size, line numbers, tab width, word wrap, status toggles → `npp-rs/settings.json`
- Project panels: MVP folder file list (not full N++ project files). Dual edit: both panes writable; Menu Edit uses the focused pane
- File compare / diff — **2-way MVP** with re-diff after edit (~200 ms debounce). 3-way and char-level diff still open. Upstream N++ uses a plugin for compare.
- Change history: MVP unsaved/saved colours + line remap shipped; full Scintilla parity still open
- Rich CLI flags — **shipped MVP**: `-h`, `-V`, `-n`/`--line`, `-ro`, path args
- LSP (call tips: word under caret + nearby `fn`/`def` line snippets in file only)

Inventory: `docs/menu-todo.md`.
