use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table},
};

use super::app::{App, View};
use super::view_data::{
    DashboardData, DetailData, DiagnosticsData, ViewColor, ViewData, WelcomeData,
};
use crate::alert;
use crate::article;
use crate::i18n::{self, T};
use crate::proc::FieldValue;

/// Convert a ViewColor to a ratatui Color.
fn view_color_to_color(vc: &ViewColor) -> Color {
    match vc {
        ViewColor::Default => Color::White,
        ViewColor::Green => Color::Green,
        ViewColor::Blue => Color::Blue,
        ViewColor::Red => Color::Red,
        ViewColor::Yellow => Color::Yellow,
        ViewColor::Cyan => Color::Cyan,
        ViewColor::Magenta => Color::Magenta,
        ViewColor::DarkGray => Color::DarkGray,
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    use super::app::HelpLevel;
    let screen_height = f.area().height;
    let max_help_height = (screen_height / 2).max(8);
    let help_height: u16 = match app.help_level {
        HelpLevel::Off => 0,
        HelpLevel::Normal => 5,
        HelpLevel::Detailed => {
            let content_lines = app.help_content_lines as u16;
            (content_lines + 3).clamp(6, max_help_height.min(12))
        }
        HelpLevel::ExtraDetailed => {
            // Dynamic: use actual content lines, generous minimum
            let content_lines = app.help_content_lines as u16;
            (content_lines + 3).clamp(10, max_help_height)
        }
    };
    let show_tab_bar = app.is_multi_host();
    let mut outer_constraints = Vec::new();
    if show_tab_bar {
        outer_constraints.push(Constraint::Length(1)); // tab bar
    }
    outer_constraints.push(Constraint::Min(10)); // main content
    if help_height > 0 {
        outer_constraints.push(Constraint::Length(help_height));
    }
    outer_constraints.push(Constraint::Length(3)); // status bar

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(outer_constraints)
        .split(f.area());

    // Determine chunk indices based on what's shown
    let tab_bar_chunk = if show_tab_bar { Some(0) } else { None };
    let main_chunk_idx = if show_tab_bar { 1 } else { 0 };
    let help_chunk_idx = if help_height > 0 {
        main_chunk_idx + 1
    } else {
        0
    };
    let status_chunk_idx = if help_height > 0 {
        help_chunk_idx + 1
    } else {
        main_chunk_idx + 1
    };

    // Draw tab bar if multi-host
    if let Some(tab_idx) = tab_bar_chunk {
        draw_host_tab_bar(f, app, chunks[tab_idx]);
    }

    // Build the ViewData layer — pre-computed data for the current view.
    let view_data = app.build_view_data();

    // Dashboard, Welcome, Tutorial etc are full-width (no sidebar)
    let is_fullwidth = matches!(
        app.view,
        View::Dashboard | View::Welcome | View::Diagnostics | View::CategoryGuide | View::Tutorial
    );

    let main_area = chunks[main_chunk_idx];

    if is_fullwidth {
        match view_data {
            ViewData::Dashboard(ref data) => draw_dashboard(f, data, main_area),
            ViewData::Welcome(ref data) => {
                draw_welcome(f, data, app.locale, main_area, &app.help_level)
            }
            ViewData::Diagnostics(ref data) => draw_diagnostics(f, data, app, main_area),
            ViewData::CategoryGuide(_) => draw_category_guide(f, app, main_area),
            ViewData::Tutorial(ref data) => draw_tutorial(f, data, main_area),
            _ => {}
        }
    } else {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(22), // sidebar
                Constraint::Min(40),    // content
            ])
            .split(main_area);

        draw_sidebar(f, app, main_chunks[0]);

        if app.pin_filter {
            draw_pin_filter(f, app, main_chunks[1]);
        } else {
            match view_data {
                ViewData::Detail(ref data) => {
                // Check if selected field is numeric — auto-show graph
                let has_numeric_field = app
                    .current_entry_fields()
                    .and_then(|fields| fields.get(app.selected_field))
                    .is_some_and(|f| {
                        matches!(
                            f.value,
                            FieldValue::Bytes(_)
                                | FieldValue::Integer(_)
                                | FieldValue::Float(_)
                                | FieldValue::Duration(_)
                        )
                    });

                if has_numeric_field && app.snapshots.len() >= 2 {
                    // Split: detail on top, auto-graph on bottom
                    let content_split = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                        .split(main_chunks[1]);
                    draw_detail(f, data, app.locale, content_split[0]);
                    draw_auto_graph(f, app, content_split[1]);
                } else {
                    draw_detail(f, data, app.locale, main_chunks[1]);
                }
            }
            ViewData::Diff(_) => draw_diff(f, app, main_chunks[1]),
            ViewData::TableView(_) => draw_table_view(f, app, main_chunks[1]),
            ViewData::ProcessDetail(ref data) => draw_process_detail(f, data, main_chunks[1]),
            _ => {}
            }
        }
    }

    if app.help_level != super::app::HelpLevel::Off {
        draw_help_panel(f, app, chunks[help_chunk_idx]);
        draw_status_bar(f, app, chunks[status_chunk_idx]);
    } else {
        draw_status_bar(f, app, chunks[status_chunk_idx]);
    }

    if app.article_overlay.is_some() {
        draw_article_overlay(f, app, f.area());
    }
}

