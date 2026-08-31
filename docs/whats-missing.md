# What’s missing (after stub sweep)

Date: 2026-08-29  
**Checkpoint:** Issues #1–#6 closed. **v0.3.1** shipped issue #7 P0. Next: `docs/next-gaps.md` (P1).

## Stubs (Coming Soon dialog)

**None.** All 478 export menu IDs are marked implemented (teal). No grey “Coming Soon” stubs remain.

## Honest partials (work, but not full Notepad++)

Handlers that work but stay shallower than upstream N++:

| Area | Gap |
|------|-----|
| View | Dual view: both panes writable; focused-pane Edit. Project panel = MVP folder list (not N++ projects) |
| Edit | RTL/LTR: editor line anchors + status cue; full bidi / UI chrome mirror still open |
| Encoding | ANSI / UTF-8 / UTF-8-BOM / UTF-16 LE-BE (BOM) per-tab save. Unmapped chars → `?` on ANSI save. No BOM-less UTF-16 detect yet |
| Search | Change History: amber/green gutter bars + wash; undo remap; CHG status. Find has case/word + match count |
| Settings | Themes: JSON tokens + chrome; N++ XML subset (GlobalStyles + one lexer); full stylers parity open; plugins: builtins listed, no drop-in load. Preferences deeper in v0.3.1 |
| File | Pin from tab chrome + `IDM_PINTAB`; Close All but Pinned keeps pinned tabs; opt-in session restore |
| View | Tab drag-reorder; Move Tab Forward / Backward still available |

## Larger product gaps (not menu stubs)

- Full Preferences (multi-language UI, …). More keys in `npp-rs/settings.json` (see `docs/preferences-p0.md`)
- Project panels: MVP folder list only
- File compare: **2-way** + re-diff + ignore-whitespace. 3-way / char-level still open
- Change history: bar marks + undo remap; full Scintilla reverted/indicator parity open
- CLI MVP: `-h`, `-V`, `-n`/`--line`, `-ro`, path args
- LSP (call tips: in-file snippets only)

**Ranked backlog:** `docs/next-gaps.md`.  
**Inventory:** `docs/menu-todo.md`.
