//! Parse and match simple keyboard chords for remappable shortcuts.
//!
//! Format: `Alt+Z`, `Ctrl+Shift+W`, `Cmd+L`, `F2` (case-insensitive).
//! `Cmd` / `Command` / `Ctrl` / `Control` all mean egui command-or-ctrl.

use eframe::egui::{Key, Modifiers};

/// Default word-wrap toggle binding (matches historical hard-wire).
pub const DEFAULT_WORD_WRAP: &str = "Alt+Z";

/// Default find-next binding (matches historical hard-wire). Find prev uses Shift toggled.
pub const DEFAULT_FIND_NEXT: &str = "F3";

/// Default global find-next binding (matches historical hard-wire). Find prev uses Shift toggled.
pub const DEFAULT_FIND_NEXT_GLOBAL: &str = "Cmd+G";

/// Default select-and-find-next binding (Search → Select and Find Next; N++ Ctrl+F3 family).
/// Shift flips to Select and Find Previous. Uses Cmd+F3 so it does not collide with Find next (F3).
pub const DEFAULT_SET_AND_FIND: &str = "Cmd+F3";

/// Default next-search-result binding (Search → Next Search Result; N++ F4 family).
/// Shift flips to Previous Search Result. Does not collide with Find next (F3).
pub const DEFAULT_NEXT_FOUND: &str = "F4";

/// Default Find All / Search Results Window binding (current document Find Results).
/// Uses Cmd+Alt+F4 so it stays on the F4 Find Results family without colliding with
/// Next Search Result (F4), Find in Files (Cmd+Alt+F), or Find (Cmd+F).
/// Shift flips to Find All in opened editor tabs.
pub const DEFAULT_FIND_ALL: &str = "Cmd+Alt+F4";

/// Default Incremental Search binding (Search → Incremental Search; Find-bar Inc).
/// Uses Alt+Shift+I so it stays near Indent Guide (Alt+I) without colliding with
/// Format Document (Cmd+Shift+I) or Close All to the Left (Cmd+Alt+I).
pub const DEFAULT_FIND_INCREMENTAL: &str = "Alt+Shift+I";

/// Default Find Mark binding (Search → Mark; Find-bar Mark / Style 1 with Case/Word/Re/Sel).
/// Uses Alt+Shift+F3 so it stays on the F3 Find family without colliding with Find next
/// (F3), Select and Find (Cmd+F3), or Inverse Bookmarks (Alt+Shift+F2).
pub const DEFAULT_FIND_MARK: &str = "Alt+Shift+F3";

/// Default Find Bookmark binding (Find-bar Bookmark all matching lines; Case/Word/Re/Sel).
/// Uses Alt+Shift+F4 so it stays near Find Mark (Alt+Shift+F3) and the F4 Find Results
/// family without colliding with Next Search Result (F4), Find All (Cmd+Alt+F4), or
/// Inverse Bookmarks (Alt+Shift+F2).
pub const DEFAULT_FIND_BOOKMARK: &str = "Alt+Shift+F4";

/// Default Find Copy binding (Find-bar Copy matching substrings; Case/Word/Re/Sel).
/// Uses Alt+Shift+F5 so it stays near Find Mark / Find Bookmark without colliding with
/// Date Time (`F5`), Find Mark (`Alt+Shift+F3`), or Find Bookmark (`Alt+Shift+F4`).
/// Shift flips to Copy matching lines (Find-bar Lines).
pub const DEFAULT_FIND_COPY: &str = "Alt+Shift+F5";

/// Default Find Cut binding (Find-bar Cut matching substrings; Case/Word/Re/Sel).
/// Uses Alt+Shift+F6 so it stays near Find Copy without colliding with Focus on Another
/// View (`F6`), Find Copy (`Alt+Shift+F5`), or Find Bookmark (`Alt+Shift+F4`).
/// Shift flips to Cut matching lines (Find-bar Shift+Cut).
pub const DEFAULT_FIND_CUT: &str = "Alt+Shift+F6";

/// Default Find Purge binding (Find-bar Purge matching lines; Case/Word/Re/Sel).
/// Uses Alt+Shift+F8 so it stays near Find Cut without colliding with Next Hidden Equal
/// (`Alt+F7` / `Alt+Shift+F7`), Next Change (`F8`), or Clear Change History (`Cmd+Shift+F8`).
/// Shift flips to Purge matching substrings (Find-bar Shift+Purge; no clipboard).
pub const DEFAULT_FIND_PURGE: &str = "Alt+Shift+F8";

/// Default Find characters in range binding (Search → Find characters in range).
/// Uses Alt+F9 so it stays near the Find Mark/Copy/Cut/Purge F-key family without colliding
/// with Find Purge (`Alt+Shift+F8` / `Alt+F8`), Next Change (`F8`), or Fullscreen (`F11`).
/// Shift flips to find the previous character in range.
pub const DEFAULT_FIND_CHAR_RANGE: &str = "Alt+F9";

/// Default Find Next (Volatile) binding (Search → Find Next (Volatile)).
/// Uses Cmd+Alt+F3 so it stays on the F3 Find family without colliding with Find next (`F3`),
/// Select and Find (`Cmd+F3`), Find Mark (`Alt+Shift+F3`), or Find All (`Cmd+Alt+F4`).
/// Shift flips to Find Previous (Volatile). Selection is the ephemeral query; Find dialog untouched.
pub const DEFAULT_FIND_VOLATILE: &str = "Cmd+Alt+F3";

/// Default mark-all Style 1 binding (Search → Using Style 1; N++ Ctrl+1 family).
/// Shift flips to Clear Style 1.
pub const DEFAULT_MARK_STYLE: &str = "Cmd+1";

/// Default mark-all Style 2 binding (Search → Using Style 2; N++ Ctrl+2 family).
/// Shift flips to Clear Style 2.
pub const DEFAULT_MARK_STYLE_2: &str = "Cmd+2";

/// Default mark-all Style 3 binding (Search → Using Style 3; N++ Ctrl+3 family).
/// Shift flips to Clear Style 3.
pub const DEFAULT_MARK_STYLE_3: &str = "Cmd+3";

/// Default mark-all Style 4 binding (Search → Using Style 4; N++ Ctrl+4 family).
/// Shift flips to Clear Style 4.
pub const DEFAULT_MARK_STYLE_4: &str = "Cmd+4";

/// Default mark-all Style 5 binding (Search → Using Style 5; N++ Ctrl+5 family).
/// Shift flips to Clear Style 5.
pub const DEFAULT_MARK_STYLE_5: &str = "Cmd+5";

/// Default clear-all-styles binding (Search → Clear all styles).
/// `Cmd+0` stays zoom restore; `Alt+0` stays fold all.
pub const DEFAULT_CLEAR_ALL_STYLES: &str = "Cmd+Shift+0";

/// Default next-style-marker binding (Search → Go to next style 1…5).
/// Uses Cmd+Alt+1 so it stays on the Style digit family without colliding with
/// Mark Style (`Cmd+1`…`5`) or Fold Level (`Alt+1`…`8`). Digit-family chords
/// accept `1`…`5` with those modifiers; Shift flips to previous style.
pub const DEFAULT_NEXT_STYLE: &str = "Cmd+Alt+1";

/// Default copy-styled-text binding (Search → Copy Styled Text 1…5 / All Styles).
/// Uses Cmd+Alt+Shift+1 so it stays on the Style digit family without colliding
/// with Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), or Fold Level
/// (`Alt+1`…`8`). Digit-family chords accept `1`…`5` with those modifiers; same
/// mods + `0` copies all styled lines.
pub const DEFAULT_STYLE_TO_CLIP: &str = "Cmd+Alt+Shift+1";

/// Default mark-current-line-style binding (Search → Mark → Using 1st…5th Style).
/// Uses Alt+Shift+1 so it stays on the Style digit family without colliding with
/// Mark Style (`Cmd+1`…`5`), Next Style (`Cmd+Alt+1`…`5`), Copy Styled
/// (`Cmd+Alt+Shift+1`…`5`), or Fold Level (`Alt+1`…`8`). Digit-family chords
/// accept `1`…`5` with those modifiers.
pub const DEFAULT_MARK_ONE_STYLE: &str = "Alt+Shift+1";

/// Default next-bookmark binding (matches historical hard-wire). Prev uses Shift toggled.
pub const DEFAULT_NEXT_BOOKMARK: &str = "F2";

/// Default toggle-bookmark binding (matches historical hard-wire).
pub const DEFAULT_TOGGLE_BOOKMARK: &str = "Cmd+F2";

/// Default clear-all-bookmarks binding (Search → Clear all bookmarks; N++ Ctrl+Shift+F2 family).
pub const DEFAULT_CLEAR_BOOKMARKS: &str = "Cmd+Shift+F2";

/// Default inverse-bookmarks binding (Search → Inverse Bookmarks).
/// Stays on the F2 family without colliding with next/prev/toggle/clear.
pub const DEFAULT_INVERSE_BOOKMARKS: &str = "Alt+Shift+F2";

/// Default copy-bookmarked-lines binding (Search → Copy Bookmarked Lines).
/// Uses Cmd+Alt+F2 so it does not collide with next (F2), toggle (Cmd+F2),
/// clear (Cmd+Shift+F2), or inverse (Alt+Shift+F2). Shift flips to Cut Bookmarked Lines.
pub const DEFAULT_COPY_MARKED_LINES: &str = "Cmd+Alt+F2";

/// Default paste-bookmarked-lines binding (Search → Paste Bookmarked Lines).
/// Uses Cmd+Alt+Q so it does not collide with Sync Vertical Scroll (Alt+Q),
/// Paste (Cmd+V), or Copy Bookmarked Lines (Cmd+Alt+F2). Shift flips to Remove
/// Bookmarked Lines. Remove Unmarked has its own remappable chord
/// (`shortcut_remove_unmarked`, default Cmd+Alt+Z).
pub const DEFAULT_PASTE_MARKED_LINES: &str = "Cmd+Alt+Q";

/// Default remove-unmarked-lines binding (Search → Remove Unmarked Lines).
/// Uses Cmd+Alt+Z so it stays near Paste/Remove Bookmarked (Cmd+Alt+Q) without
/// colliding with Word Wrap (Alt+Z) or Zoom Sync (Alt+Shift+Z).
pub const DEFAULT_REMOVE_UNMARKED: &str = "Cmd+Alt+Z";

/// Default next change-history mark binding (Search → Next Change). Prev uses Shift toggled.
pub const DEFAULT_NEXT_CHANGED: &str = "F8";

/// Default clear change-history binding (Search → Clear Change History).
/// Uses Cmd+Shift+F8 so it stays near Next/Prev Change (F8 / Shift+F8) without
/// colliding with those chords (they have no Cmd).
pub const DEFAULT_CLEAR_CHANGE_HISTORY: &str = "Cmd+Shift+F8";

/// Default next-compare-diff binding (matches historical hard-wire). Prev uses Shift toggled.
pub const DEFAULT_NEXT_DIFF: &str = "F7";

/// Default first-compare-diff binding (matches historical hard-wire). Last uses Shift toggled.
pub const DEFAULT_FIRST_DIFF: &str = "Cmd+F7";

/// Default next-hidden-equal binding (matches historical hard-wire). Prev uses Shift toggled.
pub const DEFAULT_NEXT_HIDDEN_EQUAL: &str = "Alt+F7";

/// Default first-hidden-equal binding (matches historical hard-wire). Last uses Shift toggled.
pub const DEFAULT_FIRST_HIDDEN_EQUAL: &str = "Cmd+Alt+F7";

/// Default apply-compare-hunk-from binding (matches historical hard-wire).
/// Opposite horizontal arrow applies to other; Shift applies all in that direction.
pub const DEFAULT_APPLY_COMPARE_HUNK: &str = "Cmd+Alt+Left";

/// Default hide-equal-context increase binding (matches historical hard-wire).
/// Opposite bracket (`[` ↔ `]`) decreases.
pub const DEFAULT_HIDE_EQUAL_CONTEXT: &str = "Alt+]";

/// Default hide-unchanged-lines toggle (View → Hide Unchanged Lines).
pub const DEFAULT_HIDE_EQUAL: &str = "Alt+H";

/// Default ignore-whitespace-differences toggle (View → Ignore Whitespace Differences).
/// Shift + same key is Ignore Case Differences.
pub const DEFAULT_COMPARE_IGNORE_WS: &str = "Alt+W";

/// Default ignore-blank-lines toggle (View → Ignore Blank Lines).
pub const DEFAULT_COMPARE_IGNORE_BLANK: &str = "Alt+B";

/// Default expand-all-unchanged binding (View → Expand All Unchanged Lines).
/// Shift + same key is Collapse All Unchanged Lines.
pub const DEFAULT_EXPAND_ALL_UNCHANGED: &str = "Alt+E";

/// Default expand-unchanged-at-caret binding (View → Expand Unchanged at Caret).
/// Shift + same key is Collapse Unchanged at Caret.
pub const DEFAULT_EXPAND_UNCHANGED_AT_CARET: &str = "Alt+X";

/// Default bookmark-compare-differences binding (View → Bookmark Compare Differences).
/// Shift + same key is Clear Compare Difference Bookmarks.
pub const DEFAULT_BOOKMARK_COMPARE_DIFFS: &str = "Alt+M";

/// Default start-compare binding (View → Compare with Other View).
/// Shift + same key is Clear Compare.
pub const DEFAULT_COMPARE: &str = "Alt+D";

/// Default swap-compare-sides binding (View → Swap Compare Sides).
pub const DEFAULT_SWAP_COMPARE: &str = "Alt+S";

/// Default compare-to-saved binding (View → Compare to Saved).
pub const DEFAULT_COMPARE_TO_SAVED: &str = "Alt+Shift+S";

/// Default copy-compare-diff binding (View → Copy Compare Diff).
/// Shift flips to Open Compare Diff.
pub const DEFAULT_COPY_COMPARE_DIFF: &str = "Alt+C";

/// Default copy-compare-summary binding (View → Copy Compare Summary).
/// Shift flips to Open Compare Summary.
pub const DEFAULT_COPY_COMPARE_SUMMARY: &str = "Alt+Y";

/// Default copy-compare-hunk binding (View → Copy Compare Hunk).
/// Shift flips to Open Compare Hunk.
pub const DEFAULT_COPY_COMPARE_HUNK: &str = "Alt+K";

/// Default word-jump-back binding (opposite arrow jumps forward; Shift extends).
pub const DEFAULT_WORD_JUMP: &str = "Alt+Left";

/// Default rectangular column-select binding (Alt+Shift+arrows; N++-style).
/// Arrow-family chords accept any arrow with the same modifiers (Left/Right grow
/// columns; Up/Down grow rows). Letter remaps: base key = left, partner = right.
/// On the default chord, Alt+Shift+Left/Right prefer rect over word-jump extend.
pub const DEFAULT_RECT_SELECT: &str = "Alt+Shift+Left";

/// Default go-to-line binding (matches historical hard-wire).
pub const DEFAULT_GOTO_LINE: &str = "Cmd+L";

/// Default duplicate-line binding (matches historical hard-wire).
pub const DEFAULT_DUPLICATE_LINE: &str = "Cmd+D";

/// Default delete-line binding (matches historical hard-wire).
pub const DEFAULT_DELETE_LINE: &str = "Cmd+Shift+L";

/// Default indent-lines binding (matches historical hard-wire).
pub const DEFAULT_INDENT: &str = "Cmd+]";

/// Default outdent-lines binding (matches historical hard-wire).
pub const DEFAULT_OUTDENT: &str = "Cmd+[";

/// Default format-document binding (matches historical hard-wire).
pub const DEFAULT_FORMAT_DOCUMENT: &str = "Cmd+Shift+I";

/// Default close-tab binding (matches historical hard-wire).
pub const DEFAULT_CLOSE_TAB: &str = "Cmd+W";

/// Default close-all binding (File → Close All). Shift flips to Close All but Current.
/// Uses Cmd+Alt+W so it does not collide with Close tab (Cmd+W).
pub const DEFAULT_CLOSE_ALL: &str = "Cmd+Alt+W";

/// Default Close All but Pinned binding (File → Close All but Pinned Documents).
/// Shift flips to Close All Unchanged. Uses Cmd+Alt+H so it does not collide with
/// Close All (Cmd+Alt+W), Hide unchanged (Alt+H), or Replace (Cmd+H).
pub const DEFAULT_CLOSE_BUT_PINNED: &str = "Cmd+Alt+H";

/// Default Close All to the Left binding (File → Close All to the Left).
/// Shift flips to Close All to the Right. Uses Cmd+Alt+I so it does not collide with
/// Indent Guide (Alt+I), Format Document (Cmd+Shift+I), or Close All (Cmd+Alt+W).
pub const DEFAULT_CLOSE_TO_SIDE: &str = "Cmd+Alt+I";

/// Default Save a Copy As binding (File → Save a Copy As).
/// Shift flips to Rename. Uses Cmd+Alt+Y so it does not collide with Save As
/// (Cmd+Shift+S), Save All (Cmd+Alt+S), or Copy Compare Summary (Alt+Y).
pub const DEFAULT_SAVE_COPY: &str = "Cmd+Alt+Y";

/// Default trim-trailing binding (Edit → Blank Operations → Trim Trailing Space).
/// Shift flips to Trim Leading and Trailing Space. Uses Cmd+Alt+T so it does not
/// collide with Always on Top (Alt+T) or Toggle log tail (Cmd+Shift+T).
pub const DEFAULT_TRIM_TRAILING: &str = "Cmd+Alt+T";

/// Default Trim Leading Space binding (Edit → Blank Operations → Trim Leading Space).
/// Uses Alt+Shift+T so it stays near Trim Trailing (Cmd+Alt+T) and Always on Top
/// (Alt+T) without colliding with either or with Toggle log tail (Cmd+Shift+T).
pub const DEFAULT_TRIM_LEADING: &str = "Alt+Shift+T";

/// Default new-file binding (matches historical hard-wire).
pub const DEFAULT_NEW: &str = "Cmd+N";

/// Default open-file binding (matches historical hard-wire).
pub const DEFAULT_OPEN: &str = "Cmd+O";

/// Default save binding (matches historical hard-wire).
pub const DEFAULT_SAVE: &str = "Cmd+S";

/// Default save-as binding (matches historical hard-wire).
pub const DEFAULT_SAVE_AS: &str = "Cmd+Shift+S";

/// Default find-bar binding (matches historical hard-wire).
pub const DEFAULT_FIND: &str = "Cmd+F";

/// Default Find in Files binding (Search → Find in Files).
/// Uses Cmd+Alt+F so it does not collide with Find (Cmd+F) or Replace alt (Cmd+Shift+F).
pub const DEFAULT_FIND_IN_FILES: &str = "Cmd+Alt+F";

/// Default close-find/replace binding (matches historical hard-wire).
pub const DEFAULT_CLOSE_FIND: &str = "Escape";

/// Default replace-bar binding (matches historical hard-wire).
pub const DEFAULT_REPLACE: &str = "Cmd+H";

/// Default alternate replace-bar binding (matches historical hard-wire).
pub const DEFAULT_REPLACE_ALT: &str = "Cmd+Shift+F";

/// Default select-all binding (matches historical hard-wire).
pub const DEFAULT_SELECT_ALL: &str = "Cmd+A";

/// Default copy binding (Edit → Copy; matches platform Cmd/Ctrl+C).
/// When remapped away from this default, platform `Event::Copy` is ignored so Cmd+C does not still copy.
pub const DEFAULT_COPY: &str = "Cmd+C";

/// Default cut binding (Edit → Cut; matches platform Cmd/Ctrl+X).
/// When remapped away from this default, platform `Event::Cut` is ignored so Cmd+X does not still cut.
pub const DEFAULT_CUT: &str = "Cmd+X";

