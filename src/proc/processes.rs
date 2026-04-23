use super::{Field, FieldValue, ProcEntry};
use crate::i18n::Locale;
use std::fs;

pub fn parse() -> anyhow::Result<ProcEntry> {
    let mut rows = Vec::new();

    let entries = fs::read_dir("/proc")?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Only numeric directories (PIDs)
        if !name_str.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        let pid = name_str.to_string();
        let base = format!("/proc/{}", pid);

        // Read comm (process name)
        let comm = fs::read_to_string(format!("{}/comm", base))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        // Count open file descriptors
        let fd_count = fs::read_dir(format!("{}/fd", base))
            .map(|entries| entries.flatten().count())
            .unwrap_or(0);

        // Read status for key fields
        let mut state = String::new();
        let mut vm_rss = 0u64;
        let mut threads = 0i64;
        let mut uid = String::new();

        if let Ok(status) = fs::read_to_string(format!("{}/status", base)) {
            for line in status.lines() {
                if let Some((key, val)) = line.split_once(':') {
                    match key.trim() {
                        "State" => state = val.trim().to_string(),
                        "VmRSS" => {
                            let parts: Vec<&str> = val.trim().split_whitespace().collect();
                            vm_rss = parts
                                .first()
                                .and_then(|v| v.parse::<u64>().ok())
                                .unwrap_or(0)
                                * 1024; // kB to bytes
                        }
                        "Threads" => threads = val.trim().parse().unwrap_or(0),
                        "Uid" => {
                            uid = val
                                .trim()
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .to_string();
                        }
                        _ => {}
                    }
                }
            }
        }

        if comm.is_empty() {
            continue;
        }

        rows.push(vec![
            pid,
            comm,
            state,
            format_bytes(vm_rss),
            threads.to_string(),
            uid,
            fd_count.to_string(),
        ]);
    }

    // Sort by PID numerically
    rows.sort_by(|a, b| {
        a[0].parse::<u64>()
            .unwrap_or(0)
            .cmp(&b[0].parse::<u64>().unwrap_or(0))
    });

    let count = rows.len() as i64;

    Ok(ProcEntry {
        source: "/proc/[pid]/*".into(),
        fields: vec![
            Field {
                name: "process_count".into(),
                value: FieldValue::Integer(count),
                unit: None,
                description: "Total number of processes".into(),
            },
            Field {
                name: "processes".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: "Processes (PID, name, state, RSS, threads, UID, FDs)".into(),
            },
        ],
    })
}

