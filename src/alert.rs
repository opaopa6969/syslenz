use crate::proc::{FieldValue, Snapshot};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlertRule {
    pub source: String,
    pub field: String,
    pub condition: String, // e.g. "< 500000000" or "> 90.0"
    pub severity: String,  // "info", "warning", "critical"
    pub message: String,
    /// Optional external command to execute when the alert fires.
    /// Supports `{message}`, `{source}`, `{field}`, `{value}`, `{severity}` placeholders.
    /// Example: `action = "notify-send 'syslenz: {message}'"` or `action = "curl -X POST ..."`.
    #[serde(default)]
    pub action: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompareOp {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
}

#[derive(Debug, Clone)]
pub struct AlertEvent {
    pub rule_index: usize,
    pub source: String,
    pub field: String,
    pub severity: String,
    pub message: String,
    pub current_value: String,
    pub firing: bool,
}

/// Parse a condition string like "< 500000000" or "> 90.0" into (operator, threshold).
/// Returns None if the condition is invalid.
fn parse_condition(cond: &str) -> Option<(CompareOp, f64)> {
    let cond = cond.trim();
    // Try two-char operators first
    let (op, rest) = if let Some(rest) = cond.strip_prefix(">=") {
        (CompareOp::Gte, rest)
    } else if let Some(rest) = cond.strip_prefix("<=") {
        (CompareOp::Lte, rest)
    } else if let Some(rest) = cond.strip_prefix("!=") {
        (CompareOp::Neq, rest)
    } else if let Some(rest) = cond.strip_prefix("==") {
        (CompareOp::Eq, rest)
    } else if let Some(rest) = cond.strip_prefix('>') {
        (CompareOp::Gt, rest)
    } else if let Some(rest) = cond.strip_prefix('<') {
        (CompareOp::Lt, rest)
    } else {
        return None;
    };

    // Allow underscores in numbers for readability (e.g. 500_000_000)
    let num_str: String = rest.trim().chars().filter(|c| *c != '_').collect();
    let threshold: f64 = num_str.parse().ok()?;
    Some((op, threshold))
}

fn field_to_f64(value: &FieldValue) -> Option<f64> {
    match value {
        FieldValue::Bytes(b) => Some(*b as f64),
        FieldValue::Integer(i) => Some(*i as f64),
        FieldValue::Float(f) => Some(*f),
        FieldValue::Duration(d) => Some(*d),
        FieldValue::Text(_) | FieldValue::Table(_) => None,
    }
}

fn compare(actual: f64, op: CompareOp, threshold: f64) -> bool {
    match op {
        CompareOp::Gt => actual > threshold,
        CompareOp::Lt => actual < threshold,
        CompareOp::Gte => actual >= threshold,
        CompareOp::Lte => actual <= threshold,
        CompareOp::Eq => (actual - threshold).abs() < f64::EPSILON,
        CompareOp::Neq => (actual - threshold).abs() >= f64::EPSILON,
    }
}

/// Evaluate all alert rules against a snapshot, returning active alert events.
/// `prev_firing` contains rule indices that were already firing (for debounce).
pub fn evaluate_alerts(
    snapshot: &Snapshot,
    rules: &[AlertRule],
    prev_firing: &[usize],
) -> Vec<AlertEvent> {
    let mut events = Vec::new();

    for (i, rule) in rules.iter().enumerate() {
        let (op, threshold) = match parse_condition(&rule.condition) {
            Some(parsed) => parsed,
            None => continue, // Skip invalid rules silently
        };

        // Find the source entry
        let entry = match snapshot.entries.get(&rule.source) {
            Some(e) => e,
            None => continue,
        };

        // Find the field
        let field = match entry.fields.iter().find(|f| f.name == rule.field) {
            Some(f) => f,
            None => continue,
        };

        // Get numeric value
        let actual = match field_to_f64(&field.value) {
            Some(v) => v,
            None => continue,
        };

        let firing = compare(actual, op, threshold);

        // Only emit event if firing or was previously firing (for state transition tracking)
        if firing || prev_firing.contains(&i) {
            events.push(AlertEvent {
                rule_index: i,
                source: rule.source.clone(),
                field: rule.field.clone(),
                severity: rule.severity.clone(),
                message: rule.message.clone(),
                current_value: field.value.display(),
                firing,
            });
        }
    }

    events
}

/// Count alerts by severity. Returns (info_count, warning_count, critical_count).
pub fn count_by_severity(alerts: &[AlertEvent]) -> (usize, usize, usize) {
    let mut info = 0;
    let mut warn = 0;
    let mut crit = 0;
    for a in alerts {
        if !a.firing {
            continue;
        }
        match a.severity.as_str() {
            "info" => info += 1,
            "warning" => warn += 1,
            "critical" => crit += 1,
            _ => {}
        }
    }
    (info, warn, crit)
}

/// Check if a given source has any active (firing) alerts.
pub fn source_max_severity<'a>(alerts: &'a [AlertEvent], source: &'a str) -> Option<&'a str> {
    let mut max: Option<&str> = None;
    for a in alerts {
        if !a.firing || a.source != source {
            continue;
        }
        max = Some(match (max, a.severity.as_str()) {
            (Some("critical"), _) | (_, "critical") => "critical",
            (Some("warning"), _) | (_, "warning") => "warning",
            (_, s) => s,
        });
    }
    max
}

/// Check if a specific field in a source is firing an alert.
pub fn field_alert_severity<'a>(alerts: &'a [AlertEvent], source: &str, field: &str) -> Option<&'a str> {
    for a in alerts {
        if a.firing && a.source == source && a.field == field {
            return Some(&a.severity);
        }
    }
    None
}