/// Default paste binding (Edit → Paste; matches platform Cmd/Ctrl+V).
/// Default stays on platform `Event::Paste` (system clipboard). When remapped, that event is
/// ignored and the chord runs menu Paste (`last_copied`).
pub const DEFAULT_PASTE: &str = "Cmd+V";

/// Default forward-delete binding (Edit → Delete; matches historical hard-wire).
/// Default stays in `handle_editor_input`. When remapped, bare Delete is ignored and the chord
/// runs menu Delete (`IDM_EDIT_DELETE`).
pub const DEFAULT_DELETE_FORWARD: &str = "Delete";

/// Default backward-delete binding (matches historical hard-wire).
/// Default stays in `handle_editor_input`. When remapped, bare Backspace is ignored and the chord
/// deletes backward (same as Backspace in the editor).
pub const DEFAULT_DELETE_BACKWARD: &str = "Backspace";

/// Default insert-newline binding (matches historical hard-wire).
/// Default stays in `handle_editor_input`. When remapped, bare Enter is ignored and the chord
/// inserts the preferred EOL (`settings.default_eol`). `shortcut_blank_line` (Cmd+Enter) stays separate.
pub const DEFAULT_ENTER: &str = "Enter";

/// Default line-start binding (matches historical hard-wire).
/// Opposite Home/End jumps to line end; non-Home/End remaps use Shift for end.
/// Default stays in `handle_editor_input`. When remapped, bare Home/End no longer move on the line;
/// `Cmd+Home` / `Cmd+End` document aliases use `shortcut_document_home`.
pub const DEFAULT_LINE_HOME: &str = "Home";

/// Default document-start Home alias (matches historical hard-wire).
/// Opposite Home/End jumps to document end; non-Home/End remaps use Shift for end.
/// Default stays in `handle_editor_input`. When remapped, bare `Cmd+Home` / `Cmd+End` no longer
/// jump (`shortcut_goto_document_start` / Cmd+Up stays separate).
pub const DEFAULT_DOCUMENT_HOME: &str = "Cmd+Home";

/// Default page-up binding (matches historical hard-wire).
/// Opposite PageUp/PageDown jumps page down; non-Page remaps use Shift for page down.
/// Default stays in `handle_editor_input`. When remapped, bare PageUp/PageDown no longer page.
pub const DEFAULT_PAGE_UP: &str = "PageUp";

/// Default character-left binding (matches historical hard-wire).
/// Opposite Left/Right jumps one character right; non-arrow remaps use Shift for right.
/// Default stays in `handle_editor_input`. When remapped, bare Left/Right no longer move by char
/// (word jump via `shortcut_word_jump` stays separate).
pub const DEFAULT_CHAR_LEFT: &str = "Left";

/// Default line-up binding (matches historical hard-wire).
/// Opposite Up/Down jumps one line down; non-arrow remaps use Shift for down.
/// Default stays in `handle_editor_input`. When remapped, bare Up/Down no longer move by line
/// (`shortcut_move_line` and `shortcut_goto_document_start` stay separate).
pub const DEFAULT_LINE_UP: &str = "Up";

/// Default undo binding (matches historical hard-wire). Shift flips to redo.
pub const DEFAULT_UNDO: &str = "Cmd+Z";

/// Default redo alternate binding (matches historical hard-wire). Shift+undo also redo.
pub const DEFAULT_REDO: &str = "Cmd+Y";

/// Default zoom-in binding (matches historical hard-wire).
pub const DEFAULT_ZOOM_IN: &str = "Cmd+=";

/// Default zoom-out binding (matches historical hard-wire).
pub const DEFAULT_ZOOM_OUT: &str = "Cmd+-";

/// Default zoom-restore binding (matches historical hard-wire). Mouse wheel stays hard-wired.
pub const DEFAULT_ZOOM_RESTORE: &str = "Cmd+0";

/// Default toggle-log-tail binding (matches historical hard-wire).
pub const DEFAULT_TOGGLE_LOG_TAIL: &str = "Cmd+Shift+T";

/// Default Document Map toggle binding (View → Document Map).
pub const DEFAULT_DOCUMENT_MAP: &str = "Cmd+Shift+D";

/// Default Function List toggle binding (View → Function List).
/// Shift flips to Character Panel. Uses Cmd+E so it does not collide with
/// Document Map (Cmd+Shift+D), Expand All Unchanged (Alt+E), or Remove Empty
/// Lines (Cmd+Alt+E).
pub const DEFAULT_FUNCTION_LIST: &str = "Cmd+E";

/// Default focus-other-view binding (View → Focus on Another View).
/// Shift flips to swap the active tab with the other view (header Switch).
pub const DEFAULT_FOCUS_OTHER_VIEW: &str = "F6";

/// Default move-to-other-view binding (View → Move Current Document to Other View).
/// Shift flips to Clone to Other View.
pub const DEFAULT_MOVE_OTHER_VIEW: &str = "Alt+O";

/// Default move-to-new-instance binding (View → Move to New Instance).
/// Shift flips to Open in New Instance (keeps this tab).
/// Uses F9 so it does not collide with Find characters in range (`Alt+F9`),
/// UTF-8 (`Cmd+F9`), read-only for all (`F10`), fullscreen (`F11`), or Post-It (`F12`).
pub const DEFAULT_NEW_INSTANCE: &str = "F9";

/// Default RTL text-direction binding (Edit → RTL).
/// Shift flips to LTR.
pub const DEFAULT_RTL: &str = "Alt+R";

/// Default fullscreen toggle binding (View → Toggle Full Screen Mode).
pub const DEFAULT_FULLSCREEN: &str = "F11";

/// Default Post-It / distraction-free toggle binding (View → Post-It).
pub const DEFAULT_POSTIT: &str = "F12";

/// Default always-on-top toggle binding (View → Always on Top).
pub const DEFAULT_ALWAYS_ON_TOP: &str = "Alt+T";

/// Default show-all-characters toggle binding (View → Show All Characters).
pub const DEFAULT_SHOW_ALL_CHARACTERS: &str = "Alt+A";

/// Default show-whitespace toggle binding (View → Show White Space and TAB).
/// Shift + same key is Show EOL.
pub const DEFAULT_SHOW_WHITESPACE: &str = "Alt+V";

/// Default show-NPC toggle binding (View → Show Non-Printing Characters).
pub const DEFAULT_SHOW_NPC: &str = "Alt+N";

/// Default sync-vertical-scroll toggle binding (View → Synchronize Vertical Scrolling).
/// Shift + same key is Synchronize Horizontal Scrolling.
pub const DEFAULT_SYNC_SCROLL: &str = "Alt+Q";

/// Default Zoom Sync toggle binding (View → Zoom Sync; dual-view shared font size).
/// Uses Alt+Shift+Z so it stays near word wrap (Alt+Z) without colliding with Sync H (Alt+Shift+Q).
pub const DEFAULT_ZOOM_SYNC: &str = "Alt+Shift+Z";

/// Default indent-guide toggle binding (View → Show Indent Guide).
pub const DEFAULT_INDENT_GUIDE: &str = "Alt+I";

/// Default show-line-numbers toggle binding (View → Show Line Numbers).
pub const DEFAULT_SHOW_LINE_NUMBERS: &str = "Alt+L";

/// Default Hide Lines binding (View → Hide Lines).
/// Uses Alt+Shift+L so it does not collide with Show Line Numbers (`Alt+L`),
/// Go to line (`Cmd+L`), Delete line (`Cmd+Shift+L`), or Open Containing Folder
/// (`Cmd+Alt+L`; Shift = shell).
pub const DEFAULT_HIDE_LINES: &str = "Alt+Shift+L";

/// Default Show Wrap Symbol toggle (View → Show Wrap Symbol).
/// Uses Alt+Shift+G so it does not collide with Show Fold Margin (`Alt+G`),
/// Open Folder as Workspace (`Cmd+Alt+G`; Shift = containing folder), or
/// word wrap (`Alt+Z`) / Zoom Sync (`Alt+Shift+Z`).
pub const DEFAULT_SHOW_WRAP_SYMBOL: &str = "Alt+Shift+G";

/// Default show-fold-margin toggle binding (View → Show Fold Margin).
pub const DEFAULT_SHOW_FOLD_MARGIN: &str = "Alt+G";

/// Default project-panel toggle binding (View → Project Panel).
pub const DEFAULT_PROJECT_PANEL: &str = "Alt+P";

/// Default pin-tab toggle binding (tab chrome / `IDM_PINTAB`).
/// Uses Alt+Shift+P so it does not collide with Project Panel (Alt+P).
pub const DEFAULT_PIN_TAB: &str = "Alt+Shift+P";

/// Default reload-from-disk binding (new remappable chord; File → Reload).
pub const DEFAULT_RELOAD: &str = "Cmd+R";

/// Default Run binding (Run → Run…).
/// Opens a file picker to spawn a program, or opens a shell in the active file’s
/// folder when cancelled. Uses `Cmd+Shift+R` so it stays near Reload (`Cmd+R`)
/// without taking that chord, Toggle Read-Only (`Cmd+Alt+R`), or RTL (`Alt+R`).
pub const DEFAULT_RUN: &str = "Cmd+Shift+R";

/// Default Open in Default Viewer binding (File → Open in Default Viewer).
/// Uses `Cmd+Shift+O` so it stays near Open (`Cmd+O`) without taking that chord,
/// Move/Clone Other View (`Alt+O`), or Sort Lines (`Cmd+Alt+O`).
pub const DEFAULT_OPEN_VIEWER: &str = "Cmd+Shift+O";

/// Default About binding (? → About npp-rs).
/// Uses bare `F1` (N++ Help family) so it does not collide with Summary (`Alt+F1`),
/// Load Session (`Cmd+F1`), or Sort tabs by name (`Cmd+Alt+F1`).
pub const DEFAULT_ABOUT: &str = "F1";

/// Default Move to Recycle Bin binding (File → Move to Recycle Bin).
/// Uses `Cmd+Shift+Delete` so it stays near forward Delete without taking that
/// bare key.
pub const DEFAULT_DELETE_FILE: &str = "Cmd+Shift+Delete";

/// Default next-tab binding (new remappable chord). Shift flips to previous tab.
pub const DEFAULT_NEXT_TAB: &str = "Cmd+Tab";

/// Default first-tab binding (View → First Tab).
/// Opposite Home/End jumps to Last Tab; letter remaps use Shift for last.
/// Uses Cmd+Alt+Home so it does not collide with Document Home (Cmd+Home),
/// line Home, or Next tab (Cmd+Tab).
pub const DEFAULT_FIRST_TAB: &str = "Cmd+Alt+Home";

/// Default 1st-tab binding (View → Tab → 1st Tab). Digit-family: the same
/// modifiers with `1`…`9` jump to that tab. Letter remaps jump to the 1st tab only.
/// Uses `Cmd+Shift+1` so it does not collide with Mark Style (`Cmd+1`…`5`),
/// Fold Level (`Alt+1`…`8`), Next Style (`Cmd+Alt+1`…`5`), Mark current line
/// (`Alt+Shift+1`…`5`), Copy Styled Text (`Cmd+Alt+Shift+1`…`5`), or Clear all
/// styles (`Cmd+Shift+0`).
pub const DEFAULT_GOTO_TAB: &str = "Cmd+Shift+1";

/// Default move-tab-forward binding (View → Move Current Tab Forward).
/// Opposite PageUp/PageDown moves backward; letter remaps use Shift for backward.
/// Uses Cmd+Shift+PageDown so it does not collide with Next tab (Cmd+Tab), bare
/// PageUp/PageDown, or Move line (Cmd+Shift+Up).
pub const DEFAULT_MOVE_TAB: &str = "Cmd+Shift+PageDown";

/// Default save-all binding (new remappable chord; File → Save All).
/// Uses Cmd+Alt+S so it does not collide with Save As (Cmd+Shift+S).
pub const DEFAULT_SAVE_ALL: &str = "Cmd+Alt+S";

/// Default print binding (new remappable chord; File → Print).
pub const DEFAULT_PRINT: &str = "Cmd+P";

/// Default exit binding (File → Exit).
/// Uses Cmd+Q, left free by Toggle Comment (`Cmd+/`), so it does not collide with
/// Paste Bookmarked Lines (`Cmd+Alt+Q`) or Sync Vertical Scroll (`Alt+Q`).
pub const DEFAULT_EXIT: &str = "Cmd+Q";

/// Default load-session binding (File → Load Session).
/// Uses Cmd+F1 so it does not collide with About (`F1`), Summary (`Alt+F1`),
/// Find (`Cmd+F`), or Open (`Cmd+O`). Shift flips to Save Session (`Cmd+Shift+F1`).
pub const DEFAULT_LOAD_SESSION: &str = "Cmd+F1";

/// Default sort-tabs-by-name binding (Window → Sort by name ascending).
/// Uses Cmd+Alt+F1 so it does not collide with Load Session (`Cmd+F1`) or
/// Summary (`Alt+F1`). Shift flips to descending.
pub const DEFAULT_SORT_TABS: &str = "Cmd+Alt+F1";

/// Default sort-tabs-by-path binding (Window → Path A to Z).
/// Uses Cmd+Alt+, so it does not collide with Sort by name (`Cmd+Alt+F1`)
/// or Preferences (`Cmd+,`). Shift flips to Path Z to A.
pub const DEFAULT_SORT_TABS_PATH: &str = "Cmd+Alt+,";

/// Default sort-tabs-by-type binding (Window → Type A to Z).
/// Uses Cmd+Alt+. so it does not collide with Sort by path (`Cmd+Alt+,`)
/// or Preferences (`Cmd+,`). Shift flips to Type Z to A.
pub const DEFAULT_SORT_TABS_TYPE: &str = "Cmd+Alt+.";

/// Default Sort tabs by size ascending binding (Window menu).
/// Shift flips to descending. Uses `Cmd+Alt+/` so it stays near type (`.`)
/// without colliding with Toggle comment (`Cmd+/`).
pub const DEFAULT_SORT_TABS_SIZE: &str = "Cmd+Alt+/";

/// Default Sort tabs by modified time ascending binding (Window menu).
/// Shift flips to descending. Uses `Cmd+Alt+;` so it stays next to size (`/`)
/// without colliding with that chord or Toggle comment (`Cmd+/`).
pub const DEFAULT_SORT_TABS_MODIFIED: &str = "Cmd+Alt+;";

/// Default Tab indent binding (editor Tab arm; inserts spaces at caret / multi-carets).
pub const DEFAULT_TAB_INDENT: &str = "Tab";

/// Default Shift+Tab outdent binding (editor Tab arm; remappable alternate to Cmd+[).
pub const DEFAULT_SHIFT_TAB_OUTDENT: &str = "Shift+Tab";

/// Default fold-all binding (View → Fold All). Shift flips to Unfold All.
pub const DEFAULT_FOLD_ALL: &str = "Alt+0";

/// Default fold-level binding (View → Fold Level 1…8). Digit-family: same
/// modifiers with `1`…`8` fold that level; Shift flips to Unfold Level.
/// `Alt+0` stays Fold All (`shortcut_fold_all`).
pub const DEFAULT_FOLD_LEVEL: &str = "Alt+1";

/// Default fold-current binding (View → Fold Current). Shift flips to Unfold Current.
pub const DEFAULT_FOLD_CURRENT: &str = "Alt+F";

/// Default go-to-matching-brace binding (Search → Go to matching brace).
/// Shift flips to Select matching braces.
pub const DEFAULT_MATCHING_BRACE: &str = "Cmd+B";

/// Default move-line-up binding (Edit → Move Up Current Line).
/// Opposite Up/Down moves the line down; non-arrow remaps use Shift for down.
pub const DEFAULT_MOVE_LINE: &str = "Cmd+Shift+Up";

/// Default go-to-document-start binding (View → Go to start of file).
/// Opposite Up/Down jumps to end; non-arrow remaps use Shift for end.
/// `Cmd+Home` / `Cmd+End` aliases use `shortcut_document_home`.
pub const DEFAULT_GOTO_DOCUMENT_START: &str = "Cmd+Up";

/// Default toggle single-line comment binding (Edit → Toggle Single Line Comment).
/// Shift flips to Block Comment. Uses `/` (not `Q`) so macOS Cmd+Q quit stays free.
pub const DEFAULT_TOGGLE_COMMENT: &str = "Cmd+/";

/// Default force single-line comment binding (Edit → Single Line Comment).
/// Shift flips to Single Line Uncomment. Uses Alt+/ so it does not collide with
/// Toggle / Block Comment (`Cmd+/` / `Cmd+Shift+/`) or Sort tabs by size
/// (`Cmd+Alt+/` / `Cmd+Alt+Shift+/`).
pub const DEFAULT_LINE_COMMENT: &str = "Alt+/";

/// Default block (stream) comment binding (Edit → Block Comment).
/// Shift flips to Block Uncomment. Uses Alt+, so it does not collide with
/// Toggle / Block via toggle (`Cmd+/` / `Cmd+Shift+/`), Single Line Comment
/// (`Alt+/` / `Alt+Shift+/`), Preferences (`Cmd+,`), or Sort tabs by path
/// (`Cmd+Alt+,` / `Cmd+Alt+Shift+,`).
pub const DEFAULT_STREAM_COMMENT: &str = "Alt+,";

/// Default lowercase binding (Edit → lowercase). Shift flips to UPPERCASE (N++ Ctrl+U family).
pub const DEFAULT_LOWERCASE: &str = "Cmd+U";

/// Default autocomplete binding (Edit → Function Completion File).
/// Shift flips to Function Call Tip (N++ Ctrl+Space / Ctrl+Shift+Space family).
pub const DEFAULT_AUTOCOMPLETE: &str = "Cmd+Space";

/// Default path-autocomplete binding (Edit → Function Completion Path).
/// N++ Ctrl+Alt+Space family; remappable via `shortcut_autocomplete_path`.
pub const DEFAULT_AUTOCOMPLETE_PATH: &str = "Cmd+Alt+Space";

/// Default next function-parameter hint (Edit → Function Parameters Next Hint).
/// Opposite Up is previous. Uses Alt+Down so it does not collide with Move Line
/// (`Cmd+Shift+Up`/`Down`), document start (`Cmd+Up`), line motion (plain arrows),
/// rect select (`Alt+Shift+arrows`), or word jump (`Alt+Left`/`Right`).
/// Letter remaps use Shift for the previous hint.
pub const DEFAULT_CALLTIP_NAV: &str = "Alt+Down";

/// Default join-lines binding (Edit → Join Lines). Shift flips to Split Lines (N++ Ctrl+J family).
pub const DEFAULT_JOIN_LINES: &str = "Cmd+J";

/// Default invert-case binding (Edit → iNVERT cASE). Shift flips to Proper Case.
pub const DEFAULT_INVERT_CASE: &str = "Cmd+Alt+U";

/// Default Sentence Case Force binding (Edit → Convert Case to → Sentence case).
/// Shift flips to Sentence Case Blend. Uses Cmd+Alt+8 so it stays near Sort Lines Locale
/// (`Cmd+Alt+7`) without colliding with Next Style (`Cmd+Alt+1`…`5`), Invert Case
/// (`Cmd+Alt+U`), or Fold Level (`Alt+1`…`8`).
pub const DEFAULT_SENTENCE_CASE: &str = "Cmd+Alt+8";

/// Default Proper Case Force binding (Edit → Convert Case to → Proper Case).
/// Shift flips to Proper Case Blend. Uses Cmd+Alt+9 so it stays near Sentence Case
/// (`Cmd+Alt+8`) without colliding with Next Style (`Cmd+Alt+1`…`5`), Invert Case
/// (`Cmd+Alt+U`; Shift still aliases Proper Case Force), or Fold Level (`Alt+1`…`8`).
pub const DEFAULT_PROPER_CASE: &str = "Cmd+Alt+9";