fn draw_article_overlay(f: &mut Frame, app: &mut App, area: Rect) {
    let (article_id, mut selected_link, requested_scroll) = {
        let Some(state) = app.article_overlay.as_ref() else {
            return;
        };
        (state.article_id.clone(), state.selected_link, state.scroll)
    };
    let article =
        article::find_article_by_id(&article_id).unwrap_or_else(article::fallback_article);
    let popup = centered_rect(85, 80, area);

    let kind_label = match (article.kind, app.locale) {
        (article::ArticleKind::Metric, crate::i18n::Locale::Ja) => "metric",
        (article::ArticleKind::Group, crate::i18n::Locale::Ja) => "group",
        (article::ArticleKind::Concept, crate::i18n::Locale::Ja) => "concept",
        (article::ArticleKind::Metric, crate::i18n::Locale::En) => "metric",
        (article::ArticleKind::Group, crate::i18n::Locale::En) => "group",
        (article::ArticleKind::Concept, crate::i18n::Locale::En) => "concept",
    };

    if article.links.is_empty() {
        selected_link = 0;
    } else if selected_link >= article.links.len() {
        selected_link = article.links.len() - 1;
    }
    if let Some(state) = app.article_overlay.as_mut() {
        state.selected_link = selected_link;
    }

    let has_links = !article.links.is_empty() && popup.width > 54;
    let columns = if has_links {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20), Constraint::Length(32)])
            .split(popup)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(20)])
            .split(popup)
    };
    let content_area = columns[0];
    let links_area = if has_links { Some(columns[1]) } else { None };

    let mut body_lines: Vec<Line> = Vec::new();
    body_lines.push(Line::from(Span::styled(
        article.title(app.locale),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )));
    body_lines.push(Line::from(Span::styled(
        format!("[{}] {}", kind_label, article.id),
        Style::default().fg(Color::DarkGray),
    )));
    body_lines.push(Line::from(""));

    for line in article.body(app.locale).lines() {
        body_lines.push(Line::from(Span::raw(line.to_string())));
    }
    if !has_links {
        body_lines.push(Line::from(""));
        body_lines.push(Line::from(Span::styled(
            if app.locale == crate::i18n::Locale::Ja {
                "j/k: スクロール  PgUp/PgDn: ページ  Tab/Shift+Tab: 関連項目  Enter: 移動  1-9/0: 直接ジャンプ  A/Esc/q: 閉じる"
            } else {
                "j/k: scroll  PgUp/PgDn: page  Tab/Shift+Tab: related  Enter: jump  1-9/0: direct jump  A/Esc/q: close"
            },
            Style::default().fg(Color::DarkGray),
        )));
    }

    let total_lines = body_lines.len();
    let visible_height = content_area.height.saturating_sub(2) as usize;
    app.article_content_lines = total_lines;
    app.article_visible_height = visible_height;

    let max_scroll = total_lines.saturating_sub(visible_height);
    let scroll = requested_scroll.min(max_scroll);
    if let Some(state) = app.article_overlay.as_mut() {
        state.scroll = scroll;
    }

    let title = if total_lines > visible_height {
        format!(" Article [{} / {}] ", scroll + 1, total_lines)
    } else {
        " Article ".to_string()
    };

    let paragraph = Paragraph::new(body_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll as u16, 0));

    f.render_widget(Clear, popup);
    f.render_widget(paragraph, content_area);

    if let Some(right) = links_area {
        let mut link_lines: Vec<Line> = Vec::new();
        link_lines.push(Line::from(Span::styled(
            "SEE ALSO",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        link_lines.push(Line::from(""));
        if article.links.is_empty() {
            link_lines.push(Line::from(Span::styled(
                if app.locale == crate::i18n::Locale::Ja {
                    "(none)"
                } else {
                    "(none)"
                },
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (idx, link) in article.links.iter().enumerate() {
                let is_selected = idx == selected_link;
                let marker = if is_selected { ">" } else { " " };
                let num_hint = if idx < 9 {
                    format!("[{}]", idx + 1)
                } else if idx == 9 {
                    "[0]".to_string()
                } else {
                    "   ".to_string()
                };
                let label = link.label(app.locale);
                let style = if is_selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                link_lines.push(Line::from(vec![
                    Span::styled(
                        format!("{} {} ", marker, num_hint),
                        Style::default().fg(Color::Yellow),
                    ),
                    Span::styled(label.to_string(), style),
                ]));
            }
        }
        link_lines.push(Line::from(""));
        let help = if app.locale == crate::i18n::Locale::Ja {
            "Tab/Shift+Tab: 選択\nEnter: 移動\n1-9/0: 直接ジャンプ\nA/Esc/q: 閉じる"
        } else {
            "Tab/Shift+Tab: select\nEnter: jump\n1-9/0: direct jump\nA/Esc/q: close"
        };
        for h in help.lines() {
            link_lines.push(Line::from(Span::styled(
                h,
                Style::default().fg(Color::DarkGray),
            )));
        }

        let links_paragraph = Paragraph::new(link_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Links ")
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(links_paragraph, right);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn draw_host_tab_bar(f: &mut Frame, app: &App, area: Rect) {
    use super::app::ConnectionStatus;

    let mut spans = Vec::new();
    for (i, host) in app.hosts.iter().enumerate() {
        let fkey = format!("F{}", i + 1);
        // Color based on connection status
        let conn_indicator = match &host.connection_status {
            ConnectionStatus::Local => "",
            ConnectionStatus::Connected { .. } => "\u{25CF}", // filled circle
            ConnectionStatus::Disconnected { .. } => "\u{25CB}", // empty circle
            ConnectionStatus::Connecting => "\u{25D4}",       // half circle
        };
        let label_text = if conn_indicator.is_empty() {
            format!(" [{}:{}] ", fkey, host.label)
        } else {
            format!(" [{}:{} {}] ", fkey, host.label, conn_indicator)
        };

        if i == app.active_host {
            let bg = match &host.connection_status {
                ConnectionStatus::Disconnected { .. } => Color::Yellow,
                _ => Color::Cyan,
            };
            spans.push(Span::styled(
                label_text,
                Style::default()
                    .fg(Color::Black)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            let fg = match &host.connection_status {
                ConnectionStatus::Connected { .. } => Color::Green,
                ConnectionStatus::Disconnected { .. } => Color::Yellow,
                ConnectionStatus::Connecting => Color::DarkGray,
                ConnectionStatus::Local => Color::DarkGray,
            };
            spans.push(Span::styled(label_text, Style::default().fg(fg)));
        }
    }
    let line = Line::from(spans);
    let p = Paragraph::new(line);
    f.render_widget(p, area);
}

// A visual row in the tree-mode sidebar.
enum SidebarRow<'a> {
    Dir {
        label: String,
        depth: usize,
    },
    Leaf {
        key_idx: usize,
        key: &'a str,
        depth: usize,
    },
}

fn build_tree_rows<'a>(app: &'a App) -> Vec<SidebarRow<'a>> {
    // Collect (key_idx, path_segments) from each entry's source path
    let mut items: Vec<(usize, Vec<String>)> = app
        .source_keys
        .iter()
        .enumerate()
        .filter_map(|(i, key)| {
            let source = app.current.entries.get(key)?.source.clone();
            let segs: Vec<String> = source
                .trim_start_matches('/')
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            Some((i, segs))
        })
        .collect();
    items.sort_by(|a, b| a.1.cmp(&b.1));

    let mut rows: Vec<SidebarRow<'a>> = Vec::new();
    let mut prev_segs: Vec<String> = Vec::new();

    for (key_idx, segs) in items {
        if segs.is_empty() {
            continue;
        }
        // Find how many leading segments match the previous item
        let common = prev_segs
            .iter()
            .zip(segs.iter())
            .take_while(|(a, b)| a == b)
            .count();

        // Emit dir headers for newly introduced intermediate segments
        for depth in common..segs.len().saturating_sub(1) {
            rows.push(SidebarRow::Dir {
                label: segs[depth].clone(),
                depth,
            });
        }

        // Emit leaf: depth = path length - 1 (number of ancestors)
        let depth = segs.len().saturating_sub(1);
        // SAFETY: key_idx was built from source_keys.iter().enumerate()
        let key: &'a str = app.source_keys[key_idx].as_str();
        rows.push(SidebarRow::Leaf {
            key_idx,
            key,
            depth,
        });

        prev_segs = segs;
    }
    rows
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let visible_height = area.height.saturating_sub(2) as usize;
    let mode_hint = if app.sidebar_tree {
        " [t:flat]"
    } else {
        " [t:tree]"
    };
    let title = format!(" /proc ({}){} ", app.source_keys.len(), mode_hint);

    if !app.sidebar_tree {
        // ── Flat mode (original behaviour) ──────────────────────────────────
        let selected = app.selected_source;
        let scroll = if selected >= app.sidebar_scroll + visible_height {
            selected - visible_height + 1
        } else if selected < app.sidebar_scroll {
            selected
        } else {
            app.sidebar_scroll
        };

        let items: Vec<ListItem> = app
            .source_keys
            .iter()
            .enumerate()
            .skip(scroll)
            .take(visible_height)
            .map(|(i, key)| {
                let marker = if i == selected { ">" } else { " " };
                let alert_severity = alert::source_max_severity(&app.active_alerts, key);
                let host_key = app.current_host_key();
                let is_pinned = app.pins.is_pinned_source(key, &host_key);
                let pin_marker = if is_pinned { "*" } else { " " };
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if alert_severity == Some("critical") {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else if alert_severity == Some("warning") {
                    Style::default().fg(Color::Yellow)
                } else if alert_severity == Some("info") {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{} ", marker), Style::default().fg(Color::Yellow)),
                    Span::styled(format!("{} ", pin_marker), Style::default().fg(Color::Magenta)),
                    Span::styled(format!("{:<18}", key), style),
                ]))
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(list, area);
        return;
    }

    // ── Tree mode ────────────────────────────────────────────────────────────
    let rows = build_tree_rows(app);

    // Find the visual row index of the selected leaf (for auto-scroll)
    let selected_row = rows.iter().position(
        |r| matches!(r, SidebarRow::Leaf { key_idx, .. } if *key_idx == app.selected_source),
    );

    // Compute scroll: keep selected_row inside the visible window
    let scroll = if let Some(sel_row) = selected_row {
        if sel_row >= app.sidebar_scroll + visible_height {
            sel_row - visible_height + 1
        } else if sel_row < app.sidebar_scroll {
            sel_row
        } else {
            app.sidebar_scroll
        }
    } else {
        app.sidebar_scroll
    };

    let items: Vec<ListItem> = rows
        .iter()
        .skip(scroll)
        .take(visible_height)
        .map(|row| match row {
            SidebarRow::Dir { label, depth } => {
                let indent = "  ".repeat(*depth);
                ListItem::new(Line::from(vec![
                    Span::raw(indent),
                    Span::styled(
                        format!("{}/ ", label),
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]))
            }
            SidebarRow::Leaf {
                key_idx,
                key,
                depth,
            } => {
                let is_selected = *key_idx == app.selected_source;
                let indent = "  ".repeat(*depth);
                let marker = if is_selected { ">" } else { " " };
                let alert_severity = alert::source_max_severity(&app.active_alerts, key);
                let host_key = app.current_host_key();
                let is_pinned = app.pins.is_pinned_source(key, &host_key);
                let pin_marker = if is_pinned { "*" } else { " " };
                let name_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else if alert_severity == Some("critical") {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else if alert_severity == Some("warning") {
                    Style::default().fg(Color::Yellow)
                } else if alert_severity == Some("info") {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };
                // Display the basename of the source path as leaf label
                let leaf_label = app
                    .current
                    .entries
                    .get(*key)
                    .map(|e| {
                        e.source
                            .trim_end_matches('/')
                            .rsplit('/')
                            .next()
                            .unwrap_or(&e.source)
                            .to_string()
                    })
                    .unwrap_or_else(|| (*key).to_string());
                ListItem::new(Line::from(vec![
                    Span::raw(indent),
                    Span::styled(format!("{} ", marker), Style::default().fg(Color::Yellow)),
                    Span::styled(format!("{} ", pin_marker), Style::default().fg(Color::Magenta)),
                    Span::styled(leaf_label, name_style),
                ]))
            }
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn draw_pin_filter(f: &mut Frame, app: &App, area: Rect) {
    let host_key = app.current_host_key();
    let title = if app.locale == crate::i18n::Locale::Ja {
        format!(" ピン済み ({}) [P:解除] ", app.pins.len())
    } else {
        format!(" Pinned ({}) [P:clear] ", app.pins.len())
    };

    let items: Vec<ListItem> = app
        .pins
        .pins()
        .iter()
        .map(|pin| {
            let source_exists = if pin.host == host_key {
                app.current.entries.contains_key(&pin.source)
            } else {
                false
            };
            let line = if let Some(ref field) = pin.field {
                format!("{}.{}", pin.source, field)
            } else {
                pin.source.clone()
            };
            let host_suffix = if pin.host.is_empty() {
                String::new()
            } else {
                format!(" @{}", pin.host)
            };
            let style = if source_exists {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let missing_tag = if source_exists {
                ""
            } else {
                if app.locale == crate::i18n::Locale::Ja {
                    " (不在)"
                } else {
                    " (missing)"
                }
            };
            ListItem::new(Line::from(vec![
                Span::styled("* ", Style::default().fg(Color::Magenta)),
                Span::styled(format!("{}{}{}", line, host_suffix, missing_tag), style),
            ]))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn draw_detail(f: &mut Frame, data: &DetailData, locale: crate::i18n::Locale, area: Rect) {
    let title = format!(" {} — {} ", data.source_name, data.source_path);

    if data.fields.is_empty() {
        let p =
            Paragraph::new("No data").block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(p, area);
        return;
    }

    let visible_height = area.height.saturating_sub(5) as usize; // borders + header + margin

    // Scroll field list so selected field is visible
    let field_scroll = if data.selected_field >= visible_height {
        data.selected_field - visible_height + 1
    } else {
        0
    };

    let rows: Vec<Row> = data
        .fields
        .iter()
        .enumerate()
        .skip(field_scroll)
        .take(visible_height)
        .map(|(i, field)| {
            let is_selected = i == data.selected_field;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            // Override value color if field has an active alert
            let value_color = match field.alert_severity.as_str() {
                "critical" => Color::Red,
                "warning" => Color::Yellow,
                "info" => Color::Cyan,
                _ => view_color_to_color(&field.color),
            };
            let value_modifier = if field.alert_severity == "critical" {
                Modifier::BOLD
            } else {
                Modifier::empty()
            };

            // Show drill-in indicator for table fields
            let name_display = if field.is_table {
                if locale == crate::i18n::Locale::Ja {
                    format!("{} [Enter で展開]", field.name)
                } else {
                    format!("{} [Enter to expand]", field.name)
                }
            } else {
                field.name.clone()
            };

            // Alert indicator prefix
            let alert_prefix = match field.alert_severity.as_str() {
                "critical" => "!! ",
                "warning" => "! ",
                "info" => "i ",
                _ => "",
            };

            let pin_prefix = if field.is_pinned { "* " } else { "" };

            Row::new(vec![
                Cell::from(format!("{}{}{}", pin_prefix, alert_prefix, name_display)).style(style),
                Cell::from(field.value.clone()).style(
                    Style::default()
                        .fg(value_color)
                        .add_modifier(value_modifier),
                ),
                Cell::from(field.unit.clone()).style(Style::default().fg(Color::DarkGray)),
                Cell::from(field.description.clone()).style(Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let pos_indicator = format!(
        " {} — {} {} ",
        data.source_name, data.source_path, data.position
    );

    let table = Table::new(
        rows,
        [
            Constraint::Length(24),
            Constraint::Length(18),
            Constraint::Length(10),
            Constraint::Min(20),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title(pos_indicator))
    .header(
        Row::new(vec![
            i18n::t(locale, T::FIELD),
            i18n::t(locale, T::VALUE),
            i18n::t(locale, T::UNIT),
            i18n::t(locale, T::DESCRIPTION),
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1),
    );

    f.render_widget(table, area);
}

fn draw_diff(f: &mut Frame, app: &App, area: Rect) {
    if app.diffs.is_empty() {
        let p = Paragraph::new(i18n::t(app.locale, T::NO_CHANGES))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", i18n::t(app.locale, T::DIFF))),
            )
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(p, area);
        return;
    }

    let rows: Vec<Row> = app
        .diffs
        .iter()
        .map(|d| {
            Row::new(vec![
                Cell::from(d.source.clone()).style(Style::default().fg(Color::Yellow)),
                Cell::from(d.field.clone()),
                Cell::from(d.old_value.clone()).style(Style::default().fg(Color::Red)),
                Cell::from("->".to_string()).style(Style::default().fg(Color::DarkGray)),
                Cell::from(d.new_value.clone()).style(Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let offset = app.diff_offset();
    let title = if offset > 1 {
        format!(
            " Diff (T-{}, {} changes) [/] navigate ",
            offset,
            app.diffs.len()
        )
    } else {
        format!(" Diff ({} changes) [/] navigate ", app.diffs.len())
    };
    let table = Table::new(
        rows,
        [
            Constraint::Length(16),
            Constraint::Length(24),
            Constraint::Length(18),
            Constraint::Length(3),
            Constraint::Length(18),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title(title))
    .header(
        Row::new(vec![
            i18n::t(app.locale, T::SOURCE),
            i18n::t(app.locale, T::FIELD),
            i18n::t(app.locale, T::OLD),
            "",
            i18n::t(app.locale, T::NEW),
        ])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1),
    );

    f.render_widget(table, area);
}

fn draw_table_view(f: &mut Frame, app: &App, area: Rect) {
    let Some(fields) = app.table_view_entry_fields() else {
        return;
    };

    // Find the table field (prefer the one at selected_field, else first table)
    let table_field = fields
        .get(app.selected_field)
        .filter(|f| matches!(f.value, FieldValue::Table(_)))
        .or_else(|| {
            fields
                .iter()
                .find(|f| matches!(f.value, FieldValue::Table(_)))
        });

    let Some(field) = table_field else {
        let p = Paragraph::new(" No table data in this source")
            .block(Block::default().borders(Borders::ALL).title(" Table View "));
        f.render_widget(p, area);
        return;
    };

    if let FieldValue::Table(ref data) = field.value {
        let header_titles: Vec<&str> = match app.table_view_source_name() {
            "mounts" => vec!["Device", "Mountpoint", "FSType", "Options"],
            "partitions" => vec!["Name", "Size", "Major", "Minor"],
            "net/dev" => vec!["Interface", "RX Bytes", "RX Pkts", "TX Bytes", "TX Pkts"],
            "diskstats" => vec![
                "Device",
                "Reads",
                "Read Bytes",
                "Writes",
                "Written",
                "InFlight",
            ],
            "processes" => vec!["PID", "Name", "State", "RSS", "Threads", "UID", "FDs"],
            "swaps" => vec!["Filename", "Type", "Size", "Used", "Priority"],
            "modules" => vec!["Name", "Size", "Used By", "State"],
            "net/tcp" => vec!["Local Addr", "Remote Addr", "State", "UID"],
            "net/udp" => vec!["Local Addr", "Remote Addr", "State", "UID"],
            "net/unix" => vec!["Type", "State", "Inode", "Path"],
            "net/arp" => vec!["IP Address", "HW Address", "Device"],
            "net/route" => vec!["Iface", "Destination", "Gateway", "Mask", "Metric"],
            "crypto" => vec!["Name", "Driver", "Module", "Type", "BlockSize"],
            "locks" => vec!["Type", "Mode", "RW", "PID", "Range"],
            "interrupts" => vec!["IRQ", "Count", "Type", "Device"],
            "devices" => vec!["Major", "Name"],
            "cgroups" => vec!["Name", "Hierarchy", "NumCGroups", "Enabled"],
            _ => {
                let ncols = data.first().map(|r| r.len()).unwrap_or(4);
                (0..ncols)
                    .map(|i| match i {
                        0 => "Col1",
                        1 => "Col2",
                        2 => "Col3",
                        3 => "Col4",
                        4 => "Col5",
                        5 => "Col6",
                        6 => "Col7",
                        _ => "Col",
                    })
                    .collect()
            }
        };

        let visible_rows = (area.height as usize).saturating_sub(5);
        let start = app
            .table_scroll
            .min(data.len().saturating_sub(visible_rows));

        let is_processes = app.table_view_source_name() == "processes";
        let rows: Vec<Row> = data
            .iter()
            .enumerate()
            .skip(start)
            .take(visible_rows)
            .map(|(i, row)| {
                let cells: Vec<Cell> = row.iter().map(|c| Cell::from(c.as_str())).collect();
                let r = Row::new(cells);
                if is_processes && i == app.table_scroll {
                    r.style(Style::default().bg(Color::DarkGray).fg(Color::White))
                } else {
                    r
                }
            })
            .collect();

        let ncols = header_titles.len().max(1);
        let pct = (100 / ncols) as u16;
        let widths: Vec<Constraint> = header_titles
            .iter()
            .map(|_| Constraint::Percentage(pct))
            .collect();

        let enter_hint = if app.table_view_source_name() == "processes" {
            " [Enter: detail]"
        } else {
            ""
        };
        let title = format!(
            " {} / {} ({} rows, {}-{}){} ",
            app.table_view_source_name(),
            field.name,
            data.len(),
            start + 1,
            (start + visible_rows).min(data.len()),
            enter_hint,
        );
        let table = Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL).title(title))
            .header(
                Row::new(header_titles)
                    .style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .bottom_margin(1),
            );

        f.render_widget(table, area);
    }
}

fn draw_process_detail(f: &mut Frame, data: &super::view_data::ProcessDetailData, area: Rect) {
    use ratatui::widgets::Paragraph;

    if let Some(ref err) = data.error {
        let p = Paragraph::new(err.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Process Detail "),
        );
        f.render_widget(p, area);
        return;
    }

    let title = format!(" PID {} — {} ", data.pid, data.comm);
    let inner_height = area.height.saturating_sub(2) as usize; // subtract border

    // Build all lines
    let mut lines: Vec<Line> = Vec::new();
    for field in &data.fields {
        // Section header: field name
        let section_name = field
            .name
            .trim_start_matches("status.")
            .trim_start_matches("io.");
        if !field.table_rows.is_empty() {
            // Table field
            lines.push(Line::from(vec![Span::styled(
                format!("▸ {}", section_name),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]));
            // Description
            for desc_line in wrap_text(&field.description, area.width.saturating_sub(4) as usize) {
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", desc_line),
                    Style::default().fg(Color::DarkGray),
                )]));
            }
            // Table header
            if !field.table_headers.is_empty() {
                let header_str = field.table_headers.join("  │  ");
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", header_str),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]));
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", "─".repeat(header_str.len())),
                    Style::default().fg(Color::DarkGray),
                )]));
            }
            // Table rows (cap at 50 rows to avoid overwhelming the view)
            let display_rows = field.table_rows.iter().take(50);
            for row in display_rows {
                lines.push(Line::from(vec![Span::raw(format!(
                    "  {}",
                    row.join("  │  ")
                ))]));
            }
            if field.table_rows.len() > 50 {
                lines.push(Line::from(vec![Span::styled(
                    format!("  … {} more rows", field.table_rows.len() - 50),
                    Style::default().fg(Color::DarkGray),
                )]));
            }
        } else {
            // Scalar field: name = value
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:<30}", section_name),
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(" = ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    field.value.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            // Description
            for desc_line in wrap_text(&field.description, area.width.saturating_sub(4) as usize) {
                lines.push(Line::from(vec![Span::styled(
                    format!("  {}", desc_line),
                    Style::default().fg(Color::DarkGray),
                )]));
            }
        }
        lines.push(Line::from("")); // blank line between fields
    }

    let total_lines = lines.len();
    let max_scroll = total_lines.saturating_sub(inner_height);
    let scroll = data.scroll.min(max_scroll);

    let visible: Vec<Line> = lines.into_iter().skip(scroll).take(inner_height).collect();

    let scroll_hint = if total_lines > inner_height {
        format!(" ({}/{}) ↑↓ to scroll ", scroll + 1, total_lines)
    } else {
        String::new()
    };
    let full_title = format!("{}{}", title, scroll_hint);

    let p = Paragraph::new(visible).block(Block::default().borders(Borders::ALL).title(full_title));
    f.render_widget(p, area);
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// BL-074: Render the interactive tutorial view.
fn draw_tutorial(f: &mut Frame, data: &super::view_data::TutorialData, area: Rect) {
    let title = format!(" {} ", data.title);

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    // Progress bar
    let filled = data.step + 1;
    let bar: String = (0..data.total_steps)
        .map(|i| if i < filled { '#' } else { '-' })
        .collect();
    lines.push(Line::from(vec![
        Span::raw("  Progress: ["),
        Span::styled(&bar, Style::default().fg(Color::Green)),
        Span::raw("]"),
    ]));
    lines.push(Line::from(""));

    // Body text
    for line in data.body.lines() {
        lines.push(Line::from(vec![Span::raw("  "), Span::raw(line)]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        format!("  {}", data.nav_hint),
        Style::default().fg(Color::DarkGray),
    )]));

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(p, area);
}

fn draw_welcome(
    f: &mut Frame,
    data: &WelcomeData,
    _locale: crate::i18n::Locale,
    area: Rect,
    help_level: &super::app::HelpLevel,
) {
    let title = format!(" {} ", data.title);

    let mut keys_text = vec![Line::from("")];
    for (key, desc) in &data.keybindings {
        keys_text.push(Line::from(vec![
            Span::styled(format!("  {:<10}", key), Style::default().fg(Color::Yellow)),
            Span::raw(desc.as_str()),
        ]));
    }

    // P-A2: Show advanced keybindings when help_level is Detailed or ExtraDetailed
    let show_advanced = matches!(
        help_level,
        super::app::HelpLevel::Detailed | super::app::HelpLevel::ExtraDetailed
    );
    if show_advanced && !data.advanced_keybindings.is_empty() {
        keys_text.push(Line::from(""));
        for (key, desc) in &data.advanced_keybindings {
            keys_text.push(Line::from(vec![
                Span::styled(
                    format!("  {:<10}", key),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(desc.as_str()),
            ]));
        }
    }

    keys_text.push(Line::from(""));
    keys_text.push(Line::from(vec![Span::styled(
        format!("  {}", data.footer),
        Style::default().fg(Color::Cyan),
    )]));

    // Did you know? tip
    if !data.tip.is_empty() {
        keys_text.push(Line::from(""));
        keys_text.push(Line::from(vec![Span::styled(
            "  \u{1f4a1} Did you know?  ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]));
        // Wrap long tips into multiple lines
        for line in textwrap_simple(&data.tip, area.width.saturating_sub(6) as usize) {
            keys_text.push(Line::from(vec![Span::styled(
                format!("     {}", line),
                Style::default().fg(Color::White),
            )]));
        }
    }
    keys_text.push(Line::from(""));

    let p = Paragraph::new(keys_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(p, area);
}

/// Simple text wrapping for tips display.
fn textwrap_simple(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 || text.is_empty() {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current = word.to_string();
        } else if current.len() + 1 + word.len() > max_width {
            lines.push(current);
            current = word.to_string();
        } else {
            current.push(' ');
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn draw_dashboard(f: &mut Frame, data: &DashboardData, area: Rect) {
    // Detect locale from network headers (first header is "IF" for Japanese)
    let is_ja = data
        .network
        .headers
        .first()
        .map(|h| h == "IF")
        .unwrap_or(false);

    // Split into sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // loadavg + uptime
            Constraint::Length(5), // meminfo (bar graphs)
            Constraint::Length(5), // CPU stat (bar graphs)
            Constraint::Length(9), // net/dev
            Constraint::Length(3), // disk + temp + fd
            Constraint::Min(4),    // sparkline graphs
        ])
        .split(area);

    // Section 0: Load Average + Uptime + Diagnostic badge (P-A1)
    let sec0_style = section_style(data.selected_section == 0);
    let mut load_spans = vec![
        Span::styled(" Load: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            &data.load.load1,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" / "),
        Span::styled(&data.load.load5, Style::default().fg(Color::Yellow)),
        Span::raw(" / "),
        Span::styled(&data.load.load15, Style::default().fg(Color::DarkGray)),
        Span::raw("    "),
        Span::styled(" Up: ", Style::default().fg(Color::Yellow)),
        Span::styled(&data.load.uptime, Style::default().fg(Color::Cyan)),
        Span::raw("    "),
    ];
    // P-A1: Diagnostic badge
    if data.diag_count > 0 {
        let badge_color = data
            .diag_severity
            .as_ref()
            .map(|c| view_color_to_color(c))
            .unwrap_or(Color::Yellow);
        let badge_text = if is_ja {
            format!("\u{26a0} {}件の問題 (X:診断)", data.diag_count)
        } else {
            format!("\u{26a0} {} issues (X:diagnostics)", data.diag_count)
        };
        load_spans.push(Span::styled(
            badge_text,
            Style::default()
                .fg(badge_color)
                .add_modifier(Modifier::BOLD),
        ));
    } else {
        let badge_text = if is_ja {
            "\u{2713} 正常"
        } else {
            "\u{2713} healthy"
        };
        load_spans.push(Span::styled(badge_text, Style::default().fg(Color::Green)));
    }
    let load_line = Line::from(load_spans);
    let p = Paragraph::new(load_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(if is_ja {
                " 負荷 / 稼働時間 "
            } else {
                " Load / Uptime "
            })
            .border_style(sec0_style),
    );
    f.render_widget(p, sections[0]);

    // Section 1: Memory with usage bar
    let sec1_style = section_style(data.selected_section == 1);
    let mem_lines = vec![
        Line::from(vec![
            Span::styled(" RAM  ", Style::default().fg(Color::Yellow)),
            Span::styled(&data.mem_bar, bar_color(data.mem_used_pct)),
            Span::styled(
                format!(" {}%", data.mem_used_pct),
                bar_color(data.mem_used_pct),
            ),
            Span::styled(
                format!(
                    "  {} / {}",
                    format_bytes_short(data.mem_used_bytes),
                    format_bytes_short(data.mem_total_bytes)
                ),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled(" Swap ", Style::default().fg(Color::Yellow)),
            Span::styled(&data.swap_bar, bar_color(data.swap_used_pct)),
            Span::styled(
                format!(" {}%", data.swap_used_pct),
                bar_color(data.swap_used_pct),
            ),
            Span::styled(
                format!(
                    "  {} / {}",
                    format_bytes_short(data.swap_used_bytes),
                    format_bytes_short(data.swap_total_bytes)
                ),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled(" Cache: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&data.cached),
            Span::styled("  Buf: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&data.buffers),
        ]),
    ];
    let mem_p = Paragraph::new(mem_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(if is_ja { " メモリ " } else { " Memory " })
            .border_style(sec1_style),
    );
    f.render_widget(mem_p, sections[1]);

    // Section 2: CPU stat with usage bars
    let sec2_style = section_style(data.selected_section == 2);
    let cpu_lines = vec![
        Line::from(vec![
            Span::styled(" CPU  ", Style::default().fg(Color::Yellow)),
            Span::styled(&data.cpu_bar, bar_color(data.cpu_used_pct)),
            Span::styled(
                format!(" {}%", data.cpu_used_pct),
                bar_color(data.cpu_used_pct),
            ),
        ]),
        Line::from(vec![
            Span::styled("  usr:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", data.cpu_user_pct),
                Style::default().fg(Color::Blue),
            ),
            Span::styled("  sys:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", data.cpu_sys_pct),
                Style::default().fg(Color::Magenta),
            ),
            Span::styled("  iow:", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", data.cpu_io_pct),
                Style::default().fg(if data.cpu_io_pct > 10 {
                    Color::Red
                } else {
                    Color::DarkGray
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ctx:", Style::default().fg(Color::DarkGray)),
            Span::raw(&data.ctx_switches),
            Span::styled("  run:", Style::default().fg(Color::DarkGray)),
            Span::raw(&data.procs_running),
        ]),
    ];
    let cpu_p = Paragraph::new(cpu_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" CPU ")
            .border_style(sec2_style),
    );
    f.render_widget(cpu_p, sections[2]);

    // Section 3: Network (net/dev table)
    let sec3_style = section_style(data.selected_section == 3);
    let net_rows: Vec<Row> = data
        .network
        .rows
        .iter()
        .map(|row| {
            let cells: Vec<Cell> = row.iter().map(|c| Cell::from(c.as_str())).collect();
            Row::new(cells)
        })
        .collect();

    let net_headers: Vec<&str> = data.network.headers.iter().map(|s| s.as_str()).collect();
    let net_table = Table::new(
        net_rows,
        vec![
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(12),
        ],
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(if is_ja {
                " ネットワーク "
            } else {
                " Network "
            })
            .border_style(sec3_style),
    )
    .header(
        Row::new(net_headers)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1),
    );
    f.render_widget(net_table, sections[3]);

    // Section 4: Disk + Temperature + FD (summary line)
    let sec4_style = section_style(data.selected_section == 4);
    let disk_pct = &data.system.disk_pct;
    let temp = &data.system.temp;
    let fd_pct = &data.system.fd_pct;

    let sys_line = Line::from(vec![
        Span::styled(" Disk: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{}%", disk_pct),
            Style::default().fg(if disk_pct.parse::<f64>().unwrap_or(0.0) > 80.0 {
                Color::Red
            } else {
                Color::Green
            }),
        ),
        Span::raw("    "),
        Span::styled(" Temp: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            if *temp == "-" {
                "N/A".to_string()
            } else {
                format!("{}C", temp)
            },
            Style::default().fg(if temp.parse::<f64>().unwrap_or(0.0) > 75.0 {
                Color::Red
            } else {
                Color::Cyan
            }),
        ),
        Span::raw("    "),
        Span::styled(" FD: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{}%", fd_pct),
            Style::default().fg(if fd_pct.parse::<f64>().unwrap_or(0.0) > 80.0 {
                Color::Red
            } else {
                Color::Green
            }),
        ),
    ]);
    let p = Paragraph::new(sys_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(if is_ja {
                " ディスク / 温度 / FD "
            } else {
                " Disk / Temp / FD "
            })
            .border_style(sec4_style),
    );
    f.render_widget(p, sections[4]);

    // Section 5: Sparkline graphs (load + memory history)
    if sections.len() > 5 && sections[5].height >= 4 {
        let graph_area = sections[5];
        let graph_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(graph_area);

        // Load history sparkline
        let load_title = if is_ja {
            format!(" 負荷 ({}件) ", data.load_history.len())
        } else {
            format!(" Load ({} pts) ", data.load_history.len())
        };
        let load_sparkline = ratatui::widgets::Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(load_title)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .data(&data.load_history)
            .max(100)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(load_sparkline, graph_chunks[0]);

        // Memory usage % history sparkline
        let mem_title = if is_ja {
            format!(" メモリ使用率 ({}件) ", data.mem_history.len())
        } else {
            format!(" Memory % ({} pts) ", data.mem_history.len())
        };
        let mem_sparkline = ratatui::widgets::Sparkline::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(mem_title)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .data(&data.mem_history)
            .max(100)
            .style(Style::default().fg(Color::Green));
        f.render_widget(mem_sparkline, graph_chunks[1]);
    }
}

fn draw_auto_graph(f: &mut Frame, app: &App, area: Rect) {
    let source_key = app.current_source_name();
    let field_name = app
        .current_entry_fields()
        .and_then(|fields| fields.get(app.selected_field))
        .map(|f| f.name.clone())
        .unwrap_or_default();

    // Helper closure to extract a numeric value from a snapshot entry.
    let extract = |snap: &crate::proc::Snapshot| -> Option<f64> {
        let entry = snap.entries.get(source_key)?;
        let field = entry.fields.iter().find(|f| f.name == field_name)?;
        match &field.value {
            FieldValue::Bytes(b) => Some(*b as f64),
            FieldValue::Integer(i) => Some(*i as f64),
            FieldValue::Float(f) => Some(*f),
            FieldValue::Duration(d) => Some(*d),
            _ => None,
        }
    };

    // Compute min/max from ALL snapshots (not just the visible window) so the
    // y-axis scale stays fixed across refreshes — prevents the "pikon" bounce.
    let all_values: Vec<f64> = app
        .snapshots
        .iter()
        .filter_map(|s| extract(s))
        .chain(app.current.entries.get(source_key).and_then(|entry| {
            entry
                .fields
                .iter()
                .find(|f| f.name == field_name)
                .and_then(|field| match &field.value {
                    FieldValue::Bytes(b) => Some(*b as f64),
                    FieldValue::Integer(i) => Some(*i as f64),
                    FieldValue::Float(f) => Some(*f),
                    FieldValue::Duration(d) => Some(*d),
                    _ => None,
                })
        }))
        .collect();

    if all_values.is_empty() {
        let p = Paragraph::new(" Collecting data...").block(Block::default().borders(Borders::ALL));
        f.render_widget(p, area);
        return;
    }

    let min_v = all_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_v = all_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cur_v = *all_values.last().unwrap();
    let range = max_v - min_v;

    // Collect only the visible time window for display.
    let window = app.graph_time_window;
    let snap_slice = if app.snapshots.len() > window.saturating_sub(1) {
        &app.snapshots[app.snapshots.len() - window.saturating_sub(1)..]
    } else {
        &app.snapshots[..]
    };
    let mut values: Vec<f64> = snap_slice.iter().filter_map(|s| extract(s)).collect();
    if let Some(entry) = app.current.entries.get(source_key) {
        if let Some(field) = entry.fields.iter().find(|f| f.name == field_name) {
            if let Some(v) = match &field.value {
                FieldValue::Bytes(b) => Some(*b as f64),
                FieldValue::Integer(i) => Some(*i as f64),
                FieldValue::Float(f) => Some(*f),
                FieldValue::Duration(d) => Some(*d),
                _ => None,
            } {
                values.push(v);
            }
        }
    }

    // Normalize the visible window to 0–100 using the all-time min/max scale.
    // .max(100) on the Sparkline pins the y-axis ceiling so it never rescales.
    let sparkline_data: Vec<u64> = if range < f64::EPSILON {
        vec![50; values.len()]
    } else {
        values
            .iter()
            .map(|v| ((v - min_v) / range * 100.0) as u64)
            .collect()
    };

    let title = format!(
        " {} ▸ min:{} max:{} cur:{} [{} / {}] ",
        field_name,
        format_value_short(min_v),
        format_value_short(max_v),
        format_value_short(cur_v),
        values.len(),
        app.graph_time_window_label(),
    );

    let sparkline = ratatui::widgets::Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .data(&sparkline_data)
        .max(100) // fix y-axis to the all-time max; prevents rescaling on each frame
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(sparkline, area);
}

fn bar_color(pct: u64) -> Style {
    if pct > 90 {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else if pct > 70 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Green)
    }
}

/// Format a raw f64 value for the graph title. Uses SI suffixes for large ints,
/// two decimal places for small floats.
fn format_value_short(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let abs = v.abs();
    if abs >= 1_000_000_000.0 {
        format!("{:.1}G", v / 1_000_000_000.0)
    } else if abs >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{:.1}K", v / 1_000.0)
    } else if abs < 1.0 {
        format!("{:.3}", v)
    } else {
        format!("{:.1}", v)
    }
}

fn format_bytes_short(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1}G", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.0}M", bytes as f64 / MIB as f64)
    } else {
        format!("{}K", bytes / 1024)
    }
}

fn section_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn draw_diagnostics(f: &mut Frame, data: &DiagnosticsData, app: &App, area: Rect) {
    let is_ja = app.locale == crate::i18n::Locale::Ja;
    let selected_has_runbook = data
        .findings
        .get(app.selected_diagnostic)
        .and_then(|f| f.runbook_url.as_ref())
        .is_some();
    let hint = if is_ja {
        let base = "Enter:ジャンプ  Backspace:戻る  c:コピー";
        if selected_has_runbook {
            format!("{}  R:Runbook", base)
        } else {
            base.to_string()
        }
    } else {
        let base = "Enter:jump  Backspace:back  c:copy";
        if selected_has_runbook {
            format!("{}  R:runbook", base)
        } else {
            base.to_string()
        }
    };
    let title = format!(" {} ({}) ", data.title, hint);

    if data.findings.is_empty() {
        let msg = if is_ja {
            " \u{2713} 問題は検出されませんでした"
        } else {
            " \u{2713} No issues detected"
        };
        let p = Paragraph::new(msg)
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Green));
        f.render_widget(p, area);
        return;
    }

    // If picker is open, split area into table + picker
    let (table_area, picker_area) = if app.selected_related_metric.is_some() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(8)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        (area, None)
    };

    let rows: Vec<Row> = data
        .findings
        .iter()
        .enumerate()
        .map(|(i, finding)| {
            let is_selected = i == app.selected_diagnostic;
            let sev_color = view_color_to_color(&finding.severity_color);
            let sev_style = Style::default().fg(sev_color);
            let sev_style = if matches!(finding.severity_color, ViewColor::Red) {
                sev_style.add_modifier(Modifier::BOLD)
            } else {
                sev_style
            };
            let has_metrics = !finding.related_metrics.is_empty();
            let indicator = if has_metrics { " >" } else { "" };
            let runbook_icon = if finding.runbook_url.is_some() {
                "\u{1F4D6} "
            } else {
                ""
            };
            let row = Row::new(vec![
                Cell::from(finding.severity.clone()).style(sev_style),
                Cell::from(finding.source.clone()).style(Style::default().fg(Color::DarkGray)),
                Cell::from(format!("{}{}{}", runbook_icon, finding.title, indicator))
                    .style(sev_style),
                Cell::from(finding.detail.clone()),
                Cell::from(finding.suggestion.clone()).style(Style::default().fg(Color::Green)),
            ])
            .height(2);
            if is_selected {
                row.style(Style::default().bg(Color::DarkGray))
            } else {
                row
            }
        })
        .collect();

    let header_texts = if is_ja {
        vec!["重要度", "ソース", "問題", "詳細", "対処法"]
    } else {
        vec!["Sev", "Source", "Issue", "Detail", "Suggestion"]
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Length(30),
            Constraint::Length(40),
            Constraint::Min(30),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title(title.clone()))
    .header(
        Row::new(header_texts)
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1),
    );

    f.render_widget(table, table_area);

    // Draw related_metrics picker if open
    if let Some(picker_rect) = picker_area {
        if let Some(finding) = data.findings.get(app.selected_diagnostic) {
            let selected_idx = app.selected_related_metric.unwrap_or(0);
            let picker_title = if is_ja {
                " 関連メトリクス (Enter:ジャンプ  Backspace:閉じる) "
            } else {
                " Related Metrics (Enter:jump  Backspace:close) "
            };
            let items: Vec<Row> = finding
                .related_metrics
                .iter()
                .enumerate()
                .map(|(i, (src, field))| {
                    let text = format!("{}.{}", src, field);
                    let row = Row::new(vec![Cell::from(text)]);
                    if i == selected_idx {
                        row.style(
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                                .bg(Color::DarkGray),
                        )
                    } else {
                        row.style(Style::default().fg(Color::White))
                    }
                })
                .collect();
            let picker = Table::new(items, [Constraint::Min(40)])
                .block(Block::default().borders(Borders::ALL).title(picker_title));
            f.render_widget(picker, picker_rect);
        }
    }
}

fn draw_category_guide(f: &mut Frame, app: &mut App, area: Rect) {
    use crate::education::{Category, get_content};

    let l = app.locale;
    let categories = Category::all();
    let selected = app
        .selected_category
        .min(categories.len().saturating_sub(1));

    // Split: left panel (category list) and right panel (content)
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22), // ~20% for category list
            Constraint::Min(40),    // rest for content
        ])
        .split(area);

    // Left panel: category list
    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let marker = if i == selected { ">" } else { " " };
            let style = if i == selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", marker), Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("[{}] ", cat.icon()),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(cat.name(l), style),
            ]))
        })
        .collect();

    let list_title = if l == crate::i18n::Locale::Ja {
        " カテゴリ "
    } else {
        " Categories "
    };
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(list_title)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(list, panels[0]);

    // Right panel: content for selected category
    let cat = categories[selected];
    let content = get_content(cat, l);

    let mut lines: Vec<Line> = Vec::new();

    // Section helper
    let section_title = |title_en: &str, title_ja: &str| -> Line {
        let t = if l == crate::i18n::Locale::Ja {
            title_ja
        } else {
            title_en
        };
        Line::from(Span::styled(
            format!("  === {} ===", t),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
    };

    // Related sources
    let sources = cat.related_sources();
    let sources_label = if l == crate::i18n::Locale::Ja {
        "関連ソース: "
    } else {
        "Related sources: "
    };
    let mut source_spans = vec![Span::styled(
        format!("  {}", sources_label),
        Style::default().fg(Color::DarkGray),
    )];
    for (i, s) in sources.iter().enumerate() {
        if i > 0 {
            source_spans.push(Span::styled(", ", Style::default().fg(Color::DarkGray)));
        }
        source_spans.push(Span::styled(*s, Style::default().fg(Color::Cyan)));
    }
    lines.push(Line::from(source_spans));
    lines.push(Line::from(""));

    // Overview section
    lines.push(section_title("Overview", "概要"));
    for text_line in content.overview.split('\n') {
        lines.push(Line::from(Span::styled(
            format!("  {}", text_line),
            Style::default().fg(Color::White),
        )));
    }
    lines.push(Line::from(""));

    // Story section
    lines.push(section_title("Story", "ストーリー"));
    for text_line in content.story.split('\n') {
        // Highlight source names in cyan and field names in yellow
        let styled = style_education_line(text_line);
        lines.push(styled);
    }
    lines.push(Line::from(""));

    // Diagnostic flow section
    lines.push(section_title("Diagnostic Flow", "診断フロー"));
    for text_line in content.diagnostic_flow.split('\n') {
        let color = if text_line.starts_with("Step") || text_line.starts_with("ステップ") {
            Color::Green
        } else {
            Color::White
        };
        lines.push(Line::from(Span::styled(
            format!("  {}", text_line),
            Style::default().fg(color),
        )));
    }
    lines.push(Line::from(""));

    // Common issues section
    lines.push(section_title("Common Issues", "よくある問題"));
    for text_line in content.common_issues.split('\n') {
        let color = if text_line.ends_with(':') || text_line.contains("：") {
            Color::Yellow
        } else {
            Color::White
        };
        lines.push(Line::from(Span::styled(
            format!("  {}", text_line),
            Style::default().fg(color),
        )));
    }

    // Compute total line count and visible height for scroll bounds
    let total_lines = lines.len();
    let visible_height = panels[1].height.saturating_sub(2) as usize; // borders top+bottom
    app.category_content_lines = total_lines;
    app.category_visible_height = visible_height;

    // Clamp scroll to valid range
    let max_scroll = total_lines.saturating_sub(visible_height);
    if app.category_scroll > max_scroll {
        app.category_scroll = max_scroll;
    }
    let scroll = app.category_scroll;

    // Scroll position indicator
    let scroll_indicator = if total_lines > visible_height {
        let current_line = scroll + 1;
        format!(" [{}/{}] ", current_line, total_lines)
    } else {
        String::new()
    };

    let title = format!(
        " {} — {} {}",
        if l == crate::i18n::Locale::Ja {
            "カテゴリガイド"
        } else {
            "Category Guide"
        },
        cat.name(l),
        scroll_indicator,
    );
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(p, panels[1]);
}

/// Style a line of education content, highlighting known source names in cyan
/// and known field names in yellow.
fn style_education_line(text: &str) -> Line<'static> {
    // Source names to highlight
    const SOURCES: &[&str] = &[
        "meminfo",
        "vmstat",
        "swaps",
        "buddyinfo",
        "pressure",
        "zoneinfo",
        "slabinfo",
        "pagetypeinfo",
        "stat",
        "loadavg",
        "cpuinfo",
        "schedstat",
        "softirqs",
        "interrupts",
        "net/dev",
        "net/tcp",
        "net/udp",
        "net/unix",
        "net/arp",
        "net/route",
        "net/sockstat",
        "net/snmp",
        "net/netstat",
        "net/wireless",
        "diskstats",
        "df",
        "mounts",
        "partitions",
        "locks",
        "processes",
        "file-nr",
    ];
    // Field names to highlight
    const FIELDS: &[&str] = &[
        "MemTotal",
        "MemFree",
        "MemAvailable",
        "Cached",
        "Buffers",
        "SwapUsed",
        "SwapFree",
        "SwapTotal",
        "SReclaimable",
        "SUnreclaim",
        "pgmajfault",
        "si",
        "so",
        "load1",
        "load5",
        "load15",
        "cpu_user",
        "cpu_system",
        "cpu_idle",
        "cpu_iowait",
        "cpu_steal",
        "context_switches",
        "memory_some_avg10",
        "cpu_some_avg10",
        "rx_errors",
        "rx_drop",
        "tx_errors",
        "tx_drop",
        "TCPRetransSegs",
        "InReceives",
        "OutRequests",
        "InErrors",
        "TIME_WAIT",
        "CLOSE_WAIT",
        "ESTABLISHED",
        "SYN_RECV",
        "RSS",
    ];

    let formatted = format!("  {}", text);

    // Simple approach: check if line contains any known names and highlight them
    // For performance, just do a simple scan
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut remaining = formatted.clone();

    // Try to find and highlight known terms
    let mut found_any = false;
    for &source in SOURCES {
        if remaining.contains(source) {
            found_any = true;
            break;
        }
    }
    if !found_any {
        for &field in FIELDS {
            if remaining.contains(field) {
                found_any = true;
                break;
            }
        }
    }

    if !found_any {
        return Line::from(Span::styled(formatted, Style::default().fg(Color::White)));
    }

    // Build spans by scanning through the string
    while !remaining.is_empty() {
        let mut best_pos = remaining.len();
        let mut best_len = 0;
        let mut best_is_source = false;

        for &source in SOURCES {
            if let Some(pos) = remaining.find(source) {
                if pos < best_pos || (pos == best_pos && source.len() > best_len) {
                    best_pos = pos;
                    best_len = source.len();
                    best_is_source = true;
                }
            }
        }
        for &field in FIELDS {
            if let Some(pos) = remaining.find(field) {
                if pos < best_pos || (pos == best_pos && field.len() > best_len) {
                    best_pos = pos;
                    best_len = field.len();
                    best_is_source = false;
                }
            }
        }

        if best_len == 0 {
            spans.push(Span::styled(
                remaining.clone(),
                Style::default().fg(Color::White),
            ));
            break;
        }

        if best_pos > 0 {
            spans.push(Span::styled(
                remaining[..best_pos].to_string(),
                Style::default().fg(Color::White),
            ));
        }

        let color = if best_is_source {
            Color::Cyan
        } else {
            Color::Yellow
        };
        spans.push(Span::styled(
            remaining[best_pos..best_pos + best_len].to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));

        remaining = remaining[best_pos + best_len..].to_string();
    }

    Line::from(spans)
}

/// Public API for the web server to compute contextual hints.
pub fn get_contextual_hint_for_api(app: &App, source: &str, field: &str) -> Option<String> {
    get_contextual_hint(app, source, field)
}

fn get_contextual_hint(app: &App, source: &str, field: &str) -> Option<String> {
    let snap = &app.current;
    let locale = app.locale;

    let get_bytes = |src: &str, fld: &str| -> Option<u64> {
        snap.entries
            .get(src)?
            .fields
            .iter()
            .find(|f| f.name == fld)
            .and_then(|f| match f.value {
                FieldValue::Bytes(v) => Some(v),
                _ => None,
            })
    };

    let get_float = |src: &str, fld: &str| -> Option<f64> {
        snap.entries
            .get(src)?
            .fields
            .iter()
            .find(|f| f.name == fld)
            .and_then(|f| match f.value {
                FieldValue::Float(v) => Some(v),
                FieldValue::Integer(v) => Some(v as f64),
                _ => None,
            })
    };

    let get_integer = |src: &str, fld: &str| -> Option<i64> {
        snap.entries
            .get(src)?
            .fields
            .iter()
            .find(|f| f.name == fld)
            .and_then(|f| match f.value {
                FieldValue::Integer(v) => Some(v),
                _ => None,
            })
    };

    match (source, field) {
        ("meminfo", "MemAvailable") => {
            let total = get_bytes("meminfo", "MemTotal")?;
            let avail = get_bytes("meminfo", "MemAvailable")?;
            if total > 0 {
                let pct = (avail as f64 / total as f64) * 100.0;
                if pct < 20.0 {
                    return Some(if locale == crate::i18n::Locale::Ja {
                        format!(
                            "現在メモリが圧迫されています ({:.1}%)。Cached と SwapFree を確認してください",
                            pct
                        )
                    } else {
                        format!(
                            "Memory is currently under pressure ({:.1}%). Check Cached and SwapFree",
                            pct
                        )
                    });
                }
            }
            None
        }
        ("loadavg", "load1") => {
            let load1 = get_float("loadavg", "load1")?;
            let cpu_count = get_integer("stat", "cpu_count")
                .or_else(|| get_integer("cpuinfo", "cpu_count"))
                .unwrap_or(1) as f64;
            if cpu_count > 0.0 && load1 > cpu_count {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!(
                        "負荷 {:.1} が CPU 数 ({}) を超過。プロセスがキュー待ち",
                        load1, cpu_count as i64
                    )
                } else {
                    format!(
                        "Load {:.1} exceeds CPU count ({}). Processes are queuing",
                        load1, cpu_count as i64
                    )
                });
            }
            None
        }
        ("stat", "cpu_iowait") => {
            let iowait = get_float("stat", "cpu_iowait")?;
            if iowait > 10.0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!(
                        "I/O 待ち {:.1}% — ディスクボトルネックの可能性。diskstats と pressure を確認",
                        iowait
                    )
                } else {
                    format!(
                        "I/O wait {:.1}% — possible disk bottleneck. Check diskstats and pressure",
                        iowait
                    )
                });
            }
            None
        }
        ("meminfo", "SwapFree") => {
            let total = get_bytes("meminfo", "SwapTotal")?;
            let free = get_bytes("meminfo", "SwapFree")?;
            if total > 0 {
                let pct_free = (free as f64 / total as f64) * 100.0;
                if pct_free < 50.0 {
                    return Some(if locale == crate::i18n::Locale::Ja {
                        format!(
                            "スワップ残量 {:.1}%。メモリ不足によりスワッピングが進行中",
                            pct_free
                        )
                    } else {
                        format!(
                            "Swap {:.1}% remaining. Swapping is active due to memory pressure",
                            pct_free
                        )
                    });
                }
            }
            None
        }
        ("df", "root_use_pct") => {
            let use_pct = get_float("df", "root_use_pct")?;
            if use_pct > 80.0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!("ディスク使用率 {:.1}%。容量不足に注意", use_pct)
                } else {
                    format!("Disk usage {:.1}%. Watch for space exhaustion", use_pct)
                });
            }
            None
        }
        ("file-nr", "fd_usage_pct") => {
            let usage = get_float("file-nr", "fd_usage_pct")?;
            if usage > 50.0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!(
                        "FD 使用率 {:.1}%。枯渇するとプロセスがファイルやソケットを開けな���なります",
                        usage
                    )
                } else {
                    format!(
                        "FD usage {:.1}%. Exhaustion prevents opening files or sockets",
                        usage
                    )
                });
            }
            None
        }
        ("thermal", "max_temp") => {
            let temp = get_float("thermal", "max_temp")?;
            if temp > 70.0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!(
                        "CPU 温度 {:.0}\u{00B0}C。サーマルスロットリングに注意",
                        temp
                    )
                } else {
                    format!(
                        "CPU temperature {:.0}\u{00B0}C. Watch for thermal throttling",
                        temp
                    )
                });
            }
            None
        }
        ("net/tcp", "connections") => {
            // Check for CLOSE_WAIT count
            let entry = snap.entries.get("net/tcp")?;
            let conn_field = entry.fields.iter().find(|f| f.name == "connections")?;
            if let FieldValue::Table(rows) = &conn_field.value {
                let close_wait = rows
                    .iter()
                    .filter(|r| r.len() >= 4 && r[3] == "CLOSE_WAIT")
                    .count();
                if close_wait > 10 {
                    return Some(if locale == crate::i18n::Locale::Ja {
                        format!("CLOSE_WAIT {}件 — ソケットリークの可能性", close_wait)
                    } else {
                        format!(
                            "{} CLOSE_WAIT connections — possible socket leak",
                            close_wait
                        )
                    });
                }
            }
            None
        }
        ("vmstat", "pswpout") => {
            let pswpout = get_integer("vmstat", "pswpout")?;
            if pswpout > 0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!(
                        "スワップアウト活動中 ({} ページ)。メモリ圧力が存在",
                        pswpout
                    )
                } else {
                    format!(
                        "Active swap-out ({} pages). Memory pressure exists",
                        pswpout
                    )
                });
            }
            None
        }
        ("vmstat", "oom_kill") => {
            let oom = get_integer("vmstat", "oom_kill")?;
            if oom > 0 {
                return Some(if locale == crate::i18n::Locale::Ja {
                    format!("OOM Killer が {}回発動。過去にメモリ枯渇が発生", oom)
                } else {
                    format!(
                        "OOM Killer invoked {} time(s). Past memory exhaustion occurred",
                        oom
                    )
                });
            }
            None
        }
        _ => None,
    }
}

