// Temporary: silence clippy's dead_code lint for the body of work that is
// scaffolded-but-not-yet-wired (history export, education paths, sidebar/
// statusbar/help panel builders, etc.). These are planned APIs, not
// abandoned code. Tracked in #24 — replace with per-item `#[expect(dead_code)]`
// (or actually wire them) over time. Crate-level allow keeps CI green while
// the cleanup proceeds incrementally.
//
// Also silence a batch of style lints (collapsible_if, if_same_then_else,
// manual_strip, redundant_closure, ...) that fire on the existing codebase
// but are not safety-critical. Tracked in #24 — fix incrementally.
#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_strip)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::needless_return)]
#![allow(clippy::useless_format)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::manual_map)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::identity_op)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::new_without_default)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::map_clone)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::single_match)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::unnecessary_filter_map)]
#![allow(clippy::unnecessary_get_then_check)]
#![allow(clippy::manual_contains)]
#![allow(clippy::question_mark)]
#![allow(unknown_lints)] // CI clippy may know lints that local clippy doesn't (version skew)

mod alert;
mod article;
mod article_concepts;
mod article_groups;
mod article_metrics;
mod article_metrics_generated;
mod article_sources;
mod common_metric;
mod config;
mod diagnostics;
mod education;
mod export;
mod history;
mod i18n;
mod metric_kind;
mod mcp;
mod net;
mod otel;
mod plugin;
mod proc;
mod prometheus;
mod remote;
mod serve;
mod sys;
mod ui;
#[cfg(feature = "web")]
mod web;
#[cfg(feature = "x11widget")]
mod x11_widget;

use crate::i18n::T;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use ui::app::App;

