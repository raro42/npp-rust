# Menu implementation todo

Date: 2026-08-28

Ready: **273**. Remaining stubs: **301**.

Track progress by wiring `IDM_*` in `crates/app/src/commands.rs` and updating `is_implemented`.
Teal menu labels = ready.

## Edit (74)

- [ ] `IDM_EDIT_BEGINENDSELECT` — Edit/Begin/End Select
- [ ] `IDM_EDIT_BEGINENDSELECT_COLUMNMODE` — Edit/Begin/End Select in Column Mode
- [ ] `IDM_EDIT_INSERT_DATETIME_CUSTOMIZED` — Edit/Insert/Date Time (customized)
- [ ] `IDM_EDIT_PROPERCASE_BLEND` — Edit/Convert Case to/Proper Case (blend)
- [ ] `IDM_EDIT_SENTENCECASE_FORCE` — Edit/Convert Case to/Sentence case
- [ ] `IDM_EDIT_SENTENCECASE_BLEND` — Edit/Convert Case to/Sentence case (blend)
- [ ] `IDM_EDIT_RANDOMCASE` — Edit/Convert Case to/ranDOm CasE
- [ ] `IDM_EDIT_REMOVE_ANY_DUP_LINES` — Edit/Line Operations/Remove Duplicate Lines
- [ ] `IDM_EDIT_REMOVE_CONSECUTIVE_DUP_LINES` — Edit/Line Operations/Remove Consecutive Duplicate Lines
- [ ] `IDM_EDIT_SPLIT_LINES` — Edit/Line Operations/Split Lines
- [ ] `IDM_EDIT_BLANKLINEBELOWCURRENT` — Edit/Line Operations/Insert Blank Line Below Current
- [ ] `IDM_EDIT_SORTLINES_REVERSE_ORDER` — Edit/Line Operations/Reverse Line Order
- [ ] `IDM_EDIT_SORTLINES_RANDOMLY` — Edit/Line Operations/Randomize Line Order
- [ ] `IDM_EDIT_SORTLINES_LEXICOGRAPHIC_ASCENDING` — Edit/Line Operations/Sort Lines Lexicographically Ascending
- [ ] `IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_ASCENDING` — Edit/Line Operations/Sort Lines Lex. Ascending Ignoring Case
- [ ] `IDM_EDIT_SORTLINES_LOCALE_ASCENDING` — Edit/Line Operations/Sort Lines In Locale Order Ascending
- [ ] `IDM_EDIT_SORTLINES_INTEGER_ASCENDING` — Edit/Line Operations/Sort Lines As Integers Ascending
- [ ] `IDM_EDIT_SORTLINES_DECIMALCOMMA_ASCENDING` — Edit/Line Operations/Sort Lines As Decimals (Comma) Ascending
- [ ] `IDM_EDIT_SORTLINES_DECIMALDOT_ASCENDING` — Edit/Line Operations/Sort Lines As Decimals (Dot) Ascending
- [ ] `IDM_EDIT_SORTLINES_LENGTH_ASCENDING` — Edit/Line Operations/Sort Lines By Length Ascending
- [ ] `IDM_EDIT_SORTLINES_LEXICOGRAPHIC_DESCENDING` — Edit/Line Operations/Sort Lines Lexicographically Descending
- [ ] `IDM_EDIT_SORTLINES_LEXICO_CASE_INSENS_DESCENDING` — Edit/Line Operations/Sort Lines Lex. Descending Ignoring Case
- [ ] `IDM_EDIT_SORTLINES_LOCALE_DESCENDING` — Edit/Line Operations/Sort Lines In Locale Order Descending
- [ ] `IDM_EDIT_SORTLINES_INTEGER_DESCENDING` — Edit/Line Operations/Sort Lines As Integers Descending
- [ ] `IDM_EDIT_SORTLINES_DECIMALCOMMA_DESCENDING` — Edit/Line Operations/Sort Lines As Decimals (Comma) Descending
- [ ] `IDM_EDIT_SORTLINES_DECIMALDOT_DESCENDING` — Edit/Line Operations/Sort Lines As Decimals (Dot) Descending
- [ ] `IDM_EDIT_SORTLINES_LENGTH_DESCENDING` — Edit/Line Operations/Sort Lines By Length Descending
- [ ] `IDM_EDIT_BLOCK_COMMENT` — Edit/Comment/Uncomment/Toggle Single Line Comment
- [ ] `IDM_EDIT_BLOCK_COMMENT_SET` — Edit/Comment/Uncomment/Single Line Comment
- [ ] `IDM_EDIT_BLOCK_UNCOMMENT` — Edit/Comment/Uncomment/Single Line Uncomment
- [ ] `IDM_EDIT_STREAM_COMMENT` — Edit/Comment/Uncomment/Block Comment
- [ ] `IDM_EDIT_STREAM_UNCOMMENT` — Edit/Comment/Uncomment/Block Uncomment
- [ ] `IDM_EDIT_AUTOCOMPLETE` — Edit/Auto-Completion/Function Completion
- [ ] `IDM_EDIT_AUTOCOMPLETE_CURRENTFILE` — Edit/Auto-Completion/Word Completion
- [ ] `IDM_EDIT_FUNCCALLTIP` — Edit/Auto-Completion/Function Parameters Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_PREVIOUS` — Edit/Auto-Completion/Function Parameters Previous Hint
- [ ] `IDM_EDIT_FUNCCALLTIP_NEXT` — Edit/Auto-Completion/Function Parameters Next Hint
- [ ] `IDM_EDIT_AUTOCOMPLETE_PATH` — Edit/Auto-Completion/Path Completion
- [ ] `IDM_FORMAT_TOMAC` — Edit/EOL Conversion/Macintosh (CR)
- [ ] `IDM_EDIT_TRIMLINEHEAD` — Edit/Blank Operations/Trim Leading Space
- [ ] `IDM_EDIT_TRIM_BOTH` — Edit/Blank Operations/Trim Leading and Trailing Space
- [ ] `IDM_EDIT_EOL2WS` — Edit/Blank Operations/EOL to Space
- [ ] `IDM_EDIT_TRIMALL` — Edit/Blank Operations/Trim both and EOL to Space
- [ ] `IDM_EDIT_TAB2SW` — Edit/Blank Operations/TAB to Space
- [ ] `IDM_EDIT_SW2TAB_ALL` — Edit/Blank Operations/Space to TAB (All)
- [ ] `IDM_EDIT_SW2TAB_LEADING` — Edit/Blank Operations/Space to TAB (Leading)
- [ ] `IDM_EDIT_PASTE_AS_HTML` — Edit/Paste Special/Paste HTML Content
- [ ] `IDM_EDIT_PASTE_AS_RTF` — Edit/Paste Special/Paste RTF Content
- [ ] `IDM_EDIT_COPY_BINARY` — Edit/Paste Special/Copy Binary Content
- [ ] `IDM_EDIT_CUT_BINARY` — Edit/Paste Special/Cut Binary Content
- [ ] `IDM_EDIT_PASTE_BINARY` — Edit/Paste Special/Paste Binary Content
- [ ] `IDM_EDIT_OPENSELECTEDFILETOEDIT` — Edit/On Selection/Open File
- [ ] `IDM_EDIT_OPENSELECTEDFILEFOLDERINEXPLORER` — Edit/On Selection/Open Containing Folder in Explorer
- [ ] `IDM_EDIT_REDACT_SELECTION` — Edit/On Selection/Redact Selection █ (Shift: ●)
- [ ] `IDM_EDIT_SEARCHONINTERNET` — Edit/On Selection/Search on Internet
- [ ] `IDM_EDIT_CHANGESEARCHENGINE` — Edit/On Selection/Change Search Engine...
- [ ] `IDM_EDIT_MULTISELECTALL` — Edit/Multi-select All/Ignore Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTALLMATCHCASE` — Edit/Multi-select All/Match Case Only
- [ ] `IDM_EDIT_MULTISELECTALLWHOLEWORD` — Edit/Multi-select All/Match Whole Word Only
- [ ] `IDM_EDIT_MULTISELECTALLMATCHCASEWHOLEWORD` — Edit/Multi-select All/Match Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTNEXT` — Edit/Multi-select Next/Ignore Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTNEXTMATCHCASE` — Edit/Multi-select Next/Match Case Only
- [ ] `IDM_EDIT_MULTISELECTNEXTWHOLEWORD` — Edit/Multi-select Next/Match Whole Word Only
- [ ] `IDM_EDIT_MULTISELECTNEXTMATCHCASEWHOLEWORD` — Edit/Multi-select Next/Match Case  Whole Word
- [ ] `IDM_EDIT_MULTISELECTUNDO` — Edit/Undo the Latest Added Multi-Select
- [ ] `IDM_EDIT_MULTISELECTSSKIP` — Edit/Skip Current  Go to Next Multi-select
- [ ] `IDM_EDIT_COLUMNMODETIP` — Edit/Column Mode...
- [ ] `IDM_EDIT_COLUMNMODE` — Edit/Column Editor...
- [ ] `IDM_EDIT_CHAR_PANEL` — Edit/Character Panel
- [ ] `IDM_EDIT_CLIPBOARDHISTORY_PANEL` — Edit/Clipboard History
- [ ] `IDM_EDIT_TOGGLEREADONLY` — Edit/Read-Only in Notepad++/Read-Only on Current Document
- [ ] `IDM_EDIT_SETREADONLYFORALLDOCS` — Edit/Read-Only in Notepad++/Read-Only for All Documents
- [ ] `IDM_EDIT_CLEARREADONLYFORALLDOCS` — Edit/Read-Only in Notepad++/Clear Read-Only for All Documents
- [ ] `IDM_EDIT_TOGGLESYSTEMREADONLY` — Edit/Read-Only Attribute in Windows

