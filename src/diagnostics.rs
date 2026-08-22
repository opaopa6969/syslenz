//! Automatic diagnostics engine.
//!
//! Analyzes a [`Snapshot`] and produces diagnostic findings based on
//! known thresholds and patterns.

use crate::config::RunbookConfig;
use crate::i18n::Locale;
use crate::proc::{FieldValue, Snapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

impl Severity {
    pub fn label(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Severity::Info, Locale::En) => "INFO",
            (Severity::Warning, Locale::En) => "WARN",
            (Severity::Critical, Locale::En) => "CRIT",
            (Severity::Info, Locale::Ja) => "情報",
            (Severity::Warning, Locale::Ja) => "警告",
            (Severity::Critical, Locale::Ja) => "危険",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticFinding {
    pub severity: Severity,
    pub source: String,
    pub title: String,
    pub detail: String,
    pub suggestion: String,
    pub related_metrics: Vec<(String, String)>,
    pub runbook_url: Option<String>,
}

/// Match runbook URLs to a finding based on case-insensitive pattern matching
/// against the finding's title and source.
fn apply_runbook(finding: &mut DiagnosticFinding, runbooks: &[RunbookConfig]) {
    let title_lower = finding.title.to_lowercase();
    let source_lower = finding.source.to_lowercase();
    for rb in runbooks {
        let pat = rb.pattern.to_lowercase();
        if title_lower.contains(&pat) || source_lower.contains(&pat) {
            finding.runbook_url = Some(rb.url.clone());
            return;
        }
    }
}

/// Run all diagnostic checks against a snapshot.
pub fn analyze(
    snapshot: &Snapshot,
    locale: Locale,
    runbooks: &[RunbookConfig],
) -> Vec<DiagnosticFinding> {
    let mut findings = Vec::new();

    check_memory(&mut findings, snapshot, locale);
    check_load(&mut findings, snapshot, locale);
    check_swap(&mut findings, snapshot, locale);
    check_pressure(&mut findings, snapshot, locale);
    check_processes(&mut findings, snapshot, locale);
    check_network(&mut findings, snapshot, locale);
    check_disk(&mut findings, snapshot, locale);
    check_temperature(&mut findings, snapshot, locale);
    check_fd(&mut findings, snapshot, locale);
    check_dns(&mut findings, snapshot, locale);
    check_conntrack(&mut findings, snapshot, locale);
    check_gpu(&mut findings, snapshot, locale);
    check_systemd(&mut findings, snapshot, locale);
    check_memory_leak(&mut findings, snapshot, locale);
    check_swap_activity(&mut findings, snapshot, locale);
    check_context_switches(&mut findings, snapshot, locale);
    check_oom_kills(&mut findings, snapshot, locale);
    check_network_errors(&mut findings, snapshot, locale);
    check_inode_pressure(&mut findings, snapshot, locale);
    check_uptime(&mut findings, snapshot, locale);
    check_load_trend(&mut findings, snapshot, locale);
    check_tcp_listen(&mut findings, snapshot, locale);
    check_kernel_taint(&mut findings, snapshot, locale);
    check_high_memory_process(&mut findings, snapshot, locale);
    check_ss_orphaned(&mut findings, snapshot, locale);
    check_conntrack_rate(&mut findings, snapshot, locale);
    check_ip_forwarding(&mut findings, snapshot, locale);

    // Apply runbook URLs
    for finding in &mut findings {
        apply_runbook(finding, runbooks);
    }

    // Sort: Critical first, then Warning, then Info
    findings.sort_by(|a, b| {
        let ord = |s: &Severity| match s {
            Severity::Critical => 0,
            Severity::Warning => 1,
            Severity::Info => 2,
        };
        ord(&a.severity).cmp(&ord(&b.severity))
    });

    findings
}

/// Fetch several byte fields from one source in a single pass.
///
/// Each name in `names` is paired with the result of looking it up; missing
/// fields map to `None`. Walking the source's field list once — instead of
/// once per requested field — turns an O(F*K) linear scan (F fields, K
/// requested fields) into O(F+K), which matters for large sources like
/// `vmstat` (~179 fields) and `net/snmp` (~86 fields) when several fields
/// are read together.
///
/// Returns a stack-allocated array of `Option<u64>` sized to `names.len()`,
/// in the same order as `names`. No heap allocation.
fn get_bytes_many<const N: usize>(
    snapshot: &Snapshot,
    source: &str,
    names: [&str; N],
) -> [Option<u64>; N] {
    let mut out: [Option<u64>; N] = [const { None }; N];
    let Some(entry) = snapshot.entries.get(source) else {
        return out;
    };
    let mut remaining = N;
    for f in &entry.fields {
        if remaining == 0 {
            break;
        }
        for i in 0..N {
            if out[i].is_none() && f.name == names[i] {
                if let FieldValue::Bytes(v) = f.value {
                    out[i] = Some(v);
                    remaining -= 1;
                }
                break;
            }
        }
    }
    out
}

fn get_bytes(snapshot: &Snapshot, source: &str, field: &str) -> Option<u64> {
    snapshot
        .entries
        .get(source)?
        .fields
        .iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Bytes(v) => Some(v),
            _ => None,
        })
}

fn get_float(snapshot: &Snapshot, source: &str, field: &str) -> Option<f64> {
    snapshot
        .entries
        .get(source)?
        .fields
        .iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Float(v) => Some(v),
            FieldValue::Integer(v) => Some(v as f64),
            FieldValue::Duration(v) => Some(v),
            _ => None,
        })
}

fn get_integer(snapshot: &Snapshot, source: &str, field: &str) -> Option<i64> {
    snapshot
        .entries
        .get(source)?
        .fields
        .iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Integer(v) => Some(v),
            _ => None,
        })
}

/// Fetch several integer fields from one source in a single pass.
///
/// Each name in `names` is paired with the result of looking it up; missing
/// fields map to `None`. Walking the source's field list once — instead of
/// once per requested field — turns an O(F*K) linear scan (F fields, K
/// requested fields) into O(F+K), which matters for large sources like
/// `vmstat` (~179 fields) and `net/snmp` (~86 fields) when several fields
/// are read together.
///
/// Returns a stack-allocated array of `Option<i64>` sized to `names.len()`,
/// in the same order as `names`. No heap allocation.
fn get_integers<const N: usize>(
    snapshot: &Snapshot,
    source: &str,
    names: [&str; N],
) -> [Option<i64>; N] {
    let mut out: [Option<i64>; N] = [const { None }; N];
    let Some(entry) = snapshot.entries.get(source) else {
        return out;
    };
    let mut remaining = N;
    for f in &entry.fields {
        if remaining == 0 {
            break;
        }
        for i in 0..N {
            if out[i].is_none() && f.name == names[i] {
                if let FieldValue::Integer(v) = f.value {
                    out[i] = Some(v);
                    remaining -= 1;
                }
                break;
            }
        }
    }
    out
}

fn get_table_row_count(snapshot: &Snapshot, source: &str, field: &str) -> Option<usize> {
    snapshot
        .entries
        .get(source)?
        .fields
        .iter()
        .find(|f| f.name == field)
        .and_then(|f| match &f.value {
            FieldValue::Table(rows) => Some(rows.len()),
            _ => None,
        })
}

fn get_table_col_values(snapshot: &Snapshot, source: &str, field: &str, col: usize) -> Vec<String> {
    snapshot
        .entries
        .get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field))
        .map(|f| match &f.value {
            FieldValue::Table(rows) => rows.iter().filter_map(|r| r.get(col).cloned()).collect(),
            _ => vec![],
        })
        .unwrap_or_default()
}

fn get_table_rows<'a>(
    snapshot: &'a Snapshot,
    source: &str,
    field: &str,
) -> Option<&'a Vec<Vec<String>>> {
    snapshot
        .entries
        .get(source)?
        .fields
        .iter()
        .find(|f| f.name == field)
        .and_then(|f| match &f.value {
            FieldValue::Table(rows) => Some(rows),
            _ => None,
        })
}