/// Parse detailed information for a single process from /proc/[pid]/*.
pub fn parse_detail(pid: &str, locale: Locale) -> anyhow::Result<ProcEntry> {
    let ja = matches!(locale, Locale::Ja);
    let base = format!("/proc/{}", pid);
    let mut fields: Vec<Field> = Vec::new();

    // cmdline
    if let Ok(raw) = fs::read(format!("{}/cmdline", base)) {
        let cmdline = raw
            .split(|&b| b == 0u8)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect::<Vec<_>>()
            .join(" ");
        fields.push(Field {
            name: "cmdline".into(),
            value: FieldValue::Text(if cmdline.is_empty() {
                "[kernel thread]".into()
            } else {
                cmdline
            }),
            unit: None,
            description: if ja {
                "このプロセスを起動したコマンドライン全体 (/proc/[pid]/cmdline)。引数はNUL区切りで格納されており、ここではスペースで結合して表示。カーネルスレッドはcmdlineなし([kernel thread]と表示)。"
            } else {
                "Full command line used to start this process (/proc/[pid]/cmdline). \
                Arguments are NUL-separated in the file and joined with spaces here. \
                Kernel threads have no cmdline and appear as [kernel thread]."
            }.into(),
        });
    }

    // exe
    if let Ok(exe) = fs::read_link(format!("{}/exe", base)) {
        fields.push(Field {
            name: "exe".into(),
            value: FieldValue::Text(exe.to_string_lossy().into_owned()),
            unit: None,
            description: if ja {
                "実行バイナリへのパス (/proc/[pid]/exeシンボリックリンク)。削除済み実行ファイルには '(deleted)' が付く。カーネルスレッドにはexeリンクなし。"
            } else {
                "Path to the executable binary (/proc/[pid]/exe symlink). \
                Deleted executables append ' (deleted)' to the path. \
                Kernel threads have no exe link."
            }.into(),
        });
    }

    // cwd
    if let Ok(cwd) = fs::read_link(format!("{}/cwd", base)) {
        fields.push(Field {
            name: "cwd".into(),
            value: FieldValue::Text(cwd.to_string_lossy().into_owned()),
            unit: None,
            description: if ja {
                "プロセスのカレントワーキングディレクトリ (/proc/[pid]/cwdシンボリックリンク)。相対パスによるファイル操作の基準となる。"
            } else {
                "Current working directory of the process (/proc/[pid]/cwd symlink). \
                Affects relative path resolution for file operations."
            }.into(),
        });
    }

    // status — full parse with per-field descriptions
    if let Ok(content) = fs::read_to_string(format!("{}/status", base)) {
        for line in content.lines() {
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim().to_string();
                fields.push(Field {
                    name: format!("status.{}", key),
                    value: FieldValue::Text(val),
                    unit: None,
                    description: status_field_help(key, ja).into(),
                });
            }
        }
    }

    // io — read/write accounting (may require same UID or root)
    if let Ok(content) = fs::read_to_string(format!("{}/io", base)) {
        for line in content.lines() {
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim();
                let val = val.trim();
                let value = val
                    .parse::<i64>()
                    .map(FieldValue::Integer)
                    .unwrap_or_else(|_| FieldValue::Text(val.to_string()));
                let unit = matches!(
                    key,
                    "rchar" | "wchar" | "read_bytes" | "write_bytes" | "cancelled_write_bytes"
                )
                .then(|| "bytes".into());
                fields.push(Field {
                    name: format!("io.{}", key),
                    value,
                    unit,
                    description: io_field_help(key, ja).into(),
                });
            }
        }
    }

    // oom_score
    if let Ok(s) = fs::read_to_string(format!("{}/oom_score", base)) {
        if let Ok(n) = s.trim().parse::<i64>() {
            fields.push(Field {
                name: "oom_score".into(),
                value: FieldValue::Integer(n),
                unit: None,
                description: if ja {
                    "OOMキラースコア (0〜1000)。値が高いほどメモリ不足時にkillされやすい。メモリ使用量・nice値・実行時間などから算出される。"
                } else {
                    "OOM killer score (0–1000). Higher values make this process more \
                    likely to be killed when the system runs out of memory. Influenced by \
                    memory usage, nice value, and how long the process has been running."
                }.into(),
            });
        }
    }

    // oom_score_adj
    if let Ok(s) = fs::read_to_string(format!("{}/oom_score_adj", base)) {
        if let Ok(n) = s.trim().parse::<i64>() {
            fields.push(Field {
                name: "oom_score_adj".into(),
                value: FieldValue::Integer(n),
                unit: None,
                description: if ja {
                    "OOMスコア調整値 (-1000〜1000)。-1000でOOMkillを完全に無効化。正の値はkillされやすくなる。systemdはOOMPolicyに基づきサービスごとに設定する。"
                } else {
                    "OOM score adjustment (-1000 to 1000). Write -1000 to make this \
                    process immune to OOM killing. Positive values increase kill probability. \
                    systemd sets this for services based on OOMPolicy."
                }.into(),
            });
        }
    }

    // wchan — kernel wait channel
    if let Ok(s) = fs::read_to_string(format!("{}/wchan", base)) {
        let s = s.trim().to_string();
        if !s.is_empty() && s != "0" {
            fields.push(Field {
                name: "wchan".into(),
                value: FieldValue::Text(s),
                unit: None,
                description: if ja {
                    "待機チャネル: プロセスがスリープ中のカーネル関数 (/proc/[pid]/wchan)。\
                    主な値: 'ep_poll'(epoll/イベントループ), 'futex_wait'(mutex/ロック待ち), \
                    'poll_schedule_timeout'(ネットワークI/O), 'do_wait'(子プロセス待ち)。"
                } else {
                    "Wait channel: the kernel function in which this process is sleeping \
                    (/proc/[pid]/wchan). Common values: 'ep_poll' (epoll/event loop), \
                    'futex_wait' (mutex/lock), 'poll_schedule_timeout' (network I/O), \
                    'do_wait' (waiting for child process)."
                }
                .into(),
            });
        }
    }

    // cgroup
    if let Ok(s) = fs::read_to_string(format!("{}/cgroup", base)) {
        let s = s.trim().to_string();
        if !s.is_empty() {
            fields.push(Field {
                name: "cgroup".into(),
                value: FieldValue::Text(s),
                unit: None,
                description: if ja {
                    "コントロールグループのメンバーシップ (/proc/[pid]/cgroup)。\
                    形式: 'hierarchy_id:サブシステム:パス'。CPU・メモリ・I/Oのリソース制限を管理。\
                    systemdはサービスを /system.slice/<name>.service 配下に配置する。"
                } else {
                    "Control group membership (/proc/[pid]/cgroup). Format: \
                    'hierarchy_id:subsystems:path'. Controls CPU, memory, and I/O resource \
                    limits. systemd places services under /system.slice/<name>.service."
                }
                .into(),
            });
        }
    }

    // limits — resource limits table
    if let Ok(content) = fs::read_to_string(format!("{}/limits", base)) {
        let mut rows: Vec<Vec<String>> = Vec::new();
        for line in content.lines().skip(1) {
            if let Some(pos) = line.find("  ") {
                let name = line[..pos].trim().to_string();
                let rest = line[pos..].trim();
                let parts: Vec<&str> = rest.split_whitespace().collect();
                let soft = parts.first().copied().unwrap_or("").to_string();
                let hard = parts.get(1).copied().unwrap_or("").to_string();
                let units = parts.get(2).copied().unwrap_or("").to_string();
                if !name.is_empty() {
                    rows.push(vec![name, soft, hard, units]);
                }
            }
        }
        if !rows.is_empty() {
            fields.push(Field {
                name: "limits".into(),
                value: FieldValue::Table(rows),
                unit: None,
                description: if ja {
                    "リソース制限 (/proc/[pid]/limits)。ソフトリミットが実際に適用される上限、\
                    ハードリミットはソフトリミットを上げられる天井値。\
                    主要な制限: 'Max open files'(RLIMIT_NOFILE/FD数), \
                    'Max processes'(RLIMIT_NPROC), 'Max address space'(RLIMIT_AS), \
                    'Max stack size'(RLIMIT_STACK)。"
                } else {
                    "Resource limits (/proc/[pid]/limits). Soft limit is enforced; \
                    hard limit is the ceiling a process can raise its soft limit to. \
                    Key limits: 'Max open files' (RLIMIT_NOFILE), \
                    'Max processes' (RLIMIT_NPROC), 'Max address space' (RLIMIT_AS), \
                    'Max stack size' (RLIMIT_STACK)."
                }
                .into(),
            });
        }
    }

    // open_fds — file descriptor table
    if let Ok(dir) = fs::read_dir(format!("{}/fd", base)) {
        let mut entries: Vec<_> = dir.flatten().collect();
        entries.sort_by_key(|e| {
            e.file_name()
                .to_string_lossy()
                .parse::<u64>()
                .unwrap_or(u64::MAX)
        });
        let rows: Vec<Vec<String>> = entries
            .iter()
            .map(|e| {
                let fd_num = e.file_name().to_string_lossy().to_string();
                let target = fs::read_link(e.path())
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_else(|_| "?".into());
                vec![fd_num, target]
            })
            .collect();
        fields.push(Field {
            name: "open_fds".into(),
            value: FieldValue::Table(rows),
            unit: None,
            description: if ja {
                "オープン中のファイルディスクリプタ一覧 (/proc/[pid]/fd/)。\
                各エントリは実リソースへのシンボリックリンク: 通常ファイルはフルパス、\
                ソケットは 'socket:[inode]'(/proc/net/tcp等でinodeを照合), \
                パイプは 'pipe:[inode]'、匿名fdは 'anon_inode:<type>'。\
                FD 0=stdin, 1=stdout, 2=stderr。"
            } else {
                "Open file descriptors (/proc/[pid]/fd/). Each entry is a symlink to \
                the actual resource: regular files show full paths, sockets show 'socket:[inode]' \
                (match inode against /proc/net/tcp or /proc/net/unix), pipes show 'pipe:[inode]', \
                and anonymous fds show 'anon_inode:<type>'. FD 0=stdin, 1=stdout, 2=stderr."
            }
            .into(),
        });
    }

    Ok(ProcEntry {
        source: format!("/proc/{}", pid),
        fields,
    })
}