## View (63)

- [ ] `IDM_VIEW_FULLSCREENTOGGLE` — View/Toggle Full Screen Mode
- [ ] `IDM_VIEW_POSTIT` — View/Post-It
- [ ] `IDM_VIEW_DISTRACTIONFREE` — View/Distraction Free Mode
- [ ] `IDM_VIEW_IN_FIREFOX` — View/View Current File in/Firefox
- [ ] `IDM_VIEW_IN_CHROME` — View/View Current File in/Chrome
- [ ] `IDM_VIEW_IN_EDGE` — View/View Current File in/Edge
- [ ] `IDM_VIEW_IN_IE` — View/View Current File in/IE
- [ ] `IDM_VIEW_TAB_SPACE` — View/Show Symbol/Show Space and Tab
- [ ] `IDM_VIEW_EOL` — View/Show Symbol/Show End of Line
- [ ] `IDM_VIEW_NPC` — View/Show Symbol/Show Non-Printing Characters
- [ ] `IDM_VIEW_NPC_CCUNIEOL` — View/Show Symbol/Show Control Characters  Unicode EOL
- [ ] `IDM_VIEW_ALL_CHARACTERS` — View/Show Symbol/Show All Characters
- [ ] `IDM_VIEW_INDENT_GUIDE` — View/Show Symbol/Show Indent Guide
- [ ] `IDM_VIEW_WRAP_SYMBOL` — View/Show Symbol/Show Wrap Symbol
- [ ] `IDM_VIEW_ZOOM_SYNC` — View/Zoom/Synchronize Across Views
- [ ] `IDM_VIEW_GOTO_ANOTHER_VIEW` — View/Move/Clone Current Document/Move to Other View
- [ ] `IDM_VIEW_CLONE_TO_ANOTHER_VIEW` — View/Move/Clone Current Document/Clone to Other View
- [ ] `IDM_VIEW_GOTO_NEW_INSTANCE` — View/Move/Clone Current Document/Move to New Instance
- [ ] `IDM_VIEW_LOAD_IN_NEW_INSTANCE` — View/Move/Clone Current Document/Open in New Instance
- [ ] `IDM_VIEW_TAB_MOVEFORWARD` — View/Tab/Move Tab Forward
- [ ] `IDM_VIEW_TAB_MOVEBACKWARD` — View/Tab/Move Tab Backward
- [ ] `IDM_VIEW_TAB_COLOUR_1` — View/Tab/Apply Color 1
- [ ] `IDM_VIEW_TAB_COLOUR_2` — View/Tab/Apply Color 2
- [ ] `IDM_VIEW_TAB_COLOUR_3` — View/Tab/Apply Color 3
- [ ] `IDM_VIEW_TAB_COLOUR_4` — View/Tab/Apply Color 4
- [ ] `IDM_VIEW_TAB_COLOUR_5` — View/Tab/Apply Color 5
- [ ] `IDM_VIEW_TAB_COLOUR_NONE` — View/Tab/Remove Color
- [ ] `IDM_VIEW_WRAP` — View/Word wrap
- [ ] `IDM_VIEW_SWITCHTO_OTHER_VIEW` — View/Focus on Another View
- [ ] `IDM_VIEW_HIDELINES` — View/Hide Lines
- [ ] `IDM_VIEW_FOLDALL` — View/Fold All
- [ ] `IDM_VIEW_UNFOLDALL` — View/Unfold All
- [ ] `IDM_VIEW_FOLD_CURRENT` — View/Fold Current Level
- [ ] `IDM_VIEW_UNFOLD_CURRENT` — View/Unfold Current Level
- [ ] `IDM_VIEW_FOLD_1` — View/Fold Level/1
- [ ] `IDM_VIEW_FOLD_2` — View/Fold Level/2
- [ ] `IDM_VIEW_FOLD_3` — View/Fold Level/3
- [ ] `IDM_VIEW_FOLD_4` — View/Fold Level/4
- [ ] `IDM_VIEW_FOLD_5` — View/Fold Level/5
- [ ] `IDM_VIEW_FOLD_6` — View/Fold Level/6
- [ ] `IDM_VIEW_FOLD_7` — View/Fold Level/7
- [ ] `IDM_VIEW_FOLD_8` — View/Fold Level/8
- [ ] `IDM_VIEW_UNFOLD_1` — View/Unfold Level/1
- [ ] `IDM_VIEW_UNFOLD_2` — View/Unfold Level/2
- [ ] `IDM_VIEW_UNFOLD_3` — View/Unfold Level/3
- [ ] `IDM_VIEW_UNFOLD_4` — View/Unfold Level/4
- [ ] `IDM_VIEW_UNFOLD_5` — View/Unfold Level/5
- [ ] `IDM_VIEW_UNFOLD_6` — View/Unfold Level/6
- [ ] `IDM_VIEW_UNFOLD_7` — View/Unfold Level/7
- [ ] `IDM_VIEW_UNFOLD_8` — View/Unfold Level/8
- [ ] `IDM_VIEW_SUMMARY` — View/Summary...
- [ ] `IDM_VIEW_PROJECT_PANEL_1` — View/Project Panels/Project Panel 1
- [ ] `IDM_VIEW_PROJECT_PANEL_2` — View/Project Panels/Project Panel 2
- [ ] `IDM_VIEW_PROJECT_PANEL_3` — View/Project Panels/Project Panel 3
- [ ] `IDM_VIEW_FILEBROWSER` — View/Folder as Workspace
- [ ] `IDM_VIEW_DOC_MAP` — View/Document Map
- [ ] `IDM_VIEW_DOCLIST` — View/Document List
- [ ] `IDM_VIEW_FUNC_LIST` — View/Function List
- [ ] `IDM_VIEW_SYNSCROLLV` — View/Synchronize Vertical Scrolling
- [ ] `IDM_VIEW_SYNSCROLLH` — View/Synchronize Horizontal Scrolling
- [ ] `IDM_EDIT_RTL` — View/Text Direction RTL
- [ ] `IDM_EDIT_LTR` — View/Text Direction LTR
- [ ] `IDM_VIEW_MONITORING` — View/Monitoring (tail -f)