fn check_memory(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let vals = get_bytes_many(snap, "meminfo", ["MemTotal", "MemAvailable"]);
    let total = match vals[0] {
        Some(v) => v,
        None => return,
    };
    let available = vals[1].unwrap_or(total);

    if total == 0 {
        return;
    }
    let pct_available = (available as f64 / total as f64) * 100.0;

    if pct_available < 10.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: format!("メモリ危機: 残り {:.1}%", pct_available),
                detail: format!(
                    "MemAvailable ({}) が MemTotal ({}) の 10% 未満。OOM Killer が発動する可能性。",
                    format_bytes(available),
                    format_bytes(total)
                ),
                suggestion: "ps aux --sort=-rss | head でメモリ消費の多いプロセスを特定".into(),
                related_metrics: vec![
                    ("meminfo".into(), "Cached".into()),
                    ("vmstat".into(), "pgfault".into()),
                    ("pressure".into(), "memory_some_avg10".into()),
                ],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: format!("Memory critical: {:.1}% available", pct_available),
                detail: format!(
                    "MemAvailable ({}) is below 10% of MemTotal ({}). OOM Killer may activate.",
                    format_bytes(available),
                    format_bytes(total)
                ),
                suggestion: "Run: ps aux --sort=-rss | head to find top memory consumers".into(),
                related_metrics: vec![
                    ("meminfo".into(), "Cached".into()),
                    ("vmstat".into(), "pgfault".into()),
                    ("pressure".into(), "memory_some_avg10".into()),
                ],
                runbook_url: None,
            }
        });
    } else if pct_available < 20.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "meminfo".into(),
                title: format!("メモリ残量低下: {:.1}%", pct_available),
                detail: format!(
                    "MemAvailable ({}) が 20% を下回っている。メモリリークの可能性を確認。",
                    format_bytes(available)
                ),
                suggestion: "RSS が増え続けているプロセスがないか確認".into(),
                related_metrics: vec![
                    ("meminfo".into(), "Cached".into()),
                    ("vmstat".into(), "pgfault".into()),
                    ("pressure".into(), "memory_some_avg10".into()),
                ],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "meminfo".into(),
                title: format!("Memory low: {:.1}% available", pct_available),
                detail: format!(
                    "MemAvailable ({}) is below 20%. Check for memory leaks.",
                    format_bytes(available)
                ),
                suggestion: "Check for processes with growing RSS over time".into(),
                related_metrics: vec![
                    ("meminfo".into(), "Cached".into()),
                    ("vmstat".into(), "pgfault".into()),
                    ("pressure".into(), "memory_some_avg10".into()),
                ],
                runbook_url: None,
            }
        });
    }
}

fn check_load(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let load1 = match get_float(snap, "loadavg", "load1") {
        Some(v) => v,
        None => return,
    };

    // Try to get CPU count
    let cpu_count = get_integer(snap, "stat", "cpu_count")
        .or_else(|| get_integer(snap, "cpuinfo", "cpu_count"))
        .unwrap_or(1) as f64;

    if cpu_count <= 0.0 {
        return;
    }
    let ratio = load1 / cpu_count;

    if ratio > 2.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "loadavg".into(),
                title: format!("CPU 過負荷: load {:.1} (CPU{}個の{:.0}倍)", load1, cpu_count as i64, ratio),
                detail: "プロセスがキューに溜まっている。CPU バウンドか I/O 待ちが大量に発生。".into(),
                suggestion: "pressure の PSI データを確認。CPU圧力が高ければCPU不足、I/O圧力が高ければディスクボトルネック。".into(),
                related_metrics: vec![("stat".into(), "cpu_user".into()), ("pressure".into(), "cpu_some_avg10".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "loadavg".into(),
                title: format!("CPU overloaded: load {:.1} ({:.0}x {} CPUs)", load1, ratio, cpu_count as i64),
                detail: "Processes are queuing. Either CPU-bound or massive I/O wait.".into(),
                suggestion: "Check PSI pressure data. High CPU pressure = need more CPUs. High I/O pressure = disk bottleneck.".into(),
                related_metrics: vec![("stat".into(), "cpu_user".into()), ("pressure".into(), "cpu_some_avg10".into())],
            runbook_url: None,
            }
        });
    } else if ratio > 1.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!(
                    "CPU 飽和: load {:.1} (CPU{}個の{:.0}倍)",
                    load1, cpu_count as i64, ratio
                ),
                detail: "CPU がフル稼働を超えている。レスポンスタイムが劣化している可能性。".into(),
                suggestion: "top で CPU 消費の高いプロセスを確認".into(),
                related_metrics: vec![
                    ("stat".into(), "cpu_user".into()),
                    ("pressure".into(), "cpu_some_avg10".into()),
                ],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!(
                    "CPU saturated: load {:.1} ({:.0}x {} CPUs)",
                    load1, ratio, cpu_count as i64
                ),
                detail: "CPU is beyond full utilization. Response times may be degraded.".into(),
                suggestion: "Run top to identify high-CPU processes".into(),
                related_metrics: vec![
                    ("stat".into(), "cpu_user".into()),
                    ("pressure".into(), "cpu_some_avg10".into()),
                ],
                runbook_url: None,
            }
        });
    }
}

fn check_swap(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let swap_total = get_bytes(snap, "meminfo", "SwapTotal").unwrap_or(0);
    let swap_free = get_bytes(snap, "meminfo", "SwapFree").unwrap_or(0);

    if swap_total == 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "meminfo".into(),
                title: "スワップ未設定".into(),
                detail: "スワップが設定されていない。メモリ不足時は OOM Killer が唯一の安全装置。"
                    .into(),
                suggestion: "本番環境では RAM の 1-2 倍のスワップを設定することを推奨".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "meminfo".into(),
                title: "No swap configured".into(),
                detail: "No swap space is configured. OOM Killer is the only safety net.".into(),
                suggestion: "Consider configuring swap at 1-2x RAM for production".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        });
    } else if swap_free == 0 && swap_total > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: "スワップ枯渇".into(),
                detail: format!(
                    "スワップ {}  が全て使用済み。次のメモリ確保で OOM Killer が発動する。",
                    format_bytes(swap_total)
                ),
                suggestion: "即座にメモリ消費の高いプロセスを調査".into(),
                related_metrics: vec![
                    ("meminfo".into(), "MemAvailable".into()),
                    ("vmstat".into(), "pswpout".into()),
                ],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: "Swap exhausted".into(),
                detail: format!(
                    "All {} of swap is used. Next allocation may trigger OOM Killer.",
                    format_bytes(swap_total)
                ),
                suggestion: "Investigate high-memory processes immediately".into(),
                related_metrics: vec![
                    ("meminfo".into(), "MemAvailable".into()),
                    ("vmstat".into(), "pswpout".into()),
                ],
                runbook_url: None,
            }
        });
    }
}

fn check_pressure(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    for (field, label_en, label_ja) in [
        ("cpu_some_avg10", "CPU", "CPU"),
        ("memory_some_avg10", "Memory", "メモリ"),
        ("io_some_avg10", "I/O", "I/O"),
    ] {
        if let Some(val) = get_float(snap, "pressure", field) {
            let (sev, threshold) = if val > 50.0 {
                (Severity::Critical, 50)
            } else if val > 25.0 {
                (Severity::Warning, 25)
            } else {
                continue;
            };

            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: sev,
                    source: "pressure".into(),
                    title: format!("{} 圧力: {:.1}% ({}%超)", label_ja, val, threshold),
                    detail: format!("タスクの {:.1}% の時間が {} 待ちで停滞中。", val, label_ja),
                    suggestion: match label_en {
                        "CPU" => "CPU バウンドなプロセスを特定。top で %CPU を確認。".into(),
                        "Memory" => "MemAvailable を確認。メモリリークの可能性。".into(),
                        _ => "diskstats の await を確認。SSD アップグレードを検討。".into(),
                    },
                    related_metrics: vec![],
                    runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: sev,
                    source: "pressure".into(),
                    title: format!("{} pressure: {:.1}% (>{}%)", label_en, val, threshold),
                    detail: format!("{:.1}% of time, tasks are stalled on {}.", val, label_en),
                    suggestion: match label_en {
                        "CPU" => "Identify CPU-bound processes with top.".into(),
                        "Memory" => "Check MemAvailable. Possible memory leak.".into(),
                        _ => "Check diskstats await. Consider SSD upgrade.".into(),
                    },
                    related_metrics: vec![],
                    runbook_url: None,
                }
            });
        }
    }
}

