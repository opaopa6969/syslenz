use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table},
    Frame,
};

use crate::i18n::{self, T};
use crate::proc::FieldValue;
use super::app::{App, View};
use super::graph;

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
    let outer_constraints = if help_height > 0 {
        vec![
            Constraint::Min(10),
            Constraint::Length(help_height),
            Constraint::Length(3),
        ]
    } else {
        vec![
            Constraint::Min(10),
            Constraint::Length(3),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(outer_constraints)
        .split(f.area());

    // Dashboard and Welcome are full-width (no sidebar)
    let is_fullwidth = matches!(app.view, View::Dashboard | View::Welcome | View::Diagnostics | View::CategoryGuide);

    if is_fullwidth {
        match app.view {
            View::Dashboard => draw_dashboard(f, app, chunks[0]),
            View::Welcome => draw_welcome(f, app, chunks[0]),
            View::Diagnostics => draw_diagnostics(f, app, chunks[0]),
            View::CategoryGuide => draw_category_guide(f, app, chunks[0]),
            _ => unreachable!(),
        }
    } else {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(22), // sidebar
                Constraint::Min(40),   // content
            ])
            .split(chunks[0]);

        draw_sidebar(f, app, main_chunks[0]);

        match app.view {
            View::Overview | View::Detail => draw_detail(f, app, main_chunks[1]),
            View::Diff => draw_diff(f, app, main_chunks[1]),
            View::TableView => draw_table_view(f, app, main_chunks[1]),
            View::Graph => graph::draw_graph(f, app, main_chunks[1]),
            _ => {}
        }
    }

    if app.help_level != super::app::HelpLevel::Off {
        draw_help_panel(f, app, chunks[1]);
        draw_status_bar(f, app, chunks[2]);
    } else {
        draw_status_bar(f, app, chunks[1]);
    }
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let visible_height = area.height.saturating_sub(2) as usize; // border top+bottom
    let selected = app.selected_source;

    // Scroll sidebar so selected item is always visible
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
            let style = if i == selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if key.starts_with("net/") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", marker), Style::default().fg(Color::Yellow)),
                Span::styled(format!("{:<18}", key), style),
            ]))
        })
        .collect();

    let title = format!(" /proc ({}) ", app.source_keys.len());
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(list, area);
}

fn draw_detail(f: &mut Frame, app: &App, area: Rect) {
    let source = app.current_source_name();
    let source_path = app
        .current
        .entries
        .get(source)
        .map(|e| e.source.as_str())
        .unwrap_or("");
    let title = format!(" {} — {} ", source, source_path);

    let Some(fields) = app.current_entry_fields() else {
        let p = Paragraph::new("No data")
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(p, area);
        return;
    };

    let visible_height = area.height.saturating_sub(5) as usize; // borders + header + margin

    // Scroll field list so selected field is visible
    let field_scroll = if app.selected_field >= app.field_scroll + visible_height {
        app.selected_field - visible_height + 1
    } else if app.selected_field < app.field_scroll {
        app.selected_field
    } else {
        app.field_scroll
    };

    let rows: Vec<Row> = fields
        .iter()
        .enumerate()
        .skip(field_scroll)
        .take(visible_height)
        .map(|(i, field)| {
            let is_selected = i == app.selected_field;
            let style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let value_color = match &field.value {
                FieldValue::Bytes(_) => Color::Green,
                FieldValue::Integer(_) => Color::Blue,
                FieldValue::Float(_) => Color::Magenta,
                FieldValue::Duration(_) => Color::Yellow,
                FieldValue::Text(_) => Color::White,
                FieldValue::Table(_) => Color::Cyan,
            };

            // Show drill-in indicator for table fields
            let name_display = if matches!(field.value, FieldValue::Table(_)) {
                if app.locale == crate::i18n::Locale::Ja {
                    format!("{} [Enter で展開]", field.name)
                } else {
                    format!("{} [Enter to expand]", field.name)
                }
            } else {
                field.name.clone()
            };

            Row::new(vec![
                Cell::from(name_display).style(style),
                Cell::from(field.value.display()).style(Style::default().fg(value_color)),
                Cell::from(field.unit.clone().unwrap_or_default())
                    .style(Style::default().fg(Color::DarkGray)),
                Cell::from(field.description.clone())
                    .style(Style::default().fg(Color::DarkGray)),
            ])
        })
        .collect();

    let field_count = fields.len();
    let pos_indicator = format!(
        " {} — {} [{}/{}] ",
        source, source_path,
        app.selected_field + 1,
        field_count
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
            i18n::t(app.locale, T::FIELD),
            i18n::t(app.locale, T::VALUE),
            i18n::t(app.locale, T::UNIT),
            i18n::t(app.locale, T::DESCRIPTION),
        ])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    );

    f.render_widget(table, area);
}

