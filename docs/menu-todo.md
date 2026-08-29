# Menu implementation todo

Date: 2026-08-29

## Read this first

**Teal ≠ full feature.**

`is_implemented` (teal menu colour) only means: no “Coming Soon” dialog.
Many teal items only set a status-bar message. That inflated “Ready 477 / stubs 0”.

| Class | Count | Meaning |
|-------|------:|---------|
| Explicit handlers | 343 | Match arms in `commands/*.rs` |
| Useful behaviour | 202 | Change buffer or UI beyond a note |
| Placeholder / status-only | 141 | Partial or “not yet” |
| Menu IDs in export | 478 | From `npp_menu.json` |

## Preferences

`Settings → Preferences...` opens a small Preferences window.

Today it covers:

- When opening `*.log` files (Ask / Always / Never) → `npp-rs/settings.json`
- Editor font size (session only for now)

It is **not** a full Notepad++ Preferences clone (tabs, margins, multi-language UI, …).

## Placeholder / status-only (open work)

### Search (39)

- [ ] `IDM_SEARCH_CLEAR_CHANGE_HISTORY` — Search/Change History/Clear Change History
- [ ] `IDM_SEARCH_CHANGED_NEXT` — Search/Change History/Go to Next Change
- [ ] `IDM_SEARCH_CHANGED_PREV` — Search/Change History/Go to Previous Change
- [ ] `IDM_SEARCH_UNMARKALLEXT1` — Search/Clear Style/Clear 1st Style
- [ ] `IDM_SEARCH_UNMARKALLEXT2` — Search/Clear Style/Clear 2nd Style
- [ ] `IDM_SEARCH_UNMARKALLEXT3` — Search/Clear Style/Clear 3rd Style
- [ ] `IDM_SEARCH_UNMARKALLEXT4` — Search/Clear Style/Clear 4th Style
- [ ] `IDM_SEARCH_UNMARKALLEXT5` — Search/Clear Style/Clear 5th Style
- [ ] `IDM_SEARCH_STYLE1TOCLIP` — Search/Copy Styled Text/1st Style
- [ ] `IDM_SEARCH_STYLE2TOCLIP` — Search/Copy Styled Text/2nd Style
- [ ] `IDM_SEARCH_STYLE3TOCLIP` — Search/Copy Styled Text/3rd Style
- [ ] `IDM_SEARCH_STYLE4TOCLIP` — Search/Copy Styled Text/4th Style
- [ ] `IDM_SEARCH_STYLE5TOCLIP` — Search/Copy Styled Text/5th Style
- [ ] `IDM_SEARCH_ALLSTYLESTOCLIP` — Search/Copy Styled Text/All Styles
- [ ] `IDM_SEARCH_MARKEDTOCLIP` — Search/Copy Styled Text/Find Mark Style
- [ ] `IDM_SEARCH_FINDCHARINRANGE` — Search/Find characters in range...
- [ ] `IDM_SEARCH_FINDINFILES` — Search/Find in Files...
- [ ] `IDM_SEARCH_GONEXTMARKER1` — Search/Jump Down/1st Style
- [ ] `IDM_SEARCH_GONEXTMARKER2` — Search/Jump Down/2nd Style
- [ ] `IDM_SEARCH_GONEXTMARKER3` — Search/Jump Down/3rd Style
- [ ] `IDM_SEARCH_GONEXTMARKER4` — Search/Jump Down/4th Style
- [ ] `IDM_SEARCH_GONEXTMARKER5` — Search/Jump Down/5th Style
- [ ] `IDM_SEARCH_GONEXTMARKER_DEF` — Search/Jump Down/Find Mark Style
- [ ] `IDM_SEARCH_GOPREVMARKER1` — Search/Jump Up/1st Style
- [ ] `IDM_SEARCH_GOPREVMARKER2` — Search/Jump Up/2nd Style
- [ ] `IDM_SEARCH_GOPREVMARKER3` — Search/Jump Up/3rd Style
- [ ] `IDM_SEARCH_GOPREVMARKER4` — Search/Jump Up/4th Style
- [ ] `IDM_SEARCH_GOPREVMARKER5` — Search/Jump Up/5th Style
- [ ] `IDM_SEARCH_GOPREVMARKER_DEF` — Search/Jump Up/Find Mark Style
- [ ] `IDM_SEARCH_MARKALLEXT1` — Search/Style All Occurrences of Token/Using 1st Style
- [ ] `IDM_SEARCH_MARKALLEXT2` — Search/Style All Occurrences of Token/Using 2nd Style
- [ ] `IDM_SEARCH_MARKALLEXT3` — Search/Style All Occurrences of Token/Using 3rd Style
- [ ] `IDM_SEARCH_MARKALLEXT4` — Search/Style All Occurrences of Token/Using 4th Style
- [ ] `IDM_SEARCH_MARKALLEXT5` — Search/Style All Occurrences of Token/Using 5th Style
- [ ] `IDM_SEARCH_MARKONEEXT1` — Search/Style One Token/Using 1st Style
- [ ] `IDM_SEARCH_MARKONEEXT2` — Search/Style One Token/Using 2nd Style
- [ ] `IDM_SEARCH_MARKONEEXT3` — Search/Style One Token/Using 3rd Style
- [ ] `IDM_SEARCH_MARKONEEXT4` — Search/Style One Token/Using 4th Style
- [ ] `IDM_SEARCH_MARKONEEXT5` — Search/Style One Token/Using 5th Style