fn check_processes(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    // Check for zombies in process table
    let states = get_table_col_values(snap, "processes", "processes", 2);
    let zombie_count = states.iter().filter(|s| s.starts_with('Z')).count();

    if zombie_count > 5 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("ゾンビプロセス {}個 検出", zombie_count),
                detail: "親プロセスが wait() を呼んでいない。ゾンビ自体は無害だが、親のバグを示す。".into(),
                suggestion: "ゾンビの PPID を調べて親プロセスを特定".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("{} zombie processes detected", zombie_count),
                detail: "Parent process not calling wait(). Zombies are harmless but indicate buggy parents.".into(),
                suggestion: "Check PPID of zombies to identify the parent process".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }

    let d_count = states.iter().filter(|s| s.starts_with('D')).count();
    if d_count > 3 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("D状態プロセス {}個 (割り込み不可I/O待ち)", d_count),
                detail: "NFS ハング、ディスク障害、またはカーネルドライバの問題の可能性。SIGKILL でも殺せない。".into(),
                suggestion: "dmesg でストレージ関連のエラーを確認".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("{} processes in D-state (uninterruptible I/O)", d_count),
                detail: "Possible NFS hang, disk failure, or kernel driver issue. Cannot be killed with SIGKILL.".into(),
                suggestion: "Check dmesg for storage-related errors".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_network(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    // Check TCP connection states
    let states = get_table_col_values(snap, "net/tcp", "connections", 2);
    let syn_sent = states.iter().filter(|s| *s == "SYN_SENT").count();
    let close_wait = states.iter().filter(|s| *s == "CLOSE_WAIT").count();
    let time_wait = states.iter().filter(|s| *s == "TIME_WAIT").count();

    if syn_sent > 10 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("SYN_SENT {}件 — 接続先が応答していない", syn_sent),
                detail: "接続先がダウン、ファイアウォールでドロップ、または DNS 解決が遅い可能性。"
                    .into(),
                suggestion: "接続先の IP/ポートを確認。ping や telnet でリーチャビリティをテスト。"
                    .into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("{} SYN_SENT — targets not responding", syn_sent),
                detail: "Target host may be down, firewall dropping packets, or slow DNS.".into(),
                suggestion: "Check target IPs/ports. Test reachability with ping/telnet.".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        });
    }

    if close_wait > 20 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("CLOSE_WAIT {}件 — ソケット FD リーク", close_wait),
                detail: "アプリケーションがソケットを close() していない。相手は切断済み。典型的な FD リーク。".into(),
                suggestion: "lsof でどのプロセスが CLOSE_WAIT のソケットを保持しているか確認".into(),
                related_metrics: vec![("file-nr".into(), "fd_usage_pct".into()), ("ss".into(), "tcp_orphaned".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("{} CLOSE_WAIT — socket FD leak", close_wait),
                detail: "Application not closing sockets. Remote end already closed. Classic FD leak.".into(),
                suggestion: "Use lsof to find which process holds CLOSE_WAIT sockets".into(),
                related_metrics: vec![("file-nr".into(), "fd_usage_pct".into()), ("ss".into(), "tcp_orphaned".into())],
            runbook_url: None,
            }
        });
    }

    if time_wait > 5000 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/tcp".into(),
                title: format!("TIME_WAIT {}件", time_wait),
                detail: "短命な接続が大量。エフェメラルポートが枯渇する可能性。".into(),
                suggestion: "net.ipv4.tcp_tw_reuse = 1 の設定を検討".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/tcp".into(),
                title: format!("{} TIME_WAIT connections", time_wait),
                detail: "Many short-lived connections. Ephemeral ports may be exhausted.".into(),
                suggestion: "Consider setting net.ipv4.tcp_tw_reuse = 1".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        });
    }
}

fn check_disk(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let use_pct = match get_float(snap, "df", "root_use_pct") {
        Some(v) => v,
        None => return,
    };

    if use_pct > 90.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "df".into(),
                title: format!("ディスク使用率 {:.1}% — 残り僅か", use_pct),
                detail: "ルートファイルシステムの使用率が 90% を超過。ログ書き込み失敗やサービス停止の危険。".into(),
                suggestion: "du -sh /* で大きなディレクトリを特定。journalctl --vacuum-size=500M でログ削減。".into(),
                related_metrics: vec![("diskstats".into(), "devices".into()), ("pressure".into(), "io_some_avg10".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "df".into(),
                title: format!("Disk usage {:.1}% — critically low space", use_pct),
                detail: "Root filesystem usage exceeds 90%. Log writes may fail, services may crash.".into(),
                suggestion: "Run: du -sh /* to find large directories. journalctl --vacuum-size=500M to trim logs.".into(),
                related_metrics: vec![("diskstats".into(), "devices".into()), ("pressure".into(), "io_some_avg10".into())],
            runbook_url: None,
            }
        });
    } else if use_pct > 80.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "df".into(),
                title: format!("ディスク使用率 {:.1}% — 注意", use_pct),
                detail: "ルートファイルシステムの使用率が 80% を超過。容量計画を検討すべき。"
                    .into(),
                suggestion: "不要なログやキャッシュの削除を検討。df -h で各パーティションを確認。"
                    .into(),
                related_metrics: vec![
                    ("diskstats".into(), "devices".into()),
                    ("pressure".into(), "io_some_avg10".into()),
                ],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "df".into(),
                title: format!("Disk usage {:.1}% — getting full", use_pct),
                detail: "Root filesystem usage exceeds 80%. Plan capacity expansion.".into(),
                suggestion: "Clean up old logs and caches. Run: df -h to check all partitions."
                    .into(),
                related_metrics: vec![
                    ("diskstats".into(), "devices".into()),
                    ("pressure".into(), "io_some_avg10".into()),
                ],
                runbook_url: None,
            }
        });
    }
}

fn check_temperature(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let max_temp = match get_float(snap, "thermal", "max_temp") {
        Some(v) => v,
        None => return,
    };

    if max_temp > 90.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "thermal".into(),
                title: format!("CPU温度 {:.0}°C — 過熱", max_temp),
                detail: "CPU温度が90°Cを超過。サーマルスロットリングが発生し、パフォーマンスが大幅に低下。ハードウェア損傷の危険。".into(),
                suggestion: "冷却システムを確認。ファンの動作、サーマルペーストの状態、エアフローを点検。".into(),
                related_metrics: vec![("stat".into(), "cpu_user".into()), ("loadavg".into(), "load1".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "thermal".into(),
                title: format!("CPU temperature {:.0}°C — overheating", max_temp),
                detail: "CPU temperature exceeds 90°C. Thermal throttling is active, severely degrading performance. Risk of hardware damage.".into(),
                suggestion: "Check cooling system: fan operation, thermal paste, airflow.".into(),
                related_metrics: vec![("stat".into(), "cpu_user".into()), ("loadavg".into(), "load1".into())],
            runbook_url: None,
            }
        });
    } else if max_temp > 75.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "thermal".into(),
                title: format!("CPU温度 {:.0}°C — 高温", max_temp),
                detail: "CPU温度が75°Cを超過。スロットリング手前。負荷が高い状態が続くと更に上昇。".into(),
                suggestion: "冷却の改善を検討。CPU使用率を確認し、負荷を分散。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "thermal".into(),
                title: format!("CPU temperature {:.0}°C — running hot", max_temp),
                detail: "CPU temperature exceeds 75°C. Near throttling threshold. May rise further under sustained load.".into(),
                suggestion: "Consider improving cooling. Check CPU utilization and distribute load.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_fd(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let usage_pct = match get_float(snap, "file-nr", "fd_usage_pct") {
        Some(v) => v,
        None => return,
    };

    if usage_pct > 80.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "file-nr".into(),
                title: format!("FD使用率 {:.1}% — ファイルディスクリプタ枯渇の危険", usage_pct),
                detail: "システム全体のファイルディスクリプタ使用率が80%を超過。枯渇するとプロセスがファイルやソケットを開けなくなる。".into(),
                suggestion: "lsof | wc -l で開いているFD数を確認。FDリークのあるプロセスを特定。sysctl fs.file-max で上限の引き上げを検討。".into(),
                related_metrics: vec![("net/tcp".into(), "connections".into()), ("processes".into(), "processes".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "file-nr".into(),
                title: format!("FD usage {:.1}% — file descriptor exhaustion risk", usage_pct),
                detail: "System-wide file descriptor usage exceeds 80%. Exhaustion prevents processes from opening files or sockets.".into(),
                suggestion: "Run: lsof | wc -l to check open FDs. Find leaking processes. Consider raising sysctl fs.file-max.".into(),
                related_metrics: vec![("net/tcp".into(), "connections".into()), ("processes".into(), "processes".into())],
            runbook_url: None,
            }
        });
    }
}