fn draw_diff(f: &mut Frame, app: &App, area: Rect) {
    if app.diffs.is_empty() {
        let p = Paragraph::new(i18n::t(app.locale, T::NO_CHANGES))
            .block(Block::default().borders(Borders::ALL).title(format!(" {} ", i18n::t(app.locale, T::DIFF))))
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

    let title = format!(" Diff ({} changes) ", app.diffs.len());
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
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    );

    f.render_widget(table, area);
}

fn draw_table_view(f: &mut Frame, app: &App, area: Rect) {
    let Some(fields) = app.current_entry_fields() else { return };

    // Find the table field (prefer the one at selected_field, else first table)
    let table_field = fields
        .get(app.selected_field)
        .filter(|f| matches!(f.value, FieldValue::Table(_)))
        .or_else(|| fields.iter().find(|f| matches!(f.value, FieldValue::Table(_))));

    let Some(field) = table_field else {
        let p = Paragraph::new(" No table data in this source")
            .block(Block::default().borders(Borders::ALL).title(" Table View "));
        f.render_widget(p, area);
        return;
    };

    if let FieldValue::Table(ref data) = field.value {
        let header_titles: Vec<&str> = match app.current_source_name() {
            "mounts" => vec!["Device", "Mountpoint", "FSType", "Options"],
            "partitions" => vec!["Name", "Size", "Major", "Minor"],
            "net/dev" => vec!["Interface", "RX Bytes", "RX Pkts", "TX Bytes", "TX Pkts"],
            "diskstats" => vec!["Device", "Reads", "Read Bytes", "Writes", "Written", "InFlight"],
            "processes" => vec!["PID", "Name", "State", "RSS", "Threads", "UID"],
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
                (0..ncols).map(|i| match i {
                    0 => "Col1", 1 => "Col2", 2 => "Col3", 3 => "Col4",
                    4 => "Col5", 5 => "Col6", 6 => "Col7", _ => "Col",
                }).collect()
            }
        };

        let visible_rows = (area.height as usize).saturating_sub(5);
        let start = app.table_scroll.min(data.len().saturating_sub(visible_rows));

        let rows: Vec<Row> = data
            .iter()
            .skip(start)
            .take(visible_rows)
            .map(|row| {
                let cells: Vec<Cell> = row.iter().map(|c| Cell::from(c.as_str())).collect();
                Row::new(cells)
            })
            .collect();

        let ncols = header_titles.len().max(1);
        let pct = (100 / ncols) as u16;
        let widths: Vec<Constraint> = header_titles
            .iter()
            .map(|_| Constraint::Percentage(pct))
            .collect();

        let title = format!(
            " {} / {} ({} rows, {}-{}) ",
            app.current_source_name(),
            field.name,
            data.len(),
            start + 1,
            (start + visible_rows).min(data.len()),
        );
        let table = Table::new(rows, widths)
            .block(Block::default().borders(Borders::ALL).title(title))
            .header(
                Row::new(header_titles)
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .bottom_margin(1),
            );

        f.render_widget(table, area);
    }
}

