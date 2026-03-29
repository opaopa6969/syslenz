mod alert;
mod config;
mod diagnostics;
mod education;
mod serve;
mod prometheus;
mod proc;
mod sys;
mod net;
mod ui;
mod export;
mod remote;
mod i18n;
mod otel;
mod plugin;
#[cfg(feature = "web")]
mod web;
#[cfg(feature = "x11widget")]
mod x11_widget;

use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ui::app::App;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // --export <file.json>
    if let Some(pos) = args.iter().position(|a| a == "--export") {
        let path = args.get(pos + 1).expect("--export requires a file path argument");
        let snapshot = proc::Snapshot::capture()?;
        export::export_snapshot(&snapshot, Path::new(path))?;
        eprintln!("Snapshot exported to {}", path);
        return Ok(());
    }

    // --export-series <dir> --interval <secs> --count <n>
    if let Some(pos) = args.iter().position(|a| a == "--export-series") {
        let dir = args.get(pos + 1).expect("--export-series requires a directory argument");
        let interval_pos = args.iter().position(|a| a == "--interval")
            .expect("--export-series requires --interval <secs>");
        let interval_secs: u64 = args.get(interval_pos + 1)
            .expect("--interval requires a value")
            .parse()
            .expect("--interval must be a number");
        let count_pos = args.iter().position(|a| a == "--count")
            .expect("--export-series requires --count <n>");
        let count: usize = args.get(count_pos + 1)
            .expect("--count requires a value")
            .parse()
            .expect("--count must be a number");

        std::fs::create_dir_all(dir)?;
        for i in 0..count {
            let snapshot = proc::Snapshot::capture()?;
            let ts = snapshot.timestamp
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

    // --serve [bind_addr] — TCP server mode (for Docker containers)
    if args.iter().any(|a| a == "--serve") {
        let bind = args.iter().position(|a| a == "--serve")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:9100");
        return serve::run_server(bind);
    }

    // --prometheus [bind_addr] — Prometheus metrics HTTP server
    if args.iter().any(|a| a == "--prometheus") {
        let bind = args.iter().position(|a| a == "--prometheus")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:9101");
        return prometheus::run_prometheus_server(bind);
    }

    // --otel [endpoint] [--otel-level core|full]
    if args.iter().any(|a| a == "--otel") {
        let endpoint = args.iter().position(|a| a == "--otel")
            .and_then(|pos| args.get(pos + 1))
            .filter(|s| !s.starts_with("--"))
            .map(|s| s.as_str())
            .unwrap_or("http://localhost:4317");
        let interval = args.iter().position(|a| a == "--interval")
            .and_then(|pos| args.get(pos + 1))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5);
        // BL-073: --otel-level core|full
        let level = args.iter().position(|a| a == "--otel-level")
            .and_then(|pos| args.get(pos + 1))
            .map(|s| otel::OtelLevel::from_str(s))
            .unwrap_or(otel::OtelLevel::Full);
        let locale = args.iter().position(|a| a == "--lang")
            .and_then(|pos| args.get(pos + 1))
            .map(|s| i18n::Locale::from_str(s))
            .unwrap_or(i18n::Locale::En);
        return otel::run_otel_export_with_level(endpoint, interval, level, locale);
    }

    // --web [port]
    if args.iter().any(|a| a == "--web") {
        #[cfg(feature = "web")]
        {
            let port = args.iter().position(|a| a == "--web")
                .and_then(|pos| args.get(pos + 1))
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(3000);
            let locale = args.iter().position(|a| a == "--lang")
                .and_then(|pos| args.get(pos + 1))
                .map(|s| i18n::Locale::from_str(s))
                .unwrap_or(i18n::Locale::En);
            return web::run_web_server(port, locale);
        }
        #[cfg(not(feature = "web"))]
        {
            eprintln!("Web UI support is not compiled in. Rebuild with: cargo build --features web");
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
            eprintln!("X11 widget support is not compiled in. Rebuild with: cargo build --features x11widget");
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
    let start_classic = args.iter().any(|a| a == "--classic")
        || cfg.general.default_view == "classic";

    // BL-074: --tutorial flag
    let start_tutorial = args.iter().any(|a| a == "--tutorial");

    // --ssh <user@host> (supports multiple)
    let ssh_hosts: Vec<String> = args.iter().enumerate()
        .filter(|(_, a)| *a == "--ssh")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // --docker <container_name> (supports multiple)
    let docker_containers: Vec<String> = args.iter().enumerate()
        .filter(|(_, a)| *a == "--docker")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // --connect <host:port> (supports multiple)
    let connect_addrs: Vec<String> = args.iter().enumerate()
        .filter(|(_, a)| *a == "--connect")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();

    // For backward compatibility, extract first of each for single-host mode
    let ssh_host = ssh_hosts.first().cloned();
    let docker_container = docker_containers.first().cloned();
    let connect_addr = connect_addrs.first().cloned();


    // --import <file.json>
    let imported_snapshot = if let Some(pos) = args.iter().position(|a| a == "--import") {
        let path = args.get(pos + 1).expect("--import requires a file path argument");
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

    // TUI mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let alert_rules = cfg.alert;
    // Build list of extra remote hosts (beyond the primary one used to construct the App).
    // The primary is determined by run(): first ssh_host, then docker_container, then connect_addr.
    // All remaining hosts go into extra_hosts.
    let mut extra_hosts: Vec<(String, String)> = Vec::new(); // (label, type:target)
    let mut skip_first_ssh = ssh_host.is_some();
    let mut skip_first_docker = docker_container.is_some() && ssh_host.is_none();
    let mut skip_first_connect = connect_addr.is_some() && ssh_host.is_none() && docker_container.is_none();
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
    let result = run(&mut terminal, imported_snapshot, ssh_host, docker_container, connect_addr, locale, start_classic, start_tutorial, alert_rules, extra_hosts);

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
    if start_tutorial {
        app.start_tutorial();
    } else if start_classic {
        app.view = ui::app::View::Overview;
        app.focus = ui::app::Focus::Sidebar;
    }

    // Set up multi-host if there are extra hosts
    if !extra_hosts.is_empty() {
        let mut additional: Vec<(String, Option<std::sync::mpsc::Receiver<proc::Snapshot>>)> = Vec::new();
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
                if key.kind != KeyEventKind::Press { continue; }

                // Search mode input
                if app.searching {
                    match key.code {
                        KeyCode::Enter => app.apply_search(),
                        KeyCode::Esc => app.cancel_search(),
                        KeyCode::Backspace => { app.search_query.pop(); }
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
                        KeyCode::Backspace | KeyCode::Left | KeyCode::Char('h') => app.tutorial_prev(),
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
                        if matches!(app.view, ui::app::View::Dashboard | ui::app::View::Welcome | ui::app::View::Diagnostics | ui::app::View::CategoryGuide) {
                            app.view = ui::app::View::Overview;
                            app.focus = ui::app::Focus::Sidebar;
                        }
                        app.start_search();
                    }
                    KeyCode::Char('[') => {
                        if matches!(app.view, ui::app::View::Diff) {
                            app.diff_older();
                        }
                    }
                    KeyCode::Char(']') => {
                        if matches!(app.view, ui::app::View::Diff) {
                            app.diff_newer();
                        }
                    }
                    KeyCode::Char('d') => {
                        app.view = ui::app::View::Diff;
                        app.focus = ui::app::Focus::Content;
                    }
                    KeyCode::Char('r') => { app.refresh()?; }
                    KeyCode::Char('a') => app.auto_refresh = !app.auto_refresh,
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
                    KeyCode::Char('c') => {
                        // Copy current field value to clipboard
                        if let Some(text) = app.get_copyable_text() {
                            if copy_to_clipboard(&text) {
                                app.status_message = Some(
                                    if app.locale == i18n::Locale::Ja {
                                        format!("コピー: {}", truncate(&text, 40))
                                    } else {
                                        format!("Copied: {}", truncate(&text, 40))
                                    }
                                );
                            }
                        }
                    }
                    KeyCode::Char('g') => {
                        app.start_graph();
                    }
                    KeyCode::Char('e') => {
                        let ts = app.current.timestamp
                            .duration_since(std::time::SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        let filename = format!("syslenz_snapshot_{}.json", ts);
                        if let Err(e) = export::export_snapshot(
                            &app.current,
                            Path::new(&filename),
                        ) {
                            app.status_message = Some(format!("Export failed: {}", e));
                        } else {
                            app.status_message = Some(format!("Exported {}", filename));
                        }
                    }
                    KeyCode::Tab => {
                        // If in a fullwidth view, switch to Classic mode first
                        if matches!(app.view, ui::app::View::Dashboard | ui::app::View::Welcome | ui::app::View::Diagnostics | ui::app::View::CategoryGuide) {
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

fn copy_to_clipboard(text: &str) -> bool {
    // Try xclip, then xsel, then wl-copy (Wayland), then pbcopy (macOS)
    for cmd in &["xclip -selection clipboard", "xsel --clipboard --input", "wl-copy", "pbcopy"] {
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