fn check_dns(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let entry = match snap.entries.get("dns") {
        Some(e) => e,
        None => return,
    };

    // Check if nameservers table is empty
    let has_nameservers = entry
        .fields
        .iter()
        .find(|f| f.name == "nameservers")
        .map(|f| match &f.value {
            FieldValue::Table(rows) => !rows.is_empty(),
            _ => false,
        })
        .unwrap_or(false);

    if !has_nameservers {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "dns".into(),
                title: "DNSネームサーバ未設定".into(),
                detail: "/etc/resolv.conf にネームサーバが設定されていない。名前解決が失敗する。"
                    .into(),
                suggestion: "resolv.conf にネームサーバを追加 (例: nameserver 8.8.8.8)".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "dns".into(),
                title: "No DNS nameservers configured".into(),
                detail:
                    "No nameserver entries found in /etc/resolv.conf. DNS resolution will fail."
                        .into(),
                suggestion: "Add a nameserver to resolv.conf (e.g., nameserver 8.8.8.8)".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        });
    }
}

fn check_conntrack(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let usage_pct = match get_float(snap, "conntrack", "usage_pct") {
        Some(v) => v,
        None => return,
    };

    if usage_pct > 80.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "conntrack".into(),
                title: format!("conntrack使用率 {:.1}% — テーブル枯渇の危険", usage_pct),
                detail: "コネクション追跡テーブルの使用率が80%を超過。枯渇すると新規接続がドロップされる。".into(),
                suggestion: "sysctl net.nf_conntrack_max を引き上げるか、不要な追跡を除外 (NOTRACK)".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "conntrack".into(),
                title: format!("Conntrack usage {:.1}% — table exhaustion risk", usage_pct),
                detail: "Connection tracking table usage exceeds 80%. Exhaustion will drop new connections.".into(),
                suggestion: "Increase sysctl net.nf_conntrack_max or add NOTRACK rules for high-traffic flows".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_gpu(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    if let Some(gpu_temp) = get_float(snap, "gpu", "gpu_temp") {
        if gpu_temp > 90.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Critical,
                    source: "gpu".into(),
                    title: format!("GPU温度 {:.0}°C — 過熱", gpu_temp),
                    detail: "GPU温度が90°Cを超過。サーマルスロットリングが発生し、パフォーマンスが大幅に低下。ハードウェア損傷の危険。".into(),
                    suggestion: "GPU冷却システムを確認。ファンの動作、ケース内エアフローを点検。nvidia-smi で電力制限の引き下げも検討。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Critical,
                    source: "gpu".into(),
                    title: format!("GPU temperature {:.0}°C — overheating", gpu_temp),
                    detail: "GPU temperature exceeds 90°C. Thermal throttling active, severely degrading performance. Risk of hardware damage.".into(),
                    suggestion: "Check GPU cooling: fan operation, case airflow. Consider lowering power limit with nvidia-smi.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        } else if gpu_temp > 80.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "gpu".into(),
                    title: format!("GPU温度 {:.0}°C — 高温", gpu_temp),
                    detail: "GPU温度が80°Cを超過。スロットリング手前。負荷が高い状態が続くと更に上昇。".into(),
                    suggestion: "GPU冷却の改善を検討。ファンカーブの調整やケースエアフローの改善。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "gpu".into(),
                    title: format!("GPU temperature {:.0}°C — running hot", gpu_temp),
                    detail: "GPU temperature exceeds 80°C. Near throttling threshold. May rise further under sustained load.".into(),
                    suggestion: "Consider improving GPU cooling. Adjust fan curve or improve case airflow.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }

    if let Some(gpu_util) = get_float(snap, "gpu", "gpu_util") {
        if gpu_util > 95.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "gpu".into(),
                    title: format!("GPU使用率 {:.1}% — 飽和状態", gpu_util),
                    detail: "GPUがほぼ100%使用されている。ワークロードがGPUバウンド。".into(),
                    suggestion: "nvidia-smi でGPUを使用しているプロセスを確認。ワークロードの最適化またはGPUのアップグレードを検討。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "gpu".into(),
                    title: format!("GPU utilization {:.1}% — saturated", gpu_util),
                    detail: "GPU is nearly 100% utilized. Workload is GPU-bound.".into(),
                    suggestion: "Check nvidia-smi for GPU-consuming processes. Consider optimizing workload or upgrading GPU.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }
}

fn check_systemd(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    if let Some(failed_count) = get_integer(snap, "systemd", "failed_count") {
        if failed_count > 0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "systemd".into(),
                    title: format!("systemd: {}個のサービスが失敗", failed_count),
                    detail: "1つ以上のsystemdサービスが失敗状態。システムの機能が一部損なわれている可能性。".into(),
                    suggestion: "systemctl --failed で失敗したサービスを確認。journalctl -u <service> でログを調査。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "systemd".into(),
                    title: format!("systemd: {} failed service(s)", failed_count),
                    detail: "One or more systemd services are in failed state. System functionality may be impaired.".into(),
                    suggestion: "Run: systemctl --failed to see failed services. journalctl -u <service> for logs.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }

    if let Some(entry) = snap.entries.get("systemd") {
        let system_state = entry
            .fields
            .iter()
            .find(|f| f.name == "system_state")
            .and_then(|f| match &f.value {
                FieldValue::Text(s) => Some(s.as_str()),
                _ => None,
            });

        if let Some(state) = system_state {
            if state == "degraded" {
                findings.push(if locale == Locale::Ja {
                    DiagnosticFinding {
                        severity: Severity::Warning,
                        source: "systemd".into(),
                        title: "systemd: システム状態 degraded".into(),
                        detail: "systemdがシステムをdegraded（劣化）と報告。1つ以上のユニットが失敗。".into(),
                        suggestion: "systemctl --failed で詳細を確認。systemctl reset-failed でリセット後、再起動を試行。".into(),
                related_metrics: vec![],
                    runbook_url: None,
                    }
                } else {
                    DiagnosticFinding {
                        severity: Severity::Warning,
                        source: "systemd".into(),
                        title: "systemd: system state degraded".into(),
                        detail: "systemd reports the system as degraded. One or more units have failed.".into(),
                        suggestion: "Run: systemctl --failed for details. systemctl reset-failed then restart affected units.".into(),
                related_metrics: vec![],
                    runbook_url: None,
                    }
                });
            }
        }
    }
}

fn check_memory_leak(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    // Single-pass fetch of the 3 byte fields from meminfo (~54 fields).
    let vals = get_bytes_many(snap, "meminfo", ["MemTotal", "Cached", "Dirty"]);
    let total = match vals[0] {
        Some(v) if v > 0 => v,
        _ => return,
    };

    // Check if Cached is very low relative to MemTotal
    if let Some(cached) = vals[1] {
        let cached_pct = (cached as f64 / total as f64) * 100.0;
        if cached_pct < 5.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "meminfo".into(),
                    title: format!("ページキャッシュ枯渇: Cached {:.1}%", cached_pct),
                    detail: format!("Cached ({}) が MemTotal ({}) の 5% 未満。ページキャッシュが追い出されており、I/O 性能が悪化。",
                        format_bytes(cached), format_bytes(total)),
                    suggestion: "メモリ消費の多いプロセスを調査。アプリケーションがメモリを大量に確保している可能性。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "meminfo".into(),
                    title: format!("Page cache starved: Cached {:.1}%", cached_pct),
                    detail: format!("Cached ({}) is below 5% of MemTotal ({}). Page cache is being evicted, degrading I/O performance.",
                        format_bytes(cached), format_bytes(total)),
                    suggestion: "Investigate high-memory processes. Applications may be hoarding memory.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }

    // Check for write storm (Dirty > 10% of MemTotal)
    if let Some(dirty) = vals[2] {
        let dirty_pct = (dirty as f64 / total as f64) * 100.0;
        if dirty_pct > 10.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "meminfo".into(),
                    title: format!("書き込みストーム: Dirty {:.1}%", dirty_pct),
                    detail: format!("Dirty ページ ({}) が MemTotal の 10% を超過。大量のデータがディスクへの書き込み待ち。",
                        format_bytes(dirty)),
                    suggestion: "I/O 負荷の高いプロセスを特定。iotop で調査。vm.dirty_ratio の調整も検討。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "meminfo".into(),
                    title: format!("Write storm: Dirty {:.1}%", dirty_pct),
                    detail: format!("Dirty pages ({}) exceed 10% of MemTotal. Large amount of data pending disk write.",
                        format_bytes(dirty)),
                    suggestion: "Identify I/O-heavy processes with iotop. Consider tuning vm.dirty_ratio.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }
}

fn check_swap_activity(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    if snap.entries.get("vmstat").is_none() {
        return;
    }

    // Single-pass fetch from vmstat (~179 fields).
    let vals = get_integers(snap, "vmstat", ["pswpin", "pswpout"]);
    let pswpin = vals[0].unwrap_or(0);
    let pswpout = vals[1].unwrap_or(0);

    if pswpin > 0 || pswpout > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "vmstat".into(),
                title: format!("スワップ活動検出: in={} out={} ページ", pswpin, pswpout),
                detail: "スワップイン/アウトが発生している。メモリ不足によりディスクとの間でページが移動中。パフォーマンスが劣化。".into(),
                suggestion: "メモリ消費量の多いプロセスを確認。スワップ使用量と MemAvailable を監視。".into(),
                related_metrics: vec![("meminfo".into(), "MemAvailable".into()), ("meminfo".into(), "SwapFree".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "vmstat".into(),
                title: format!("Swap activity detected: in={} out={} pages", pswpin, pswpout),
                detail: "Pages are being swapped in/out. Memory pressure is causing disk I/O for paging. Performance is degraded.".into(),
                suggestion: "Check top memory-consuming processes. Monitor swap usage and MemAvailable.".into(),
                related_metrics: vec![("meminfo".into(), "MemAvailable".into()), ("meminfo".into(), "SwapFree".into())],
            runbook_url: None,
            }
        });
    }
}

fn check_context_switches(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let procs_running = match get_integer(snap, "stat", "procs_running") {
        Some(v) => v,
        None => return,
    };

    let cpu_count = get_integer(snap, "stat", "cpu_count")
        .or_else(|| get_integer(snap, "cpuinfo", "cpu_count"))
        .or_else(|| get_integer(snap, "interrupts", "cpu_count"))
        .unwrap_or(1);

    if cpu_count <= 0 {
        return;
    }

    if procs_running > cpu_count * 2 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "stat".into(),
                title: format!("実行中プロセス過多: {} (CPU{}個の{:.0}倍)", procs_running, cpu_count, procs_running as f64 / cpu_count as f64),
                detail: "実行可能なプロセスがCPU数の2倍を超過。過剰なコンテキストスイッチが発生し、スループットが低下。".into(),
                suggestion: "top で CPU 消費の高いプロセスを特定。並列度を CPU 数に合わせて調整。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "stat".into(),
                title: format!("Runnable process overload: {} ({:.0}x {} CPUs)", procs_running, procs_running as f64 / cpu_count as f64, cpu_count),
                detail: "Runnable processes exceed 2x CPU count. Excessive context switching is reducing throughput.".into(),
                suggestion: "Identify high-CPU processes with top. Tune parallelism to match CPU count.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_oom_kills(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let oom_kill = match get_integer(snap, "vmstat", "oom_kill") {
        Some(v) => v,
        None => return,
    };

    if oom_kill > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "vmstat".into(),
                title: format!("OOM Killer 発動 {}回", oom_kill),
                detail: "起動以降、OOM Killer がプロセスを強制終了した回数。メモリ不足が過去に発生。".into(),
                suggestion: "dmesg | grep -i oom で詳細を確認。メモリの増設またはワークロードの見直しを検討。".into(),
                related_metrics: vec![("meminfo".into(), "MemAvailable".into()), ("meminfo".into(), "SwapFree".into()), ("vmstat".into(), "pswpout".into())],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "vmstat".into(),
                title: format!("OOM Killer invoked {} time(s)", oom_kill),
                detail: "OOM Killer has killed processes since boot. System experienced memory exhaustion.".into(),
                suggestion: "Run: dmesg | grep -i oom for details. Consider adding memory or reviewing workload.".into(),
                related_metrics: vec![("meminfo".into(), "MemAvailable".into()), ("meminfo".into(), "SwapFree".into()), ("vmstat".into(), "pswpout".into())],
            runbook_url: None,
            }
        });
    }
}