fn draw_welcome(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;
    let title = if l == crate::i18n::Locale::Ja {
        " syslenz — /proc の全てを構造化データで "
    } else {
        " syslenz — Wireshark for /proc "
    };

    let (keys_text, _footer) = if l == crate::i18n::Locale::Ja {
        (vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  D       ", Style::default().fg(Color::Yellow)),
                Span::raw("ダッシュボード（システム概要）"),
            ]),
            Line::from(vec![
                Span::styled("  O       ", Style::default().fg(Color::Yellow)),
                Span::raw("クラシックモード（全ソース一覧）"),
            ]),
            Line::from(vec![
                Span::styled("  j/k ↑/↓ ", Style::default().fg(Color::Yellow)),
                Span::raw("ソース / フィールド移動"),
            ]),
            Line::from(vec![
                Span::styled("  Enter → ", Style::default().fg(Color::Yellow)),
                Span::raw("ドリルイン（詳細表示）"),
            ]),
            Line::from(vec![
                Span::styled("  BS ←    ", Style::default().fg(Color::Yellow)),
                Span::raw("戻る"),
            ]),
            Line::from(vec![
                Span::styled("  d       ", Style::default().fg(Color::Yellow)),
                Span::raw("差分ビュー"),
            ]),
            Line::from(vec![
                Span::styled("  g       ", Style::default().fg(Color::Yellow)),
                Span::raw("グラフ（数値フィールドの推移）"),
            ]),
            Line::from(vec![
                Span::styled("  /       ", Style::default().fg(Color::Yellow)),
                Span::raw("ソース検索"),
            ]),
            Line::from(vec![
                Span::styled("  ?       ", Style::default().fg(Color::Yellow)),
                Span::raw("ヘルプパネル（フィールド説明）"),
            ]),
            Line::from(vec![
                Span::styled("  L       ", Style::default().fg(Color::Yellow)),
                Span::raw("言語切り替え (EN/JA)"),
            ]),
            Line::from(vec![
                Span::styled("  e       ", Style::default().fg(Color::Yellow)),
                Span::raw("JSON エクスポート"),
            ]),
            Line::from(vec![
                Span::styled("  q       ", Style::default().fg(Color::Yellow)),
                Span::raw("終了"),
            ]),
            Line::from(""),
        ],
        " D でダッシュボード、O でクラシックモード ")
    } else {
        (vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  D       ", Style::default().fg(Color::Yellow)),
                Span::raw("Dashboard (system overview)"),
            ]),
            Line::from(vec![
                Span::styled("  O       ", Style::default().fg(Color::Yellow)),
                Span::raw("Classic mode (all sources list)"),
            ]),
            Line::from(vec![
                Span::styled("  j/k ↑/↓ ", Style::default().fg(Color::Yellow)),
                Span::raw("Navigate sources / fields"),
            ]),
            Line::from(vec![
                Span::styled("  Enter → ", Style::default().fg(Color::Yellow)),
                Span::raw("Drill in (detail view)"),
            ]),
            Line::from(vec![
                Span::styled("  BS ←    ", Style::default().fg(Color::Yellow)),
                Span::raw("Go back"),
            ]),
            Line::from(vec![
                Span::styled("  d       ", Style::default().fg(Color::Yellow)),
                Span::raw("Diff view"),
            ]),
            Line::from(vec![
                Span::styled("  g       ", Style::default().fg(Color::Yellow)),
                Span::raw("Graph (sparkline of numeric field)"),
            ]),
            Line::from(vec![
                Span::styled("  /       ", Style::default().fg(Color::Yellow)),
                Span::raw("Search sources"),
            ]),
            Line::from(vec![
                Span::styled("  ?       ", Style::default().fg(Color::Yellow)),
                Span::raw("Help panel (field descriptions)"),
            ]),
            Line::from(vec![
                Span::styled("  L       ", Style::default().fg(Color::Yellow)),
                Span::raw("Toggle language (EN/JA)"),
            ]),
            Line::from(vec![
                Span::styled("  e       ", Style::default().fg(Color::Yellow)),
                Span::raw("Export snapshot as JSON"),
            ]),
            Line::from(vec![
                Span::styled("  q       ", Style::default().fg(Color::Yellow)),
                Span::raw("Quit"),
            ]),
            Line::from(""),
        ],
        " Press D for Dashboard, O for Classic mode ")
    };

    let p = Paragraph::new(keys_text)
        .block(Block::default().borders(Borders::ALL).title(title)
            .border_style(Style::default().fg(Color::Cyan)))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(p, area);
}