/// Default Random Case binding (Edit → Convert Case to → Random Case).
/// Uses Cmd+Alt+0 so it stays near Proper Case (`Cmd+Alt+9`) without colliding with
/// Fold All (`Alt+0`), Copy Styled Text all (`Cmd+Alt+Shift+0`), Next Style
/// (`Cmd+Alt+1`…`5`), or Fold Level (`Alt+1`…`8`).
pub const DEFAULT_RANDOM_CASE: &str = "Cmd+Alt+0";

/// Default Redact Selection binding (Edit → On Selection → Redact Selection █).
/// Shift flips to ● fill. Uses Alt+J so it does not collide with Join Lines (`Cmd+J`),
/// RTL (`Alt+R`), Toggle Read-Only (`Cmd+Alt+R`), or Invert Case (`Cmd+Alt+U`).
pub const DEFAULT_REDACT: &str = "Alt+J";

/// Default Search on Internet binding (Edit → Search on Internet).
/// Shift flips to Change Search Engine. Uses Alt+U so it does not collide with Invert Case
/// (`Cmd+Alt+U`), lowercase (`Cmd+U`), or Redact Selection (`Alt+J`).
pub const DEFAULT_SEARCH_INTERNET: &str = "Alt+U";

/// Default blank-line-below binding (Edit → Insert Empty Line Below Current).
/// Shift flips to Insert Empty Line Above Current.
pub const DEFAULT_BLANK_LINE: &str = "Cmd+Enter";

/// Default multi-select-next binding (Edit → Multi-select Next).
/// Shift flips to Skip Current / Go to Next Multi-select.
pub const DEFAULT_MULTI_SELECT_NEXT: &str = "Cmd+Alt+N";

/// Default multi-select-all binding (Edit → Multi-select All).
/// Shift flips to Multi-select All (Whole Word).
pub const DEFAULT_MULTI_SELECT_ALL: &str = "Cmd+Alt+A";

/// Default undo-latest-multi-select binding (Edit → Undo the Latest Added Multi-Select).
/// Uses Cmd+Alt+Backspace so it stays on the multi-caret family without colliding with
/// Undo (`Cmd+Z`), Multi-select next (`Cmd+Alt+N`), Multi-select all (`Cmd+Alt+A`),
/// or bare Backspace (delete backward).
pub const DEFAULT_MULTI_SELECT_UNDO: &str = "Cmd+Alt+Backspace";

/// Default Begin/End Select binding (Edit → Begin/End Select).
/// Shift flips to Begin/End Select in Column Mode.
pub const DEFAULT_BEGIN_END_SELECT: &str = "Cmd+Alt+B";

/// Default toggle read-only binding (Edit → Read-Only).
/// Uses Cmd+Alt+R so it does not collide with Reload (Cmd+R) or RTL (Alt+R).
pub const DEFAULT_TOGGLE_READONLY: &str = "Cmd+Alt+R";

/// Default read-only-for-all-documents binding (Edit → Set Read-Only for All Documents).
/// Shift flips to Clear Read-Only for All Documents. Uses bare F10 so it does not
/// collide with Toggle Read-Only (`Cmd+Alt+R`), RTL (`Alt+R`; Shift = LTR), Reload
/// (`Cmd+R`), or Sort Lines by Length (`Cmd+Alt+F10` / `Cmd+Alt+Shift+F10`).
pub const DEFAULT_READONLY_ALL: &str = "F10";

/// Default Summary binding (View → Summary).
/// Shift flips to Document List. Uses Alt+F1 so it does not collide with
/// About (`F1`), Find characters in range (`Alt+F9`), Read-only for all documents
/// (`F10`), or Sort Lines by Length (`Cmd+Alt+F10`).
pub const DEFAULT_SUMMARY: &str = "Alt+F1";

/// Default Preferences binding (Settings → Preferences).
/// Shift flips to Style Configurator. Uses Cmd+, so it does not collide with
/// Summary (`Alt+F1`), Print (`Cmd+P`), or Project Panel (`Alt+P`).
pub const DEFAULT_PREFERENCES: &str = "Cmd+,";

/// Default Shortcut Mapper binding (Settings → Shortcut Mapper).
/// Uses Alt+Shift+K so it does not collide with Copy Compare Hunk (`Alt+K`),
/// Copy Current Directory Path (`Cmd+Alt+K`), or Preferences (`Cmd+,`).
pub const DEFAULT_SHORTCUT_MAPPER: &str = "Alt+Shift+K";

/// Default Tab colour 1 binding (View → Apply Color 1).
/// Shift clears the colour. Uses Cmd+F5 so it does not collide with Date Time
/// (`F5` / `Shift+F5`), Sort Lines Integer (`Cmd+Alt+F5`), or Find Copy
/// (`Alt+Shift+F5`).
pub const DEFAULT_TAB_COLOUR: &str = "Cmd+F5";

/// Default Tab colour 2 binding (View → Apply Color 2).
/// Shift clears the colour. Uses Cmd+F4 so it stays next to Tab colour 1
/// (`Cmd+F5` / `Cmd+Shift+F5`) without taking Next/Prev Search Result
/// (`F4` / `Shift+F4`), Find All (`Cmd+Alt+F4` / `Cmd+Alt+Shift+F4`), or
/// Find Bookmark (`Alt+Shift+F4` / `Alt+F4`).
pub const DEFAULT_TAB_COLOUR_2: &str = "Cmd+F4";

/// Default Tab colour 3 binding (View → Apply Color 3).
/// Shift clears the colour. Uses Cmd+K so it stays near Shortcut Mapper
/// (`Alt+Shift+K`) and Copy Current Directory (`Cmd+Alt+K`) without taking
/// Copy Compare Hunk (`Alt+K`), Tab colour 1/2 (`Cmd+F5` / `Cmd+F4`), or
/// Clear Change History (`Cmd+Shift+F8`).
pub const DEFAULT_TAB_COLOUR_3: &str = "Cmd+K";

/// Default Tab colour 4 binding (View → Apply Color 4).
/// Shift clears the colour. Uses Cmd+M so it stays near Tab colour 3 (`Cmd+K`)
/// without taking Bookmark Compare Diffs (`Alt+M`), Copy All Paths (`Cmd+Alt+M`),
/// or Tab colour 1/2/3 (`Cmd+F5` / `Cmd+F4` / `Cmd+K`).
pub const DEFAULT_TAB_COLOUR_4: &str = "Cmd+M";

/// Default Tab colour 5 binding (View → Apply Color 5).
/// Shift clears the colour. Uses `Cmd+;` so it stays near Sort tabs by modified
/// (`Cmd+Alt+;`) without taking that chord, Tab colour 4 (`Cmd+M`), or
/// Preferences (`Cmd+,`).
pub const DEFAULT_TAB_COLOUR_5: &str = "Cmd+;";

/// Default Clipboard History binding (Edit → Clipboard History).
/// Shows the last copied entry in the status line (1-entry MVP). Uses `Cmd+Shift+C`
/// so it stays near Copy (`Cmd+C`) without taking that chord, Column Editor
/// (`Cmd+Alt+C`), or Copy Compare Diff (`Alt+C`).
pub const DEFAULT_CLIPBOARD_HISTORY: &str = "Cmd+Shift+C";

/// Default Column Editor binding (Edit → Column Editor).
/// Uses Cmd+Alt+C so it does not collide with Copy (Cmd+C) or Copy Compare Diff (Alt+C).
pub const DEFAULT_COLUMN_EDITOR: &str = "Cmd+Alt+C";

/// Default Sort Lines Lexicographic Ascending binding.
/// Uses Cmd+Alt+O so it does not collide with Open (Cmd+O) or Move/Clone Other View (Alt+O).
/// Shift flips to Lexicographic Descending.
pub const DEFAULT_SORT_LINES: &str = "Cmd+Alt+O";

/// Default Sort Lines Integer Ascending binding (Edit → Line Operations).
/// Shift flips to Integer Descending. Uses Cmd+Alt+F5 so it stays near Sort Lines
/// (`Cmd+Alt+O`) without colliding with Date Time (`F5`) or Find Copy (`Alt+Shift+F5`).
pub const DEFAULT_SORT_LINES_INTEGER: &str = "Cmd+Alt+F5";

/// Default Sort Lines Lexicographic Ascending Ignoring Case binding (Edit → Line Operations).
/// Shift flips to Descending Ignoring Case. Uses Cmd+Alt+F6 so it stays near Integer
/// sort (`Cmd+Alt+F5`) without colliding with Focus Other View (`F6`) or Find Cut
/// (`Alt+Shift+F6`).
pub const DEFAULT_SORT_LINES_CASE_INSENSITIVE: &str = "Cmd+Alt+F6";

/// Default Sort Lines Decimal Dot Ascending binding (Edit → Line Operations).
/// Shift flips to Decimal Dot Descending. Uses Cmd+Alt+F8 so it stays near Integer /
/// Ignoring Case sort (`Cmd+Alt+F5` / `Cmd+Alt+F6`) without colliding with Next Change
/// (`F8`), Find Purge (`Alt+Shift+F8`), or Clear Change History (`Cmd+Shift+F8`).
pub const DEFAULT_SORT_LINES_DECIMAL_DOT: &str = "Cmd+Alt+F8";

/// Default Sort Lines Decimal Comma Ascending binding (Edit → Line Operations).
/// Shift flips to Decimal Comma Descending. Uses Cmd+Alt+F9 so it stays near Decimal Dot
/// sort (`Cmd+Alt+F8`) without colliding with Find characters in range (`Alt+F9`).
pub const DEFAULT_SORT_LINES_DECIMAL_COMMA: &str = "Cmd+Alt+F9";

/// Default Sort Lines by Length Ascending binding (Edit → Line Operations).
/// Shift flips to Length Descending. Uses Cmd+Alt+F10 so it stays near Decimal Comma
/// sort (`Cmd+Alt+F9`) without colliding with First Hidden Equal (`Cmd+Alt+F7`).
pub const DEFAULT_SORT_LINES_LENGTH: &str = "Cmd+Alt+F10";

/// Default Sort Lines Reverse Order binding (Edit → Line Operations).
/// Shift flips to Sort Lines Randomly. Uses Cmd+Alt+6 so it stays near Length sort
/// (`Cmd+Alt+F10`) without colliding with Next Style (`Cmd+Alt+1`…`5`) or Fold Level
/// (`Alt+1`…`8`).
pub const DEFAULT_SORT_LINES_REVERSE: &str = "Cmd+Alt+6";

/// Default Sort Lines Locale Ascending binding (Edit → Line Operations).
/// Shift flips to Locale Descending. Uses Cmd+Alt+7 so it stays near Reverse sort
/// (`Cmd+Alt+6`) without colliding with Next Style (`Cmd+Alt+1`…`5`) or Fold Level
/// (`Alt+1`…`8`).
pub const DEFAULT_SORT_LINES_LOCALE: &str = "Cmd+Alt+7";

/// Default EOL to Space binding (Edit → Blank Operations → EOL to Space).
/// Shift flips to Trim Both and EOL to Space (`IDM_EDIT_TRIMALL`). Uses Cmd+Alt+F11 so it
/// stays near Sort Lines by Length (`Cmd+Alt+F10`) and Trim Trailing (`Cmd+Alt+T`) without
/// colliding with Fullscreen (`F11`) or Post-It (`F12`).
pub const DEFAULT_EOL_TO_SPACE: &str = "Cmd+Alt+F11";

/// Default Convert to Unix (LF) EOL binding (Format → Convert to Unix).
/// Shift flips to Convert to Windows (CRLF). Uses Cmd+Alt+F12 so it stays near EOL to Space
/// (`Cmd+Alt+F11`) without colliding with Post-It (`F12`) or EOL to Space (`Cmd+Alt+F11`).
/// Macintosh (CR) uses `shortcut_convert_eol_mac` / `DEFAULT_CONVERT_EOL_MAC`.
pub const DEFAULT_CONVERT_EOL: &str = "Cmd+Alt+F12";

/// Default Convert to Macintosh (CR) EOL binding (Format → Convert to Macintosh).
/// Uses Alt+Shift+F12 so it stays near Convert EOL Unix/Windows (`Cmd+Alt+F12`) without
/// colliding with Post-It (`F12`), Convert EOL + Shift (`Cmd+Alt+Shift+F12`), or Find Purge
/// (`Alt+Shift+F8`).
pub const DEFAULT_CONVERT_EOL_MAC: &str = "Alt+Shift+F12";

/// Default UTF-8 (no BOM) binding (Encoding → UTF-8).
/// Shift flips to UTF-8-BOM. Uses Cmd+F9 so it stays near Find characters in range
/// (`Alt+F9`) and Sort Lines Decimal Comma (`Cmd+Alt+F9`) without taking either chord.
pub const DEFAULT_ENCODING_UTF8: &str = "Cmd+F9";

/// Default UTF-16 LE BOM binding (Encoding → UTF-16 LE BOM).
/// Shift flips to UTF-16 BE BOM. Uses Cmd+F10 so it stays off Read-only for all
/// (`F10` / Shift+F10) and Sort Lines by Length (`Cmd+Alt+F10` / `Cmd+Alt+Shift+F10`).
pub const DEFAULT_ENCODING_UTF16: &str = "Cmd+F10";

/// Default ANSI (Windows-1252) binding (Encoding → ANSI).
/// Uses Cmd+F11 so it stays next to UTF-8 (`Cmd+F9`) and UTF-16 (`Cmd+F10`)
/// without taking Fullscreen (`F11`) or EOL to Space (`Cmd+Alt+F11`).
pub const DEFAULT_ENCODING_ANSI: &str = "Cmd+F11";

/// Default MD5 generate binding (Tools → MD5 → Generate).
/// Shift flips to Generate from selection into clipboard. Uses Alt+F10 so it
/// stays off Read-only for all (`F10` / Shift+F10), UTF-16 (`Cmd+F10` /
/// `Cmd+Shift+F10`), and Sort Lines by Length (`Cmd+Alt+F10` / `Cmd+Alt+Shift+F10`).
pub const DEFAULT_MD5: &str = "Alt+F10";

/// Default SHA-256 generate binding (Tools → SHA-256 → Generate).
/// Shift flips to Generate from selection into clipboard. Uses Alt+F11 so it
/// stays next to MD5 (`Alt+F10`) without taking Fullscreen (`F11`), ANSI
/// (`Cmd+F11`), or EOL to Space (`Cmd+Alt+F11`).
pub const DEFAULT_SHA256: &str = "Alt+F11";

/// Default SHA-1 generate binding (Tools → SHA-1 → Generate).
/// Shift flips to Generate from selection into clipboard. Uses Cmd+F12 so it
/// stays next to the hash family without taking Post-It (`F12`), Convert EOL
/// Macintosh (`Alt+Shift+F12`), or Convert EOL Unix/Windows (`Cmd+Alt+F12`).
pub const DEFAULT_SHA1: &str = "Cmd+F12";

/// Default SHA-512 generate binding (Tools → SHA-512 → Generate).
/// Shift flips to Generate from selection into clipboard. Uses Cmd+F6 so it
/// stays off Focus on Another View (`F6` / Shift+F6), Find Cut
/// (`Alt+Shift+F6` / `Alt+F6`), and Sort Lines Lex Ignoring Case
/// (`Cmd+Alt+F6` / `Cmd+Alt+Shift+F6`).
pub const DEFAULT_SHA512: &str = "Cmd+F6";

/// Default Remove Empty Lines binding (Edit → Line Operations → Remove Empty Lines).
/// Uses Cmd+Alt+E so it does not collide with Expand All Unchanged (Alt+E).
/// Shift flips to Remove Empty Lines (Containing Blank Characters).
pub const DEFAULT_REMOVE_EMPTY_LINES: &str = "Cmd+Alt+E";

/// Default Remove Consecutive Duplicate Lines binding (Edit → Line Operations).
/// Uses Cmd+Alt+D so it does not collide with Duplicate Line (Cmd+D).
/// Shift flips to Remove Duplicate Lines (any occurrence).
pub const DEFAULT_REMOVE_DUP_LINES: &str = "Cmd+Alt+D";

/// Default Date Time (short) binding (Edit → Insert; N++ F5 family).
/// Shift flips to Date Time (long). Customized uses `DEFAULT_INSERT_DATETIME_CUSTOM`.
pub const DEFAULT_INSERT_DATETIME: &str = "F5";

/// Default Date Time (customized) binding (Edit → Insert).
/// Uses Cmd+Alt+= so it does not collide with Date Time short/long (`F5` / `Shift+F5`),
/// Zoom in (`Cmd+=`), Find Copy lines (`Alt+F5`), Tab colour 1 (`Cmd+F5`),
/// or Sort Lines Integer (`Cmd+Alt+F5`).
pub const DEFAULT_INSERT_DATETIME_CUSTOM: &str = "Cmd+Alt+=";

/// Default Macro Playback binding (Macro → Playback).
/// Uses Cmd+Shift+P (N++ playback) so it does not collide with Print (`Cmd+P`),
/// Copy Full Path (`Cmd+Alt+P`), Project Panel (`Alt+P`), or Pin tab (`Alt+Shift+P`).
/// Shift is part of the chord, not a flip (a flip would be Print).
pub const DEFAULT_MACRO_PLAYBACK: &str = "Cmd+Shift+P";

/// Default Start/Stop Recording binding (Macro → Start Recording / Stop Recording).
/// Same chord toggles: start when idle, stop while recording (N++ shares one accelerator).
/// Uses Cmd+Alt+Shift+R so it does not collide with Reload (`Cmd+R`), Run (`Cmd+Shift+R`),
/// Toggle Read-Only (`Cmd+Alt+R`), or RTL/LTR (`Alt+R` / `Alt+Shift+R`).
/// Shift is part of the chord, not a flip.
pub const DEFAULT_MACRO_RECORD: &str = "Cmd+Alt+Shift+R";

/// Default Save Current Recorded Macro binding (Macro → Save Current Recorded Macro).
/// Uses Cmd+Alt+Shift+S so it does not collide with Save (`Cmd+S`), Save As
/// (`Cmd+Shift+S`), Save All (`Cmd+Alt+S`), Swap Compare (`Alt+S`), or Compare
/// to Saved (`Alt+Shift+S`). Shift is part of the chord, not a flip.
pub const DEFAULT_MACRO_SAVE: &str = "Cmd+Alt+Shift+S";

/// Default Run a Macro Multiple Times binding (Macro → Run a Macro Multiple Times).
/// Uses Cmd+Alt+Shift+K so it does not collide with Copy Current Directory
/// (`Cmd+Alt+K`), Copy Compare Hunk (`Alt+K`), Shortcut Mapper (`Alt+Shift+K`),
/// or Tab colour 3 (`Cmd+K`). Shift is part of the chord, not a flip.
pub const DEFAULT_MACRO_REPEAT: &str = "Cmd+Alt+Shift+K";

/// Default Copy Full Path to Clipboard binding (Edit → Copy to Clipboard).
/// Uses Cmd+Alt+P so it does not collide with Print (Cmd+P), Project Panel (Alt+P),
/// or Pin tab (Alt+Shift+P). Shift flips to Copy File Name.
pub const DEFAULT_COPY_PATH: &str = "Cmd+Alt+P";

/// Default Copy Current Directory Path binding (Edit → Copy to Clipboard).
/// Uses Cmd+Alt+K so it does not collide with Copy Compare Hunk (Alt+K) or
/// Copy Full Path (Cmd+Alt+P).
pub const DEFAULT_COPY_DIR: &str = "Cmd+Alt+K";

/// Default Copy All File Paths to Clipboard binding (Edit → Copy to Clipboard).
/// Uses Cmd+Alt+M so it does not collide with Bookmark Compare Diffs (Alt+M),
/// Copy Full Path (Cmd+Alt+P), or Copy Current Directory (Cmd+Alt+K).
/// Shift flips to Copy All Filenames.
pub const DEFAULT_COPY_ALL_PATHS: &str = "Cmd+Alt+M";