fn check_network_errors(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    if snap.entries.get("net/snmp").is_none() {
        return;
    }

    // Single-pass fetch of all 5 integer fields from net/snmp (~86 fields).
    let vals = get_integers(
        snap,
        "net/snmp",
        [
            "Tcp_InErrs",
            "Tcp_RetransSegs",
            "Tcp_OutSegs",
            "Udp_InErrors",
            "Udp_RcvbufErrors",
        ],
    );
    let tcp_in_errs = vals[0].unwrap_or(0);
    let tcp_retrans = vals[1].unwrap_or(0);
    let tcp_out_segs = vals[2].unwrap_or(0);
    let udp_in_errs = vals[3].unwrap_or(0);
    let udp_rcvbuf_errs = vals[4].unwrap_or(0);

    if tcp_in_errs > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/snmp".into(),
                title: format!("TCPエラー受信: {} セグメント", tcp_in_errs),
                detail: "チェックサムエラー等のある TCP セグメントを受信。NIC障害、不良ケーブル、またはドライバの問題。".into(),
                suggestion: "ethtool -S <iface> でNICレベルのエラーカウンタを確認。ケーブル接続を点検。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/snmp".into(),
                title: format!("TCP input errors: {} segments", tcp_in_errs),
                detail: "TCP segments received with checksum or other errors. Possible NIC failure, bad cable, or driver issue.".into(),
                suggestion: "Check NIC-level counters with ethtool -S <iface>. Inspect cable connections.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }

    if tcp_out_segs > 1000 && tcp_retrans > 0 {
        let retrans_pct = (tcp_retrans as f64 / tcp_out_segs as f64) * 100.0;
        if retrans_pct > 1.0 {
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "net/snmp".into(),
                    title: format!("TCP再送率 {:.2}%", retrans_pct),
                    detail: format!("TCP 再送 {} / 送信 {} — 1%超はパケットロスが深刻。ネットワーク品質に問題。", tcp_retrans, tcp_out_segs),
                    suggestion: "ネットワーク経路を調査。mtr でパケットロスの発生箇所を特定。".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "net/snmp".into(),
                    title: format!("TCP retransmission rate {:.2}%", retrans_pct),
                    detail: format!("TCP retransmits {} / sent {} — above 1% indicates serious packet loss.", tcp_retrans, tcp_out_segs),
                    suggestion: "Investigate network path. Use mtr to locate where packet loss occurs.".into(),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }

    if udp_in_errs > 0 || udp_rcvbuf_errs > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/snmp".into(),
                title: format!("UDPエラー: InErrors={} RcvbufErrors={}", udp_in_errs, udp_rcvbuf_errs),
                detail: "UDP データグラムの配信失敗または受信バッファオーバーフロー。パケットがドロップされている。".into(),
                suggestion: "受信バッファサイズを確認: sysctl net.core.rmem_max。アプリケーションの受信処理速度も確認。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/snmp".into(),
                title: format!("UDP errors: InErrors={} RcvbufErrors={}", udp_in_errs, udp_rcvbuf_errs),
                detail: "UDP datagram delivery failures or receive buffer overflows. Packets are being dropped.".into(),
                suggestion: "Check receive buffer size: sysctl net.core.rmem_max. Review application receive throughput.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_inode_pressure(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let total = match get_bytes(snap, "meminfo", "MemTotal") {
        Some(v) if v > 0 => v,
        _ => return,
    };
    let sreclaimable = match get_bytes(snap, "meminfo", "SReclaimable") {
        Some(v) => v,
        None => return,
    };

    let slab_pct = (sreclaimable as f64 / total as f64) * 100.0;
    if slab_pct > 25.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "meminfo".into(),
                title: format!("スラブキャッシュ肥大: SReclaimable {:.1}%", slab_pct),
                detail: format!("SReclaimable ({}) が MemTotal の 25% 超。dentry/inode キャッシュが大量のメモリを使用。",
                    format_bytes(sreclaimable)),
                suggestion: "ファイルサーバーでは正常。必要なら echo 2 > /proc/sys/vm/drop_caches で手動回収。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "meminfo".into(),
                title: format!("Large slab cache: SReclaimable {:.1}%", slab_pct),
                detail: format!("SReclaimable ({}) exceeds 25% of MemTotal. Dentry/inode caches are consuming significant memory.",
                    format_bytes(sreclaimable)),
                suggestion: "Normal on fileservers. If needed: echo 2 > /proc/sys/vm/drop_caches to reclaim manually.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_uptime(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let uptime_secs = match get_float(snap, "uptime", "uptime") {
        Some(v) => v,
        None => return,
    };

    if uptime_secs < 300.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "uptime".into(),
                title: format!("直近の再起動: {:.0}秒前", uptime_secs),
                detail: "システムが5分以内に再起動された。予期しない再起動の可能性を確認。".into(),
                suggestion: "last reboot と dmesg で再起動の原因を確認。panic やハードウェア障害のログを調査。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "uptime".into(),
                title: format!("System recently rebooted: {:.0}s ago", uptime_secs),
                detail: "System was rebooted within the last 5 minutes. Check for unexpected restarts.".into(),
                suggestion: "Run: last reboot and dmesg to investigate cause. Check for panic or hardware failure logs.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_load_trend(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let load1 = match get_float(snap, "loadavg", "load_1min") {
        Some(v) => v,
        None => return,
    };
    let load15 = match get_float(snap, "loadavg", "load_15min") {
        Some(v) if v > 0.0 => v,
        _ => return,
    };

    let ratio = load1 / load15;

    if ratio > 2.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!("負荷急上昇: load1={:.2} load15={:.2} ({:.1}倍)", load1, load15, ratio),
                detail: "1分間のロードが15分間の2倍を超過。負荷が急激に上昇中。".into(),
                suggestion: "直近で開始されたプロセスやジョブを確認。top でリアルタイムの CPU 状況を監視。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!("Load spiking: load1={:.2} load15={:.2} ({:.1}x)", load1, load15, ratio),
                detail: "1-minute load is more than 2x the 15-minute average. Load is increasing rapidly.".into(),
                suggestion: "Check for recently started processes or jobs. Monitor with top in real time.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    } else if ratio < 0.5 && load15 > 1.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "loadavg".into(),
                title: format!("負荷回復中: load1={:.2} load15={:.2}", load1, load15),
                detail: "1分間のロードが15分間の半分未満。以前の高負荷から回復中。".into(),
                suggestion: "状況を監視。回復が継続するか確認。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "loadavg".into(),
                title: format!("Load recovering: load1={:.2} load15={:.2}", load1, load15),
                detail: "1-minute load is less than half the 15-minute average. System is recovering from prior load.".into(),
                suggestion: "Monitor the trend. Confirm recovery continues.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_tcp_listen(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let rows = match get_table_rows(snap, "net/tcp", "connections") {
        Some(r) => r,
        None => return,
    };

    // Table columns: proto, local_addr, remote_addr, state, uid
    let well_known_ports: &[(u16, &str)] = &[
        (22, "SSH"),
        (80, "HTTP"),
        (443, "HTTPS"),
        (3306, "MySQL"),
        (5432, "PostgreSQL"),
        (6379, "Redis"),
        (8080, "HTTP-alt"),
        (8443, "HTTPS-alt"),
        (3000, "Dev server"),
        (5000, "Dev server"),
        (9090, "Prometheus"),
        (27017, "MongoDB"),
    ];

    let mut listening_services: Vec<String> = Vec::new();
    for row in rows {
        if row.len() >= 4 && row[3] == "LISTEN" {
            let local_addr = &row[1];
            if let Some(port_str) = local_addr.rsplit(':').next() {
                if let Ok(port) = port_str.parse::<u16>() {
                    for (known_port, name) in well_known_ports {
                        if port == *known_port {
                            listening_services.push(format!("{}({})", name, port));
                            break;
                        }
                    }
                }
            }
        }
    }

    listening_services.sort();
    listening_services.dedup();

    if !listening_services.is_empty() {
        let services = listening_services.join(", ");
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/tcp".into(),
                title: format!("リスン中のサービス: {}", services),
                detail: "よく知られたポートでリスンしているサービスを検出。".into(),
                suggestion: "想定外のサービスがないか確認。不要なサービスは停止を検討。".into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/tcp".into(),
                title: format!("Listening services: {}", services),
                detail: "Detected services listening on well-known ports.".into(),
                suggestion:
                    "Verify all listed services are expected. Consider stopping unnecessary ones."
                        .into(),
                related_metrics: vec![],
                runbook_url: None,
            }
        });
    }
}