fn get_field_value(app: &App, source: &str, field_name: &str) -> String {
    app.current.entries.get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field_name))
        .map(|f| f.value.display())
        .unwrap_or_else(|| "-".to_string())
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;
    let _title = if l == crate::i18n::Locale::Ja {
        " ダッシュボード — システム概要 "
    } else {
        " Dashboard — System Overview "
    };

    // Split into sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // loadavg + uptime
            Constraint::Length(5),  // meminfo (bar graphs)
            Constraint::Length(5),  // CPU stat (bar graphs)
            Constraint::Length(9),  // net/dev
            Constraint::Length(3),  // disk + temp + fd
            Constraint::Min(4),    // sparkline graphs
        ])
        .split(area);

    // Section 0: Load Average + Uptime (single row)
    let load1 = get_field_value(app, "loadavg", "load_1min");
    let load5 = get_field_value(app, "loadavg", "load_5min");
    let load15 = get_field_value(app, "loadavg", "load_15min");
    let uptime_val = get_field_value(app, "uptime", "uptime");

    let sec0_style = section_style(app.selected_dashboard_section == 0);
    let load_line = Line::from(vec![
        Span::styled(" Load: ", Style::default().fg(Color::Yellow)),
        Span::styled(&load1, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw(" / "),
        Span::styled(&load5, Style::default().fg(Color::Yellow)),
        Span::raw(" / "),
        Span::styled(&load15, Style::default().fg(Color::DarkGray)),
        Span::raw("    "),
        Span::styled(" Up: ", Style::default().fg(Color::Yellow)),
        Span::styled(&uptime_val, Style::default().fg(Color::Cyan)),
    ]);
    let p = Paragraph::new(load_line)
        .block(Block::default().borders(Borders::ALL)
            .title(if l == crate::i18n::Locale::Ja { " 負荷 / 稼働時間 " } else { " Load / Uptime " })
            .border_style(sec0_style));
    f.render_widget(p, sections[0]);

    // Section 1: Memory with usage bar
    let sec1_style = section_style(app.selected_dashboard_section == 1);
    let mem_total = get_bytes_value(app, "meminfo", "MemTotal");
    let mem_available = get_bytes_value(app, "meminfo", "MemAvailable");
    let mem_used = mem_total.saturating_sub(mem_available);
    let mem_pct = if mem_total > 0 { (mem_used as f64 / mem_total as f64 * 100.0) as u64 } else { 0 };
    let swap_total = get_bytes_value(app, "meminfo", "SwapTotal");
    let swap_free = get_bytes_value(app, "meminfo", "SwapFree");
    let swap_used = swap_total.saturating_sub(swap_free);
    let swap_pct = if swap_total > 0 { (swap_used as f64 / swap_total as f64 * 100.0) as u64 } else { 0 };

    let mem_bar = make_bar(mem_pct, 30);
    let swap_bar = make_bar(swap_pct, 30);
    let cached_val = get_field_value(app, "meminfo", "Cached");
    let buffers_val = get_field_value(app, "meminfo", "Buffers");

    let mem_lines = vec![
        Line::from(vec![
            Span::styled(" RAM  ", Style::default().fg(Color::Yellow)),
            Span::styled(&mem_bar, bar_color(mem_pct)),
            Span::styled(format!(" {}%", mem_pct), bar_color(mem_pct)),
            Span::styled(format!("  {} / {}", format_bytes_short(mem_used), format_bytes_short(mem_total)), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(" Swap ", Style::default().fg(Color::Yellow)),
            Span::styled(&swap_bar, bar_color(swap_pct)),
            Span::styled(format!(" {}%", swap_pct), bar_color(swap_pct)),
            Span::styled(format!("  {} / {}", format_bytes_short(swap_used), format_bytes_short(swap_total)), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled(" Cache: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&cached_val),
            Span::styled("  Buf: ", Style::default().fg(Color::DarkGray)),
            Span::raw(&buffers_val),
        ]),
    ];
    let mem_p = Paragraph::new(mem_lines)
        .block(Block::default().borders(Borders::ALL)
            .title(if l == crate::i18n::Locale::Ja { " メモリ " } else { " Memory " })
            .border_style(sec1_style));
    f.render_widget(mem_p, sections[1]);

    // Section 2: CPU stat with usage bars
    let sec2_style = section_style(app.selected_dashboard_section == 2);
    let cpu_user = get_float_value(app, "stat", "cpu_usage_pct").unwrap_or(0.0) as u64;
    let cpu_user_raw = get_field_value(app, "stat", "cpu_user");
    let cpu_sys_raw = get_field_value(app, "stat", "cpu_system");
    let cpu_idle_raw = get_field_value(app, "stat", "cpu_idle");
    let cpu_iowait_raw = get_field_value(app, "stat", "cpu_iowait");
    let ctx_switches = get_field_value(app, "stat", "context_switches");
    let procs_running = get_field_value(app, "stat", "processes_running");

    // Calculate rough percentages from cumulative counters for bar display
    let cpu_u: f64 = cpu_user_raw.parse().unwrap_or(0.0);
    let cpu_s: f64 = cpu_sys_raw.parse().unwrap_or(0.0);
    let cpu_i: f64 = cpu_idle_raw.parse().unwrap_or(1.0);
    let cpu_w: f64 = cpu_iowait_raw.parse().unwrap_or(0.0);
    let cpu_total = cpu_u + cpu_s + cpu_i + cpu_w;
    let user_pct = if cpu_total > 0.0 { (cpu_u / cpu_total * 100.0) as u64 } else { 0 };
    let sys_pct = if cpu_total > 0.0 { (cpu_s / cpu_total * 100.0) as u64 } else { 0 };
    let io_pct = if cpu_total > 0.0 { (cpu_w / cpu_total * 100.0) as u64 } else { 0 };
    let used_pct = user_pct + sys_pct;

    let cpu_bar = make_bar(used_pct.min(100), 30);

    let cpu_lines = vec![
        Line::from(vec![
            Span::styled(" CPU  ", Style::default().fg(Color::Yellow)),
            Span::styled(&cpu_bar, bar_color(used_pct)),
            Span::styled(format!(" {}%", used_pct), bar_color(used_pct)),
        ]),
        Line::from(vec![
            Span::styled("  usr:", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}%", user_pct), Style::default().fg(Color::Blue)),
            Span::styled("  sys:", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}%", sys_pct), Style::default().fg(Color::Magenta)),
            Span::styled("  iow:", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}%", io_pct), Style::default().fg(if io_pct > 10 { Color::Red } else { Color::DarkGray })),
        ]),
        Line::from(vec![
            Span::styled("  ctx:", Style::default().fg(Color::DarkGray)),
            Span::raw(&ctx_switches),
            Span::styled("  run:", Style::default().fg(Color::DarkGray)),
            Span::raw(&procs_running),
        ]),
    ];
    let cpu_p = Paragraph::new(cpu_lines)
        .block(Block::default().borders(Borders::ALL)
            .title(if l == crate::i18n::Locale::Ja { " CPU " } else { " CPU " })
            .border_style(sec2_style));
    f.render_widget(cpu_p, sections[2]);

    // Section 3: Network (net/dev table)
    let sec3_style = section_style(app.selected_dashboard_section == 3);
    let net_rows: Vec<Row> = app.current.entries.get("net/dev")
        .and_then(|e| e.fields.iter().find(|f| matches!(f.value, FieldValue::Table(_))))
        .map(|f| {
            if let FieldValue::Table(ref data) = f.value {
                data.iter().take(6).map(|row| {
                    let cells: Vec<Cell> = row.iter().map(|c| Cell::from(c.as_str())).collect();
                    Row::new(cells)
                }).collect()
            } else {
                vec![]
            }
        })
        .unwrap_or_default();

    let net_headers = if l == crate::i18n::Locale::Ja {
        vec!["IF", "RX バイト", "RX パケット", "TX バイト", "TX パケット"]
    } else {
        vec!["Interface", "RX Bytes", "RX Pkts", "TX Bytes", "TX Pkts"]
    };
    let net_table = Table::new(net_rows, vec![
        Constraint::Length(12), Constraint::Length(14), Constraint::Length(12),
        Constraint::Length(14), Constraint::Min(12),
    ])
    .block(Block::default().borders(Borders::ALL)
        .title(if l == crate::i18n::Locale::Ja { " ネットワーク " } else { " Network " })
        .border_style(sec3_style))
    .header(Row::new(net_headers)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .bottom_margin(1));
    f.render_widget(net_table, sections[3]);

    // Section 4: Disk + Temperature + FD (summary line)
    let sec4_style = section_style(app.selected_dashboard_section == 4);
    let disk_pct = get_field_value(app, "df", "root_use_pct");
    let temp = get_field_value(app, "thermal", "max_temp");
    let fd_pct = get_field_value(app, "file-nr", "fd_usage_pct");

    let sys_line = Line::from(vec![
        Span::styled(" Disk: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{}%", disk_pct),
            Style::default().fg(if disk_pct.parse::<f64>().unwrap_or(0.0) > 80.0 { Color::Red } else { Color::Green }),
        ),
        Span::raw("    "),
        Span::styled(" Temp: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            if temp == "-" { "N/A".to_string() } else { format!("{}C", temp) },
            Style::default().fg(if temp.parse::<f64>().unwrap_or(0.0) > 75.0 { Color::Red } else { Color::Cyan }),
        ),
        Span::raw("    "),
        Span::styled(" FD: ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{}%", fd_pct),
            Style::default().fg(if fd_pct.parse::<f64>().unwrap_or(0.0) > 80.0 { Color::Red } else { Color::Green }),
        ),
    ]);
    let p = Paragraph::new(sys_line)
        .block(Block::default().borders(Borders::ALL)
            .title(if l == crate::i18n::Locale::Ja { " ディスク / 温度 / FD " } else { " Disk / Temp / FD " })
            .border_style(sec4_style));
    f.render_widget(p, sections[4]);

    // Section 5: Sparkline graphs (load + memory history)
    if sections.len() > 5 && sections[5].height >= 4 {
        let graph_area = sections[5];
        let graph_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(graph_area);

        // Load history sparkline
        let load_data: Vec<u64> = app.snapshots.iter()
            .chain(std::iter::once(&app.current))
            .filter_map(|snap| {
                snap.entries.get("loadavg")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "load_1min"))
                    .and_then(|f| match f.value {
                        FieldValue::Float(v) => Some((v * 100.0) as u64),
                        _ => None,
                    })
            })
            .collect();

        let load_title = if l == crate::i18n::Locale::Ja {
            format!(" 負荷 ({}件) ", load_data.len())
        } else {
            format!(" Load ({} pts) ", load_data.len())
        };
        let load_sparkline = ratatui::widgets::Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title(load_title)
                .border_style(Style::default().fg(Color::DarkGray)))
            .data(&load_data)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(load_sparkline, graph_chunks[0]);

        // Memory usage % history sparkline
        let mem_data: Vec<u64> = app.snapshots.iter()
            .chain(std::iter::once(&app.current))
            .filter_map(|snap| {
                let total = snap.entries.get("meminfo")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "MemTotal"))
                    .and_then(|f| match f.value { FieldValue::Bytes(v) => Some(v), _ => None })
                    .unwrap_or(1);
                let avail = snap.entries.get("meminfo")
                    .and_then(|e| e.fields.iter().find(|f| f.name == "MemAvailable"))
                    .and_then(|f| match f.value { FieldValue::Bytes(v) => Some(v), _ => None })
                    .unwrap_or(0);
                let used_pct = if total > 0 { ((total - avail) as f64 / total as f64 * 100.0) as u64 } else { 0 };
                Some(used_pct)
            })
            .collect();

        let mem_title = if l == crate::i18n::Locale::Ja {
            format!(" メモリ使用率 ({}件) ", mem_data.len())
        } else {
            format!(" Memory % ({} pts) ", mem_data.len())
        };
        let mem_sparkline = ratatui::widgets::Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title(mem_title)
                .border_style(Style::default().fg(Color::DarkGray)))
            .data(&mem_data)
            .style(Style::default().fg(Color::Green));
        f.render_widget(mem_sparkline, graph_chunks[1]);
    }
}