fn main() -> Result<()> {
    // glibc malloc の arena 数を制限する。
    // デフォルトは 8×CPU コア数で、多数のスレッドがそれぞれ巨大なアロケーションを
    // 繰り返すと、arena ごとにカーネルに返さない断片化メモリが蓄積し RSS が膨張する。
    // （実測: 120 回の capture で RSS 49MB、malloc_trim(0) で 11MB に戻った = 38MB が断片化）
    // MALLOC_ARENA_MAX=2 相当に抑える。
    #[cfg(target_os = "linux")]
    unsafe {
        libc::mallopt(libc::M_ARENA_MAX, 2);
    }

    let args: Vec<String> = std::env::args().collect();

    // --help / -h — print usage and exit 0
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return Ok(());
    }

    // --export <file.json>
    if let Some(pos) = args.iter().position(|a| a == "--export") {
        let path = args
            .get(pos + 1)
            .expect("--export requires a file path argument");
        let snapshot = proc::Snapshot::capture()?;
        export::export_snapshot(&snapshot, Path::new(path))?;
        eprintln!("Snapshot exported to {}", path);
        return Ok(());
    }

    // --export-article-resources <dir>
    if let Some(pos) = args.iter().position(|a| a == "--export-article-resources") {
        let dir = args
            .get(pos + 1)
            .expect("--export-article-resources requires a directory argument");
        std::fs::create_dir_all(dir)?;
        let (index_json, en_json, ja_json) = article::export_split_resources_json()?;
        let index_path = Path::new(dir).join("index.json");
        let en_path = Path::new(dir).join("en.json");
        let ja_path = Path::new(dir).join("ja.json");
        std::fs::write(&index_path, index_json)?;
        std::fs::write(&en_path, en_json)?;
        std::fs::write(&ja_path, ja_json)?;
        eprintln!("Exported article resources:");
        eprintln!("  {}", index_path.display());
        eprintln!("  {}", en_path.display());
        eprintln!("  {}", ja_path.display());
        return Ok(());
    }

    // --export-article-markdown-resources <dir>
    if let Some(pos) = args
        .iter()
        .position(|a| a == "--export-article-markdown-resources")
    {
        let dir = args
            .get(pos + 1)
            .expect("--export-article-markdown-resources requires a directory argument");
        let count = article::export_markdown_resources(Path::new(dir))?;
        eprintln!(
            "Exported markdown article resources to {} ({} articles)",
            dir, count
        );
        return Ok(());
    }

    // --export-series <dir> --interval <secs> --count <n>
    if let Some(pos) = args.iter().position(|a| a == "--export-series") {
        let dir = args
            .get(pos + 1)
            .expect("--export-series requires a directory argument");
        let interval_pos = args
            .iter()
            .position(|a| a == "--interval")
            .expect("--export-series requires --interval <secs>");
        let interval_secs: u64 = args
            .get(interval_pos + 1)
            .expect("--interval requires a value")
            .parse()
            .expect("--interval must be a number");
        let count_pos = args
            .iter()
            .position(|a| a == "--count")
            .expect("--export-series requires --count <n>");
        let count: usize = args
            .get(count_pos + 1)
            .expect("--count requires a value")
            .parse()
            .expect("--count must be a number");

        std::fs::create_dir_all(dir)?;
        for i in 0..count {
            let snapshot = proc::Snapshot::capture()?;
            let ts = snapshot
                .timestamp
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let filename = format!("{}/snapshot_{}.json", dir, ts);
            export::export_snapshot(&snapshot, Path::new(&filename))?;
            eprintln!("[{}/{}] Exported {}", i + 1, count, filename);
            if i + 1 < count {
                std::thread::sleep(Duration::from_secs(interval_secs));
            }
        }
        return Ok(());
    }

    // --query [source[.field]] [--json] — CLI query mode (no TUI)
    if let Some(pos) = args.iter().position(|a| a == "--query") {
        let query = args
            .get(pos + 1)
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str());
        let json_mode = args.iter().any(|a| a == "--json");
        let code = run_query(query, json_mode);
        std::process::exit(code);
    }

    // --install-service — generate and install a systemd service file
    if args.iter().any(|a| a == "--install-service") {
        return install_service(&args);
    }

    // --uninstall-service — stop, disable, and remove the systemd service
    if args.iter().any(|a| a == "--uninstall-service") {
        return uninstall_service(&args);
    }

    // --serve [bind_addr] — TCP server mode (for Docker containers)
    if args.iter().any(|a| a == "--serve") {
        let bind = args
            .iter()
            .position(|a| a == "--serve")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:9100");
        return serve::run_server(bind);
    }

    // --prometheus [bind_addr] — Prometheus metrics HTTP server
    if args.iter().any(|a| a == "--prometheus") {
        let bind = args
            .iter()
            .position(|a| a == "--prometheus")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:9101");
        return prometheus::run_prometheus_server(bind);
    }

    // --otel [endpoint] [--otel-level core|full]
    if args.iter().any(|a| a == "--otel") {
        let endpoint = args
            .iter()
            .position(|a| a == "--otel")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("http://localhost:4317");
        let interval = args
            .iter()
            .position(|a| a == "--interval")
            .and_then(|pos| args.get(pos + 1))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5);
        // BL-073: --otel-level core|full
        let level = args
            .iter()
            .position(|a| a == "--otel-level")
            .and_then(|pos| args.get(pos + 1))
            .map(|s| otel::OtelLevel::from_str(s))
            .unwrap_or(otel::OtelLevel::Full);
        let locale = args
            .iter()
            .position(|a| a == "--lang")
            .and_then(|pos| args.get(pos + 1))
            .map(|s| i18n::Locale::from_str(s))
            .unwrap_or(i18n::Locale::En);
        return otel::run_otel_export_with_level(endpoint, interval, level, locale);
    }

    // --web [port]
    if args.iter().any(|a| a == "--web") {
        #[cfg(feature = "web")]
        {
            let port = args
                .iter()
                .position(|a| a == "--web")
                .and_then(|pos| args.get(pos + 1))
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(3000);
            let locale = args
                .iter()
                .position(|a| a == "--lang")
                .and_then(|pos| args.get(pos + 1))
                .map(|s| i18n::Locale::from_str(s))
                .unwrap_or(i18n::Locale::En);
            return web::run_web_server(port, locale);
        }
        #[cfg(not(feature = "web"))]
        {
            eprintln!(
                "Web UI support is not compiled in. Rebuild with: cargo build --features web"
            );
            return Ok(());
        }
    }

    // --widget
    if args.iter().any(|a| a == "--widget") {
        #[cfg(feature = "x11widget")]
        {
            return x11_widget::run_widget();
        }
        #[cfg(not(feature = "x11widget"))]
        {
            eprintln!(
                "X11 widget support is not compiled in. Rebuild with: cargo build --features x11widget"
            );
            return Ok(());
        }
    }

    // Load config (silent fallback to defaults if not found)
    let cfg = config::Config::load();

    // --lang <en|ja> (CLI overrides config)
    let locale = if let Some(pos) = args.iter().position(|a| a == "--lang") {
        i18n::Locale::from_str(args.get(pos + 1).expect("--lang requires a locale (en|ja)"))
    } else {
        i18n::Locale::from_str(&cfg.general.lang)
    };

    // --classic flag (start in classic Overview mode)
    let start_classic =
        args.iter().any(|a| a == "--classic") || cfg.general.default_view == "classic";

    // BL-074: --tutorial flag
    let start_tutorial = args.iter().any(|a| a == "--tutorial");

    // --ssh <user@host> (supports multiple)
    let ssh_hosts: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| *a == "--ssh")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // --docker <container_name> (supports multiple)
    let docker_containers: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| *a == "--docker")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // --connect <host:port> (supports multiple)
    let connect_addrs: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| *a == "--connect")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // For backward compatibility, extract first of each for single-host mode
    let ssh_host = ssh_hosts.first().cloned();
    let docker_container = docker_containers.first().cloned();
    let connect_addr = connect_addrs.first().cloned();

    // --import <file.json>
    let imported_snapshot = if let Some(pos) = args.iter().position(|a| a == "--import") {
        let path = args
            .get(pos + 1)
            .expect("--import requires a file path argument");
        let path = Path::new(path);
        // Try importing as series first, fall back to single snapshot
        match export::import_series(path) {
            Ok(series) if !series.is_empty() => Some(series),
            _ => {
                let snap = export::import_snapshot(path)?;
                Some(vec![snap])
            }
        }
    } else {
        None
    };

    // --no-web flag disables automatic web server
    let no_web = args.iter().any(|a| a == "--no-web");

    // --web-port <PORT> (default 8080)
    let web_port: u16 = args
        .iter()
        .position(|a| a == "--web-port")
        .and_then(|pos| args.get(pos + 1))
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8080);

    // Start web server alongside TUI (unless --no-web)
    #[cfg(feature = "web")]
    let _web_thread = if !no_web {
        let web_locale = locale;
        let port = web_port;
        Some(std::thread::spawn(move || {
            if let Err(e) = web::run_web_server(port, web_locale) {
                eprintln!("Web server error: {}", e);
            }
        }))
    } else {
        None
    };
    #[cfg(not(feature = "web"))]
    let _web_thread: Option<std::thread::JoinHandle<()>> = None;

    // TUI mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let alert_rules = cfg.alert;
    let diagnostic_runbooks = cfg.diagnostic_runbook;
    // Build list of extra remote hosts (beyond the primary one used to construct the App).
    // The primary is determined by run(): first ssh_host, then docker_container, then connect_addr.
    // All remaining hosts go into extra_hosts.
    let mut extra_hosts: Vec<(String, String)> = Vec::new(); // (label, type:target)
    let mut skip_first_ssh = ssh_host.is_some();
    let mut skip_first_docker = docker_container.is_some() && ssh_host.is_none();
    let mut skip_first_connect =
        connect_addr.is_some() && ssh_host.is_none() && docker_container.is_none();
    for h in &ssh_hosts {
        if skip_first_ssh {
            skip_first_ssh = false;
            continue;
        }
        extra_hosts.push((format!("ssh:{}", h), format!("ssh:{}", h)));
    }
    for c in &docker_containers {
        if skip_first_docker {
            skip_first_docker = false;
            continue;
        }
        extra_hosts.push((format!("docker:{}", c), format!("docker:{}", c)));
    }
    for a in &connect_addrs {
        if skip_first_connect {
            skip_first_connect = false;
            continue;
        }
        extra_hosts.push((format!("tcp:{}", a), format!("tcp:{}", a)));
    }
    let result = run(
        &mut terminal,
        imported_snapshot,
        ssh_host,
        docker_container,
        connect_addr,
        locale,
        start_classic,
        start_tutorial,
        alert_rules,
        diagnostic_runbooks,
        extra_hosts,
    );

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    Ok(())
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    imported: Option<Vec<proc::Snapshot>>,
    ssh_host: Option<String>,
    docker_container: Option<String>,
    connect_addr: Option<String>,
    locale: i18n::Locale,
    start_classic: bool,
    start_tutorial: bool,
    alert_rules: Vec<alert::AlertRule>,
    diagnostic_runbooks: Vec<config::RunbookConfig>,
    extra_hosts: Vec<(String, String)>,
) -> Result<()> {
    let mut app = if let Some(ref host) = ssh_host {
        let rx = remote::stream_remote(host, 1000)?;
        App::from_remote(host, rx)?
    } else if let Some(ref container) = docker_container {
        let rx = remote::stream_docker(container, 1000)?;
        let label = format!("docker:{}", container);
        App::from_remote(&label, rx)?
    } else if let Some(ref addr) = connect_addr {
        let rx = remote::stream_tcp(addr, 1000)?;
        let label = format!("tcp:{}", addr);
        App::from_remote(&label, rx)?
    } else if let Some(snapshots) = imported {
        App::from_imported(snapshots)?
    } else {
        App::new()?
    };
    app.locale = locale;
    app.alert_rules = alert_rules;
    app.diagnostic_runbooks = diagnostic_runbooks;
    if start_tutorial {
        app.start_tutorial();
    } else if start_classic {
        app.view = ui::app::View::Overview;
        app.focus = ui::app::Focus::Sidebar;
    }

    // Set up multi-host if there are extra hosts
    if !extra_hosts.is_empty() {
        let mut additional: Vec<(String, Option<std::sync::mpsc::Receiver<proc::Snapshot>>)> =
            Vec::new();
        for (label, target) in &extra_hosts {
            let rx = if target.starts_with("ssh:") {
                let host = &target[4..];
                Some(remote::stream_remote(host, 1000)?)
            } else if target.starts_with("docker:") {
                let container = &target[7..];
                Some(remote::stream_docker(container, 1000)?)
            } else if target.starts_with("tcp:") {
                let addr = &target[4..];
                Some(remote::stream_tcp(addr, 1000)?)
            } else {
                None
            };
            additional.push((label.clone(), rx));
        }
        app.init_multi_host(additional);
    }

    while app.running {
        terminal.draw(|f| ui::render::draw(f, &mut app))?;

        let timeout = if app.auto_refresh {
            Duration::from_millis(app.refresh_interval_ms)
        } else {
            Duration::from_millis(100)
        };

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // Search mode input
                if app.searching {
                    match key.code {
                        KeyCode::Enter => app.apply_search(),
                        KeyCode::Esc => app.cancel_search(),
                        KeyCode::Backspace => {
                            app.search_query.pop();
                        }
                        KeyCode::Char(c) => app.search_query.push(c),
                        _ => {}
                    }
                    continue;
                }

                // BL-074: Tutorial mode key handling
                if app.tutorial_step.is_some() && matches!(app.view, ui::app::View::Tutorial) {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.tutorial_finish(),
                        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => app.tutorial_next(),
                        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => {
                            app.tutorial_prev()
                        }
                        _ => {}
                    }
                    continue;
                }

                // Article overlay key handling (priority while open)
                if app.article_overlay.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('A') => {
                            app.close_article_overlay();
                        }
                        KeyCode::Up | KeyCode::Char('k') => app.article_scroll_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.article_scroll_down(),
                        KeyCode::PageUp => app.article_scroll_page_up(),
                        KeyCode::PageDown => app.article_scroll_page_down(),
                        KeyCode::Tab => app.article_next_link(),
                        KeyCode::BackTab => app.article_prev_link(),
                        KeyCode::Enter => app.article_activate_selected_link(),
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            let n = if c == '0' {
                                10
                            } else {
                                (c as u8 - b'0') as usize
                            };
                            app.article_activate_link_by_number(n);
                        }
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::Left | KeyCode::Char('h') => {
                        if matches!(app.view, ui::app::View::CategoryGuide) {
                            app.select_prev_category();
                        } else {
                            app.go_back();
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        if matches!(app.view, ui::app::View::CategoryGuide) {
                            app.select_next_category();
                        } else {
                            app.enter_selected();
                        }
                    }
                    KeyCode::Enter => app.enter_selected(),
                    KeyCode::Backspace => app.go_back(),
                    KeyCode::PageUp => app.scroll_page_up(),
                    KeyCode::PageDown => app.scroll_page_down(),
                    KeyCode::Char('/') => {
                        // Search requires Classic mode (sidebar)
                        if matches!(
                            app.view,
                            ui::app::View::Dashboard
                                | ui::app::View::Welcome
                                | ui::app::View::Diagnostics
                                | ui::app::View::CategoryGuide
                        ) {
                            app.view = ui::app::View::Overview;
                            app.focus = ui::app::Focus::Sidebar;
                        }
                        app.start_search();
                    }
                    KeyCode::Char('[') => {
                        if matches!(app.view, ui::app::View::Diff) {
                            app.diff_older();
                        } else {
                            app.graph_time_window_shrink();
                        }
                    }
                    KeyCode::Char(']') => {
                        if matches!(app.view, ui::app::View::Diff) {
                            app.diff_newer();
                        } else {
                            app.graph_time_window_grow();
                        }
                    }
                    KeyCode::Char('d') => {
                        app.view = ui::app::View::Diff;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('r') => {
                        app.refresh()?;
                    }
                    KeyCode::Char('a') => app.auto_refresh = !app.auto_refresh,
                    KeyCode::Char('S') => {
                        app.toggle_dashboard_axis();
                        let axis_state = if app.dash_zero_axis {
                            i18n::t(app.locale, T::AXIS_ZERO)
                        } else {
                            i18n::t(app.locale, T::AXIS_AUTO)
                        };
                        app.status_message =
                            Some(format!("{}: {}", i18n::t(app.locale, T::AXIS), axis_state));
                    }
                    KeyCode::Char('L') => app.locale = app.locale.next(),
                    KeyCode::Char('?') => {
                        app.help_level = app.help_level.next();
                        app.help_scroll = 0;
                    }
                    KeyCode::Char('D') => {
                        app.view = ui::app::View::Dashboard;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('W') => {
                        app.view = ui::app::View::Welcome;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('O') => {
                        app.view = ui::app::View::Overview;
                        app.focus = ui::app::Focus::Sidebar;
                        app.came_from_dashboard = false;
                    }
                    KeyCode::Char('X') => {
                        app.view = ui::app::View::Diagnostics;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('C') => {
                        app.view = ui::app::View::CategoryGuide;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('A') => {
                        app.toggle_article_overlay();
                    }
                    KeyCode::Char('t') => {
                        app.toggle_sidebar_tree();
                    }
                    KeyCode::Char('c') => {
                        // Copy current field value to clipboard
                        if let Some(text) = app.get_copyable_text() {
                            if copy_to_clipboard(&text) {
                                app.status_message = Some(if app.locale == i18n::Locale::Ja {
                                    format!("コピー: {}", truncate(&text, 40))
                                } else {
                                    format!("Copied: {}", truncate(&text, 40))
                                });
                            }
                        }
                    }
                    KeyCode::Char('R') => {
                        if matches!(app.view, ui::app::View::Diagnostics) {
                            let findings = crate::diagnostics::analyze(
                                &app.current,
                                app.locale,
                                &app.diagnostic_runbooks,
                            );
                            if let Some(finding) = findings.get(app.selected_diagnostic) {
                                if let Some(ref url) = finding.runbook_url {
                                    // Try to open in browser; show URL in status either way
                                    let _ = std::process::Command::new("xdg-open")
                                        .arg(url)
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn()
                                        .or_else(|_| {
                                            std::process::Command::new("open")
                                                .arg(url)
                                                .stdout(std::process::Stdio::null())
                                                .stderr(std::process::Stdio::null())
                                                .spawn()
                                        });
                                    app.status_message = Some(if app.locale == i18n::Locale::Ja {
                                        format!("Runbook: {}", url)
                                    } else {
                                        format!("Runbook: {}", url)
                                    });
                                }
                            }
                        }
                    }
                    KeyCode::Char('g') => {
                        app.start_graph();
                    }
                    KeyCode::Char('e') => {
                        let ts = app
                            .current
                            .timestamp
                            .duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        let filename = format!("syslenz_snapshot_{}.json", ts);
                        if let Err(e) = export::export_snapshot(&app.current, Path::new(&filename))
                        {
                            app.status_message = Some(format!("Export failed: {}", e));
                        } else {
                            app.status_message = Some(format!("Exported {}", filename));
                        }
                    }
                    KeyCode::Tab => {
                        // If in a fullwidth view, switch to Classic mode first
                        if matches!(
                            app.view,
                            ui::app::View::Dashboard
                                | ui::app::View::Welcome
                                | ui::app::View::Diagnostics
                                | ui::app::View::CategoryGuide
                        ) {
                            app.view = ui::app::View::Overview;
                            app.focus = ui::app::Focus::Sidebar;
                        } else {
                            app.focus = if app.focus == ui::app::Focus::Sidebar {
                                ui::app::Focus::Content
                            } else {
                                ui::app::Focus::Sidebar
                            };
                        }
                    }
                    // Multi-host tab switching: F1-F9
                    KeyCode::F(n @ 1..=9) if app.is_multi_host() => {
                        app.switch_host((n as usize) - 1);
                    }
                    _ => {}
                }
            }
        } else if app.auto_refresh {
            app.refresh()?;
        }
    }

    Ok(())
}

const SERVICE_NAME: &str = "syslenz";
const SERVICE_PATH: &str = "/etc/systemd/system/syslenz.service";

fn detect_locale_from_args(args: &[String]) -> i18n::Locale {
    args.iter()
        .position(|a| a == "--lang")
        .and_then(|pos| args.get(pos + 1))
        .map(|s| i18n::Locale::from_str(s))
        .unwrap_or(i18n::Locale::En)
}

fn service_msg(locale: i18n::Locale, key: &str) -> &'static str {
    match locale {
        i18n::Locale::En => match key {
            "install_root" => "Installing systemd service...",
            "install_written" => "Service file written to /etc/systemd/system/syslenz.service",
            "install_enabled" => "Service enabled and started.",
            "install_status" => "Check status with: systemctl status syslenz",
            "install_not_root" => "Not running as root. Printing service file to stdout.",
            "install_hint" => {
                "To install manually, save the above to /etc/systemd/system/syslenz.service and run:"
            }
            "install_hint_cmds" => {
                "  sudo systemctl daemon-reload\n  sudo systemctl enable --now syslenz"
            }
            "uninstall_root" => "Removing systemd service...",
            "uninstall_stopped" => "Service stopped and disabled.",
            "uninstall_removed" => "Service file removed.",
            "uninstall_not_found" => "Service file not found. Nothing to remove.",
            "uninstall_not_root" => "Not running as root. To uninstall manually, run:",
            "uninstall_hint_cmds" => {
                "  sudo systemctl stop syslenz\n  sudo systemctl disable syslenz\n  sudo rm /etc/systemd/system/syslenz.service\n  sudo systemctl daemon-reload"
            }
            _ => "?",
        },
        i18n::Locale::Ja => match key {
            "install_root" => "systemd サービスをインストール中...",
            "install_written" => {
                "サービスファイルを /etc/systemd/system/syslenz.service に書き込みました"
            }
            "install_enabled" => "サービスを有効化し、起動しました。",
            "install_status" => "状態確認: systemctl status syslenz",
            "install_not_root" => "root 権限がありません。サービスファイルを標準出力に表示します。",
            "install_hint" => {
                "手動でインストールするには、上記を /etc/systemd/system/syslenz.service に保存し、以下を実行してください:"
            }
            "install_hint_cmds" => {
                "  sudo systemctl daemon-reload\n  sudo systemctl enable --now syslenz"
            }
            "uninstall_root" => "systemd サービスを削除中...",
            "uninstall_stopped" => "サービスを停止し、無効化しました。",
            "uninstall_removed" => "サービスファイルを削除しました。",
            "uninstall_not_found" => "サービスファイルが見つかりません。削除するものはありません。",
            "uninstall_not_root" => {
                "root 権限がありません。手動でアンインストールするには、以下を実行してください:"
            }
            "uninstall_hint_cmds" => {
                "  sudo systemctl stop syslenz\n  sudo systemctl disable syslenz\n  sudo rm /etc/systemd/system/syslenz.service\n  sudo systemctl daemon-reload"
            }
            _ => "?",
        },
    }
}