fn status_field_help(key: &str, ja: bool) -> &'static str {
    if ja {
        match key {
            "Name" => "プロセス名 (実行ファイル名の最大16文字, /proc/[pid]/comm から取得)。",
            "Umask" => {
                "ファイル作成マスク(8進数)。新規ファイルのパーミッションは ~umask との AND になる。"
            }
            "State" => {
                "プロセス状態: R=実行中, S=割り込み可能スリープ, D=割り込み不可スリープ(I/O待ち), Z=ゾンビ, T=シグナルで停止, t=デバッガで停止, I=アイドルカーネルスレッド。"
            }
            "Tgid" => {
                "スレッドグループID。メインスレッドではPIDと同じ。同じプロセスのスレッドは全て同じTgidを持つ。"
            }
            "Ngid" => "NUMA自動バランシング用のNUMAグループID (グループ未参加の場合は0)。",
            "Pid" => "プロセスID。fork時にカーネルが割り当てる一意の数値識別子。",
            "PPid" => {
                "親プロセスID。このプロセスをfork()したプロセス。PID 1(init/systemd)が孤児プロセスの親になる。"
            }
            "TracerPid" => {
                "このプロセスをトレース中のプロセスのPID (gdb, strace, ltrace等)。トレースされていない場合は0。"
            }
            "Uid" => {
                "実UID・実効UID・保存セットUID・ファイルシステムUID。実効UIDが実際のファイルアクセス権限を決定する。"
            }
            "Gid" => "実GID・実効GID・保存セットGID・ファイルシステムGID。",
            "FDSize" => {
                "ファイルディスクリプタテーブルの現在の容量 (最大FD番号より大きい2のべき乗)。"
            }
            "Groups" => "補助グループID一覧。実効GIDに加えてパーミッション確認に使用される。",
            "VmPeak" => {
                "仮想メモリの最大値: これまでのVmSizeの最大値。キャパシティプランニングに有用。"
            }
            "VmSize" => {
                "現在の仮想メモリ総量: コード・データ・スタック・共有ライブラリ等全マップ領域の合計。全てがRAMにあるわけではない。"
            }
            "VmLck" => {
                "ロックメモリ: RAMに固定されスワップアウトできないページ (mlock/mlockall で確保)。"
            }
            "VmPin" => {
                "ピン留めメモリ: カーネルのメモリコンパクションで移動できないページ (DMA・ヒュージページ等)。"
            }
            "VmHWM" => "最大RSS(ハイウォーターマーク): このプロセスが同時に使用したRAMの最大値。",
            "VmRSS" => {
                "常駐セットサイズ: 現在占有している物理RAM量。= RssAnon + RssFile + RssShmem。"
            }
            "RssAnon" => {
                "匿名RSS: ファイルに紐付かないプライベートメモリ (ヒープ確保・スタック拡張・mmap(MAP_ANONYMOUS))。"
            }
            "RssFile" => {
                "ファイルバックRSS: ファイルからマップされたメモリ (実行コード・共有ライブラリ・mmapデータファイル)。"
            }
            "RssShmem" => {
                "共有メモリRSS: 共有メモリセグメント内のメモリ (tmpfs・SysV shm・POSIX mmap共有)。"
            }
            "VmData" => {
                "データセグメントサイズ: ヒープとBSS (未初期化グローバル変数)。malloc/brkで拡大する。"
            }
            "VmStk" => {
                "メインスレッドのスタックサイズ。各スレッドは独自のスタックを持ち、ここにはメインスレッド分のみ表示。"
            }
            "VmExe" => {
                "テキストセグメントサイズ: 実行ファイル本体のコード (共有ライブラリを除く)。"
            }
            "VmLib" => {
                "共有ライブラリのコードサイズ: このプロセスにロードされた全.soファイルのコード合計。"
            }
            "VmPTE" => {
                "ページテーブルエントリのサイズ: 仮想→物理アドレスマッピング維持のカーネルオーバーヘッド。"
            }
            "VmSwap" => {
                "スワップアウト済みメモリ量。高値はメモリ逼迫を示す。このプロセスのページがスワップに追い出された。"
            }
            "HugetlbPages" => {
                "ヒュージページ (通常2MB) でバックされたメモリ。大きなワーキングセットを持つDB等でTLBプレッシャーを低減。"
            }
            "CoreDumping" => "クラッシュシグナルによりコアダンプが現在書き込まれていれば1。",
            "THP_enabled" => {
                "透過的ヒュージページ: 1=有効(カーネルが自動的に小ページをヒュージページに統合可能), 0=無効。"
            }
            "Threads" => {
                "メインスレッドを含むこのプロセスのスレッド数。各スレッドは /proc/[pid]/task/ 以下に個別エントリを持つ。"
            }
            "SigQ" => {
                "シグナルキュー: 現在/最大。形式: 'キュー数/上限'。リアルタイムシグナルの待機数を示す。"
            }
            "SigPnd" => "スレッド専用の保留シグナルビットマスク。このスレッド専宛のシグナル。",
            "ShdPnd" => {
                "プロセス共通の保留シグナルビットマスク。どのスレッドにも配信される可能性がある。"
            }
            "SigBlk" => {
                "ブロック中のシグナルビットマスク。sigprocmaskで明示的に解除するまで配信されない。"
            }
            "SigIgn" => {
                "無視しているシグナルビットマスク。SIG_IGNハンドラまたはSA_NOCLDWAITなSIGCHLDの場合。"
            }
            "SigCgt" => {
                "キャッチしているシグナルビットマスク。カスタムハンドラがインストールされているシグナル。"
            }
            "CapInh" => {
                "継承可能ケーパビリティビットマスク。exec()を跨いで子プロセスに引き渡せるケーパビリティ。"
            }
            "CapPrm" => {
                "許可ケーパビリティビットマスク。このプロセスが有効化できるケーパビリティの最大セット。"
            }
            "CapEff" => {
                "実効ケーパビリティビットマスク。現在有効で、カーネルが特権操作のチェックに使用するケーパビリティ。"
            }
            "CapBnd" => {
                "ケーパビリティバウンディングセット。ここから削除されたケーパビリティは二度と取得できないハード上限。"
            }
            "CapAmb" => {
                "アンビエントケーパビリティ。特権なしバイナリへのexec()を跨いで保持される (Linux 4.3+)。"
            }
            "NoNewPrivs" => {
                "1の場合(PR_SET_NO_NEW_PRIVSで設定), exec()で新たな特権を取得できない: setuidビットとファイルケーパビリティが無視される。"
            }
            "Seccomp" => {
                "Seccompモード: 0=無効, 1=strictモード(read/write/exit/sigreturnのみ許可), 2=filterモード(BPFポリシー適用)。"
            }
            "Seccomp_filters" => "このプロセスに適用中のBPF seccompフィルタプログラム数。",
            "Cpus_allowed" => {
                "CPUアフィニティビットマスク(16進数)。このプロセスがスケジュールされうるCPUを示す。"
            }
            "Cpus_allowed_list" => {
                "人間が読みやすいCPUアフィニティリスト (例: '0-3,7')。taskset(1)またはsched_setaffinity(2)で設定。"
            }
            "Mems_allowed" => {
                "NUMAメモリノードビットマスク。このプロセスがメモリを確保できるNUMAノードを示す。"
            }
            "Mems_allowed_list" => {
                "人間が読みやすいNUMAノードリスト (例: '0-1')。マルチソケットサーバで有用。"
            }
            "voluntary_ctxt_switches" => {
                "自発的コンテキストスイッチ: プロセスが自らCPUを解放した回数(ブロッキングI/O・スリープ・mutex待ち)。I/Oバウンドなプロセスでは高値が正常。"
            }
            "nonvoluntary_ctxt_switches" => {
                "非自発的コンテキストスイッチ: カーネルがプロセスを横取りした回数(タイムスライス切れ)。高値はCPU競合または実行可能プロセスが多すぎることを示す。"
            }
            _ => "/proc/[pid]/status のフィールド。",
        }
    } else {
        match key {
            "Name" => {
                "Process name (up to 16 chars of the executable name, from /proc/[pid]/comm)."
            }
            "Umask" => {
                "File mode creation mask (octal). New files get permissions ANDed with ~umask."
            }
            "State" => {
                "Process state: R=running, S=interruptible sleep, D=uninterruptible disk sleep, Z=zombie, T=stopped by signal, t=stopped by debugger, I=idle kernel thread."
            }
            "Tgid" => {
                "Thread group ID. Equal to PID for the main thread. All threads in a process share the same Tgid."
            }
            "Ngid" => "NUMA group ID for automatic NUMA balancing (0 if not in a NUMA group).",
            "Pid" => "Process ID. Unique numeric identifier assigned by the kernel at fork time.",
            "PPid" => {
                "Parent process ID. The process that fork()ed this one. PID 1 (init/systemd) is parent of orphaned processes."
            }
            "TracerPid" => {
                "PID of the process tracing this one (e.g., gdb, strace, ltrace). 0 if not being traced."
            }
            "Uid" => {
                "Real, effective, saved-set, and filesystem UIDs. Effective UID determines actual file access permissions."
            }
            "Gid" => "Real, effective, saved-set, and filesystem GIDs.",
            "FDSize" => {
                "Current file descriptor table capacity (next power of 2 above the highest open FD number)."
            }
            "Groups" => {
                "Supplementary group IDs. Used in addition to the effective GID for permission checks."
            }
            "VmPeak" => {
                "Peak virtual memory size: highest VmSize this process has ever had. Useful for capacity planning."
            }
            "VmSize" => {
                "Current virtual memory size: all mapped regions (code, data, stack, shared libs, kernel mappings). Not all is in RAM."
            }
            "VmLck" => {
                "Locked memory: pages that are pinned in RAM and cannot be swapped out (mlock/mlockall)."
            }
            "VmPin" => {
                "Pinned memory: pages that cannot be moved by the kernel's memory compaction (used by DMA, huge pages)."
            }
            "VmHWM" => {
                "High water mark: peak resident set size. The most RAM this process has simultaneously used."
            }
            "VmRSS" => {
                "Resident set size: physical RAM currently occupied. = RssAnon + RssFile + RssShmem."
            }
            "RssAnon" => {
                "Anonymous RSS: private memory not backed by a file (heap allocations, stack growth, mmap(MAP_ANONYMOUS))."
            }
            "RssFile" => {
                "File-backed RSS: memory mapped from files (executable code, shared libraries, mmap'd data files)."
            }
            "RssShmem" => {
                "Shared memory RSS: memory in shared memory segments (tmpfs, SysV shm, POSIX mmap shared)."
            }
            "VmData" => {
                "Data segment size: heap and BSS (uninitialized globals). Grows with malloc/brk."
            }
            "VmStk" => {
                "Main thread stack size. Each thread has its own stack; only the main thread's is shown here."
            }
            "VmExe" => {
                "Text segment size: the executable code itself (not including shared libraries)."
            }
            "VmLib" => {
                "Shared library code size: total code from all shared libraries (.so files) loaded into this process."
            }
            "VmPTE" => {
                "Page table entries size: kernel overhead for maintaining virtual→physical address mappings."
            }
            "VmSwap" => {
                "Swapped-out memory size. High values indicate memory pressure; this process's pages were pushed to swap."
            }
            "HugetlbPages" => {
                "Memory backed by huge pages (typically 2MB). Reduces TLB pressure for large working sets like databases."
            }
            "CoreDumping" => {
                "1 if a core dump is currently being written for this process (e.g., due to a crash signal)."
            }
            "THP_enabled" => {
                "Transparent huge pages: 1=enabled (kernel may collapse small pages into huge pages automatically), 0=disabled."
            }
            "Threads" => {
                "Number of threads in this process, including the main thread. Each thread appears as a separate entry under /proc/[pid]/task/."
            }
            "SigQ" => {
                "Signal queue: current/max. Format: 'queued/limit'. Shows how many real-time signals are pending."
            }
            "SigPnd" => {
                "Thread-private pending signals bitmask. These signals are directed to this specific thread."
            }
            "ShdPnd" => {
                "Process-wide pending signals bitmask. These signals may be delivered to any thread."
            }
            "SigBlk" => {
                "Blocked signals bitmask. Signals in this set will not be delivered until explicitly unblocked (sigprocmask)."
            }
            "SigIgn" => {
                "Ignored signals bitmask. These signals are silently discarded (SIG_IGN handler or SIGCHLD with SA_NOCLDWAIT)."
            }
            "SigCgt" => {
                "Caught signals bitmask. The process has installed custom handlers for these signals."
            }
            "CapInh" => {
                "Inheritable capabilities bitmask. These capabilities may be passed to child processes across exec()."
            }
            "CapPrm" => {
                "Permitted capabilities bitmask. The maximum set of capabilities this process may activate."
            }
            "CapEff" => {
                "Effective capabilities bitmask. Currently active capabilities checked by the kernel for privileged operations."
            }
            "CapBnd" => {
                "Capability bounding set. Hard limit — capabilities dropped from here cannot be regained."
            }
            "CapAmb" => {
                "Ambient capabilities. Preserved across exec() for unprivileged binaries (Linux 4.3+)."
            }
            "NoNewPrivs" => {
                "If 1 (set via PR_SET_NO_NEW_PRIVS), exec() cannot gain new privileges: setuid bits and file capabilities are ignored."
            }
            "Seccomp" => {
                "Seccomp mode: 0=disabled, 1=strict (only read/write/exit/sigreturn allowed), 2=filter (BPF policy applied)."
            }
            "Seccomp_filters" => {
                "Number of BPF seccomp filter programs currently applied to this process."
            }
            "Cpus_allowed" => {
                "CPU affinity bitmask (hex). Bits set indicate CPUs this process may be scheduled on."
            }
            "Cpus_allowed_list" => {
                "Human-readable CPU affinity list (e.g., '0-3,7'). Set with taskset(1) or sched_setaffinity(2)."
            }
            "Mems_allowed" => {
                "NUMA memory node bitmask. Bits set indicate NUMA nodes this process may allocate memory from."
            }
            "Mems_allowed_list" => {
                "Human-readable NUMA node list (e.g., '0-1'). Relevant on multi-socket servers."
            }
            "voluntary_ctxt_switches" => {
                "Voluntary context switches: the process willingly yielded the CPU (blocking I/O, sleep, mutex wait). High values are expected for I/O-bound processes."
            }
            "nonvoluntary_ctxt_switches" => {
                "Involuntary context switches: the kernel preempted the process (timeslice expired). High values indicate CPU contention or too many runnable processes."
            }
            _ => "Field from /proc/[pid]/status.",
        }
    }
}

