//! egui shell: menus, tabs, viewport editor, find bar.

use crate::editor::EditorState;
use eframe::egui::{self, Color32, FontId, Key, Pos2, Rect, RichText, Sense, Vec2};
use highlight::color_for;

/// Soft teal — ready menu items (works on light and dark themes).
const MENU_READY: Color32 = Color32::from_rgb(42, 148, 118);

pub struct EditorApp {
    state: EditorState,
    find_focus_once: bool,
    /// Vertical scroll in lines.
    scroll_line: f32,
    /// When true, next paint scrolls so the caret stays in view.
    follow_caret: bool,
    show_about: bool,
    /// Drag-select anchor (char index), while primary button is held.
    drag_anchor: Option<usize>,
    show_replace: bool,
    replace_with: String,
    /// Friendly dialog for menu items not wired yet.
    coming_soon: Option<crate::commands::ComingSoon>,
    /// Editor monospace size (zoom).
    font_size: f32,
    show_goto_line: bool,
    goto_line_input: String,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: EditorState::new(),
            find_focus_once: false,
            scroll_line: 0.0,
            follow_caret: false,
            show_about: false,
            drag_anchor: None,
            show_replace: false,
            replace_with: String::new(),
            coming_soon: None,
            font_size: 14.0,
            show_goto_line: false,
            goto_line_input: String::new(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.poll_loads();
        if self.state.poll_tail() {
            self.follow_caret = true;
        }
        if !self.state.pending.is_empty()
            || self.state.tabs.iter().any(|d| d.tail_follow)
        {
            // Steady poll while tailing; avoid hammering every frame.
            ctx.request_repaint_after(std::time::Duration::from_millis(300));
        }

        self.handle_shortcuts(ctx);
        self.menu_bar(ctx);
        self.tab_bar(ctx);
        if self.state.find_open || self.show_replace {
            self.find_replace_bar(ctx);
        }
        self.editor_pane(ctx);
        self.status_bar(ctx);
        self.about_window(ctx);
        self.coming_soon_window(ctx);
        self.goto_line_window(ctx);
    }
}