fn generate_service_unit() -> String {
    // Find the absolute path of the current executable
    let exe_path = std::env::current_exe()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/usr/local/bin/syslenz".to_string());

    format!(
        r#"[Unit]
Description=SysLenz System Monitor (Prometheus + TCP)
Documentation=https://github.com/syslenz/syslenz
After=network.target

[Service]
Type=simple
ExecStart={exe} --prometheus --serve
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal
SyslogIdentifier=syslenz

[Install]
WantedBy=multi-user.target
"#,
        exe = exe_path,
    )
}

fn is_root() -> bool {
    // On Unix, UID 0 means root
    unsafe { libc::geteuid() == 0 }
}

fn install_service(args: &[String]) -> Result<()> {
    let locale = detect_locale_from_args(args);
    let unit = generate_service_unit();

    if is_root() {
        eprintln!("{}", service_msg(locale, "install_root"));
        fs::write(SERVICE_PATH, &unit)?;
        eprintln!("{}", service_msg(locale, "install_written"));

        // daemon-reload + enable + start
        Command::new("systemctl").arg("daemon-reload").status()?;
        Command::new("systemctl")
            .args(["enable", "--now", SERVICE_NAME])
            .status()?;

        eprintln!("{}", service_msg(locale, "install_enabled"));
        eprintln!("{}", service_msg(locale, "install_status"));
    } else {
        eprintln!("{}", service_msg(locale, "install_not_root"));
        eprintln!();
        println!("{}", unit);
        eprintln!("{}", service_msg(locale, "install_hint"));
        eprintln!("{}", service_msg(locale, "install_hint_cmds"));
    }

    Ok(())
}