### View (39)

- [ ] `IDM_VIEW_DOC_MAP` — View/Document Map
- [ ] `IDM_VIEW_SWITCHTO_OTHER_VIEW` — View/Focus on Another View
- [ ] `IDM_VIEW_FOLDALL` — View/Fold All
- [ ] `IDM_VIEW_FOLD_CURRENT` — View/Fold Current Level
- [ ] `IDM_VIEW_FOLD_1` — View/Fold Level/1
- [ ] `IDM_VIEW_FOLD_2` — View/Fold Level/2
- [ ] `IDM_VIEW_FOLD_3` — View/Fold Level/3
- [ ] `IDM_VIEW_FOLD_4` — View/Fold Level/4
- [ ] `IDM_VIEW_FOLD_5` — View/Fold Level/5
- [ ] `IDM_VIEW_FOLD_6` — View/Fold Level/6
- [ ] `IDM_VIEW_FOLD_7` — View/Fold Level/7
- [ ] `IDM_VIEW_FOLD_8` — View/Fold Level/8
- [ ] `IDM_VIEW_FUNC_LIST` — View/Function List
- [ ] `IDM_VIEW_HIDELINES` — View/Hide Lines
- [ ] `IDM_VIEW_GOTO_NEW_INSTANCE` — View/Move/Clone Current Document/Move to New Instance
- [ ] `IDM_VIEW_GOTO_ANOTHER_VIEW` — View/Move/Clone Current Document/Move to Other View
- [ ] `IDM_VIEW_LOAD_IN_NEW_INSTANCE` — View/Move/Clone Current Document/Open in New Instance
- [ ] `IDM_VIEW_PROJECT_PANEL_1` — View/Project Panels/Project Panel 1
- [ ] `IDM_VIEW_PROJECT_PANEL_2` — View/Project Panels/Project Panel 2
- [ ] `IDM_VIEW_PROJECT_PANEL_3` — View/Project Panels/Project Panel 3
- [ ] `IDM_VIEW_SYNSCROLLH` — View/Synchronize Horizontal Scrolling
- [ ] `IDM_VIEW_SYNSCROLLV` — View/Synchronize Vertical Scrolling
- [ ] `IDM_EDIT_LTR` — View/Text Direction LTR
- [ ] `IDM_EDIT_RTL` — View/Text Direction RTL
- [ ] `IDM_VIEW_UNFOLDALL` — View/Unfold All
- [ ] `IDM_VIEW_UNFOLD_CURRENT` — View/Unfold Current Level
- [ ] `IDM_VIEW_UNFOLD_1` — View/Unfold Level/1
- [ ] `IDM_VIEW_UNFOLD_2` — View/Unfold Level/2
- [ ] `IDM_VIEW_UNFOLD_3` — View/Unfold Level/3
- [ ] `IDM_VIEW_UNFOLD_4` — View/Unfold Level/4
- [ ] `IDM_VIEW_UNFOLD_5` — View/Unfold Level/5
- [ ] `IDM_VIEW_UNFOLD_6` — View/Unfold Level/6
- [ ] `IDM_VIEW_UNFOLD_7` — View/Unfold Level/7
- [ ] `IDM_VIEW_UNFOLD_8` — View/Unfold Level/8
- [ ] `IDM_VIEW_IN_CHROME` — View/View Current File in/Chrome
- [ ] `IDM_VIEW_IN_EDGE` — View/View Current File in/Edge
- [ ] `IDM_VIEW_IN_FIREFOX` — View/View Current File in/Firefox
- [ ] `IDM_VIEW_IN_IE` — View/View Current File in/IE
- [ ] `IDM_VIEW_ZOOM_SYNC` — View/Zoom/Synchronize Across Views

