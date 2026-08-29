//! Theme apply MVP: egui visuals + editor bg/fg/gutter.

use eframe::egui::{Color32, Visuals};
use serde::Deserialize;
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
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
        })
        .collect();
    files.sort();
    for path in files {
        let fname = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("theme.json");
        let id = format!("file:{fname}");
        let label = load_theme_file(&path)
            .ok()
            .and_then(|t| t.name)
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| fname.to_string());
        out.push((id, label));
    }
    out
}

fn load_theme_file(path: &Path) -> Result<ThemeFile, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
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
        if let Ok(tf) = load_theme_file(&path) {
            let base = if tf.egui.as_deref() == Some("light") {
                AppliedTheme::light()
            } else {
                AppliedTheme::dark()
            };
            return AppliedTheme {
                id: theme_id.to_string(),
                label: tf.name.clone().unwrap_or_else(|| name.to_string()),
                editor_bg: tf.bg.map(rgb).unwrap_or(base.editor_bg),
                gutter_bg: tf.gutter.map(rgb).unwrap_or(base.gutter_bg),
                gutter_line: tf.gutter_line.map(rgb).unwrap_or(base.gutter_line),
                plain_fg: tf.fg.map(rgb).unwrap_or(base.plain_fg),
                line_number_fg: tf.line_number.map(rgb).unwrap_or(base.line_number_fg),
                egui_dark: tf.egui.as_deref() != Some("light"),
            };
        }
    }
    AppliedTheme::dark()
}