fn uninstall_service(args: &[String]) -> Result<()> {
    let locale = detect_locale_from_args(args);

    if is_root() {
        if !Path::new(SERVICE_PATH).exists() {
            eprintln!("{}", service_msg(locale, "uninstall_not_found"));
            return Ok(());
        }
        eprintln!("{}", service_msg(locale, "uninstall_root"));

        let _ = Command::new("systemctl")
            .args(["stop", SERVICE_NAME])
            .status();
        let _ = Command::new("systemctl")
            .args(["disable", SERVICE_NAME])
            .status();

        fs::remove_file(SERVICE_PATH)?;
        Command::new("systemctl").arg("daemon-reload").status()?;

        eprintln!("{}", service_msg(locale, "uninstall_stopped"));
        eprintln!("{}", service_msg(locale, "uninstall_removed"));
    } else {
        eprintln!("{}", service_msg(locale, "uninstall_not_root"));
        eprintln!("{}", service_msg(locale, "uninstall_hint_cmds"));
    }

    Ok(())
}

fn copy_to_clipboard(text: &str) -> bool {
    // Try xclip, then xsel, then wl-copy (Wayland), then pbcopy (macOS)
    for cmd in &[
        "xclip -selection clipboard",
        "xsel --clipboard --input",
        "wl-copy",
        "pbcopy",
    ] {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if let Ok(mut child) = Command::new(parts[0])
            .args(&parts[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        {
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(text.as_bytes());
            }
            return child.wait().map(|s| s.success()).unwrap_or(false);
        }
    }
    false
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max])
    } else {
        s.to_string()
    }
}