fn check_kernel_taint(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let tainted = match get_integer(snap, "sysctl", "kernel_tainted")
        .or_else(|| get_integer(snap, "kernel", "tainted"))
    {
        Some(v) => v,
        None => return,
    };

    if tainted != 0 {
        let mut reasons = Vec::new();
        if tainted & 1 != 0 {
            reasons.push("proprietary module");
        }
        if tainted & 2 != 0 {
            reasons.push("module force-loaded");
        }
        if tainted & 4 != 0 {
            reasons.push("SMP unsafe module");
        }
        if tainted & 8 != 0 {
            reasons.push("module force-unloaded");
        }
        if tainted & 16 != 0 {
            reasons.push("MCE (hardware error)");
        }
        if tainted & 32 != 0 {
            reasons.push("bad page found");
        }
        if tainted & 64 != 0 {
            reasons.push("user request");
        }
        if tainted & 128 != 0 {
            reasons.push("kernel died/OOPS");
        }
        if tainted & 256 != 0 {
            reasons.push("ACPI overridden");
        }
        if tainted & 512 != 0 {
            reasons.push("kernel warning");
        }
        if tainted & 1024 != 0 {
            reasons.push("staging driver");
        }

        let detail_str = if reasons.is_empty() {
            format!("tainted={}", tainted)
        } else {
            reasons.join(", ")
        };

        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "kernel".into(),
                title: format!("カーネル汚染フラグ: {}", tainted),
                detail: format!("カーネルが汚染状態: {}。カーネルのバグレポートが受理されない可能性。", detail_str),
                suggestion: "lsmod でロードされたモジュールを確認。プロプライエタリモジュール (nvidia 等) は一般的。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "kernel".into(),
                title: format!("Kernel tainted: {}", tainted),
                detail: format!("Kernel is tainted: {}. Kernel bug reports may not be accepted upstream.", detail_str),
                suggestion: "Check loaded modules with lsmod. Proprietary modules (e.g., nvidia) are common causes.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_high_memory_process(
    findings: &mut Vec<DiagnosticFinding>,
    snap: &Snapshot,
    locale: Locale,
) {
    let total = match get_bytes(snap, "meminfo", "MemTotal") {
        Some(v) if v > 0 => v,
        _ => return,
    };

    let rows = match get_table_rows(snap, "processes", "processes") {
        Some(r) => r,
        None => return,
    };

    // Process table columns: PID, name, state, RSS, threads, UID
    let threshold = total / 2; // 50% of MemTotal

    for row in rows {
        if row.len() < 4 {
            continue;
        }
        let rss_bytes = parse_formatted_bytes(&row[3]);
        if rss_bytes > threshold {
            let pct = (rss_bytes as f64 / total as f64) * 100.0;
            let pid = &row[0];
            let name = &row[1];
            findings.push(if locale == Locale::Ja {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "processes".into(),
                    title: format!("高メモリプロセス: {} (PID {}) — RSS {:.1}%", name, pid, pct),
                    detail: format!("単一プロセスが MemTotal の 50% 以上 ({}) を使用。メモリリークまたは設定の問題の可能性。",
                        row[3]),
                    suggestion: format!("pmap -x {} でメモリマップを確認。RSS が増え続けていないか監視。", pid),
                related_metrics: vec![],
                runbook_url: None,
                }
            } else {
                DiagnosticFinding {
                    severity: Severity::Warning,
                    source: "processes".into(),
                    title: format!("High-memory process: {} (PID {}) — RSS {:.1}%", name, pid, pct),
                    detail: format!("Single process using over 50% of MemTotal ({}). Possible memory leak or misconfiguration.",
                        row[3]),
                    suggestion: format!("Run: pmap -x {} to inspect memory map. Monitor if RSS keeps growing.", pid),
                related_metrics: vec![],
                runbook_url: None,
                }
            });
        }
    }
}

