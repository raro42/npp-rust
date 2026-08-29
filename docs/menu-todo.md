# Menu implementation todo

Date: 2026-08-29

## Read this first

**Teal ≠ full feature.**

`is_implemented` (teal menu colour) only means: no “Coming Soon” dialog.
Many teal items only set a status-bar message. That inflated “Ready 477 / stubs 0”.

| Class | Count | Meaning |
|-------|------:|---------|
| Explicit handlers | 343 | Match arms in `commands/*.rs` |
| Useful behaviour | 308 | Change buffer or UI beyond a note |
| Placeholder / status-only | 35 | Partial or “not yet” |
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

### View (12)

- [ ] `IDM_VIEW_DOC_MAP` — View/Document Map
- [ ] `IDM_VIEW_SWITCHTO_OTHER_VIEW` — View/Focus on Another View
- [ ] `IDM_VIEW_FUNC_LIST` — View/Function List
- [ ] `IDM_VIEW_GOTO_ANOTHER_VIEW` — View/Move/Clone Current Document/Move to Other View
- [ ] `IDM_VIEW_PROJECT_PANEL_1` — View/Project Panels/Project Panel 1
- [ ] `IDM_VIEW_PROJECT_PANEL_2` — View/Project Panels/Project Panel 2
- [ ] `IDM_VIEW_PROJECT_PANEL_3` — View/Project Panels/Project Panel 3
- [ ] `IDM_VIEW_SYNSCROLLH` — View/Synchronize Horizontal Scrolling
- [ ] `IDM_VIEW_SYNSCROLLV` — View/Synchronize Vertical Scrolling
- [ ] `IDM_EDIT_LTR` — View/Text Direction LTR
- [ ] `IDM_EDIT_RTL` — View/Text Direction RTL
- [ ] `IDM_VIEW_ZOOM_SYNC` — View/Zoom/Synchronize Across Views

### Edit (6)

- [ ] `IDM_EDIT_FUNCCALLTIP` — Edit/Auto-Completion/Function Parameters Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_NEXT` — Edit/Auto-Completion/Function Parameters Next Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_PREVIOUS` — Edit/Auto-Completion/Function Parameters Previous Hint
- [ ] `IDM_EDIT_CHAR_PANEL` — Edit/Character Panel
- [ ] `IDM_EDIT_COLUMNMODE` — Edit/Column Editor...
- [ ] `IDM_EDIT_COLUMNMODETIP` — Edit/Column Mode...

### Settings (4)

- [ ] `IDM_SETTING_IMPORTPLUGIN` — Settings/Import/Import plugin(s)...
- [ ] `IDM_SETTING_IMPORTSTYLETHEMES` — Settings/Import/Import style theme(s)...
- [ ] `IDM_SETTING_SHORTCUT_MAPPER` — Settings/Shortcut Mapper...
- [ ] `IDM_LANGSTYLE_CONFIG_DLG` — Settings/Style Configurator...

### Encoding (3)

- [ ] `IDM_FORMAT_ANSI` — Encoding/ANSI
- [ ] `IDM_FORMAT_AS_UTF_8` — Encoding/UTF-8
- [ ] `IDM_FORMAT_UTF_8` — Encoding/UTF-8-BOM

### Run (2)

- [ ] `IDM_EXECUTE` — Run/Run...
- [ ] `IDM_EXECUTE_VALIDATE_SHORTCUTSXML` — Run/Validate shortcuts.xml

### Help / misc (2)

- [ ] `IDM_CMDLINEARGUMENTS` — ?/Command Line Arguments...
- [ ] `IDM_SETTING_PLUGINADM` — IDM_SETTING_PLUGINADM

### File (1)

- [ ] `IDM_FILE_CLOSEALL_BUT_PINNED` — File/Close Multiple Documents/Close All but Pinned Documents

## Cleared this batch (2026-08-29)

Paint path hides `hidden_lines` and paints `style_marks` / bookmark ticks. Menu Cut/Copy/Paste use the session clipboard. Find-mark jump uses bookmarks.

- Search style mark / jump / clear / copy-styled (34)
- View fold / unfold / hide lines / open in browser / new instance (27)
- Edit Cut / Copy / Paste / paste-special / autocomplete / multi-select / system read-only / open folder on selection (20)
- Tools hash suite (12)
- Help URLs + Changelog (6)
- Preferences + Open Plugins Folder (2)
- File delete / print / session load-save (5)