/// Default Open Selected Filename binding (Edit → Open File).
/// Uses Cmd+Alt+J so it does not collide with Join Lines (Cmd+J), Open (Cmd+O),
/// or Sort Lines (Cmd+Alt+O). Shift flips to Open Containing Folder in Explorer.
pub const DEFAULT_OPEN_SELECTED: &str = "Cmd+Alt+J";

/// Default Open Containing Folder (Explorer) binding (File → Open Containing Folder).
/// Uses Cmd+Alt+L so it does not collide with Show Line Numbers (Alt+L), Go to Line
/// (Cmd+L), or Open Selected Filename (Cmd+Alt+J). Shift flips to open a shell here.
pub const DEFAULT_OPEN_FOLDER: &str = "Cmd+Alt+L";

/// Default Open Folder as Workspace binding (File → Open Folder as Workspace).
/// Uses Cmd+Alt+G so it does not collide with Show Fold Margin (Alt+G), Find next
/// (global) (Cmd+G), or Open Containing Folder (Cmd+Alt+L). Shift flips to Open
/// Containing Folder as Workspace.
pub const DEFAULT_OPEN_WORKSPACE: &str = "Cmd+Alt+G";

/// Default TAB → Space binding (Edit → TAB to Space).
/// Uses Cmd+Alt+V so it does not collide with Paste (Cmd+V), Show White Space (Alt+V),
/// or Trim Trailing (Cmd+Alt+T). Shift flips to Space to TAB (Leading).
pub const DEFAULT_TAB_TO_SPACE: &str = "Cmd+Alt+V";

/// Default Space → TAB (All) binding (Edit → Space to TAB (All)).
/// Uses Cmd+Alt+X so it does not collide with Expand Unchanged at Caret (Alt+X),
/// TAB to Space (Cmd+Alt+V), or Close All but Pinned (Cmd+Alt+H).
pub const DEFAULT_SPACE_TO_TAB_ALL: &str = "Cmd+Alt+X";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyChord {
    pub ctrl_or_cmd: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: Key,
}

impl KeyChord {
    pub fn matches(self, mods: Modifiers, pressed: Key) -> bool {
        if pressed != self.key {
            return false;
        }
        let cmd = mods.command || mods.ctrl;
        cmd == self.ctrl_or_cmd && mods.shift == self.shift && mods.alt == self.alt
    }

    /// Match ctrl/alt/key; Shift may differ (word-jump extend selection).
    pub fn matches_ignore_shift(self, mods: Modifiers, pressed: Key) -> bool {
        if pressed != self.key {
            return false;
        }
        let cmd = mods.command || mods.ctrl;
        cmd == self.ctrl_or_cmd && mods.alt == self.alt
    }

    /// Match modifiers only (ctrl/cmd, alt, shift); ignore which key was pressed.
    pub fn matches_mods(self, mods: Modifiers) -> bool {
        let cmd = mods.command || mods.ctrl;
        cmd == self.ctrl_or_cmd && mods.shift == self.shift && mods.alt == self.alt
    }

    /// Direction for remappable rect column select (`shortcut_rect_select`).
    ///
    /// Returns `(char_delta, line_delta)`. Arrow-family chords accept any arrow
    /// with matching modifiers. Letter remaps: base key = left (-1, 0); partner
    /// via [`Self::char_right_chord`] = right (1, 0). Vertical letter remaps are
    /// not supported.
    pub fn rect_select_delta(self, mods: Modifiers, pressed: Key) -> Option<(isize, isize)> {
        let arrow_family = matches!(
            self.key,
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown
        );
        if arrow_family {
            if !self.matches_mods(mods) {
                return None;
            }
            return match pressed {
                Key::ArrowLeft => Some((-1, 0)),
                Key::ArrowRight => Some((1, 0)),
                Key::ArrowUp => Some((0, -1)),
                Key::ArrowDown => Some((0, 1)),
                _ => None,
            };
        }
        if self.matches(mods, pressed) {
            Some((-1, 0))
        } else if self.char_right_chord().matches(mods, pressed) {
            Some((1, 0))
        } else {
            None
        }
    }

    /// Human-readable rect-select binding for Shortcut Mapper / About.
    pub fn rect_select_display(self) -> String {
        let arrow_family = matches!(
            self.key,
            Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown
        );
        if arrow_family {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("Left/Right/Up/Down");
            parts.join("+")
        } else {
            format!("{} / {}", self.display(), self.char_right_chord().display())
        }
    }

    /// Fold level digit `1`…`8` for [`Key::Num1`]…[`Key::Num8`].
    pub fn fold_level_of_key(key: Key) -> Option<usize> {
        match key {
            Key::Num1 => Some(1),
            Key::Num2 => Some(2),
            Key::Num3 => Some(3),
            Key::Num4 => Some(4),
            Key::Num5 => Some(5),
            Key::Num6 => Some(6),
            Key::Num7 => Some(7),
            Key::Num8 => Some(8),
            _ => None,
        }
    }

    /// Key for fold level `1`…`8`.
    pub fn key_for_fold_level(level: usize) -> Option<Key> {
        match level {
            1 => Some(Key::Num1),
            2 => Some(Key::Num2),
            3 => Some(Key::Num3),
            4 => Some(Key::Num4),
            5 => Some(Key::Num5),
            6 => Some(Key::Num6),
            7 => Some(Key::Num7),
            8 => Some(Key::Num8),
            _ => None,
        }
    }

    /// Fold / unfold level for remappable `shortcut_fold_level`.
    ///
    /// Returns `(level, fold)` where `fold` is true to fold and false to unfold.
    /// Digit-family chords (base key `1`…`8`) accept any of those digits with
    /// matching Ctrl/Cmd/Alt; Shift matching the base folds, flipped unfolds.
    /// Letter remaps only drive level 1 (Shift flips to unfold).
    pub fn fold_level_action(self, mods: Modifiers, pressed: Key) -> Option<(usize, bool)> {
        let digit_family = Self::fold_level_of_key(self.key).is_some();
        if digit_family {
            let level = Self::fold_level_of_key(pressed)?;
            let cmd = mods.command || mods.ctrl;
            if cmd != self.ctrl_or_cmd || mods.alt != self.alt {
                return None;
            }
            if mods.shift == self.shift {
                Some((level, true))
            } else {
                Some((level, false))
            }
        } else if self.matches(mods, pressed) {
            Some((1, true))
        } else if self.flipped_shift().matches(mods, pressed) {
            Some((1, false))
        } else {
            None
        }
    }

    /// Human-readable fold-level binding for Shortcut Mapper / About.
    pub fn fold_level_display(self) -> String {
        if Self::fold_level_of_key(self.key).is_some() {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("1…8");
            parts.join("+")
        } else {
            format!("{} / {}", self.display(), self.flipped_shift().display())
        }
    }

    /// Display for one fold/unfold level menu row (digit-family uses that digit).
    pub fn fold_level_display_for(self, level: usize, unfold: bool) -> Option<String> {
        let digit_family = Self::fold_level_of_key(self.key).is_some();
        if digit_family {
            let key = Self::key_for_fold_level(level)?;
            let chord = Self {
                key,
                shift: if unfold { !self.shift } else { self.shift },
                ..self
            };
            Some(chord.display())
        } else if level == 1 {
            Some(if unfold {
                self.flipped_shift().display()
            } else {
                self.display()
            })
        } else {
            None
        }
    }

    /// Tab index `1`…`9` for [`Key::Num1`]…[`Key::Num9`].
    pub fn tab_index_of_key(key: Key) -> Option<usize> {
        match key {
            Key::Num1 => Some(1),
            Key::Num2 => Some(2),
            Key::Num3 => Some(3),
            Key::Num4 => Some(4),
            Key::Num5 => Some(5),
            Key::Num6 => Some(6),
            Key::Num7 => Some(7),
            Key::Num8 => Some(8),
            Key::Num9 => Some(9),
            _ => None,
        }
    }

    /// Key for tab index `1`…`9`.
    pub fn key_for_tab_index(index: usize) -> Option<Key> {
        match index {
            1 => Some(Key::Num1),
            2 => Some(Key::Num2),
            3 => Some(Key::Num3),
            4 => Some(Key::Num4),
            5 => Some(Key::Num5),
            6 => Some(Key::Num6),
            7 => Some(Key::Num7),
            8 => Some(Key::Num8),
            9 => Some(Key::Num9),
            _ => None,
        }
    }

    /// Jump to tab `1`…`9` for remappable `shortcut_goto_tab`.
    ///
    /// Digit-family chords (base key `1`…`9`) accept any of those digits with
    /// matching Ctrl/Cmd/Alt/Shift. Letter remaps jump to the 1st tab only.
    pub fn goto_tab_action(self, mods: Modifiers, pressed: Key) -> Option<usize> {
        let digit_family = Self::tab_index_of_key(self.key).is_some();
        if digit_family {
            let index = Self::tab_index_of_key(pressed)?;
            let cmd = mods.command || mods.ctrl;
            if cmd != self.ctrl_or_cmd || mods.alt != self.alt || mods.shift != self.shift {
                return None;
            }
            Some(index)
        } else if self.matches(mods, pressed) {
            Some(1)
        } else {
            None
        }
    }

    /// Human-readable 1st–9th tab binding for Shortcut Mapper / About.
    pub fn goto_tab_display(self) -> String {
        if Self::tab_index_of_key(self.key).is_some() {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("1…9");
            parts.join("+")
        } else {
            self.display()
        }
    }

    /// Display for one 1st–9th tab menu row (digit-family uses that digit).
    pub fn goto_tab_display_for(self, index: usize) -> Option<String> {
        let digit_family = Self::tab_index_of_key(self.key).is_some();
        if digit_family {
            let key = Self::key_for_tab_index(index)?;
            let chord = Self { key, ..self };
            Some(chord.display())
        } else if index == 1 {
            Some(self.display())
        } else {
            None
        }
    }

    /// Style marker digit `1`…`5` for [`Key::Num1`]…[`Key::Num5`].
    pub fn style_marker_of_key(key: Key) -> Option<usize> {
        match key {
            Key::Num1 => Some(1),
            Key::Num2 => Some(2),
            Key::Num3 => Some(3),
            Key::Num4 => Some(4),
            Key::Num5 => Some(5),
            _ => None,
        }
    }

    /// Key for style marker `1`…`5`.
    pub fn key_for_style_marker(style: usize) -> Option<Key> {
        match style {
            1 => Some(Key::Num1),
            2 => Some(Key::Num2),
            3 => Some(Key::Num3),
            4 => Some(Key::Num4),
            5 => Some(Key::Num5),
            _ => None,
        }
    }

    /// Next / previous style marker for remappable `shortcut_next_style`.
    ///
    /// Returns `(style, forward)` where `forward` is true for next and false for previous.
    /// Digit-family chords (base key `1`…`5`) accept any of those digits with matching
    /// Ctrl/Cmd/Alt; Shift matching the base goes next, flipped goes previous.
    /// Letter remaps only drive style 1 (Shift flips to previous).
    pub fn style_marker_action(self, mods: Modifiers, pressed: Key) -> Option<(usize, bool)> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let style = Self::style_marker_of_key(pressed)?;
            let cmd = mods.command || mods.ctrl;
            if cmd != self.ctrl_or_cmd || mods.alt != self.alt {
                return None;
            }
            if mods.shift == self.shift {
                Some((style, true))
            } else {
                Some((style, false))
            }
        } else if self.matches(mods, pressed) {
            Some((1, true))
        } else if self.flipped_shift().matches(mods, pressed) {
            Some((1, false))
        } else {
            None
        }
    }

    /// Human-readable next-style binding for Shortcut Mapper / About.
    pub fn style_marker_display(self) -> String {
        if Self::style_marker_of_key(self.key).is_some() {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("1…5");
            parts.join("+")
        } else {
            format!("{} / {}", self.display(), self.flipped_shift().display())
        }
    }

    /// Display for one next/prev style menu row (digit-family uses that digit).
    pub fn style_marker_display_for(self, style: usize, previous: bool) -> Option<String> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let key = Self::key_for_style_marker(style)?;
            let chord = Self {
                key,
                shift: if previous { !self.shift } else { self.shift },
                ..self
            };
            Some(chord.display())
        } else if style == 1 {
            Some(if previous {
                self.flipped_shift().display()
            } else {
                self.display()
            })
        } else {
            None
        }
    }

    /// Mark current line with style for remappable `shortcut_mark_one_style`.
    ///
    /// Returns `Some(style)` for style `1`…`5`, or `None` when the chord does not match.
    /// Digit-family chords (base key `1`…`5`) accept those digits with matching
    /// Ctrl/Cmd/Alt/Shift. Letter remaps mark Style 1 only.
    pub fn mark_one_style_action(self, mods: Modifiers, pressed: Key) -> Option<u8> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let cmd = mods.command || mods.ctrl;
            if cmd != self.ctrl_or_cmd || mods.alt != self.alt || mods.shift != self.shift {
                return None;
            }
            Self::style_marker_of_key(pressed).map(|s| s as u8)
        } else if self.matches(mods, pressed) {
            Some(1)
        } else {
            None
        }
    }

    /// Human-readable mark-current-line-style binding for Shortcut Mapper / About.
    pub fn mark_one_style_display(self) -> String {
        if Self::style_marker_of_key(self.key).is_some() {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("1…5");
            parts.join("+")
        } else {
            self.display()
        }
    }

    /// Display for one Mark Current Line Style menu row (digit-family uses that digit).
    pub fn mark_one_style_display_for(self, style: usize) -> Option<String> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let key = Self::key_for_style_marker(style)?;
            let chord = Self { key, ..self };
            Some(chord.display())
        } else if style == 1 {
            Some(self.display())
        } else {
            None
        }
    }

    /// Copy styled lines for remappable `shortcut_style_to_clip`.
    ///
    /// Returns `Some(Some(style))` for style `1`…`5`, `Some(None)` for all styles,
    /// or `None` when the chord does not match.
    /// Digit-family chords (base key `1`…`5`) accept those digits with matching
    /// Ctrl/Cmd/Alt/Shift; same mods + `0` copies all styles. Letter remaps copy
    /// Style 1; Shift flips to all styles.
    pub fn style_to_clip_action(self, mods: Modifiers, pressed: Key) -> Option<Option<u8>> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let cmd = mods.command || mods.ctrl;
            if cmd != self.ctrl_or_cmd || mods.alt != self.alt || mods.shift != self.shift {
                return None;
            }
            if pressed == Key::Num0 {
                return Some(None);
            }
            let style = Self::style_marker_of_key(pressed)?;
            Some(Some(style as u8))
        } else if self.matches(mods, pressed) {
            Some(Some(1))
        } else if self.flipped_shift().matches(mods, pressed) {
            Some(None)
        } else {
            None
        }
    }

    /// Human-readable copy-styled-text binding for Shortcut Mapper / About.
    pub fn style_to_clip_display(self) -> String {
        if Self::style_marker_of_key(self.key).is_some() {
            let mut parts = Vec::new();
            if self.ctrl_or_cmd {
                parts.push("Cmd");
            }
            if self.alt {
                parts.push("Alt");
            }
            if self.shift {
                parts.push("Shift");
            }
            parts.push("1…5");
            format!("{} (0 = all)", parts.join("+"))
        } else {
            format!("{} / {}", self.display(), self.flipped_shift().display())
        }
    }

    /// Display for one Copy Styled Text menu row (digit-family uses that digit).
    pub fn style_to_clip_display_for(self, style: usize) -> Option<String> {
        let digit_family = Self::style_marker_of_key(self.key).is_some();
        if digit_family {
            let key = Self::key_for_style_marker(style)?;
            let chord = Self { key, ..self };
            Some(chord.display())
        } else if style == 1 {
            Some(self.display())
        } else {
            None
        }
    }

    /// Display for Copy All Styles to Clipboard (digit-family uses `0`).
    pub fn style_to_clip_display_all(self) -> String {
        if Self::style_marker_of_key(self.key).is_some() {
            let chord = Self {
                key: Key::Num0,
                ..self
            };
            chord.display()
        } else {
            self.flipped_shift().display()
        }
    }

    /// Same chord with Shift flipped (find next ↔ find previous).
    pub fn flipped_shift(self) -> Self {
        Self {
            shift: !self.shift,
            ..self
        }
    }

    /// Same chord with Left ↔ Right (apply hunk from ↔ to). Non-arrow keys unchanged.
    pub fn flipped_horizontal(self) -> Self {
        let key = match self.key {
            Key::ArrowLeft => Key::ArrowRight,
            Key::ArrowRight => Key::ArrowLeft,
            other => other,
        };
        Self { key, ..self }
    }

    /// Same chord with Up ↔ Down (move line up ↔ down). Non-arrow keys unchanged.
    pub fn flipped_vertical(self) -> Self {
        let key = match self.key {
            Key::ArrowUp => Key::ArrowDown,
            Key::ArrowDown => Key::ArrowUp,
            other => other,
        };
        Self { key, ..self }
    }

    /// Partner chord for move-line-down: opposite vertical arrow, else Shift flip.
    pub fn move_line_down_chord(self) -> Self {
        match self.key {
            Key::ArrowUp | Key::ArrowDown => self.flipped_vertical(),
            _ => self.flipped_shift(),
        }
    }

    /// Partner chord for the previous function-parameter hint.
    /// Opposite Up/Down when the binding is vertical; otherwise Shift flip.
    pub fn calltip_prev_chord(self) -> Self {
        self.move_line_down_chord()
    }

    /// Partner chord for document-end: same rules as [`Self::move_line_down_chord`].
    pub fn goto_document_end_chord(self) -> Self {
        self.move_line_down_chord()
    }

    /// Partner chord for line-end: Home ↔ End, else Shift flip (letter remaps).
    pub fn line_end_chord(self) -> Self {
        match self.key {
            Key::Home => Self {
                key: Key::End,
                ..self
            },
            Key::End => Self {
                key: Key::Home,
                ..self
            },
            _ => self.flipped_shift(),
        }
    }

    /// Partner chord for document-end Home alias: Home ↔ End, else Shift flip.
    pub fn document_end_chord(self) -> Self {
        self.line_end_chord()
    }

    /// Partner chord for page-down: PageUp ↔ PageDown, else Shift flip (letter remaps).
    pub fn page_down_chord(self) -> Self {
        match self.key {
            Key::PageUp => Self {
                key: Key::PageDown,
                ..self
            },
            Key::PageDown => Self {
                key: Key::PageUp,
                ..self
            },
            _ => self.flipped_shift(),
        }
    }

    /// Partner chord for character-right: Left ↔ Right, else Shift flip (letter remaps).
    pub fn char_right_chord(self) -> Self {
        match self.key {
            Key::ArrowLeft | Key::ArrowRight => self.flipped_horizontal(),
            _ => self.flipped_shift(),
        }
    }

    /// Partner chord for line-down: Up ↔ Down, else Shift flip (letter remaps).
    pub fn line_down_chord(self) -> Self {
        match self.key {
            Key::ArrowUp | Key::ArrowDown => self.flipped_vertical(),
            _ => self.flipped_shift(),
        }
    }

    /// Same chord with `]` ↔ `[` (hide-equal context ±1). Non-bracket keys unchanged.
    pub fn flipped_bracket(self) -> Self {
        let key = match self.key {
            Key::CloseBracket => Key::OpenBracket,
            Key::OpenBracket => Key::CloseBracket,
            other => other,
        };
        Self { key, ..self }
    }

    /// Human-readable form for Shortcut Mapper / About (`Cmd` = Ctrl on non-mac).
    pub fn display(self) -> String {
        let mut parts = Vec::new();
        if self.ctrl_or_cmd {
            parts.push("Cmd");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        parts.push(key_token(self.key));
        parts.join("+")
    }
}

