//! Tree-sitter syntax highlight for source text.

use std::collections::HashMap;
use thiserror::Error;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

#[derive(Debug, Error)]
pub enum HighlightError {
    #[error("unknown language: {0}")]
    UnknownLanguage(String),
    #[error("highlight failed: {0}")]
    Failed(String),
}

/// One highlighted span in **char** offsets into the source.
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub name: String,
}

const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "function",
    "function.builtin",
    "keyword",
    "label",
    "module",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.special",
    "string",
    "string.escape",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "markup",
    "markup.heading",
    "markup.italic",
    "markup.bold",
    "markup.strikethrough",
    "markup.list",
    "markup.link",
    "markup.link.url",
    "markup.raw",
];

/// Syntax highlighter with cached language configs.
pub struct SyntaxHighlighter {
    configs: HashMap<String, HighlightConfiguration>,
    highlighter: Highlighter,
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        for (name, build) in [
            (
                "rust",
                make_rust as fn() -> Result<HighlightConfiguration, String>,
            ),
            ("c", make_c),
            ("cpp", make_cpp),
            ("json", make_json),
            ("python", make_python),
            ("sql", make_sql),
            ("markdown", make_markdown),
        ] {
            if let Ok(c) = build() {
                configs.insert(name.to_string(), c);
            }
        }
        Self {
            configs,
            highlighter: Highlighter::new(),
        }
    }

    pub fn supports(&self, language: &str) -> bool {
        self.configs.contains_key(language)
    }

    /// Highlight `source` for `language`. Empty for plain / unknown.
    pub fn highlight(&mut self, language: &str, source: &str) -> Result<Vec<Span>, HighlightError> {
        if language == "plain" || source.is_empty() {
            return Ok(Vec::new());
        }
        let config = self
            .configs
            .get_mut(language)
            .ok_or_else(|| HighlightError::UnknownLanguage(language.to_string()))?;

        let events = self
            .highlighter
            .highlight(config, source.as_bytes(), None, |_| None)
            .map_err(|e| HighlightError::Failed(e.to_string()))?;

        let mut spans = Vec::new();
        let mut active: Vec<usize> = Vec::new();

        for event in events {
            let event = event.map_err(|e| HighlightError::Failed(e.to_string()))?;
            match event {
                HighlightEvent::Source { start, end } => {
                    if let Some(&name_idx) = active.last() {
                        let name = HIGHLIGHT_NAMES
                            .get(name_idx)
                            .copied()
                            .unwrap_or("variable")
                            .to_string();
                        let start_b = floor_char_boundary(source, start);
                        let end_b = floor_char_boundary(source, end);
                        let start_c = source[..start_b].chars().count();
                        let end_c = source[..end_b].chars().count();
                        spans.push(Span {
                            start: start_c,
                            end: end_c,
                            name,
                        });
                    }
                }
                HighlightEvent::HighlightStart(idx) => {
                    active.push(idx.0);
                }
                HighlightEvent::HighlightEnd => {
                    active.pop();
                }
            }
        }

        Ok(spans)
    }
}

fn configure(mut config: HighlightConfiguration) -> HighlightConfiguration {
    config.configure(HIGHLIGHT_NAMES);
    config
}

fn make_rust() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_c() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_c::LANGUAGE.into(),
            "c",
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_cpp() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_cpp::LANGUAGE.into(),
            "cpp",
            tree_sitter_cpp::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_json() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_python() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_sql() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_sequel::LANGUAGE.into(),
            "sql",
            tree_sitter_sequel::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn make_markdown() -> Result<HighlightConfiguration, String> {
    Ok(configure(
        HighlightConfiguration::new(
            tree_sitter_md::LANGUAGE.into(),
            "markdown",
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            "",
        )
        .map_err(|e| e.to_string())?,
    ))
}

fn floor_char_boundary(s: &str, mut i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

/// Map highlight name to RGB (0–1).
pub fn color_for(name: &str) -> (f32, f32, f32) {
    match name {
        "comment" => (0.45, 0.55, 0.45),
        "string" | "string.special" | "string.escape" | "markup.raw" => (0.80, 0.45, 0.35),
        "keyword" => (0.75, 0.35, 0.70),
        "number" | "constant" | "constant.builtin" => (0.70, 0.55, 0.30),
        "function" | "function.builtin" | "constructor" => (0.35, 0.55, 0.85),
        "type" | "type.builtin" | "module" => (0.40, 0.70, 0.70),
        "property" | "attribute" | "label" => (0.60, 0.65, 0.40),
        "operator"
        | "punctuation"
        | "punctuation.bracket"
        | "punctuation.delimiter"
        | "punctuation.special" => (0.70, 0.70, 0.70),
        "markup.heading" => (0.55, 0.70, 0.90),
        "markup.link" | "markup.link.url" | "markup.list" => (0.50, 0.65, 0.80),
        "markup.bold" | "markup.italic" | "markup" => (0.85, 0.80, 0.55),
        _ => (0.85, 0.85, 0.85),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlights_rust_keyword() {
        let mut h = SyntaxHighlighter::new();
        assert!(h.supports("rust"));
        let spans = h.highlight("rust", "fn main() {}").unwrap();
        assert!(!spans.is_empty());
    }

    #[test]
    fn supports_python_sql_markdown() {
        let h = SyntaxHighlighter::new();
        assert!(h.supports("python"));
        assert!(h.supports("sql"));
        assert!(h.supports("markdown"));
    }

    #[test]
    fn color_for_keyword() {
        let (r, g, b) = color_for("keyword");
        assert!(r > 0.0 && g > 0.0 && b > 0.0);
    }
}