fn io_field_help(key: &str, ja: bool) -> &'static str {
    if ja {
        match key {
            "rchar" => {
                "read()等のシスコールに渡されたバイト数合計 — ページキャッシュから提供されたデータを含む。実際のディスク読み込みは反映しない。"
            }
            "wchar" => {
                "write()等のシスコールに渡されたバイト数合計 — ページキャッシュで吸収されディスクに届いていないデータも含む。"
            }
            "syscr" => "このプロセスが発行したread(), pread64(), readv()等のシスコール回数。",
            "syscw" => "このプロセスが発行したwrite(), pwrite64(), writev()等のシスコール回数。",
            "read_bytes" => {
                "ブロックストレージから実際に取得されたバイト数 (キャッシュミス)。真のディスク読み込みI/O量。root または同じUIDが必要。"
            }
            "write_bytes" => {
                "ブロックストレージに実際に書き込まれたバイト数。フラッシュ前のページキャッシュ内の書き込みは除く。root または同じUIDが必要。"
            }
            "cancelled_write_bytes" => {
                "ストレージに送られなかった書き込みバイト数 (例: フラッシュ前にファイルが切り詰められた)。write_bytesからこの値を引くと正味のストレージ書き込み量になる。"
            }
            _ => "/proc/[pid]/io のフィールド。",
        }
    } else {
        match key {
            "rchar" => {
                "Total bytes passed to read() and similar syscalls — includes data served from page cache. Does NOT reflect actual disk reads."
            }
            "wchar" => {
                "Total bytes passed to write() and similar syscalls — includes data absorbed by the page cache without hitting disk."
            }
            "syscr" => {
                "Number of read(), pread64(), readv(), and similar syscalls issued by this process."
            }
            "syscw" => {
                "Number of write(), pwrite64(), writev(), and similar syscalls issued by this process."
            }
            "read_bytes" => {
                "Bytes actually fetched from block storage (cache misses). This is true disk read I/O. Requires root or the same UID to read."
            }
            "write_bytes" => {
                "Bytes actually written to block storage. Excludes writes still in page cache that haven't been flushed. Requires root or same UID."
            }
            "cancelled_write_bytes" => {
                "Write bytes that were never sent to storage (e.g., file was truncated before flush). Subtracting this from write_bytes gives net storage writes."
            }
            _ => "Field from /proc/[pid]/io.",
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const MIB: u64 = 1024 * 1024;
    const KIB: u64 = 1024;
    if bytes >= MIB {
        format!("{:.1} MiB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes as f64 / KIB as f64)
    } else if bytes == 0 {
        "-".to_string()
    } else {
        format!("{} B", bytes)
    }
}