/// Parse a chord string. Empty / invalid → `None`.
pub fn parse_chord(raw: &str) -> Option<KeyChord> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    let mut ctrl_or_cmd = false;
    let mut shift = false;
    let mut alt = false;
    let mut key: Option<Key> = None;
    for part in s.split('+') {
        let p = part.trim();
        if p.is_empty() {
            return None;
        }
        let lower = p.to_ascii_lowercase();
        match lower.as_str() {
            "cmd" | "command" | "ctrl" | "control" | "super" | "meta" => ctrl_or_cmd = true,
            "shift" => shift = true,
            "alt" | "option" => alt = true,
            _ => {
                if key.is_some() {
                    return None;
                }
                key = Some(parse_key(&lower)?);
            }
        }
    }
    Some(KeyChord {
        ctrl_or_cmd,
        shift,
        alt,
        key: key?,
    })
}

/// Resolve a settings string to a chord, falling back to `default` when invalid.
pub fn resolve_chord(raw: &str, default: &str) -> KeyChord {
    parse_chord(raw)
        .or_else(|| parse_chord(default))
        .expect("default chord must parse")
}

fn parse_key(lower: &str) -> Option<Key> {
    match lower {
        "a" => Some(Key::A),
        "b" => Some(Key::B),
        "c" => Some(Key::C),
        "d" => Some(Key::D),
        "e" => Some(Key::E),
        "f" => Some(Key::F),
        "g" => Some(Key::G),
        "h" => Some(Key::H),
        "i" => Some(Key::I),
        "j" => Some(Key::J),
        "k" => Some(Key::K),
        "l" => Some(Key::L),
        "m" => Some(Key::M),
        "n" => Some(Key::N),
        "o" => Some(Key::O),
        "p" => Some(Key::P),
        "q" => Some(Key::Q),
        "r" => Some(Key::R),
        "s" => Some(Key::S),
        "t" => Some(Key::T),
        "u" => Some(Key::U),
        "v" => Some(Key::V),
        "w" => Some(Key::W),
        "x" => Some(Key::X),
        "y" => Some(Key::Y),
        "z" => Some(Key::Z),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        "escape" | "esc" => Some(Key::Escape),
        "tab" => Some(Key::Tab),
        "delete" | "del" => Some(Key::Delete),
        "backspace" | "bs" => Some(Key::Backspace),
        "home" => Some(Key::Home),
        "end" => Some(Key::End),
        "pageup" | "pgup" => Some(Key::PageUp),
        "pagedown" | "pgdn" | "pgdown" => Some(Key::PageDown),
        "enter" | "return" => Some(Key::Enter),
        "space" | "spacebar" => Some(Key::Space),
        "equals" | "=" | "plus" => Some(Key::Equals),
        "minus" | "-" => Some(Key::Minus),
        "0" | "num0" | "digit0" => Some(Key::Num0),
        "1" | "num1" | "digit1" => Some(Key::Num1),
        "2" | "num2" | "digit2" => Some(Key::Num2),
        "3" | "num3" | "digit3" => Some(Key::Num3),
        "4" | "num4" | "digit4" => Some(Key::Num4),
        "5" | "num5" | "digit5" => Some(Key::Num5),
        "6" | "num6" | "digit6" => Some(Key::Num6),
        "7" | "num7" | "digit7" => Some(Key::Num7),
        "8" | "num8" | "digit8" => Some(Key::Num8),
        "9" | "num9" | "digit9" => Some(Key::Num9),
        "]" | "closebracket" | "bracketright" => Some(Key::CloseBracket),
        "[" | "openbracket" | "bracketleft" => Some(Key::OpenBracket),
        "/" | "slash" => Some(Key::Slash),
        ";" | "semicolon" => Some(Key::Semicolon),
        "," | "comma" => Some(Key::Comma),
        "." | "period" | "dot" => Some(Key::Period),
        "left" | "arrowleft" => Some(Key::ArrowLeft),
        "right" | "arrowright" => Some(Key::ArrowRight),
        "up" | "arrowup" => Some(Key::ArrowUp),
        "down" | "arrowdown" => Some(Key::ArrowDown),
        _ => None,
    }
}