/// Execute external action commands for newly-firing alerts (BL-071).
///
/// Only fires actions for alerts that are newly firing (not in `prev_firing`).
/// Runs the command asynchronously (spawns a detached child process) so the
/// TUI never blocks.
pub fn execute_actions(rules: &[AlertRule], events: &[AlertEvent], prev_firing: &[usize]) {
    for event in events {
        if !event.firing {
            continue;
        }
        // Only fire action on *new* alerts (not already firing previously)
        if prev_firing.contains(&event.rule_index) {
            continue;
        }
        if let Some(ref action_template) = rules.get(event.rule_index).and_then(|r| r.action.as_ref()) {
            let cmd = action_template
                .replace("{message}", &event.message)
                .replace("{source}", &event.source)
                .replace("{field}", &event.field)
                .replace("{value}", &event.current_value)
                .replace("{severity}", &event.severity);

            // Spawn detached so TUI doesn't block
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_condition() {
        let (op, val) = parse_condition("> 8.0").unwrap();
        assert!(matches!(op, CompareOp::Gt));
        assert!((val - 8.0).abs() < f64::EPSILON);

        let (op, val) = parse_condition("< 500_000_000").unwrap();
        assert!(matches!(op, CompareOp::Lt));
        assert!((val - 500_000_000.0).abs() < 1.0);

        let (op, val) = parse_condition(">= 90").unwrap();
        assert!(matches!(op, CompareOp::Gte));
        assert!((val - 90.0).abs() < f64::EPSILON);

        let (op, val) = parse_condition("!= 0").unwrap();
        assert!(matches!(op, CompareOp::Neq));
        assert!((val - 0.0).abs() < f64::EPSILON);

        assert!(parse_condition("hello world").is_none());
        assert!(parse_condition("").is_none());
    }

    #[test]
    fn test_compare() {
        assert!(compare(10.0, CompareOp::Gt, 5.0));
        assert!(!compare(3.0, CompareOp::Gt, 5.0));
        assert!(compare(3.0, CompareOp::Lt, 5.0));
        assert!(compare(5.0, CompareOp::Gte, 5.0));
        assert!(compare(5.0, CompareOp::Lte, 5.0));
        assert!(compare(5.0, CompareOp::Eq, 5.0));
        assert!(compare(5.1, CompareOp::Neq, 5.0));
    }

    // BL-071: AlertRule with action field deserializes correctly
    #[test]
    fn alert_rule_action_field_optional() {
        // Without action
        let toml_str = r#"
            source = "loadavg"
            field = "load_1min"
            condition = "> 8.0"
            severity = "warning"
            message = "High load"
        "#;
        let rule: AlertRule = toml::from_str(toml_str).unwrap();
        assert!(rule.action.is_none());

        // With action
        let toml_str = r#"
            source = "loadavg"
            field = "load_1min"
            condition = "> 8.0"
            severity = "warning"
            message = "High load"
            action = "echo {message}"
        "#;
        let rule: AlertRule = toml::from_str(toml_str).unwrap();
        assert_eq!(rule.action, Some("echo {message}".to_string()));
    }

    // BL-071: execute_actions does not fire for already-firing alerts
    #[test]
    fn execute_actions_skips_prev_firing() {
        let rules = vec![AlertRule {
            source: "loadavg".to_string(),
            field: "load_1min".to_string(),
            condition: "> 8.0".to_string(),
            severity: "warning".to_string(),
            message: "High load".to_string(),
            action: Some("echo test".to_string()),
        }];
        let events = vec![AlertEvent {
            rule_index: 0,
            source: "loadavg".to_string(),
            field: "load_1min".to_string(),
            severity: "warning".to_string(),
            message: "High load".to_string(),
            current_value: "10.0".to_string(),
            firing: true,
        }];
        // With prev_firing containing index 0, action should be skipped
        // (This just ensures no panic — we can't easily test command execution)
        execute_actions(&rules, &events, &[0]);
        // With empty prev_firing, action should fire (spawns `echo test` harmlessly)
        execute_actions(&rules, &events, &[]);
    }
}
