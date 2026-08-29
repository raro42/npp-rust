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
| Placeholder / status-only | 0 | Partial or “not yet” |
| Menu IDs in export | 478 | From `npp_menu.json` |

## Preferences

`Settings → Preferences...` opens a small Preferences window.

Today it covers:

- When opening `*.log` files (Ask / Always / Never) → `npp-rs/settings.json`
- Editor font size (persisted in `AppSettings.font_size`)
- Show line numbers (persisted in `AppSettings.show_line_numbers`)

It is **not** a full Notepad++ Preferences clone (tabs, margins, multi-language UI, …).

## Remaining notes (not open stubs)

### Search — Progress note (2026-08-29)

Cleared: Find characters in range (Find text = `ascii` / `non-ascii` / `start-end`), Find in Files (cwd scan → results tab).

Change History MVP: `changed_unsaved` (amber) / `changed_saved` (green). Next/Prev jump marks. Clear empties both sets. Save promotes unsaved→saved. Edits remap line indices via `prepare_edit` / `apply_line_snap`.

### View — Dual view (2026-08-29)

- Right-hand **Other view** pane is a writable editor for its tab
- Click a pane to focus typing; sync H/V / zoom sync share scroll / font size
- Move / Switch / Clone to other view wire that pane
- Project panels 1–3 still open the document list

### Edit (0)

_(cleared — column editor + call tips; Character Panel egui grid inserts at caret)_

### Help (0)

- [x] `IDM_CMDLINEARGUMENTS` — ?/Command Line Arguments... (read-only tab; no CLI flags yet; README link)

## Cleared this batch (2026-08-29)

Paint path hides `hidden_lines` and paints `style_marks` / bookmark ticks. Menu Cut/Copy/Paste use the session clipboard. Find-mark jump uses bookmarks.

- Search Find in Files / char range / change-history marks MVP (5)
- Search style mark / jump / clear / copy-styled (34)
- View fold / unfold / hide lines / open in browser / new instance (27)
- Edit Cut / Copy / Paste / paste-special / autocomplete / multi-select / system read-only / open folder on selection (20)
- Tools hash suite (12)
- Help URLs + Changelog (6)
- Preferences: log-tail + **persisted font size** + **line numbers** (2)
- File delete / print / session load-save (5)
- File Close All but Pinned + `IDM_PINTAB` + **tab UI pin** (`[P]` marker, pin button, context menu) (1)
- View dual-view MVP + sync H/V / zoom sync honest status (3)
- View switch / move / clone to other view (secondary pane); project panels 1–3: doc list (5)
- View text direction LTR/RTL: honest status only (layout stays LTR; no doc flag) (2)
- Encoding ANSI / UTF-8 / UTF-8-BOM: per-tab save encoding; ANSI → Windows-1252 (lossy) (see `docs/encoding.md`) (1)
- Edit column editor: insert clipboard text (or 0,1,2…) at caret column on selected lines / multi-carets; tip documents that path (2)
- Edit function call tip: status tip from word under caret + cycle call-site snippets in file (no LSP) (3)
- View Document Map: density strip; click/drag sets `scroll_line` (`UiFlags.show_doc_map`) (1)
- View Function List: fn/class-like line list; click jumps caret (`UiFlags.show_func_list`) (1)
- Edit Character Panel: egui grid inserts basic/unicode at caret (`UiFlags.show_char_panel`) (1)
- Settings Shortcut Mapper / Style Config tabs; Import plugins+themes open folders + list tabs; Plugin Admin lists builtins (5)
- Run… (rfd pick or shell-here) + validate shortcuts.xml absence (2)
