//! Theme apply: egui visuals, editor chrome, and syntax token colours.
//!
//! JSON (`themes/*.json`) is the native format. A useful subset of Notepad++
//! styler XML (`themes/*.xml`) is also accepted: GlobalStyles chrome + one
//! LexerType WordsStyle map (prefers cpp / rust / python / c).

use eframe::egui::{Color32, Visuals};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct AppliedTheme {
    #[allow(dead_code)]
    pub id: String,
    pub label: String,
    pub editor_bg: Color32,
    pub gutter_bg: Color32,
    pub gutter_line: Color32,
    pub plain_fg: Color32,
    pub line_number_fg: Color32,
    pub selection_bg: Color32,
    pub caret_fg: Color32,
    pub whitespace_fg: Color32,
    pub indent_guide: Color32,
    /// Tree-sitter highlight names → colour (optional overrides).
    pub tokens: HashMap<String, Color32>,
    pub egui_dark: bool,
}

impl AppliedTheme {
    pub fn dark() -> Self {
        Self {
            id: "dark".into(),
            label: "Dark (built-in)".into(),
            editor_bg: Color32::from_rgb(30, 30, 30),
            gutter_bg: Color32::from_rgb(24, 24, 24),
            gutter_line: Color32::from_rgb(55, 55, 55),
            plain_fg: Color32::from_rgb(220, 220, 220),
            line_number_fg: Color32::from_rgb(100, 100, 100),
            selection_bg: Color32::from_rgb(50, 80, 120),
            caret_fg: Color32::from_rgb(220, 220, 220),
            whitespace_fg: Color32::from_rgb(90, 110, 140),
            indent_guide: Color32::from_rgb(55, 55, 65),
            tokens: HashMap::new(),
            egui_dark: true,
        }
    }

    pub fn light() -> Self {
        Self {
            id: "light".into(),
            label: "Light (built-in)".into(),
            editor_bg: Color32::from_rgb(245, 245, 248),
            gutter_bg: Color32::from_rgb(232, 232, 236),
            gutter_line: Color32::from_rgb(190, 190, 196),
            plain_fg: Color32::from_rgb(30, 30, 34),
            line_number_fg: Color32::from_rgb(110, 110, 120),
            selection_bg: Color32::from_rgb(180, 200, 230),
            caret_fg: Color32::from_rgb(30, 30, 34),
            whitespace_fg: Color32::from_rgb(140, 150, 165),
            indent_guide: Color32::from_rgb(200, 200, 210),
            tokens: HashMap::new(),
            egui_dark: false,
        }
    }

    pub fn visuals(&self) -> Visuals {
        if self.egui_dark {
            Visuals::dark()
        } else {
            Visuals::light()
        }
    }

    /// Resolve a highlight span name to a paint colour.
    pub fn token_color(&self, name: &str) -> Color32 {
        if name.is_empty() {
            return self.plain_fg;
        }
        if let Some(c) = self.tokens.get(name) {
            return *c;
        }
        if let Some((base, _)) = name.split_once('.') {
            if let Some(c) = self.tokens.get(base) {
                return *c;
            }
        }
        let (r, g, b) = highlight::color_for(name);
        Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }
}

#[derive(Debug, Deserialize)]
struct ThemeFile {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    egui: Option<String>,
    #[serde(default)]
    bg: Option<[u8; 3]>,
    #[serde(default)]
    fg: Option<[u8; 3]>,
    #[serde(default)]
    gutter: Option<[u8; 3]>,
    #[serde(default)]
    gutter_line: Option<[u8; 3]>,
    #[serde(default)]
    line_number: Option<[u8; 3]>,
    #[serde(default)]
    selection: Option<[u8; 3]>,
    #[serde(default)]
    caret: Option<[u8; 3]>,
    #[serde(default)]
    whitespace: Option<[u8; 3]>,
    #[serde(default)]
    indent_guide: Option<[u8; 3]>,
    #[serde(default)]
    tokens: HashMap<String, [u8; 3]>,
}

fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

fn themes_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("themes")
}

pub fn ensure_themes_dir() -> PathBuf {
    let dir = themes_dir();
    let _ = fs::create_dir_all(&dir);
    dir
}

fn is_theme_ext(ext: Option<&str>) -> bool {
    matches!(
        ext.map(|e| e.to_ascii_lowercase()).as_deref(),
        Some("json" | "xml")
    )
}

