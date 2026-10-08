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

/// Default next-bookmark binding (matches historical hard-wire). Prev uses Shift toggled.
pub const DEFAULT_NEXT_BOOKMARK: &str = "F2";

/// Default toggle-bookmark binding (matches historical hard-wire).
pub const DEFAULT_TOGGLE_BOOKMARK: &str = "Cmd+F2";

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

/// Default start-compare binding (View → Compare with Other View).
/// Shift + same key is Clear Compare.
pub const DEFAULT_COMPARE: &str = "Alt+D";

/// Default swap-compare-sides binding (View → Swap Compare Sides).
pub const DEFAULT_SWAP_COMPARE: &str = "Alt+S";

/// Default word-jump-back binding (opposite arrow jumps forward; Shift extends).
pub const DEFAULT_WORD_JUMP: &str = "Alt+Left";

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

/// Default close-find/replace binding (matches historical hard-wire).
pub const DEFAULT_CLOSE_FIND: &str = "Escape";

/// Default replace-bar binding (matches historical hard-wire).
pub const DEFAULT_REPLACE: &str = "Cmd+H";

/// Default alternate replace-bar binding (matches historical hard-wire).
pub const DEFAULT_REPLACE_ALT: &str = "Cmd+Shift+F";

/// Default select-all binding (matches historical hard-wire).
pub const DEFAULT_SELECT_ALL: &str = "Cmd+A";

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

/// Default reload-from-disk binding (new remappable chord; File → Reload).
pub const DEFAULT_RELOAD: &str = "Cmd+R";

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
        "f7" => Some(Key::F7),
        "escape" | "esc" => Some(Key::Escape),
        "equals" | "=" | "plus" => Some(Key::Equals),
        "minus" | "-" => Some(Key::Minus),
        "0" | "num0" | "digit0" => Some(Key::Num0),
        "]" | "closebracket" | "bracketright" => Some(Key::CloseBracket),
        "[" | "openbracket" | "bracketleft" => Some(Key::OpenBracket),
        "left" | "arrowleft" => Some(Key::ArrowLeft),
        "right" | "arrowright" => Some(Key::ArrowRight),
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
        Key::F7 => "F7",
        Key::Escape => "Escape",
        Key::Equals => "=",
        Key::Minus => "-",
        Key::Num0 => "0",
        Key::CloseBracket => "]",
        Key::OpenBracket => "[",
        Key::ArrowLeft => "Left",
        Key::ArrowRight => "Right",
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
    fn parse_cmd_shift_t_default_toggle_log_tail() {
        let c = parse_chord(DEFAULT_TOGGLE_LOG_TAIL).unwrap();
        assert!(c.ctrl_or_cmd);
        assert!(!c.alt);
        assert!(c.shift);
        assert_eq!(c.key, Key::T);
        assert_eq!(c.display(), "Cmd+Shift+T");
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
