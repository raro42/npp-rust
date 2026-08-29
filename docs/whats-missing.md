# What’s missing (after stub sweep)

Date: 2026-08-29

## Stubs (Coming Soon dialog)

**None.** All 478 export menu IDs are marked implemented (teal). No grey “Coming Soon” stubs remain.

## Honest partials (work, but not full Notepad++)

About **12** handlers still say the limit in the status line or use a stand-in, including:

| Area | Gap |
|------|-----|
| View | Dual view MVP: read-only secondary pane + switch/move/clone/sync. Not two full editors. Project panels still open the doc list |
| Edit | RTL/LTR toggles session flag; layout stays LTR until ui_paint |
| Encoding | ANSI menu strips BOM; save stays UTF-8 (no BOM). No per-tab code-page re-encode on save. Load falls back to Windows-1252 when bytes are not UTF-8 |
| Search | Change History next/prev/clear use per-line marks (MVP; no Scintilla save-vs-session colours) |
| Settings | Themes: no apply API — Import lists `themes/` + opens folder; plugins: builtins listed, drop-in not loaded |
| File | Pin works from tab chrome (`[P]` / context / button) and `IDM_PINTAB` — Close All but Pinned keeps pinned tabs |

## Larger product gaps (not menu stubs)

- Full Preferences (tabs, margins, multi-language UI, …). Partial: log-tail, font size, line numbers → `npp-rs/settings.json`
- Full dual edit (both panes writable) + project panels
- File compare / diff (2-way or 3-way) with sync scroll — **not started**. Upstream Notepad++ also has **no built-in compare**; users install a plugin (Compare / ComparePlus). Core N++ does ship dual view + sync scroll. `IDM_LANG_DIFF` here is only syntax highlight for `.diff` files.
- Change history: line remap on insert/delete; saved-vs-unsaved colours (MVP marks exist)
- Rich CLI flags (only `-h`/`--help` + open existing path args today)
- LSP (call tips: word under caret + nearby `fn`/`def` line snippets in file only)
- Multi-encoding save as Windows-1252 from the ANSI menu (API exists in `fs`; save path still uses UTF-8 / UTF-8-BOM from buffer)

Inventory: `docs/menu-todo.md`.