/// CLI --query mode: returns exit code (0 = success, 1 = not found).
fn run_query(query: Option<&str>, json_mode: bool) -> i32 {
    let snapshot = match proc::Snapshot::capture() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to capture snapshot: {}", e);
            return 1;
        }
    };

    match query {
        // No argument: list all source names
        None => {
            for key in snapshot.entries.keys() {
                println!("{}", key);
            }
            0
        }
        Some(q) => {
            if let Some(dot_pos) = q.find('.') {
                // source.field form
                let source = &q[..dot_pos];
                let field_name = &q[dot_pos + 1..];
                let entry = match snapshot.entries.get(source) {
                    Some(e) => e,
                    None => {
                        eprintln!("Source '{}' not found", source);
                        return 1;
                    }
                };
                let field = match entry.fields.iter().find(|f| f.name == field_name) {
                    Some(f) => f,
                    None => {
                        eprintln!("Field '{}' not found in '{}'", field_name, source);
                        return 1;
                    }
                };
                if json_mode {
                    let obj = serde_json::json!({
                        "source": source,
                        "field": field.name,
                        "value": field.value,
                        "unit": field.unit,
                        "description": field.description,
                    });
                    println!("{}", serde_json::to_string(&obj).unwrap());
                } else {
                    println!("{}", field.value.display());
                }
                0
            } else {
                // source only: print all fields as TSV
                let entry = match snapshot.entries.get(q) {
                    Some(e) => e,
                    None => {
                        eprintln!("Source '{}' not found", q);
                        return 1;
                    }
                };
                for field in &entry.fields {
                    println!("{}\t{}", field.name, field.value.display());
                }
                0
            }
        }
    }
}