fn check_ss_orphaned(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let orphaned = match get_integer(snap, "ss", "tcp_orphaned") {
        Some(v) => v,
        None => return,
    };

    if orphaned > 100 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "ss".into(),
                title: format!("孤立TCPソケット: {}個", orphaned),
                detail: "プロセスに属さない TCP ソケットが100個以上。接続のクリーンアップに問題がある。".into(),
                suggestion: "net.ipv4.tcp_max_orphans を確認。アプリケーションのソケットクローズ処理を調査。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "ss".into(),
                title: format!("Orphaned TCP sockets: {}", orphaned),
                detail: "Over 100 TCP sockets not attached to any process. Connection cleanup issues.".into(),
                suggestion: "Check net.ipv4.tcp_max_orphans. Investigate application socket close handling.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_conntrack_rate(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let usage_pct = match get_float(snap, "conntrack", "usage_pct") {
        Some(v) => v,
        None => return,
    };

    // Only fire at >50% (the existing check_conntrack fires at >80%)
    if usage_pct > 50.0 && usage_pct <= 80.0 {
        let max = get_integer(snap, "conntrack", "conntrack_max").unwrap_or(0);
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "conntrack".into(),
                title: format!("conntrack テーブル {:.1}% 使用", usage_pct),
                detail: format!("コネクション追跡テーブルが半分以上使用中 (max={})。高トラフィック時に枯渇する可能性。", max),
                suggestion: format!("sysctl net.nf_conntrack_max の引き上げを検討 (現在: {})。nf_conntrack_tcp_timeout_* の短縮も効果的。", max),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "conntrack".into(),
                title: format!("Conntrack table {:.1}% used", usage_pct),
                detail: format!("Connection tracking table is over half full (max={}). May exhaust under high traffic.", max),
                suggestion: format!("Consider increasing sysctl net.nf_conntrack_max (currently: {}). Shortening nf_conntrack_tcp_timeout_* values also helps.", max),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

fn check_ip_forwarding(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    // Ip_Forwarding: 1 = forwarding enabled (gateway), 2 = forwarding disabled (host-only)
    let forwarding = match get_integer(snap, "net/snmp", "Ip_Forwarding") {
        Some(v) => v,
        None => return,
    };

    if forwarding == 1 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/snmp".into(),
                title: "IP転送が有効".into(),
                detail: "net.ipv4.ip_forward = 1。このホストはルーター/NATゲートウェイとして動作中。".into(),
                suggestion: "ルーター/コンテナホストとして意図的であれば問題なし。通常のサーバーでは無効にすることを推奨。".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/snmp".into(),
                title: "IP forwarding enabled".into(),
                detail: "net.ipv4.ip_forward = 1. This host is acting as a router/NAT gateway.".into(),
                suggestion: "Expected on routers/container hosts. For regular servers, consider disabling with sysctl net.ipv4.ip_forward=0.".into(),
                related_metrics: vec![],
            runbook_url: None,
            }
        });
    }
}

/// Parse a human-formatted byte string back to raw bytes.
/// Handles formats like "1.5 GiB", "512.0 MiB", "100.0 KiB", "0 B", "-".
fn parse_formatted_bytes(s: &str) -> u64 {
    let s = s.trim();
    if s == "-" || s.is_empty() {
        return 0;
    }
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() < 2 {
        return parts[0].parse().unwrap_or(0);
    }
    let num: f64 = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return 0,
    };
    match parts[1] {
        "GiB" => (num * 1024.0 * 1024.0 * 1024.0) as u64,
        "MiB" => (num * 1024.0 * 1024.0) as u64,
        "KiB" => (num * 1024.0) as u64,
        "B" => num as u64,
        _ => 0,
    }
}

