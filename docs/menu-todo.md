# Menu implementation todo

Date: 2026-08-29

## Read this first

**Teal ≠ full feature.**

`is_implemented` (teal menu colour) only means: no “Coming Soon” dialog.
Many teal items only set a status-bar message. That inflated “Ready 477 / stubs 0”.

| Class | Count | Meaning |
|-------|------:|---------|
| Explicit handlers | 343 | Match arms in `commands/*.rs` |
| Useful behaviour | 321 | Change buffer or UI beyond a note |
| Placeholder / status-only | 17 | Partial or “not yet” |
| Menu IDs in export | 478 | From `npp_menu.json` |

## Preferences

`Settings → Preferences...` opens a small Preferences window.

Today it covers:

- When opening `*.log` files (Ask / Always / Never) → `npp-rs/settings.json`
- Editor font size (session only for now)

It is **not** a full Notepad++ Preferences clone (tabs, margins, multi-language UI, …).

## Placeholder / status-only (open work)

### Search (5)

- [ ] `IDM_SEARCH_CLEAR_CHANGE_HISTORY` — Search/Change History/Clear Change History
- [ ] `IDM_SEARCH_CHANGED_NEXT` — Search/Change History/Go to Next Change
- [ ] `IDM_SEARCH_CHANGED_PREV` — Search/Change History/Go to Previous Change
- [ ] `IDM_SEARCH_FINDCHARINRANGE` — Search/Find characters in range...
- [ ] `IDM_SEARCH_FINDINFILES` — Search/Find in Files...

### View (0)

_(cleared — Document Map + Function List open egui panels)_

### Edit (5)

- [ ] `IDM_EDIT_FUNCCALLTIP` — Edit/Auto-Completion/Function Parameters Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_NEXT` — Edit/Auto-Completion/Function Parameters Next Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_PREVIOUS` — Edit/Auto-Completion/Function Parameters Previous Hint
- [ ] `IDM_EDIT_COLUMNMODE` — Edit/Column Editor...
- [ ] `IDM_EDIT_COLUMNMODETIP` — Edit/Column Mode...

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
- Settings Shortcut Mapper / Style Config tabs; Import plugins+themes open folders; Plugin Admin list (5)
- Run… (rfd pick or shell-here) + validate shortcuts.xml absence (2)