impl EditorApp {
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
                i.key_pressed(Key::ArrowLeft), // reserved
                i.key_pressed(Key::ArrowRight),
                i.key_pressed(Key::CloseBracket),
                i.key_pressed(Key::OpenBracket),
            )
        });
        let (mods, n, o, s, f, z, y, w, g, a, d, l, i_key, t, _left, _right, close_br, open_br) =
            input;
        let cmd = mods.command || mods.ctrl;

        if cmd && mods.shift && t {
            if self.state.toggle_tail_follow() {
                self.follow_caret = true;
            }
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
        if cmd && f && mods.shift {
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
            self.state.tabs.active_mut().buffer.select_all();
        }
        if cmd && d {
            self.state.tabs.active_mut().buffer.duplicate_line();
            self.state.mark_text_changed();
            self.follow_caret = true;
        }
        if cmd && l && mods.shift {
            self.state.tabs.active_mut().buffer.delete_line();
            self.state.mark_text_changed();
            self.follow_caret = true;
        }
        if cmd && close_br {
            self.state.tabs.active_mut().buffer.indent_lines("    ");
            self.state.mark_text_changed();
            self.follow_caret = true;
        }
        if cmd && open_br {
            self.state.tabs.active_mut().buffer.outdent_lines(4);
            self.state.mark_text_changed();
            self.follow_caret = true;
        }
        if cmd && mods.shift && i_key {
            self.state.format_document();
            self.follow_caret = true;
        }

        if cmd && z && mods.shift {
            self.state.redo();
        } else if cmd && z {
            self.state.undo();
        }
        if cmd && y {
            self.state.redo();
        }
        if cmd && w {
            let idx = self.state.tabs.active_index();
            self.state.tabs.close(idx);
            self.state.highlight_dirty = true;
            self.scroll_line = 0.0;
        }
        if (self.state.find_open || self.show_replace) && cmd && g && mods.shift {
            self.state.find_prev();
            self.follow_caret = true;
        } else if (self.state.find_open || self.show_replace) && cmd && g {
            self.state.find_next();
            self.follow_caret = true;
        }
        if (self.state.find_open || self.show_replace)
            && ctx.input(|i| i.key_pressed(Key::Escape))
        {
            self.state.find_open = false;
            self.show_replace = false;
        }
    }

    fn menu_bar(&mut self, ctx: &egui::Context) {
        let menu = crate::menu_data::load_npp_menu();
        let mut flags = crate::commands::UiFlags {
            show_about: self.show_about,
            find_open: self.state.find_open,
            show_replace: self.show_replace,
            find_focus_once: self.find_focus_once,
            follow_caret: self.follow_caret,
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
            let result = crate::commands::dispatch(&cmd, &mut self.state, &mut flags);
            if result == crate::commands::CmdResult::Stub {
                self.coming_soon = Some(crate::commands::coming_soon_for(&cmd));
                self.state.status = format!("Coming soon: {}", self.coming_soon.as_ref().unwrap().feature);
            }
            if flags.coming_soon.is_some() {
                self.coming_soon = flags.coming_soon.take();
            }
            self.show_about = flags.show_about;
            self.state.find_open = flags.find_open;
            self.show_replace = flags.show_replace;
            self.find_focus_once = flags.find_focus_once;
            self.follow_caret = flags.follow_caret;
            if flags.show_goto_line {
                self.show_goto_line = true;
                let line = self.state.tabs.active().buffer.char_to_line(
                    self.state.tabs.active().buffer.caret(),
                ) + 1;
                self.goto_line_input = line.to_string();
            }
            match flags.zoom_delta {
                Some(1) => self.font_size = (self.font_size + 1.0).min(48.0),
                Some(-1) => self.font_size = (self.font_size - 1.0).max(8.0),
                Some(0) => self.font_size = 14.0,
                _ => {}
            }
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
                ctx.copy_text(t);
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
                if ui.button(text).clicked() {
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
                                        RichText::new(format!("{}.  {label}", i + 1)).color(MENU_READY)
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
                        RichText::new("a Notepad++-inspired editor, rebuilt for fun")
                            .italics()
                            .color(Color32::from_rgb(180, 180, 190)),
                    );
                    ui.add_space(6.0);
                    ui.label(RichText::new("v0.1.2 · Rust · macOS / Linux / Windows").small());
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
                            ("⌘/Ctrl F / ⇧ F", "Find / Replace"),
                            ("⌘/Ctrl G", "Find next"),
                            ("⌘/Ctrl A", "Select all"),
                            ("⌘/Ctrl D", "Duplicate line"),
                            ("⌘/Ctrl ] / [", "Indent / Outdent"),
                            ("⌘/Ctrl ⇧ I", "Format document"),
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
                        RichText::new("We’re building npp-rs in the background — one honest menu at a time.")
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
                    let line = (n - 1).min(self.state.tabs.active().buffer.line_count().saturating_sub(1));
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

    fn tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let count = self.state.tabs.len();
                let mut switch_to = None;
                let mut close_idx = None;
                for i in 0..count {
                    let doc = self.state.tabs.get(i).unwrap();
                    let mut label = doc.title.clone();
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
                    let resp = ui.selectable_label(selected, label);
                    if resp.clicked() {
                        switch_to = Some(i);
                    }
                    if resp.middle_clicked() {
                        close_idx = Some(i);
                    }
                    ui.push_id(("close_tab", i), |ui| {
                        if ui.small_button("×").on_hover_text("Close tab").clicked() {
                            close_idx = Some(i);
                        }
                    });
                }
                if ui.button("+").clicked() {
                    self.state.new_file();
                }
                if let Some(i) = switch_to {
                    self.state.tabs.set_active(i);
                    self.state.highlight_dirty = true;
                    self.scroll_line = 0.0;
                }
                if let Some(i) = close_idx {
                    self.state.tabs.close(i);
                    self.state.highlight_dirty = true;
                    self.scroll_line = 0.0;
                }
            });
        });
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
            ui.horizontal(|ui| {
                ui.label("Find:");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut self.state.find_query)
                        .desired_width(220.0)
                        .hint_text("search text"),
                );
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
                if self.show_replace {
                    ui.separator();
                    ui.label("Replace:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.replace_with).desired_width(140.0),
                    );
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
                    self.state.find_open = false;
                    self.show_replace = false;
                }
                if self.find_focus_once {
                    resp.request_focus();
                    self.find_focus_once = false;
                }
            });
        });
    }

    fn status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            let (lang, line, col, chars, on, status) = {
                let doc = self.state.tabs.active();
                let caret = doc.buffer.caret();
                let line = doc.buffer.char_to_line(caret);
                let col = caret - doc.buffer.line_to_char(line) + 1;
                (
                    doc.language.clone(),
                    line + 1,
                    col,
                    doc.buffer.len_chars(),
                    doc.tail_follow,
                    self.state.status.clone(),
                )
            };
            ui.horizontal(|ui| {
                ui.label(&status);
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
                {
                    if self.state.toggle_tail_follow() {
                        self.follow_caret = true;
                    }
                }
                ui.separator();
                ui.label(format!("Lang: {lang}"));
                ui.separator();
                ui.label(format!("Ln {line}, Col {col}"));
                ui.separator();
                ui.label(format!("{chars} chars"));
            });
        });
    }

    fn editor_pane(&mut self, ctx: &egui::Context) {
        self.state.refresh_highlight_if_needed();

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.state.tabs.active().loading {
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("Loading file…").size(18.0));
                });
                return;
            }

            let font_id = FontId::monospace(self.font_size);
            let row_height = ui.fonts(|f| f.row_height(&font_id)) + 2.0;
            let total_lines = self.state.tabs.active().buffer.line_count().max(1);
            let avail = ui.available_size();
            let (rect, response) = ui.allocate_exact_size(avail, Sense::click_and_drag());
            // Do not steal focus every frame — that breaks Find (Ctrl/Cmd+F).
            if !self.state.find_open && !self.show_replace {
                if response.clicked() || response.drag_started() {
                    response.request_focus();
                }
            } else if response.clicked() {
                // Clicking the editor closes find focus but keeps the bar open.
                response.request_focus();
            }

            if self.state.reset_view {
                self.scroll_line = 0.0;
                self.follow_caret = false;
                self.state.reset_view = false;
            }
            let visible_rows =
                ((rect.height() / row_height).floor() as usize).max(1);
            let max_scroll = (total_lines.saturating_sub(visible_rows) as f32).max(0.0);

            // Mouse-wheel scroll must not be overridden by caret-follow.
            let scroll = ui.input(|i| i.raw_scroll_delta.y);
            if scroll != 0.0 {
                self.follow_caret = false;
                self.scroll_line =
                    (self.scroll_line - scroll / row_height).clamp(0.0, max_scroll);
            }

            let gutter_w = 52.0;
            let text_left = rect.left() + gutter_w;

            let hit_index = |ui: &egui::Ui, pos: Pos2, buf: &buffer::TextBuffer, scroll: f32| -> usize {
                let first = scroll.floor() as usize;
                let line = first
                    + ((pos.y - rect.top()) / row_height).floor().max(0.0) as usize;
                let line = line.min(total_lines.saturating_sub(1));
                let line_start = buf.line_to_char(line);
                let line_text = buf.line(line);
                let line_body = line_text.trim_end_matches(['\n', '\r']);
                let col = col_from_x(ui, &font_id, line_body, pos.x - text_left);
                line_start + col
            };

            // Double-click → word; triple-click → line; click → caret; drag → select.
            if response.triple_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(ui, pos, &self.state.tabs.active().buffer, self.scroll_line);
                    self.state.tabs.active_mut().buffer.select_line_at(idx);
                    self.drag_anchor = None;
                    self.follow_caret = false;
                }
            } else if response.double_clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(ui, pos, &self.state.tabs.active().buffer, self.scroll_line);
                    self.state.tabs.active_mut().buffer.select_word_at(idx);
                    self.drag_anchor = None;
                    self.follow_caret = false;
                }
            } else if response.drag_started() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(ui, pos, &self.state.tabs.active().buffer, self.scroll_line);
                    let shift = ui.input(|i| i.modifiers.shift);
                    if shift {
                        let anchor = self
                            .state
                            .tabs
                            .active()
                            .buffer
                            .selection()
                            .map(|(s, _)| s)
                            .unwrap_or_else(|| self.state.tabs.active().buffer.caret());
                        self.drag_anchor = Some(anchor);
                        self.state.tabs.active_mut().buffer.set_selection(anchor, idx);
                    } else {
                        self.drag_anchor = Some(idx);
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                    }
                    self.follow_caret = false;
                }
            } else if response.dragged() {
                if let (Some(anchor), Some(pos)) = (self.drag_anchor, response.interact_pointer_pos())
                {
                    let idx = hit_index(ui, pos, &self.state.tabs.active().buffer, self.scroll_line);
                    self.state.tabs.active_mut().buffer.set_selection(anchor, idx);
                    self.follow_caret = false;
                }
            } else if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let idx = hit_index(ui, pos, &self.state.tabs.active().buffer, self.scroll_line);
                    let shift = ui.input(|i| i.modifiers.shift);
                    if shift {
                        let anchor = self
                            .state
                            .tabs
                            .active()
                            .buffer
                            .selection()
                            .map(|(s, _)| s)
                            .unwrap_or_else(|| self.state.tabs.active().buffer.caret());
                        self.state.tabs.active_mut().buffer.set_selection(anchor, idx);
                    } else {
                        self.state.tabs.active_mut().buffer.set_caret(idx);
                    }
                    self.drag_anchor = None;
                    self.follow_caret = false;
                }
            }
            if ui.input(|i| i.pointer.any_released()) {
                self.drag_anchor = None;
            }

            // Text input (arrows / typing may request caret follow)
            if response.has_focus() && !self.state.find_open && !self.show_replace {
                if self.handle_editor_input(ui) {
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
                let caret_line = caret_line as f32;
                if caret_line < self.scroll_line {
                    self.scroll_line = caret_line;
                } else if caret_line >= self.scroll_line + visible_rows as f32 {
                    self.scroll_line = caret_line - visible_rows as f32 + 1.0;
                }
                self.scroll_line = self.scroll_line.clamp(0.0, max_scroll);
                self.follow_caret = false;
            }

            let first_line = self.scroll_line.floor() as usize;
            let visible = visible_rows + 2;
            let last_line = (first_line + visible).min(total_lines);

            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 30));

            let hl = &self.state.highlight_cache;
            let lang = self.state.tabs.active().language.clone();
            // gutter_w / text_left already set above for hit-testing

            for line_idx in first_line..last_line {
                let y = rect.top() + (line_idx as f32 - self.scroll_line) * row_height;
                let line_rect = Rect::from_min_size(
                    Pos2::new(rect.left(), y),
                    Vec2::new(rect.width(), row_height),
                );
                // Gutter
                painter.text(
                    Pos2::new(rect.left() + 8.0, y),
                    egui::Align2::LEFT_TOP,
                    format!("{}", line_idx + 1),
                    font_id.clone(),
                    Color32::from_rgb(100, 100, 100),
                );

                let line_start = self.state.tabs.active().buffer.line_to_char(line_idx);
                let raw = self.state.tabs.active().buffer.line(line_idx);
                let line_text = raw.trim_end_matches(['\n', '\r']);

                // Selection highlight on line
                if let Some((sel_s, sel_e)) = self.state.tabs.active().buffer.selection() {
                    let line_end = line_start + line_text.chars().count();
                    if sel_s < line_end && sel_e > line_start {
                        let local_s = sel_s.saturating_sub(line_start).min(line_text.chars().count());
                        let local_e = sel_e.saturating_sub(line_start).min(line_text.chars().count());
                        let x0 = text_left
                            + text_width(ui, &font_id, &line_text.chars().take(local_s).collect::<String>());
                        let x1 = text_left
                            + text_width(ui, &font_id, &line_text.chars().take(local_e).collect::<String>());
                        painter.rect_filled(
                            Rect::from_min_max(
                                Pos2::new(x0, y),
                                Pos2::new(x1.max(x0 + 2.0), y + row_height),
                            ),
                            0.0,
                            Color32::from_rgb(50, 80, 120),
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
                );

                // Caret
                let caret = self.state.tabs.active().buffer.caret();
                let line_end = line_start + line_text.chars().count();
                if caret >= line_start && caret <= line_end {
                    let col = caret - line_start;
                    let prefix: String = line_text.chars().take(col).collect();
                    let cx = text_left + text_width(ui, &font_id, &prefix);
                    painter.line_segment(
                        [Pos2::new(cx, y), Pos2::new(cx, y + row_height - 1.0)],
                        egui::Stroke::new(1.0, Color32::from_rgb(220, 220, 220)),
                    );
                }

                let _ = line_rect;
            }

            // Scrollbar thumb
            if max_scroll > 0.0 {
                let bar_w = 8.0;
                let bar_x = rect.right() - bar_w - 2.0;
                let frac = (self.scroll_line / max_scroll).clamp(0.0, 1.0);
                let thumb_h =
                    (rect.height() * (visible_rows as f32 / total_lines as f32)).max(20.0);
                let thumb_y = rect.top() + frac * (rect.height() - thumb_h);
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(bar_x, thumb_y), Vec2::new(bar_w, thumb_h)),
                    2.0,
                    Color32::from_rgb(80, 80, 80),
                );
            }
        });
    }

    /// Returns true if the caret moved or text changed (caller should follow caret).
    fn handle_editor_input(&mut self, ui: &egui::Ui) -> bool {
        let mut changed = false;
        let mut caret_moved = false;
        let mut copy_text: Option<String> = None;
        let events: Vec<egui::Event> = ui.input(|i| i.events.clone());
        let mods = ui.input(|i| i.modifiers);
        let read_only = self.state.tabs.active().read_only;

        for event in events {
            match event {
                egui::Event::Paste(t) => {
                    if read_only {
                        self.state.status = "Document is read-only".into();
                        continue;
                    }
                    self.state.tabs.active_mut().buffer.insert(&t);
                    changed = true;
                }
                egui::Event::Copy | egui::Event::Cut => {
                    if let Some((s, e)) = self.state.tabs.active().buffer.selection() {
                        copy_text = Some(self.state.tabs.active().buffer.slice(s, e));
                        if matches!(event, egui::Event::Cut) {
                            if read_only {
                                self.state.status = "Document is read-only".into();
                            } else {
                                self.state.tabs.active_mut().buffer.delete_backward();
                                changed = true;
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
                    self.state.tabs.active_mut().buffer.insert(&t);
                    changed = true;
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
                        self.state.tabs.active_mut().buffer.insert("\n");
                        changed = true;
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
                        self.state.tabs.active_mut().buffer.insert("    ");
                        changed = true;
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
                        self.state.tabs.active_mut().buffer.delete_backward();
                        changed = true;
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
                        self.state.tabs.active_mut().buffer.delete_forward();
                        changed = true;
                    }
                }
                egui::Event::Key {
                    key: Key::ArrowLeft,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.alt {
                        self.state.tabs.active_mut().buffer.move_word(false, modifiers.shift);
                        caret_moved = true;
                    } else {
                        let b = self.state.tabs.active_mut();
                        let c = b.buffer.caret();
                        if c > 0 {
                            if modifiers.shift {
                                let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                b.buffer.set_selection(anchor, c - 1);
                            } else {
                                b.buffer.set_caret(c - 1);
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
                    if modifiers.alt {
                        self.state.tabs.active_mut().buffer.move_word(true, modifiers.shift);
                        caret_moved = true;
                    } else {
                        let b = self.state.tabs.active_mut();
                        let c = b.buffer.caret();
                        let len = b.buffer.len_chars();
                        if c < len {
                            if modifiers.shift {
                                let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                                b.buffer.set_selection(anchor, c + 1);
                            } else {
                                b.buffer.set_caret(c + 1);
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
                    if modifiers.command || modifiers.ctrl {
                        // macOS: ⌘↑ = start of document
                        go_doc_start(&mut self.state, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, -1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::ArrowDown,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.command || modifiers.ctrl {
                        // macOS: ⌘↓ = end of document
                        go_doc_end(&mut self.state, modifiers.shift);
                    } else {
                        move_caret_vert(&mut self.state, 1);
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::Home,
                    pressed: true,
                    modifiers,
                    ..
                } => {
                    if modifiers.command || modifiers.ctrl {
                        go_doc_start(&mut self.state, modifiers.shift);
                    } else {
                        let b = self.state.tabs.active_mut();
                        let c = b.buffer.caret();
                        let line = b.buffer.char_to_line(c);
                        let line_start = b.buffer.line_to_char(line);
                        if modifiers.shift {
                            let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            b.buffer.set_selection(anchor, line_start);
                        } else {
                            b.buffer.set_caret(line_start);
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
                        go_doc_end(&mut self.state, modifiers.shift);
                    } else {
                        let b = self.state.tabs.active_mut();
                        let c = b.buffer.caret();
                        let line = b.buffer.char_to_line(c);
                        let raw = b.buffer.line(line);
                        let n = raw.trim_end_matches(['\n', '\r']).chars().count();
                        let line_end = b.buffer.line_to_char(line) + n;
                        if modifiers.shift {
                            let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or(c);
                            b.buffer.set_selection(anchor, line_end);
                        } else {
                            b.buffer.set_caret(line_end);
                        }
                    }
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageUp,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, -30);
                    caret_moved = true;
                }
                egui::Event::Key {
                    key: Key::PageDown,
                    pressed: true,
                    ..
                } => {
                    move_caret_vert(&mut self.state, 30);
                    caret_moved = true;
                }
                _ => {}
            }
        }
        if let Some(t) = copy_text {
            ui.ctx().copy_text(t);
        }
        if changed {
            self.state.mark_text_changed();
        }
        changed || caret_moved
    }
}

fn move_caret_vert(state: &mut EditorState, delta: i32) {
    let b = state.tabs.active_mut();
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

fn go_doc_start(state: &mut EditorState, select: bool) {
    let b = state.tabs.active_mut();
    if select {
        let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, 0);
    } else {
        b.buffer.set_caret(0);
    }
}

fn go_doc_end(state: &mut EditorState, select: bool) {
    let b = state.tabs.active_mut();
    let end = b.buffer.len_chars();
    if select {
        let anchor = b.buffer.selection().map(|(s, _)| s).unwrap_or_else(|| b.buffer.caret());
        b.buffer.set_selection(anchor, end);
    } else {
        b.buffer.set_caret(end);
    }
}

fn text_width(ui: &egui::Ui, font_id: &FontId, text: &str) -> f32 {
    ui.fonts(|f| f.layout_no_wrap(text.to_owned(), font_id.clone(), Color32::WHITE).size().x)
}

fn col_from_x(ui: &egui::Ui, font_id: &FontId, line: &str, x: f32) -> usize {
    if x <= 0.0 || line.is_empty() {
        return 0;
    }
    let chars: Vec<char> = line.chars().collect();
    let mut best = 0usize;
    let mut best_dist = f32::MAX;
    for i in 0..=chars.len() {
        let prefix: String = chars.iter().take(i).collect();
        let w = text_width(ui, font_id, &prefix);
        let d = (w - x).abs();
        if d < best_dist {
            best_dist = d;
            best = i;
        }
        if w > x + 2.0 {
            break;
        }
    }
    best
}

fn paint_line_text(
    painter: &egui::Painter,
    ui: &egui::Ui,
    font_id: &FontId,
    x0: f32,
    y: f32,
    line_text: &str,
    line_start_char: usize,
    spans: &[highlight::Span],
    language: &str,
) {
    if line_text.is_empty() {
        return;
    }

    if language == "plain" || spans.is_empty() {
        painter.text(
            Pos2::new(x0, y),
            egui::Align2::LEFT_TOP,
            line_text,
            font_id.clone(),
            Color32::from_rgb(220, 220, 220),
        );
        return;
    }

    let mut x = x0;
    let chars: Vec<char> = line_text.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let global = line_start_char + i;
        let name = spans
            .iter()
            .find(|s| s.start <= global && global < s.end)
            .map(|s| s.name.as_str())
            .unwrap_or("");
        let (r, g, b) = if name.is_empty() {
            (0.85, 0.85, 0.85)
        } else {
            color_for(name)
        };
        let ch = chars[i].to_string();
        let w = text_width(ui, font_id, &ch);
        painter.text(
            Pos2::new(x, y),
            egui::Align2::LEFT_TOP,
            &ch,
            font_id.clone(),
            Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8),
        );
        x += w;
        i += 1;
    }
}
