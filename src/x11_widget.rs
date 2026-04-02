//! X11 floating widget — always-on-top compact system monitor.
//!
//! Displays CPU%, memory usage, and load average in a small translucent
//! window using the X11 core protocol.

#[cfg(feature = "x11widget")]
use crate::proc::{FieldValue, Snapshot};

#[cfg(feature = "x11widget")]
use x11rb::COPY_DEPTH_FROM_PARENT;
#[cfg(feature = "x11widget")]
use x11rb::connection::Connection;
#[cfg(feature = "x11widget")]
use x11rb::protocol::xproto::*;
#[cfg(feature = "x11widget")]
use x11rb::wrapper::ConnectionExt as _;

#[cfg(feature = "x11widget")]
const WIDTH: u16 = 260;
#[cfg(feature = "x11widget")]
const HEIGHT: u16 = 120;
#[cfg(feature = "x11widget")]
const REFRESH_MS: u64 = 1000;
#[cfg(feature = "x11widget")]
const BG_COLOR: u32 = 0x1a1b26;
#[cfg(feature = "x11widget")]
const FG_COLOR: u32 = 0xc0caf5;
#[cfg(feature = "x11widget")]
const ACCENT_COLOR: u32 = 0x7aa2f7;
#[cfg(feature = "x11widget")]
const GREEN_COLOR: u32 = 0x9ece6a;
#[cfg(feature = "x11widget")]
const YELLOW_COLOR: u32 = 0xe0af68;
#[cfg(feature = "x11widget")]
const RED_COLOR: u32 = 0xf7768e;

#[cfg(feature = "x11widget")]
struct WidgetMetrics {
    cpu_percent: f64,
    mem_total: u64,
    mem_available: u64,
    load1: f64,
    load5: f64,
    load15: f64,
    uptime: f64,
}