### Edit (26)

- [ ] `IDM_EDIT_AUTOCOMPLETE` — Edit/Auto-Completion/Function Completion
- [ ] `IDM_EDIT_FUNCCALLTIP` — Edit/Auto-Completion/Function Parameters Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_NEXT` — Edit/Auto-Completion/Function Parameters Next Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_PREVIOUS` — Edit/Auto-Completion/Function Parameters Previous Hint
- [ ] `IDM_EDIT_AUTOCOMPLETE_PATH` — Edit/Auto-Completion/Path Completion
- [ ] `IDM_EDIT_AUTOCOMPLETE_CURRENTFILE` — Edit/Auto-Completion/Word Completion
- [ ] `IDM_EDIT_CHAR_PANEL` — Edit/Character Panel
- [ ] `IDM_EDIT_COLUMNMODE` — Edit/Column Editor...
- [ ] `IDM_EDIT_COLUMNMODETIP` — Edit/Column Mode...
- [ ] `IDM_EDIT_COPY` — Edit/Copy
- [ ] `IDM_EDIT_CUT` — Edit/Cut
- [ ] `IDM_EDIT_MULTISELECTALL` — Edit/Multi-select All/Ignore Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTALLMATCHCASEWHOLEWORD` — Edit/Multi-select All/Match Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTALLMATCHCASE` — Edit/Multi-select All/Match Case Only
- [ ] `IDM_EDIT_MULTISELECTALLWHOLEWORD` — Edit/Multi-select All/Match Whole Word Only
- [ ] `IDM_EDIT_MULTISELECTNEXT` — Edit/Multi-select Next/Ignore Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTNEXTMATCHCASEWHOLEWORD` — Edit/Multi-select Next/Match Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTNEXTMATCHCASE` — Edit/Multi-select Next/Match Case Only
- [ ] `IDM_EDIT_MULTISELECTNEXTWHOLEWORD` — Edit/Multi-select Next/Match Whole Word Only
- [ ] `IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER` — Edit/On Selection/Open Containing Folder in Explorer
- [ ] `IDM_EDIT_PASTE` — Edit/Paste
- [ ] `IDM_EDIT_PASTE_BINARY` — Edit/Paste Special/Paste Binary Content
- [ ] `IDM_EDIT_PASTE_AS_HTML` — Edit/Paste Special/Paste HTML Content
- [ ] `IDM_EDIT_PASTE_AS_RTF` — Edit/Paste Special/Paste RTF Content
- [ ] `IDM_EDIT_TOGGLESYSTEMREADONLY` — Edit/Read-Only Attribute in Windows
- [ ] `IDM_EDIT_MULTISELECTSSKIP` — Edit/Skip Current  Go to Next Multi-select

