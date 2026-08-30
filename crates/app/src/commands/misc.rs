//! Tools / Window / Settings menu commands.
use super::common::*;
use super::{CmdResult, UiFlags};
use crate::editor::EditorState;
use std::path::PathBuf;

pub fn covers(cmd: &str) -> bool {
    cmd.starts_with("IDM_TOOL_")
        || cmd.starts_with("IDM_WINDOW_")
        || cmd.starts_with("IDM_SETTING_")
        || cmd.starts_with("IDM_EXECUTE")
}

pub fn try_dispatch(cmd: &str, state: &mut EditorState, ui: &mut UiFlags) -> Option<CmdResult> {
    if !covers(cmd) {
        return None;
    }
    Some(match cmd {
        "IDM_SETTING_PLUGINADM" => {
            show_plugin_admin(state);
            CmdResult::Handled
        }
        "IDM_SETTING_SHORTCUT_MAPPER" => {
            show_shortcut_mapper(state);
            CmdResult::Handled
        }
        "IDM_SETTING_PREFERENCE" => {
            ui.show_preferences = true;
            CmdResult::Handled
        }
        "IDM_LANGSTYLE_CONFIG_DLG" => {
            show_style_config(state);
            CmdResult::Handled
        }
        "IDM_SETTING_IMPORTPLUGIN" => {
            import_plugins(state);
            CmdResult::Handled
        }
        "IDM_SETTING_IMPORTSTYLETHEMES" => {
            import_themes(state, ui);
            CmdResult::Handled
        }
        "IDM_WINDOW_WINDOWS" => {
            ui.show_doc_list = true;
            CmdResult::Handled
        }
        "IDM_EXECUTE" => {
            run_execute(state);
            CmdResult::Handled
        }
        "IDM_EXECUTE_VALIDATE_SHORTCUTSXML" => {
            validate_shortcuts_xml(state);
            CmdResult::Handled
        }
        "IDM_TOOL_MD5_GENERATE" | "IDM_TOOL_MD5_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "md5", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_MD5_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "md5");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA1_GENERATE" | "IDM_TOOL_SHA1_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha1", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA1_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha1");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA256_GENERATE" | "IDM_TOOL_SHA256_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha256", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA256_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha256");
            CmdResult::Handled
        }
        "IDM_TOOL_SHA512_GENERATE" | "IDM_TOOL_SHA512_GENERATEINTOCLIPBOARD" => {
            hash_selection_or_doc(state, ui, "sha512", cmd.ends_with("CLIPBOARD"));
            CmdResult::Handled
        }
        "IDM_TOOL_SHA512_GENERATEFROMFILE" => {
            hash_active_file(state, ui, "sha512");
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FN_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
            state.status = "Tabs sorted by name ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FN_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| b.title.to_lowercase().cmp(&a.title.to_lowercase()));
            state.status = "Tabs sorted by name ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_ASC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let bp = b
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                ap.cmp(&bp)
            });
            state.status = "Tabs sorted by path ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FP_DSC" => {
            state.tabs.sort_tabs(|a, b| {
                let ap = a
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let bp = b
                    .path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                bp.cmp(&ap)
            });
            state.status = "Tabs sorted by path ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| tab_type_key(a).cmp(&tab_type_key(b)));
            state.status = "Tabs sorted by type ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FT_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| tab_type_key(b).cmp(&tab_type_key(a)));
            state.status = "Tabs sorted by type ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_ASC" => {
            state
                .tabs
                .sort_tabs(|a, b| a.buffer.len_chars().cmp(&b.buffer.len_chars()));
            state.status = "Tabs sorted by size ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FS_DSC" => {
            state
                .tabs
                .sort_tabs(|a, b| b.buffer.len_chars().cmp(&a.buffer.len_chars()));
            state.status = "Tabs sorted by size ↓".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FD_ASC" => {
            state.tabs.sort_tabs(|a, b| tab_mtime(a).cmp(&tab_mtime(b)));
            state.status = "Tabs sorted by modified ↑".into();
            CmdResult::Handled
        }
        "IDM_WINDOW_SORT_FD_DSC" => {
            state.tabs.sort_tabs(|a, b| tab_mtime(b).cmp(&tab_mtime(a)));
            state.status = "Tabs sorted by modified ↓".into();
            CmdResult::Handled
        }
        "IDM_SETTING_OPENPLUGINSDIR" => {
            let dir = cwd();
            open_path_in_os(state, &dir);
            CmdResult::Handled
        }
        _ => CmdResult::Stub,
    })
}

