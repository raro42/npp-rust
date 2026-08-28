//! In-process plugin registry for editor commands.

use format::format_document;

/// A simple in-process editor plugin.
pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn menu_path(&self) -> &str;
    /// Run on document text. `Some` means replace the buffer.
    fn run(&self, lang: &str, text: &str) -> Option<String>;
}

struct FormatDocument;
struct TrimTrailingWhitespace;
struct Uppercase;
struct Lowercase;
struct ToUnixEol;
struct ToWindowsEol;

impl Plugin for FormatDocument {
    fn id(&self) -> &str {
        "format.document"
    }
    fn name(&self) -> &str {
        "Format Document"
    }
    fn menu_path(&self) -> &str {
        "Plugins/Format Document"
    }
    fn run(&self, lang: &str, text: &str) -> Option<String> {
        Some(format_document(lang, text))
    }
}

impl Plugin for TrimTrailingWhitespace {
    fn id(&self) -> &str {
        "edit.trim_trailing"
    }
    fn name(&self) -> &str {
        "Trim Trailing Whitespace"
    }
    fn menu_path(&self) -> &str {
        "Plugins/Trim Trailing Whitespace"
    }
    fn run(&self, _lang: &str, text: &str) -> Option<String> {
        let mut out = String::with_capacity(text.len());
        for line in text.split_inclusive('\n') {
            if let Some(stripped) = line.strip_suffix("\r\n") {
                out.push_str(stripped.trim_end());
                out.push_str("\r\n");
            } else if let Some(stripped) = line.strip_suffix('\n') {
                out.push_str(stripped.trim_end());
                out.push('\n');
            } else {
                out.push_str(line.trim_end());
            }
        }
        Some(out)
    }
}

impl Plugin for Uppercase {
    fn id(&self) -> &str {
        "edit.uppercase"
    }
    fn name(&self) -> &str {
        "Uppercase"
    }
    fn menu_path(&self) -> &str {
        "Plugins/Uppercase"
    }
    fn run(&self, _lang: &str, text: &str) -> Option<String> {
        Some(text.to_uppercase())
    }
}

impl Plugin for Lowercase {
    fn id(&self) -> &str {
        "edit.lowercase"
    }
    fn name(&self) -> &str {
        "Lowercase"
    }
    fn menu_path(&self) -> &str {
        "Plugins/Lowercase"
    }
    fn run(&self, _lang: &str, text: &str) -> Option<String> {
        Some(text.to_lowercase())
    }
}

impl Plugin for ToUnixEol {
    fn id(&self) -> &str {
        "edit.to_unix_eol"
    }
    fn name(&self) -> &str {
        "To Unix EOL"
    }
    fn menu_path(&self) -> &str {
        "Plugins/To Unix EOL"
    }
    fn run(&self, _lang: &str, text: &str) -> Option<String> {
        Some(text.replace("\r\n", "\n").replace('\r', "\n"))
    }
}

impl Plugin for ToWindowsEol {
    fn id(&self) -> &str {
        "edit.to_windows_eol"
    }
    fn name(&self) -> &str {
        "To Windows EOL"
    }
    fn menu_path(&self) -> &str {
        "Plugins/To Windows EOL"
    }
    fn run(&self, _lang: &str, text: &str) -> Option<String> {
        let unix = text.replace("\r\n", "\n").replace('\r', "\n");
        Some(unix.replace('\n', "\r\n"))
    }
}

/// Host that lists builtin plugins.
pub struct PluginHost {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            plugins: vec![
                Box::new(FormatDocument),
                Box::new(TrimTrailingWhitespace),
                Box::new(Uppercase),
                Box::new(Lowercase),
                Box::new(ToUnixEol),
                Box::new(ToWindowsEol),
            ],
        }
    }

    pub fn list(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }

    pub fn get(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins.iter().find(|p| p.id() == id).map(|p| p.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_plugin_changes_python_text() {
        let host = PluginHost::new();
        let plugin = host.get("format.document").expect("format plugin");
        let src = "def f():\n\treturn 1  \n";
        let out = plugin.run("python", src).expect("output");
        assert_ne!(out, src);
        assert!(out.contains("    return 1\n"));
        assert!(!out.contains("1  "));
    }

    #[test]
    fn lists_builtins() {
        let host = PluginHost::new();
        assert!(host.list().len() >= 6);
    }
}