fn make_bar(pct: u64, width: usize) -> String {
    let filled = (pct as usize * width / 100).min(width);
    let empty = width - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

fn bar_color(pct: u64) -> Style {
    if pct > 90 { Style::default().fg(Color::Red).add_modifier(Modifier::BOLD) }
    else if pct > 70 { Style::default().fg(Color::Yellow) }
    else { Style::default().fg(Color::Green) }
}

fn get_bytes_value(app: &App, source: &str, field: &str) -> u64 {
    app.current.entries.get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field))
        .and_then(|f| match f.value {
            FieldValue::Bytes(v) => Some(v),
            _ => None,
        })
        .unwrap_or(0)
}

fn get_float_value(app: &App, source: &str, field: &str) -> Option<f64> {
    app.current.entries.get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field))
        .and_then(|f| match f.value {
            FieldValue::Float(v) => Some(v),
            _ => None,
        })
}

fn format_bytes_short(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB { format!("{:.1}G", bytes as f64 / GIB as f64) }
    else if bytes >= MIB { format!("{:.0}M", bytes as f64 / MIB as f64) }
    else { format!("{}K", bytes / 1024) }
}

fn section_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn draw_diagnostics(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;
    let findings = crate::diagnostics::analyze(&app.current, l);

    let title = if l == crate::i18n::Locale::Ja {
        format!(" 診断 — {}件の検出 (c でコピー) ", findings.len())
    } else {
        format!(" Diagnostics — {} findings (c to copy) ", findings.len())
    };

    if findings.is_empty() {
        let msg = if l == crate::i18n::Locale::Ja {
            " ✓ 問題は検出されませんでした"
        } else {
            " ✓ No issues detected"
        };
        let p = Paragraph::new(msg)
            .block(Block::default().borders(Borders::ALL).title(title))
            .style(Style::default().fg(Color::Green));
        f.render_widget(p, area);
        return;
    }

    let rows: Vec<Row> = findings.iter().map(|finding| {
        let sev_style = match finding.severity {
            crate::diagnostics::Severity::Critical => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            crate::diagnostics::Severity::Warning => Style::default().fg(Color::Yellow),
            crate::diagnostics::Severity::Info => Style::default().fg(Color::Cyan),
        };
        Row::new(vec![
            Cell::from(finding.severity.label(l)).style(sev_style),
            Cell::from(finding.source.clone()).style(Style::default().fg(Color::DarkGray)),
            Cell::from(finding.title.clone()).style(sev_style),
            Cell::from(finding.detail.clone()),
            Cell::from(finding.suggestion.clone()).style(Style::default().fg(Color::Green)),
        ]).height(2)
    }).collect();

    let header_texts = if l == crate::i18n::Locale::Ja {
        vec!["重要度", "ソース", "問題", "詳細", "対処法"]
    } else {
        vec!["Sev", "Source", "Issue", "Detail", "Suggestion"]
    };

    let table = Table::new(rows, [
        Constraint::Length(6),
        Constraint::Length(12),
        Constraint::Length(30),
        Constraint::Length(40),
        Constraint::Min(30),
    ])
    .block(Block::default().borders(Borders::ALL).title(title))
    .header(
        Row::new(header_texts)
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    );

    f.render_widget(table, area);
}