## Search (59)

- [ ] `IDM_SEARCH_FINDINFILES` — Search/Find in Files...
- [ ] `IDM_SEARCH_VOLATILE_FINDNEXT` — Search/Find (Volatile) Next
- [ ] `IDM_SEARCH_VOLATILE_FINDPREV` — Search/Find (Volatile) Previous
- [ ] `IDM_SEARCH_FINDINCREMENT` — Search/Incremental Search
- [ ] `IDM_FOCUS_ON_FOUND_RESULTS` — Search/Search Results Window
- [ ] `IDM_SEARCH_GOTONEXTFOUND` — Search/Next Search Result
- [ ] `IDM_SEARCH_GOTOPREVFOUND` — Search/Previous Search Result
- [ ] `IDM_SEARCH_GOTOMATCHINGBRACE` — Search/Go to Matching Brace
- [ ] `IDM_SEARCH_SELECTMATCHINGBRACES` — Search/Select All In-between {} [] or ()
- [ ] `IDM_SEARCH_MARK` — Search/Mark...
- [ ] `IDM_SEARCH_CHANGED_NEXT` — Search/Change History/Go to Next Change
- [ ] `IDM_SEARCH_CHANGED_PREV` — Search/Change History/Go to Previous Change
- [ ] `IDM_SEARCH_CLEAR_CHANGE_HISTORY` — Search/Change History/Clear Change History
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
- [ ] `IDM_SEARCH_UNMARKALLEXT1` — Search/Clear Style/Clear 1st Style
- [ ] `IDM_SEARCH_UNMARKALLEXT2` — Search/Clear Style/Clear 2nd Style
- [ ] `IDM_SEARCH_UNMARKALLEXT3` — Search/Clear Style/Clear 3rd Style
- [ ] `IDM_SEARCH_UNMARKALLEXT4` — Search/Clear Style/Clear 4th Style
- [ ] `IDM_SEARCH_UNMARKALLEXT5` — Search/Clear Style/Clear 5th Style
- [ ] `IDM_SEARCH_CLEARALLMARKS` — Search/Clear Style/Clear all Styles
- [ ] `IDM_SEARCH_GOPREVMARKER1` — Search/Jump Up/1st Style
- [ ] `IDM_SEARCH_GOPREVMARKER2` — Search/Jump Up/2nd Style
- [ ] `IDM_SEARCH_GOPREVMARKER3` — Search/Jump Up/3rd Style
- [ ] `IDM_SEARCH_GOPREVMARKER4` — Search/Jump Up/4th Style
- [ ] `IDM_SEARCH_GOPREVMARKER5` — Search/Jump Up/5th Style
- [ ] `IDM_SEARCH_GOPREVMARKER_DEF` — Search/Jump Up/Find Mark Style
- [ ] `IDM_SEARCH_GONEXTMARKER1` — Search/Jump Down/1st Style
- [ ] `IDM_SEARCH_GONEXTMARKER2` — Search/Jump Down/2nd Style
- [ ] `IDM_SEARCH_GONEXTMARKER3` — Search/Jump Down/3rd Style
- [ ] `IDM_SEARCH_GONEXTMARKER4` — Search/Jump Down/4th Style
- [ ] `IDM_SEARCH_GONEXTMARKER5` — Search/Jump Down/5th Style
- [ ] `IDM_SEARCH_GONEXTMARKER_DEF` — Search/Jump Down/Find Mark Style
- [ ] `IDM_SEARCH_STYLE1TOCLIP` — Search/Copy Styled Text/1st Style
- [ ] `IDM_SEARCH_STYLE2TOCLIP` — Search/Copy Styled Text/2nd Style
- [ ] `IDM_SEARCH_STYLE3TOCLIP` — Search/Copy Styled Text/3rd Style
- [ ] `IDM_SEARCH_STYLE4TOCLIP` — Search/Copy Styled Text/4th Style
- [ ] `IDM_SEARCH_STYLE5TOCLIP` — Search/Copy Styled Text/5th Style
- [ ] `IDM_SEARCH_ALLSTYLESTOCLIP` — Search/Copy Styled Text/All Styles
- [ ] `IDM_SEARCH_MARKEDTOCLIP` — Search/Copy Styled Text/Find Mark Style
- [ ] `IDM_SEARCH_TOGGLE_BOOKMARK` — Search/Bookmark/Toggle Bookmark
- [ ] `IDM_SEARCH_NEXT_BOOKMARK` — Search/Bookmark/Next Bookmark
- [ ] `IDM_SEARCH_PREV_BOOKMARK` — Search/Bookmark/Previous Bookmark
- [ ] `IDM_SEARCH_CLEAR_BOOKMARKS` — Search/Bookmark/Clear All Bookmarks
- [ ] `IDM_SEARCH_CUTMARKEDLINES` — Search/Bookmark/Cut Bookmarked Lines
- [ ] `IDM_SEARCH_COPYMARKEDLINES` — Search/Bookmark/Copy Bookmarked Lines
- [ ] `IDM_SEARCH_PASTEMARKEDLINES` — Search/Bookmark/Paste to (Replace) Bookmarked Lines
- [ ] `IDM_SEARCH_DELETEMARKEDLINES` — Search/Bookmark/Remove Bookmarked Lines
- [ ] `IDM_SEARCH_DELETEUNMARKEDLINES` — Search/Bookmark/Remove Non-Bookmarked Lines
- [ ] `IDM_SEARCH_INVERSEMARKS` — Search/Bookmark/Inverse Bookmarks
- [ ] `IDM_SEARCH_FINDCHARINRANGE` — Search/Find characters in range...

