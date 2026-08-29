# Menu implementation todo

Date: 2026-08-29

## Read this first

**Teal ≠ full feature.**

`is_implemented` (teal menu colour) only means: no “Coming Soon” dialog.
Many teal items only set a status-bar message. That inflated “Ready 477 / stubs 0”.

| Class | Count | Meaning |
|-------|------:|---------|
| Explicit handlers | 343 | Match arms in `commands/*.rs` |
| Useful behaviour | 338 | Change buffer or UI beyond a note |
| Placeholder / status-only | 1 | Partial or “not yet” |
| Menu IDs in export | 478 | From `npp_menu.json` |

## Preferences

`Settings → Preferences...` opens a small Preferences window.

Today it covers:

- When opening `*.log` files (Ask / Always / Never) → `npp-rs/settings.json`
- Editor font size (session only for now)

It is **not** a full Notepad++ Preferences clone (tabs, margins, multi-language UI, …).

## Placeholder / status-only (open work)

### Search — Progress note (2026-08-29)

Cleared: Find characters in range (Find text = `ascii` / `non-ascii` / `start-end`), Find in Files (cwd scan → results tab), Change History stand-in (next/prev dirty tab; clear selection).

Real Scintilla-style change marks need `editor.rs`. Do not expect per-edit marks from Search alone.

### View (0)

_(cleared — Document Map + Function List open egui panels)_

### Edit (0)

_(cleared — column editor + call tips; Character Panel egui grid inserts at caret)_

### Help (0)

- [x] `IDM_CMDLINEARGUMENTS` — ?/Command Line Arguments... (read-only tab; no CLI flags yet; README link)

## Cleared this batch (2026-08-29)

Paint path hides `hidden_lines` and paints `style_marks` / bookmark ticks. Menu Cut/Copy/Paste use the session clipboard. Find-mark jump uses bookmarks.

- Search Find in Files / char range / change-history stand-in (5)
- Search style mark / jump / clear / copy-styled (34)
- View fold / unfold / hide lines / open in browser / new instance (27)
- Edit Cut / Copy / Paste / paste-special / autocomplete / multi-select / system read-only / open folder on selection (20)
- Tools hash suite (12)
- Help URLs + Changelog (6)
- Preferences + Open Plugins Folder (2)
- File delete / print / session load-save (5)
- File Close All but Pinned (`Document.pinned`; no pin UI yet → status “Nothing pinned — closed none”) (1)
- View sync H/V / zoom sync: session toggle + honest single-view status (3)
- View switch / move to other view + project panels 1–3: open document list (5)
- View text direction LTR/RTL: honest status only (layout stays LTR; no doc flag) (2)
- Encoding ANSI / UTF-8 / UTF-8-BOM: strip or keep leading U+FEFF for save; memory stays UTF-8; no ANSI convert (3)
- Edit column editor: insert clipboard text (or 0,1,2…) at caret column on selected lines / multi-carets; tip documents that path (2)
- Edit function call tip: status tip from word under caret + cycle call-site snippets in file (no LSP) (3)
- View Document Map: density strip; click/drag sets `scroll_line` (`UiFlags.show_doc_map`) (1)
- View Function List: fn/class-like line list; click jumps caret (`UiFlags.show_func_list`) (1)
- Edit Character Panel: egui grid inserts basic/unicode at caret (`UiFlags.show_char_panel`) (1)
- Settings Shortcut Mapper / Style Config tabs; Import plugins+themes open folders; Plugin Admin list (5)
- Run… (rfd pick or shell-here) + validate shortcuts.xml absence (2)
