//! Automatic diagnostics engine.
//!
//! Analyzes a [`Snapshot`] and produces diagnostic findings based on
//! known thresholds and patterns.

use crate::i18n::Locale;
use crate::proc::{FieldValue, Snapshot};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct DiagnosticFinding {
    pub severity: Severity,
    pub source: String,
    pub title: String,
    pub detail: String,
    pub suggestion: String,
}

/// Run all diagnostic checks against a snapshot.
pub fn analyze(snapshot: &Snapshot, locale: Locale) -> Vec<DiagnosticFinding> {
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

fn get_bytes(snapshot: &Snapshot, source: &str, field: &str) -> Option<u64> {
    snapshot.entries.get(source)?
        .fields.iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Bytes(v) => Some(v),
            _ => None,
        })
}

fn get_float(snapshot: &Snapshot, source: &str, field: &str) -> Option<f64> {
    snapshot.entries.get(source)?
        .fields.iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Float(v) => Some(v),
            FieldValue::Integer(v) => Some(v as f64),
            FieldValue::Duration(v) => Some(v),
            _ => None,
        })
}

fn get_integer(snapshot: &Snapshot, source: &str, field: &str) -> Option<i64> {
    snapshot.entries.get(source)?
        .fields.iter()
        .find(|f| f.name == field)
        .and_then(|f| match f.value {
            FieldValue::Integer(v) => Some(v),
            _ => None,
        })
}

fn get_table_row_count(snapshot: &Snapshot, source: &str, field: &str) -> Option<usize> {
    snapshot.entries.get(source)?
        .fields.iter()
        .find(|f| f.name == field)
        .and_then(|f| match &f.value {
            FieldValue::Table(rows) => Some(rows.len()),
            _ => None,
        })
}

fn get_table_col_values(snapshot: &Snapshot, source: &str, field: &str, col: usize) -> Vec<String> {
    snapshot.entries.get(source)
        .and_then(|e| e.fields.iter().find(|f| f.name == field))
        .map(|f| match &f.value {
            FieldValue::Table(rows) => rows.iter()
                .filter_map(|r| r.get(col).cloned())
                .collect(),
            _ => vec![],
        })
        .unwrap_or_default()
}