## Encoding (53)

- [ ] `IDM_FORMAT_UTF_16BE` — Encoding/UTF-16 BE BOM
- [ ] `IDM_FORMAT_UTF_16LE` — Encoding/UTF-16 LE BOM
- [ ] `IDM_FORMAT_ISO_8859_6` — Encoding/Character sets/Arabic/ISO 8859-6
- [ ] `IDM_FORMAT_DOS_720` — Encoding/Character sets/Arabic/OEM 720
- [ ] `IDM_FORMAT_WIN_1256` — Encoding/Character sets/Arabic/Windows-1256
- [ ] `IDM_FORMAT_ISO_8859_4` — Encoding/Character sets/Baltic/ISO 8859-4
- [ ] `IDM_FORMAT_ISO_8859_13` — Encoding/Character sets/Baltic/ISO 8859-13
- [ ] `IDM_FORMAT_DOS_775` — Encoding/Character sets/Baltic/OEM 775
- [ ] `IDM_FORMAT_WIN_1257` — Encoding/Character sets/Baltic/Windows-1257
- [ ] `IDM_FORMAT_ISO_8859_14` — Encoding/Character sets/Celtic/ISO 8859-14
- [ ] `IDM_FORMAT_ISO_8859_5` — Encoding/Character sets/Cyrillic/ISO 8859-5
- [ ] `IDM_FORMAT_KOI8R_CYRILLIC` — Encoding/Character sets/Cyrillic/KOI8-R
- [ ] `IDM_FORMAT_KOI8U_CYRILLIC` — Encoding/Character sets/Cyrillic/KOI8-U
- [ ] `IDM_FORMAT_MAC_CYRILLIC` — Encoding/Character sets/Cyrillic/Macintosh
- [ ] `IDM_FORMAT_DOS_855` — Encoding/Character sets/Cyrillic/OEM 855
- [ ] `IDM_FORMAT_DOS_866` — Encoding/Character sets/Cyrillic/OEM 866
- [ ] `IDM_FORMAT_WIN_1251` — Encoding/Character sets/Cyrillic/Windows-1251
- [ ] `IDM_FORMAT_DOS_852` — Encoding/Character sets/Central European/OEM 852
- [ ] `IDM_FORMAT_WIN_1250` — Encoding/Character sets/Central European/Windows-1250
- [ ] `IDM_FORMAT_BIG5` — Encoding/Character sets/Chinese/Big5 (Traditional)
- [ ] `IDM_FORMAT_GB2312` — Encoding/Character sets/Chinese/GB2312 (Simplified)
- [ ] `IDM_FORMAT_ISO_8859_2` — Encoding/Character sets/Eastern European/ISO 8859-2
- [ ] `IDM_FORMAT_ISO_8859_7` — Encoding/Character sets/Greek/ISO 8859-7
- [ ] `IDM_FORMAT_DOS_737` — Encoding/Character sets/Greek/OEM 737
- [ ] `IDM_FORMAT_DOS_869` — Encoding/Character sets/Greek/OEM 869
- [ ] `IDM_FORMAT_WIN_1253` — Encoding/Character sets/Greek/Windows-1253
- [ ] `IDM_FORMAT_ISO_8859_8` — Encoding/Character sets/Hebrew/ISO 8859-8
- [ ] `IDM_FORMAT_DOS_862` — Encoding/Character sets/Hebrew/OEM 862
- [ ] `IDM_FORMAT_WIN_1255` — Encoding/Character sets/Hebrew/Windows-1255
- [ ] `IDM_FORMAT_SHIFT_JIS` — Encoding/Character sets/Japanese/Shift-JIS
- [ ] `IDM_FORMAT_KOREAN_WIN` — Encoding/Character sets/Korean/Windows 949
- [ ] `IDM_FORMAT_EUC_KR` — Encoding/Character sets/Korean/EUC-KR
- [ ] `IDM_FORMAT_DOS_861` — Encoding/Character sets/North European/OEM 861 : Icelandic
- [ ] `IDM_FORMAT_DOS_865` — Encoding/Character sets/North European/OEM 865 : Nordic
- [ ] `IDM_FORMAT_TIS_620` — Encoding/Character sets/Thai/TIS-620
- [ ] `IDM_FORMAT_ISO_8859_3` — Encoding/Character sets/Turkish/ISO 8859-3
- [ ] `IDM_FORMAT_ISO_8859_9` — Encoding/Character sets/Turkish/ISO 8859-9
- [ ] `IDM_FORMAT_DOS_857` — Encoding/Character sets/Turkish/OEM 857
- [ ] `IDM_FORMAT_WIN_1254` — Encoding/Character sets/Turkish/Windows-1254
- [ ] `IDM_FORMAT_ISO_8859_1` — Encoding/Character sets/Western European/ISO 8859-1
- [ ] `IDM_FORMAT_ISO_8859_15` — Encoding/Character sets/Western European/ISO 8859-15
- [ ] `IDM_FORMAT_DOS_850` — Encoding/Character sets/Western European/OEM 850
- [ ] `IDM_FORMAT_DOS_858` — Encoding/Character sets/Western European/OEM 858
- [ ] `IDM_FORMAT_DOS_860` — Encoding/Character sets/Western European/OEM 860 : Portuguese
- [ ] `IDM_FORMAT_DOS_863` — Encoding/Character sets/Western European/OEM 863 : French
- [ ] `IDM_FORMAT_DOS_437` — Encoding/Character sets/Western European/OEM-US
- [ ] `IDM_FORMAT_WIN_1252` — Encoding/Character sets/Western European/Windows-1252
- [ ] `IDM_FORMAT_WIN_1258` — Encoding/Character sets/Vietnamese/Windows-1258
- [ ] `IDM_FORMAT_CONV2_ANSI` — Encoding/Convert to ANSI
- [ ] `IDM_FORMAT_CONV2_AS_UTF_8` — Encoding/Convert to UTF-8
- [ ] `IDM_FORMAT_CONV2_UTF_8` — Encoding/Convert to UTF-8-BOM
- [ ] `IDM_FORMAT_CONV2_UTF_16BE` — Encoding/Convert to UTF-16 BE BOM
- [ ] `IDM_FORMAT_CONV2_UTF_16LE` — Encoding/Convert to UTF-16 LE BOM