fn key_token(key: Key) -> &'static str {
    match key {
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::Escape => "Escape",
        Key::Tab => "Tab",
        Key::Delete => "Delete",
        Key::Backspace => "Backspace",
        Key::Home => "Home",
        Key::End => "End",
        Key::PageUp => "PageUp",
        Key::PageDown => "PageDown",
        Key::Enter => "Enter",
        Key::Space => "Space",
        Key::Equals => "=",
        Key::Minus => "-",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::CloseBracket => "]",
        Key::OpenBracket => "[",
        Key::Slash => "/",
        Key::Semicolon => ";",
        Key::Comma => ",",
        Key::Period => ".",
        Key::ArrowLeft => "Left",
        Key::ArrowRight => "Right",
        Key::ArrowUp => "Up",
        Key::ArrowDown => "Down",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_alt_z_default() {
        let c = parse_chord("Alt+Z").unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Z);
        assert_eq!(c.display(), "Alt+Z");
    }

    #[test]
    fn parse_ctrl_shift_w() {
        let c = parse_chord("ctrl+shift+w").unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.shift);
        assert!(!c.alt);
        assert_eq!(c.key, Key::W);
    }

    #[test]
    fn resolve_invalid_falls_back() {
        let c = resolve_chord("not-a-chord", DEFAULT_WORD_WRAP);
        assert_eq!(c, parse_chord(DEFAULT_WORD_WRAP).unwrap());
    }

    #[test]
    fn matches_respects_modifiers() {
        let c = parse_chord("Alt+Z").unwrap();
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(alt, Key::Z));
        let alt_ctrl = Modifiers {
            alt: true,
            ctrl: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_ctrl, Key::Z));
    }

    #[test]
    fn parse_f3_default_find_next() {
        let c = parse_chord(DEFAULT_FIND_NEXT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F3);
        assert_eq!(c.display(), "F3");
        assert_eq!(c.flipped_shift().display(), "Shift+F3");
    }

    #[test]
    fn parse_cmd_f3_default_set_and_find() {
        let c = parse_chord(DEFAULT_SET_AND_FIND).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F3);
        assert_eq!(c.display(), "Cmd+F3");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+F3");
    }

    #[test]
    fn parse_f4_default_next_found() {
        let c = parse_chord(DEFAULT_NEXT_FOUND).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F4);
        assert_eq!(c.display(), "F4");
        assert_eq!(c.flipped_shift().display(), "Shift+F4");
    }

    #[test]
    fn parse_cmd_alt_f4_default_find_all() {
        let c = parse_chord(DEFAULT_FIND_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F4);
        assert_eq!(c.display(), "Cmd+Alt+F4");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+F4");
    }

    #[test]
    fn parse_alt_shift_i_default_find_incremental() {
        let c = parse_chord(DEFAULT_FIND_INCREMENTAL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::I);
        assert_eq!(c.display(), "Alt+Shift+I");
    }

    #[test]
    fn parse_alt_shift_f3_default_find_mark() {
        let c = parse_chord(DEFAULT_FIND_MARK).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F3);
        assert_eq!(c.display(), "Alt+Shift+F3");
    }

    #[test]
    fn parse_alt_shift_f4_default_find_bookmark() {
        let c = parse_chord(DEFAULT_FIND_BOOKMARK).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F4);
        assert_eq!(c.display(), "Alt+Shift+F4");
    }

    #[test]
    fn parse_alt_shift_f5_default_find_copy() {
        let c = parse_chord(DEFAULT_FIND_COPY).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F5);
        assert_eq!(c.display(), "Alt+Shift+F5");
        let lines = c.flipped_shift();
        assert!(!lines.shift);
        assert_eq!(lines.display(), "Alt+F5");
    }

    #[test]
    fn parse_alt_shift_f6_default_find_cut() {
        let c = parse_chord(DEFAULT_FIND_CUT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F6);
        assert_eq!(c.display(), "Alt+Shift+F6");
        let lines = c.flipped_shift();
        assert!(!lines.shift);
        assert_eq!(lines.display(), "Alt+F6");
    }

    #[test]
    fn parse_alt_shift_f8_default_find_purge() {
        let c = parse_chord(DEFAULT_FIND_PURGE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F8);
        assert_eq!(c.display(), "Alt+Shift+F8");
        let matches = c.flipped_shift();
        assert!(!matches.shift);
        assert_eq!(matches.display(), "Alt+F8");
    }

    #[test]
    fn parse_alt_f9_default_find_char_range() {
        let c = parse_chord(DEFAULT_FIND_CHAR_RANGE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F9);
        assert_eq!(c.display(), "Alt+F9");
        let prev = c.flipped_shift();
        assert!(prev.shift);
        assert_eq!(prev.display(), "Alt+Shift+F9");
    }

    #[test]
    fn parse_cmd_alt_f3_default_find_volatile() {
        let c = parse_chord(DEFAULT_FIND_VOLATILE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F3);
        assert_eq!(c.display(), "Cmd+Alt+F3");
        let prev = c.flipped_shift();
        assert!(prev.shift);
        assert_eq!(prev.display(), "Cmd+Alt+Shift+F3");
    }

    #[test]
    fn parse_cmd_1_default_mark_style() {
        let c = parse_chord(DEFAULT_MARK_STYLE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.display(), "Cmd+1");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+1");
    }

    #[test]
    fn parse_cmd_2_default_mark_style_2() {
        let c = parse_chord(DEFAULT_MARK_STYLE_2).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num2);
        assert_eq!(c.display(), "Cmd+2");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+2");
    }

    #[test]
    fn parse_cmd_3_default_mark_style_3() {
        let c = parse_chord(DEFAULT_MARK_STYLE_3).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num3);
        assert_eq!(c.display(), "Cmd+3");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+3");
    }

    #[test]
    fn parse_cmd_4_default_mark_style_4() {
        let c = parse_chord(DEFAULT_MARK_STYLE_4).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num4);
        assert_eq!(c.display(), "Cmd+4");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+4");
    }

    #[test]
    fn parse_cmd_5_default_mark_style_5() {
        let c = parse_chord(DEFAULT_MARK_STYLE_5).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num5);
        assert_eq!(c.display(), "Cmd+5");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+5");
    }

    #[test]
    fn parse_cmd_shift_0_default_clear_all_styles() {
        let c = parse_chord(DEFAULT_CLEAR_ALL_STYLES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Num0);
        assert_eq!(c.display(), "Cmd+Shift+0");
    }

    #[test]
    fn parse_f2_default_next_bookmark() {
        let c = parse_chord(DEFAULT_NEXT_BOOKMARK).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F2);
        assert_eq!(c.display(), "F2");
        assert_eq!(c.flipped_shift().display(), "Shift+F2");
    }

    #[test]
    fn parse_f7_default_next_diff() {
        let c = parse_chord(DEFAULT_NEXT_DIFF).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F7);
        assert_eq!(c.display(), "F7");
        assert_eq!(c.flipped_shift().display(), "Shift+F7");
    }

    #[test]
    fn parse_cmd_f7_default_first_diff() {
        let c = parse_chord(DEFAULT_FIRST_DIFF).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F7);
        assert_eq!(c.display(), "Cmd+F7");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+F7");
    }

    #[test]
    fn parse_alt_f7_default_next_hidden_equal() {
        let c = parse_chord(DEFAULT_NEXT_HIDDEN_EQUAL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F7);
        assert_eq!(c.display(), "Alt+F7");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+F7");
    }

    #[test]
    fn parse_cmd_alt_f7_default_first_hidden_equal() {
        let c = parse_chord(DEFAULT_FIRST_HIDDEN_EQUAL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F7);
        assert_eq!(c.display(), "Cmd+Alt+F7");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+F7");
    }

    #[test]
    fn parse_cmd_alt_left_default_apply_compare_hunk() {
        let c = parse_chord(DEFAULT_APPLY_COMPARE_HUNK).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowLeft);
        assert_eq!(c.display(), "Cmd+Alt+Left");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+Left");
        assert_eq!(c.flipped_horizontal().display(), "Cmd+Alt+Right");
        assert_eq!(
            c.flipped_horizontal().flipped_shift().display(),
            "Cmd+Alt+Shift+Right"
        );
        let arrow_alias = parse_chord("Ctrl+Alt+ArrowLeft").unwrap();
        assert_eq!(arrow_alias.key, Key::ArrowLeft);
        assert!(arrow_alias.ctrl_or_cmd);
        assert!(arrow_alias.alt);
    }

    #[test]
    fn parse_alt_h_default_hide_equal() {
        let c = parse_chord(DEFAULT_HIDE_EQUAL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::H);
        assert_eq!(c.display(), "Alt+H");
    }

    #[test]
    fn parse_alt_w_default_compare_ignore_ws() {
        let c = parse_chord(DEFAULT_COMPARE_IGNORE_WS).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::W);
        assert_eq!(c.display(), "Alt+W");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+W");
    }

    #[test]
    fn parse_alt_b_default_compare_ignore_blank() {
        let c = parse_chord(DEFAULT_COMPARE_IGNORE_BLANK).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::B);
        assert_eq!(c.display(), "Alt+B");
    }

    #[test]
    fn parse_alt_e_default_expand_all_unchanged() {
        let c = parse_chord(DEFAULT_EXPAND_ALL_UNCHANGED).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::E);
        assert_eq!(c.display(), "Alt+E");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+E");
    }

    #[test]
    fn parse_alt_x_default_expand_unchanged_at_caret() {
        let c = parse_chord(DEFAULT_EXPAND_UNCHANGED_AT_CARET).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::X);
        assert_eq!(c.display(), "Alt+X");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+X");
    }

    #[test]
    fn parse_alt_m_default_bookmark_compare_diffs() {
        let c = parse_chord(DEFAULT_BOOKMARK_COMPARE_DIFFS).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::M);
        assert_eq!(c.display(), "Alt+M");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+M");
    }

    #[test]
    fn parse_alt_d_default_compare() {
        let c = parse_chord(DEFAULT_COMPARE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::D);
        assert_eq!(c.display(), "Alt+D");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+D");
    }

    #[test]
    fn parse_alt_s_default_swap_compare() {
        let c = parse_chord(DEFAULT_SWAP_COMPARE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Alt+S");
    }

    #[test]
    fn parse_alt_shift_s_default_compare_to_saved() {
        let c = parse_chord(DEFAULT_COMPARE_TO_SAVED).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Alt+Shift+S");
    }

    #[test]
    fn parse_alt_c_default_copy_compare_diff() {
        let c = parse_chord(DEFAULT_COPY_COMPARE_DIFF).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::C);
        assert_eq!(c.display(), "Alt+C");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+C");
    }

    #[test]
    fn parse_alt_y_default_copy_compare_summary() {
        let c = parse_chord(DEFAULT_COPY_COMPARE_SUMMARY).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Y);
        assert_eq!(c.display(), "Alt+Y");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+Y");
    }

    #[test]
    fn parse_alt_k_default_copy_compare_hunk() {
        let c = parse_chord(DEFAULT_COPY_COMPARE_HUNK).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::K);
        assert_eq!(c.display(), "Alt+K");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+K");
    }

    #[test]
    fn parse_cmd_tab_default_next_tab() {
        let c = parse_chord(DEFAULT_NEXT_TAB).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Tab);
        assert_eq!(c.display(), "Cmd+Tab");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+Tab");
    }

    #[test]
    fn parse_cmd_alt_home_default_first_tab() {
        let c = parse_chord(DEFAULT_FIRST_TAB).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Home);
        assert_eq!(c.display(), "Cmd+Alt+Home");
        assert_eq!(c.line_end_chord().display(), "Cmd+Alt+End");
        let letter = parse_chord("Ctrl+Alt+T").unwrap();
        assert_eq!(letter.line_end_chord().display(), "Cmd+Alt+Shift+T");
    }

    #[test]
    fn parse_cmd_alt_v_default_tab_to_space() {
        let c = parse_chord(DEFAULT_TAB_TO_SPACE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::V);
        assert_eq!(c.display(), "Cmd+Alt+V");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+V");
    }

    #[test]
    fn parse_cmd_alt_x_default_space_to_tab_all() {
        let c = parse_chord(DEFAULT_SPACE_TO_TAB_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::X);
        assert_eq!(c.display(), "Cmd+Alt+X");
    }

    #[test]
    fn parse_cmd_shift_pagedown_default_move_tab() {
        let c = parse_chord(DEFAULT_MOVE_TAB).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::PageDown);
        assert_eq!(c.display(), "Cmd+Shift+PageDown");
        assert_eq!(c.page_down_chord().display(), "Cmd+Shift+PageUp");
        let letter = parse_chord("Ctrl+Alt+T").unwrap();
        assert_eq!(letter.page_down_chord().display(), "Cmd+Alt+Shift+T");
    }

    #[test]
    fn parse_cmd_alt_s_default_save_all() {
        let c = parse_chord(DEFAULT_SAVE_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Cmd+Alt+S");
    }

    #[test]
    fn parse_cmd_p_default_print() {
        let c = parse_chord(DEFAULT_PRINT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::P);
        assert_eq!(c.display(), "Cmd+P");
    }

    #[test]
    fn parse_cmd_q_default_exit() {
        let c = parse_chord(DEFAULT_EXIT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Q);
        assert_eq!(c.display(), "Cmd+Q");
    }

    #[test]
    fn parse_cmd_f1_default_load_session() {
        let c = parse_chord(DEFAULT_LOAD_SESSION).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F1);
        assert_eq!(c.display(), "Cmd+F1");
        let save = c.flipped_shift();
        assert!(save.shift);
        assert_eq!(save.display(), "Cmd+Shift+F1");
    }

    #[test]
    fn parse_cmd_alt_f1_default_sort_tabs() {
        let c = parse_chord(DEFAULT_SORT_TABS).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F1);
        assert_eq!(c.display(), "Cmd+Alt+F1");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert!(desc.alt);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F1");
        let load = parse_chord(DEFAULT_LOAD_SESSION).unwrap();
        assert_ne!(c.display(), load.display());
    }

    #[test]
    fn parse_cmd_alt_comma_default_sort_tabs_path() {
        let c = parse_chord(DEFAULT_SORT_TABS_PATH).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Comma);
        assert_eq!(c.display(), "Cmd+Alt+,");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert!(desc.alt);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+,");
        let name = parse_chord(DEFAULT_SORT_TABS).unwrap();
        let prefs = parse_chord(DEFAULT_PREFERENCES).unwrap();
        assert_ne!(c.display(), name.display());
        assert_ne!(c.display(), prefs.display());
        assert_ne!(desc.display(), prefs.flipped_shift().display());
    }

    #[test]
    fn parse_cmd_alt_period_default_sort_tabs_type() {
        let c = parse_chord(DEFAULT_SORT_TABS_TYPE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Period);
        assert_eq!(c.display(), "Cmd+Alt+.");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert!(desc.alt);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+.");
        let path = parse_chord(DEFAULT_SORT_TABS_PATH).unwrap();
        let prefs = parse_chord(DEFAULT_PREFERENCES).unwrap();
        assert_ne!(c.display(), path.display());
        assert_ne!(c.display(), prefs.display());
        assert_ne!(desc.display(), path.flipped_shift().display());
        assert_eq!(parse_chord("Ctrl+period").unwrap().key, Key::Period);
    }

    #[test]
    fn parse_cmd_alt_slash_default_sort_tabs_size() {
        let c = parse_chord(DEFAULT_SORT_TABS_SIZE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Slash);
        assert_eq!(c.display(), "Cmd+Alt+/");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert!(desc.alt);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+/");
        let kind = parse_chord(DEFAULT_SORT_TABS_TYPE).unwrap();
        let comment = parse_chord(DEFAULT_TOGGLE_COMMENT).unwrap();
        assert_ne!(c.display(), kind.display());
        assert_ne!(c.display(), comment.display());
        assert_ne!(desc.display(), comment.flipped_shift().display());
        assert_eq!(parse_chord("Ctrl+slash").unwrap().key, Key::Slash);
    }

    #[test]
    fn parse_cmd_alt_semicolon_default_sort_tabs_modified() {
        let c = parse_chord(DEFAULT_SORT_TABS_MODIFIED).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Semicolon);
        assert_eq!(c.display(), "Cmd+Alt+;");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert!(desc.alt);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+;");
        let size = parse_chord(DEFAULT_SORT_TABS_SIZE).unwrap();
        let comment = parse_chord(DEFAULT_TOGGLE_COMMENT).unwrap();
        assert_ne!(c.display(), size.display());
        assert_ne!(c.display(), comment.display());
        assert_ne!(desc.display(), size.flipped_shift().display());
        assert_eq!(parse_chord("Ctrl+semicolon").unwrap().key, Key::Semicolon);
    }

    #[test]
    fn parse_tab_default_indent() {
        let c = parse_chord(DEFAULT_TAB_INDENT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Tab);
        assert_eq!(c.display(), "Tab");
    }

    #[test]
    fn parse_shift_tab_default_outdent() {
        let c = parse_chord(DEFAULT_SHIFT_TAB_OUTDENT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Tab);
        assert_eq!(c.display(), "Shift+Tab");
    }

    #[test]
    fn parse_alt_shift_left_default_rect_select() {
        let c = parse_chord(DEFAULT_RECT_SELECT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::ArrowLeft);
        assert_eq!(c.rect_select_display(), "Alt+Shift+Left/Right/Up/Down");
        let mods = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.rect_select_delta(mods, Key::ArrowLeft), Some((-1, 0)));
        assert_eq!(c.rect_select_delta(mods, Key::ArrowRight), Some((1, 0)));
        assert_eq!(c.rect_select_delta(mods, Key::ArrowUp), Some((0, -1)));
        assert_eq!(c.rect_select_delta(mods, Key::ArrowDown), Some((0, 1)));
        assert_eq!(
            c.rect_select_delta(Modifiers::default(), Key::ArrowLeft),
            None
        );
    }

    #[test]
    fn parse_alt_one_default_fold_level() {
        let c = parse_chord(DEFAULT_FOLD_LEVEL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.fold_level_display(), "Alt+1…8");
        let mods = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert_eq!(c.fold_level_action(mods, Key::Num1), Some((1, true)));
        assert_eq!(c.fold_level_action(mods, Key::Num8), Some((8, true)));
        let unfold = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.fold_level_action(unfold, Key::Num3), Some((3, false)));
        assert_eq!(c.fold_level_action(Modifiers::default(), Key::Num1), None);
        assert_eq!(c.fold_level_display_for(4, false).as_deref(), Some("Alt+4"));
        assert_eq!(
            c.fold_level_display_for(4, true).as_deref(),
            Some("Alt+Shift+4")
        );
    }

    #[test]
    fn parse_cmd_alt_one_default_next_style() {
        let c = parse_chord(DEFAULT_NEXT_STYLE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.style_marker_display(), "Cmd+Alt+1…5");
        let mods = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert_eq!(c.style_marker_action(mods, Key::Num1), Some((1, true)));
        assert_eq!(c.style_marker_action(mods, Key::Num5), Some((5, true)));
        assert_eq!(c.style_marker_action(mods, Key::Num6), None);
        let prev = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.style_marker_action(prev, Key::Num3), Some((3, false)));
        assert_eq!(c.style_marker_action(Modifiers::default(), Key::Num1), None);
        assert_eq!(
            c.style_marker_display_for(4, false).as_deref(),
            Some("Cmd+Alt+4")
        );
        assert_eq!(
            c.style_marker_display_for(4, true).as_deref(),
            Some("Cmd+Alt+Shift+4")
        );
    }

    #[test]
    fn parse_cmd_alt_shift_one_default_style_to_clip() {
        let c = parse_chord(DEFAULT_STYLE_TO_CLIP).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.style_to_clip_display(), "Cmd+Alt+Shift+1…5 (0 = all)");
        let mods = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.style_to_clip_action(mods, Key::Num1), Some(Some(1)));
        assert_eq!(c.style_to_clip_action(mods, Key::Num5), Some(Some(5)));
        assert_eq!(c.style_to_clip_action(mods, Key::Num0), Some(None));
        assert_eq!(c.style_to_clip_action(mods, Key::Num6), None);
        // Without Shift: must not steal Next Style (Cmd+Alt+1…5).
        let no_shift = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert_eq!(c.style_to_clip_action(no_shift, Key::Num1), None);
        assert_eq!(
            c.style_to_clip_display_for(3).as_deref(),
            Some("Cmd+Alt+Shift+3")
        );
        assert_eq!(c.style_to_clip_display_all(), "Cmd+Alt+Shift+0");
    }

    #[test]
    fn parse_alt_shift_one_default_mark_one_style() {
        let c = parse_chord(DEFAULT_MARK_ONE_STYLE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.mark_one_style_display(), "Alt+Shift+1…5");
        let mods = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.mark_one_style_action(mods, Key::Num1), Some(1));
        assert_eq!(c.mark_one_style_action(mods, Key::Num5), Some(5));
        assert_eq!(c.mark_one_style_action(mods, Key::Num6), None);
        // Without Shift: must not steal Fold Level (Alt+1…8).
        let no_shift = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert_eq!(c.mark_one_style_action(no_shift, Key::Num1), None);
        // With Cmd: must not steal Copy Styled (Cmd+Alt+Shift+1…5).
        let with_cmd = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.mark_one_style_action(with_cmd, Key::Num1), None);
        assert_eq!(
            c.mark_one_style_display_for(3).as_deref(),
            Some("Alt+Shift+3")
        );
    }

    #[test]
    fn parse_alt_left_default_word_jump() {
        let c = parse_chord(DEFAULT_WORD_JUMP).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowLeft);
        assert_eq!(c.display(), "Alt+Left");
        assert_eq!(c.flipped_horizontal().display(), "Alt+Right");
        let mods_alt = Modifiers {
            alt: true,
            ..Default::default()
        };
        assert!(c.matches_ignore_shift(mods_alt, Key::ArrowLeft));
        let mods_alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Default::default()
        };
        assert!(c.matches_ignore_shift(mods_alt_shift, Key::ArrowLeft));
        assert!(!c.matches(mods_alt_shift, Key::ArrowLeft));
    }

    #[test]
    fn parse_alt_close_bracket_default_hide_equal_context() {
        let c = parse_chord(DEFAULT_HIDE_EQUAL_CONTEXT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::CloseBracket);
        assert_eq!(c.display(), "Alt+]");
        assert_eq!(c.flipped_bracket().display(), "Alt+[");
        let open = parse_chord("Alt+OpenBracket").unwrap();
        assert_eq!(open.key, Key::OpenBracket);
        assert!(open.alt);
    }

    #[test]
    fn parse_cmd_f2_default_toggle_bookmark() {
        let c = parse_chord(DEFAULT_TOGGLE_BOOKMARK).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F2);
        assert_eq!(c.display(), "Cmd+F2");
    }

    #[test]
    fn parse_cmd_shift_f2_default_clear_bookmarks() {
        let c = parse_chord(DEFAULT_CLEAR_BOOKMARKS).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F2);
        assert_eq!(c.display(), "Cmd+Shift+F2");
    }

    #[test]
    fn parse_alt_shift_f2_default_inverse_bookmarks() {
        let c = parse_chord(DEFAULT_INVERSE_BOOKMARKS).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F2);
        assert_eq!(c.display(), "Alt+Shift+F2");
    }

    #[test]
    fn parse_cmd_alt_f2_default_copy_marked_lines() {
        let c = parse_chord(DEFAULT_COPY_MARKED_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F2);
        assert_eq!(c.display(), "Cmd+Alt+F2");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+F2");
    }

    #[test]
    fn parse_cmd_alt_z_default_remove_unmarked() {
        let c = parse_chord(DEFAULT_REMOVE_UNMARKED).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Z);
        assert_eq!(c.display(), "Cmd+Alt+Z");
    }

    #[test]
    fn parse_cmd_shift_f8_default_clear_change_history() {
        let c = parse_chord(DEFAULT_CLEAR_CHANGE_HISTORY).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F8);
        assert_eq!(c.display(), "Cmd+Shift+F8");
    }

    #[test]
    fn parse_cmd_alt_q_default_paste_marked_lines() {
        let c = parse_chord(DEFAULT_PASTE_MARKED_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Q);
        assert_eq!(c.display(), "Cmd+Alt+Q");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+Q");
    }

    #[test]
    fn parse_cmd_l_default_goto_line() {
        let c = parse_chord(DEFAULT_GOTO_LINE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::L);
        assert_eq!(c.display(), "Cmd+L");
    }

    #[test]
    fn parse_cmd_d_default_duplicate_line() {
        let c = parse_chord(DEFAULT_DUPLICATE_LINE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::D);
        assert_eq!(c.display(), "Cmd+D");
    }

    #[test]
    fn parse_cmd_shift_l_default_delete_line() {
        let c = parse_chord(DEFAULT_DELETE_LINE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::L);
        assert_eq!(c.display(), "Cmd+Shift+L");
    }

    #[test]
    fn parse_cmd_close_bracket_default_indent() {
        let c = parse_chord(DEFAULT_INDENT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::CloseBracket);
        assert_eq!(c.display(), "Cmd+]");
    }

    #[test]
    fn parse_cmd_shift_i_default_format_document() {
        let c = parse_chord(DEFAULT_FORMAT_DOCUMENT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::I);
        assert_eq!(c.display(), "Cmd+Shift+I");
    }

    #[test]
    fn parse_cmd_w_default_close_tab() {
        let c = parse_chord(DEFAULT_CLOSE_TAB).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::W);
        assert_eq!(c.display(), "Cmd+W");
    }

    #[test]
    fn parse_cmd_alt_w_default_close_all() {
        let c = parse_chord(DEFAULT_CLOSE_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::W);
        assert_eq!(c.display(), "Cmd+Alt+W");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+W");
    }

    #[test]
    fn parse_cmd_alt_h_default_close_but_pinned() {
        let c = parse_chord(DEFAULT_CLOSE_BUT_PINNED).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::H);
        assert_eq!(c.display(), "Cmd+Alt+H");
        let unchanged = c.flipped_shift();
        assert!(unchanged.shift);
        assert_eq!(unchanged.display(), "Cmd+Alt+Shift+H");
    }

    #[test]
    fn parse_cmd_alt_i_default_close_to_side() {
        let c = parse_chord(DEFAULT_CLOSE_TO_SIDE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::I);
        assert_eq!(c.display(), "Cmd+Alt+I");
        let to_right = c.flipped_shift();
        assert!(to_right.shift);
        assert_eq!(to_right.display(), "Cmd+Alt+Shift+I");
    }

    #[test]
    fn parse_cmd_alt_y_default_save_copy() {
        let c = parse_chord(DEFAULT_SAVE_COPY).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Y);
        assert_eq!(c.display(), "Cmd+Alt+Y");
        let rename = c.flipped_shift();
        assert!(rename.shift);
        assert_eq!(rename.display(), "Cmd+Alt+Shift+Y");
    }

    #[test]
    fn parse_cmd_alt_t_default_trim_trailing() {
        let c = parse_chord(DEFAULT_TRIM_TRAILING).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::T);
        assert_eq!(c.display(), "Cmd+Alt+T");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+T");
    }

    #[test]
    fn parse_alt_shift_t_default_trim_leading() {
        let c = parse_chord(DEFAULT_TRIM_LEADING).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::T);
        assert_eq!(c.display(), "Alt+Shift+T");
    }

    #[test]
    fn parse_cmd_n_default_new() {
        let c = parse_chord(DEFAULT_NEW).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::N);
        assert_eq!(c.display(), "Cmd+N");
    }

    #[test]
    fn parse_cmd_o_default_open() {
        let c = parse_chord(DEFAULT_OPEN).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::O);
        assert_eq!(c.display(), "Cmd+O");
    }

    #[test]
    fn parse_cmd_s_default_save() {
        let c = parse_chord(DEFAULT_SAVE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Cmd+S");
    }

    #[test]
    fn parse_cmd_shift_s_default_save_as() {
        let c = parse_chord(DEFAULT_SAVE_AS).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Cmd+Shift+S");
    }

    #[test]
    fn parse_cmd_f_default_find() {
        let c = parse_chord(DEFAULT_FIND).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F);
        assert_eq!(c.display(), "Cmd+F");
    }

    #[test]
    fn parse_cmd_h_default_replace() {
        let c = parse_chord(DEFAULT_REPLACE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::H);
        assert_eq!(c.display(), "Cmd+H");
    }

    #[test]
    fn parse_cmd_a_default_select_all() {
        let c = parse_chord(DEFAULT_SELECT_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::A);
        assert_eq!(c.display(), "Cmd+A");
    }

    #[test]
    fn parse_cmd_c_default_copy() {
        let c = parse_chord(DEFAULT_COPY).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::C);
        assert_eq!(c.display(), "Cmd+C");
    }

    #[test]
    fn parse_cmd_x_default_cut() {
        let c = parse_chord(DEFAULT_CUT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::X);
        assert_eq!(c.display(), "Cmd+X");
    }

    #[test]
    fn parse_cmd_v_default_paste() {
        let c = parse_chord(DEFAULT_PASTE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::V);
        assert_eq!(c.display(), "Cmd+V");
    }

    #[test]
    fn parse_cmd_z_default_undo() {
        let c = parse_chord(DEFAULT_UNDO).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Z);
        assert_eq!(c.display(), "Cmd+Z");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+Z");
    }

    #[test]
    fn parse_cmd_y_default_redo() {
        let c = parse_chord(DEFAULT_REDO).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Y);
        assert_eq!(c.display(), "Cmd+Y");
    }

    #[test]
    fn parse_cmd_equals_default_zoom_in() {
        let c = parse_chord(DEFAULT_ZOOM_IN).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Equals);
        assert_eq!(c.display(), "Cmd+=");
    }

    #[test]
    fn parse_cmd_minus_default_zoom_out() {
        let c = parse_chord(DEFAULT_ZOOM_OUT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Minus);
        assert_eq!(c.display(), "Cmd+-");
    }

    #[test]
    fn parse_cmd_zero_default_zoom_restore() {
        let c = parse_chord(DEFAULT_ZOOM_RESTORE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num0);
        assert_eq!(c.display(), "Cmd+0");
    }

    #[test]
    fn parse_alt_zero_default_fold_all() {
        let c = parse_chord(DEFAULT_FOLD_ALL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num0);
        assert_eq!(c.display(), "Alt+0");
        let unfold = c.flipped_shift();
        assert!(unfold.shift);
        assert_eq!(unfold.display(), "Alt+Shift+0");
    }

    #[test]
    fn parse_alt_f_default_fold_current() {
        let c = parse_chord(DEFAULT_FOLD_CURRENT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F);
        assert_eq!(c.display(), "Alt+F");
        let unfold = c.flipped_shift();
        assert!(unfold.shift);
        assert_eq!(unfold.display(), "Alt+Shift+F");
    }

    #[test]
    fn parse_cmd_b_default_matching_brace() {
        let c = parse_chord(DEFAULT_MATCHING_BRACE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::B);
        assert_eq!(c.display(), "Cmd+B");
        let select = c.flipped_shift();
        assert!(select.shift);
        assert_eq!(select.display(), "Cmd+Shift+B");
    }

    #[test]
    fn parse_cmd_shift_up_default_move_line() {
        let c = parse_chord(DEFAULT_MOVE_LINE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::ArrowUp);
        assert_eq!(c.display(), "Cmd+Shift+Up");
        let down = c.move_line_down_chord();
        assert_eq!(down.key, Key::ArrowDown);
        assert_eq!(down.display(), "Cmd+Shift+Down");
        let arrow_alias = parse_chord("Ctrl+Shift+ArrowUp").unwrap();
        assert_eq!(arrow_alias.key, Key::ArrowUp);
        let letter = parse_chord("Alt+M").unwrap();
        assert_eq!(letter.move_line_down_chord().display(), "Alt+Shift+M");
    }

    #[test]
    fn parse_cmd_up_default_goto_document_start() {
        let c = parse_chord(DEFAULT_GOTO_DOCUMENT_START).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowUp);
        assert_eq!(c.display(), "Cmd+Up");
        let end = c.goto_document_end_chord();
        assert_eq!(end.key, Key::ArrowDown);
        assert_eq!(end.display(), "Cmd+Down");
        let letter = parse_chord("Alt+G").unwrap();
        assert_eq!(letter.goto_document_end_chord().display(), "Alt+Shift+G");
    }

    #[test]
    fn parse_cmd_slash_default_toggle_comment() {
        let c = parse_chord(DEFAULT_TOGGLE_COMMENT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Slash);
        assert_eq!(c.display(), "Cmd+/");
        let stream = c.flipped_shift();
        assert!(stream.shift);
        assert_eq!(stream.display(), "Cmd+Shift+/");
        let alias = parse_chord("Ctrl+Slash").unwrap();
        assert_eq!(alias.key, Key::Slash);
    }

    #[test]
    fn parse_alt_slash_default_line_comment() {
        let c = parse_chord(DEFAULT_LINE_COMMENT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Slash);
        assert_eq!(c.display(), "Alt+/");
        let uncomment = c.flipped_shift();
        assert!(uncomment.shift);
        assert_eq!(uncomment.display(), "Alt+Shift+/");
        let toggle = parse_chord(DEFAULT_TOGGLE_COMMENT).unwrap();
        let sort_size = parse_chord(DEFAULT_SORT_TABS_SIZE).unwrap();
        assert_ne!(c.display(), toggle.display());
        assert_ne!(c.display(), toggle.flipped_shift().display());
        assert_ne!(c.display(), sort_size.display());
        assert_ne!(uncomment.display(), sort_size.flipped_shift().display());
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(alt, Key::Slash));
        assert!(!toggle.matches(alt, Key::Slash));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(uncomment.matches(alt_shift, Key::Slash));
        assert!(!c.matches(alt_shift, Key::Slash));
    }

    #[test]
    fn parse_alt_comma_default_stream_comment() {
        let c = parse_chord(DEFAULT_STREAM_COMMENT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Comma);
        assert_eq!(c.display(), "Alt+,");
        let uncomment = c.flipped_shift();
        assert!(uncomment.shift);
        assert_eq!(uncomment.display(), "Alt+Shift+,");
        let toggle = parse_chord(DEFAULT_TOGGLE_COMMENT).unwrap();
        let line = parse_chord(DEFAULT_LINE_COMMENT).unwrap();
        let prefs = parse_chord(DEFAULT_PREFERENCES).unwrap();
        let sort_path = parse_chord(DEFAULT_SORT_TABS_PATH).unwrap();
        assert_ne!(c.display(), toggle.display());
        assert_ne!(c.display(), toggle.flipped_shift().display());
        assert_ne!(c.display(), line.display());
        assert_ne!(uncomment.display(), line.flipped_shift().display());
        assert_ne!(c.display(), prefs.display());
        assert_ne!(c.display(), sort_path.display());
        assert_ne!(uncomment.display(), sort_path.flipped_shift().display());
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(alt, Key::Comma));
        assert!(!prefs.matches(alt, Key::Comma));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(uncomment.matches(alt_shift, Key::Comma));
        assert!(!c.matches(alt_shift, Key::Comma));
    }

    #[test]
    fn parse_cmd_shift_t_default_toggle_log_tail() {
        let c = parse_chord(DEFAULT_TOGGLE_LOG_TAIL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::T);
        assert_eq!(c.display(), "Cmd+Shift+T");
    }

    #[test]
    fn parse_cmd_shift_d_default_document_map() {
        let c = parse_chord(DEFAULT_DOCUMENT_MAP).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::D);
        assert_eq!(c.display(), "Cmd+Shift+D");
    }

    #[test]
    fn parse_cmd_e_default_function_list() {
        let c = parse_chord(DEFAULT_FUNCTION_LIST).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::E);
        assert_eq!(c.display(), "Cmd+E");
        let char_panel = c.flipped_shift();
        assert!(char_panel.shift);
        assert_eq!(char_panel.display(), "Cmd+Shift+E");
    }

    #[test]
    fn parse_f6_default_focus_other_view() {
        let c = parse_chord(DEFAULT_FOCUS_OTHER_VIEW).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F6);
        assert_eq!(c.display(), "F6");
        assert_eq!(c.flipped_shift().display(), "Shift+F6");
    }

    #[test]
    fn parse_alt_o_default_move_other_view() {
        let c = parse_chord(DEFAULT_MOVE_OTHER_VIEW).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::O);
        assert_eq!(c.display(), "Alt+O");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+O");
    }

    #[test]
    fn parse_alt_r_default_rtl() {
        let c = parse_chord(DEFAULT_RTL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::R);
        assert_eq!(c.display(), "Alt+R");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+R");
    }

    #[test]
    fn parse_f8_default_next_changed() {
        let c = parse_chord(DEFAULT_NEXT_CHANGED).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F8);
        assert_eq!(c.display(), "F8");
        assert_eq!(c.flipped_shift().display(), "Shift+F8");
    }

    #[test]
    fn parse_f11_default_fullscreen() {
        let c = parse_chord(DEFAULT_FULLSCREEN).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F11);
        assert_eq!(c.display(), "F11");
    }

    #[test]
    fn parse_f12_default_postit() {
        let c = parse_chord(DEFAULT_POSTIT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F12);
        assert_eq!(c.display(), "F12");
    }

    #[test]
    fn parse_alt_t_default_always_on_top() {
        let c = parse_chord(DEFAULT_ALWAYS_ON_TOP).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::T);
        assert_eq!(c.display(), "Alt+T");
    }

    #[test]
    fn parse_alt_a_default_show_all_characters() {
        let c = parse_chord(DEFAULT_SHOW_ALL_CHARACTERS).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::A);
        assert_eq!(c.display(), "Alt+A");
    }

    #[test]
    fn parse_alt_v_default_show_whitespace() {
        let c = parse_chord(DEFAULT_SHOW_WHITESPACE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::V);
        assert_eq!(c.display(), "Alt+V");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+V");
    }

    #[test]
    fn parse_alt_n_default_show_npc() {
        let c = parse_chord(DEFAULT_SHOW_NPC).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::N);
        assert_eq!(c.display(), "Alt+N");
    }

    #[test]
    fn parse_alt_q_default_sync_scroll() {
        let c = parse_chord(DEFAULT_SYNC_SCROLL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Q);
        assert_eq!(c.display(), "Alt+Q");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+Q");
    }

    #[test]
    fn parse_alt_shift_z_default_zoom_sync() {
        let c = parse_chord(DEFAULT_ZOOM_SYNC).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Z);
        assert_eq!(c.display(), "Alt+Shift+Z");
    }

    #[test]
    fn parse_alt_i_default_indent_guide() {
        let c = parse_chord(DEFAULT_INDENT_GUIDE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::I);
        assert_eq!(c.display(), "Alt+I");
    }

    #[test]
    fn parse_alt_l_default_show_line_numbers() {
        let c = parse_chord(DEFAULT_SHOW_LINE_NUMBERS).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::L);
        assert_eq!(c.display(), "Alt+L");
    }

    #[test]
    fn parse_alt_g_default_show_fold_margin() {
        let c = parse_chord(DEFAULT_SHOW_FOLD_MARGIN).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::G);
        assert_eq!(c.display(), "Alt+G");
    }

    #[test]
    fn parse_alt_p_default_project_panel() {
        let c = parse_chord(DEFAULT_PROJECT_PANEL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::P);
        assert_eq!(c.display(), "Alt+P");
    }

    #[test]
    fn parse_alt_shift_p_default_pin_tab() {
        let c = parse_chord(DEFAULT_PIN_TAB).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::P);
        assert_eq!(c.display(), "Alt+Shift+P");
    }

    #[test]
    fn parse_cmd_u_default_lowercase() {
        let c = parse_chord(DEFAULT_LOWERCASE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::U);
        assert_eq!(c.display(), "Cmd+U");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+U");
    }

    #[test]
    fn parse_cmd_space_default_autocomplete() {
        let c = parse_chord(DEFAULT_AUTOCOMPLETE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Space);
        assert_eq!(c.display(), "Cmd+Space");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+Space");
        assert_eq!(parse_chord("Ctrl+Spacebar").unwrap().key, Key::Space);
    }

    #[test]
    fn parse_alt_down_default_calltip_nav() {
        let c = parse_chord(DEFAULT_CALLTIP_NAV).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowDown);
        assert_eq!(c.display(), "Alt+Down");
        assert_eq!(c.calltip_prev_chord().display(), "Alt+Up");
        let letter = parse_chord("Alt+K").unwrap();
        assert_eq!(letter.calltip_prev_chord().display(), "Alt+Shift+K");
    }

    #[test]
    fn parse_cmd_alt_space_default_autocomplete_path() {
        let c = parse_chord(DEFAULT_AUTOCOMPLETE_PATH).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Space);
        assert_eq!(c.display(), "Cmd+Alt+Space");
        assert_eq!(parse_chord("Ctrl+Alt+Spacebar").unwrap().key, Key::Space);
    }

    #[test]
    fn parse_cmd_j_default_join_lines() {
        let c = parse_chord(DEFAULT_JOIN_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::J);
        assert_eq!(c.display(), "Cmd+J");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+J");
    }

    #[test]
    fn parse_cmd_alt_u_default_invert_case() {
        let c = parse_chord(DEFAULT_INVERT_CASE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::U);
        assert_eq!(c.display(), "Cmd+Alt+U");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+U");
    }

    #[test]
    fn parse_cmd_enter_default_blank_line() {
        let c = parse_chord(DEFAULT_BLANK_LINE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Enter);
        assert_eq!(c.display(), "Cmd+Enter");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+Enter");
        assert_eq!(parse_chord("Cmd+Return").unwrap().key, Key::Enter);
    }

    #[test]
    fn parse_cmd_alt_n_default_multi_select_next() {
        let c = parse_chord(DEFAULT_MULTI_SELECT_NEXT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::N);
        assert_eq!(c.display(), "Cmd+Alt+N");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+N");
    }

    #[test]
    fn parse_cmd_alt_a_default_multi_select_all() {
        let c = parse_chord(DEFAULT_MULTI_SELECT_ALL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::A);
        assert_eq!(c.display(), "Cmd+Alt+A");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+A");
    }

    #[test]
    fn parse_cmd_alt_backspace_default_multi_select_undo() {
        let c = parse_chord(DEFAULT_MULTI_SELECT_UNDO).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Backspace);
        assert_eq!(c.display(), "Cmd+Alt+Backspace");
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd_alt, Key::Backspace));
        // Must not steal bare Backspace (delete backward).
        assert!(!c.matches(Modifiers::default(), Key::Backspace));
        // Must not steal Undo (Cmd+Z).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::Z));
        assert!(!c.matches(cmd, Key::Backspace));
    }

    #[test]
    fn parse_cmd_shift_one_default_goto_tab() {
        let c = parse_chord(DEFAULT_GOTO_TAB).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Num1);
        assert_eq!(c.goto_tab_display(), "Cmd+Shift+1…9");
        let mods = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert_eq!(c.goto_tab_action(mods, Key::Num1), Some(1));
        assert_eq!(c.goto_tab_action(mods, Key::Num9), Some(9));
        assert_eq!(c.goto_tab_action(mods, Key::Num0), None);
        let no_shift = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert_eq!(c.goto_tab_action(no_shift, Key::Num1), None);
        assert_eq!(c.goto_tab_display_for(4).as_deref(), Some("Cmd+Shift+4"));
        assert_eq!(c.goto_tab_display_for(9).as_deref(), Some("Cmd+Shift+9"));
        assert!(c.goto_tab_display_for(10).is_none());
        // Must not steal Mark Style 1 (Cmd+1) or Clear all styles (Cmd+Shift+0).
        assert_eq!(c.goto_tab_action(no_shift, Key::Num1), None);
        assert_eq!(c.goto_tab_action(mods, Key::Num0), None);
    }

    #[test]
    fn parse_f9_default_new_instance() {
        let c = parse_chord(DEFAULT_NEW_INSTANCE).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F9);
        assert_eq!(c.display(), "F9");
        assert_eq!(c.flipped_shift().display(), "Shift+F9");
        assert!(c.matches(Modifiers::default(), Key::F9));
        let shift = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift, Key::F9));
        assert!(c.flipped_shift().matches(shift, Key::F9));
        // Must not steal Find characters in range (Alt+F9) or UTF-8 (Cmd+F9).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::F9));
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::F9));
        assert!(!c.matches(Modifiers::default(), Key::F10));
    }

    #[test]
    fn parse_cmd_alt_b_default_begin_end_select() {
        let c = parse_chord(DEFAULT_BEGIN_END_SELECT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::B);
        assert_eq!(c.display(), "Cmd+Alt+B");
        assert_eq!(c.flipped_shift().display(), "Cmd+Alt+Shift+B");
    }

    #[test]
    fn parse_cmd_alt_r_default_toggle_readonly() {
        let c = parse_chord(DEFAULT_TOGGLE_READONLY).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::R);
        assert_eq!(c.display(), "Cmd+Alt+R");
    }

    #[test]
    fn parse_alt_f1_default_summary() {
        let c = parse_chord(DEFAULT_SUMMARY).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F1);
        assert_eq!(c.display(), "Alt+F1");
        assert_eq!(c.flipped_shift().display(), "Alt+Shift+F1");
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(alt, Key::F1));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::F1));
        assert!(c.flipped_shift().matches(alt_shift, Key::F1));
        // Must not steal Find characters in range (Alt+F9) or read-only-all (F10).
        assert!(!c.matches(alt, Key::F9));
        assert!(!c.matches(Modifiers::default(), Key::F10));
        assert!(!c.flipped_shift().matches(alt, Key::F9));
    }

    #[test]
    fn parse_cmd_comma_default_preferences() {
        let c = parse_chord(DEFAULT_PREFERENCES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Comma);
        assert_eq!(c.display(), "Cmd+,");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+,");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::Comma));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::Comma));
        assert!(c.flipped_shift().matches(cmd_shift, Key::Comma));
        assert_eq!(parse_chord("Ctrl+comma").unwrap().key, Key::Comma);
    }

    #[test]
    fn parse_cmd_f5_default_tab_colour() {
        let c = parse_chord(DEFAULT_TAB_COLOUR).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F5);
        assert_eq!(c.display(), "Cmd+F5");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+F5");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::F5));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::F5));
        assert!(c.flipped_shift().matches(cmd_shift, Key::F5));
        // Must not steal Date Time (F5 / Shift+F5).
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F5));
        assert!(!c.flipped_shift().matches(bare, Key::F5));
        let shift = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift, Key::F5));
        assert!(!c.flipped_shift().matches(shift, Key::F5));
        // Must not steal Sort Lines Integer (Cmd+Alt+F5) or Find Copy (Alt+Shift+F5).
        let sort = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(sort, Key::F5));
        assert!(!c.flipped_shift().matches(sort, Key::F5));
        let find_copy = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(find_copy, Key::F5));
        assert!(!c.flipped_shift().matches(find_copy, Key::F5));
    }

    #[test]
    fn parse_cmd_f4_default_tab_colour_2() {
        let c = parse_chord(DEFAULT_TAB_COLOUR_2).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F4);
        assert_eq!(c.display(), "Cmd+F4");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+F4");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::F4));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::F4));
        assert!(c.flipped_shift().matches(cmd_shift, Key::F4));
        // Must not steal Next/Prev Search Result (F4 / Shift+F4).
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F4));
        assert!(!c.flipped_shift().matches(bare, Key::F4));
        let shift = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift, Key::F4));
        assert!(!c.flipped_shift().matches(shift, Key::F4));
        // Must not steal Find All (Cmd+Alt+F4) or Find Bookmark (Alt+Shift+F4).
        let find_all = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(find_all, Key::F4));
        assert!(!c.flipped_shift().matches(find_all, Key::F4));
        let bookmark = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(bookmark, Key::F4));
        assert!(!c.flipped_shift().matches(bookmark, Key::F4));
    }

    #[test]
    fn parse_cmd_k_default_tab_colour_3() {
        let c = parse_chord(DEFAULT_TAB_COLOUR_3).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::K);
        assert_eq!(c.display(), "Cmd+K");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+K");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::K));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::K));
        assert!(c.flipped_shift().matches(cmd_shift, Key::K));
        // Must not steal Copy Compare Hunk (Alt+K) or Shortcut Mapper (Alt+Shift+K).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::K));
        assert!(!c.flipped_shift().matches(alt, Key::K));
        let mapper = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(mapper, Key::K));
        assert!(!c.flipped_shift().matches(mapper, Key::K));
        // Must not steal Copy Current Directory (Cmd+Alt+K).
        let copy_dir = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(copy_dir, Key::K));
        assert!(!c.flipped_shift().matches(copy_dir, Key::K));
    }

    #[test]
    fn parse_cmd_m_default_tab_colour_4() {
        let c = parse_chord(DEFAULT_TAB_COLOUR_4).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::M);
        assert_eq!(c.display(), "Cmd+M");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+M");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::M));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::M));
        assert!(c.flipped_shift().matches(cmd_shift, Key::M));
        // Must not steal Bookmark Compare Diffs (Alt+M).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::M));
        assert!(!c.flipped_shift().matches(alt, Key::M));
        // Must not steal Copy All Paths (Cmd+Alt+M).
        let copy_all = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(copy_all, Key::M));
        assert!(!c.flipped_shift().matches(copy_all, Key::M));
    }

    #[test]
    fn parse_cmd_semicolon_default_tab_colour_5() {
        let c = parse_chord(DEFAULT_TAB_COLOUR_5).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Semicolon);
        assert_eq!(c.display(), "Cmd+;");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+;");
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd, Key::Semicolon));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::Semicolon));
        assert!(c.flipped_shift().matches(cmd_shift, Key::Semicolon));
        // Must not steal Sort tabs by modified (Cmd+Alt+;).
        let sort_mod = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(sort_mod, Key::Semicolon));
        assert!(!c.flipped_shift().matches(sort_mod, Key::Semicolon));
        // Must not steal Preferences (Cmd+,).
        assert!(!c.matches(cmd, Key::Comma));
    }

    #[test]
    fn parse_cmd_shift_c_default_clipboard_history() {
        let c = parse_chord(DEFAULT_CLIPBOARD_HISTORY).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::C);
        assert_eq!(c.display(), "Cmd+Shift+C");
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd_shift, Key::C));
        // Must not steal Copy (Cmd+C).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::C));
        // Must not steal Column Editor (Cmd+Alt+C).
        let col = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(col, Key::C));
        // Must not steal Copy Compare Diff (Alt+C).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::C));
    }

    #[test]
    fn parse_alt_shift_k_default_shortcut_mapper() {
        let c = parse_chord(DEFAULT_SHORTCUT_MAPPER).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::K);
        assert_eq!(c.display(), "Alt+Shift+K");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::K));
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::K));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(alt_shift, Key::K));
        let cmd_alt_k = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt_k, Key::K));
    }

    #[test]
    fn parse_f10_default_readonly_all() {
        let c = parse_chord(DEFAULT_READONLY_ALL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F10);
        assert_eq!(c.display(), "F10");
        assert_eq!(c.flipped_shift().display(), "Shift+F10");
        let bare = Modifiers::default();
        assert!(c.matches(bare, Key::F10));
        let shift = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift, Key::F10));
        assert!(c.flipped_shift().matches(shift, Key::F10));
        // Must not steal Sort Lines by Length (Cmd+Alt+F10 / Cmd+Alt+Shift+F10).
        let length = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(length, Key::F10));
        assert!(!c.flipped_shift().matches(length, Key::F10));
        let length_shift = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(length_shift, Key::F10));
        assert!(!c.flipped_shift().matches(length_shift, Key::F10));
    }

    #[test]
    fn parse_cmd_alt_c_default_column_editor() {
        let c = parse_chord(DEFAULT_COLUMN_EDITOR).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::C);
        assert_eq!(c.display(), "Cmd+Alt+C");
    }

    #[test]
    fn parse_cmd_alt_o_default_sort_lines() {
        let c = parse_chord(DEFAULT_SORT_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::O);
        assert_eq!(c.display(), "Cmd+Alt+O");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+O");
    }

    #[test]
    fn parse_cmd_alt_f5_default_sort_lines_integer() {
        let c = parse_chord(DEFAULT_SORT_LINES_INTEGER).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F5);
        assert_eq!(c.display(), "Cmd+Alt+F5");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F5");
    }

    #[test]
    fn parse_cmd_alt_f6_default_sort_lines_case_insensitive() {
        let c = parse_chord(DEFAULT_SORT_LINES_CASE_INSENSITIVE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F6);
        assert_eq!(c.display(), "Cmd+Alt+F6");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F6");
    }

    #[test]
    fn parse_cmd_alt_f8_default_sort_lines_decimal_dot() {
        let c = parse_chord(DEFAULT_SORT_LINES_DECIMAL_DOT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F8);
        assert_eq!(c.display(), "Cmd+Alt+F8");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F8");
    }

    #[test]
    fn parse_cmd_alt_f9_default_sort_lines_decimal_comma() {
        let c = parse_chord(DEFAULT_SORT_LINES_DECIMAL_COMMA).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F9);
        assert_eq!(c.display(), "Cmd+Alt+F9");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F9");
    }

    #[test]
    fn parse_cmd_alt_f10_default_sort_lines_length() {
        let c = parse_chord(DEFAULT_SORT_LINES_LENGTH).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F10);
        assert_eq!(c.display(), "Cmd+Alt+F10");
        let desc = c.flipped_shift();
        assert!(desc.shift);
        assert_eq!(desc.display(), "Cmd+Alt+Shift+F10");
    }

    #[test]
    fn parse_cmd_alt_f11_default_eol_to_space() {
        let c = parse_chord(DEFAULT_EOL_TO_SPACE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F11);
        assert_eq!(c.display(), "Cmd+Alt+F11");
        let trim_all = c.flipped_shift();
        assert!(trim_all.shift);
        assert_eq!(trim_all.display(), "Cmd+Alt+Shift+F11");
    }

    #[test]
    fn parse_cmd_alt_f12_default_convert_eol() {
        let c = parse_chord(DEFAULT_CONVERT_EOL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F12);
        assert_eq!(c.display(), "Cmd+Alt+F12");
        let windows = c.flipped_shift();
        assert!(windows.shift);
        assert_eq!(windows.display(), "Cmd+Alt+Shift+F12");
    }

    #[test]
    fn parse_alt_shift_l_default_hide_lines() {
        let c = parse_chord(DEFAULT_HIDE_LINES).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::L);
        assert_eq!(c.display(), "Alt+Shift+L");
        let mods = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(mods, Key::L));
        // Without Shift: must not steal Show Line Numbers (Alt+L).
        let no_shift = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(no_shift, Key::L));
        // With Cmd: must not steal Open Containing Folder (Cmd+Alt+L / shell).
        let with_cmd = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(with_cmd, Key::L));
    }

    #[test]
    fn parse_alt_shift_g_default_show_wrap_symbol() {
        let c = parse_chord(DEFAULT_SHOW_WRAP_SYMBOL).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::G);
        assert_eq!(c.display(), "Alt+Shift+G");
        let mods = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(mods, Key::G));
        // Without Shift: must not steal Show Fold Margin (Alt+G).
        let no_shift = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(no_shift, Key::G));
        // With Cmd: must not steal Open Folder as Workspace (Cmd+Alt+G / containing folder).
        let with_cmd = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(with_cmd, Key::G));
    }

    #[test]
    fn parse_cmd_f9_default_encoding_utf8() {
        let c = parse_chord(DEFAULT_ENCODING_UTF8).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F9);
        assert_eq!(c.display(), "Cmd+F9");
        let bom = c.flipped_shift();
        assert!(bom.shift);
        assert!(!bom.alt);
        assert_eq!(bom.display(), "Cmd+Shift+F9");
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::F9));
        assert!(!bom.matches(alt, Key::F9));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F9));
        assert!(!bom.matches(cmd_alt, Key::F9));
    }

    #[test]
    fn parse_cmd_f10_default_encoding_utf16() {
        let c = parse_chord(DEFAULT_ENCODING_UTF16).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F10);
        assert_eq!(c.display(), "Cmd+F10");
        let be = c.flipped_shift();
        assert!(be.shift);
        assert!(!be.alt);
        assert_eq!(be.display(), "Cmd+Shift+F10");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F10));
        assert!(!be.matches(bare, Key::F10));
        let shift_only = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift_only, Key::F10));
        assert!(!be.matches(shift_only, Key::F10));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F10));
        assert!(!be.matches(cmd_alt, Key::F10));
    }

    #[test]
    fn parse_cmd_f11_default_encoding_ansi() {
        let c = parse_chord(DEFAULT_ENCODING_ANSI).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F11);
        assert_eq!(c.display(), "Cmd+F11");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F11));
        let shift_only = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift_only, Key::F11));
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::F11));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F11));
    }

    #[test]
    fn parse_alt_f10_default_md5() {
        let c = parse_chord(DEFAULT_MD5).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F10);
        assert_eq!(c.display(), "Alt+F10");
        let clip = c.flipped_shift();
        assert!(clip.alt);
        assert!(clip.shift);
        assert!(!clip.ctrl_or_cmd);
        assert_eq!(clip.display(), "Alt+Shift+F10");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F10));
        assert!(!clip.matches(bare, Key::F10));
        let shift_only = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift_only, Key::F10));
        assert!(!clip.matches(shift_only, Key::F10));
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::F10));
        assert!(!clip.matches(cmd, Key::F10));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F10));
        assert!(!clip.matches(cmd_alt, Key::F10));
    }

    #[test]
    fn parse_alt_f11_default_sha256() {
        let c = parse_chord(DEFAULT_SHA256).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F11);
        assert_eq!(c.display(), "Alt+F11");
        let clip = c.flipped_shift();
        assert!(clip.alt);
        assert!(clip.shift);
        assert!(!clip.ctrl_or_cmd);
        assert_eq!(clip.display(), "Alt+Shift+F11");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F11));
        assert!(!clip.matches(bare, Key::F11));
        let shift_only = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift_only, Key::F11));
        assert!(!clip.matches(shift_only, Key::F11));
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::F11));
        assert!(!clip.matches(cmd, Key::F11));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F11));
        assert!(!clip.matches(cmd_alt, Key::F11));
    }

    #[test]
    fn parse_cmd_f12_default_sha1() {
        let c = parse_chord(DEFAULT_SHA1).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F12);
        assert_eq!(c.display(), "Cmd+F12");
        let clip = c.flipped_shift();
        assert!(clip.ctrl_or_cmd);
        assert!(clip.shift);
        assert!(!clip.alt);
        assert_eq!(clip.display(), "Cmd+Shift+F12");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F12));
        assert!(!clip.matches(bare, Key::F12));
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::F12));
        assert!(!clip.matches(alt, Key::F12));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::F12));
        assert!(!clip.matches(alt_shift, Key::F12));
    }

    #[test]
    fn parse_cmd_f6_default_sha512() {
        let c = parse_chord(DEFAULT_SHA512).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F6);
        assert_eq!(c.display(), "Cmd+F6");
        let clip = c.flipped_shift();
        assert!(clip.ctrl_or_cmd);
        assert!(clip.shift);
        assert!(!clip.alt);
        assert_eq!(clip.display(), "Cmd+Shift+F6");
        let bare = Modifiers::default();
        assert!(!c.matches(bare, Key::F6));
        assert!(!clip.matches(bare, Key::F6));
        let shift_only = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift_only, Key::F6));
        assert!(!clip.matches(shift_only, Key::F6));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::F6));
        assert!(!clip.matches(alt_shift, Key::F6));
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::F6));
        assert!(!clip.matches(cmd_alt, Key::F6));
    }

    #[test]
    fn parse_alt_shift_f12_default_convert_eol_mac() {
        let c = parse_chord(DEFAULT_CONVERT_EOL_MAC).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F12);
        assert_eq!(c.display(), "Alt+Shift+F12");
    }

    #[test]
    fn parse_cmd_alt_6_default_sort_lines_reverse() {
        let c = parse_chord(DEFAULT_SORT_LINES_REVERSE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num6);
        assert_eq!(c.display(), "Cmd+Alt+6");
        let randomly = c.flipped_shift();
        assert!(randomly.shift);
        assert_eq!(randomly.display(), "Cmd+Alt+Shift+6");
    }

    #[test]
    fn parse_cmd_alt_7_default_sort_lines_locale() {
        let c = parse_chord(DEFAULT_SORT_LINES_LOCALE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num7);
        assert_eq!(c.display(), "Cmd+Alt+7");
        let descending = c.flipped_shift();
        assert!(descending.shift);
        assert_eq!(descending.display(), "Cmd+Alt+Shift+7");
    }

    #[test]
    fn parse_cmd_alt_8_default_sentence_case() {
        let c = parse_chord(DEFAULT_SENTENCE_CASE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num8);
        assert_eq!(c.display(), "Cmd+Alt+8");
        let blend = c.flipped_shift();
        assert!(blend.shift);
        assert_eq!(blend.display(), "Cmd+Alt+Shift+8");
    }

    #[test]
    fn parse_cmd_alt_9_default_proper_case() {
        let c = parse_chord(DEFAULT_PROPER_CASE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num9);
        assert_eq!(c.display(), "Cmd+Alt+9");
        let blend = c.flipped_shift();
        assert!(blend.shift);
        assert_eq!(blend.display(), "Cmd+Alt+Shift+9");
    }

    #[test]
    fn parse_cmd_alt_0_default_random_case() {
        let c = parse_chord(DEFAULT_RANDOM_CASE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Num0);
        assert_eq!(c.display(), "Cmd+Alt+0");
    }

    #[test]
    fn parse_alt_j_default_redact() {
        let c = parse_chord(DEFAULT_REDACT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::J);
        assert_eq!(c.display(), "Alt+J");
        let circle = c.flipped_shift();
        assert!(circle.shift);
        assert_eq!(circle.display(), "Alt+Shift+J");
    }

    #[test]
    fn parse_alt_u_default_search_internet() {
        let c = parse_chord(DEFAULT_SEARCH_INTERNET).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::U);
        assert_eq!(c.display(), "Alt+U");
        let change_engine = c.flipped_shift();
        assert!(change_engine.shift);
        assert_eq!(change_engine.display(), "Alt+Shift+U");
    }

    #[test]
    fn parse_cmd_alt_e_default_remove_empty_lines() {
        let c = parse_chord(DEFAULT_REMOVE_EMPTY_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::E);
        assert_eq!(c.display(), "Cmd+Alt+E");
        let with_blank = c.flipped_shift();
        assert!(with_blank.shift);
        assert_eq!(with_blank.display(), "Cmd+Alt+Shift+E");
    }

    #[test]
    fn parse_cmd_alt_d_default_remove_dup_lines() {
        let c = parse_chord(DEFAULT_REMOVE_DUP_LINES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::D);
        assert_eq!(c.display(), "Cmd+Alt+D");
        let any = c.flipped_shift();
        assert!(any.shift);
        assert_eq!(any.display(), "Cmd+Alt+Shift+D");
    }

    #[test]
    fn parse_f5_default_insert_datetime() {
        let c = parse_chord(DEFAULT_INSERT_DATETIME).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F5);
        assert_eq!(c.display(), "F5");
        let long = c.flipped_shift();
        assert!(long.shift);
        assert_eq!(long.display(), "Shift+F5");
    }

    #[test]
    fn parse_cmd_alt_equals_default_insert_datetime_custom() {
        let c = parse_chord(DEFAULT_INSERT_DATETIME_CUSTOM).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Equals);
        assert_eq!(c.display(), "Cmd+Alt+=");
        let both = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(c.matches(both, Key::Equals));
        // Must not steal Zoom in (Cmd+=).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::Equals));
        // Must not steal Date Time short (F5) or long (Shift+F5).
        let none = Modifiers::default();
        assert!(!c.matches(none, Key::F5));
        let shift = Modifiers {
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(shift, Key::F5));
    }

    #[test]
    fn parse_cmd_shift_p_default_macro_playback() {
        let c = parse_chord(DEFAULT_MACRO_PLAYBACK).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::P);
        assert_eq!(c.display(), "Cmd+Shift+P");
        let both = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(both, Key::P));
        // Must not steal Print (Cmd+P).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::P));
        // Must not steal Copy Full Path (Cmd+Alt+P).
        let alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::P));
    }

    #[test]
    fn parse_cmd_alt_shift_r_default_macro_record() {
        let c = parse_chord(DEFAULT_MACRO_RECORD).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::R);
        assert_eq!(c.display(), "Cmd+Alt+Shift+R");
        let all = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(all, Key::R));
        // Must not steal Reload (Cmd+R).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::R));
        // Must not steal Run (Cmd+Shift+R).
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::R));
        // Must not steal Toggle Read-Only (Cmd+Alt+R).
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::R));
        // Must not steal RTL (Alt+R) or LTR (Alt+Shift+R).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::R));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::R));
    }

    #[test]
    fn parse_cmd_alt_shift_s_default_macro_save() {
        let c = parse_chord(DEFAULT_MACRO_SAVE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::S);
        assert_eq!(c.display(), "Cmd+Alt+Shift+S");
        let all = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(all, Key::S));
        // Must not steal Save (Cmd+S).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::S));
        // Must not steal Save As (Cmd+Shift+S).
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_shift, Key::S));
        // Must not steal Save All (Cmd+Alt+S).
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::S));
        // Must not steal Swap Compare (Alt+S) or Compare to Saved (Alt+Shift+S).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::S));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::S));
    }

    #[test]
    fn parse_cmd_alt_shift_k_default_macro_repeat() {
        let c = parse_chord(DEFAULT_MACRO_REPEAT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::K);
        assert_eq!(c.display(), "Cmd+Alt+Shift+K");
        let all = Modifiers {
            command: true,
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(all, Key::K));
        // Must not steal Copy Current Directory (Cmd+Alt+K).
        let cmd_alt = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd_alt, Key::K));
        // Must not steal Tab colour 3 (Cmd+K).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::K));
        // Must not steal Copy Compare Hunk (Alt+K) or Shortcut Mapper (Alt+Shift+K).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::K));
        let alt_shift = Modifiers {
            alt: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt_shift, Key::K));
    }

    #[test]
    fn parse_cmd_alt_p_default_copy_path() {
        let c = parse_chord(DEFAULT_COPY_PATH).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::P);
        assert_eq!(c.display(), "Cmd+Alt+P");
        let name = c.flipped_shift();
        assert!(name.shift);
        assert_eq!(name.display(), "Cmd+Alt+Shift+P");
    }

    #[test]
    fn parse_cmd_alt_k_default_copy_dir() {
        let c = parse_chord(DEFAULT_COPY_DIR).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::K);
        assert_eq!(c.display(), "Cmd+Alt+K");
    }

    #[test]
    fn parse_cmd_alt_m_default_copy_all_paths() {
        let c = parse_chord(DEFAULT_COPY_ALL_PATHS).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::M);
        assert_eq!(c.display(), "Cmd+Alt+M");
        let names = c.flipped_shift();
        assert!(names.shift);
        assert_eq!(names.display(), "Cmd+Alt+Shift+M");
    }

    #[test]
    fn parse_cmd_alt_j_default_open_selected() {
        let c = parse_chord(DEFAULT_OPEN_SELECTED).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::J);
        assert_eq!(c.display(), "Cmd+Alt+J");
        let folder = c.flipped_shift();
        assert!(folder.shift);
        assert_eq!(folder.display(), "Cmd+Alt+Shift+J");
    }

    #[test]
    fn parse_cmd_alt_l_default_open_folder() {
        let c = parse_chord(DEFAULT_OPEN_FOLDER).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::L);
        assert_eq!(c.display(), "Cmd+Alt+L");
        let shell = c.flipped_shift();
        assert!(shell.shift);
        assert_eq!(shell.display(), "Cmd+Alt+Shift+L");
    }

    #[test]
    fn parse_cmd_alt_g_default_open_workspace() {
        let c = parse_chord(DEFAULT_OPEN_WORKSPACE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::G);
        assert_eq!(c.display(), "Cmd+Alt+G");
        let containing = c.flipped_shift();
        assert!(containing.shift);
        assert_eq!(containing.display(), "Cmd+Alt+Shift+G");
    }

    #[test]
    fn parse_cmd_alt_f_default_find_in_files() {
        let c = parse_chord(DEFAULT_FIND_IN_FILES).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F);
        assert_eq!(c.display(), "Cmd+Alt+F");
    }

    #[test]
    fn parse_escape_default_close_find() {
        let c = parse_chord(DEFAULT_CLOSE_FIND).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Escape);
        assert_eq!(c.display(), "Escape");
        assert_eq!(parse_chord("Esc").unwrap().key, Key::Escape);
    }

    #[test]
    fn parse_delete_default_delete_forward() {
        let c = parse_chord(DEFAULT_DELETE_FORWARD).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Delete);
        assert_eq!(c.display(), "Delete");
        assert_eq!(parse_chord("Del").unwrap().key, Key::Delete);
    }

    #[test]
    fn parse_backspace_default_delete_backward() {
        let c = parse_chord(DEFAULT_DELETE_BACKWARD).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Backspace);
        assert_eq!(c.display(), "Backspace");
        assert_eq!(parse_chord("Bs").unwrap().key, Key::Backspace);
    }

    #[test]
    fn parse_enter_default_enter() {
        let c = parse_chord(DEFAULT_ENTER).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Enter);
        assert_eq!(c.display(), "Enter");
        assert_eq!(parse_chord("Return").unwrap().key, Key::Enter);
    }

    #[test]
    fn parse_home_default_line_home() {
        let c = parse_chord(DEFAULT_LINE_HOME).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Home);
        assert_eq!(c.display(), "Home");
        assert_eq!(c.line_end_chord().display(), "End");
        let letter = parse_chord("Ctrl+Alt+H").unwrap();
        assert_eq!(letter.line_end_chord().display(), "Cmd+Alt+Shift+H");
    }

    #[test]
    fn parse_cmd_home_default_document_home() {
        let c = parse_chord(DEFAULT_DOCUMENT_HOME).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::Home);
        assert_eq!(c.display(), "Cmd+Home");
        assert_eq!(c.document_end_chord().display(), "Cmd+End");
        let letter = parse_chord("Ctrl+Alt+H").unwrap();
        assert_eq!(letter.document_end_chord().display(), "Cmd+Alt+Shift+H");
    }

    #[test]
    fn parse_pageup_default_page_up() {
        let c = parse_chord(DEFAULT_PAGE_UP).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::PageUp);
        assert_eq!(c.display(), "PageUp");
        assert_eq!(c.page_down_chord().display(), "PageDown");
        assert_eq!(parse_chord("PgUp").unwrap().key, Key::PageUp);
        assert_eq!(parse_chord("PgDn").unwrap().key, Key::PageDown);
        let letter = parse_chord("Ctrl+Alt+P").unwrap();
        assert_eq!(letter.page_down_chord().display(), "Cmd+Alt+Shift+P");
    }

    #[test]
    fn parse_left_default_char_left() {
        let c = parse_chord(DEFAULT_CHAR_LEFT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowLeft);
        assert_eq!(c.display(), "Left");
        assert_eq!(c.char_right_chord().display(), "Right");
        assert_eq!(parse_chord("ArrowLeft").unwrap().key, Key::ArrowLeft);
        assert_eq!(parse_chord("ArrowRight").unwrap().key, Key::ArrowRight);
        let letter = parse_chord("Ctrl+Alt+L").unwrap();
        assert_eq!(letter.char_right_chord().display(), "Cmd+Alt+Shift+L");
    }

    #[test]
    fn parse_up_default_line_up() {
        let c = parse_chord(DEFAULT_LINE_UP).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::ArrowUp);
        assert_eq!(c.display(), "Up");
        assert_eq!(c.line_down_chord().display(), "Down");
        assert_eq!(parse_chord("ArrowUp").unwrap().key, Key::ArrowUp);
        assert_eq!(parse_chord("ArrowDown").unwrap().key, Key::ArrowDown);
        let letter = parse_chord("Ctrl+Alt+I").unwrap();
        assert_eq!(letter.line_down_chord().display(), "Cmd+Alt+Shift+I");
    }

    #[test]
    fn parse_cmd_g_default_find_next_global() {
        let c = parse_chord(DEFAULT_FIND_NEXT_GLOBAL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::G);
        assert_eq!(c.display(), "Cmd+G");
        assert_eq!(c.flipped_shift().display(), "Cmd+Shift+G");
    }

    #[test]
    fn parse_cmd_shift_f_default_replace_alt() {
        let c = parse_chord(DEFAULT_REPLACE_ALT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::F);
        assert_eq!(c.display(), "Cmd+Shift+F");
    }

    #[test]
    fn parse_cmd_open_bracket_default_outdent() {
        let c = parse_chord(DEFAULT_OUTDENT).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::OpenBracket);
        assert_eq!(c.display(), "Cmd+[");
    }

    #[test]
    fn parse_cmd_r_default_reload() {
        let c = parse_chord(DEFAULT_RELOAD).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::R);
        assert_eq!(c.display(), "Cmd+R");
    }

    #[test]
    fn parse_cmd_shift_r_default_run() {
        let c = parse_chord(DEFAULT_RUN).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::R);
        assert_eq!(c.display(), "Cmd+Shift+R");
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd_shift, Key::R));
        // Must not steal Reload (Cmd+R).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::R));
        // Must not steal Toggle Read-Only (Cmd+Alt+R).
        let readonly = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(readonly, Key::R));
        // Must not steal RTL (Alt+R).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::R));
    }

    #[test]
    fn parse_f1_default_about() {
        let c = parse_chord(DEFAULT_ABOUT).unwrap();
        assert!(!c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(!c.shift);
        assert_eq!(c.key, Key::F1);
        assert_eq!(c.display(), "F1");
        let none = Modifiers::default();
        assert!(c.matches(none, Key::F1));
        // Must not steal Summary (Alt+F1).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::F1));
        // Must not steal Load Session (Cmd+F1).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::F1));
    }

    #[test]
    fn parse_cmd_shift_delete_default_delete_file() {
        let c = parse_chord(DEFAULT_DELETE_FILE).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::Delete);
        assert_eq!(c.display(), "Cmd+Shift+Delete");
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd_shift, Key::Delete));
        // Must not steal forward Delete.
        let none = Modifiers::default();
        assert!(!c.matches(none, Key::Delete));
        // Must not steal Delete line (Cmd+Shift+L).
        assert!(!c.matches(cmd_shift, Key::L));
    }

    #[test]
    fn parse_cmd_shift_o_default_open_viewer() {
        let c = parse_chord(DEFAULT_OPEN_VIEWER).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::O);
        assert_eq!(c.display(), "Cmd+Shift+O");
        let cmd_shift = Modifiers {
            command: true,
            shift: true,
            ..Modifiers::default()
        };
        assert!(c.matches(cmd_shift, Key::O));
        // Must not steal Open (Cmd+O).
        let cmd = Modifiers {
            command: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(cmd, Key::O));
        // Must not steal Move/Clone Other View (Alt+O).
        let alt = Modifiers {
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(alt, Key::O));
        // Must not steal Sort Lines (Cmd+Alt+O).
        let sort = Modifiers {
            command: true,
            alt: true,
            ..Modifiers::default()
        };
        assert!(!c.matches(sort, Key::O));
    }
}
