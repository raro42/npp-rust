# What’s missing (after stub sweep)

Date: 2026-08-29

## Stubs (Coming Soon dialog)

**None.** All 478 export menu IDs are marked implemented (teal). No grey “Coming Soon” stubs remain.

## Honest partials (work, but not full Notepad++)

About **14** handlers still say the limit in the status line or use a stand-in, including:

| Area | Gap |
|------|-----|
| View | No second pane — sync / other view / project panels open doc list or toggle a flag |
| Edit | RTL noted; layout stays LTR |
| Encoding | ANSI choice does not convert code page; memory stays UTF-8 |
| Search | Change History = dirty-tab stand-in (no per-edit marks) |
| Settings | Import plugin/theme = open folder only |
| File | Pins exist in model; no pin UI yet → Close All but Pinned often does nothing |

## Larger product gaps (not menu stubs)

- Full Preferences (tabs, margins, multi-language UI, …)
- Real dual view + project panels
- Scintilla-style change history marks
- Pin toggle in the tab UI
- CLI path args / flags (Help tab documents the limit)
- LSP / rich call tips
- True ANSI / multi-encoding load-save

Inventory: `docs/menu-todo.md`.