## Tools (12)

- [ ] `IDM_TOOL_MD5_GENERATE` — Tools/MD5/Generate...
- [ ] `IDM_TOOL_MD5_GENERATEFROMFILE` — Tools/MD5/Generate from files...
- [ ] `IDM_TOOL_MD5_GENERATEINTOCLIPBOARD` — Tools/MD5/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA1_GENERATE` — Tools/SHA-1/Generate...
- [ ] `IDM_TOOL_SHA1_GENERATEFROMFILE` — Tools/SHA-1/Generate from files...
- [ ] `IDM_TOOL_SHA1_GENERATEINTOCLIPBOARD` — Tools/SHA-1/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA256_GENERATE` — Tools/SHA-256/Generate...
- [ ] `IDM_TOOL_SHA256_GENERATEFROMFILE` — Tools/SHA-256/Generate from files...
- [ ] `IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD` — Tools/SHA-256/Generate from selection into clipboard
- [ ] `IDM_TOOL_SHA512_GENERATE` — Tools/SHA-512/Generate...
- [ ] `IDM_TOOL_SHA512_GENERATEFROMFILE` — Tools/SHA-512/Generate from files...
- [ ] `IDM_TOOL_SHA512_GENERATEINTOCLIPBOARD` — Tools/SHA-512/Generate from selection into clipboard

