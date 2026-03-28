use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame,
};

use crate::proc::FieldValue;
use super::app::App;

fn extract_numeric(value: &FieldValue) -> Option<f64> {
    match value {
        FieldValue::Bytes(b) => Some(*b as f64),
        FieldValue::Integer(i) => Some(*i as f64),
        FieldValue::Float(f) => Some(*f),
        FieldValue::Duration(d) => Some(*d),
        _ => None,
    }
}

pub fn draw_graph(f: &mut Frame, app: &App, area: Rect) {
    let Some((ref source_key, ref field_name)) = app.graph_field else {
        let p = Paragraph::new(" No field selected for graphing")
            .block(Block::default().borders(Borders::ALL).title(" Graph "));
        f.render_widget(p, area);
        return;
    };

    // Collect historical values from snapshots + current
    let mut values: Vec<f64> = Vec::new();
    for snap in &app.snapshots {
        if let Some(entry) = snap.entries.get(source_key) {
            if let Some(field) = entry.fields.iter().find(|f| f.name == *field_name) {
                if let Some(v) = extract_numeric(&field.value) {
                    values.push(v);
                }
            }
        }
    }
    // Add current value
    if let Some(entry) = app.current.entries.get(source_key) {
        if let Some(field) = entry.fields.iter().find(|f| f.name == *field_name) {
            if let Some(v) = extract_numeric(&field.value) {
                values.push(v);
            }
        }
    }

    let title = format!(" Graph: {} / {} ", source_key, field_name);

    if values.is_empty() {
        let p = Paragraph::new(" Collecting data...")
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(p, area);
        return;
    }

    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg_val: f64 = values.iter().sum::<f64>() / values.len() as f64;
    let current_val = *values.last().unwrap();

    // Split area into stats text and sparkline
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // stats
            Constraint::Min(4),   // sparkline
        ])
        .split(area);

    // Stats line
    let stats = Line::from(vec![
        Span::styled(" Min: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format_value(min_val), Style::default().fg(Color::Cyan)),
        Span::styled("  Max: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format_value(max_val), Style::default().fg(Color::Red)),
        Span::styled("  Avg: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format_value(avg_val), Style::default().fg(Color::Yellow)),
        Span::styled("  Current: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format_value(current_val),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ({} samples)", values.len()),
            Style::default().fg(Color::DarkGray),
        ),
    ]);
    let stats_paragraph = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(stats_paragraph, chunks[0]);

    // Normalize values for sparkline (u64 range 0..100)
    let range = max_val - min_val;
    let sparkline_data: Vec<u64> = if range < f64::EPSILON {
        vec![50; values.len()]
    } else {
        values
            .iter()
            .map(|v| ((v - min_val) / range * 100.0) as u64)
            .collect()
    };

    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Sparkline "),
        )
        .data(&sparkline_data)
        .style(Style::default().fg(Color::Green));
    f.render_widget(sparkline, chunks[1]);
}

fn format_value(v: f64) -> String {
    if v.abs() >= 1_073_741_824.0 {
        format!("{:.2} GiB", v / 1_073_741_824.0)
    } else if v.abs() >= 1_048_576.0 {
        format!("{:.1} MiB", v / 1_048_576.0)
    } else if v.abs() >= 10_000.0 {
        format!("{:.1}K", v / 1_000.0)
    } else if v == v.floor() {
        format!("{}", v as i64)
    } else {
        format!("{:.2}", v)
    }
}