fn draw_category_guide(f: &mut Frame, app: &mut App, area: Rect) {
    use crate::education::{Category, get_content};

    let l = app.locale;
    let categories = Category::all();
    let selected = app.selected_category.min(categories.len().saturating_sub(1));

    // Split: left panel (category list) and right panel (content)
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(22),  // ~20% for category list
            Constraint::Min(40),     // rest for content
        ])
        .split(area);

    // Left panel: category list
    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let marker = if i == selected { ">" } else { " " };
            let style = if i == selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", marker), Style::default().fg(Color::Cyan)),
                Span::styled(format!("[{}] ", cat.icon()), Style::default().fg(Color::Yellow)),
                Span::styled(cat.name(l), style),
            ]))
        })
        .collect();

    let list_title = if l == crate::i18n::Locale::Ja {
        " カテゴリ "
    } else {
        " Categories "
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(list_title)
            .border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(list, panels[0]);

    // Right panel: content for selected category
    let cat = categories[selected];
    let content = get_content(cat, l);

    let mut lines: Vec<Line> = Vec::new();

    // Section helper
    let section_title = |title_en: &str, title_ja: &str| -> Line {
        let t = if l == crate::i18n::Locale::Ja { title_ja } else { title_en };
        Line::from(Span::styled(
            format!("  === {} ===", t),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
    };

    // Related sources
    let sources = cat.related_sources();
    let sources_label = if l == crate::i18n::Locale::Ja { "関連ソース: " } else { "Related sources: " };
    let mut source_spans = vec![
        Span::styled(format!("  {}", sources_label), Style::default().fg(Color::DarkGray)),
    ];
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
        if l == crate::i18n::Locale::Ja { "カテゴリガイド" } else { "Category Guide" },
        cat.name(l),
        scroll_indicator,
    );
    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title)
            .border_style(Style::default().fg(Color::Cyan)))
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(p, panels[1]);
}