fn format_bytes(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = 1024 * KIB;
    const GIB: u64 = 1024 * MIB;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use crate::i18n::Locale;
    use crate::proc::{Field, FieldValue, ProcEntry, Snapshot};
    use std::collections::BTreeMap;
    use std::time::{Duration, Instant};

    fn field(name: &str, value: FieldValue) -> Field {
        Field {
            name: name.into(),
            value,
            unit: None,
            description: String::new(),
        }
    }

    fn make_snapshot() -> Snapshot {
        let mut entries = BTreeMap::new();

        let mut meminfo_fields: Vec<Field> = (0..47)
            .map(|i| field(&format!("filler_{i}"), FieldValue::Bytes(0)))
            .collect();
        meminfo_fields.extend([
            field("MemTotal", FieldValue::Bytes(16 * 1024 * 1024 * 1024)),
            field("MemFree", FieldValue::Bytes(8 * 1024 * 1024 * 1024)),
            field("MemAvailable", FieldValue::Bytes(12 * 1024 * 1024 * 1024)),
            field("Cached", FieldValue::Bytes(4 * 1024 * 1024 * 1024)),
            field("SwapTotal", FieldValue::Bytes(8 * 1024 * 1024 * 1024)),
            field("SwapFree", FieldValue::Bytes(8 * 1024 * 1024 * 1024)),
            field("Dirty", FieldValue::Bytes(100 * 1024 * 1024)),
            field("SReclaimable", FieldValue::Bytes(200 * 1024 * 1024)),
        ]);
        entries.insert(
            "meminfo".into(),
            ProcEntry {
                source: "meminfo".into(),
                fields: meminfo_fields,
            },
        );

        let loadavg = ProcEntry {
            source: "loadavg".into(),
            fields: vec![
                field("load1", FieldValue::Float(2.5)),
                field("load5", FieldValue::Float(2.0)),
                field("load15", FieldValue::Float(1.8)),
                field("load_1min", FieldValue::Float(2.5)),
                field("load_5min", FieldValue::Float(2.0)),
                field("load_15min", FieldValue::Float(1.8)),
            ],
        };
        entries.insert("loadavg".into(), loadavg);

        let stat = ProcEntry {
            source: "stat".into(),
            fields: vec![
                field("cpu_user", FieldValue::Float(25.0)),
                field("cpu_system", FieldValue::Float(10.0)),
                field("cpu_iowait", FieldValue::Float(5.0)),
                field("cpu_count", FieldValue::Integer(8)),
                field("procs_running", FieldValue::Integer(4)),
            ],
        };
        entries.insert("stat".into(), stat);

        let cpuinfo = ProcEntry {
            source: "cpuinfo".into(),
            fields: vec![field("cpu_count", FieldValue::Integer(8))],
        };
        entries.insert("cpuinfo".into(), cpuinfo);

        let uptime = ProcEntry {
            source: "uptime".into(),
            fields: vec![
                field("uptime", FieldValue::Duration(86400.0)),
                field("idle", FieldValue::Duration(43200.0)),
            ],
        };
        entries.insert("uptime".into(), uptime);

        let mut pressure_fields: Vec<Field> = (0..21)
            .map(|i| field(&format!("filler_{i}"), FieldValue::Float(0.0)))
            .collect();
        pressure_fields.extend([
            field("cpu_some_avg10", FieldValue::Float(5.0)),
            field("cpu_some_avg60", FieldValue::Float(4.0)),
            field("memory_some_avg10", FieldValue::Float(2.0)),
        ]);
        entries.insert(
            "pressure".into(),
            ProcEntry {
                source: "pressure".into(),
                fields: pressure_fields,
            },
        );

        let mut vmstat_fields: Vec<Field> = (0..174)
            .map(|i| field(&format!("filler_{i}"), FieldValue::Integer(0)))
            .collect();
        vmstat_fields.extend([
            field("oom_kill", FieldValue::Integer(0)),
            field("pswpin", FieldValue::Integer(0)),
            field("pswpout", FieldValue::Integer(0)),
            field("pgfault", FieldValue::Integer(1_000_000)),
            field("pgmajfault", FieldValue::Integer(100)),
        ]);
        entries.insert(
            "vmstat".into(),
            ProcEntry {
                source: "vmstat".into(),
                fields: vmstat_fields,
            },
        );

        let processes = ProcEntry {
            source: "processes".into(),
            fields: vec![
                field("process_count", FieldValue::Integer(150)),
                field(
                    "processes",
                    FieldValue::Table(vec![
                        vec!["1234".into(), "chrome".into(), "500000".into()],
                        vec!["5678".into(), "firefox".into(), "400000".into()],
                    ]),
                ),
            ],
        };
        entries.insert("processes".into(), processes);

        let mut net_snmp_fields: Vec<Field> = (0..80)
            .map(|i| field(&format!("filler_{i}"), FieldValue::Integer(0)))
            .collect();
        net_snmp_fields.extend([
            field("Tcp_InErrs", FieldValue::Integer(0)),
            field("Tcp_OutSegs", FieldValue::Integer(1_000_000)),
            field("Tcp_RetransSegs", FieldValue::Integer(100)),
            field("Udp_InErrors", FieldValue::Integer(0)),
            field("Udp_RcvbufErrors", FieldValue::Integer(0)),
            field("Ip_Forwarding", FieldValue::Integer(0)),
        ]);
        entries.insert(
            "net/snmp".into(),
            ProcEntry {
                source: "net/snmp".into(),
                fields: net_snmp_fields,
            },
        );

        let net_tcp = ProcEntry {
            source: "net/tcp".into(),
            fields: vec![field(
                "connections",
                FieldValue::Table(vec![
                    vec![
                        "0".into(),
                        "0100007F:1F90".into(),
                        "00000000:0000".into(),
                        "0A".into(),
                    ],
                    vec![
                        "1".into(),
                        "0100007F:2328".into(),
                        "0100007F:D431".into(),
                        "01".into(),
                    ],
                ]),
            )],
        };
        entries.insert("net/tcp".into(), net_tcp);

        let ss = ProcEntry {
            source: "ss".into(),
            fields: vec![field("tcp_orphaned", FieldValue::Integer(0))],
        };
        entries.insert("ss".into(), ss);

        let conntrack = ProcEntry {
            source: "conntrack".into(),
            fields: vec![
                field("usage_pct", FieldValue::Float(30.0)),
                field("conntrack_max", FieldValue::Integer(65536)),
            ],
        };
        entries.insert("conntrack".into(), conntrack);

        let df = ProcEntry {
            source: "df".into(),
            fields: vec![field("root_use_pct", FieldValue::Float(45.0))],
        };
        entries.insert("df".into(), df);

        let file_nr = ProcEntry {
            source: "file-nr".into(),
            fields: vec![field("fd_usage_pct", FieldValue::Float(20.0))],
        };
        entries.insert("file-nr".into(), file_nr);

        let thermal = ProcEntry {
            source: "thermal".into(),
            fields: vec![field("max_temp", FieldValue::Float(55.0))],
        };
        entries.insert("thermal".into(), thermal);

        let gpu = ProcEntry {
            source: "gpu".into(),
            fields: vec![
                field("gpu_util", FieldValue::Float(10.0)),
                field("gpu_temp", FieldValue::Float(50.0)),
            ],
        };
        entries.insert("gpu".into(), gpu);

        let dns = ProcEntry {
            source: "dns".into(),
            fields: vec![field(
                "nameservers",
                FieldValue::Table(vec![vec!["8.8.8.8".into()]]),
            )],
        };
        entries.insert("dns".into(), dns);

        let systemd = ProcEntry {
            source: "systemd".into(),
            fields: vec![
                field("failed_count", FieldValue::Integer(0)),
                field("system_state", FieldValue::Text("running".into())),
            ],
        };
        entries.insert("systemd".into(), systemd);

        let kernel = ProcEntry {
            source: "kernel".into(),
            fields: vec![field("tainted", FieldValue::Integer(0))],
        };
        entries.insert("kernel".into(), kernel);

        let sysctl = ProcEntry {
            source: "sysctl".into(),
            fields: vec![field("kernel_tainted", FieldValue::Integer(0))],
        };
        entries.insert("sysctl".into(), sysctl);

        let interrupts = ProcEntry {
            source: "interrupts".into(),
            fields: vec![field("cpu_count", FieldValue::Integer(8))],
        };
        entries.insert("interrupts".into(), interrupts);

        Snapshot {
            timestamp: std::time::UNIX_EPOCH + Duration::from_secs(1_700_000_000),
            entries,
            alerts: vec![],
        }
    }

    #[test]
    #[ignore]
    fn bench_diagnostics_engine() {
        let snap = make_snapshot();
        let runbooks: Vec<RunbookConfig> = vec![];
        const ITERS: u32 = 100_000;
        const WARMUP: u32 = 1_000;

        for _ in 0..WARMUP {
            let _ = analyze(&snap, Locale::En, &runbooks);
        }

        let mut samples_ns: Vec<u128> = Vec::with_capacity(ITERS as usize);
        for _ in 0..ITERS {
            let t0 = Instant::now();
            let findings = analyze(&snap, Locale::En, &runbooks);
            let t1 = Instant::now();
            samples_ns.push(t1.duration_since(t0).as_nanos());
            std::hint::black_box(&findings);
        }
        samples_ns.sort_unstable();
        let median_ns = samples_ns[(ITERS as usize) / 2];
        let mean_ns: u128 = samples_ns.iter().sum::<u128>() / ITERS as u128;
        let p99_ns = samples_ns[(ITERS as usize * 99) / 100];
        eprintln!(
            "bench_diagnostics_engine: iters={ITERS} | median={median_ns}ns mean={mean_ns}ns p99={p99_ns}ns"
        );
    }

    /// Per-check profiler: runs each check_* in isolation over many
    /// iterations to find the hottest checks. Helps target optimization.
    #[test]
    #[ignore]
    fn profile_each_check() {
        let snap = make_snapshot();
        const ITERS: usize = 200_000;
        let mut profile: Vec<(&'static str, u128)> = Vec::new();

        macro_rules! run_check {
            ($name:expr, $func:path) => {{
                let mut findings = Vec::new();
                for _ in 0..1000 {
                    $func(&mut findings, &snap, Locale::En);
                }
                let t0 = Instant::now();
                for _ in 0..ITERS {
                    let mut f = Vec::new();
                    $func(&mut f, &snap, Locale::En);
                    std::hint::black_box(&f);
                }
                let ns = t0.elapsed().as_nanos() / ITERS as u128;
                profile.push(($name, ns));
            }};
        }

        run_check!("check_memory", check_memory);
        run_check!("check_load", check_load);
        run_check!("check_swap", check_swap);
        run_check!("check_pressure", check_pressure);
        run_check!("check_processes", check_processes);
        run_check!("check_network", check_network);
        run_check!("check_disk", check_disk);
        run_check!("check_temperature", check_temperature);
        run_check!("check_fd", check_fd);
        run_check!("check_dns", check_dns);
        run_check!("check_conntrack", check_conntrack);
        run_check!("check_gpu", check_gpu);
        run_check!("check_systemd", check_systemd);
        run_check!("check_memory_leak", check_memory_leak);
        run_check!("check_swap_activity", check_swap_activity);
        run_check!("check_context_switches", check_context_switches);
        run_check!("check_oom_kills", check_oom_kills);
        run_check!("check_network_errors", check_network_errors);
        run_check!("check_inode_pressure", check_inode_pressure);
        run_check!("check_uptime", check_uptime);
        run_check!("check_load_trend", check_load_trend);
        run_check!("check_tcp_listen", check_tcp_listen);
        run_check!("check_kernel_taint", check_kernel_taint);
        run_check!("check_high_memory_process", check_high_memory_process);
        run_check!("check_ss_orphaned", check_ss_orphaned);
        run_check!("check_conntrack_rate", check_conntrack_rate);
        run_check!("check_ip_forwarding", check_ip_forwarding);

        profile.sort_by(|a, b| b.1.cmp(&a.1));
        eprintln!("profile_each_check (ns/iter, sorted desc):");
        let total: u128 = profile.iter().map(|(_, n)| *n).sum();
        for (name, ns) in &profile {
            let pct = (*ns as f64 / total as f64) * 100.0;
            eprintln!("  {name:30} {ns:>5} ns  ({pct:4.1}%)");
        }
        eprintln!("  TOTAL                          {total:>5} ns");
    }
}