fn cwd() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Open an untitled read-only info tab (same pattern as Debug Info).
fn open_info_tab(state: &mut EditorState, title: &str, text: &str) {
    state.tabs.open_untitled();
    {
        let doc = state.tabs.active_mut();
        doc.title = title.into();
        doc.buffer = buffer::TextBuffer::from_str(text);
        doc.dirty = false;
        doc.language = "plain".into();
        doc.read_only = true;
    }
    state.highlight_dirty = true;
    state.reset_view = true;
}

fn show_shortcut_mapper(state: &mut EditorState) {
    // Mirrors crates/app/src/ui.rs handle_shortcuts (read-only dump).
    let text = "\
npp-rs keyboard shortcuts
=========================
Source: ui.rs handle_shortcuts (hard-wired; no shortcuts.xml yet).
Settings → Validate shortcuts.xml reports presence only.

modifier notes
--------------
- Cmd on macOS, Ctrl elsewhere (egui command/ctrl).
- Shift means Shift held with the key.

File / edit
-----------
Cmd+N                 New file
Cmd+O                 Open…
Cmd+S                 Save
Cmd+Shift+S           Save As…
Cmd+W                 Close tab
Cmd+Z                 Undo
Cmd+Shift+Z / Cmd+Y   Redo
Cmd+A                 Select all
Cmd+D                 Duplicate line
Cmd+Shift+L           Delete line
Cmd+]                 Indent lines (4 spaces)
Cmd+[                 Outdent lines (4 spaces)
Cmd+Shift+I           Format Document (format.document plugin)

Find
----
Cmd+F                 Find
Cmd+Shift+F           Replace
Cmd+G                 Find next (when Find/Replace open)
Cmd+Shift+G           Find previous (when Find/Replace open)
Escape                Close Find/Replace

View / monitoring
-----------------
Cmd+Shift+T           Toggle log tail follow

Language / style
----------------
Use the Language menu (IDM_LANG_*) to set highlight via EditorState::set_language.
Style Configurator lists which langs have tree-sitter grammars.
Preferences (Settings → Preferences) sets log-tail policy and font size.
";
    open_info_tab(state, "Shortcut Mapper", text);
    state.status = "Shortcut Mapper opened".into();
}

fn show_style_config(state: &mut EditorState) {
    let current = state.tabs.active().language.clone();
    let candidates = [
        "rust",
        "c",
        "cpp",
        "json",
        "python",
        "sql",
        "markdown",
        "plain",
        "toml",
        "yaml",
        "shell",
        "javascript",
        "typescript",
        "html",
        "css",
        "go",
        "java",
    ];
    let mut with_hl = Vec::new();
    let mut without_hl = Vec::new();
    for lang in candidates {
        if lang == "plain" || state.highlighter.supports(lang) {
            with_hl.push(lang);
        } else {
            without_hl.push(lang);
        }
    }
    let mut text = format!(
        "\
npp-rs Style Configurator
=========================
Current document language: {current}

Tree-sitter highlight (highlighter.supports)
--------------------------------------------
"
    );
    for lang in &with_hl {
        let mark = if *lang == current.as_str() {
            "  ← current"
        } else {
            ""
        };
        text.push_str(&format!("  {lang}{mark}\n"));
    }
    text.push_str(
        "\n\
Detected / menu langs without highlight grammar
-----------------------------------------------
",
    );
    for lang in &without_hl {
        let mark = if *lang == current.as_str() {
            "  ← current"
        } else {
            ""
        };
        text.push_str(&format!("  {lang}{mark}\n"));
    }
    text.push_str(
        "\n\
How to set language / look
--------------------------
1. Language menu (IDM_LANG_*) → EditorState::set_language.
2. Preferences → font size (session slider; log-tail policy saves).
3. Settings → Import Style Theme(s) lists themes/ and opens that folder.
   There is no theme apply API yet (egui default look).
There is no full Style Configurator colour UI yet.
",
    );
    open_info_tab(state, "Style Configurator", &text);
    state.status = format!("Style Config: language={current}");
}

fn format_plugin_rows(host: &plugins::PluginHost) -> String {
    let mut text = String::new();
    for row in host.summaries() {
        text.push_str(&format!(
            "  {name}\n    id:   {id}\n    menu: {menu}\n    run:  Plugins menu → {name}\n\n",
            name = row.name,
            id = row.id,
            menu = row.menu_path,
        ));
    }
    text
}