### Tools (12)

- [ ] `IDM_TOOL_MD5_GENERATEFROMFILE` — Tools/MD5/Generate from files...
- [ ] `IDM_TOOL_MD5_GENERATEINTOCLIPBOARD` — Tools/MD5/Generate from selection into clipboard
- [ ] `IDM_TOOL_MD5_GENERATE` — Tools/MD5/Generate...
- [ ] `IDM_TOOL_SHA1_GENERATEFROMFILE` — Tools/SHA-1/Generate from files...
- [ ] `IDM_TOOL_SHA1_GENERATEINTOCLIPBOARD` — Tools/SHA-1/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA1_GENERATE` — Tools/SHA-1/Generate...
- [ ] `IDM_TOOL_SHA256_GENERATEFROMFILE` — Tools/SHA-256/Generate from files...
- [ ] `IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD` — Tools/SHA-256/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA256_GENERATE` — Tools/SHA-256/Generate...
- [ ] `IDM_TOOL_SHA512_GENERATEFROMFILE` — Tools/SHA-512/Generate from files...
- [ ] `IDM_TOOL_SHA512_GENERATEINTOCLIPBOARD` — Tools/SHA-512/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA512_GENERATE` — Tools/SHA-512/Generate...

### ? (7)

- [ ] `IDM_CHANGELOG` — ?/Changelog
- [ ] `IDM_CMDLINEARGUMENTS` — ?/Command Line Arguments...
- [ ] `IDM_FORUM` — ?/Discussions
- [ ] `IDM_PROJECTPAGE` — ?/Issues
- [ ] `IDM_UPDATE_NPP` — ?/Releases
- [ ] `IDM_ONLINEDOCUMENT` — ?/Wiki
- [ ] `IDM_HOMESWEETHOME` — ?/npp-rust on GitHub

### File (6)

- [ ] `IDM_FILE_CLOSEALL_BUT_PINNED` — File/Close Multiple Documents/Close All but Pinned Documents
- [ ] `IDM_FILE_LOADSESSION` — File/Load Session...
- [ ] `IDM_FILE_DELETE` — File/Move to Recycle Bin
- [ ] `IDM_FILE_PRINTNOW` — File/Print Now
- [ ] `IDM_FILE_PRINT` — File/Print...
- [ ] `IDM_FILE_SAVESESSION` — File/Save Session...

### Settings (5)

- [ ] `IDM_SETTING_IMPORTPLUGIN` — Settings/Import/Import plugin(s)...
- [ ] `IDM_SETTING_IMPORTSTYLETHEMES` — Settings/Import/Import style theme(s)...
- [ ] `IDM_SETTING_PREFERENCE` — Settings/Preferences...
- [ ] `IDM_SETTING_SHORTCUT_MAPPER` — Settings/Shortcut Mapper...
- [ ] `IDM_LANGSTYLE_CONFIG_DLG` — Settings/Style Configurator...

### Encoding (3)

- [ ] `IDM_FORMAT_ANSI` — Encoding/ANSI
- [ ] `IDM_FORMAT_AS_UTF_8` — Encoding/UTF-8
- [ ] `IDM_FORMAT_UTF_8` — Encoding/UTF-8-BOM

### Run (2)

- [ ] `IDM_EXECUTE` — Run/Run...
- [ ] `IDM_EXECUTE_VALIDATE_SHORTCUTSXML` — Run/Validate shortcuts.xml

### IDM_SETTING_PLUGINADM (1)

- [ ] `IDM_SETTING_PLUGINADM` — IDM_SETTING_PLUGINADM

### Plugins (1)

- [ ] `IDM_SETTING_OPENPLUGINSDIR` — Plugins/Open Plugins Folder...

