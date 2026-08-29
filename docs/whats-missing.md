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
- File compare / diff (2-way or 3-way) with sync scroll — **not started**. Upstream Notepad++ also has **no built-in compare**; users install a plugin (Compare / ComparePlus). Core N++ does ship dual view + sync scroll. `IDM_LANG_DIFF` here is only syntax highlight for `.diff` files.
- Scintilla-style change history marks
- Pin toggle in the tab UI
- Rich CLI flags (only `-h`/`--help` + open existing path args today)
- LSP / rich call tips
- True ANSI / multi-encoding load-save

Inventory: `docs/menu-todo.md`.