fn draw_help_panel(f: &mut Frame, app: &mut App, area: Rect) {
    use super::app::HelpLevel;

    let source_name = app.current_source_name();
    let source_desc = i18n::source_description(app.locale, source_name);

    // Get field name and i18n description override
    let field_info = app
        .current_entry_fields()
        .and_then(|fields| fields.get(app.selected_field))
        .map(|f| (f.name.clone(), f.description.clone()));

    let (field_name, fallback_desc) =
        field_info.unwrap_or_else(|| ("".to_string(), "".to_string()));

    // Look up i18n field description (normal, detailed, extra)
    let i18n_desc = i18n::field_description(app.locale, source_name, &field_name);

    let mut lines = vec![Line::from(vec![
        Span::styled(
            format!(" {} ", source_name),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("— {}", source_desc),
            Style::default().fg(Color::White),
        ),
    ])];

    if !field_name.is_empty() {
        let (desc_text, is_fallback) = match (app.help_level, i18n_desc) {
            (HelpLevel::Normal, Some((normal, _, _))) => (normal.to_string(), false),
            (HelpLevel::Detailed, Some((_, detailed, _))) => (detailed.to_string(), false),
            (HelpLevel::ExtraDetailed, Some((_, _, extra))) => (extra.to_string(), false),
            _ => (fallback_desc.clone(), !fallback_desc.is_empty()),
        };

        if is_fallback && app.locale == crate::i18n::Locale::Ja && !desc_text.is_empty() {
            lines.push(Line::from(Span::styled(
                "  (翻訳準備中 / translation pending)",
                Style::default().fg(Color::DarkGray),
            )));
        }

        lines.push(Line::from(vec![Span::styled(
            format!("  {} ", field_name),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]));

        // Split multi-line descriptions
        for desc_line in desc_text.split('\n') {
            let style = if desc_line.starts_with("💡") || desc_line.starts_with("  •") {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            lines.push(Line::from(Span::styled(format!("  {}", desc_line), style)));
        }

        // Contextual hint based on current value
        if matches!(
            app.help_level,
            HelpLevel::Detailed | HelpLevel::ExtraDetailed
        ) {
            let contextual_hint = get_contextual_hint(app, source_name, &field_name);
            if let Some(hint) = contextual_hint {
                lines.push(Line::from(Span::styled(
                    format!("  \u{26A0} {}", hint),
                    Style::default().fg(Color::Yellow),
                )));
            }
        }

        // SEE ALSO section for Detailed and ExtraDetailed
        if matches!(
            app.help_level,
            HelpLevel::Detailed | HelpLevel::ExtraDetailed
        ) {
            let related = i18n::see_also(app.locale, source_name, &field_name);
            if !related.is_empty() {
                lines.push(Line::from(Span::raw("")));
                lines.push(Line::from(Span::styled(
                    "  SEE ALSO:",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
                for (src, fld, reason) in &related {
                    lines.push(Line::from(Span::styled(
                        format!("   \u{2192} {}/{} ({})", src, fld, reason),
                        Style::default().fg(Color::Cyan),
                    )));
                }
            }
        }

        // Learning Breadcrumbs for ExtraDetailed only
        if matches!(app.help_level, HelpLevel::ExtraDetailed) {
            let crumbs = crate::education::breadcrumbs(app.locale, source_name, &field_name);
            if !crumbs.is_empty() {
                let header = if app.locale == crate::i18n::Locale::Ja {
                    "  \u{1F4DA} 次に学ぶ:"
                } else {
                    "  \u{1F4DA} Learn next:"
                };
                lines.push(Line::from(Span::raw("")));
                lines.push(Line::from(Span::styled(
                    header,
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                )));
                for (i, (src, fld, reason)) in crumbs.iter().enumerate() {
                    lines.push(Line::from(Span::styled(
                        format!("   {}. {}/{} — {}", i + 1, src, fld, reason),
                        Style::default().fg(Color::Magenta),
                    )));
                }
            }
        }
    }

    // Store metadata for scroll bounds
    let total_lines = lines.len();
    let visible_height = area.height.saturating_sub(2) as usize;
    app.help_content_lines = total_lines;
    app.help_visible_height = visible_height;

    // Clamp scroll
    let max_scroll = total_lines.saturating_sub(visible_height);
    if app.help_scroll > max_scroll {
        app.help_scroll = max_scroll;
    }
    let scroll = app.help_scroll;

    let level_label = app.help_level.label();
    let scroll_indicator = if total_lines > visible_height {
        format!(" [{}/{}]", scroll + 1, total_lines)
    } else {
        String::new()
    };
    let title = format!(
        " {} [{}]{} (? cycle) ",
        i18n::t(app.locale, T::HELP),
        level_label,
        scroll_indicator
    );
    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(p, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;

    // If searching, show search input instead of normal status bar
    if app.searching {
        let label = if l == crate::i18n::Locale::Ja {
            " 検索: "
        } else {
            " Search: "
        };
        let search_line = Line::from(vec![
            Span::styled(
                label,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &app.search_query,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(Color::Cyan)), // cursor
            Span::styled(
                if l == crate::i18n::Locale::Ja {
                    "  (Enter で確定, Esc でキャンセル)"
                } else {
                    "  (Enter to apply, Esc to cancel)"
                },
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        let p = Paragraph::new(search_line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(p, area);
        return;
    }

    let refresh_indicator = if app.auto_refresh { "AUTO" } else { "MANUAL" };
    let elapsed = app.last_refresh.elapsed().as_secs();
    let snapshot_count = app.snapshots.len();

    let view_name = match app.view {
        View::Dashboard => {
            if l == crate::i18n::Locale::Ja {
                "ダッシュボード"
            } else {
                "DASHBOARD"
            }
        }
        View::Welcome => {
            if l == crate::i18n::Locale::Ja {
                "ようこそ"
            } else {
                "WELCOME"
            }
        }
        View::Diagnostics => {
            if l == crate::i18n::Locale::Ja {
                "診断"
            } else {
                "DIAGNOSTICS"
            }
        }
        View::Overview => i18n::t(l, T::VIEW_OVERVIEW),
        View::Detail => i18n::t(l, T::VIEW_DETAIL),
        View::Diff => i18n::t(l, T::VIEW_DIFF),
        View::TableView => i18n::t(l, T::VIEW_TABLE),
        View::Graph => i18n::t(l, T::VIEW_GRAPH),
        View::CategoryGuide => {
            if l == crate::i18n::Locale::Ja {
                "カテゴリガイド"
            } else {
                "CATEGORY GUIDE"
            }
        }
        View::Tutorial => {
            if l == crate::i18n::Locale::Ja {
                "チュートリアル"
            } else {
                "TUTORIAL"
            }
        }
        View::ProcessDetail => {
            if l == crate::i18n::Locale::Ja {
                "プロセス詳細"
            } else {
                "PROCESS DETAIL"
            }
        }
    };

    let axis_state = if app.dash_zero_axis {
        i18n::t(l, T::AXIS_ZERO)
    } else {
        i18n::t(l, T::AXIS_AUTO)
    };
    let status = Line::from(vec![
        Span::styled(" j/k ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::SOURCE))),
        Span::styled("Enter ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::DRILL_IN))),
        Span::styled("BS ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::BACK))),
        Span::styled("d ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::DIFF))),
        Span::styled("/ ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::SEARCH))),
        Span::styled("s ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::AXIS))),
        Span::styled("A ", Style::default().fg(Color::Yellow)),
        Span::raw("ARTICLE  "),
        Span::styled("? ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::HELP))),
        Span::styled("L ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::LANG))),
        Span::styled("q ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::QUIT))),
        Span::styled(
            if let Some(ref host) = app.remote_host {
                let conn_label = match &app.connection_status {
                    super::app::ConnectionStatus::Connected { last_seen } => {
                        let secs_ago = last_seen.elapsed().as_secs();
                        format!("Connected ({}s ago)", secs_ago)
                    }
                    super::app::ConnectionStatus::Disconnected {
                        last_seen,
                        since: _,
                    } => {
                        let secs_ago = last_seen.elapsed().as_secs();
                        format!("DISCONNECTED (last: {}s ago)", secs_ago)
                    }
                    super::app::ConnectionStatus::Connecting => "Connecting...".to_string(),
                    super::app::ConnectionStatus::Local => "local".to_string(),
                };
                format!(
                    " {} | {}{} | {} {} | [{}] {} ",
                    refresh_indicator,
                    elapsed,
                    i18n::t(l, T::AGO),
                    snapshot_count,
                    i18n::t(l, T::SNAPS),
                    host,
                    conn_label
                )
            } else {
                format!(
                    " {} | {}{} | {} {} ",
                    refresh_indicator,
                    elapsed,
                    i18n::t(l, T::AGO),
                    snapshot_count,
                    i18n::t(l, T::SNAPS)
                )
            },
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!(" [{}] [{}] ", view_name, l.name()),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {}: {} ", i18n::t(l, T::AXIS), axis_state),
            Style::default().fg(Color::Magenta),
        ),
    ]);

    // Show alert counts in status bar
    let (info_count, warn_count, crit_count) = alert::count_by_severity(&app.active_alerts);
    let status = {
        let mut spans = status.spans;
        if warn_count > 0 {
            spans.push(Span::styled(
                format!(" [!{} WARN]", warn_count),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        if crit_count > 0 {
            spans.push(Span::styled(
                format!(" [!!{} CRIT]", crit_count),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ));
        }
        if info_count > 0 {
            spans.push(Span::styled(
                format!(" [i{} INFO]", info_count),
                Style::default().fg(Color::Cyan),
            ));
        }
        Line::from(spans)
    };

    // Show diff target in status bar when in Diff view
    let status = if matches!(app.view, View::Diff) && app.diff_target_index.is_some() {
        let mut spans = status.spans;
        let offset = app.diff_offset();
        spans.push(Span::styled(
            format!(" T-{}", offset),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        Line::from(spans)
    } else {
        status
    };

    // Show status message if present
    let status = if let Some(ref msg) = app.status_message {
        let mut spans = status.spans;
        spans.push(Span::styled(
            format!(" | {} ", msg),
            Style::default().fg(Color::Green),
        ));
        Line::from(spans)
    } else {
        status
    };

    // Show pin count / filter state
    let status = if !app.pins.is_empty() {
        let mut spans = status.spans;
        let filter_tag = if app.pin_filter {
            if l == crate::i18n::Locale::Ja {
                "PIN"
            } else {
                "PIN"
            }
        } else {
            ""
        };
        let label = if l == crate::i18n::Locale::Ja {
            format!(" ピン:{}", app.pins.len())
        } else {
            format!(" pins:{}", app.pins.len())
        };
        let style = if app.pin_filter {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Magenta)
        };
        spans.push(Span::styled(format!(" [{}{}]", label, filter_tag), style));
        Line::from(spans)
    } else {
        status
    };

    let p = Paragraph::new(status).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
