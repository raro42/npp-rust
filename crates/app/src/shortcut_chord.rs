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

/// Default next-bookmark binding (matches historical hard-wire). Prev uses Shift toggled.
pub const DEFAULT_NEXT_BOOKMARK: &str = "F2";

/// Default toggle-bookmark binding (matches historical hard-wire).
pub const DEFAULT_TOGGLE_BOOKMARK: &str = "Cmd+F2";

/// Default clear-all-bookmarks binding (Search → Clear all bookmarks; N++ Ctrl+Shift+F2 family).
pub const DEFAULT_CLEAR_BOOKMARKS: &str = "Cmd+Shift+F2";

/// Default inverse-bookmarks binding (Search → Inverse Bookmarks).
/// Stays on the F2 family without colliding with next/prev/toggle/clear.
pub const DEFAULT_INVERSE_BOOKMARKS: &str = "Alt+Shift+F2";

/// Default next change-history mark binding (Search → Next Change). Prev uses Shift toggled.
pub const DEFAULT_NEXT_CHANGED: &str = "F8";

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

/// Default show-fold-margin toggle binding (View → Show Fold Margin).
pub const DEFAULT_SHOW_FOLD_MARGIN: &str = "Alt+G";

/// Default project-panel toggle binding (View → Project Panel).
pub const DEFAULT_PROJECT_PANEL: &str = "Alt+P";

/// Default pin-tab toggle binding (tab chrome / `IDM_PINTAB`).
/// Uses Alt+Shift+P so it does not collide with Project Panel (Alt+P).
pub const DEFAULT_PIN_TAB: &str = "Alt+Shift+P";

/// Default reload-from-disk binding (new remappable chord; File → Reload).
pub const DEFAULT_RELOAD: &str = "Cmd+R";

/// Default next-tab binding (new remappable chord). Shift flips to previous tab.
pub const DEFAULT_NEXT_TAB: &str = "Cmd+Tab";

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

/// Default lowercase binding (Edit → lowercase). Shift flips to UPPERCASE (N++ Ctrl+U family).
pub const DEFAULT_LOWERCASE: &str = "Cmd+U";

/// Default join-lines binding (Edit → Join Lines). Shift flips to Split Lines (N++ Ctrl+J family).
pub const DEFAULT_JOIN_LINES: &str = "Cmd+J";

/// Default invert-case binding (Edit → iNVERT cASE). Shift flips to Proper Case.
pub const DEFAULT_INVERT_CASE: &str = "Cmd+Alt+U";

/// Default blank-line-below binding (Edit → Insert Empty Line Below Current).
/// Shift flips to Insert Empty Line Above Current.
pub const DEFAULT_BLANK_LINE: &str = "Cmd+Enter";

/// Default multi-select-next binding (Edit → Multi-select Next).
/// Shift flips to Skip Current / Go to Next Multi-select.
pub const DEFAULT_MULTI_SELECT_NEXT: &str = "Cmd+Alt+N";

/// Default multi-select-all binding (Edit → Multi-select All).
/// Shift flips to Multi-select All (Whole Word).
pub const DEFAULT_MULTI_SELECT_ALL: &str = "Cmd+Alt+A";

/// Default Begin/End Select binding (Edit → Begin/End Select).
/// Shift flips to Begin/End Select in Column Mode.
pub const DEFAULT_BEGIN_END_SELECT: &str = "Cmd+Alt+B";

/// Default toggle read-only binding (Edit → Read-Only).
/// Uses Cmd+Alt+R so it does not collide with Reload (Cmd+R) or RTL (Alt+R).
pub const DEFAULT_TOGGLE_READONLY: &str = "Cmd+Alt+R";

/// Default Column Editor binding (Edit → Column Editor).
/// Uses Cmd+Alt+C so it does not collide with Copy (Cmd+C) or Copy Compare Diff (Alt+C).
pub const DEFAULT_COLUMN_EDITOR: &str = "Cmd+Alt+C";

/// Default Sort Lines Lexicographic Ascending binding.
/// Uses Cmd+Alt+O so it does not collide with Open (Cmd+O) or Move/Clone Other View (Alt+O).
/// Shift flips to Lexicographic Descending.
pub const DEFAULT_SORT_LINES: &str = "Cmd+Alt+O";

/// Default Remove Empty Lines binding (Edit → Line Operations → Remove Empty Lines).
/// Uses Cmd+Alt+E so it does not collide with Expand All Unchanged (Alt+E).
/// Shift flips to Remove Empty Lines (Containing Blank Characters).
pub const DEFAULT_REMOVE_EMPTY_LINES: &str = "Cmd+Alt+E";

/// Default Remove Consecutive Duplicate Lines binding (Edit → Line Operations).
/// Uses Cmd+Alt+D so it does not collide with Duplicate Line (Cmd+D).
/// Shift flips to Remove Duplicate Lines (any occurrence).
pub const DEFAULT_REMOVE_DUP_LINES: &str = "Cmd+Alt+D";

/// Default Date Time (short) binding (Edit → Insert; N++ F5 family).
/// Shift flips to Date Time (long). Customized stays menu-only.
pub const DEFAULT_INSERT_DATETIME: &str = "F5";

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
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
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
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
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
}