## Window (11)

- [ ] `IDM_WINDOW_SORT_FN_ASC` — Window/Sort By/Name A to Z
- [ ] `IDM_WINDOW_SORT_FN_DSC` — Window/Sort By/Name Z to A
- [ ] `IDM_WINDOW_SORT_FP_ASC` — Window/Sort By/Path A to Z
- [ ] `IDM_WINDOW_SORT_FP_DSC` — Window/Sort By/Path Z to A
- [ ] `IDM_WINDOW_SORT_FT_ASC` — Window/Sort By/Type A to Z
- [ ] `IDM_WINDOW_SORT_FT_DSC` — Window/Sort By/Type Z to A
- [ ] `IDM_WINDOW_SORT_FS_ASC` — Window/Sort By/Content Length Ascending
- [ ] `IDM_WINDOW_SORT_FS_DSC` — Window/Sort By/Content Length Descending
- [ ] `IDM_WINDOW_SORT_FD_ASC` — Window/Sort By/Modified Time Ascending
- [ ] `IDM_WINDOW_SORT_FD_DSC` — Window/Sort By/Modified Time Descending
- [ ] `IDM_WINDOW_WINDOWS` — Window/Windows...

## File (8)

- [ ] `IDM_FILE_CONTAININGFOLDERASWORKSPACE` — File/Open Containing Folder/Folder as Workspace
- [ ] `IDM_FILE_OPENFOLDERASWORKSPACE` — File/Open Folder as Workspace...
- [ ] `IDM_FILE_CLOSEALL_BUT_PINNED` — File/Close Multiple Documents/Close All but Pinned Documents
- [ ] `IDM_FILE_DELETE` — File/Move to Recycle Bin
- [ ] `IDM_FILE_LOADSESSION` — File/Load Session...
- [ ] `IDM_FILE_SAVESESSION` — File/Save Session...
- [ ] `IDM_FILE_PRINT` — File/Print...
- [ ] `IDM_FILE_PRINTNOW` — File/Print Now