/// Style a line of education content, highlighting known source names in cyan
/// and known field names in yellow.
fn style_education_line(text: &str) -> Line<'static> {
    // Source names to highlight
    const SOURCES: &[&str] = &[
        "meminfo", "vmstat", "swaps", "buddyinfo", "pressure", "zoneinfo",
        "slabinfo", "pagetypeinfo", "stat", "loadavg", "cpuinfo", "schedstat",
        "softirqs", "interrupts", "net/dev", "net/tcp", "net/udp", "net/unix",
        "net/arp", "net/route", "net/sockstat", "net/snmp", "net/netstat",
        "net/wireless", "diskstats", "df", "mounts", "partitions", "locks",
        "processes", "file-nr",
    ];
    // Field names to highlight
    const FIELDS: &[&str] = &[
        "MemTotal", "MemFree", "MemAvailable", "Cached", "Buffers", "SwapUsed",
        "SwapFree", "SwapTotal", "SReclaimable", "SUnreclaim",
        "pgmajfault", "si", "so",
        "load1", "load5", "load15", "cpu_user", "cpu_system", "cpu_idle",
        "cpu_iowait", "cpu_steal", "context_switches",
        "memory_some_avg10", "cpu_some_avg10",
        "rx_errors", "rx_drop", "tx_errors", "tx_drop",
        "TCPRetransSegs", "InReceives", "OutRequests", "InErrors",
        "TIME_WAIT", "CLOSE_WAIT", "ESTABLISHED", "SYN_RECV",
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
            spans.push(Span::styled(remaining.clone(), Style::default().fg(Color::White)));
            break;
        }

        if best_pos > 0 {
            spans.push(Span::styled(
                remaining[..best_pos].to_string(),
                Style::default().fg(Color::White),
            ));
        }

        let color = if best_is_source { Color::Cyan } else { Color::Yellow };
        spans.push(Span::styled(
            remaining[best_pos..best_pos + best_len].to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ));

        remaining = remaining[best_pos + best_len..].to_string();
    }

    Line::from(spans)
}

