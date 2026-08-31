//! egui shell: menus, tabs, viewport editor, find bar.

use crate::editor::EditorState;
use crate::ui_paint::{
    change_history_joins, change_history_wash, col_from_x, display_row_for,
    paint_change_history_bar, paint_line_text, style_mark_bg, text_width, visible_line_indices,
};
use eframe::egui::{self, Color32, CursorIcon, FontId, Key, Pos2, Rect, RichText, Sense, Vec2};
use std::path::PathBuf;

/// Soft teal — ready menu items (works on light and dark themes).
const MENU_READY: Color32 = Color32::from_rgb(42, 148, 118);

/// Argv options passed from `main` into the UI shell.
#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub paths: Vec<std::path::PathBuf>,
    /// Jump to this 1-based line in the first opened file.
    pub goto_line: Option<usize>,
    /// Mark argv-opened files read-only.
    pub read_only: bool,
}

/// Which dual-view pane receives keyboard edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorPane {
    Primary,
    Secondary,
}

/// In-progress drag of selected text (move, or copy with Ctrl/Cmd).
struct SelTextDrag {
    tab: usize,
    drop_at: usize,
}

/// Choose the right-hand tab for Compare.
///
/// Order: marked partner → dual-view other pane → tab to the right → tab to the left.
fn pick_compare_right(
    n: usize,
    active: usize,
    partner: Option<usize>,
    dual_view: bool,
    other_view_tab: usize,
) -> Option<usize> {
    if n < 2 {
        return None;
    }
    if let Some(p) = partner {
        if p < n && p != active {
            return Some(p);
        }
    }
    if dual_view && other_view_tab < n && other_view_tab != active {
        return Some(other_view_tab);
    }
    if active + 1 < n {
        return Some(active + 1);
    }
    if active > 0 {
        return Some(active - 1);
    }
    None
}

pub struct EditorApp {
    state: EditorState,
    find_focus_once: bool,
    /// Vertical scroll in lines.
    scroll_line: f32,
    /// When true, next paint scrolls so the caret stays in view.
    follow_caret: bool,
    /// Caret-follow for the secondary pane.
    follow_caret_other: bool,
    show_about: bool,
    show_preferences: bool,
    /// Checkbox state for the log-tail prompt.
    log_tail_remember: bool,
    /// Drag-select anchor (char index), while primary button is held.
    drag_anchor: Option<usize>,
    /// Alt+drag rectangular / column selection in progress.
    rect_drag: bool,
    /// Drag selected text to move (or Ctrl/Cmd+drag to copy).
    sel_text_drag: Option<SelTextDrag>,
    /// Tab bar drag-reorder: source index while the pointer drags a tab.
    tab_drag_from: Option<usize>,
    show_replace: bool,
    replace_with: String,
    /// Friendly dialog for menu items not wired yet.
    coming_soon: Option<crate::commands::ComingSoon>,
    /// Editor monospace size (zoom).
    font_size: f32,
    show_goto_line: bool,
    goto_line_input: String,
    show_summary: bool,
    show_doc_list: bool,
    show_project_panel: bool,
    show_theme_picker: bool,
    show_doc_map: bool,
    show_func_list: bool,
    show_char_panel: bool,
    /// Last text copied via menu (`pending_clipboard`).
    last_app_clipboard: Option<String>,
    /// Next Paste replaces bookmarked lines.
    await_paste_bookmarks: bool,
    /// Second editor pane (writable).
    dual_view: bool,
    /// Tab index shown in the secondary pane.
    other_view_tab: usize,
    /// Pane that owns keyboard typing / caret keys.
    focused_pane: EditorPane,
    /// Vertical scroll for the secondary pane (lines).
    scroll_line_other: f32,
    /// Sync vertical scroll between panes.
    sync_scroll_v: bool,
    /// Sync flag for horizontal (MVP shares line scroll with V when either is on).
    sync_scroll_h: bool,
    /// Session flag: both panes share font size (always true in practice).
    zoom_sync: bool,
    /// 2-way compare mode (colours + dual view).
    compare_on: bool,
    compare_left_tab: usize,
    compare_right_tab: usize,
    compare_left_tags: Vec<crate::diff::LineKind>,
    compare_right_tags: Vec<crate::diff::LineKind>,
    /// When set, wait until this instant before re-diff (debounce while typing).
    compare_refresh_at: Option<std::time::Instant>,
    /// Optional second tab for Compare (⌘/Ctrl-click a tab, or context menu).
    compare_partner_tab: Option<usize>,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, cli: CliOptions) -> Self {
        let state = EditorState::new();
        let font_size = state.settings.font_size.clamp(8.0, 48.0);
        let mut app = Self {
            state,
            find_focus_once: false,
            scroll_line: 0.0,
            follow_caret: false,
            follow_caret_other: false,
            show_about: false,
            show_preferences: false,
            log_tail_remember: true,
            drag_anchor: None,
            rect_drag: false,
            sel_text_drag: None,
            tab_drag_from: None,
            show_replace: false,
            replace_with: String::new(),
            coming_soon: None,
            font_size,
            show_goto_line: false,
            goto_line_input: String::new(),
            show_summary: false,
            show_doc_list: false,
            show_project_panel: false,
            show_theme_picker: false,
            show_doc_map: false,
            show_func_list: false,
            show_char_panel: false,
            last_app_clipboard: None,
            await_paste_bookmarks: false,
            dual_view: false,
            other_view_tab: 0,
            focused_pane: EditorPane::Primary,
            scroll_line_other: 0.0,
            sync_scroll_v: false,
            sync_scroll_h: false,
            zoom_sync: false,
            compare_on: false,
            compare_left_tab: 0,
            compare_right_tab: 0,
            compare_left_tags: Vec::new(),
            compare_right_tags: Vec::new(),
            compare_refresh_at: None,
            compare_partner_tab: None,
        };
        let had_argv = !cli.paths.is_empty();
        app.replace_with = app.state.settings.replace_with.clone();
        app.open_argv_paths(cli);
        if !had_argv && app.state.settings.restore_session {
            app.state.restore_session_from_disk();
        }
        app
    }

    /// Open existing argv paths; skip missing and report on the status line.
    fn open_argv_paths(&mut self, cli: CliOptions) {
        if cli.paths.is_empty() {
            return;
        }
        let mut opened = 0usize;
        let mut missing: Vec<String> = Vec::new();
        let mut first_opened_tab: Option<usize> = None;
        for path in cli.paths {
            if path.exists() {
                self.state.open_path(path);
                let tab = self.state.tabs.active_index();
                if cli.read_only {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.read_only = true;
                    }
                }
                if first_opened_tab.is_none() {
                    first_opened_tab = Some(tab);
                }
                opened += 1;
            } else {
                missing.push(path.display().to_string());
            }
        }
        if opened > 0 && self.state.tabs.len() > 1 {
            if let Some(doc) = self.state.tabs.get(0) {
                if doc.path.is_none() && !doc.dirty && doc.buffer.is_empty() {
                    self.state.close_tab(0);
                    if let Some(t) = first_opened_tab.as_mut() {
                        *t = t.saturating_sub(1);
                    }
                }
            }
        }
        if let (Some(tab), Some(line_1based)) = (first_opened_tab, cli.goto_line) {
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let line = line_1based.saturating_sub(1);
                let max_line = doc.buffer.line_count().saturating_sub(1);
                let line = line.min(max_line);
                let pos = doc.buffer.line_to_char(line);
                doc.buffer.set_caret(pos);
                self.state.tabs.set_active(tab);
                self.follow_caret = true;
                self.scroll_line = line as f32;
            }
        }
        if !missing.is_empty() {
            let skip = missing.join(", ");
            self.state.status = if opened > 0 {
                format!("Opened {opened} file(s); skipped missing: {skip}")
            } else {
                format!("Skipped missing: {skip}")
            };
        } else if opened > 1 {
            self.state.status = format!("Opened {opened} file(s) from command line");
        }
        if cli.read_only && opened > 0 {
            self.state.status = format!("{} (read-only)", self.state.status);
        }
        if let Some(n) = cli.goto_line {
            if opened > 0 {
                self.state.status = format!("{} → line {n}", self.state.status);
            }
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            self.state.persist_session_if_enabled();
            let dirty = self.state.tabs.iter().any(|d| d.dirty);
            if dirty || self.state.pending_close.is_some() {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                if self.state.pending_close.is_none() {
                    let mut flags = crate::commands::UiFlags::default();
                    self.state.request_quit(&mut flags);
                    if flags.request_quit {
                        self.state.persist_session_if_enabled();
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            }
        }

        self.state.poll_loads();
        if self.state.poll_tail() {
            self.follow_caret = true;
        }
        let _ = self.state.tick_autosave();
        if !self.state.pending.is_empty()
            || self.state.tabs.iter().any(|d| d.tail_follow)
            || self.state.settings.autosave_secs() > 0
        {
            // Steady poll while tailing or autosave is armed; avoid hammering every frame.
            ctx.request_repaint_after(std::time::Duration::from_millis(300));
        }

        self.apply_theme_visuals(ctx);
        self.handle_file_drops(ctx);
        self.handle_shortcuts(ctx);
        self.menu_bar(ctx);
        self.refresh_compare_if_stale(ctx);
        self.tab_bar(ctx);
        if self.state.find_open || self.show_replace {
            self.find_replace_bar(ctx);
        }
        // Bottom panel before CentralPanel so the editor height excludes the status bar.
        self.status_bar(ctx);
        self.editor_pane(ctx);
        self.about_window(ctx);
        self.preferences_window(ctx);
        self.log_tail_prompt_window(ctx);
        self.encoding_notice_window(ctx);
        self.lossy_ansi_confirm_window(ctx);
        self.unsaved_close_window(ctx);
        self.unsaved_reload_window(ctx);
        self.coming_soon_window(ctx);
        self.goto_line_window(ctx);
        self.summary_window(ctx);
        self.doc_list_window(ctx);
        self.project_panel_window(ctx);
        self.theme_picker_window(ctx);
        self.doc_map_window(ctx);
        self.func_list_window(ctx);
        self.char_panel_window(ctx);
    }
}