pub fn list_theme_choices() -> Vec<(String, String)> {
    let mut out = vec![
        ("dark".into(), "Dark (built-in)".into()),
        ("light".into(), "Light (built-in)".into()),
    ];
    let Ok(rd) = fs::read_dir(themes_dir()) else {
        return out;
    };
    let mut files: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| is_theme_ext(p.extension().and_then(|e| e.to_str())))
        .collect();
    files.sort();
    for path in files {
        let fname = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("theme.json");
        let id = format!("file:{fname}");
        let label = theme_label_for_path(&path).unwrap_or_else(|| fname.to_string());
        out.push((id, label));
    }
    out
}

fn theme_label_for_path(path: &Path) -> Option<String> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.eq_ignore_ascii_case("json") {
        return load_theme_file(path)
            .ok()
            .and_then(|t| t.name)
            .filter(|s| !s.is_empty());
    }
    if ext.eq_ignore_ascii_case("xml") {
        let text = fs::read_to_string(path).ok()?;
        return xml_theme_name(&text).or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
        });
    }
    None
}

fn load_theme_file(path: &Path) -> Result<ThemeFile, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn applied_from_json(theme_id: &str, name: &str, tf: ThemeFile) -> AppliedTheme {
    let base = if tf.egui.as_deref() == Some("light") {
        AppliedTheme::light()
    } else {
        AppliedTheme::dark()
    };
    let mut tokens = HashMap::new();
    for (k, v) in tf.tokens {
        tokens.insert(k, rgb(v));
    }
    AppliedTheme {
        id: theme_id.to_string(),
        label: tf.name.unwrap_or_else(|| name.to_string()),
        editor_bg: tf.bg.map(rgb).unwrap_or(base.editor_bg),
        gutter_bg: tf.gutter.map(rgb).unwrap_or(base.gutter_bg),
        gutter_line: tf.gutter_line.map(rgb).unwrap_or(base.gutter_line),
        plain_fg: tf.fg.map(rgb).unwrap_or(base.plain_fg),
        line_number_fg: tf.line_number.map(rgb).unwrap_or(base.line_number_fg),
        selection_bg: tf.selection.map(rgb).unwrap_or(base.selection_bg),
        caret_fg: tf.caret.map(rgb).unwrap_or(base.caret_fg),
        whitespace_fg: tf.whitespace.map(rgb).unwrap_or(base.whitespace_fg),
        indent_guide: tf.indent_guide.map(rgb).unwrap_or(base.indent_guide),
        tokens,
        egui_dark: tf.egui.as_deref() != Some("light"),
    }
}

pub fn resolve_theme(theme_id: &str) -> AppliedTheme {
    if theme_id == "light" {
        return AppliedTheme::light();
    }
    if theme_id == "dark" || theme_id.is_empty() {
        return AppliedTheme::dark();
    }
    if let Some(name) = theme_id.strip_prefix("file:") {
        let path = themes_dir().join(name);
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext.eq_ignore_ascii_case("json") {
            if let Ok(tf) = load_theme_file(&path) {
                return applied_from_json(theme_id, name, tf);
            }
        } else if ext.eq_ignore_ascii_case("xml") {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Some(t) = applied_from_npp_xml(theme_id, name, &text) {
                    return t;
                }
            }
        }
    }
    AppliedTheme::dark()
}

fn xml_theme_name(text: &str) -> Option<String> {
    // Optional <?npp-rs name="…"?> processing hint, else file stem via caller.
    for line in text.lines().take(20) {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("<?npp-rs") {
            if let Some(n) = xml_attr(rest, "name") {
                if !n.is_empty() {
                    return Some(n);
                }
            }
        }
    }
    None
}