fn print_help() {
    println!(
        "syslenz {} — live system metrics viewer, exporter, and server\n",
        env!("CARGO_PKG_VERSION")
    );
    println!("USAGE:");
    println!("    syslenz [OPTIONS]\n");
    println!("OPTIONS:");
    let opts: &[(&str, &str)] = &[
        ("--help, -h", "Print this help and exit"),
        (
            "--query [source[.field]] [--json]",
            "CLI query mode (no TUI)",
        ),
        (
            "--export <file.json>",
            "Capture one snapshot and write JSON",
        ),
        (
            "--export-series <dir> --interval <secs> --count <n>",
            "Capture a time series of snapshots",
        ),
        (
            "--export-article-resources <dir>",
            "Export article index/en/ja JSON",
        ),
        (
            "--export-article-markdown-resources <dir>",
            "Export markdown article resources",
        ),
        (
            "--import <file.json>",
            "Import a snapshot or series into the TUI",
        ),
        (
            "--serve [bind_addr]",
            "TCP server mode (default 0.0.0.0:9100)",
        ),
        (
            "--prometheus [bind_addr]",
            "Prometheus metrics HTTP server (default 0.0.0.0:9101)",
        ),
        ("--web [port]", "Web UI server (default 3000, feature: web)"),
        (
            "--otel [endpoint] [--otel-level core|full]",
            "OpenTelemetry export",
        ),
        ("--widget", "X11 desktop widget (feature: x11widget)"),
        (
            "--ssh <user@host>",
            "Monitor remote host via SSH (repeatable)",
        ),
        (
            "--docker <container>",
            "Monitor Docker container (repeatable)",
        ),
        (
            "--connect <host:port>",
            "Connect to a --serve endpoint (repeatable)",
        ),
        ("--lang <en|ja>", "Override UI/CLI language"),
        ("--classic", "Start in classic Overview mode"),
        ("--tutorial", "Start with the tutorial"),
        (
            "--install-service",
            "Generate and install a systemd service file",
        ),
        (
            "--uninstall-service",
            "Stop, disable, and remove the systemd service",
        ),
    ];
    for (flag, desc) in opts {
        println!("    {:<48} {}", flag, desc);
    }
    println!("\nRun without options to start the interactive TUI.");
}