fn check_memory(findings: &mut Vec<DiagnosticFinding>, snap: &Snapshot, locale: Locale) {
    let total = match get_bytes(snap, "meminfo", "MemTotal") {
        Some(v) => v,
        None => return,
    };
    let available = get_bytes(snap, "meminfo", "MemAvailable").unwrap_or(total);

    if total == 0 { return; }
    let pct_available = (available as f64 / total as f64) * 100.0;

    if pct_available < 10.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: format!("メモリ危機: 残り {:.1}%", pct_available),
                detail: format!("MemAvailable ({}) が MemTotal ({}) の 10% 未満。OOM Killer が発動する可能性。",
                    format_bytes(available), format_bytes(total)),
                suggestion: "ps aux --sort=-rss | head でメモリ消費の多いプロセスを特定".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: format!("Memory critical: {:.1}% available", pct_available),
                detail: format!("MemAvailable ({}) is below 10% of MemTotal ({}). OOM Killer may activate.",
                    format_bytes(available), format_bytes(total)),
                suggestion: "Run: ps aux --sort=-rss | head to find top memory consumers".into(),
            }
        });
    } else if pct_available < 20.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "meminfo".into(),
                title: format!("メモリ残量低下: {:.1}%", pct_available),
                detail: format!("MemAvailable ({}) が 20% を下回っている。メモリリークの可能性を確認。",
                    format_bytes(available)),
                suggestion: "RSS が増え続けているプロセスがないか確認".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "meminfo".into(),
                title: format!("Memory low: {:.1}% available", pct_available),
                detail: format!("MemAvailable ({}) is below 20%. Check for memory leaks.",
                    format_bytes(available)),
                suggestion: "Check for processes with growing RSS over time".into(),
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

    if cpu_count <= 0.0 { return; }
    let ratio = load1 / cpu_count;

    if ratio > 2.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "loadavg".into(),
                title: format!("CPU 過負荷: load {:.1} (CPU{}個の{:.0}倍)", load1, cpu_count as i64, ratio),
                detail: "プロセスがキューに溜まっている。CPU バウンドか I/O 待ちが大量に発生。".into(),
                suggestion: "pressure の PSI データを確認。CPU圧力が高ければCPU不足、I/O圧力が高ければディスクボトルネック。".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "loadavg".into(),
                title: format!("CPU overloaded: load {:.1} ({:.0}x {} CPUs)", load1, ratio, cpu_count as i64),
                detail: "Processes are queuing. Either CPU-bound or massive I/O wait.".into(),
                suggestion: "Check PSI pressure data. High CPU pressure = need more CPUs. High I/O pressure = disk bottleneck.".into(),
            }
        });
    } else if ratio > 1.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!("CPU 飽和: load {:.1} (CPU{}個の{:.0}倍)", load1, cpu_count as i64, ratio),
                detail: "CPU がフル稼働を超えている。レスポンスタイムが劣化している可能性。".into(),
                suggestion: "top で CPU 消費の高いプロセスを確認".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "loadavg".into(),
                title: format!("CPU saturated: load {:.1} ({:.0}x {} CPUs)", load1, ratio, cpu_count as i64),
                detail: "CPU is beyond full utilization. Response times may be degraded.".into(),
                suggestion: "Run top to identify high-CPU processes".into(),
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
                detail: "スワップが設定されていない。メモリ不足時は OOM Killer が唯一の安全装置。".into(),
                suggestion: "本番環境では RAM の 1-2 倍のスワップを設定することを推奨".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "meminfo".into(),
                title: "No swap configured".into(),
                detail: "No swap space is configured. OOM Killer is the only safety net.".into(),
                suggestion: "Consider configuring swap at 1-2x RAM for production".into(),
            }
        });
    } else if swap_free == 0 && swap_total > 0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: "スワップ枯渇".into(),
                detail: format!("スワップ {}  が全て使用済み。次のメモリ確保で OOM Killer が発動する。",
                    format_bytes(swap_total)),
                suggestion: "即座にメモリ消費の高いプロセスを調査".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "meminfo".into(),
                title: "Swap exhausted".into(),
                detail: format!("All {} of swap is used. Next allocation may trigger OOM Killer.",
                    format_bytes(swap_total)),
                suggestion: "Investigate high-memory processes immediately".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("{} zombie processes detected", zombie_count),
                detail: "Parent process not calling wait(). Zombies are harmless but indicate buggy parents.".into(),
                suggestion: "Check PPID of zombies to identify the parent process".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "processes".into(),
                title: format!("{} processes in D-state (uninterruptible I/O)", d_count),
                detail: "Possible NFS hang, disk failure, or kernel driver issue. Cannot be killed with SIGKILL.".into(),
                suggestion: "Check dmesg for storage-related errors".into(),
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
                detail: "接続先がダウン、ファイアウォールでドロップ、または DNS 解決が遅い可能性。".into(),
                suggestion: "接続先の IP/ポートを確認。ping や telnet でリーチャビリティをテスト。".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("{} SYN_SENT — targets not responding", syn_sent),
                detail: "Target host may be down, firewall dropping packets, or slow DNS.".into(),
                suggestion: "Check target IPs/ports. Test reachability with ping/telnet.".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "net/tcp".into(),
                title: format!("{} CLOSE_WAIT — socket FD leak", close_wait),
                detail: "Application not closing sockets. Remote end already closed. Classic FD leak.".into(),
                suggestion: "Use lsof to find which process holds CLOSE_WAIT sockets".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Info,
                source: "net/tcp".into(),
                title: format!("{} TIME_WAIT connections", time_wait),
                detail: "Many short-lived connections. Ephemeral ports may be exhausted.".into(),
                suggestion: "Consider setting net.ipv4.tcp_tw_reuse = 1".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "df".into(),
                title: format!("Disk usage {:.1}% — critically low space", use_pct),
                detail: "Root filesystem usage exceeds 90%. Log writes may fail, services may crash.".into(),
                suggestion: "Run: du -sh /* to find large directories. journalctl --vacuum-size=500M to trim logs.".into(),
            }
        });
    } else if use_pct > 80.0 {
        findings.push(if locale == Locale::Ja {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "df".into(),
                title: format!("ディスク使用率 {:.1}% — 注意", use_pct),
                detail: "ルートファイルシステムの使用率が 80% を超過。容量計画を検討すべき。".into(),
                suggestion: "不要なログやキャッシュの削除を検討。df -h で各パーティションを確認。".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "df".into(),
                title: format!("Disk usage {:.1}% — getting full", use_pct),
                detail: "Root filesystem usage exceeds 80%. Plan capacity expansion.".into(),
                suggestion: "Clean up old logs and caches. Run: df -h to check all partitions.".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Critical,
                source: "thermal".into(),
                title: format!("CPU temperature {:.0}°C — overheating", max_temp),
                detail: "CPU temperature exceeds 90°C. Thermal throttling is active, severely degrading performance. Risk of hardware damage.".into(),
                suggestion: "Check cooling system: fan operation, thermal paste, airflow.".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "thermal".into(),
                title: format!("CPU temperature {:.0}°C — running hot", max_temp),
                detail: "CPU temperature exceeds 75°C. Near throttling threshold. May rise further under sustained load.".into(),
                suggestion: "Consider improving cooling. Check CPU utilization and distribute load.".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "file-nr".into(),
                title: format!("FD usage {:.1}% — file descriptor exhaustion risk", usage_pct),
                detail: "System-wide file descriptor usage exceeds 80%. Exhaustion prevents processes from opening files or sockets.".into(),
                suggestion: "Run: lsof | wc -l to check open FDs. Find leaking processes. Consider raising sysctl fs.file-max.".into(),
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
    let has_nameservers = entry.fields.iter()
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
                detail: "/etc/resolv.conf にネームサーバが設定されていない。名前解決が失敗する。".into(),
                suggestion: "resolv.conf にネームサーバを追加 (例: nameserver 8.8.8.8)".into(),
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "dns".into(),
                title: "No DNS nameservers configured".into(),
                detail: "No nameserver entries found in /etc/resolv.conf. DNS resolution will fail.".into(),
                suggestion: "Add a nameserver to resolv.conf (e.g., nameserver 8.8.8.8)".into(),
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
            }
        } else {
            DiagnosticFinding {
                severity: Severity::Warning,
                source: "conntrack".into(),
                title: format!("Conntrack usage {:.1}% — table exhaustion risk", usage_pct),
                detail: "Connection tracking table usage exceeds 80%. Exhaustion will drop new connections.".into(),
                suggestion: "Increase sysctl net.nf_conntrack_max or add NOTRACK rules for high-traffic flows".into(),
            }
        });
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