impl EditorApp {
    /// Open paths dropped onto the window (skip folders / missing).
    fn handle_file_drops(&mut self, ctx: &egui::Context) {
        let hovering = ctx.input(|i| !i.raw.hovered_files.is_empty());
        if hovering {
            ctx.set_cursor_icon(CursorIcon::Copy);
            if self.state.status.is_empty() || !self.state.status.starts_with("Drop ") {
                self.state.status = "Drop files here to open…".into();
            }
        }

        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect()
        });
        if dropped.is_empty() {
            return;
        }

        let mut opened = 0usize;
        let mut skipped = 0usize;
        for path in dropped {
            if path.is_file() {
                self.state.open_path(path);
                opened += 1;
            } else {
                skipped += 1;
            }
        }
        self.state.status = if opened == 0 && skipped == 0 {
            "Drop ignored (no local paths)".into()
        } else if skipped == 0 {
            format!("Opened {opened} dropped file(s)")
        } else if opened == 0 {
            format!("Skipped {skipped} dropped path(s) (not files)")
        } else {
            format!("Opened {opened} dropped file(s); skipped {skipped}")
        };
    }

    fn finish_sel_text_drag(&mut self, copy: bool) {
        let Some(drag) = self.sel_text_drag.take() else {
            return;
        };
        let Some(doc) = self.state.tabs.get_mut(drag.tab) else {
            return;
        };
        if doc.read_only {
            self.state.status = "Read-only — cannot move or copy selection".into();
            return;
        }
        let ok = doc.buffer.drag_selection_to(drag.drop_at, copy);
        if ok {
            self.state.mark_text_changed_at(drag.tab);
            self.state.status = if copy {
                "Copied selection by drag".into()
            } else {
                "Moved selection by drag".into()
            };
            if drag.tab == self.state.tabs.active_index() {
                self.follow_caret = true;
            } else {
                self.follow_caret_other = true;
            }
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| {
            (
                i.modifiers,
                i.key_pressed(Key::N),
                i.key_pressed(Key::O),
                i.key_pressed(Key::S),
                i.key_pressed(Key::F),
                i.key_pressed(Key::Z),
                i.key_pressed(Key::Y),
                i.key_pressed(Key::W),
                i.key_pressed(Key::G),
                i.key_pressed(Key::A),
                i.key_pressed(Key::D),
                i.key_pressed(Key::L),
                i.key_pressed(Key::I),
                i.key_pressed(Key::T),
                i.key_pressed(Key::H),
                i.key_pressed(Key::F2),
                i.key_pressed(Key::F3),
                i.key_pressed(Key::Equals),
                i.key_pressed(Key::Minus),
                i.key_pressed(Key::Num0),
                i.key_pressed(Key::CloseBracket),
                i.key_pressed(Key::OpenBracket),
                i.raw_scroll_delta.y,
            )
        });
        let (
            mods,
            n,
            o,
            s,
            f,
            z,
            y,
            w,
            g,
            a,
            d,
            l,
            i_key,
            t,
            h,
            f2,
            f3,
            equals,
            minus,
            num0,
            close_br,
            open_br,
            scroll_y,
        ) = input;
        let cmd = mods.command || mods.ctrl;

        if cmd && mods.shift && t && self.state.toggle_tail_follow() {
            self.follow_caret = true;
        }
        if cmd && n {
            self.state.new_file();
        }
        if cmd && o {
            self.state.open_dialog();
        }
        if cmd && s && mods.shift {
            self.state.save_as_dialog();
        } else if cmd && s {
            self.state.save();
        }
        if (cmd && h && !mods.shift) || (cmd && f && mods.shift) {
            self.show_replace = true;
            self.state.find_open = true;
            self.find_focus_once = true;
            self.seed_find_from_selection();
        } else if cmd && f {
            self.state.find_open = true;
            self.show_replace = false;
            self.find_focus_once = true;
            self.seed_find_from_selection();
        }
        if cmd && a {
            let tab = self.focused_edit_tab();
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.select_all();
            }
        }
        if cmd && d {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.duplicate_line();
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && l && mods.shift {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                doc.buffer.delete_line();
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        } else if cmd && l {
            self.open_goto_line_dialog();
        }
        if cmd && close_br {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let n = self.state.settings.tab_width.max(1) as usize;
                let pad = " ".repeat(n);
                doc.buffer.indent_lines(&pad);
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && open_br {
            let tab = self.focused_edit_tab();
            self.state.prepare_edit_at(tab);
            if let Some(doc) = self.state.tabs.get_mut(tab) {
                let n = self.state.settings.tab_width.max(1) as usize;
                doc.buffer.outdent_lines(n);
            }
            self.state.mark_text_changed_at(tab);
            self.follow_focused_caret();
        }
        if cmd && mods.shift && i_key {
            self.state.format_document();
            self.follow_caret = true;
        }

        if cmd && z && mods.shift {
            self.state.redo_at(self.focused_edit_tab());
        } else if cmd && z {
            self.state.undo_at(self.focused_edit_tab());
        } else if mods.alt && z && !cmd {
            self.state.word_wrap = !self.state.word_wrap;
            self.state.settings.word_wrap = self.state.word_wrap;
            self.state.settings.save();
            self.state.status = format!(
                "Word wrap: {}",
                if self.state.word_wrap { "on" } else { "off" }
            );
        }
        if cmd && y {
            self.state.redo_at(self.focused_edit_tab());
        }
        if cmd && w {
            let idx = self.state.tabs.active_index();
            self.state.request_close_tab(idx);
            self.scroll_line = 0.0;
        }

        // Find next/prev: F3 / Shift+F3 and Cmd+G / Cmd+Shift+G (global; Find bar optional).
        let find_prev_key = (f3 && mods.shift) || (cmd && g && mods.shift);
        let find_next_key = (f3 && !mods.shift) || (cmd && g && !mods.shift);
        if find_prev_key {
            if self.state.find_query.is_empty() {
                self.seed_find_from_selection();
            }
            self.state.find_prev();
            self.follow_focused_caret();
        } else if find_next_key {
            if self.state.find_query.is_empty() {
                self.seed_find_from_selection();
            }
            self.state.find_next();
            self.follow_focused_caret();
        }

        // Bookmarks: F2 next, Shift+F2 prev, Cmd+F2 toggle.
        if f2 && cmd {
            self.run_shortcut_cmd("IDM_SEARCH_TOGGLE_BOOKMARK");
        } else if f2 && mods.shift {
            self.run_shortcut_cmd("IDM_SEARCH_PREV_BOOKMARK");
        } else if f2 {
            self.run_shortcut_cmd("IDM_SEARCH_NEXT_BOOKMARK");
        }

        // Zoom: Cmd+= / Cmd+- / Cmd+0, and Cmd+mouse wheel.
        if cmd && equals {
            self.font_size = (self.font_size + 1.0).min(48.0);
            self.persist_font_size();
        } else if cmd && minus {
            self.font_size = (self.font_size - 1.0).max(8.0);
            self.persist_font_size();
        } else if cmd && num0 {
            self.font_size = 14.0;
            self.persist_font_size();
        } else if cmd && scroll_y != 0.0 {
            if scroll_y > 0.0 {
                self.font_size = (self.font_size + 1.0).min(48.0);
            } else {
                self.font_size = (self.font_size - 1.0).max(8.0);
            }
            self.persist_font_size();
        }

        if (self.state.find_open || self.show_replace) && ctx.input(|i| i.key_pressed(Key::Escape))
        {
            self.state.find_open = false;
            self.show_replace = false;
        }
    }

    fn open_goto_line_dialog(&mut self) {
        self.show_goto_line = true;
        let line = self
            .state
            .tabs
            .active()
            .buffer
            .char_to_line(self.state.tabs.active().buffer.caret())
            + 1;
        self.goto_line_input = line.to_string();
    }

    fn run_shortcut_cmd(&mut self, cmd: &str) {
        let mut flags = crate::commands::UiFlags::default();
        let _ = self.dispatch_menu_cmd(cmd, &mut flags);
        if flags.follow_caret {
            self.follow_focused_caret();
        }
        if flags.show_goto_line {
            self.open_goto_line_dialog();
        }
        match flags.zoom_delta {
            Some(1) => {
                self.font_size = (self.font_size + 1.0).min(48.0);
                self.persist_font_size();
            }
            Some(-1) => {
                self.font_size = (self.font_size - 1.0).max(8.0);
                self.persist_font_size();
            }
            Some(0) => {
                self.font_size = 14.0;
                self.persist_font_size();
            }
            _ => {}
        }
    }

    fn menu_bar(&mut self, ctx: &egui::Context) {
        let menu = crate::menu_data::load_npp_menu();
        let mut flags = crate::commands::UiFlags {
            show_about: self.show_about,
            show_preferences: self.show_preferences,
            find_open: self.state.find_open,
            show_replace: self.show_replace,
            find_focus_once: self.find_focus_once,
            follow_caret: self.follow_caret,
            last_copied: self.last_app_clipboard.clone(),
            ..Default::default()
        };
        let mut run_cmd: Option<String> = None;

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                for node in &menu {
                    self.render_menu_node(ui, node, &mut run_cmd, true);
                }
            });
        });

        if let Some(cmd) = run_cmd {
            // Edit/Format menus mutate the focused dual-view pane, not only the tab-bar active tab.
            let result = self.dispatch_menu_cmd(&cmd, &mut flags);
            if result == crate::commands::CmdResult::Stub {
                self.coming_soon = Some(crate::commands::coming_soon_for(&cmd));
                if let Some(cs) = self.coming_soon.as_ref() {
                    self.state.status = format!("Coming soon: {}", cs.feature);
                }
            }
            if flags.coming_soon.is_some() {
                self.coming_soon = flags.coming_soon.take();
            }
            self.show_about = flags.show_about;
            self.show_preferences = flags.show_preferences;
            self.state.find_open = flags.find_open;
            self.show_replace = flags.show_replace;
            self.find_focus_once = flags.find_focus_once;
            if flags.follow_caret {
                self.follow_focused_caret();
            } else {
                self.follow_caret = false;
            }
            if flags.show_goto_line {
                self.open_goto_line_dialog();
            }
            if flags.show_summary {
                self.show_summary = true;
            }
            if flags.show_doc_list {
                self.show_doc_list = true;
            }
            if flags.show_project_panel {
                self.show_project_panel = true;
            }
            if flags.show_theme_picker {
                self.show_theme_picker = true;
            }
            if flags.show_doc_map {
                self.show_doc_map = true;
            }
            if flags.show_func_list {
                self.show_func_list = true;
            }
            if flags.show_char_panel {
                self.show_char_panel = true;
            }
            match flags.zoom_delta {
                Some(1) => {
                    self.font_size = (self.font_size + 1.0).min(48.0);
                    self.persist_font_size();
                }
                Some(-1) => {
                    self.font_size = (self.font_size - 1.0).max(8.0);
                    self.persist_font_size();
                }
                Some(0) => {
                    self.font_size = 14.0;
                    self.persist_font_size();
                }
                _ => {}
            }
            self.apply_dual_view_flags(&flags);
            if let Some(on) = flags.always_on_top {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(if on {
                    egui::WindowLevel::AlwaysOnTop
                } else {
                    egui::WindowLevel::Normal
                }));
            }
            if flags.fullscreen_toggle {
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                    !ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)),
                ));
            }
            if let Some(t) = flags.pending_clipboard.take() {
                self.last_app_clipboard = Some(t.clone());
                ctx.copy_text(t);
            } else if let Some(t) = flags.last_copied.take() {
                self.last_app_clipboard = Some(t);
            }
            if flags.await_paste_bookmarks {
                self.await_paste_bookmarks = true;
            }
            if flags.request_quit {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if flags.highlight_dirty_scroll_reset {
                self.scroll_line = 0.0;
            }
        }
    }

    fn render_menu_node(
        &mut self,
        ui: &mut egui::Ui,
        node: &crate::menu_data::MenuNode,
        run_cmd: &mut Option<String>,
        top_level: bool,
    ) {
        use crate::menu_data::MenuNode;
        match node {
            MenuNode::Separator => {
                ui.separator();
            }
            MenuNode::Item { label, cmd } => {
                let text = if crate::commands::is_implemented(cmd) {
                    RichText::new(label).color(MENU_READY)
                } else {
                    RichText::new(label)
                };
                let response = ui.button(text);
                let response = match cmd.as_str() {
                    "IDM_OPEN_NPP_LOGS" => response.on_hover_text(
                        "Open logs/*.log relative to the process cwd (e.g. logs/panic.log)",
                    ),
                    "IDM_DEBUGINFO" => {
                        response.on_hover_text("Open a tab with version, OS, and log status")
                    }
                    _ => response,
                };
                if response.clicked() {
                    *run_cmd = Some(cmd.clone());
                    ui.close_menu();
                }
            }
            MenuNode::Popup { label, children } => {
                // Inject Recent Files into File menu (Notepad++ inserts this at runtime).
                let is_file = top_level && label == "File";
                let is_plugins = top_level && label == "Plugins";
                ui.menu_button(label, |ui| {
                    if is_file {
                        // Match N++: recent list near the end; we place after Open block via full tree,
                        // and also expose an explicit Recent submenu at the top of File for clarity.
                        ui.menu_button(RichText::new("Recent Files").color(MENU_READY), |ui| {
                            let recent_paths: Vec<_> = self.state.recent.paths().to_vec();
                            if recent_paths.is_empty() {
                                ui.label(RichText::new("(empty)").italics().weak());
                            } else {
                                let mut open_path = None;
                                for (i, path) in recent_paths.iter().enumerate() {
                                    let label = crate::recent::recent_label(path);
                                    let exists = path.exists();
                                    let text = if exists {
                                        RichText::new(format!("{}.  {label}", i + 1))
                                            .color(MENU_READY)
                                    } else {
                                        RichText::new(format!("{}.  {label}  (missing)", i + 1))
                                            .weak()
                                    };
                                    if ui
                                        .add_enabled(exists, egui::Button::new(text))
                                        .on_hover_text(path.display().to_string())
                                        .clicked()
                                    {
                                        open_path = Some(path.clone());
                                    }
                                }
                                ui.separator();
                                if ui
                                    .button(RichText::new("Clear Recent Files").color(MENU_READY))
                                    .clicked()
                                {
                                    self.state.clear_recent();
                                    ui.close_menu();
                                }
                                if let Some(path) = open_path {
                                    self.state.open_path(path);
                                    ui.close_menu();
                                }
                            }
                        });
                        ui.separator();
                    }
                    for child in children {
                        self.render_menu_node(ui, child, run_cmd, false);
                    }
                    if is_plugins {
                        ui.separator();
                        ui.label(RichText::new("npp-rs builtins").small().weak());
                        let host = plugins::PluginHost::new();
                        let mut run_id = None;
                        for p in host.list() {
                            if ui
                                .button(RichText::new(p.name()).color(MENU_READY))
                                .clicked()
                            {
                                run_id = Some(p.id().to_string());
                            }
                        }
                        if ui
                            .button(RichText::new("Format Document").color(MENU_READY))
                            .clicked()
                        {
                            self.state.format_document();
                            ui.close_menu();
                        }
                        if let Some(id) = run_id {
                            self.state.run_plugin(&id);
                            ui.close_menu();
                        }
                    }
                });
            }
        }
    }

    fn about_window(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }
        let mut open = true;
        egui::Window::new("About npp-rust")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .default_size([440.0, 420.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.heading(
                        RichText::new("npp-rust")
                            .size(28.0)
                            .color(Color32::from_rgb(120, 200, 255)),
                    );
                    ui.label(
                        RichText::new("a Notepad++ inspired editor, rebuilt for fun")
                            .italics()
                            .color(Color32::from_rgb(180, 180, 190)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!(
                            "v{} · {} · Rust · macOS / Linux / Windows",
                            env!("CARGO_PKG_VERSION"),
                            env!("NPP_GIT_HASH")
                        ))
                        .small(),
                    );
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.hyperlink_to("GitHub", "https://github.com/raro42/npp-rust");
                        ui.label("·");
                        ui.hyperlink_to("Issues", "https://github.com/raro42/npp-rust/issues");
                        ui.label("·");
                        ui.hyperlink_to(
                            "Discussions",
                            "https://github.com/raro42/npp-rust/discussions",
                        );
                        ui.label("·");
                        ui.hyperlink_to("Wiki", "https://github.com/raro42/npp-rust/wiki");
                        ui.label("·");
                        ui.hyperlink_to("Releases", "https://github.com/raro42/npp-rust/releases");
                        ui.label("·");
                        ui.hyperlink_to(
                            "Changelog",
                            "https://github.com/raro42/npp-rust/blob/main/docs/changelog.md",
                        );
                    });
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                ui.label(RichText::new("Why it exists").strong());
                ui.label(
                    "Built as a side adventure — something nice to grow in the background \
while other work runs. Not a line-by-line port. A fresh editor with a rope buffer, \
Tree-sitter highlight, and a calm UI.",
                );

                ui.add_space(10.0);
                ui.label(RichText::new("Made by").strong());
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Ralf Roeber").strong());
                    ui.label("·");
                    ui.label("El Masnou (Barcelona), Catalonia");
                });
                ui.label(
                    RichText::new("Germany roots · Spain home · open source habit")
                        .small()
                        .color(Color32::from_rgb(150, 150, 160)),
                );

                ui.add_space(10.0);
                ui.label(RichText::new("Shortcuts").strong());
                egui::Grid::new("about_shortcuts")
                    .num_columns(2)
                    .spacing([16.0, 4.0])
                    .show(ui, |ui| {
                        for (keys, action) in [
                            ("⌘/Ctrl N", "New file"),
                            ("⌘/Ctrl O", "Open"),
                            ("⌘/Ctrl S", "Save"),
                            ("⌘/Ctrl ⇧ S", "Save As"),
                            ("⌘/Ctrl F / H", "Find / Replace"),
                            ("F3 / ⇧ F3", "Find next / prev"),
                            ("⌘/Ctrl G", "Find next (global)"),
                            ("⌘/Ctrl L", "Go to line"),
                            ("F2 / ⇧ F2", "Next / prev bookmark"),
                            ("⌘/Ctrl = / -", "Zoom in / out"),
                            ("Alt Z", "Word wrap"),
                            ("⌘/Ctrl A", "Select all"),
                            ("⌘/Ctrl D", "Duplicate line"),
                            ("⌘/Ctrl ] / [", "Indent / Outdent"),
                            ("⌘/Ctrl ⇧ I", "Format document"),
                            ("⌘/Ctrl ⇧ T", "Toggle log tail"),
                            ("Alt ←/→", "Word jump"),
                            ("Double-click", "Select word"),
                            ("⌘/Ctrl Z / Y", "Undo / Redo"),
                            ("⌘/Ctrl W", "Close tab"),
                        ] {
                            ui.monospace(keys);
                            ui.label(action);
                            ui.end_row();
                        }
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Inspired by Notepad++ · MIT · separate project")
                            .small()
                            .color(Color32::from_rgb(140, 140, 150)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close").clicked() {
                            self.show_about = false;
                        }
                    });
                });
            });
        if !open {
            self.show_about = false;
        }
    }

    fn preferences_window(&mut self, ctx: &egui::Context) {
        if !self.show_preferences {
            return;
        }
        use crate::recent::{DefaultEol, LogTailOnOpen};
        let mut open = true;
        let mut changed = false;
        egui::Window::new("Preferences")
            .open(&mut open)
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(440.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(480.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("When opening *.log files").strong());
                        ui.add_space(4.0);
                        let cur = &mut self.state.settings.log_tail_on_open;
                        if ui
                            .radio_value(cur, LogTailOnOpen::Ask, "Ask each time")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(
                                cur,
                                LogTailOnOpen::Always,
                                "Always enable Monitoring (tail)",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(
                                cur,
                                LogTailOnOpen::Never,
                                "Never ask — open as a normal file",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Editor").strong());
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Font size");
                            if ui
                                .add(egui::Slider::new(&mut self.font_size, 8.0..=48.0))
                                .changed()
                            {
                                changed = true;
                                self.state.settings.font_size = self.font_size;
                            }
                        });
                        if ui
                            .checkbox(
                                &mut self.state.settings.show_line_numbers,
                                "Show line numbers",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Gutter extra");
                            let mut g = self.state.settings.gutter_extra as i32;
                            if ui.add(egui::Slider::new(&mut g, 0..=40)).changed() {
                                self.state.settings.gutter_extra = g as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(&mut self.state.settings.caret_blink, "Caret blink")
                            .changed()
                        {
                            changed = true;
                        }
                        ui.horizontal(|ui| {
                            ui.label("Tab width");
                            let mut tw = self.state.settings.tab_width as i32;
                            if ui.add(egui::Slider::new(&mut tw, 2..=8)).changed() {
                                self.state.settings.tab_width = tw as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(&mut self.state.settings.word_wrap, "Word wrap")
                            .changed()
                        {
                            self.state.word_wrap = self.state.settings.word_wrap;
                            changed = true;
                        }
                        ui.label("Default EOL (Enter key)");
                        let eol = &mut self.state.settings.default_eol;
                        if ui
                            .radio_value(eol, DefaultEol::Lf, DefaultEol::Lf.label())
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .radio_value(eol, DefaultEol::Crlf, DefaultEol::Crlf.label())
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Files").strong());
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Recent file count");
                            let mut rm = self.state.settings.recent_max as i32;
                            if ui.add(egui::Slider::new(&mut rm, 5..=40)).changed() {
                                self.state.settings.recent_max = rm as u8;
                                changed = true;
                            }
                        });
                        if ui
                            .checkbox(
                                &mut self.state.settings.restore_session,
                                "Restore last session on launch",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.state.settings.restore_session {
                                self.state.persist_session_if_enabled();
                            }
                        }
                        ui.label(
                            RichText::new(format!("Session file: {}", crate::session::SESSION_REL))
                                .small()
                                .weak(),
                        );
                        if ui
                            .checkbox(
                                &mut self.state.settings.backup_on_save,
                                "Backup on save (copy into config backup folder)",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            RichText::new(format!(
                                "Backup folder: {} (mirrors path layout)",
                                crate::backup::BACKUP_REL
                            ))
                            .small()
                            .weak(),
                        );
                        ui.horizontal(|ui| {
                            ui.label("Autosave interval (sec)");
                            let mut secs = self.state.settings.autosave_interval_secs as i32;
                            if ui.add(egui::Slider::new(&mut secs, 0..=900)).changed() {
                                if secs > 0 && secs < 15 {
                                    secs = 15;
                                }
                                self.state.settings.autosave_interval_secs = secs as u32;
                                changed = true;
                            }
                        });
                        ui.label(
                            RichText::new(
                                "0 = off; otherwise 15–900. Dirty tabs with a path only (skip untitled).",
                            )
                            .small()
                            .weak(),
                        );
                        ui.add_space(10.0);
                        ui.label(RichText::new("Find").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(&mut self.state.settings.find_match_case, "Match case")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(&mut self.state.settings.find_whole_word, "Whole word")
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Compare").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(
                                &mut self.state.settings.compare_ignore_ws,
                                "Ignore whitespace differences",
                            )
                            .changed()
                        {
                            changed = true;
                            if self.compare_on {
                                self.state.compare_stale = true;
                            }
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Status bar").strong());
                        ui.add_space(4.0);
                        if ui
                            .checkbox(&mut self.state.settings.status_show_lang, "Show language")
                            .changed()
                        {
                            changed = true;
                        }
                        if ui
                            .checkbox(
                                &mut self.state.settings.status_show_chars,
                                "Show character count",
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.add_space(10.0);
                        ui.label(RichText::new("Theme").strong());
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(
                                "JSON tokens + chrome; N++ XML subset (GlobalStyles + one lexer).",
                            )
                            .small()
                            .weak(),
                        );
                        let mut theme_id = self.state.settings.theme_id.clone();
                        for (id, label) in crate::theme::list_theme_choices() {
                            if ui.radio_value(&mut theme_id, id.clone(), label).changed() {
                                self.state.settings.theme_id = theme_id.clone();
                                changed = true;
                            }
                        }
                        if ui.button("Open Theme picker…").clicked() {
                            self.show_theme_picker = true;
                        }
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("Saved to {}", crate::recent::SETTINGS_REL))
                                .small()
                                .weak(),
                        );
                        ui.add_space(8.0);
                        if ui.button("Close").clicked() {
                            self.show_preferences = false;
                        }
                    });
            });
        if changed {
            self.state.settings.font_size = self.font_size;
            self.state.settings.save();
            self.state.status = format!("Preferences saved ({})", crate::recent::SETTINGS_REL);
        }
        if !open {
            self.show_preferences = false;
        }
    }

    fn log_tail_prompt_window(&mut self, ctx: &egui::Context) {
        if !self.state.pending_log_tail_prompt {
            return;
        }
        let name = self
            .state
            .tabs
            .active()
            .path
            .as_ref()
            .map(|p| crate::recent::short_path_label(p))
            .unwrap_or_else(|| "*.log".into());
        let mut enable = false;
        let mut skip = false;
        let mut open = true;
        egui::Window::new("Follow this log?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(360.0)
            .show(ctx, |ui| {
                ui.label(format!("You opened {name}."));
                ui.label("Enable Monitoring (tail -f) now?");
                ui.add_space(6.0);
                ui.checkbox(
                    &mut self.log_tail_remember,
                    "Remember for future *.log files",
                );
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Enable tail").clicked() {
                        enable = true;
                    }
                    if ui.button("Not now").clicked() {
                        skip = true;
                    }
                });
                if ui
                    .small_button("Reset remembered preference to ask")
                    .on_hover_text(crate::recent::SETTINGS_REL)
                    .clicked()
                {
                    self.state.reset_log_tail_preference();
                }
            });
        if enable {
            self.state
                .resolve_log_tail_prompt(true, self.log_tail_remember);
            self.follow_caret = true;
        } else if skip {
            self.state
                .resolve_log_tail_prompt(false, self.log_tail_remember);
        } else if !open {
            // Window closed: dismiss once; do not persist.
            self.state.pending_log_tail_prompt = false;
        }
    }

    fn encoding_notice_window(&mut self, ctx: &egui::Context) {
        let Some(msg) = self.state.pending_encoding_notice.clone() else {
            return;
        };
        let mut dismiss = false;
        let mut open = true;
        egui::Window::new("Encoding notice")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.label(&msg);
                ui.add_space(8.0);
                if ui.button("OK").clicked() {
                    dismiss = true;
                }
            });
        if dismiss || !open {
            self.state.pending_encoding_notice = None;
        }
    }

    fn unsaved_close_window(&mut self, ctx: &egui::Context) {
        if self.state.pending_close.is_none() {
            return;
        }
        let title = self.state.close_tab_title();
        let mut save = false;
        let mut discard = false;
        let mut cancel = false;
        let mut open = true;
        egui::Window::new("Save changes?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.label(format!("Save changes to \"{title}\"?"));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save = true;
                    }
                    if ui.button("Don't Save").clicked() {
                        discard = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if !open {
            cancel = true;
        }
        if save {
            if !self.state.confirm_close_save() {
                return;
            }
        } else if discard {
            self.state.confirm_close_discard();
        } else if cancel {
            self.state.confirm_close_cancel();
            return;
        } else {
            return;
        }
        if self.state.take_want_quit() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        self.scroll_line = 0.0;
    }

    fn unsaved_reload_window(&mut self, ctx: &egui::Context) {
        if self.state.pending_reload.is_none() {
            return;
        }
        let title = self.state.tabs.active().title.clone();
        let mut save = false;
        let mut discard = false;
        let mut cancel = false;
        let mut open = true;
        egui::Window::new("Reload from disk?")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.label(format!(
                    "\"{title}\" has unsaved changes. Reload and lose them?"
                ));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        save = true;
                    }
                    if ui.button("Don't Save").clicked() {
                        discard = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if !open {
            cancel = true;
        }
        if save {
            let _ = self.state.confirm_reload_save();
        } else if discard {
            self.state.confirm_reload_discard();
            self.scroll_line = 0.0;
        } else if cancel {
            self.state.confirm_reload_cancel();
        }
    }

    fn coming_soon_window(&mut self, ctx: &egui::Context) {
        let Some(info) = self.coming_soon.clone() else {
            return;
        };
        let blurb = crate::commands::coming_soon_blurb(&info.cmd);
        let mut open = true;
        egui::Window::new("Coming soon")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    ui.heading(
                        RichText::new("Not ready yet")
                            .size(22.0)
                            .color(Color32::from_rgb(255, 196, 120)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(format!("“{}”", info.feature))
                            .strong()
                            .size(16.0),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(blurb)
                            .italics()
                            .color(Color32::from_rgb(200, 200, 210)),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(
                            "We’re building npp-rs in the background — one honest menu at a time.",
                        )
                        .small()
                        .color(Color32::from_rgb(150, 150, 160)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("Come back tomorrow. Bring a smile; we’ll bring a feature.")
                            .small()
                            .strong()
                            .color(Color32::from_rgb(140, 200, 160)),
                    );
                    ui.add_space(12.0);
                    if ui.button("Alright, see you tomorrow").clicked() {
                        self.coming_soon = None;
                    }
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(info.cmd)
                            .small()
                            .monospace()
                            .color(Color32::from_rgb(110, 110, 120)),
                    );
                });
            });
        if !open {
            self.coming_soon = None;
        }
    }

    fn goto_line_window(&mut self, ctx: &egui::Context) {
        if !self.show_goto_line {
            return;
        }
        let mut open = true;
        let mut go = false;
        let mut cancel = false;
        egui::Window::new("Go to line")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Line number (1-based):");
                let resp = ui.text_edit_singleline(&mut self.goto_line_input);
                if resp.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                    go = true;
                }
                ui.horizontal(|ui| {
                    if ui.button("Go").clicked() {
                        go = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if go {
            if let Ok(n) = self.goto_line_input.trim().parse::<usize>() {
                if n >= 1 {
                    let line = (n - 1).min(
                        self.state
                            .tabs
                            .active()
                            .buffer
                            .line_count()
                            .saturating_sub(1),
                    );
                    let at = self.state.tabs.active().buffer.line_to_char(line);
                    self.state.tabs.active_mut().buffer.set_caret(at);
                    self.follow_caret = true;
                    self.state.status = format!("Go to line {n}");
                }
            }
            self.show_goto_line = false;
        }
        if cancel || !open {
            self.show_goto_line = false;
        }
    }

    fn summary_window(&mut self, ctx: &egui::Context) {
        if !self.show_summary {
            return;
        }
        let doc = self.state.tabs.active();
        let text = doc.buffer.to_string();
        let lines = doc.buffer.line_count();
        let chars = doc.buffer.len_chars();
        let bytes = text.len();
        let words = text.split_whitespace().count();
        let path = doc
            .path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(untitled)".into());
        let title = doc.title.clone();
        let language = doc.language.clone();
        let dirty = doc.dirty;
        let read_only = doc.read_only;
        let marks = doc.bookmarks.len();
        let (chg_u, chg_s) = doc.change_history_counts();
        let mut open = self.show_summary;
        let mut close = false;
        egui::Window::new("Summary")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(format!("Title: {title}"));
                ui.label(format!("Path: {path}"));
                ui.label(format!("Language: {language}"));
                ui.label(format!("Lines: {lines}"));
                ui.label(format!("Words: {words}"));
                ui.label(format!("Characters: {chars}"));
                ui.label(format!("Bytes: {bytes}"));
                ui.label(format!("Dirty: {dirty}"));
                ui.label(format!("Read-only: {read_only}"));
                ui.label(format!("Bookmarks: {marks}"));
                ui.label(format!("Change history: {chg_u} unsaved, {chg_s} saved"));
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        self.show_summary = open && !close;
    }

    fn apply_theme_visuals(&mut self, ctx: &egui::Context) {
        let theme = crate::theme::resolve_theme(&self.state.settings.theme_id);
        ctx.set_visuals(theme.visuals());
    }

    fn current_theme(&self) -> crate::theme::AppliedTheme {
        crate::theme::resolve_theme(&self.state.settings.theme_id)
    }

    fn project_panel_window(&mut self, ctx: &egui::Context) {
        if !self.show_project_panel {
            return;
        }
        let mut open = self.show_project_panel;
        let mut open_path: Option<std::path::PathBuf> = None;
        let mut enter_dir: Option<std::path::PathBuf> = None;
        let mut pick_folder = false;
        let mut close = false;
        let mut persist_filter = false;
        let mut refresh = false;
        let root = self.state.workspace_root.clone();
        egui::Window::new("Project")
            .open(&mut open)
            .default_width(320.0)
            .default_height(420.0)
            .show(ctx, |ui| {
                ui.label(RichText::new(root.display().to_string()).small().weak());
                ui.horizontal(|ui| {
                    if ui.button("Pick folder…").clicked() {
                        pick_folder = true;
                    }
                    if ui
                        .small_button("Up")
                        .on_hover_text("Parent folder")
                        .clicked()
                    {
                        if let Some(parent) = root.parent() {
                            enter_dir = Some(parent.to_path_buf());
                        }
                    }
                    if ui.small_button("Refresh").clicked() {
                        refresh = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Filter");
                    if ui
                        .add(
                            egui::TextEdit::singleline(&mut self.state.settings.project_filter)
                                .desired_width(180.0)
                                .hint_text("name contains…"),
                        )
                        .changed()
                    {
                        persist_filter = true;
                    }
                });
                ui.separator();
                let filter = self.state.settings.project_filter.to_ascii_lowercase();
                egui::ScrollArea::vertical()
                    .max_height(340.0)
                    .show(ui, |ui| {
                        let mut entries: Vec<std::path::PathBuf> = Vec::new();
                        if let Ok(rd) = std::fs::read_dir(&root) {
                            for ent in rd.flatten() {
                                let p = ent.path();
                                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                                if name.starts_with('.') {
                                    continue;
                                }
                                if !filter.is_empty()
                                    && !name.to_ascii_lowercase().contains(&filter)
                                {
                                    continue;
                                }
                                entries.push(p);
                            }
                        }
                        entries.sort_by(|a, b| {
                            b.is_dir()
                                .cmp(&a.is_dir())
                                .then(a.file_name().cmp(&b.file_name()))
                        });
                        if entries.is_empty() {
                            ui.label(if filter.is_empty() {
                                "(empty folder)"
                            } else {
                                "(no matches)"
                            });
                        }
                        for p in entries {
                            let is_dir = p.is_dir();
                            let name = p
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| p.display().to_string());
                            let label = if is_dir { format!("{name}/") } else { name };
                            if ui.selectable_label(false, label).clicked() {
                                if is_dir {
                                    enter_dir = Some(p);
                                } else {
                                    open_path = Some(p);
                                }
                            }
                        }
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        let _ = refresh;
        if pick_folder {
            self.state.pick_workspace_folder();
        }
        if let Some(dir) = enter_dir {
            self.state.workspace_root = dir;
            self.state.persist_workspace_root();
        }
        if persist_filter {
            self.state.settings.save();
        }
        if let Some(p) = open_path {
            self.state.open_path(p);
            self.scroll_line = 0.0;
        }
        self.show_project_panel = open && !close;
    }

    fn lossy_ansi_confirm_window(&mut self, ctx: &egui::Context) {
        let Some(path) = self.state.pending_lossy_ansi.clone() else {
            return;
        };
        let unmapped =
            ::fs::count_windows_1252_unmapped(&self.state.tabs.active().buffer.to_string());
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        let mut save = false;
        let mut cancel = false;
        egui::Window::new("ANSI save warning")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(format!(
                    "Saving “{name}” as Windows-1252 will turn {unmapped} character(s) into '?'."
                ));
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save anyway").clicked() {
                        save = true;
                    }
                    if ui.button("Cancel").clicked() {
                        cancel = true;
                    }
                });
            });
        if save {
            self.state.confirm_lossy_ansi_save();
        }
        if cancel {
            self.state.cancel_lossy_ansi_save();
        }
    }

    fn theme_picker_window(&mut self, ctx: &egui::Context) {
        if !self.show_theme_picker {
            return;
        }
        let mut open = self.show_theme_picker;
        let mut applied: Option<String> = None;
        let mut close = false;
        egui::Window::new("Themes")
            .open(&mut open)
            .default_width(420.0)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(
                        "Apply: egui visuals, editor chrome, selection/caret, and syntax tokens from themes/*.json or a Notepad++ XML subset (GlobalStyles + preferred lexer).",
                    )
                    .small()
                    .weak(),
                );
                ui.separator();
                let cur = self.state.settings.theme_id.clone();
                for (id, label) in crate::theme::list_theme_choices() {
                    if ui.selectable_label(cur == id, label).clicked() {
                        applied = Some(id);
                    }
                }
                ui.separator();
                if ui.button("Open themes/ folder").clicked() {
                    let dir = crate::theme::ensure_themes_dir();
                    crate::commands::common::open_path_in_os(&mut self.state, &dir);
                }
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        if let Some(id) = applied {
            self.state.settings.theme_id = id.clone();
            self.state.settings.save();
            let t = crate::theme::resolve_theme(&id);
            self.state.status = format!("Theme applied: {} ({})", t.label, id);
            ctx.set_visuals(t.visuals());
        }
        self.show_theme_picker = open && !close;
    }

    fn doc_list_window(&mut self, ctx: &egui::Context) {
        if !self.show_doc_list {
            return;
        }
        let mut open = self.show_doc_list;
        let mut switch_to = None;
        let mut close = false;
        egui::Window::new("Document List")
            .open(&mut open)
            .default_width(420.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(320.0)
                    .show(ui, |ui| {
                        for i in 0..self.state.tabs.len() {
                            let Some(doc) = self.state.tabs.get(i) else {
                                continue;
                            };
                            let mut label = doc.title.clone();
                            if doc.dirty {
                                label.push('*');
                            }
                            if ui
                                .selectable_label(i == self.state.tabs.active_index(), label)
                                .clicked()
                            {
                                switch_to = Some(i);
                            }
                        }
                    });
                if ui.button("Close").clicked() {
                    close = true;
                }
            });
        if let Some(i) = switch_to {
            self.state.switch_tab(i);
            close = true;
        }
        self.show_doc_list = open && !close;
    }

    fn doc_map_window(&mut self, ctx: &egui::Context) {
        if !self.show_doc_map {
            return;
        }
        let line_count = self.state.tabs.active().buffer.line_count().max(1);
        let max_scroll = (line_count.saturating_sub(1) as f32).max(0.0);
        let mut open = self.show_doc_map;
        let mut jump_line: Option<f32> = None;
        egui::Window::new("Document Map")
            .open(&mut open)
            .default_width(72.0)
            .default_height(360.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!("{line_count} lines"));
                let (resp, painter) = ui.allocate_painter(
                    Vec2::new(
                        ui.available_width().max(40.0),
                        ui.available_height().max(120.0),
                    ),
                    Sense::click_and_drag(),
                );
                let rect = resp.rect;
                painter.rect_filled(rect, 0.0, Color32::from_rgb(28, 28, 32));
                let sample_n = 128.min(line_count);
                for i in 0..sample_n {
                    let line_idx = i * line_count / sample_n;
                    let raw = self.state.tabs.active().buffer.line(line_idx);
                    let dens = (raw.trim_end_matches(['\n', '\r']).chars().count() as f32 / 80.0)
                        .clamp(0.05, 1.0);
                    let y0 = rect.top() + (i as f32 / sample_n as f32) * rect.height();
                    let y1 = rect.top() + ((i + 1) as f32 / sample_n as f32) * rect.height();
                    let g = (40.0 + dens * 140.0) as u8;
                    painter.rect_filled(
                        Rect::from_min_max(Pos2::new(rect.left(), y0), Pos2::new(rect.right(), y1)),
                        0.0,
                        Color32::from_rgb(g, g, g.saturating_add(8)),
                    );
                }
                if max_scroll > 0.0 {
                    let frac = (self.scroll_line / max_scroll).clamp(0.0, 1.0);
                    let mark_h = (rect.height() * 0.08).max(6.0);
                    let mark_y = rect.top() + frac * (rect.height() - mark_h);
                    painter.rect_stroke(
                        Rect::from_min_size(
                            Pos2::new(rect.left() + 1.0, mark_y),
                            Vec2::new(rect.width() - 2.0, mark_h),
                        ),
                        0.0,
                        egui::Stroke::new(1.5_f32, Color32::from_rgb(80, 180, 140)),
                        egui::StrokeKind::Outside,
                    );
                }
                if let Some(pos) = resp.interact_pointer_pos() {
                    if resp.clicked() || resp.dragged() {
                        let t = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
                        jump_line = Some(t * max_scroll);
                    }
                }
            });
        if let Some(line) = jump_line {
            self.scroll_line = line;
            self.follow_caret = false;
            self.state.status = format!("Document Map → line {}", line as usize + 1);
        }
        self.show_doc_map = open;
    }

    fn func_list_window(&mut self, ctx: &egui::Context) {
        if !self.show_func_list {
            return;
        }
        let entries = collect_func_like_lines(&self.state.tabs.active().buffer);
        let mut open = self.show_func_list;
        let mut jump: Option<usize> = None;
        egui::Window::new("Function List")
            .open(&mut open)
            .default_width(360.0)
            .default_height(320.0)
            .show(ctx, |ui| {
                if entries.is_empty() {
                    ui.label("No fn/class-like lines found.");
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(280.0)
                        .show(ui, |ui| {
                            for (line_idx, preview) in &entries {
                                let label = format!("{}: {preview}", line_idx + 1);
                                if ui.selectable_label(false, label).clicked() {
                                    jump = Some(*line_idx);
                                }
                            }
                        });
                }
            });
        if let Some(line) = jump {
            let at = self.state.tabs.active().buffer.line_to_char(line);
            self.state.tabs.active_mut().buffer.set_caret(at);
            self.follow_caret = true;
            self.state.status = format!("Function List → line {}", line + 1);
        }
        self.show_func_list = open;
    }

    fn char_panel_window(&mut self, ctx: &egui::Context) {
        if !self.show_char_panel {
            return;
        }
        let mut open = self.show_char_panel;
        let mut insert: Option<char> = None;
        const CHARS: &[char] = &[
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{',
            '}', '\\', '|', ';', ':', '\'', '"', ',', '.', '<', '>', '/', '?', '~', '`', '©', '®',
            '™', '€', '£', '¥', '§', '¶', '°', '±', '×', '÷', '½', '¼', '¾', '…', '–', '—', '‘',
            '’', '“', '”', '«', '»', '•', '†', '‡', 'α', 'β', 'γ', 'δ', 'π', 'μ', 'Ω', '←', '→',
            '↑', '↓', '✓', '✗', '★', '☆', '♠', '♣', '♥', '♦', '☺', '☻', '♪', '♫', '∞', '≈', '≠',
            '≤', '≥', '√', '∑', '∏', '∫', '∂', '∆', '∈', '∉', '∩', '∪', '⊂',
        ];
        egui::Window::new("Character Panel")
            .open(&mut open)
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.label("Click a character to insert at the caret.");
                egui::ScrollArea::vertical()
                    .max_height(240.0)
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for &c in CHARS {
                                if ui
                                    .add_sized([28.0, 28.0], egui::Button::new(c.to_string()))
                                    .clicked()
                                {
                                    insert = Some(c);
                                }
                            }
                        });
                    });
            });
        if let Some(c) = insert {
            if self.state.tabs.active().read_only {
                self.state.status = "Document is read-only".into();
            } else {
                let s = c.to_string();
                self.state.prepare_edit();
                self.state.tabs.active_mut().buffer.insert(&s);
                self.state.mark_text_changed();
                self.follow_caret = true;
                self.state.status = format!("Inserted U+{:04X}", c as u32);
            }
        }
        self.show_char_panel = open;
    }

    fn tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let count = self.state.tabs.len();
                let mut switch_to = None;
                let mut close_idx = None;
                let mut toggle_pin = None;
                let mut pending_move: Option<(usize, usize)> = None;
                let mut start_compare_now = false;
                for i in 0..count {
                    let Some(doc) = self.state.tabs.get(i) else {
                        continue;
                    };
                    let pinned = doc.pinned;
                    let mut label = String::new();
                    if pinned {
                        label.push_str("[P] ");
                    }
                    label.push_str(&doc.title);
                    if doc.dirty {
                        label.push('*');
                    }
                    if doc.tail_follow {
                        label.push_str(" [tail]");
                    }
                    if doc.loading {
                        label.push_str(" …");
                    }
                    let selected = i == self.state.tabs.active_index();
                    let is_partner = self.compare_partner_tab == Some(i) && !selected;
                    if is_partner {
                        label.push_str(" ⇄");
                    }
                    let colour = match doc.tab_colour {
                        Some(1) => Some(Color32::from_rgb(180, 70, 70)),
                        Some(2) => Some(Color32::from_rgb(70, 140, 80)),
                        Some(3) => Some(Color32::from_rgb(70, 110, 180)),
                        Some(4) => Some(Color32::from_rgb(160, 120, 40)),
                        Some(5) => Some(Color32::from_rgb(140, 70, 160)),
                        _ => None,
                    };
                    let text = if let Some(c) = colour {
                        RichText::new(&label).color(c)
                    } else if is_partner {
                        RichText::new(&label).color(Color32::from_rgb(40, 120, 140))
                    } else {
                        RichText::new(&label)
                    };
                    let resp = ui.add(
                        egui::Button::new(text)
                            .selected(selected)
                            .sense(Sense::click_and_drag()),
                    );
                    if resp.drag_started() {
                        self.tab_drag_from = Some(i);
                        switch_to = Some(i);
                    }
                    if resp.dragged() {
                        ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
                    }
                    if self.tab_drag_from.is_some() && resp.hovered() {
                        if let Some(from) = self.tab_drag_from {
                            if from != i {
                                pending_move = Some((from, i));
                            }
                        }
                    }
                    if resp.clicked() {
                        let mod_pick = ui.input(|inp| inp.modifiers.command || inp.modifiers.ctrl);
                        if mod_pick && i != self.state.tabs.active_index() {
                            if self.compare_partner_tab == Some(i) {
                                self.compare_partner_tab = None;
                                self.state.status = "Compare partner cleared".into();
                            } else {
                                self.compare_partner_tab = Some(i);
                                self.state.status = format!(
                                    "Compare partner: “{}” — View → Compare with Other View",
                                    doc.title
                                );
                            }
                        } else {
                            switch_to = Some(i);
                        }
                    }
                    if resp.middle_clicked() {
                        close_idx = Some(i);
                    }
                    resp.context_menu(|ui| {
                        let pin_label = if pinned { "Unpin tab" } else { "Pin tab" };
                        if ui.button(pin_label).clicked() {
                            toggle_pin = Some(i);
                            ui.close_menu();
                        }
                        if i != self.state.tabs.active_index() {
                            let mark_label = if self.compare_partner_tab == Some(i) {
                                "Clear compare partner"
                            } else {
                                "Mark for compare"
                            };
                            if ui.button(mark_label).clicked() {
                                if self.compare_partner_tab == Some(i) {
                                    self.compare_partner_tab = None;
                                    self.state.status = "Compare partner cleared".into();
                                } else {
                                    self.compare_partner_tab = Some(i);
                                    self.state.status = format!(
                                        "Compare partner: “{}” — View → Compare with Other View",
                                        doc.title
                                    );
                                }
                                ui.close_menu();
                            }
                            if ui.button("Compare with this tab").clicked() {
                                self.compare_partner_tab = Some(i);
                                start_compare_now = true;
                                ui.close_menu();
                            }
                        }
                    });
                    ui.push_id(("pin_tab", i), |ui| {
                        let pin_btn = if pinned { "P" } else { "·" };
                        if ui
                            .small_button(pin_btn)
                            .on_hover_text(if pinned { "Unpin tab" } else { "Pin tab" })
                            .clicked()
                        {
                            toggle_pin = Some(i);
                        }
                    });
                    ui.push_id(("close_tab", i), |ui| {
                        if ui.small_button("×").on_hover_text("Close tab").clicked() {
                            close_idx = Some(i);
                        }
                    });
                }
                if let Some((from, to)) = pending_move {
                    if self.state.tabs.move_tab(from, to) {
                        self.remap_tab_indices(from, to);
                        self.tab_drag_from = Some(to);
                        self.state.status = "Tab moved".into();
                    }
                }
                if !ui.input(|i| i.pointer.any_down()) {
                    self.tab_drag_from = None;
                }
                if ui.button("+").clicked() {
                    self.state.new_file();
                }
                if self.dual_view {
                    ui.separator();
                    let other_title = self
                        .state
                        .tabs
                        .get(self.other_view_tab)
                        .map(|d| d.title.clone())
                        .unwrap_or_else(|| "?".into());
                    ui.label(
                        RichText::new(format!("| other: {other_title}"))
                            .small()
                            .weak(),
                    );
                    if ui
                        .small_button("×dual")
                        .on_hover_text("Close dual view")
                        .clicked()
                    {
                        self.dual_view = false;
                        self.focused_pane = EditorPane::Primary;
                        self.state.status = "Dual view closed".into();
                    }
                }
                if let Some(i) = toggle_pin {
                    if let Some(doc) = self.state.tabs.get_mut(i) {
                        doc.pinned = !doc.pinned;
                        let name = doc.title.clone();
                        let on = doc.pinned;
                        self.state.status = if on {
                            format!("Pinned “{name}”")
                        } else {
                            format!("Unpinned “{name}”")
                        };
                    }
                }
                if let Some(i) = switch_to {
                    self.state.tabs.set_active(i);
                    self.state.highlight_dirty = true;
                    self.scroll_line = 0.0;
                }
                if let Some(i) = close_idx {
                    if self.compare_partner_tab == Some(i) {
                        self.compare_partner_tab = None;
                    } else if let Some(p) = self.compare_partner_tab {
                        if p > i {
                            self.compare_partner_tab = Some(p - 1);
                        }
                    }
                    self.state.request_close_tab(i);
                    self.scroll_line = 0.0;
                }
                if start_compare_now {
                    self.start_compare();
                }
            });
        });
    }

    fn remap_tab_indices(&mut self, from: usize, to: usize) {
        use doc::TabSet;
        self.other_view_tab = TabSet::remap_index(self.other_view_tab, from, to);
        self.compare_left_tab = TabSet::remap_index(self.compare_left_tab, from, to);
        self.compare_right_tab = TabSet::remap_index(self.compare_right_tab, from, to);
        if let Some(p) = self.compare_partner_tab {
            self.compare_partner_tab = Some(TabSet::remap_index(p, from, to));
        }
    }

    fn seed_find_from_selection(&mut self) {
        if let Some((s, e)) = self.state.tabs.active().buffer.selection() {
            let sel = self.state.tabs.active().buffer.slice(s, e);
            if !sel.is_empty() && !sel.contains('\n') && sel.chars().count() <= 200 {
                self.state.find_query = sel;
            }
        }
    }

    fn find_replace_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("find").show(ctx, |ui| {
            let mut persist = false;
            let mut run_find_files = false;
            ui.horizontal(|ui| {
                ui.label("Find:");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.state.find_query)
                        .desired_width(180.0)
                        .hint_text("search text"),
                );
                if resp.changed() {
                    persist = true;
                }
                let enter = ui.input(|i| i.key_pressed(Key::Enter));
                if resp.has_focus() && enter {
                    self.state.find_next();
                    self.follow_caret = true;
                }
                if ui.button("Next").clicked() {
                    self.state.find_next();
                    self.follow_caret = true;
                }
                if ui.button("Prev").clicked() {
                    self.state.find_prev();
                    self.follow_caret = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_match_case, "Case")
                    .changed()
                {
                    persist = true;
                }
                if ui
                    .checkbox(&mut self.state.settings.find_whole_word, "Word")
                    .changed()
                {
                    persist = true;
                }
                let n = self.state.find_match_count();
                if self.state.find_query.is_empty() {
                    ui.label(RichText::new("0 matches").weak());
                } else {
                    ui.label(format!("{n} match{}", if n == 1 { "" } else { "es" }));
                }
                if self.show_replace {
                    ui.separator();
                    ui.label("Replace:");
                    let rresp = ui.add(
                        egui::TextEdit::singleline(&mut self.replace_with).desired_width(120.0),
                    );
                    if rresp.changed() {
                        persist = true;
                    }
                    if ui.button("Replace").clicked() {
                        let r = self.replace_with.clone();
                        self.state.replace_next(&r);
                        self.follow_caret = true;
                    }
                    if ui.button("Replace All").clicked() {
                        let r = self.replace_with.clone();
                        self.state.replace_all(&r);
                    }
                } else if ui.button("Replace…").clicked() {
                    self.show_replace = true;
                }
                if ui.button("Close").clicked() {
                    persist = true;
                    self.state.find_open = false;
                    self.show_replace = false;
                }
                if self.find_focus_once {
                    resp.request_focus();
                    self.find_focus_once = false;
                }
            });
            ui.horizontal(|ui| {
                ui.label("In files:");
                let inc = ui.add(
                    egui::TextEdit::singleline(&mut self.state.settings.find_files_include)
                        .desired_width(140.0)
                        .hint_text("*.rs,*.md (empty=all)"),
                );
                if inc.changed() {
                    persist = true;
                }
                ui.label("Exclude:");
                let exc = ui.add(
                    egui::TextEdit::singleline(&mut self.state.settings.find_files_exclude)
                        .desired_width(160.0)
                        .hint_text("target,node_modules"),
                );
                if exc.changed() {
                    persist = true;
                }
                if ui
                    .button("Find in Files")
                    .on_hover_text("Search workspace root recursively")
                    .clicked()
                {
                    persist = true;
                    run_find_files = true;
                }
            });
            if persist {
                self.state.settings.find_query = self.state.find_query.clone();
                self.state.settings.replace_with = self.replace_with.clone();
                self.state.settings.save();
            }
            if run_find_files {
                let mut flags = crate::commands::UiFlags {
                    find_open: self.state.find_open,
                    show_replace: self.show_replace,
                    find_focus_once: self.find_focus_once,
                    follow_caret: self.follow_caret,
                    ..Default::default()
                };
                let _ = self.dispatch_menu_cmd("IDM_SEARCH_FINDINFILES", &mut flags);
                self.state.find_open = flags.find_open;
                self.show_replace = flags.show_replace;
                self.find_focus_once = flags.find_focus_once;
                if flags.follow_caret {
                    self.follow_focused_caret();
                }
            }
        });
    }

    fn status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            let (lang, line, col, chars, on, status, chg_u, chg_s) = {
                let doc = self.state.tabs.active();
                let caret = doc.buffer.caret();
                let line = doc.buffer.char_to_line(caret);
                let col = caret - doc.buffer.line_to_char(line) + 1;
                let (chg_u, chg_s) = doc.change_history_counts();
                (
                    doc.language.clone(),
                    line + 1,
                    col,
                    doc.buffer.len_chars(),
                    doc.tail_follow,
                    self.state.status.clone(),
                    chg_u,
                    chg_s,
                )
            };
            ui.horizontal(|ui| {
                ui.label(&status);
                if crate::commands::edit::text_is_rtl() {
                    ui.separator();
                    ui.label(
                        RichText::new("RTL")
                            .strong()
                            .color(Color32::from_rgb(42, 148, 118)),
                    );
                }

                ui.separator();
                // Clickable tail toggle (same as View → Monitoring).
                let tail_text = if on {
                    RichText::new("TAIL").strong().color(MENU_READY)
                } else {
                    RichText::new("tail").weak()
                };
                if ui
                    .add(egui::Button::new(tail_text).frame(false))
                    .on_hover_text(if on {
                        "Tail ON — click to stop following the file"
                    } else {
                        "Click to tail this file (Monitoring / ⌘⇧T)"
                    })
                    .clicked()
                    && self.state.toggle_tail_follow()
                {
                    self.follow_caret = true;
                }
                if chg_u + chg_s > 0 {
                    ui.separator();
                    let label = format!("CHG {chg_u}/{chg_s}");
                    let color = if chg_u > 0 {
                        Color32::from_rgb(210, 140, 40)
                    } else {
                        Color32::from_rgb(70, 160, 90)
                    };
                    ui.label(RichText::new(label).color(color)).on_hover_text(
                        "Change history: unsaved (amber) / saved (green) line marks",
                    );
                }
                if self.state.settings.status_show_lang {
                    ui.separator();
                    ui.label(format!("Lang: {lang}"));
                }
                ui.separator();
                ui.label(format!("Ln {line}, Col {col}"));
                if self.state.settings.status_show_chars {
                    ui.separator();
                    ui.label(format!("{chars} chars"));
                }

                // Bottom-right: package version + git commit → GitHub.
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let short = env!("NPP_GIT_HASH");
                    let full = env!("NPP_GIT_HASH_FULL");
                    let ver = env!("CARGO_PKG_VERSION");
                    let label = format!("v{ver} · {short}");
                    let url = format!("https://github.com/raro42/npp-rust/commit/{full}");
                    ui.hyperlink_to(RichText::new(label).small().weak(), url)
                        .on_hover_text("Open this build’s commit on GitHub (raro42/npp-rust)");
                });
            });
        });
    }

    fn editor_pane(&mut self, ctx: &egui::Context) {
        self.state
            .refresh_highlight_if_needed(self.scroll_line.floor() as usize);
        self.clamp_other_view_tab();
        self.sync_compare_panes();

        if self.dual_view {
            let title = self
                .state
                .tabs
                .get(self.other_view_tab)
                .map(|d| d.title.clone())
                .unwrap_or_else(|| "Other view".into());
            egui::SidePanel::right("dual_other_view")
                .resizable(true)
                .default_width(420.0)
                .min_width(180.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("Other view: {title}")).strong());
                        if ui
                            .small_button("Switch")
                            .on_hover_text("Swap with active tab")
                            .clicked()
                        {
                            self.switch_other_view_now();
                        }
                    });
                    ui.separator();
                    self.paint_secondary_pane(ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.state.tabs.active().loading {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("Loading file…").size(18.0));
                });
                return;
            }

            let font_id = FontId::monospace(self.font_size);
            let row_height = ui.fonts(|f| f.row_height(&font_id)) + 2.0;
            let buf_line_count = self.state.tabs.active().buffer.line_count().max(1);
            let visible_lines =
                visible_line_indices(buf_line_count, &self.state.tabs.active().hidden_lines);
            let display_count = visible_lines.len().max(1);
            let avail = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(avail, Sense::click_and_drag());
            // Do not steal focus every frame — that breaks Find (Ctrl/Cmd+F).
            if !self.state.find_open && !self.show_replace {
                if response.clicked() || response.drag_started() {
                    response.request_focus();
                    self.focused_pane = EditorPane::Primary;
                }
            } else if response.clicked() {
                // Clicking the editor closes find focus but keeps the bar open.
                response.request_focus();
                self.focused_pane = EditorPane::Primary;
            }

            if self.state.reset_view {
                self.scroll_line = 0.0;
                self.follow_caret = false;
                self.state.reset_view = false;
            }
            let visible_rows = {
                // One extra row of air above the status bar — last line must stay fully visible.
                let usable = (rect.height() - row_height).max(row_height);
                ((usable / row_height).floor() as usize).max(1)
            };
            let max_scroll = (display_count.saturating_sub(visible_rows) as f32).max(0.0);

            // Mouse-wheel scroll must not be overridden by caret-follow.
            // In dual view, only scroll this pane when the pointer is over it.
            // Cmd/Ctrl+wheel zooms (handled in handle_shortcuts); skip scroll then.
            let scroll = if !self.dual_view || response.hovered() {
                ui.input(|i| {
                    if i.modifiers.command || i.modifiers.ctrl {
                        0.0
                    } else {
                        i.raw_scroll_delta.y
                    }
                })
            } else {
                0.0
            };
            if scroll != 0.0 {
                self.follow_caret = false;
                self.scroll_line = (self.scroll_line - scroll / row_height).clamp(0.0, max_scroll);
                if self.dual_view && (self.sync_scroll_v || self.sync_scroll_h) {
                    self.scroll_line_other = self.scroll_line;
                }
            }

            let show_ln = self.state.settings.show_line_numbers;
            let gutter_w =
                if show_ln { 56.0 } else { 16.0 } + f32::from(self.state.settings.gutter_extra);
            // Gap between line numbers and text (was flush before).
            let gutter_gap = 12.0;
            let text_left = rect.left() + gutter_w + gutter_gap;

            let hit_index = |ui: &egui::Ui,
                             pos: Pos2,
                             buf: &buffer::TextBuffer,
                             scroll: f32,
                             visible: &[usize]|
             -> usize {
                let first = scroll.floor() as usize;
                let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
                let row = row.min(visible.len().saturating_sub(1));
                let line = visible.get(row).copied().unwrap_or(0);
                let line_start = buf.line_to_char(line);
                let line_text = buf.line(line);
                let line_body = line_text.trim_end_matches(['\n', '\r']);
                let col = col_from_x(ui, &font_id, line_body, pos.x - text_left);
                line_start + col
            };

            // Double-click → word; triple-click → line; click → caret;
            // Alt+drag → rect/column multi-carets; drag inside selection → move/copy;
            // else drag → select.
            if response.triple_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    self.state.tabs.active_mut().clear_multi_sels();
                    self.state.tabs.active_mut().buffer.select_line_at(idx);
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            } else if response.double_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    self.state.tabs.active_mut().clear_multi_sels();
                    self.state.tabs.active_mut().buffer.select_word_at(idx);
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            } else if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
                    let tab = self.state.tabs.active_index();
                    let doc = self.state.tabs.active();
                    let inside_sel = doc
                        .buffer
                        .selection()
                        .is_some_and(|(s, e)| idx >= s && idx < e);
                    let read_only = doc.read_only;
                    let sel_anchor = doc
                        .buffer
                        .selection()
                        .map(|(s, _)| s)
                        .unwrap_or_else(|| doc.buffer.caret());
                    if alt {
                        self.sel_text_drag = None;
                        self.rect_drag = true;
                        self.drag_anchor = Some(idx);
                        self.state.tabs.active_mut().set_rect_selection(idx, idx);
                        self.state.status = "Column select (Alt+drag)".into();
                    } else if !shift && inside_sel && !read_only {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = Some(SelTextDrag { tab, drop_at: idx });
                        self.drag_anchor = None;
                        self.rect_drag = false;
                    } else if shift {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        self.drag_anchor = Some(sel_anchor);
                        self.state
                            .tabs
                            .active_mut()
                            .buffer
                            .set_selection(sel_anchor, idx);
                    } else {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        self.drag_anchor = Some(idx);
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                    }
                    self.follow_caret = false;
                }
            } else if response.dragged() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    if let Some(drag) = self.sel_text_drag.as_mut() {
                        if drag.tab == self.state.tabs.active_index() {
                            drag.drop_at = idx;
                            let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                            ui.ctx().set_cursor_icon(if copy {
                                CursorIcon::Copy
                            } else {
                                CursorIcon::Grabbing
                            });
                        }
                    } else if let Some(anchor) = self.drag_anchor {
                        if self.rect_drag {
                            self.state.tabs.active_mut().set_rect_selection(anchor, idx);
                        } else {
                            self.state
                                .tabs
                                .active_mut()
                                .buffer
                                .set_selection(anchor, idx);
                        }
                        self.follow_caret = false;
                    }
                }
            } else if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(
                        ui,
                        pos,
                        &self.state.tabs.active().buffer,
                        self.scroll_line,
                        &visible_lines,
                    );
                    let shift = ui.input(|i| i.modifiers.shift);
                    if shift {
                        self.state.tabs.active_mut().clear_multi_sels();
                        let anchor = self
                            .state
                            .tabs
                            .active()
                            .buffer
                            .selection()
                            .map(|(s, _)| s)
                            .unwrap_or_else(|| self.state.tabs.active().buffer.caret());
                        self.state
                            .tabs
                            .active_mut()
                            .buffer
                            .set_selection(anchor, idx);
                    } else {
                        self.state.tabs.active_mut().clear_multi_sels();
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                    }
                    self.drag_anchor = None;
                    self.rect_drag = false;
                    self.sel_text_drag = None;
                    self.follow_caret = false;
                }
            }
            if ui.input(|i| i.pointer.any_released()) {
                if self.sel_text_drag.is_some() {
                    let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                    self.finish_sel_text_drag(copy);
                }
                self.drag_anchor = None;
                self.rect_drag = false;
            }

            // Text input (arrows / typing may request caret follow)
            if response.has_focus()
                && self.focused_pane == EditorPane::Primary
                && !self.state.find_open
                && !self.show_replace
            {
                let tab = self.state.tabs.active_index();
                if self.handle_editor_input(ui, tab) {
                    self.follow_caret = true;
                }
            }

            // Only keep caret in view after caret motion — not after wheel scroll.
            if self.follow_caret {
                let caret_line = self
                    .state
                    .tabs
                    .active()
                    .buffer
                    .char_to_line(self.state.tabs.active().buffer.caret());
                let caret_row = display_row_for(&visible_lines, caret_line) as f32;
                if caret_row < self.scroll_line {
                    self.scroll_line = caret_row;
                } else if caret_row >= self.scroll_line + visible_rows as f32 {
                    self.scroll_line = caret_row - visible_rows as f32 + 1.0;
                }
                self.scroll_line = self.scroll_line.clamp(0.0, max_scroll);
                self.follow_caret = false;
            }

            let first_row = self.scroll_line.floor() as usize;
            let visible = visible_rows + 2;
            let last_row = (first_row + visible).min(display_count);

            let painter = ui.painter_at(rect);
            let theme = self.current_theme();
            painter.rect_filled(rect, 0.0, theme.editor_bg);
            // Gutter band + hairline so numbers stay separate from text.
            let gutter_right = rect.left() + gutter_w;
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(rect.left(), rect.top()),
                    Pos2::new(gutter_right, rect.bottom()),
                ),
                0.0,
                theme.gutter_bg,
            );
            painter.vline(
                gutter_right,
                rect.y_range(),
                egui::Stroke::new(1.0_f32, theme.gutter_line),
            );

            let hl = &self.state.highlight_cache;
            let lang = self.state.tabs.active().language.clone();
            let bookmarks = self.state.tabs.active().bookmarks.clone();
            let changed_unsaved = self.state.tabs.active().changed_unsaved.clone();
            let changed_saved = self.state.tabs.active().changed_saved.clone();
            let style_marks = self.state.tabs.active().style_marks.clone();

            for row in first_row..last_row {
                let Some(&line_idx) = visible_lines.get(row) else {
                    break;
                };
                let y = rect.top() + (row as f32 - self.scroll_line) * row_height;
                let line_rect = Rect::from_min_size(
                    Pos2::new(rect.left(), y),
                    Vec2::new(rect.width(), row_height),
                );

                // Style-mark soft wash (first matching slot wins for paint).
                for (slot, marks) in style_marks.iter().enumerate() {
                    if marks.contains(&line_idx) {
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(text_left, y),
                                Pos2::new(rect.right(), y + row_height),
                            ),
                            0.0,
                            style_mark_bg((slot as u8) + 1),
                        );
                        break;
                    }
                }

                // 2-way compare wash (primary shows left side while comparing).
                if self.compare_on {
                    if let Some(kind) = self.compare_left_tags.get(line_idx) {
                        if let Some(bg) = crate::diff::line_kind_bg(*kind) {
                            painter.rect_filled(
                                Rect::from_min_max(
                                    Pos2::new(text_left, y),
                                    Pos2::new(rect.right(), y + row_height),
                                ),
                                0.0,
                                bg,
                            );
                        }
                    }
                }

                // Bookmark tick in the gutter.
                if bookmarks.contains(&line_idx) {
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(rect.left() + 4.0, y + 3.0),
                            Pos2::new(rect.left() + 10.0, y + row_height - 3.0),
                        ),
                        1.0,
                        Color32::from_rgb(80, 180, 220),
                    );
                }

                // Change-history bar in the gutter (amber = unsaved, green = saved).
                if changed_unsaved.contains(&line_idx) {
                    let (join_above, join_below) =
                        change_history_joins(line_idx, false, &changed_unsaved, &changed_saved);
                    paint_change_history_bar(
                        &painter,
                        rect.left(),
                        y,
                        row_height,
                        false,
                        join_above,
                        join_below,
                    );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(text_left, y),
                            Pos2::new(rect.right(), y + row_height),
                        ),
                        0.0,
                        change_history_wash(false),
                    );
                } else if changed_saved.contains(&line_idx) {
                    let (join_above, join_below) =
                        change_history_joins(line_idx, true, &changed_unsaved, &changed_saved);
                    paint_change_history_bar(
                        &painter,
                        rect.left(),
                        y,
                        row_height,
                        true,
                        join_above,
                        join_below,
                    );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(text_left, y),
                            Pos2::new(rect.right(), y + row_height),
                        ),
                        0.0,
                        change_history_wash(true),
                    );
                }

                // Line number — right-aligned inside the gutter.
                if show_ln {
                    painter.text(
                        Pos2::new(gutter_right - 6.0, y),
                        egui::Align2::RIGHT_TOP,
                        format!("{}", line_idx + 1),
                        font_id.clone(),
                        theme.line_number_fg,
                    );
                }

                let line_start = self.state.tabs.active().buffer.line_to_char(line_idx);
                let raw = self.state.tabs.active().buffer.line(line_idx);
                let line_text = raw.trim_end_matches(['\n', '\r']);

                // Selection highlight on line
                if let Some((sel_s, sel_e)) = self.state.tabs.active().buffer.selection() {
                    let line_end = line_start + line_text.chars().count();
                    if sel_s < line_end && sel_e > line_start {
                        let local_s = sel_s
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let local_e = sel_e
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let x0 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_s).collect::<String>(),
                            );
                        let x1 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_e).collect::<String>(),
                            );
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(x0, y),
                                Pos2::new(x1.max(x0 + 2.0), y + row_height),
                            ),
                            0.0,
                            theme.selection_bg,
                        );
                    }
                }
                // Extra multi-caret / rect ranges (skip primary to avoid double paint)
                let primary_sel = self.state.tabs.active().buffer.selection();
                let multi = self.state.tabs.active().multi_sels.clone();
                for &(sel_s, sel_e) in &multi {
                    if primary_sel == Some((sel_s, sel_e)) {
                        continue;
                    }
                    let line_end = line_start + line_text.chars().count();
                    if sel_s == sel_e {
                        // Zero-width caret mark
                        if sel_s >= line_start && sel_s <= line_end {
                            let col = sel_s - line_start;
                            let prefix: String = line_text.chars().take(col).collect();
                            let cx = text_left + text_width(ui, &font_id, &prefix);
                            painter.line_segment(
                                [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                                egui::Stroke::new(1.0_f32, theme.caret_fg),
                            );
                        }
                        continue;
                    }
                    if sel_s < line_end && sel_e > line_start {
                        let local_s = sel_s
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let local_e = sel_e
                            .saturating_sub(line_start)
                            .min(line_text.chars().count());
                        let x0 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_s).collect::<String>(),
                            );
                        let x1 = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(local_e).collect::<String>(),
                            );
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(x0, y),
                                Pos2::new(x1.max(x0 + 2.0), y + row_height),
                            ),
                            0.0,
                            theme.selection_bg,
                        );
                    }
                }

                paint_line_text(
                    &painter,
                    ui,
                    &font_id,
                    text_left,
                    y,
                    line_text,
                    line_start,
                    hl,
                    &lang,
                    &theme,
                    crate::commands::edit::text_is_rtl(),
                );

                // Whitespace / NPC / EOL overlays
                let ws_color = theme.whitespace_fg;
                if self.state.show_whitespace || self.state.show_npc {
                    for (col, ch) in line_text.chars().enumerate() {
                        let x = text_left
                            + text_width(
                                ui,
                                &font_id,
                                &line_text.chars().take(col).collect::<String>(),
                            );
                        let mark = if self.state.show_whitespace && ch == ' ' {
                            Some("·")
                        } else if self.state.show_whitespace && ch == '\t' {
                            Some("→")
                        } else if self.state.show_npc && ch.is_control() {
                            Some("·")
                        } else {
                            None
                        };
                        if let Some(m) = mark {
                            painter.text(
                                Pos2::new(x, y),
                                egui::Align2::LEFT_TOP,
                                m,
                                font_id.clone(),
                                ws_color,
                            );
                        }
                    }
                }
                if self.state.show_eol {
                    let x = text_left + text_width(ui, &font_id, line_text);
                    painter.text(
                        Pos2::new(x, y),
                        egui::Align2::LEFT_TOP,
                        "¶",
                        font_id.clone(),
                        ws_color,
                    );
                }
                if self.state.show_indent_guide {
                    let avail = (rect.right() - text_left).max(0.0);
                    let col_w = text_width(ui, &font_id, " ");
                    if col_w > 0.0 {
                        let tab_w = self.state.settings.tab_width.max(1) as f32;
                        let mut gx = text_left + col_w * tab_w;
                        while gx < text_left + avail {
                            painter.line_segment(
                                [Pos2::new(gx, y), Pos2::new(gx, y + row_height)],
                                egui::Stroke::new(1.0_f32, theme.indent_guide),
                            );
                            gx += col_w * tab_w;
                        }
                    }
                }
                if self.state.word_wrap {
                    let max_w = (rect.right() - text_left - 8.0).max(40.0);
                    if text_width(ui, &font_id, line_text) > max_w {
                        painter.text(
                            Pos2::new(rect.right() - 14.0, y),
                            egui::Align2::LEFT_TOP,
                            "↩",
                            font_id.clone(),
                            ws_color,
                        );
                    }
                }

                // Caret
                let caret = self.state.tabs.active().buffer.caret();
                let line_end = line_start + line_text.chars().count();
                if caret >= line_start && caret <= line_end {
                    let blink_on = if self.state.settings.caret_blink {
                        ctx.request_repaint_after(std::time::Duration::from_millis(500));
                        ((ctx.input(|i| i.time) * 2.0_f64) as i64).rem_euclid(2) == 0
                    } else {
                        true
                    };
                    if blink_on {
                        let col = caret - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(1.0_f32, theme.caret_fg),
                        );
                    }
                }

                // Drop caret while dragging selected text.
                if let Some(drag) = self.sel_text_drag.as_ref() {
                    if drag.tab == self.state.tabs.active_index()
                        && drag.drop_at >= line_start
                        && drag.drop_at <= line_end
                    {
                        let col = drag.drop_at - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(2.0_f32, Color32::from_rgb(220, 140, 40)),
                        );
                    }
                }

                let _ = line_rect;
            }

            // Scrollbar thumb
            if max_scroll > 0.0 {
                let bar_w = 8.0;
                let bar_x = rect.right() - bar_w - 2.0;
                let frac = (self.scroll_line / max_scroll).clamp(0.0, 1.0);
                let thumb_h =
                    (rect.height() * (visible_rows as f32 / display_count as f32)).max(20.0);
                let thumb_y = rect.top() + frac * (rect.height() - thumb_h);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(bar_x, thumb_y), Vec2::new(bar_w, thumb_h)),
                    2.0,
                    theme.gutter_line,
                );
            }
        });
    }

    fn persist_font_size(&mut self) {
        self.state.settings.font_size = self.font_size;
        self.state.settings.save();
    }

    fn clamp_other_view_tab(&mut self) {
        let n = self.state.tabs.len();
        if n == 0 {
            self.other_view_tab = 0;
            self.compare_partner_tab = None;
            return;
        }
        if self.other_view_tab >= n {
            self.other_view_tab = n - 1;
        }
        if let Some(p) = self.compare_partner_tab {
            if p >= n {
                self.compare_partner_tab = None;
            }
        }
    }

    /// Tab that receives typing / undo for the focused pane.
    fn focused_edit_tab(&self) -> usize {
        if self.dual_view && self.focused_pane == EditorPane::Secondary {
            self.other_view_tab
        } else {
            self.state.tabs.active_index()
        }
    }

    /// Run a menu command. Edit/Format IDs temporarily activate the focused dual-view tab.
    fn dispatch_menu_cmd(
        &mut self,
        cmd: &str,
        flags: &mut crate::commands::UiFlags,
    ) -> crate::commands::CmdResult {
        let retarget_menu =
            (cmd.starts_with("IDM_EDIT_") || cmd.starts_with("IDM_FORMAT_")) && self.dual_view;
        let focus = self.focused_edit_tab();
        let saved = self.state.tabs.active_index();
        let redirect = retarget_menu && focus != saved;
        if redirect {
            self.state.tabs.set_active(focus);
        }
        let result = crate::commands::dispatch(cmd, &mut self.state, flags);
        if redirect {
            let n = self.state.tabs.len();
            if n > 0 && saved < n {
                self.state.tabs.set_active(saved);
                self.state.highlight_dirty = true;
            }
            self.clamp_other_view_tab();
        }
        result
    }

    fn follow_focused_caret(&mut self) {
        if self.focused_pane == EditorPane::Secondary && self.dual_view {
            self.follow_caret_other = true;
            self.follow_caret = false;
        } else {
            self.follow_caret = true;
            self.follow_caret_other = false;
        }
    }

    fn ensure_other_view_tab(&mut self) {
        self.clamp_other_view_tab();
        let n = self.state.tabs.len();
        if n <= 1 {
            return;
        }
        let active = self.state.tabs.active_index();
        if self.other_view_tab == active {
            self.other_view_tab = (active + 1) % n;
        }
    }

    fn switch_other_view_now(&mut self) {
        self.dual_view = true;
        self.ensure_other_view_tab();
        let a = self.state.tabs.active_index();
        let b = self.other_view_tab;
        if a == b {
            self.state.status = "Other view shows the same tab".into();
            return;
        }
        self.state.tabs.set_active(b);
        self.other_view_tab = a;
        self.state.highlight_dirty = true;
        std::mem::swap(&mut self.scroll_line, &mut self.scroll_line_other);
        if self.compare_on {
            std::mem::swap(&mut self.compare_left_tab, &mut self.compare_right_tab);
            std::mem::swap(&mut self.compare_left_tags, &mut self.compare_right_tags);
        }
        self.state.status = "Switched to other view".into();
    }

    /// Keep compare panes on the compared pair (left = primary, right = other).
    fn sync_compare_panes(&mut self) {
        if !self.compare_on {
            return;
        }
        let n = self.state.tabs.len();
        if n == 0 {
            self.clear_compare();
            return;
        }
        if self.compare_left_tab >= n || self.compare_right_tab >= n {
            self.clear_compare();
            return;
        }
        if self.compare_left_tab == self.compare_right_tab {
            self.clear_compare();
            return;
        }
        self.dual_view = true;
        self.other_view_tab = self.compare_right_tab;
        if self.state.tabs.active_index() != self.compare_left_tab {
            self.state.tabs.set_active(self.compare_left_tab);
            self.state.highlight_dirty = true;
        }
    }

    fn apply_dual_view_flags(&mut self, flags: &crate::commands::UiFlags) {
        if let Some(on) = flags.sync_scroll_h {
            self.sync_scroll_h = on;
        }
        if let Some(on) = flags.sync_scroll_v {
            self.sync_scroll_v = on;
        }
        if let Some(on) = flags.zoom_sync {
            self.zoom_sync = on;
        }
        if let Some(on) = flags.dual_view {
            self.dual_view = on;
            if on {
                self.ensure_other_view_tab();
            } else {
                self.focused_pane = EditorPane::Primary;
            }
        }
        if let Some(idx) = flags.other_view_tab {
            self.other_view_tab = idx;
            self.dual_view = true;
            self.clamp_other_view_tab();
        }
        if flags.assign_other_view {
            self.other_view_tab = self.state.tabs.active_index();
            self.dual_view = true;
            let n = self.state.tabs.len();
            if n > 1 {
                let next = (self.other_view_tab + 1) % n;
                self.state.tabs.set_active(next);
                self.state.highlight_dirty = true;
            }
        }
        if flags.switch_other_view {
            self.switch_other_view_now();
        }
        if flags.clear_compare {
            self.clear_compare();
        }
        if flags.start_compare {
            self.start_compare();
        }
    }

    fn clear_compare(&mut self) {
        self.compare_on = false;
        self.compare_left_tags.clear();
        self.compare_right_tags.clear();
        self.state.compare_stale = false;
        self.compare_refresh_at = None;
        self.state.status = "Compare cleared".into();
    }

    /// Rebuild compare colours after an edit (debounce ~200 ms while typing).
    fn refresh_compare_if_stale(&mut self, ctx: &egui::Context) {
        if !self.compare_on || !self.state.compare_stale {
            self.compare_refresh_at = None;
            return;
        }
        let now = std::time::Instant::now();
        let due = *self
            .compare_refresh_at
            .get_or_insert_with(|| now + std::time::Duration::from_millis(200));
        if now < due {
            ctx.request_repaint_after(due.saturating_duration_since(now));
            return;
        }
        self.compare_refresh_at = None;
        self.state.compare_stale = false;
        let left = self.compare_left_tab;
        let right = self.compare_right_tab;
        let n = self.state.tabs.len();
        if left >= n || right >= n || left == right {
            self.clear_compare();
            return;
        }
        match self.compute_compare_tags(left, right) {
            Some((lt, rt, del, ins)) => {
                self.compare_left_tags = lt;
                self.compare_right_tags = rt;
                let lname = self
                    .state
                    .tabs
                    .get(left)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "left".into());
                let rname = self
                    .state
                    .tabs
                    .get(right)
                    .map(|d| d.title.clone())
                    .unwrap_or_else(|| "right".into());
                self.state.status = format!("Compare “{lname}” | “{rname}” (−{del} +{ins})");
            }
            None => {
                // Too many lines or missing tabs — leave prior tags; status already set.
            }
        }
    }

    fn compute_compare_tags(
        &mut self,
        left: usize,
        right: usize,
    ) -> Option<(
        Vec<crate::diff::LineKind>,
        Vec<crate::diff::LineKind>,
        usize,
        usize,
    )> {
        let left_lines: Vec<String> = self
            .state
            .tabs
            .get(left)
            .map(|d| {
                (0..d.buffer.line_count())
                    .map(|i| d.buffer.line(i).trim_end_matches(['\n', '\r']).to_string())
                    .collect()
            })
            .unwrap_or_default();
        let right_lines: Vec<String> = self
            .state
            .tabs
            .get(right)
            .map(|d| {
                (0..d.buffer.line_count())
                    .map(|i| d.buffer.line(i).trim_end_matches(['\n', '\r']).to_string())
                    .collect()
            })
            .unwrap_or_default();
        if left_lines.len() > crate::diff::MAX_COMPARE_LINES
            || right_lines.len() > crate::diff::MAX_COMPARE_LINES
        {
            self.state.status = format!(
                "Compare MVP max is {} lines per side",
                crate::diff::MAX_COMPARE_LINES
            );
            return None;
        }
        let ignore_ws = self.state.settings.compare_ignore_ws;
        let left_keys: Vec<String> = left_lines
            .iter()
            .map(|s| {
                if ignore_ws {
                    s.split_whitespace().collect::<Vec<_>>().join(" ")
                } else {
                    s.clone()
                }
            })
            .collect();
        let right_keys: Vec<String> = right_lines
            .iter()
            .map(|s| {
                if ignore_ws {
                    s.split_whitespace().collect::<Vec<_>>().join(" ")
                } else {
                    s.clone()
                }
            })
            .collect();
        let left_refs: Vec<&str> = left_keys.iter().map(|s| s.as_str()).collect();
        let right_refs: Vec<&str> = right_keys.iter().map(|s| s.as_str()).collect();
        let (lt, rt) = crate::diff::diff_line_tags(&left_refs, &right_refs);
        let (del, ins) = crate::diff::count_changes(&lt, &rt);
        Some((lt, rt, del, ins))
    }

    /// Pick the right-hand tab for Compare.
    ///
    /// Order: marked partner → dual-view other pane → tab to the right → tab to the left.
    fn resolve_compare_right(&self) -> Option<usize> {
        pick_compare_right(
            self.state.tabs.len(),
            self.state.tabs.active_index(),
            self.compare_partner_tab,
            self.dual_view,
            self.other_view_tab,
        )
    }

    fn start_compare(&mut self) {
        if self.state.tabs.len() < 2 {
            self.state.status = "Compare needs two open tabs".into();
            return;
        }
        let left = self.state.tabs.active_index();
        let Some(right) = self.resolve_compare_right() else {
            self.state.status = "Compare: pick a different tab first".into();
            return;
        };
        let Some((lt, rt, del, ins)) = self.compute_compare_tags(left, right) else {
            return;
        };
        self.compare_on = true;
        self.state.compare_stale = false;
        self.compare_left_tab = left;
        self.compare_right_tab = right;
        self.compare_left_tags = lt;
        self.compare_right_tags = rt;
        self.compare_partner_tab = None;
        self.dual_view = true;
        self.sync_scroll_v = true;
        self.sync_scroll_h = true;
        self.other_view_tab = right;
        self.state.tabs.set_active(left);
        self.state.highlight_dirty = true;
        self.focused_pane = EditorPane::Primary;
        let lname = self
            .state
            .tabs
            .get(left)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "left".into());
        let rname = self
            .state
            .tabs
            .get(right)
            .map(|d| d.title.clone())
            .unwrap_or_else(|| "right".into());
        self.state.status = format!("Compare “{lname}” | “{rname}” (−{del} +{ins})");
    }

    /// Writable secondary pane: edits `other_view_tab` when this pane has focus.
    fn paint_secondary_pane(&mut self, ui: &mut egui::Ui) {
        let tab = self.other_view_tab;
        let (loading, buf_line_count, hidden) = match self.state.tabs.get(tab) {
            None => {
                ui.label("No tab for other view");
                return;
            }
            Some(doc) => (
                doc.loading,
                doc.buffer.line_count().max(1),
                doc.hidden_lines.clone(),
            ),
        };
        if loading {
            ui.label("Loading…");
            return;
        }

        let font_id = FontId::monospace(self.font_size);
        let row_height = ui.fonts(|f| f.row_height(&font_id)) + 2.0;
        let visible_lines = visible_line_indices(buf_line_count, &hidden);
        let display_count = visible_lines.len().max(1);
        let avail = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(avail, Sense::click_and_drag());

        if response.clicked() || response.drag_started() {
            response.request_focus();
            self.focused_pane = EditorPane::Secondary;
        }

        let visible_rows = {
            let usable = (rect.height() - row_height).max(row_height);
            ((usable / row_height).floor() as usize).max(1)
        };
        let max_scroll = (display_count.saturating_sub(visible_rows) as f32).max(0.0);

        let sync = self.sync_scroll_v || self.sync_scroll_h;
        let mut scroll_line = if sync {
            self.scroll_line
        } else {
            self.scroll_line_other
        };

        let scroll = if response.hovered() {
            ui.input(|i| {
                if i.modifiers.command || i.modifiers.ctrl {
                    0.0
                } else {
                    i.raw_scroll_delta.y
                }
            })
        } else {
            0.0
        };
        if scroll != 0.0 {
            self.follow_caret_other = false;
            scroll_line = (scroll_line - scroll / row_height).clamp(0.0, max_scroll);
            if sync {
                self.scroll_line = scroll_line;
            }
            self.scroll_line_other = scroll_line;
        } else {
            scroll_line = scroll_line.clamp(0.0, max_scroll);
            if sync {
                self.scroll_line_other = self.scroll_line;
                scroll_line = self.scroll_line_other;
            }
        }

        let show_ln = self.state.settings.show_line_numbers;
        let gutter_w =
            if show_ln { 48.0 } else { 12.0 } + f32::from(self.state.settings.gutter_extra);
        let gutter_gap = 8.0;
        let text_left = rect.left() + gutter_w + gutter_gap;
        let gutter_right = rect.left() + gutter_w;

        let hit_index = |ui: &egui::Ui,
                         pos: Pos2,
                         buf: &buffer::TextBuffer,
                         scroll: f32,
                         visible: &[usize]|
         -> usize {
            let first = scroll.floor() as usize;
            let row = first + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
            let row = row.min(visible.len().saturating_sub(1));
            let line = visible.get(row).copied().unwrap_or(0);
            let line_start = buf.line_to_char(line);
            let line_text = buf.line(line);
            let line_body = line_text.trim_end_matches(['\n', '\r']);
            let col = col_from_x(ui, &font_id, line_body, pos.x - text_left);
            line_start + col
        };

        if response.triple_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        doc.buffer.select_line_at(idx);
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        } else if response.double_clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        doc.buffer.select_word_at(idx);
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        } else if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    let (shift, alt) = ui.input(|i| (i.modifiers.shift, i.modifiers.alt));
                    let caret = doc.buffer.caret();
                    let sel_anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(caret);
                    let inside_sel = doc
                        .buffer
                        .selection()
                        .is_some_and(|(s, e)| idx >= s && idx < e);
                    let read_only = doc.read_only;
                    if alt {
                        self.sel_text_drag = None;
                        self.rect_drag = true;
                        self.drag_anchor = Some(idx);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.set_rect_selection(idx, idx);
                        }
                        self.state.status = "Column select (Alt+drag)".into();
                    } else if !shift && inside_sel && !read_only {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.clear_multi_sels();
                        }
                        self.sel_text_drag = Some(SelTextDrag { tab, drop_at: idx });
                        self.drag_anchor = None;
                        self.rect_drag = false;
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        self.sel_text_drag = None;
                        self.rect_drag = false;
                        if shift {
                            self.drag_anchor = Some(sel_anchor);
                            doc.buffer.set_selection(sel_anchor, idx);
                        } else {
                            self.drag_anchor = Some(idx);
                            doc.buffer.set_caret(idx);
                        }
                    }
                }
                self.follow_caret_other = false;
            }
        } else if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    if let Some(drag) = self.sel_text_drag.as_mut() {
                        if drag.tab == tab {
                            drag.drop_at = idx;
                            let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                            ui.ctx().set_cursor_icon(if copy {
                                CursorIcon::Copy
                            } else {
                                CursorIcon::Grabbing
                            });
                        }
                    } else if let Some(anchor) = self.drag_anchor {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if self.rect_drag {
                                doc.set_rect_selection(anchor, idx);
                            } else {
                                doc.buffer.set_selection(anchor, idx);
                            }
                        }
                        self.follow_caret_other = false;
                    }
                }
            }
        } else if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if let Some(doc) = self.state.tabs.get(tab) {
                    let idx = hit_index(ui, pos, &doc.buffer, scroll_line, &visible_lines);
                    let shift = ui.input(|i| i.modifiers.shift);
                    let caret = doc.buffer.caret();
                    let sel_anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(caret);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                        if shift {
                            doc.buffer.set_selection(sel_anchor, idx);
                        } else {
                            doc.buffer.set_caret(idx);
                        }
                    }
                }
                self.drag_anchor = None;
                self.rect_drag = false;
                self.sel_text_drag = None;
                self.follow_caret_other = false;
            }
        }
        if ui.input(|i| i.pointer.any_released()) {
            if self.sel_text_drag.is_some() {
                let copy = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);
                self.finish_sel_text_drag(copy);
            }
            self.drag_anchor = None;
            self.rect_drag = false;
        }

        if response.has_focus()
            && self.focused_pane == EditorPane::Secondary
            && !self.state.find_open
            && !self.show_replace
            && self.handle_editor_input(ui, tab)
        {
            self.follow_caret_other = true;
        }

        if self.follow_caret_other {
            if let Some(doc) = self.state.tabs.get(tab) {
                let caret_line = doc.buffer.char_to_line(doc.buffer.caret());
                let caret_row = display_row_for(&visible_lines, caret_line) as f32;
                if caret_row < scroll_line {
                    scroll_line = caret_row;
                } else if caret_row >= scroll_line + visible_rows as f32 {
                    scroll_line = caret_row - visible_rows as f32 + 1.0;
                }
                scroll_line = scroll_line.clamp(0.0, max_scroll);
                self.scroll_line_other = scroll_line;
                if sync {
                    self.scroll_line = scroll_line;
                }
            }
            self.follow_caret_other = false;
        }

        let theme = self.current_theme();
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, theme.editor_bg);
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(rect.left(), rect.top()),
                Pos2::new(gutter_right, rect.bottom()),
            ),
            0.0,
            theme.gutter_bg,
        );
        if self.focused_pane == EditorPane::Secondary {
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(1.0_f32, Color32::from_rgb(60, 100, 140)),
                egui::StrokeKind::Inside,
            );
        }

        let first_row = scroll_line.floor() as usize;
        let last_row = (first_row + visible_rows + 2).min(display_count);
        let plain = theme.plain_fg;
        let (changed_unsaved, changed_saved) = self
            .state
            .tabs
            .get(tab)
            .map(|d| (d.changed_unsaved.clone(), d.changed_saved.clone()))
            .unwrap_or_default();

        for row in first_row..last_row {
            let Some(&line_idx) = visible_lines.get(row) else {
                break;
            };
            let y = rect.top() + (row as f32 - scroll_line) * row_height;
            if self.compare_on {
                if let Some(kind) = self.compare_right_tags.get(line_idx) {
                    if let Some(bg) = crate::diff::line_kind_bg(*kind) {
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(text_left, y),
                                Pos2::new(rect.right(), y + row_height),
                            ),
                            0.0,
                            bg,
                        );
                    }
                }
            }
            if changed_unsaved.contains(&line_idx) {
                let (join_above, join_below) =
                    change_history_joins(line_idx, false, &changed_unsaved, &changed_saved);
                paint_change_history_bar(
                    &painter,
                    rect.left(),
                    y,
                    row_height,
                    false,
                    join_above,
                    join_below,
                );
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(text_left, y),
                        Pos2::new(rect.right(), y + row_height),
                    ),
                    0.0,
                    change_history_wash(false),
                );
            } else if changed_saved.contains(&line_idx) {
                let (join_above, join_below) =
                    change_history_joins(line_idx, true, &changed_unsaved, &changed_saved);
                paint_change_history_bar(
                    &painter,
                    rect.left(),
                    y,
                    row_height,
                    true,
                    join_above,
                    join_below,
                );
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(text_left, y),
                        Pos2::new(rect.right(), y + row_height),
                    ),
                    0.0,
                    change_history_wash(true),
                );
            }
            if show_ln {
                painter.text(
                    Pos2::new(gutter_right - 4.0, y),
                    egui::Align2::RIGHT_TOP,
                    format!("{}", line_idx + 1),
                    font_id.clone(),
                    theme.line_number_fg,
                );
            }

            let Some(doc) = self.state.tabs.get(tab) else {
                break;
            };
            let line_start = doc.buffer.line_to_char(line_idx);
            let raw = doc.buffer.line(line_idx);
            let line_text = raw.trim_end_matches(['\n', '\r']);
            let primary_sel = doc.buffer.selection();
            let multi = doc.multi_sels.clone();

            if let Some((sel_s, sel_e)) = primary_sel {
                let line_end = line_start + line_text.chars().count();
                if sel_s < line_end && sel_e > line_start {
                    let local_s = sel_s
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let local_e = sel_e
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let x0 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_s).collect::<String>(),
                        );
                    let x1 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_e).collect::<String>(),
                        );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(x0, y),
                            Pos2::new(x1.max(x0 + 2.0), y + row_height),
                        ),
                        0.0,
                        theme.selection_bg,
                    );
                }
            }
            for &(sel_s, sel_e) in &multi {
                if primary_sel == Some((sel_s, sel_e)) {
                    continue;
                }
                let line_end = line_start + line_text.chars().count();
                if sel_s == sel_e {
                    if sel_s >= line_start && sel_s <= line_end {
                        let col = sel_s - line_start;
                        let prefix: String = line_text.chars().take(col).collect();
                        let cx = text_left + text_width(ui, &font_id, &prefix);
                        painter.line_segment(
                            [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                            egui::Stroke::new(1.0_f32, theme.caret_fg),
                        );
                    }
                    continue;
                }
                if sel_s < line_end && sel_e > line_start {
                    let local_s = sel_s
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let local_e = sel_e
                        .saturating_sub(line_start)
                        .min(line_text.chars().count());
                    let x0 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_s).collect::<String>(),
                        );
                    let x1 = text_left
                        + text_width(
                            ui,
                            &font_id,
                            &line_text.chars().take(local_e).collect::<String>(),
                        );
                    painter.rect_filled(
                        Rect::from_min_max(
                            Pos2::new(x0, y),
                            Pos2::new(x1.max(x0 + 2.0), y + row_height),
                        ),
                        0.0,
                        theme.selection_bg,
                    );
                }
            }

            painter.text(
                Pos2::new(text_left, y),
                egui::Align2::LEFT_TOP,
                line_text,
                font_id.clone(),
                plain,
            );

            let caret = doc.buffer.caret();
            let line_end = line_start + line_text.chars().count();
            if caret >= line_start && caret <= line_end {
                let blink_on = if self.state.settings.caret_blink {
                    ui.ctx()
                        .request_repaint_after(std::time::Duration::from_millis(500));
                    ((ui.input(|i| i.time) * 2.0_f64) as i64).rem_euclid(2) == 0
                } else {
                    true
                };
                if blink_on {
                    let col = caret - line_start;
                    let prefix: String = line_text.chars().take(col).collect();
                    let cx = text_left + text_width(ui, &font_id, &prefix);
                    painter.line_segment(
                        [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                        egui::Stroke::new(1.0_f32, theme.caret_fg),
                    );
                }
            }

            if let Some(drag) = self.sel_text_drag.as_ref() {
                if drag.tab == tab && drag.drop_at >= line_start && drag.drop_at <= line_end {
                    let col = drag.drop_at - line_start;
                    let prefix: String = line_text.chars().take(col).collect();
                    let cx = text_left + text_width(ui, &font_id, &prefix);
                    painter.line_segment(
                        [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                        egui::Stroke::new(2.0_f32, Color32::from_rgb(220, 140, 40)),
                    );
                }
            }
        }
    }

    /// Returns true if the caret moved or text changed (caller should follow caret).
    fn handle_editor_input(&mut self, ui: &egui::Ui, tab: usize) -> bool {
        let mut changed = false;
        let mut caret_moved = false;
        let mut copy_text: Option<String> = None;
        let events: Vec<egui::Event> = ui.input(|i| i.events.clone());
        let mods = ui.input(|i| i.modifiers);
        let read_only = self
            .state
            .tabs
            .get(tab)
            .map(|d| d.read_only)
            .unwrap_or(true);

        for event in events {
            match event {
                egui::Event::Paste(t) => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                        continue;
                    }
                    if self.await_paste_bookmarks && tab == self.state.tabs.active_index() {
                        self.await_paste_bookmarks = false;
                        self.last_app_clipboard = Some(t.clone());
                        crate::commands::paste_over_bookmarked_lines(&mut self.state, &t);
                        changed = true;
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&t) {
                                doc.buffer.insert(&t);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Copy | egui::Event::Cut => {
                    if let Some(doc) = self.state.tabs.get(tab) {
                        let multi_text = doc.multi_sels_clipboard_text();
                        if let Some(text) = multi_text {
                            copy_text = Some(text);
                            if matches!(event, egui::Event::Cut) {
                                if read_only {
                                    self.state.status = "Document is read-only".into();
                                } else {
                                    self.state.prepare_edit_at(tab);
                                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                                        let _ = doc.delete_backward_multi();
                                        changed = true;
                                    }
                                }
                            }
                        } else if let Some((s, e)) = doc.buffer.selection() {
                            copy_text = Some(doc.buffer.slice(s, e));
                            if matches!(event, egui::Event::Cut) {
                                if read_only {
                                    self.state.status = "Document is read-only".into();
                                } else {
                                    self.state.prepare_edit_at(tab);
                                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                                        doc.buffer.delete_backward();
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
                egui::Event::Text(t) => {
                    if t.is_empty() || t.chars().all(|c| c.is_control()) {
                        continue;
                    }
                    if mods.command || mods.ctrl {
                        continue;
                    }
                    if read_only {
                        self.state.status = "Document is read-only".into();
                        continue;
                    }
                    self.state.prepare_edit_at(tab);
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        if !doc.insert_multi(&t) {
                            doc.buffer.insert(&t);
                        }
                        changed = true;
                    }
                }
                egui::Event::Key {
                    key: Key::Enter,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.command && !modifiers.ctrl => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        let eol = self.state.settings.default_eol.as_str().to_string();
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&eol) {
                                doc.buffer.insert(&eol);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Tab,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.command && !modifiers.ctrl => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        let n = self.state.settings.tab_width.max(1) as usize;
                        let pad = " ".repeat(n);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.insert_multi(&pad) {
                                doc.buffer.insert(&pad);
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Backspace,
                    pressed: true,
                    ..
                } => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.delete_backward_multi() {
                                doc.buffer.delete_backward();
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::Delete,
                    pressed: true,
                    ..
                } => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                    } else {
                        self.state.prepare_edit_at(tab);
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            if !doc.delete_forward_multi() {
                                doc.buffer.delete_forward();
                            }
                            changed = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowLeft,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.alt {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.buffer.move_word(false, modifiers.shift);
                        }
                        caret_moved = true;
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        if c > 0 {
                            if modifiers.shift {
                                let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                doc.buffer.set_selection(anchor, c - 1);
                            } else {
                                doc.buffer.set_caret(c - 1);
                            }
                            caret_moved = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowRight,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.alt {
                        if let Some(doc) = self.state.tabs.get_mut(tab) {
                            doc.buffer.move_word(true, modifiers.shift);
                        }
                        caret_moved = true;
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        let len = doc.buffer.len_chars();
                        if c < len {
                            if modifiers.shift {
                                let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                doc.buffer.set_selection(anchor, c + 1);
                            } else {
                                doc.buffer.set_caret(c + 1);
                            }
                            caret_moved = true;
                        }
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowUp,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_start(&mut self.state, tab, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, tab, -1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::ArrowDown,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_end(&mut self.state, tab, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, tab, 1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::Home,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if let Some(doc) = self.state.tabs.get_mut(tab) {
                        doc.clear_multi_sels();
                    }
                    if modifiers.command || modifiers.ctrl {
                        go_doc_start(&mut self.state, tab, modifiers.shift);
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        let line = doc.buffer.char_to_line(c);
                        let line_start = doc.buffer.line_to_char(line);
                        if modifiers.shift {
                            let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            doc.buffer.set_selection(anchor, line_start);
                        } else {
                            doc.buffer.set_caret(line_start);
                        }
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::End,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.command || modifiers.ctrl {
                        go_doc_end(&mut self.state, tab, modifiers.shift);
                    } else if let Some(doc) = self.state.tabs.get_mut(tab) {
                        let c = doc.buffer.caret();
                        let line = doc.buffer.char_to_line(c);
                        let raw = doc.buffer.line(line);
                        let n = raw.trim_end_matches(['\n', '\r']).chars().count();
                        let line_end = doc.buffer.line_to_char(line) + n;
                        if modifiers.shift {
                            let anchor = doc.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            doc.buffer.set_selection(anchor, line_end);
                        } else {
                            doc.buffer.set_caret(line_end);
                        }
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageUp,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, tab, -30);
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageDown,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, tab, 30);
                    caret_moved = true;
                }
                _ => {}
            }
        }
        if let Some(t) = copy_text {
            self.last_app_clipboard = Some(t.clone());
            ui.ctx().copy_text(t);
        }
        if changed {
            self.state.mark_text_changed_at(tab);
        }
        changed || caret_moved
    }
}

/// Lines that look like fn / class / struct / def declarations (simple prefix match).
fn collect_func_like_lines(buf: &buffer::TextBuffer) -> Vec<(usize, String)> {
    const KEYS: &[&str] = &[
        "fn ",
        "def ",
        "function ",
        "class ",
        "struct ",
        "impl ",
        "impl<",
        "trait ",
        "interface ",
        "enum ",
        "mod ",
        "type ",
    ];
    const MODS: &[&str] = &[
        "pub ",
        "async ",
        "static ",
        "export ",
        "private ",
        "protected ",
        "public ",
        "crate ",
        "super ",
    ];
    let mut out = Vec::new();
    for i in 0..buf.line_count() {
        let raw = buf.line(i);
        let trimmed = raw.trim_end_matches(['\n', '\r']);
        let mut s = trimmed.trim_start();
        if s.is_empty() || s.starts_with("//") || s.starts_with('#') || s.starts_with("/*") {
            continue;
        }
        for _ in 0..4 {
            let mut hit = false;
            for m in MODS {
                if let Some(rest) = s.strip_prefix(m) {
                    s = rest.trim_start();
                    hit = true;
                    break;
                }
            }
            if !hit {
                break;
            }
        }
        let lower = s.to_ascii_lowercase();
        if KEYS.iter().any(|k| lower.starts_with(k)) {
            let preview: String = trimmed.chars().take(72).collect();
            out.push((i, preview));
        }
    }
    out
}

fn move_caret_vert(state: &mut EditorState, tab: usize, delta: i32) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    let caret = b.buffer.caret();
    let line = b.buffer.char_to_line(caret) as i32 + delta;
    if line < 0 {
        b.buffer.set_caret(0);
        return;
    }
    let line = line as usize;
    if line >= b.buffer.line_count() {
        b.buffer.set_caret(b.buffer.len_chars());
        return;
    }
    let col = caret - b.buffer.line_to_char(b.buffer.char_to_line(caret));
    let raw = b.buffer.line(line);
    let n = raw.trim_end_matches(['\n', '\r']).chars().count();
    b.buffer.set_caret(b.buffer.line_to_char(line) + col.min(n));
}

fn go_doc_start(state: &mut EditorState, tab: usize, select: bool) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    if select {
        let anchor = b
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, 0);
    } else {
        b.buffer.set_caret(0);
    }
}

fn go_doc_end(state: &mut EditorState, tab: usize, select: bool) {
    let Some(b) = state.tabs.get_mut(tab) else {
        return;
    };
    let end = b.buffer.len_chars();
    if select {
        let anchor = b
            .buffer
            .selection()
            .map(|(s, _)| s)
            .unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, end);
    } else {
        b.buffer.set_caret(end);
    }
}

#[cfg(test)]
mod compare_pair_tests {
    use super::pick_compare_right;

    #[test]
    fn needs_two_tabs() {
        assert_eq!(pick_compare_right(1, 0, None, false, 0), None);
    }

    #[test]
    fn prefers_tab_to_the_right() {
        assert_eq!(pick_compare_right(3, 0, None, false, 0), Some(1));
        assert_eq!(pick_compare_right(3, 1, None, false, 1), Some(2));
    }

    #[test]
    fn last_tab_uses_left_neighbor() {
        assert_eq!(pick_compare_right(3, 2, None, false, 2), Some(1));
    }

    #[test]
    fn marked_partner_wins() {
        assert_eq!(pick_compare_right(4, 0, Some(3), true, 1), Some(3));
    }

    #[test]
    fn dual_view_other_when_no_partner() {
        assert_eq!(pick_compare_right(4, 0, None, true, 2), Some(2));
    }
}
