use crate::proc::{FieldValue, Snapshot};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AlertRule {
    pub source: String,
    pub field: String,
    pub condition: String, // e.g. "< 500000000" or "> 90.0"
    pub severity: String,  // "info", "warning", "critical"
    pub message: String,
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
}