## ? (8)

- [ ] `IDM_CMDLINEARGUMENTS` — ?/Command Line Arguments...
- [ ] `IDM_HOMESWEETHOME` — ?/Notepad++ Home
- [ ] `IDM_PROJECTPAGE` — ?/Notepad++ Project Page
- [ ] `IDM_ONLINEDOCUMENT` — ?/Notepad++ Online User Manual
- [ ] `IDM_FORUM` — ?/Notepad++ Community (Forum)
- [ ] `IDM_UPDATE_NPP` — ?/Update Notepad++
- [ ] `IDM_CONFUPDATERPROXY` — ?/Set Updater Proxy...
- [ ] `IDM_DEBUGINFO` — ?/Debug Info...

## Settings (5)

- [ ] `IDM_SETTING_PREFERENCE` — Settings/Preferences...
- [ ] `IDM_LANGSTYLE_CONFIG_DLG` — Settings/Style Configurator...
- [ ] `IDM_SETTING_SHORTCUT_MAPPER` — Settings/Shortcut Mapper...
- [ ] `IDM_SETTING_IMPORTPLUGIN` — Settings/Import/Import plugin(s)...
- [ ] `IDM_SETTING_IMPORTSTYLETHEMES` — Settings/Import/Import style theme(s)...

## Macro (5)

- [ ] `IDM_MACRO_STARTRECORDINGMACRO` — Macro/Start Recording
- [ ] `IDM_MACRO_STOPRECORDINGMACRO` — Macro/Stop Recording
- [ ] `IDM_MACRO_PLAYBACKRECORDEDMACRO` — Macro/Playback
- [ ] `IDM_MACRO_SAVECURRENTMACRO` — Macro/Save Current Recorded Macro...
- [ ] `IDM_MACRO_RUNMULTIMACRODLG` — Macro/Run a Macro Multiple Times...

## Run (2)

- [ ] `IDM_EXECUTE` — Run/Run...
- [ ] `IDM_EXECUTE_VALIDATE_SHORTCUTSXML` — Run/Validate shortcuts.xml

## Plugins (1)

- [ ] `IDM_SETTING_OPENPLUGINSDIR` — Plugins/Open Plugins Folder...