/// Parse a useful Notepad++ styler subset into [`AppliedTheme`].
fn applied_from_npp_xml(theme_id: &str, file_name: &str, text: &str) -> Option<AppliedTheme> {
    let mut base = AppliedTheme::dark();
    base.id = theme_id.to_string();
    base.label = xml_theme_name(text).unwrap_or_else(|| {
        Path::new(file_name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(file_name)
            .to_string()
    });

    let mut saw_global = false;
    for tag in iter_open_tags(text, "WidgetStyle") {
        let name = xml_attr(&tag, "name").unwrap_or_default();
        let fg = xml_attr(&tag, "fgColor").and_then(|s| parse_hex_rgb(&s));
        let bg = xml_attr(&tag, "bgColor").and_then(|s| parse_hex_rgb(&s));
        match name.as_str() {
            "Default Style" => {
                saw_global = true;
                if let Some(c) = fg {
                    base.plain_fg = rgb(c);
                    base.caret_fg = rgb(c);
                }
                if let Some(c) = bg {
                    base.editor_bg = rgb(c);
                    // Dark vs light from luminance of default bg.
                    let y = (c[0] as u16 + c[1] as u16 + c[2] as u16) / 3;
                    base.egui_dark = y < 140;
                    if !base.egui_dark {
                        let light = AppliedTheme::light();
                        base.selection_bg = light.selection_bg;
                        base.indent_guide = light.indent_guide;
                        base.whitespace_fg = light.whitespace_fg;
                    }
                }
            }
            "Line number margin" => {
                saw_global = true;
                if let Some(c) = fg {
                    base.line_number_fg = rgb(c);
                }
                if let Some(c) = bg {
                    base.gutter_bg = rgb(c);
                    base.gutter_line = Color32::from_rgb(
                        c[0].saturating_add(20),
                        c[1].saturating_add(20),
                        c[2].saturating_add(20),
                    );
                }
            }
            "Selected text colour" | "Selected text color" => {
                saw_global = true;
                if let Some(c) = bg {
                    base.selection_bg = rgb(c);
                }
            }
            "Caret colour" | "Caret color" => {
                saw_global = true;
                if let Some(c) = fg {
                    base.caret_fg = rgb(c);
                }
            }
            "White space symbol" => {
                saw_global = true;
                if let Some(c) = fg {
                    base.whitespace_fg = rgb(c);
                }
            }
            "Indent guideline style" => {
                saw_global = true;
                if let Some(c) = fg {
                    base.indent_guide = rgb(c);
                }
            }
            _ => {}
        }
    }

    let tokens = tokens_from_npp_lexers(text);
    if !tokens.is_empty() {
        saw_global = true;
        base.tokens = tokens;
    }

    if saw_global {
        Some(base)
    } else {
        None
    }
}

fn tokens_from_npp_lexers(text: &str) -> HashMap<String, Color32> {
    // Prefer languages we highlight with tree-sitter.
    const PREFERRED: &[&str] = &["cpp", "c", "rust", "python", "json", "sql"];
    let mut by_lexer: HashMap<String, HashMap<String, Color32>> = HashMap::new();
    let mut current: Option<String> = None;

    for (tag_name, tag) in iter_named_open_tags(text) {
        if tag_name == "LexerType" {
            current = xml_attr(&tag, "name").map(|s| s.to_ascii_lowercase());
            continue;
        }
        if tag_name != "WordsStyle" {
            continue;
        }
        let Some(lexer) = current.clone() else {
            continue;
        };
        let style = xml_attr(&tag, "name").unwrap_or_default();
        let Some(fg) = xml_attr(&tag, "fgColor").and_then(|s| parse_hex_rgb(&s)) else {
            continue;
        };
        let Some(hl) = map_npp_style_to_highlight(&style) else {
            continue;
        };
        by_lexer
            .entry(lexer)
            .or_default()
            .entry(hl.to_string())
            .or_insert_with(|| rgb(fg));
    }

    for pref in PREFERRED {
        if let Some(map) = by_lexer.remove(*pref) {
            if !map.is_empty() {
                return map;
            }
        }
    }
    // Fall back to any lexer that yielded styles.
    by_lexer
        .into_values()
        .find(|m| !m.is_empty())
        .unwrap_or_default()
}

fn map_npp_style_to_highlight(style: &str) -> Option<&'static str> {
    let u = style.to_ascii_uppercase();
    if u.contains("COMMENT") {
        return Some("comment");
    }
    if u.contains("STRING")
        || u == "CHARACTER"
        || u == "VERBATIM"
        || u == "REGEX"
        || u.contains("TRIPLE")
    {
        return Some("string");
    }
    if u.contains("NUMBER") || u == "NUMERIC OID DEFINITION" || u == "NON OID NUMBERS" {
        return Some("number");
    }
    if u.contains("INSTRUCTION")
        || u.contains("KEYWORD")
        || u == "WORD"
        || u == "WORD2"
        || u.starts_with("USER KEYWORDS")
    {
        return Some("keyword");
    }
    if u.contains("TYPE") || u == "CLASS NAME" || u == "TYPES" {
        return Some("type");
    }
    if u.contains("FUNCTION") || u == "FUNC NAME" {
        return Some("function");
    }
    if u.contains("OPERATOR") || u == "DELIMITER" || u == "OPERATORS" {
        return Some("operator");
    }
    if u.contains("PREPROCESSOR") || u == "ANNOTATION" || u.contains("ATTRIBUTE") {
        return Some("attribute");
    }
    if u == "LABEL" {
        return Some("label");
    }
    None
}

fn parse_hex_rgb(s: &str) -> Option<[u8; 3]> {
    let s = s.trim();
    if s.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    Some([r, g, b])
}