fn draw_help_panel(f: &mut Frame, app: &mut App, area: Rect) {
    use super::app::HelpLevel;

    let source_name = app.current_source_name();
    let source_desc = i18n::source_description(app.locale, source_name);

    // Get field name and i18n description override
    let field_info = app.current_entry_fields()
        .and_then(|fields| fields.get(app.selected_field))
        .map(|f| (f.name.clone(), f.description.clone()));

    let (field_name, fallback_desc) = field_info
        .unwrap_or_else(|| ("".to_string(), "".to_string()));

    // Look up i18n field description (normal, detailed, extra)
    let i18n_desc = i18n::field_description(app.locale, source_name, &field_name);

    let mut lines = vec![
        Line::from(vec![
            Span::styled(
                format!(" {} ", source_name),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("— {}", source_desc),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

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

        lines.push(Line::from(vec![
            Span::styled(
                format!("  {} ", field_name),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]));

        // Split multi-line descriptions
        for desc_line in desc_text.split('\n') {
            let style = if desc_line.starts_with("💡") || desc_line.starts_with("  •") {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            lines.push(Line::from(Span::styled(
                format!("  {}", desc_line),
                style,
            )));
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
    let title = format!(" {} [{}]{} (? cycle) ", i18n::t(app.locale, T::HELP), level_label, scroll_indicator);
    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title)
            .border_style(Style::default().fg(Color::DarkGray)))
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(p, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let l = app.locale;

    // If searching, show search input instead of normal status bar
    if app.searching {
        let label = if l == crate::i18n::Locale::Ja { " 検索: " } else { " Search: " };
        let search_line = Line::from(vec![
            Span::styled(label, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                &app.search_query,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(Color::Cyan)), // cursor
            Span::styled(
                if l == crate::i18n::Locale::Ja { "  (Enter で確定, Esc でキャンセル)" } else { "  (Enter to apply, Esc to cancel)" },
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        let p = Paragraph::new(search_line)
            .block(Block::default().borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)));
        f.render_widget(p, area);
        return;
    }

    let refresh_indicator = if app.auto_refresh { "AUTO" } else { "MANUAL" };
    let elapsed = app.last_refresh.elapsed().as_secs();
    let snapshot_count = app.snapshots.len();

    let view_name = match app.view {
        View::Dashboard => if l == crate::i18n::Locale::Ja { "ダッシュボード" } else { "DASHBOARD" },
        View::Welcome => if l == crate::i18n::Locale::Ja { "ようこそ" } else { "WELCOME" },
        View::Diagnostics => if l == crate::i18n::Locale::Ja { "診断" } else { "DIAGNOSTICS" },
        View::Overview => i18n::t(l, T::VIEW_OVERVIEW),
        View::Detail => i18n::t(l, T::VIEW_DETAIL),
        View::Diff => i18n::t(l, T::VIEW_DIFF),
        View::TableView => i18n::t(l, T::VIEW_TABLE),
        View::Graph => i18n::t(l, T::VIEW_GRAPH),
        View::CategoryGuide => if l == crate::i18n::Locale::Ja { "カテゴリガイド" } else { "CATEGORY GUIDE" },
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
        Span::styled("? ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::HELP))),
        Span::styled("L ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::LANG))),
        Span::styled("q ", Style::default().fg(Color::Yellow)),
        Span::raw(format!("{}  ", i18n::t(l, T::QUIT))),
        Span::styled(
            if let Some(ref host) = app.remote_host {
                format!(" {} | {}{} | {} {} | SSH:{} ", refresh_indicator, elapsed, i18n::t(l, T::AGO), snapshot_count, i18n::t(l, T::SNAPS), host)
            } else {
                format!(" {} | {}{} | {} {} ", refresh_indicator, elapsed, i18n::t(l, T::AGO), snapshot_count, i18n::t(l, T::SNAPS))
            },
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            format!(" [{}] [{}] ", view_name, l.name()),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ]);

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

    let p = Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}