#[cfg(feature = "x11widget")]
impl WidgetMetrics {
    fn from_snapshot(snapshot: &Snapshot) -> Self {
        let mut m = WidgetMetrics {
            cpu_percent: 0.0,
            mem_total: 0,
            mem_available: 0,
            load1: 0.0,
            load5: 0.0,
            load15: 0.0,
            uptime: 0.0,
        };

        if let Some(entry) = snapshot.entries.get("meminfo") {
            for field in &entry.fields {
                match field.name.as_str() {
                    "MemTotal" => {
                        if let FieldValue::Bytes(v) = field.value {
                            m.mem_total = v;
                        }
                    }
                    "MemAvailable" => {
                        if let FieldValue::Bytes(v) = field.value {
                            m.mem_available = v;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(entry) = snapshot.entries.get("loadavg") {
            for field in &entry.fields {
                match field.name.as_str() {
                    "load1" => {
                        if let FieldValue::Float(v) = field.value {
                            m.load1 = v;
                        }
                    }
                    "load5" => {
                        if let FieldValue::Float(v) = field.value {
                            m.load5 = v;
                        }
                    }
                    "load15" => {
                        if let FieldValue::Float(v) = field.value {
                            m.load15 = v;
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(entry) = snapshot.entries.get("uptime") {
            for field in &entry.fields {
                if field.name == "uptime" {
                    if let FieldValue::Duration(v) = field.value {
                        m.uptime = v;
                    }
                }
            }
        }

        // Estimate CPU% from load average and number of CPUs
        if let Some(entry) = snapshot.entries.get("cpuinfo") {
            let num_cpus = entry
                .fields
                .iter()
                .filter(|f| f.name == "processor")
                .count()
                .max(1);
            m.cpu_percent = (m.load1 / num_cpus as f64 * 100.0).min(100.0);
        }

        m
    }

    fn mem_percent(&self) -> f64 {
        if self.mem_total == 0 {
            return 0.0;
        }
        (1.0 - self.mem_available as f64 / self.mem_total as f64) * 100.0
    }
}

#[cfg(feature = "x11widget")]
pub fn run_widget() -> anyhow::Result<()> {
    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let win = conn.generate_id()?;
    let gc = conn.generate_id()?;

    // Create window
    let x_pos = screen.width_in_pixels.saturating_sub(WIDTH + 20);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win,
        screen.root,
        x_pos as i16,
        20, // top-right corner
        WIDTH,
        HEIGHT,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(BG_COLOR)
            .event_mask(EventMask::EXPOSURE | EventMask::KEY_PRESS | EventMask::BUTTON_PRESS)
            .override_redirect(1), // bypass window manager for floating
    )?;

    // Create GC
    conn.create_gc(
        gc,
        win,
        &CreateGCAux::new().foreground(FG_COLOR).background(BG_COLOR),
    )?;

    // Set window title
    conn.change_property8(
        PropMode::REPLACE,
        win,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        b"syslenz",
    )?;

    // Set _NET_WM_WINDOW_TYPE to DOCK for always-on-top behavior
    let net_wm_type = conn
        .intern_atom(false, b"_NET_WM_WINDOW_TYPE")?
        .reply()?
        .atom;
    let dock_type = conn
        .intern_atom(false, b"_NET_WM_WINDOW_TYPE_DOCK")?
        .reply()?
        .atom;
    conn.change_property32(
        PropMode::REPLACE,
        win,
        net_wm_type,
        AtomEnum::ATOM,
        &[dock_type],
    )?;

    // Set _NET_WM_STATE to ABOVE
    let net_wm_state = conn.intern_atom(false, b"_NET_WM_STATE")?.reply()?.atom;
    let above = conn
        .intern_atom(false, b"_NET_WM_STATE_ABOVE")?
        .reply()?
        .atom;
    let sticky = conn
        .intern_atom(false, b"_NET_WM_STATE_STICKY")?
        .reply()?
        .atom;
    conn.change_property32(
        PropMode::REPLACE,
        win,
        net_wm_state,
        AtomEnum::ATOM,
        &[above, sticky],
    )?;

    conn.map_window(win)?;
    conn.flush()?;

    // History for mini sparkline
    let mut cpu_history: Vec<f64> = Vec::new();
    let mut mem_history: Vec<f64> = Vec::new();

    loop {
        // Handle X11 events (non-blocking)
        while let Some(event) = conn.poll_for_event()? {
            match event {
                x11rb::protocol::Event::Expose(_) => {
                    // Will redraw below
                }
                x11rb::protocol::Event::KeyPress(ev) => {
                    // 'q' key (keycode 24 on most layouts)
                    if ev.detail == 24 {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        // Capture and render
        if let Ok(snapshot) = Snapshot::capture() {
            let metrics = WidgetMetrics::from_snapshot(&snapshot);

            cpu_history.push(metrics.cpu_percent);
            mem_history.push(metrics.mem_percent());
            if cpu_history.len() > 40 {
                cpu_history.remove(0);
            }
            if mem_history.len() > 40 {
                mem_history.remove(0);
            }

            // Clear
            conn.change_gc(gc, &ChangeGCAux::new().foreground(BG_COLOR))?;
            conn.poly_fill_rectangle(
                win,
                gc,
                &[Rectangle {
                    x: 0,
                    y: 0,
                    width: WIDTH,
                    height: HEIGHT,
                }],
            )?;

            // Title
            draw_text(&conn, win, gc, 8, 16, "syslenz", ACCENT_COLOR)?;

            // CPU bar
            let cpu_color = if metrics.cpu_percent > 80.0 {
                RED_COLOR
            } else if metrics.cpu_percent > 50.0 {
                YELLOW_COLOR
            } else {
                GREEN_COLOR
            };
            let cpu_label = format!("CPU: {:.0}%", metrics.cpu_percent);
            draw_text(&conn, win, gc, 8, 34, &cpu_label, FG_COLOR)?;
            draw_bar(
                &conn,
                win,
                gc,
                100,
                26,
                150,
                10,
                metrics.cpu_percent,
                cpu_color,
            )?;

            // Memory bar
            let mem_pct = metrics.mem_percent();
            let mem_used_gb = (metrics.mem_total - metrics.mem_available) as f64 / 1073741824.0;
            let mem_total_gb = metrics.mem_total as f64 / 1073741824.0;
            let mem_label = format!(
                "MEM: {:.1}/{:.1}G ({:.0}%)",
                mem_used_gb, mem_total_gb, mem_pct
            );
            draw_text(&conn, win, gc, 8, 50, &mem_label, FG_COLOR)?;
            draw_bar(&conn, win, gc, 100, 42, 150, 10, mem_pct, ACCENT_COLOR)?;

            // Load average
            let load_label = format!(
                "Load: {:.2} {:.2} {:.2}",
                metrics.load1, metrics.load5, metrics.load15
            );
            draw_text(&conn, win, gc, 8, 66, &load_label, FG_COLOR)?;

            // Mini sparklines
            draw_sparkline(&conn, win, gc, 8, 76, 120, 16, &cpu_history, GREEN_COLOR)?;
            draw_sparkline(&conn, win, gc, 136, 76, 120, 16, &mem_history, ACCENT_COLOR)?;

            // Uptime
            let days = (metrics.uptime / 86400.0) as u64;
            let hours = ((metrics.uptime % 86400.0) / 3600.0) as u64;
            let uptime_label = format!("Up: {}d {}h", days, hours);
            draw_text(&conn, win, gc, 8, 108, &uptime_label, 0x565f89)?;

            conn.flush()?;
        }

        std::thread::sleep(std::time::Duration::from_millis(REFRESH_MS));
    }
}

#[cfg(feature = "x11widget")]
fn draw_text(
    conn: &impl Connection,
    win: Window,
    gc: Gcontext,
    x: i16,
    y: i16,
    text: &str,
    color: u32,
) -> anyhow::Result<()> {
    conn.change_gc(gc, &ChangeGCAux::new().foreground(color))?;
    conn.image_text8(win, gc, x, y, text.as_bytes())?;
    Ok(())
}

#[cfg(feature = "x11widget")]
fn draw_bar(
    conn: &impl Connection,
    win: Window,
    gc: Gcontext,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    percent: f64,
    color: u32,
) -> anyhow::Result<()> {
    // Background
    conn.change_gc(gc, &ChangeGCAux::new().foreground(0x292e42))?;
    conn.poly_fill_rectangle(
        win,
        gc,
        &[Rectangle {
            x,
            y,
            width,
            height,
        }],
    )?;
    // Filled portion
    let fill_width = ((width as f64 * percent / 100.0) as u16).min(width);
    if fill_width > 0 {
        conn.change_gc(gc, &ChangeGCAux::new().foreground(color))?;
        conn.poly_fill_rectangle(
            win,
            gc,
            &[Rectangle {
                x,
                y,
                width: fill_width,
                height,
            }],
        )?;
    }
    Ok(())
}

#[cfg(feature = "x11widget")]
fn draw_sparkline(
    conn: &impl Connection,
    win: Window,
    gc: Gcontext,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    data: &[f64],
    color: u32,
) -> anyhow::Result<()> {
    if data.is_empty() {
        return Ok(());
    }

    // Background
    conn.change_gc(gc, &ChangeGCAux::new().foreground(0x1f2335))?;
    conn.poly_fill_rectangle(
        win,
        gc,
        &[Rectangle {
            x,
            y,
            width,
            height,
        }],
    )?;

    let max = data
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);
    let step = width as f64 / data.len().max(1) as f64;

    conn.change_gc(gc, &ChangeGCAux::new().foreground(color))?;
    let points: Vec<Point> = data
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let px = x + (i as f64 * step) as i16;
            let py = y + height as i16 - (v / max * height as f64) as i16;
            Point { x: px, y: py }
        })
        .collect();

    if points.len() >= 2 {
        conn.poly_line(CoordMode::ORIGIN, win, gc, &points)?;
    }

    Ok(())
}

#[cfg(not(feature = "x11widget"))]
pub fn run_widget() -> anyhow::Result<()> {
    anyhow::bail!(
        "X11 widget support is not compiled in.\n\
         Rebuild with: cargo build --features x11widget"
    )
}