fn xml_attr(tag: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=\"");
    let start = tag.find(&needle)? + needle.len();
    let rest = &tag[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Yield raw open-tag strings for `<name …>` / `<name …/>` (no nested parse).
fn iter_open_tags<'a>(text: &'a str, name: &'a str) -> impl Iterator<Item = String> + 'a {
    iter_named_open_tags(text)
        .filter(move |(n, _)| n == name)
        .map(|(_, tag)| tag)
}

fn iter_named_open_tags(text: &str) -> impl Iterator<Item = (String, String)> + '_ {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            i += 1;
            continue;
        }
        if i + 1 < bytes.len()
            && (bytes[i + 1] == b'/' || bytes[i + 1] == b'!' || bytes[i + 1] == b'?')
        {
            i += 1;
            continue;
        }
        let Some(end) = text[i..].find('>') else {
            break;
        };
        let tag = &text[i..i + end + 1];
        i += end + 1;
        let inner = tag
            .trim_start_matches('<')
            .trim_end_matches('>')
            .trim_end_matches('/');
        let name = inner.split_whitespace().next().unwrap_or("").to_string();
        if !name.is_empty() {
            out.push((name, tag.to_string()));
        }
    }
    out.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_tokens_override_builtin() {
        let tf = ThemeFile {
            name: Some("T".into()),
            egui: Some("dark".into()),
            bg: Some([10, 10, 10]),
            fg: Some([200, 200, 200]),
            gutter: None,
            gutter_line: None,
            line_number: None,
            selection: Some([1, 2, 3]),
            caret: Some([4, 5, 6]),
            whitespace: None,
            indent_guide: None,
            tokens: HashMap::from([("keyword".into(), [255, 0, 0])]),
        };
        let t = applied_from_json("file:t.json", "t.json", tf);
        assert_eq!(t.selection_bg, Color32::from_rgb(1, 2, 3));
        assert_eq!(t.caret_fg, Color32::from_rgb(4, 5, 6));
        assert_eq!(t.token_color("keyword"), Color32::from_rgb(255, 0, 0));
        // Nested name falls back to base key.
        assert_eq!(
            t.token_color("keyword.control"),
            Color32::from_rgb(255, 0, 0)
        );
    }

    #[test]
    fn npp_xml_global_and_cpp_tokens() {
        let xml = r#"<?npp-rs name="Mini Dark"?>
<NotepadPlus>
  <LexerStyles>
    <LexerType name="cpp" desc="C++" ext="">
      <WordsStyle name="INSTRUCTION WORD" styleID="5" fgColor="FF00AA" bgColor="3F3F3F" />
      <WordsStyle name="COMMENT LINE" styleID="2" fgColor="00FF00" bgColor="3F3F3F" />
      <WordsStyle name="STRING" styleID="6" fgColor="0000FF" bgColor="3F3F3F" />
    </LexerType>
  </LexerStyles>
  <GlobalStyles>
    <WidgetStyle name="Default Style" styleID="32" fgColor="DCDCCC" bgColor="3F3F3F" />
    <WidgetStyle name="Line number margin" styleID="33" fgColor="8A8A8A" bgColor="0C0C0C" />
    <WidgetStyle name="Selected text colour" styleID="0" bgColor="585858" />
    <WidgetStyle name="Caret colour" styleID="2069" fgColor="8FAF9F" />
    <WidgetStyle name="White space symbol" styleID="0" fgColor="5F5F5F" />
    <WidgetStyle name="Indent guideline style" styleID="37" fgColor="4F5F5F" />
  </GlobalStyles>
</NotepadPlus>
"#;
        let t = applied_from_npp_xml("file:mini.xml", "mini.xml", xml).expect("parse");
        assert_eq!(t.label, "Mini Dark");
        assert_eq!(t.editor_bg, Color32::from_rgb(0x3F, 0x3F, 0x3F));
        assert_eq!(t.plain_fg, Color32::from_rgb(0xDC, 0xDC, 0xCC));
        assert_eq!(t.gutter_bg, Color32::from_rgb(0x0C, 0x0C, 0x0C));
        assert_eq!(t.selection_bg, Color32::from_rgb(0x58, 0x58, 0x58));
        assert_eq!(t.caret_fg, Color32::from_rgb(0x8F, 0xAF, 0x9F));
        assert_eq!(
            t.token_color("keyword"),
            Color32::from_rgb(0xFF, 0x00, 0xAA)
        );
        assert_eq!(
            t.token_color("comment"),
            Color32::from_rgb(0x00, 0xFF, 0x00)
        );
        assert_eq!(t.token_color("string"), Color32::from_rgb(0x00, 0x00, 0xFF));
        assert!(t.egui_dark);
    }

    #[test]
    fn parse_hex_rgb_ok() {
        assert_eq!(parse_hex_rgb("AABBCC"), Some([0xAA, 0xBB, 0xCC]));
        assert_eq!(parse_hex_rgb("zz"), None);
    }
}