fn list_folder_entries(dir: &PathBuf) -> (usize, String) {
    let mut names = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for ent in rd.flatten() {
            let name = ent.file_name().to_string_lossy().into_owned();
            let kind = if ent.path().is_dir() { "/" } else { "" };
            names.push(format!("{name}{kind}"));
        }
    }
    names.sort();
    let n = names.len();
    let body = if names.is_empty() {
        "  (empty)\n".into()
    } else {
        names
            .into_iter()
            .map(|n| format!("  {n}\n"))
            .collect::<String>()
    };
    (n, body)
}

fn show_plugin_admin(state: &mut EditorState) {
    let host = plugins::PluginHost::new();
    let n = host.summaries().len();
    let plugins_dir = cwd().join("plugins");
    let _ = std::fs::create_dir_all(&plugins_dir);
    let (disk_n, disk_list) = list_folder_entries(&plugins_dir);

    let mut text = format!(
        "\
npp-rs Plugin Admin
===================
In-process plugins only (crates/plugins). DLL / drop-in loaders are not supported.

Installed ({n})
-------------
"
    );
    text.push_str(&format_plugin_rows(&host));
    text.push_str(&format!(
        "\
plugins/ on disk ({disk_n} entries)
--------------------------------
Path: {path}

{disk}
Action
------
- Run a builtin: Plugins menu (uses PluginHost id).
- Format Document: also Cmd+Shift+I.
- Import Plugin: opens plugins/ and refreshes this listing in a tab.
- Drop-in files in plugins/ are listed only; they do not load.
",
        path = plugins_dir.display(),
        disk = disk_list,
    ));
    open_info_tab(state, "Plugin Admin", &text);
    state.status = format!("Plugin Admin: {n} in-process, {disk_n} on disk in plugins/");
}

fn import_plugins(state: &mut EditorState) {
    let dir = cwd().join("plugins");
    let _ = std::fs::create_dir_all(&dir);
    open_path_in_os(state, &dir);

    let host = plugins::PluginHost::new();
    let n = host.summaries().len();
    let (disk_n, disk_list) = list_folder_entries(&dir);

    let mut text = format!(
        "\
npp-rs Import Plugin
====================
Opened folder: {path}

Limit
-----
Drop-in / external plugins do not load. Only in-process builtins from crates/plugins run.

In-process builtins ({n})
-----------------------
",
        path = dir.display(),
    );
    text.push_str(&format_plugin_rows(&host));
    text.push_str(&format!(
        "\
Current plugins/ listing ({disk_n})
----------------------------------
{disk}
What you can do
---------------
1. Use Plugins menu items for the builtins above.
2. Put files in plugins/ for your own notes; reopen Import Plugin to refresh this list.
3. Plugin Admin shows the same registry + folder snapshot.
",
        disk = disk_list,
    ));
    open_info_tab(state, "Import Plugin", &text);
    state.status = format!(
        "Import Plugin: opened plugins/ — {n} builtin(s), {disk_n} on disk (drop-in not loaded)"
    );
}

fn import_themes(state: &mut EditorState, ui: &mut UiFlags) {
    let dir = crate::theme::ensure_themes_dir();
    open_path_in_os(state, &dir);
    ui.show_theme_picker = true;
    let n = crate::theme::list_theme_choices().len();
    state.status =
        format!("Themes: opened themes/ — {n} choice(s). Use the Theme window to apply (JSON tokens or N++ XML subset).");
}

fn run_execute(state: &mut EditorState) {
    if let Some(path) = rfd::FileDialog::new().set_title("Run…").pick_file() {
        let label = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "program".into());
        match std::process::Command::new(&path).spawn() {
            Ok(_) => state.status = format!("Started {label}"),
            Err(e) => state.status = format!("Run failed: {e}"),
        }
    } else {
        state.open_shell_here();
    }
}

fn validate_shortcuts_xml(state: &mut EditorState) {
    let root = cwd();
    let candidates = [
        "shortcuts.xml",
        "npp-rs/shortcuts.xml",
        "crates/app/data/shortcuts.xml",
    ];
    for rel in candidates {
        let path = root.join(rel);
        if path.is_file() {
            state.status = format!(
                "Found {rel} — XML validator not wired yet (shortcuts still hard-coded in ui.rs)"
            );
            return;
        }
    }
    state.status =
        "Validate shortcuts.xml: file absent (checked shortcuts.xml, npp-rs/, crates/app/data/)"
            .into();
}
