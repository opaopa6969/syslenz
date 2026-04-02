#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Locale {
    En,
    Ja,
}

impl Locale {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ja" | "jp" | "japanese" => Locale::Ja,
            _ => Locale::En,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Locale::En => Locale::Ja,
            Locale::Ja => Locale::En,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Locale::En => "EN",
            Locale::Ja => "JA",
        }
    }
}

/// UI label keys
pub struct T;

impl T {
    // Status bar
    pub const SOURCE: &str = "source";
    pub const DRILL_IN: &str = "drill_in";
    pub const BACK: &str = "back";
    pub const DIFF: &str = "diff";
    pub const SEARCH: &str = "search";
    pub const REFRESH: &str = "refresh";
    pub const GRAPH: &str = "graph";
    pub const AUTO: &str = "auto";
    pub const EXPORT: &str = "export";
    pub const QUIT: &str = "quit";
    pub const HELP: &str = "help";
    pub const LANG: &str = "lang";
    pub const AGO: &str = "ago";
    pub const SNAPS: &str = "snaps";
    pub const AXIS: &str = "axis";
    pub const AXIS_AUTO: &str = "axis_auto";
    pub const AXIS_ZERO: &str = "axis_zero";

    // View names
    pub const VIEW_OVERVIEW: &str = "view_overview";
    pub const VIEW_DETAIL: &str = "view_detail";
    pub const VIEW_DIFF: &str = "view_diff";
    pub const VIEW_TABLE: &str = "view_table";
    pub const VIEW_GRAPH: &str = "view_graph";
    pub const VIEW_DASHBOARD: &str = "view_dashboard";
    pub const VIEW_WELCOME: &str = "view_welcome";

    // Welcome screen keys
    pub const WELCOME_NAV: &str = "welcome_nav";
    pub const WELCOME_DRILL: &str = "welcome_drill";
    pub const WELCOME_DIFF: &str = "welcome_diff";
    pub const WELCOME_SEARCH: &str = "welcome_search";
    pub const WELCOME_GRAPH: &str = "welcome_graph";
    pub const WELCOME_HELP: &str = "welcome_help";
    pub const WELCOME_LANG: &str = "welcome_lang";
    pub const WELCOME_CTA: &str = "welcome_cta";

    // Table headers
    pub const FIELD: &str = "field";
    pub const VALUE: &str = "value";
    pub const UNIT: &str = "unit";
    pub const DESCRIPTION: &str = "description";
    pub const OLD: &str = "old";
    pub const NEW: &str = "new";

    // Messages
    pub const NO_DATA: &str = "no_data";
    pub const NO_CHANGES: &str = "no_changes";
    pub const NO_TABLE_DATA: &str = "no_table_data";
    pub const EXPORTED: &str = "exported";
    pub const EXPORT_FAILED: &str = "export_failed";
    pub const SEARCHING: &str = "searching";
}

pub fn t(locale: Locale, key: &str) -> &'static str {
    match locale {
        Locale::En => en(key),
        Locale::Ja => ja(key),
    }
}

fn en(key: &str) -> &'static str {
    match key {
        "source" => "source",
        "drill_in" => "drill-in",
        "back" => "back",
        "diff" => "diff",
        "search" => "search",
        "refresh" => "refresh",
        "graph" => "graph",
        "auto" => "auto",
        "export" => "export",
        "quit" => "quit",
        "help" => "help",
        "lang" => "lang",
        "ago" => "s ago",
        "snaps" => "snaps",
        "axis" => "axis",
        "axis_auto" => "Axis (auto range)",
        "axis_zero" => "Axis (zero baseline)",

        "view_overview" => "OVERVIEW",
        "view_detail" => "DETAIL",
        "view_diff" => "DIFF",
        "view_table" => "TABLE",
        "view_graph" => "GRAPH",
        "view_dashboard" => "DASHBOARD",
        "view_welcome" => "WELCOME",

        "welcome_nav" => "Navigate sources / fields",
        "welcome_drill" => "Drill in (detail view)",
        "welcome_diff" => "Diff view",
        "welcome_search" => "Search sources",
        "welcome_graph" => "Graph (sparkline of numeric field)",
        "welcome_help" => "Help panel (field descriptions)",
        "welcome_lang" => "Toggle language (EN/JA)",
        "welcome_cta" => "Press D for Dashboard, O for Classic mode",

        "field" => "Field",
        "value" => "Value",
        "unit" => "Unit",
        "description" => "Description",
        "old" => "Old",
        "new" => "New",

        "no_data" => "No data",
        "no_changes" => " No changes detected since last refresh",
        "no_table_data" => " No table data in this source",
        "exported" => "Exported",
        "export_failed" => "Export failed",
        "searching" => "Search",

        _ => "?",
    }
}

fn ja(key: &str) -> &'static str {
    match key {
        "source" => "ソース",
        "drill_in" => "詳細",
        "back" => "戻る",
        "diff" => "差分",
        "search" => "検索",
        "refresh" => "更新",
        "graph" => "グラフ",
        "auto" => "自動",
        "export" => "出力",
        "quit" => "終了",
        "help" => "ヘルプ",
        "lang" => "言語",
        "ago" => "秒前",
        "snaps" => "件",
        "axis" => "軸",
        "axis_auto" => "自動レンジ",
        "axis_zero" => "ゼロ基点",

        "view_overview" => "概要",
        "view_detail" => "詳細",
        "view_diff" => "差分",
        "view_table" => "テーブル",
        "view_graph" => "グラフ",
        "view_dashboard" => "ダッシュボード",
        "view_welcome" => "ようこそ",

        "welcome_nav" => "ソース / フィールド移動",
        "welcome_drill" => "ドリルイン（詳細表示）",
        "welcome_diff" => "差分ビュー",
        "welcome_search" => "ソース検索",
        "welcome_graph" => "グラフ（数値フィールドの推移）",
        "welcome_help" => "ヘルプパネル（フィールド説明）",
        "welcome_lang" => "言語切り替え (EN/JA)",
        "welcome_cta" => "D でダッシュボード、O でクラシックモード",

        "field" => "フィールド",
        "value" => "値",
        "unit" => "単位",
        "description" => "説明",
        "old" => "変更前",
        "new" => "変更後",

        "no_data" => "データなし",
        "no_changes" => " 前回更新から変更なし",
        "no_table_data" => " このソースにテーブルデータはありません",
        "exported" => "出力完了",
        "export_failed" => "出力失敗",
        "searching" => "検索",

        _ => "?",
    }
}

/// Source-level descriptions for the help panel
pub fn source_description(locale: Locale, source: &str) -> &'static str {
    match locale {
        Locale::En => source_desc_en(source),
        Locale::Ja => source_desc_ja(source),
    }
}

fn source_desc_en(source: &str) -> &'static str {
    if source.starts_with("plugin/") {
        return "Plugin-provided metrics";
    }
    match source {
        "meminfo" => "System memory usage: total, free, available, buffers, cache, swap",
        "uptime" => "System uptime and idle time since boot",
        "loadavg" => "CPU load averages for 1, 5, and 15 minute intervals",
        "version" => "Linux kernel version string and build info",
        "cpuinfo" => "Per-CPU details: model, frequency, cache, features",
        "stat" => "Kernel/system statistics: CPU time, context switches, boot time",
        "mounts" => "Currently mounted filesystems and their options",
        "partitions" => "Block device partition table",
        "diskstats" => "Per-disk I/O statistics: reads, writes, in-flight",
        "processes" => "Running processes: PID, name, state, memory, threads",
        "swaps" => "Active swap devices and their usage",
        "net/dev" => "Per-interface network traffic: bytes, packets, errors",
        "net/tcp" => "Active TCP connections: local/remote address, state",
        "net/udp" => "Active UDP sockets: local/remote address, state",
        "net/unix" => "Unix domain sockets: type, state, path",
        "net/arp" => "ARP cache: IP to MAC address mappings",
        "net/route" => "Kernel routing table",
        "net/sockstat" => "Socket statistics summary: TCP, UDP, RAW counts",
        "net/snmp" => "SNMP counters: IP, TCP, UDP protocol statistics",
        "net/netstat" => "Extended network statistics",
        "net/wireless" => "Wireless interface statistics",
        "vmstat" => "Virtual memory statistics: page faults, swapping, I/O",
        "buddyinfo" => "Memory fragmentation: free pages per order per zone",
        "zoneinfo" => "Detailed per-zone memory information",
        "slabinfo" => "Kernel slab allocator cache statistics",
        "pagetypeinfo" => "Page allocation type information per zone",
        "modules" => "Loaded kernel modules and their dependencies",
        "interrupts" => "Hardware interrupt counters per CPU",
        "softirqs" => "Software interrupt counters per CPU",
        "schedstat" => "CPU scheduler statistics",
        "timer_list" => "Active kernel timers",
        "pressure" => "PSI: CPU, memory, and I/O pressure stall info",
        "cgroups" => "Available cgroup controllers",
        "cmdline" => "Kernel boot command line parameters",
        "consoles" => "Registered console devices",
        "crypto" => "Available cryptographic algorithms",
        "devices" => "Registered character and block devices",
        "filesystems" => "Supported filesystem types",
        "iomem" => "Physical memory map and device regions",
        "ioports" => "I/O port address ranges",
        "locks" => "Current file locks (flock/posix)",
        "misc" => "Miscellaneous registered devices",
        "dma" => "DMA channel usage",
        "df" => "Disk filesystem usage: total, used, available per mount point",
        "thermal" => "CPU/GPU temperature from thermal zones",
        "file-nr" => "System-wide file descriptor allocation and limits",
        "ip/route" => "IP routing table from 'ip route show': default gateway, routes, metrics",
        "ip/neighbor" => {
            "ARP/NDP neighbor cache from 'ip neighbor show': IP-to-MAC mappings and states"
        }
        "ss" => {
            "Socket statistics summary from 'ss -s': TCP established/timewait/orphaned, UDP counts"
        }
        "dns" => {
            "DNS configuration from /etc/resolv.conf: nameservers, search domains, resolution timing"
        }
        "conntrack" => {
            "Connection tracking table usage: current count, max limit, utilization percentage"
        }
        "gpu" => "NVIDIA GPU metrics: temperature, utilization, memory, fan speed, power draw",
        "systemd" => "Systemd service status: running, failed, and degraded service counts",
        _ => "System information source",
    }
}

fn source_desc_ja(source: &str) -> &'static str {
    if source.starts_with("plugin/") {
        return "プラグイン提供メトリクス";
    }
    match source {
        "meminfo" => "メモリ使用状況: 合計、空き、利用可能、バッファ、キャッシュ、スワップ",
        "uptime" => "システム起動からの稼働時間とアイドル時間",
        "loadavg" => "CPU負荷平均: 1分、5分、15分間隔",
        "version" => "Linuxカーネルバージョンとビルド情報",
        "cpuinfo" => "CPU詳細: モデル、周波数、キャッシュ、機能",
        "stat" => "カーネル統計: CPU時間、コンテキストスイッチ、起動時刻",
        "mounts" => "マウント済みファイルシステムとオプション",
        "partitions" => "ブロックデバイスのパーティションテーブル",
        "diskstats" => "ディスクI/O統計: 読み込み、書き込み、処理中",
        "processes" => "プロセス一覧: PID、名前、状態、メモリ、スレッド数",
        "swaps" => "有効なスワップデバイスと使用状況",
        "net/dev" => "インターフェース別ネットワーク通信量: バイト、パケット、エラー",
        "net/tcp" => "TCP接続: ローカル/リモートアドレス、状態",
        "net/udp" => "UDPソケット: ローカル/リモートアドレス、状態",
        "net/unix" => "Unixドメインソケット: 種類、状態、パス",
        "net/arp" => "ARPキャッシュ: IPアドレスとMACアドレスの対応",
        "net/route" => "カーネルルーティングテーブル",
        "net/sockstat" => "ソケット統計概要: TCP、UDP、RAWの数",
        "net/snmp" => "SNMPカウンタ: IP、TCP、UDPプロトコル統計",
        "net/netstat" => "拡張ネットワーク統計",
        "net/wireless" => "無線インターフェース統計",
        "vmstat" => "仮想メモリ統計: ページフォルト、スワッピング、I/O",
        "buddyinfo" => "メモリ断片化: ゾーン別オーダー別空きページ数",
        "zoneinfo" => "ゾーン別メモリ詳細情報",
        "slabinfo" => "カーネルスラブアロケータのキャッシュ統計",
        "pagetypeinfo" => "ゾーン別ページ割り当て種別情報",
        "modules" => "ロード済みカーネルモジュールと依存関係",
        "interrupts" => "CPU別ハードウェア割り込みカウンタ",
        "softirqs" => "CPU別ソフトウェア割り込みカウンタ",
        "schedstat" => "CPUスケジューラ統計",
        "timer_list" => "アクティブなカーネルタイマー",
        "pressure" => "PSI: CPU、メモリ、I/Oの圧力ストール情報",
        "cgroups" => "利用可能なcgroupコントローラ",
        "cmdline" => "カーネル起動コマンドラインパラメータ",
        "consoles" => "登録済みコンソールデバイス",
        "crypto" => "利用可能な暗号アルゴリズム",
        "devices" => "登録済みキャラクタ/ブロックデバイス",
        "filesystems" => "サポートされているファイルシステム",
        "iomem" => "物理メモリマップとデバイス領域",
        "ioports" => "I/Oポートアドレス範囲",
        "locks" => "現在のファイルロック (flock/posix)",
        "misc" => "その他の登録済みデバイス",
        "dma" => "DMAチャネル使用状況",
        "df" => "ディスク使用状況: マウントポイント別の合計・使用量・空き容量",
        "thermal" => "CPU/GPU温度: サーマルゾーンからの温度情報",
        "file-nr" => "システム全体のファイルディスクリプタ割り当てと上限",
        "ip/route" => {
            "IPルーティングテーブル (ip route show): デフォルトゲートウェイ、経路、メトリクス"
        }
        "ip/neighbor" => "ARP/NDPネイバーキャッシュ (ip neighbor show): IP-MAC対応と状態",
        "ss" => "ソケット統計サマリ (ss -s): TCP確立/タイムウェイト/孤立、UDP数",
        "dns" => "DNS設定 (/etc/resolv.conf): ネームサーバ、検索ドメイン、名前解決時間",
        "conntrack" => "コネクション追跡テーブル使用状況: 現在数、上限、使用率",
        "gpu" => "NVIDIA GPUメトリクス: 温度、使用率、メモリ、ファン速度、消費電力",
        "systemd" => "systemdサービス状態: 実行中、失敗、劣化のサービス数",
        _ => "システム情報ソース",
    }
}

/// Field-level descriptions with detail levels.
/// Returns (normal, detailed, extra_detailed) for the given source+field.
/// Returns None if no override exists (fall back to parser's hardcoded description).
pub fn field_description(
    locale: Locale,
    source: &str,
    field: &str,
) -> Option<(&'static str, &'static str, &'static str)> {
    match locale {
        Locale::Ja => field_desc_ja(source, field),
        Locale::En => field_desc_en(source, field),
    }
}

fn field_desc_en(source: &str, field: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match (source, field) {
        // meminfo
        ("meminfo", "MemTotal") => (
            "Total usable RAM",
            "Total physical memory installed, minus reserved regions for kernel. This is the maximum memory available to the system.",
            "Total usable RAM (physical memory minus kernel-reserved regions).\n\nWhen MemAvailable drops below 10% of MemTotal, the OOM Killer may start terminating processes. On a 64GB server, that means trouble below ~6.4GB available.\n\n💡 Diagnostic: If MemTotal seems lower than your physical RAM, check BIOS memory reservations or `dmesg | grep Memory`.",
        ),
        ("meminfo", "MemFree") => (
            "Free memory (completely unused)",
            "Memory not used by anything. This is often low on healthy systems because Linux uses free RAM for disk cache. Don't panic if it's low — check MemAvailable instead.",
            "Completely unused memory — not even used for caches.\n\nA common misconception: 'My server has no free memory!' In reality, Linux aggressively uses free RAM as disk cache (Cached + Buffers). This is good — unused RAM is wasted RAM.\n\n💡 Diagnostic: If both MemFree AND MemAvailable are low, then you have a real memory problem. If only MemFree is low but MemAvailable is fine, your system is healthy.",
        ),
        ("meminfo", "MemAvailable") => (
            "Available memory for new processes",
            "Estimated memory available without swapping. This accounts for reclaimable cache and buffers. This is the number you should watch, not MemFree.",
            "The kernel's estimate of how much memory can be allocated without swapping.\n\nThis is THE metric to watch for memory pressure. It includes free memory plus reclaimable page cache minus reserved watermarks.\n\n💡 Diagnostic:\n  • < 10% of MemTotal → OOM danger zone, investigate immediately\n  • < 20% → Warning, check for memory leaks (RSS growing processes)\n  • Steadily decreasing → Likely a memory leak. Run `ps aux --sort=-rss | head` to find the culprit.",
        ),
        ("meminfo", "Buffers") => (
            "Memory used for block device buffers",
            "Memory used for raw block device I/O (metadata like filesystem structures). Usually small. Will be reclaimed under memory pressure.",
            "Raw block device I/O buffers (filesystem metadata, superblocks, etc.).\n\nThis is separate from page cache (Cached). Buffers hold disk metadata, while Cached holds file content. Both are reclaimable.\n\n💡 If Buffers is unusually large, you may have many block devices or heavy metadata operations (like `find /` or massive directory listings).",
        ),
        ("meminfo", "Cached") => (
            "Memory used for file cache",
            "Page cache — files read from disk are kept in memory for faster subsequent access. This is reclaimable under memory pressure. High values are normal and good.",
            "Page cache — the kernel's file content cache.\n\nEvery file read from disk is cached here. This is why 'free' memory looks low on a healthy Linux system — the kernel is doing its job by caching frequently accessed files.\n\n💡 Diagnostic: If Cached drops to near-zero while the system is under load, you're thrashing — the kernel can't keep files in cache because RAM is genuinely exhausted. This causes massive I/O amplification.",
        ),
        ("meminfo", "SwapTotal") => (
            "Total swap space",
            "Total swap space available (disk-based virtual memory). Used when physical RAM is exhausted.",
            "Total configured swap space.\n\nSwap acts as overflow for RAM — inactive pages get written to disk to free up physical memory. Some swap usage is normal, but heavy swap activity (check `vmstat si/so`) means RAM is insufficient.\n\n💡 Diagnostic:\n  • SwapTotal = 0 → No swap configured. OOM Killer is your only safety net.\n  • Swap on SSD → Acceptable. Swap on spinning disk → Severe performance hit under pressure.",
        ),
        ("meminfo", "SwapFree") => (
            "Free swap space",
            "Unused swap space. If this is decreasing over time, the system is under memory pressure and actively swapping.",
            "Remaining unused swap.\n\n💡 Diagnostic:\n  • SwapTotal - SwapFree > 0 but stable → Some pages were swapped out once, normal.\n  • SwapFree steadily decreasing → Active swapping, system is under real memory pressure.\n  • SwapFree = 0 → Next allocation that can't fit in RAM triggers OOM Killer.",
        ),

        // loadavg
        ("loadavg", "load1") => (
            "1-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 1 minute. Compare with CPU count to assess saturation.",
            "1-minute CPU load average.\n\nLoad average counts processes that are: (1) currently running on a CPU, or (2) waiting for CPU time, or (3) in uninterruptible I/O wait (D state).\n\n💡 Diagnostic:\n  • load1 < CPU_count → System has spare capacity\n  • load1 ≈ CPU_count → Fully utilized, acceptable\n  • load1 > CPU_count × 2 → Severely overloaded. Processes are queuing.\n  • load1 >> load15 → Load is spiking RIGHT NOW\n  • load1 << load15 → Load is recovering from a recent spike",
        ),
        ("loadavg", "load5") => (
            "5-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 5 minutes. More stable than 1-minute average.",
            "5-minute load average — the 'trend' indicator.\n\nCompare load1 vs load5 to see direction:\n  • load1 > load5 → Load is increasing\n  • load1 < load5 → Load is decreasing\n  • load1 ≈ load5 → Stable load",
        ),
        ("loadavg", "load15") => (
            "15-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 15 minutes. Best for understanding baseline system load.",
            "15-minute load average — the 'baseline'.\n\nThis tells you what 'normal' looks like for this system. If load15 is always around 4.0, then load1 jumping to 8.0 is notable but load1 at 4.5 is routine.\n\n💡 If load15 has been high for a long time, the system may need more CPUs, or there's a persistent I/O bottleneck (check `pressure` for PSI data).",
        ),

        // net/tcp
        ("net/tcp", "connections") => (
            "Active TCP connections",
            "Table of all TCP connections with local/remote addresses and states. Key states: ESTABLISHED (active), TIME_WAIT (closing), SYN_SENT (connecting).",
            "All TCP connections on the system.\n\n💡 Diagnostic patterns by connection state:\n  • Many SYN_SENT → Outbound connections timing out. Target host is down, firewall dropping packets, or DNS is slow.\n  • Many TIME_WAIT → Server is handling lots of short-lived connections. Normal for HTTP servers, but excessive counts (>10000) may exhaust ephemeral ports.\n  • Many CLOSE_WAIT → YOUR application isn't closing sockets. This is a bug — the remote end closed, but your code never called close(). Classic FD leak.\n  • Many ESTABLISHED to same IP → Connection pooling or persistent connections. Normal for database connections.",
        ),

        // processes
        ("processes", "processes") => (
            "Running processes",
            "Table of all processes: PID, name, state, RSS memory, threads, UID. States: S=sleeping, R=running, Z=zombie, D=uninterruptible I/O.",
            "All processes on the system.\n\n💡 Diagnostic patterns:\n  • Z (Zombie) processes accumulating → Parent process not calling wait(). The zombies themselves use no resources, but indicate a buggy parent.\n  • D (Disk sleep) processes stuck → Uninterruptible I/O wait. Often NFS hangs, disk failures, or kernel driver bugs. These can't be killed with SIGKILL.\n  • High RSS on a single process growing over time → Memory leak. Compare RSS now vs. 1 hour ago.\n  • Many threads (>1000) on one process → Thread leak or thread-per-connection architecture under high load.\n  • UID 0 processes → Running as root. Security concern if unexpected.",
        ),

        // pressure (PSI)
        ("pressure", "cpu_some_avg10") => (
            "CPU pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on CPU in the last 10 seconds. Values above 25% indicate CPU contention.",
            "CPU PSI (Pressure Stall Information) — 10-second average.\n\n'some' means at least one runnable task couldn't get CPU time. This is more precise than load average because it directly measures stall time.\n\n💡 Diagnostic:\n  • < 5% → Healthy, plenty of CPU headroom\n  • 5-25% → Moderate pressure, some tasks waiting\n  • > 25% → Significant CPU contention, performance degrading\n  • > 50% → Severe — half the time, tasks are waiting for CPU",
        ),
        ("pressure", "memory_some_avg10") => (
            "Memory pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on memory in the last 10 seconds. Non-zero values indicate memory pressure or active swapping.",
            "Memory PSI — 10-second average.\n\nUnlike CPU pressure, ANY non-zero memory pressure is noteworthy. It means the system is actively reclaiming memory or swapping.\n\n💡 Diagnostic:\n  • 0% → No memory pressure at all\n  • > 0% → System is reclaiming cache or swapping. Check MemAvailable.\n  • > 10% → Significant memory pressure. Tasks are stalling due to memory reclaim.\n  • > 40% → Critical. System is thrashing. Performance severely degraded.",
        ),
        ("pressure", "io_some_avg10") => (
            "I/O pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on I/O in the last 10 seconds. Indicates disk bottleneck.",
            "I/O PSI — 10-second average.\n\nMeasures how often tasks are blocked waiting for disk/storage I/O.\n\n💡 Diagnostic:\n  • < 5% → Normal I/O activity\n  • 5-20% → I/O is becoming a bottleneck. Check diskstats for high await times.\n  • > 20% → Significant I/O bottleneck. Consider: SSD upgrade, I/O scheduler tuning, reducing write amplification, or adding RAM for larger page cache.\n  • Spikes correlating with load spikes → I/O-bound workload. Load average includes D-state processes, so high load + high I/O pressure = I/O problem, not CPU.",
        ),

        // uptime
        ("uptime", "uptime") => (
            "System uptime since boot",
            "Total time the system has been running since last boot. Includes time spent in suspend/hibernate on some kernels.",
            "System uptime since boot.\n\nThis is the wall-clock time since the kernel started. It does NOT reset on suspend on modern kernels (3.x+), but behavior varies.\n\n💡 Diagnostic:\n  • Very short uptime on a production server → Unexpected reboot. Check `dmesg` and `/var/log/kern.log` for panic/oops.\n  • Extremely long uptime (months/years) → Kernel may be missing critical security patches. Plan a maintenance window.",
        ),
        ("uptime", "idle") => (
            "Total idle time across all CPUs",
            "Cumulative time all CPUs spent idle since boot. On multi-core systems, this can exceed uptime because it sums all cores.",
            "Total idle time summed across all CPU cores.\n\nOn a 4-core system, if idle = 3 * uptime, that means on average 3 out of 4 cores were idle.\n\n💡 Diagnostic: idle < uptime on a multi-core system is impossible — if you see this, the value may be from a single-core VM or something is wrong with the clock source.",
        ),
        ("uptime", "idle_pct") => (
            "Idle percentage (idle / uptime)",
            "Ratio of total idle time to uptime. On multi-core systems this is (idle / uptime * 100), so values above 100% are normal for mostly-idle machines.",
            "Idle percentage = idle_time / uptime * 100.\n\nBecause idle is summed across all CPUs, this can exceed 100% on multi-core systems. A 4-core idle system would show ~400%.\n\n💡 Diagnostic:\n  • idle_pct / cpu_count < 20% → System is heavily loaded\n  • idle_pct / cpu_count > 80% → System is mostly idle\n  • Values dropping over time → Workload is increasing",
        ),

        // version
        ("version", "raw") => (
            "Full kernel version string",
            "Complete /proc/version output including kernel version, build host, compiler, and build date.",
            "The full kernel version string as reported by the kernel.\n\nContains kernel version, who built it, which compiler was used, and when.\n\n💡 Useful for verifying that the correct kernel is running after an update, or for support tickets that need the exact kernel build info.",
        ),
        ("version", "kernel_version") => (
            "Kernel version number",
            "The Linux kernel version (e.g., 6.6.87). This determines available features, syscall support, and driver compatibility.",
            "The Linux kernel version (e.g., 6.6.87).\n\nVersion format: major.minor.patch. The minor number determines the feature set, and patches contain bug/security fixes.\n\n💡 Diagnostic:\n  • Check against CVE databases to ensure critical vulnerabilities are patched.\n  • Kernel < 5.x may lack modern features like PSI (pressure stall info), io_uring, etc.\n  • Compare with `uname -r` — they should match.",
        ),
        ("version", "compiler") => (
            "Compiler used to build the kernel",
            "The compiler (typically GCC) and its version used to compile the running kernel. Relevant for debugging ABI compatibility.",
            "The compiler that built this kernel.\n\nTypically GCC, but some distributions use Clang/LLVM. The compiler version matters for:\n  • ABI compatibility with out-of-tree kernel modules (e.g., NVIDIA drivers)\n  • Compiler-specific optimizations affecting performance\n\n💡 If kernel modules fail to load with 'version magic' errors, compiler mismatch is a common cause.",
        ),

        // cmdline
        ("cmdline", "cmdline") => (
            "Kernel boot parameters",
            "The command line passed to the kernel at boot by the bootloader (GRUB, systemd-boot, etc.). Controls hardware settings, security features, and debug options.",
            "Kernel boot command line from the bootloader.\n\nCommon important parameters:\n  • root= — Root filesystem device\n  • quiet/splash — Suppress boot messages\n  • nomodeset — Disable kernel mode setting (GPU troubleshooting)\n  • mitigations=off — Disable CPU vulnerability mitigations (insecure but faster)\n  • crashkernel= — Memory reserved for kdump\n\n💡 Diagnostic: If performance is unexpectedly poor, check for missing 'mitigations=off' or debug parameters that add overhead.",
        ),
        ("cmdline", "param_count") => (
            "Number of boot parameters",
            "Count of space-separated kernel boot parameters. Useful for a quick overview of boot config complexity.",
            "Number of space-separated tokens in the kernel command line.\n\n💡 A very large param_count may indicate custom kernel tuning or workarounds for hardware issues. Minimal systems typically have 5-15 parameters.",
        ),

        // stat
        ("stat", "cpu_user") => (
            "CPU time in user mode",
            "Cumulative time (in jiffies) all CPUs spent executing user-space code since boot. Includes application code but not kernel syscalls.",
            "Cumulative user-mode CPU time since boot (jiffies = typically 1/100th sec).\n\nUser time includes all application code execution. This is the 'us' column in `top`.\n\n💡 Diagnostic:\n  • High user% with low system% → Application is CPU-bound (computation-heavy)\n  • Compare with cpu_system: user >> system is normal for compute workloads\n  • Watch the rate of change between snapshots, not the absolute value",
        ),
        ("stat", "cpu_system") => (
            "CPU time in kernel mode",
            "Cumulative time all CPUs spent in kernel (system) code since boot. Includes syscalls, interrupts, and kernel threads.",
            "Cumulative kernel-mode CPU time since boot.\n\nSystem time = time spent in syscalls, interrupt handlers, and kernel threads. The 'sy' column in `top`.\n\n💡 Diagnostic:\n  • High system% → Heavy syscall usage (lots of I/O, context switches, or network ops)\n  • system > user → Unusual. Could indicate excessive syscalls (e.g., millions of tiny reads instead of buffered I/O)\n  • Sudden system% spike → Check for interrupt storms or kernel driver issues",
        ),
        ("stat", "cpu_idle") => (
            "CPU idle time",
            "Cumulative time all CPUs spent idle since boot. High idle means the CPU has spare capacity.",
            "Cumulative CPU idle time since boot.\n\nThe CPU was doing nothing and no tasks were runnable.\n\n💡 Diagnostic:\n  • Idle steadily near 0 → CPU is fully saturated\n  • Compare idle rate between snapshots: (idle_delta / total_delta * 100) gives real-time idle%\n  • On multi-core: this is summed, so max idle per second = number_of_cores * 100 jiffies",
        ),
        ("stat", "cpu_iowait") => (
            "CPU time waiting for I/O",
            "Cumulative time CPUs spent idle while waiting for outstanding I/O. High iowait indicates a storage bottleneck.",
            "CPU time spent waiting for I/O completion.\n\niowait means the CPU had nothing to do AND there was outstanding I/O. This is a subset of idle — the CPU is idle, but blocked on disk.\n\n💡 Diagnostic:\n  • High iowait → Storage is the bottleneck, not CPU\n  • iowait can appear low on busy systems because other tasks fill the CPU while I/O completes\n  • Compare with pressure io_some_avg10 for a more accurate I/O bottleneck signal\n  • Spikes → Large sequential reads/writes or filesystem journal flushes",
        ),
        ("stat", "cpu_usage_pct") => (
            "Overall CPU usage percentage (cumulative)",
            "Percentage of CPU time spent doing useful work since boot. Calculated as (total - idle - iowait) / total * 100.",
            "Cumulative CPU usage percentage since boot.\n\nThis is (busy_time / total_time * 100) where busy = user + nice + system + irq + softirq + steal.\n\n💡 Note: This is a cumulative average since boot, not real-time usage. For real-time CPU%, compare delta values between two snapshots. A system that was idle for 23 hours then 100% busy for 1 hour will show ~4% here.",
        ),
        ("stat", "forks_total") => (
            "Total forks (process creations) since boot",
            "Number of times fork()/clone() has been called since boot. A high rate indicates many short-lived processes.",
            "Total fork()/clone() calls since boot.\n\nEvery process or thread creation increments this counter.\n\n💡 Diagnostic:\n  • High fork rate (delta between snapshots) → Shell scripts spawning many subprocesses, cron jobs, or a fork-bomb\n  • Compare with context_switches: high forks + high context switches = lots of short-lived processes\n  • Steady growth is normal; sudden spikes warrant investigation",
        ),
        ("stat", "procs_running") => (
            "Processes currently running on CPU",
            "Number of processes in the R (running/runnable) state right now. Consistently above CPU count means the CPU is overloaded.",
            "Number of processes currently in R (running) state.\n\nThese are either actively executing on a CPU or in the run queue waiting for CPU time.\n\n💡 Diagnostic:\n  • procs_running <= cpu_count → Normal, CPUs can service all runners\n  • procs_running > cpu_count → Tasks are queuing for CPU time\n  • Persistently high → CPU bottleneck, correlates with load average",
        ),
        ("stat", "procs_blocked") => (
            "Processes blocked on I/O",
            "Number of processes in the D (uninterruptible sleep) state. These are waiting for I/O and cannot be interrupted, even by signals.",
            "Processes in D state (uninterruptible sleep), blocked on I/O.\n\nThese processes are waiting for disk, network filesystem (NFS), or device I/O.\n\n💡 Diagnostic:\n  • procs_blocked > 0 temporarily → Normal during I/O operations\n  • procs_blocked persistently high → I/O bottleneck. Check diskstats and pressure.\n  • D-state processes stuck for minutes → NFS hang, dead disk, or kernel driver bug. These CANNOT be killed with SIGKILL.",
        ),
        ("stat", "context_switches") => (
            "Total context switches since boot",
            "Number of CPU context switches since boot. Each switch saves/restores process state. A very high rate can indicate excessive multitasking overhead.",
            "Total CPU context switches since boot.\n\nA context switch happens when the CPU changes from one process/thread to another. Both voluntary (blocking I/O) and involuntary (preemption) switches are counted.\n\n💡 Diagnostic:\n  • Normal rate: 1000-50000/sec depending on workload\n  • > 100000/sec → High. Many threads contending or excessive I/O operations.\n  • Correlates with high system% CPU — each switch has kernel overhead\n  • Compare delta between snapshots for the current rate",
        ),

        // cpuinfo
        ("cpuinfo", "logical_cpus") => (
            "Number of logical CPUs (threads)",
            "Total logical processors visible to the OS. Includes hyperthreading — two logical CPUs may share one physical core.",
            "Total logical CPUs (hardware threads) visible to the OS.\n\nWith Hyperthreading/SMT enabled, logical_cpus = physical_cores * 2 (typically). Without HT, logical_cpus = physical_cores.\n\n💡 Diagnostic:\n  • Compare with cores_per_socket to detect Hyperthreading\n  • This is the number you compare load average against\n  • If lower than expected, check BIOS settings or kernel parameters (maxcpus=, nr_cpus=)",
        ),
        ("cpuinfo", "model") => (
            "CPU model name",
            "Full CPU model identifier as reported by the processor. Includes brand, generation, and variant information.",
            "CPU model name from the processor's CPUID instruction.\n\nExamples: 'Intel(R) Core(TM) i9-13900K', 'AMD EPYC 9654'.\n\n💡 Useful for:\n  • Identifying hardware generation and expected performance\n  • Checking if the CPU supports required instruction sets (AVX-512, etc.)\n  • Verifying that VMs are exposing the correct CPU model",
        ),
        ("cpuinfo", "frequency") => (
            "Current CPU frequency in MHz",
            "Current operating frequency of the CPU. May vary due to frequency scaling (turbo boost, power saving).",
            "Current CPU frequency in MHz.\n\nModern CPUs dynamically adjust frequency based on load (P-states). The reported value may be:\n  • Base frequency under light load (power saving)\n  • Turbo/boost frequency under heavy load\n  • Capped by thermal throttling\n\n💡 Diagnostic:\n  • Frequency much lower than rated → Check governor: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`\n  • 'powersave' governor → May limit performance. Switch to 'performance' for benchmarks.\n  • Frequency not reaching turbo → Thermal throttling. Check CPU temperature.",
        ),
        ("cpuinfo", "cache_size") => (
            "CPU cache size (L2/L3)",
            "Size of the last-level cache reported per core. Larger caches improve performance for memory-intensive workloads.",
            "Last-level cache size as reported by /proc/cpuinfo.\n\nThis is typically the L3 cache shared across cores, or L2 per-core on some architectures.\n\n💡 Diagnostic:\n  • Larger cache = better performance for working sets that fit\n  • Database workloads benefit significantly from large L3 cache\n  • For detailed cache topology, check `/sys/devices/system/cpu/cpu0/cache/`",
        ),
        ("cpuinfo", "cores_per_socket") => (
            "Physical cores per CPU socket",
            "Number of physical CPU cores per socket. With Hyperthreading, each core appears as 2 logical CPUs.",
            "Physical CPU cores per socket.\n\nPhysical cores have their own execution units. Hyperthreading creates 2 logical CPUs per physical core by sharing execution resources.\n\n💡 Diagnostic:\n  • logical_cpus / cores_per_socket = 2 → Hyperthreading is enabled\n  • logical_cpus / cores_per_socket = 1 → Hyperthreading is disabled or unavailable\n  • For HPC/latency-sensitive workloads, disabling HT can sometimes improve performance",
        ),
        ("cpuinfo", "key_flags") => (
            "Key CPU feature flags",
            "Important CPU capability flags: SSE/AVX (SIMD), aes (encryption), vmx/svm (virtualization), ht (hyperthreading), lm (64-bit).",
            "Key CPU feature flags from /proc/cpuinfo.\n\nNotable flags:\n  • sse/sse2/avx/avx2/avx512f → SIMD instruction sets (important for numeric workloads)\n  • aes → Hardware AES encryption (critical for TLS performance)\n  • vmx (Intel) / svm (AMD) → Hardware virtualization support\n  • ht → Hyperthreading capable\n  • lm → Long mode (64-bit support)\n  • nx → No-eXecute bit (security: prevents executing data pages)\n  • hypervisor → Running inside a VM\n\n💡 Diagnostic: 'hypervisor' flag present → This is a VM. Missing 'vmx'/'svm' → Nested virtualization not available.",
        ),

        // vmstat
        ("vmstat", "pgfault") => (
            "Total page faults since boot",
            "Total minor + major page faults. Minor faults are resolved from memory; major faults require disk I/O.",
            "Total page faults (minor + major) since boot.\n\nMinor faults: page exists in memory but not mapped in the process's page table. Resolved instantly — normal and frequent.\nMajor faults: page must be read from disk. These are expensive.\n\n💡 Diagnostic: High pgfault is normal. Focus on pgmajfault for real I/O-causing faults.",
        ),
        ("vmstat", "pgmajfault") => (
            "Major page faults (required disk I/O)",
            "Page faults that required reading from disk. Each one causes a process stall. High rates indicate insufficient RAM or cold cache.",
            "Major page faults since boot — each one required disk I/O.\n\nA major fault means the requested page was not in RAM and had to be fetched from storage. This stalls the faulting process.\n\n💡 Diagnostic:\n  • High pgmajfault rate (delta/sec) → RAM insufficient for working set, or cold start\n  • Correlates with high iowait and I/O pressure\n  • After a fresh boot, major faults spike as applications load — this is normal\n  • Persistent high rate → Add more RAM or reduce working set size",
        ),
        ("vmstat", "pgpgin") => (
            "Pages paged in from disk",
            "Total pages read from block devices into memory since boot. Includes normal file I/O and demand paging.",
            "Pages paged in from disk (in 1KB units).\n\nIncludes both file I/O (read() syscall causing page cache fill) and demand paging (loading executable pages).\n\n💡 Compare pgpgin rate with pgpgout rate: if pgpgin >> pgpgout, the workload is read-heavy.",
        ),
        ("vmstat", "pgpgout") => (
            "Pages paged out to disk",
            "Total pages written from memory to block devices since boot. Includes dirty page writeback and swap-out.",
            "Pages paged out to disk (in 1KB units).\n\nIncludes dirty page cache writeback (normal) and swap page-out (memory pressure).\n\n💡 Diagnostic: To distinguish normal writeback from swap pressure, check pswpout separately. High pgpgout with low pswpout = normal file writes.",
        ),
        ("vmstat", "pswpin") => (
            "Pages swapped in from swap",
            "Pages read back from swap into memory. Non-zero means the system previously swapped out and is now retrieving those pages.",
            "Pages swapped in from swap space.\n\nSwap-in happens when a process accesses a page that was previously swapped out due to memory pressure.\n\n💡 Diagnostic:\n  • pswpin rate > 0 → System is actively reading from swap. Performance impact depends on swap device speed.\n  • High pswpin + high pswpout → Thrashing — pages are being constantly swapped in and out. Critical condition.\n  • pswpin but no current pswpout → Recovering pages from a past memory pressure event. May be transient.",
        ),
        ("vmstat", "pswpout") => (
            "Pages swapped out to swap",
            "Pages moved from RAM to swap. Active swap-out indicates current memory pressure.",
            "Pages swapped out to swap space.\n\nSwap-out happens when the kernel needs to free RAM and has exhausted reclaimable caches.\n\n💡 Diagnostic:\n  • pswpout rate > 0 → Active memory pressure RIGHT NOW\n  • High sustained pswpout → System needs more RAM\n  • Intermittent pswpout → Temporary memory spikes, may be acceptable",
        ),
        ("vmstat", "nr_free_pages") => (
            "Free memory pages",
            "Number of completely free pages in the system. Low values are normal — Linux uses free pages for caching.",
            "Number of free (unused) memory pages.\n\nSimilar to MemFree in meminfo. Low values are expected because Linux uses free memory for page cache.\n\n💡 Diagnostic:\n  • Free pages below the 'min' watermark → Kernel enters direct reclaim, allocations may stall\n  • Check zoneinfo for per-zone free vs. watermark comparison",
        ),
        ("vmstat", "nr_active_anon") => (
            "Active anonymous pages",
            "Anonymous (non-file-backed) pages on the active LRU list. These are recently accessed heap/stack pages.",
            "Anonymous pages on the active LRU (Least Recently Used) list.\n\nAnonymous pages = process heap, stack, mmap(MAP_ANONYMOUS). 'Active' means recently accessed.\n\n💡 These pages can only be freed by swapping. High active anon = processes using lots of heap memory.",
        ),
        ("vmstat", "nr_inactive_anon") => (
            "Inactive anonymous pages",
            "Anonymous pages not recently accessed. These are candidates for swap-out under memory pressure.",
            "Anonymous pages on the inactive LRU list — not recently accessed.\n\nThese are first candidates for swap-out when memory pressure occurs.\n\n💡 Diagnostic: Large inactive_anon with no swap activity → Memory that could be reclaimed if needed. Large inactive_anon with active swapping → These pages are being swapped out.",
        ),
        ("vmstat", "nr_active_file") => (
            "Active file-backed pages",
            "File-backed pages recently accessed (page cache hot pages). These cache file contents for fast re-reads.",
            "File-backed pages on the active LRU — recently used page cache.\n\nThese pages cache file contents that were recently read. They improve I/O performance by avoiding disk reads.\n\n💡 Healthy systems have a large active file cache. Shrinking active_file under load means cache is being evicted due to memory pressure.",
        ),
        ("vmstat", "nr_inactive_file") => (
            "Inactive file-backed pages",
            "File-backed pages not recently accessed. Can be quickly reclaimed without I/O (unless dirty).",
            "File-backed pages on the inactive LRU — not recently accessed.\n\nThese are the first to be reclaimed under memory pressure, and reclaim is cheap (just drop the page, unless dirty).\n\n💡 Diagnostic: Large inactive_file = good buffer against memory pressure. Small inactive_file = less room for reclaim before swapping starts.",
        ),
        ("vmstat", "nr_dirty") => (
            "Dirty pages (modified, not yet written)",
            "Pages modified in memory but not yet written to disk. These will be written by the kernel's writeback mechanism.",
            "Pages with modifications not yet flushed to disk.\n\nThe kernel periodically flushes dirty pages (controlled by /proc/sys/vm/dirty_writeback_centisecs). If the system crashes, dirty pages are lost.\n\n💡 Diagnostic:\n  • High nr_dirty → Heavy write workload or slow storage\n  • nr_dirty above dirty_ratio → Processes will block on write() until pages are flushed (write throttling)\n  • Persistently high → Storage cannot keep up with write rate",
        ),
        ("vmstat", "nr_writeback") => (
            "Pages currently being written to disk",
            "Pages actively being flushed to storage right now. High values indicate heavy I/O activity.",
            "Pages currently being written back to storage.\n\nThese pages are in-flight to disk. The number depends on storage speed and write volume.\n\n💡 Diagnostic:\n  • nr_writeback > 0 most of the time → Constant write pressure\n  • Very high nr_writeback → Storage device is saturated, writes are backing up\n  • Check with diskstats for device-level I/O metrics",
        ),
        ("vmstat", "nr_slab_reclaimable") => (
            "Reclaimable slab pages",
            "Kernel slab allocator pages that can be freed under memory pressure. Includes dentry cache and inode cache.",
            "Reclaimable slab memory (kernel caches).\n\nPrimarily dentry cache (directory entries) and inode cache. These speed up filesystem operations and are released when memory is needed.\n\n💡 Diagnostic: Very large reclaimable slab on a fileserver is normal — it's caching millions of directory entries. The kernel will shrink it automatically under pressure.",
        ),
        ("vmstat", "nr_slab_unreclaimable") => (
            "Unreclaimable slab pages",
            "Kernel slab pages that cannot be freed. These are active kernel data structures that must remain in memory.",
            "Non-reclaimable slab memory — active kernel objects.\n\nThese are kernel data structures in active use (task structs, network buffers, etc.) that cannot be freed.\n\n💡 Diagnostic:\n  • Steadily growing unreclaimable slab → Possible kernel memory leak\n  • Check slabinfo for which specific caches are growing\n  • Large unreclaimable slab with many network connections → Network buffer memory",
        ),
        ("vmstat", "oom_kill") => (
            "OOM killer invocations",
            "Number of times the OOM (Out of Memory) killer has been triggered since boot. Each invocation kills a process to free memory.",
            "OOM (Out of Memory) killer invocations since boot.\n\nThe OOM killer is the kernel's last resort — when all memory (RAM + swap) is exhausted, it kills a process to survive.\n\n💡 Diagnostic:\n  • Any non-zero value deserves investigation\n  • Check `dmesg | grep -i oom` for details on which processes were killed and why\n  • Prevent OOM: add swap, increase RAM, set memory limits via cgroups\n  • Protect critical processes: `echo -1000 > /proc/<pid>/oom_score_adj`",
        ),
        ("vmstat", "nr_mapped") => (
            "Pages mapped into page tables",
            "Pages currently mapped into at least one process's virtual address space. Includes both file-backed and anonymous mapped pages.",
            "Pages mapped into process page tables.\n\nThese are pages actively referenced by processes via their virtual memory mappings, including mmap'd files and shared libraries.\n\n💡 Diagnostic:\n  • High nr_mapped relative to total memory → Many processes sharing libraries or mmap'd files\n  • Steadily growing → Possible memory-mapped file leak or growing shared memory usage\n  • Compare with nr_anon_pages + nr_file_pages for the full memory picture",
        ),
        ("vmstat", "nr_shmem") => (
            "Shared memory pages (tmpfs, shmem)",
            "Pages used by shared memory segments, tmpfs filesystems, and POSIX shared memory. Counted as part of Cached in meminfo.",
            "Shared memory pages — tmpfs, POSIX shmem, and SysV shared memory.\n\nThese pages appear in Cached (meminfo) but are NOT reclaimable like normal page cache — they persist until explicitly freed.\n\n💡 Diagnostic:\n  • High nr_shmem → Check tmpfs mounts (`df -h /dev/shm`, `/tmp` if tmpfs)\n  • Common consumers: databases (PostgreSQL shared buffers), web browsers, Docker overlays\n  • Unlike regular cache, shmem counts against MemAvailable",
        ),
        ("vmstat", "nr_anon_pages") => (
            "Total anonymous pages",
            "Pages allocated for process heap, stack, and private anonymous mappings. These can only be freed by the process exiting or by swapping.",
            "Total anonymous (non-file-backed) pages in use.\n\nAnonymous pages hold process private data: heap (malloc), stack, and mmap(MAP_ANONYMOUS|MAP_PRIVATE).\n\n💡 Diagnostic:\n  • nr_anon_pages growing over time → Possible memory leak in one or more processes\n  • Large nr_anon_pages + low MemAvailable → Processes consuming most RAM\n  • To find the culprit: `ps aux --sort=-rss | head` or check /proc/<pid>/smaps_rollup",
        ),
        ("vmstat", "allocstall_normal") => (
            "Allocation stalls in Normal zone",
            "Times a process stalled waiting for memory from the Normal zone. Direct reclaim is triggered, causing latency spikes.",
            "Allocation stalls in the Normal memory zone.\n\nWhen free pages drop below the low watermark, new allocations trigger direct reclaim — the allocating process must wait while the kernel frees pages. This causes latency spikes.\n\n💡 Diagnostic:\n  • allocstall increasing → System is under memory pressure\n  • Correlates with high latency in applications\n  • High stall rate → kswapd cannot keep up; consider adding RAM\n  • Compare with pgscan_direct and pgsteal_direct for the full picture",
        ),

        // buddyinfo
        ("buddyinfo", "zone_count") => (
            "Number of memory zones",
            "Count of memory zones in the buddy allocator. Common zones: DMA, DMA32, Normal, and optionally Movable.",
            "Number of memory zones tracked by the buddy allocator.\n\nTypical zones:\n  • DMA — First 16MB, for legacy ISA devices\n  • DMA32 — First 4GB, for 32-bit DMA devices\n  • Normal — Main memory zone\n  • Movable — Pages that can be migrated (for memory hotplug)\n\n💡 More zones on NUMA systems (one set per NUMA node).",
        ),
        ("buddyinfo", "zones") => (
            "Free page chunks per zone and order",
            "Memory fragmentation data: for each zone, shows free chunks at each order (0-10). Order N = 2^N contiguous pages (4KB each).",
            "Buddy allocator free page counts per zone per order.\n\nOrder 0 = 4KB, Order 1 = 8KB, ... Order 10 = 4MB. Higher orders represent larger contiguous free blocks.\n\n💡 Diagnostic:\n  • Many order-0 pages but zero higher orders → Memory is fragmented. Large allocations will fail or require compaction.\n  • Zero pages at all orders in a zone → Zone is exhausted\n  • Important for huge pages: 2MB huge pages need order-9 (x86). If order-9 count is 0, transparent huge pages will fail.\n  • Run `echo 1 > /proc/sys/vm/compact_memory` to trigger compaction.",
        ),

        // zoneinfo
        ("zoneinfo", "zone_count") => (
            "Number of memory zones",
            "Count of memory zones with detailed watermark information. Each zone has min/low/high watermarks controlling reclaim behavior.",
            "Number of memory zones in zoneinfo.\n\nEach zone has watermarks that control page reclaim:\n  • min — Below this, allocation stalls (direct reclaim)\n  • low — kswapd starts background reclaim\n  • high — kswapd stops reclaiming\n\n💡 More zones appear on NUMA systems with multiple nodes.",
        ),
        ("zoneinfo", "zones") => (
            "Per-zone memory details (free, min, low, high)",
            "Table of memory zones showing free pages and watermark thresholds. When free drops below 'low', background reclaim starts.",
            "Detailed per-zone memory information.\n\nColumns: zone identifier, free pages, min watermark, low watermark, high watermark.\n\n💡 Diagnostic:\n  • free < min → Direct reclaim active. Allocations are stalling — processes may block.\n  • free < low → kswapd is running (background reclaim). Normal under moderate pressure.\n  • free > high → No memory pressure in this zone.\n  • Check each zone independently — memory pressure can be zone-specific (e.g., DMA32 exhausted while Normal is fine).",
        ),

        // slabinfo
        ("slabinfo", "cache_count") => (
            "Number of slab caches",
            "Total number of kernel slab allocator caches. Each cache serves a specific type of kernel object (inodes, dentries, buffers, etc.).",
            "Number of active slab caches in the kernel.\n\nThe slab allocator provides efficient allocation for fixed-size kernel objects. Each cache pools objects of the same type.\n\n💡 Diagnostic: A very high cache count may indicate many loaded kernel modules, each registering their own caches.",
        ),
        ("slabinfo", "caches") => (
            "Slab cache details",
            "Table of slab caches: name, active objects, total objects, object size, objects per slab, pages per slab.",
            "Detailed slab cache statistics.\n\nColumns: cache name, active objects, total allocated objects, object size (bytes), objects per slab page, pages per slab.\n\n💡 Diagnostic:\n  • dentry cache very large → Lots of filesystem paths cached. Normal on fileservers.\n  • inode_cache growing → Many unique files accessed.\n  • active_objs << num_objs → Memory waste. Many pre-allocated but unused objects.\n  • Look for unknown caches growing → Possible kernel module memory leak.\n  • To reclaim slab caches: `echo 2 > /proc/sys/vm/drop_caches` (dentry+inode only).",
        ),

        // pagetypeinfo
        ("pagetypeinfo", "entry_count") => (
            "Number of pagetype info entries",
            "Count of entries in the page type breakdown. Each entry shows free page counts per migration type per zone.",
            "Number of pagetype info entries.\n\nEntries are broken down by NUMA node, memory zone, and migration type (Unmovable, Movable, Reclaimable, etc.).\n\n💡 Migration types affect defragmentation: Movable pages can be relocated to create contiguous blocks.",
        ),
        ("pagetypeinfo", "entries") => (
            "Page allocation type breakdown per zone",
            "Free page counts by migration type (Unmovable, Movable, Reclaimable) per zone per order. Shows how pages are categorized for compaction.",
            "Detailed page allocation type info.\n\nShows how free pages at each order are distributed across migration types:\n  • Unmovable — Cannot be relocated (kernel allocations)\n  • Movable — Can be migrated for compaction (user pages)\n  • Reclaimable — Can be freed (page cache, slab)\n\n💡 Diagnostic:\n  • Lots of Unmovable fragments → Hard to compact. Fragmentation will persist.\n  • Movable pages dominate → Good for transparent huge pages and compaction.\n  • Use with buddyinfo to understand fragmentation causes.",
        ),

        // swaps
        ("swaps", "total_size") => (
            "Total swap space available",
            "Combined size of all swap areas (files and partitions). This is the maximum overflow space for RAM.",
            "Total swap space across all swap areas.\n\nSwap extends virtual memory beyond physical RAM. Pages that haven't been accessed recently get moved to swap to free up RAM.\n\n💡 Diagnostic:\n  • 0 bytes → No swap configured. OOM is the only option when RAM is full.\n  • Recommended: at least 1-2GB of swap even on large-RAM systems as a safety net.",
        ),
        ("swaps", "total_used") => (
            "Used swap space",
            "Amount of swap currently in use. Some swap usage is normal; watch the trend rather than the absolute value.",
            "Currently used swap space.\n\nSome swap usage is normal and doesn't indicate a problem — the kernel may have swapped out idle pages proactively.\n\n💡 Diagnostic:\n  • Stable used swap → Pages were swapped out once and remain there. Normal.\n  • Growing used swap → Active memory pressure. Check MemAvailable.\n  • Used swap close to total → Danger zone. Next allocation may trigger OOM.\n  • To see which processes use swap: `grep VmSwap /proc/*/status | sort -k2 -n`",
        ),
        ("swaps", "usage_pct") => (
            "Swap usage percentage",
            "Percentage of total swap currently in use. Below 50% is usually fine; above 80% warrants investigation.",
            "Swap usage as a percentage of total swap.\n\n💡 Diagnostic:\n  • 0% → No swap in use (or no swap configured)\n  • < 50% → Normal, especially if stable\n  • 50-80% → Elevated. Monitor the trend.\n  • > 80% → High. System may OOM if memory demand increases.\n  • 100% → Swap is full. Any new memory demand triggers OOM Killer.",
        ),
        ("swaps", "swap_areas") => (
            "Individual swap area details",
            "Table of swap areas: filename/device, type (partition/file), size, used, priority. Higher priority areas are used first.",
            "Individual swap area details.\n\nColumns: device/file path, type (partition or file), size, used, priority.\n\n💡 Diagnostic:\n  • Priority determines usage order — higher priority areas are used first\n  • Multiple swap areas with same priority → Round-robin (striped) across them for better performance\n  • Swap on SSD → Acceptable performance. Swap on HDD → Major latency under pressure.\n  • Swap file vs partition → Files are slightly slower but easier to resize",
        ),

        // ── Group 3: Network ──────────────────────────────────────────

        // net/dev
        ("net/dev", "total_rx") => (
            "Total bytes received across all interfaces.",
            "Cumulative receive byte counter for every network interface since boot. Includes loopback traffic.",
            "Total received bytes summed across all interfaces.\n\nThis counter resets on reboot. Compare two snapshots to derive throughput. Includes loopback (lo) traffic, so subtract lo if you need external-only numbers.\n\n💡 Diagnostic: If total_rx grows much faster than total_tx, the host is a consumer (downloads, database reads). The reverse pattern suggests a serving role (web server, NFS export).",
        ),
        ("net/dev", "total_tx") => (
            "Total bytes transmitted across all interfaces.",
            "Cumulative transmit byte counter for every network interface since boot. Includes loopback traffic.",
            "Total transmitted bytes summed across all interfaces.\n\nSame as total_rx but for the send path. Heavy TX with low RX often indicates a content-serving workload.\n\n💡 Diagnostic: A sudden spike in TX with no corresponding application change may indicate a compromised host exfiltrating data or participating in a DDoS amplification attack.",
        ),
        ("net/dev", "interface_count") => (
            "Number of network interfaces.",
            "Count of all network interfaces visible in /proc/net/dev, including loopback, virtual, and physical interfaces.",
            "Total network interface count.\n\nIncludes lo (loopback), physical NICs, bridges, veth pairs (containers), tun/tap devices (VPN), and bond interfaces.\n\n💡 Diagnostic: If this number is unexpectedly high, check for container sprawl (each container adds a veth pair). If unexpectedly low, a NIC driver may have failed to load — check `dmesg | grep -i eth`.",
        ),
        ("net/dev", "interfaces") => (
            "Per-interface traffic statistics.",
            "Table of all network interfaces showing name, received bytes, received packets, transmitted bytes, and transmitted packets.",
            "Per-interface network traffic breakdown.\n\nColumns: name, RX bytes, RX packets, TX bytes, TX packets.\n\n💡 Diagnostic:\n  • High RX/TX errors → Cable issues, duplex mismatch, or driver bugs\n  • One interface with zero traffic → Link may be down; check `ip link show`\n  • lo with heavy traffic → Lots of inter-process communication on localhost (common for databases)\n  • Large packet counts but small byte counts → Many small packets, possible SYN flood or chatty protocol",
        ),

        // net/udp
        ("net/udp", "socket_count") => (
            "Number of open UDP sockets.",
            "Count of all UDP sockets on the system. Unlike TCP, UDP is connectionless, so each entry represents a bound socket.",
            "Total UDP socket count.\n\nUDP sockets don't have connection states like TCP. Each entry is a socket bound to a local port, optionally associated with a remote address.\n\n💡 Diagnostic:\n  • High count with many different local ports → Possible DNS amplification or UDP-based scanning\n  • Common legitimate UDP users: DNS (port 53), NTP (port 123), SNMP (port 161), syslog (port 514)\n  • If socket_count keeps growing → Possible FD leak in a UDP application",
        ),
        ("net/udp", "sockets") => (
            "Active UDP sockets.",
            "Table of all UDP sockets showing local address, remote address, state, and UID of the owning process.",
            "All UDP sockets on the system.\n\nColumns: local_addr, remote_addr, state, uid.\n\n💡 Diagnostic:\n  • Remote address 0.0.0.0:0 → Socket is listening (not connected to a specific peer)\n  • Many sockets bound to same port → Multiple processes or SO_REUSEPORT\n  • UID 0 sockets → Running as root; check if expected\n  • UDP has no built-in reliability — packet loss is invisible at this level. Use net/snmp Udp_InErrors for drop detection.",
        ),

        // net/unix
        ("net/unix", "socket_count") => (
            "Number of Unix domain sockets.",
            "Count of all Unix domain sockets. These are used for fast inter-process communication on the same host.",
            "Total Unix domain socket count.\n\nUnix sockets are the preferred IPC mechanism for local communication (much faster than TCP loopback). Databases, display servers, and systemd all use them heavily.\n\n💡 Diagnostic:\n  • High count is normal on a modern systemd-based system (200+ is typical)\n  • Steadily growing count → Possible socket leak; a process is creating sockets without closing them\n  • Look for sockets with empty path → Abstract namespace sockets (prefixed with @)",
        ),
        ("net/unix", "sockets") => (
            "Unix domain socket details.",
            "Table of all Unix domain sockets showing reference count, type, state, inode, and path.",
            "All Unix domain sockets.\n\nColumns: refcount, type, state, inode, path.\n\n💡 Diagnostic:\n  • Type 1 = STREAM (like TCP), Type 2 = DGRAM (like UDP), Type 5 = SEQPACKET\n  • Sockets with well-known paths: /var/run/dbus/system_bus_socket (D-Bus), /run/systemd/journal/stdout (journald), /var/run/docker.sock (Docker)\n  • High refcount on a single socket → Many processes sharing it (normal for D-Bus)",
        ),

        // net/arp
        ("net/arp", "entry_count") => (
            "Number of ARP table entries.",
            "Count of IP-to-MAC address mappings in the kernel ARP cache. Each entry represents a recently-communicated neighbor on the local network.",
            "ARP cache entry count.\n\nThe ARP table maps IPv4 addresses to MAC (hardware) addresses for hosts on the same L2 network segment.\n\n💡 Diagnostic:\n  • Very high count (1000+) → Large flat network or ARP table poisoning/scanning\n  • Count keeps growing → Possible ARP storm or network scan in progress\n  • Stale entries (flags=0x0) → Neighbor went offline; kernel will eventually expire them",
        ),
        ("net/arp", "entries") => (
            "ARP table entries.",
            "Table of ARP cache entries showing IP address, hardware (MAC) address, flags, and network device.",
            "ARP cache contents.\n\nColumns: ip, hw_addr, flags, device.\n\n💡 Diagnostic:\n  • Flags 0x2 = complete (resolved), 0x6 = complete+permanent (static entry)\n  • Duplicate MAC for different IPs → ARP spoofing or misconfigured network\n  • 00:00:00:00:00:00 MAC → Unresolved entry; host is unreachable at L2\n  • Multiple entries on unexpected interfaces → Possible VLAN or routing misconfiguration",
        ),

        // net/route
        ("net/route", "route_count") => (
            "Number of routing table entries.",
            "Count of entries in the kernel IPv4 routing table. Includes default gateway, directly connected networks, and static routes.",
            "IPv4 routing table entry count.\n\nEach entry tells the kernel how to reach a network: which interface to use and which gateway to forward through.\n\n💡 Diagnostic:\n  • 0 routes → No networking configured; host is isolated\n  • Missing default route (0.0.0.0 destination) → Host cannot reach the internet or other non-local networks\n  • Very high count → Complex routing setup or dynamic routing protocol (OSPF, BGP) injecting routes",
        ),
        ("net/route", "routes") => (
            "Kernel routing table.",
            "Table of routing entries showing interface, destination, gateway, mask, flags, and metric. The default route has destination 0.0.0.0.",
            "IPv4 kernel routing table.\n\nColumns: iface, destination, gateway, mask, flags, metric.\n\n💡 Diagnostic:\n  • Destination 0.0.0.0 with mask 0.0.0.0 → Default route (gateway of last resort)\n  • Gateway 0.0.0.0 → Directly connected network, no gateway needed\n  • Multiple default routes with different metrics → Failover configuration; lower metric = preferred\n  • Flags: U=up, G=gateway, H=host route. Missing U flag means route is down.",
        ),

        // net/sockstat
        ("net/sockstat", "sockets_used") => (
            "Total sockets in use.",
            "Total count of all socket types currently allocated by the kernel. This is a high-level indicator of network activity.",
            "Total allocated sockets across all protocols.\n\nThis is the grand total — TCP + UDP + RAW + FRAG + Unix domain sockets.\n\n💡 Diagnostic:\n  • Steadily growing → Possible socket/FD leak\n  • Compare with `ulimit -n` to check how close you are to the per-process FD limit\n  • System-wide limit: /proc/sys/fs/file-max",
        ),
        ("net/sockstat", "TCP_inuse") => (
            "TCP sockets in use.",
            "Number of TCP sockets currently in use (all states except TIME_WAIT). A key indicator of active network connections.",
            "TCP sockets currently in use.\n\nIncludes ESTABLISHED, SYN_SENT, SYN_RECV, FIN_WAIT, CLOSE_WAIT, LAST_ACK, LISTEN states — everything except TIME_WAIT (tracked separately).\n\n💡 Diagnostic:\n  • Compare with TCP_tw (TIME_WAIT) — if tw >> inuse, you have lots of short-lived connections\n  • Approaching /proc/sys/net/ipv4/tcp_max_orphans → Risk of connection drops\n  • Baseline this value; a sudden jump indicates a traffic spike or connection leak",
        ),
        ("net/sockstat", "TCP_orphan") => (
            "Orphaned TCP sockets.",
            "TCP sockets not attached to any process. These consume kernel memory and are waiting to be cleaned up.",
            "Orphaned TCP connections — no longer owned by any user-space process.\n\nOrphans happen when a process closes a socket that still has data in flight. The kernel keeps the socket alive to complete the TCP teardown.\n\n💡 Diagnostic:\n  • High orphan count → Application crashing or exiting without clean shutdown\n  • Limit: /proc/sys/net/ipv4/tcp_max_orphans (default ~16384)\n  • Exceeding the limit causes the kernel to aggressively reset connections, dropping data",
        ),
        ("net/sockstat", "TCP_tw") => (
            "TCP sockets in TIME_WAIT.",
            "TCP connections in TIME_WAIT state, waiting for late packets before fully closing. High counts are normal for busy HTTP servers.",
            "TCP TIME_WAIT socket count.\n\nTIME_WAIT is a normal TCP state — after closing, the socket waits 2*MSL (typically 60 seconds) to catch delayed packets.\n\n💡 Diagnostic:\n  • < 5000 → Normal\n  • 5000-30000 → Heavy short-lived connection workload; consider connection pooling\n  • > 30000 → May exhaust ephemeral ports (check /proc/sys/net/ipv4/ip_local_port_range)\n  • Enable tcp_tw_reuse (sysctl) to allow reuse of TIME_WAIT sockets for new outbound connections",
        ),
        ("net/sockstat", "TCP_alloc") => (
            "TCP sockets allocated.",
            "Total TCP sockets allocated by the kernel, including sockets in all states. This is the total TCP memory footprint.",
            "Total allocated TCP sockets.\n\nThis includes every TCP socket in any state (inuse + TIME_WAIT + orphan + listen).\n\n💡 Diagnostic:\n  • alloc >> inuse + tw → Many sockets in transitional states or kernel overhead\n  • Memory usage per socket: roughly 1-2 KB for basic, more with large buffers\n  • Total TCP memory limits: /proc/sys/net/ipv4/tcp_mem (in pages)",
        ),
        ("net/sockstat", "TCP_mem") => (
            "TCP memory usage (pages).",
            "Kernel memory pages consumed by all TCP sockets. Each page is typically 4KB.",
            "TCP memory consumption in kernel pages.\n\nMultiply by page size (usually 4096 bytes) to get bytes. This memory is used for socket buffers, control structures, and data in flight.\n\n💡 Diagnostic:\n  • Compare with /proc/sys/net/ipv4/tcp_mem thresholds (low, pressure, high)\n  • When usage exceeds the 'pressure' threshold, the kernel starts reducing buffer sizes\n  • When usage exceeds 'high', new allocations may fail → connection drops",
        ),
        ("net/sockstat", "UDP_inuse") => (
            "UDP sockets in use.",
            "Number of UDP sockets currently in use. Includes DNS resolvers, NTP clients, logging daemons, and game servers.",
            "Active UDP sockets.\n\nUDP is stateless at the protocol level, so 'in use' means a socket is bound and ready to send/receive.\n\n💡 Diagnostic:\n  • Typical values: 5-20 on a quiet server\n  • High values → Many UDP services or a DNS/NTP heavy workload\n  • UDP has no congestion control — a flood of UDP traffic can overwhelm the network without any backpressure",
        ),
        ("net/sockstat", "UDP_mem") => (
            "UDP memory usage (pages).",
            "Kernel memory pages consumed by UDP sockets. Usually much smaller than TCP since UDP has no connection state or retransmission buffers.",
            "UDP memory consumption in kernel pages.\n\nMultiply by page size (usually 4096) to get bytes.\n\n💡 Diagnostic:\n  • Compare with /proc/sys/net/ipv4/udp_mem limits\n  • If UDP_mem is high, check for applications with large receive buffers (e.g., video streaming receivers)\n  • Under memory pressure, the kernel drops incoming UDP packets silently — check net/snmp Udp_RcvbufErrors",
        ),
        ("net/sockstat", "FRAG_inuse") => (
            "IP fragment reassembly sockets.",
            "Number of IP fragment reassembly entries. Non-zero values indicate fragmented packets are being received.",
            "IP fragment reassembly queue entries.\n\nIP fragmentation occurs when a packet exceeds the MTU. The kernel holds fragments here until all pieces arrive for reassembly.\n\n💡 Diagnostic:\n  • Usually 0 on modern networks with path MTU discovery\n  • Non-zero → Some path has an MTU mismatch or PMTUD is blocked (ICMP filtered)\n  • Persistent high values → Possible fragmentation attack; check net/snmp Ip_ReasmFails",
        ),

        // net/snmp — key protocol counters
        ("net/snmp", "Tcp_ActiveOpens") => (
            "TCP connections initiated (client-side).",
            "Cumulative count of TCP connections where this host sent the initial SYN. Indicates outbound connection activity.",
            "TCP active opens — connections initiated by this host.\n\nEvery time your system connects to a remote server (HTTP request, database query, SSH), this counter increments.\n\n💡 Diagnostic:\n  • Compare with Tcp_PassiveOpens to understand if the host is primarily a client or server\n  • High rate of ActiveOpens → Busy client workload or connection churn (not pooling connections)\n  • Tcp_AttemptFails / Tcp_ActiveOpens = connection failure rate",
        ),
        ("net/snmp", "Tcp_PassiveOpens") => (
            "TCP connections accepted (server-side).",
            "Cumulative count of TCP connections accepted via listen/accept. Indicates inbound connection activity.",
            "TCP passive opens — connections accepted by listening sockets.\n\nEvery incoming client connection (web request, SSH login, database client) increments this.\n\n💡 Diagnostic:\n  • High PassiveOpens → Busy server\n  • PassiveOpens >> ActiveOpens → Primarily a server role\n  • Sudden drop in PassiveOpens rate → Clients can't reach the service (firewall, DNS, or service crash)",
        ),
        ("net/snmp", "Tcp_RetransSegs") => (
            "TCP segments retransmitted.",
            "Cumulative count of TCP segments retransmitted. Non-zero growth indicates packet loss on the network.",
            "TCP retransmission counter.\n\nRetransmissions occur when a sent segment is not acknowledged within the timeout. This is THE key indicator of network quality issues.\n\n💡 Diagnostic:\n  • RetransSegs / OutSegs = retransmission rate\n  • < 0.1% → Excellent network\n  • 0.1-1% → Moderate packet loss, noticeable latency\n  • > 1% → Severe packet loss. Check for: congested links, faulty cables, overloaded switches, MTU issues\n  • Sudden spike → Network event (link flap, route change, congestion)",
        ),
        ("net/snmp", "Tcp_InErrs") => (
            "TCP segments received with errors.",
            "Cumulative count of TCP segments received with checksum or other errors. Indicates data corruption in transit.",
            "TCP input errors — segments with invalid checksums or other protocol errors.\n\n💡 Diagnostic:\n  • Should be 0 or very close to 0\n  • Non-zero → Data corruption on the network path\n  • Possible causes: faulty NIC, bad cable, memory bit-flip (ECC failure), buggy network driver\n  • Compare with Udp_InErrors to see if the problem is protocol-specific or link-wide",
        ),
        ("net/snmp", "Udp_InErrors") => (
            "UDP datagrams that could not be delivered.",
            "Cumulative count of received UDP datagrams that could not be delivered (no matching socket, buffer overflow, checksum error).",
            "UDP input errors — datagrams that arrived but could not be delivered to an application.\n\n💡 Diagnostic:\n  • Common causes: no process listening on the destination port, receive buffer overflow\n  • Growing steadily → An application is too slow to read its UDP socket, causing kernel to drop packets\n  • Check Udp_RcvbufErrors specifically for buffer overflow drops\n  • For DNS servers: high InErrors = clients are sending queries faster than the server can process",
        ),
        ("net/snmp", "Ip_InReceives") => (
            "Total IP datagrams received.",
            "Cumulative count of all IP datagrams received, including those with errors. The top-level input counter for all network traffic.",
            "Total IP input datagrams — every packet that arrived at this host.\n\nThis is the grand total input counter before any protocol demuxing (TCP, UDP, ICMP).\n\n💡 Diagnostic:\n  • Rate of change (packets/sec) indicates overall network input load\n  • Ip_InReceives - Ip_InDelivers = packets dropped or forwarded\n  • If the host is not a router, InReceives should roughly equal InDelivers",
        ),
        ("net/snmp", "Ip_OutRequests") => (
            "Total IP datagrams sent.",
            "Cumulative count of all IP datagrams handed to the network layer for transmission. The top-level output counter.",
            "Total IP output datagrams — every packet sent by this host.\n\n💡 Diagnostic:\n  • Rate of change indicates overall network output load\n  • Ip_OutRequests >> Ip_InReceives → Host is generating more traffic than receiving (serving role)\n  • Sudden spike → New workload, backup job, or possibly compromised host",
        ),

        // net/netstat — extended TCP/IP stats
        ("net/netstat", "TcpExt_ListenOverflows") => (
            "Times the listen queue overflowed.",
            "Count of times a SYN was received but the listen backlog queue was full. Indicates the server cannot accept connections fast enough.",
            "TCP listen queue overflows.\n\nWhen a client SYN arrives and the server's accept queue (backlog) is full, this counter increments and the connection is dropped.\n\n💡 Diagnostic:\n  • Should be 0 under normal operation\n  • Non-zero → Server is too slow calling accept(), or backlog is too small\n  • Fix: Increase net.core.somaxconn and the application's listen backlog parameter\n  • Also check TcpExt_ListenDrops for total drops",
        ),
        ("net/netstat", "TcpExt_ListenDrops") => (
            "Connections dropped from listen queue.",
            "Count of connections dropped because the listen queue was full. Clients experience connection timeouts when this happens.",
            "TCP listen queue drops — connections lost because the server couldn't keep up.\n\n💡 Diagnostic:\n  • Non-zero = clients are being turned away\n  • Monitor the rate (drops/sec) rather than the absolute value\n  • Common during traffic spikes or application GC pauses\n  • Fix: tune somaxconn, optimize accept() path, reduce per-request latency",
        ),
        ("net/netstat", "TcpExt_TCPTimeouts") => (
            "TCP connection timeouts.",
            "Count of TCP connections that timed out waiting for a response. Indicates network issues or unresponsive remote hosts.",
            "TCP timeout events.\n\nA timeout occurs when a sent segment receives no ACK within the retransmission timeout (RTO). After several retries, the connection is aborted.\n\n💡 Diagnostic:\n  • Correlates with Tcp_RetransSegs but represents the final failure\n  • High timeouts + high retransmissions → Persistent network path failure\n  • High timeouts to specific hosts → Those hosts or the path to them is unreliable\n  • Check if timeouts correlate with specific times of day (congestion patterns)",
        ),

        // net/wireless
        ("net/wireless", "interface_count") => (
            "Number of wireless interfaces.",
            "Count of wireless (Wi-Fi) network interfaces detected by the kernel.",
            "Wireless interface count.\n\n💡 Diagnostic:\n  • 0 → No wireless hardware detected, or driver not loaded (check `lspci` and `modules`)\n  • Typically 1 on a laptop/desktop with Wi-Fi\n  • Multiple interfaces → Wi-Fi card with multiple radios or USB Wi-Fi adapter added",
        ),
        ("net/wireless", "interfaces") => (
            "Wireless interface statistics.",
            "Table of wireless interfaces with signal quality metrics: status, link quality, signal level, and noise level.",
            "Per-interface wireless statistics.\n\nColumns: iface, status, link quality, signal level (dBm), noise level (dBm).\n\n💡 Diagnostic:\n  • Link quality: higher is better (max varies by driver, often 70)\n  • Signal level (dBm): -30 = excellent, -67 = good, -70 = fair, -80 = weak, -90 = unusable\n  • Noise level: lower (more negative) is better\n  • SNR (signal - noise) > 25 dB → Good; < 15 dB → Poor, expect packet loss",
        ),

        // ── Group 4: Storage ──────────────────────────────────────────

        // mounts
        ("mounts", "count") => (
            "Number of mounted filesystems.",
            "Total count of all mounted filesystems, including virtual filesystems like proc, sysfs, tmpfs, and cgroup mounts.",
            "Total mounted filesystem count.\n\nIncludes physical disks, network mounts (NFS/CIFS), and virtual/pseudo filesystems (proc, sysfs, tmpfs, devtmpfs, cgroup).\n\n💡 Diagnostic:\n  • Typical Linux system: 30-60 mounts (many are virtual)\n  • Very high count (200+) → Container host with many mount namespaces, or NFS-heavy environment\n  • Missing expected mount → Filesystem failed to mount at boot; check `dmesg` and `systemctl --failed`",
        ),
        ("mounts", "mounts") => (
            "Mounted filesystem details.",
            "Table of all mounted filesystems showing device, mountpoint, filesystem type, and mount options.",
            "All currently mounted filesystems.\n\nColumns: device, mountpoint, fstype, options.\n\n💡 Diagnostic:\n  • Look for 'ro' in options → Filesystem remounted read-only, usually due to disk errors\n  • 'noatime' option → Reduces I/O by not updating access timestamps (good for SSDs)\n  • 'errors=remount-ro' → ext4 default; filesystem goes read-only on errors rather than continuing with corruption\n  • NFS mounts with 'hard' option → Processes will hang indefinitely if NFS server is unreachable\n  • tmpfs mounts → RAM-backed; check their size limits to avoid memory exhaustion",
        ),

        // partitions
        ("partitions", "count") => (
            "Number of block device partitions.",
            "Total count of all block device partitions recognized by the kernel, including whole disks and their sub-partitions.",
            "Block device partition count.\n\nIncludes whole disks (sda, nvme0n1) and their partitions (sda1, nvme0n1p1), plus device-mapper entries (dm-0), loop devices, and RAM disks.\n\n💡 Diagnostic:\n  • Check that all expected disks appear — a missing disk may indicate hardware failure or driver issue\n  • Unexpected entries → Hot-plugged USB device or new virtual disk\n  • Size of 0 → Partition table entry exists but has no allocated space",
        ),
        ("partitions", "partitions") => (
            "Partition table details.",
            "Table of all block device partitions showing name, size, and major/minor device numbers.",
            "Block device partition table.\n\nColumns: name, size, major, minor.\n\n💡 Diagnostic:\n  • Major 8 = SCSI/SATA disks (sd*), Major 259 = NVMe, Major 253 = device-mapper (LVM/LUKS)\n  • Compare partition sizes with `df` output — large partitions with small filesystems may have wasted space\n  • Loop devices (major 7) → Snap packages or mounted ISO images",
        ),

        // diskstats
        ("diskstats", "active_devices") => (
            "Devices with I/O activity.",
            "Number of block devices that have had at least one read or write operation since boot. Filters out inactive devices.",
            "Active block device count — devices with non-zero I/O.\n\nOnly devices with at least one completed read or write are counted, filtering out inactive loop devices and unused partitions.\n\n💡 Diagnostic:\n  • Compare with total partition count — many inactive devices is normal (loop devices, unused partitions)\n  • If an expected device shows zero I/O → It may not be in use, or the workload hasn't started",
        ),
        ("diskstats", "devices") => (
            "Per-device I/O statistics.",
            "Table of active block devices showing name, read count, bytes read, write count, bytes written, and in-flight I/O operations. Critical for identifying I/O bottlenecks.",
            "Per-device disk I/O statistics.\n\nColumns: name, reads completed, bytes read, writes completed, bytes written, I/O in-flight.\n\n💡 Diagnostic:\n  • in-flight > 0 sustained → Disk is under active I/O load. High sustained in-flight on HDD (>2) suggests saturation.\n  • Reads >> Writes → Read-heavy workload (database scans, file serving). Increase RAM for better page cache.\n  • Writes >> Reads → Write-heavy workload (logging, database inserts). Check if write-back caching is enabled.\n  • Compare whole disk (sda) vs partition (sda1) — the disk-level stats include all partitions.\n  • For SSDs: high write volume degrades lifespan. Check SMART data with `smartctl -a /dev/sdX`.\n  • Correlate with I/O pressure (PSI io_some_avg10) to determine if I/O is actually stalling processes.",
        ),

        // locks
        ("locks", "lock_count") => (
            "Number of active file locks.",
            "Count of all active file locks (POSIX and FLOCK types). Locks coordinate access between processes to prevent data corruption.",
            "Active file lock count.\n\nIncludes both POSIX locks (fcntl-based, byte-range granularity) and FLOCK locks (flock-based, whole-file). Also includes lease locks used by Samba.\n\n💡 Diagnostic:\n  • Typical values: 10-50 on a normal server\n  • Very high count → Database or file server under heavy concurrent access\n  • If an application hangs → Check if it's waiting on a lock held by another process. The PID column identifies the holder.",
        ),
        ("locks", "locks") => (
            "Active file lock details.",
            "Table of all active file locks showing type (POSIX/FLOCK), mode (READ/WRITE), PID of holder, inode info, and byte range.",
            "All active file locks.\n\nColumns: type, mode, pid, inode_info, range_start, range_end.\n\n💡 Diagnostic:\n  • WRITE locks block all other access — if a process holds a WRITE lock and is stuck, other processes will hang\n  • Range 0 to EOF → Whole-file lock\n  • Multiple READ locks on same inode → Shared read access, normal for concurrent readers\n  • To find which file an inode belongs to: `find / -inum <inode_number>`\n  • Deadlock: Process A holds lock on file X, waits for file Y; Process B holds Y, waits for X",
        ),

        // ── Group 5: Security & Kernel ────────────────────────────────

        // modules
        ("modules", "module_count") => (
            "Number of loaded kernel modules.",
            "Count of currently loaded kernel modules (drivers, filesystem modules, etc.). Modules extend kernel functionality without recompilation.",
            "Loaded kernel module count.\n\nKernel modules are dynamically loadable pieces of kernel code: device drivers, filesystem implementations, network protocols, crypto algorithms.\n\n💡 Diagnostic:\n  • Typical Linux server: 50-150 modules\n  • Unexpected module → Possible rootkit. Compare with known-good list.\n  • Missing expected module → Driver not loaded; try `modprobe <name>` and check `dmesg` for errors",
        ),
        ("modules", "modules") => (
            "Loaded kernel module details.",
            "Table of loaded kernel modules showing name, memory size, reference count, dependencies, and state (Live/Loading/Unloading).",
            "All loaded kernel modules.\n\nColumns: name, size (bytes), refcount, dependencies, state.\n\n💡 Diagnostic:\n  • refcount > 0 → Module is in use and cannot be unloaded\n  • refcount = 0 → Module can be safely unloaded with `rmmod`\n  • State 'Live' → Normal. 'Loading' or 'Unloading' should be transient; if stuck, the module has a bug.\n  • Dependencies show module relationships — unloading requires removing dependents first\n  • Large module size → Significant kernel memory usage. GPU drivers (nvidia, amdgpu) are often the largest.",
        ),

        // interrupts
        ("interrupts", "cpu_count") => (
            "Number of CPUs handling interrupts.",
            "Count of CPUs visible in the interrupt table. Each CPU independently handles hardware interrupts.",
            "CPU count from the interrupt table.\n\nThis should match the CPU count from cpuinfo. If it doesn't, some CPUs may be offline.\n\n💡 Diagnostic: Check if interrupts are balanced across CPUs. If one CPU handles most interrupts, performance may suffer. Use `irqbalance` daemon or set IRQ affinity manually.",
        ),
        ("interrupts", "irq_count") => (
            "Number of IRQ lines.",
            "Count of distinct interrupt request lines (hardware and software). Each IRQ represents a device or subsystem that can interrupt the CPU.",
            "Total IRQ line count.\n\n💡 Diagnostic:\n  • Common IRQs: timer (IRQ 0), keyboard (IRQ 1), NIC, disk controller, USB\n  • Very high counts of a specific IRQ → That device is very active or misbehaving\n  • Shared IRQs (multiple devices on same line) can cause performance issues on legacy systems",
        ),
        ("interrupts", "interrupts") => (
            "Hardware interrupt counters.",
            "Table of all interrupt lines showing IRQ number, total count across all CPUs, and description of the interrupt source.",
            "Per-IRQ interrupt counters.\n\nColumns: irq, total_count, description.\n\n💡 Diagnostic:\n  • NIC interrupts (usually labeled with driver name like 'ixgbe', 'mlx5') — high count = high network throughput\n  • Timer interrupts (LOC, local APIC timer) — should be roughly equal across CPUs\n  • NMI (Non-Maskable Interrupt) > 0 → Hardware watchdog or performance monitoring event\n  • Spurious IRQ (SPU) > 0 → Hardware issue, usually harmless but worth noting\n  • Unbalanced interrupt distribution across CPUs → Run `irqbalance` or manually set /proc/irq/N/smp_affinity",
        ),

        // softirqs
        ("softirqs", "softirq_count") => (
            "Number of software interrupt types.",
            "Count of distinct softirq types (typically 10). Softirqs handle deferred work like network packet processing and timer callbacks.",
            "Software interrupt type count.\n\nLinux has a fixed set of softirq types: HI, TIMER, NET_TX, NET_RX, BLOCK, IRQ_POLL, TASKLET, SCHED, HRTIMER, RCU.\n\n💡 Diagnostic: The count itself is fixed. What matters is the per-type activity in the softirqs table below.",
        ),
        ("softirqs", "softirqs") => (
            "Software interrupt counters.",
            "Table of softirq types and their total invocation count across all CPUs. NET_RX and NET_TX handle network traffic; TIMER handles kernel timers.",
            "Per-type software interrupt counters.\n\nColumns: name, total_count (summed across all CPUs).\n\n💡 Diagnostic:\n  • NET_RX very high → Heavy inbound network traffic processing\n  • NET_TX very high → Heavy outbound network traffic processing\n  • TIMER → Kernel timer callbacks; should be proportional to uptime and HZ\n  • SCHED → Scheduler events; high on busy multi-threaded systems\n  • RCU → Read-Copy-Update synchronization; high is normal on active systems\n  • TASKLET → Deferred work from interrupt handlers; if disproportionately high, a driver may be inefficient\n  • Unbalanced softirq counts across CPUs (check per-CPU view) → IRQ affinity issue; RX processing stuck on one core",
        ),

        // cgroups
        ("cgroups", "controller_count") => (
            "Number of cgroup controllers.",
            "Count of available cgroup (control group) controllers. Cgroups partition processes for resource management (CPU, memory, I/O limits).",
            "Available cgroup controller count.\n\nCgroup controllers manage resource allocation: cpu, memory, blkio (disk I/O), pids, cpuset, devices, etc.\n\n💡 Diagnostic:\n  • Common controllers: cpu, cpuacct, memory, blkio, pids, freezer, devices, net_cls\n  • Missing controllers → Kernel compiled without support, or not mounted\n  • cgroups v2 unified hierarchy vs v1 per-controller hierarchies — check hierarchy column",
        ),
        ("cgroups", "controllers") => (
            "Cgroup controller details.",
            "Table of cgroup controllers showing name, hierarchy ID, number of cgroups in that hierarchy, and whether the controller is enabled.",
            "Cgroup controller table.\n\nColumns: name, hierarchy, num_cgroups, enabled.\n\n💡 Diagnostic:\n  • hierarchy = 0 → Controller not attached to any hierarchy (not in use)\n  • enabled = 1 → Controller is available; 0 → Disabled (kernel parameter or not compiled in)\n  • High num_cgroups → Many containers/services using this controller (normal on container hosts)\n  • memory controller not enabled → No per-cgroup memory limits; a runaway container can consume all host memory",
        ),

        // crypto
        ("crypto", "algorithm_count") => (
            "Number of registered crypto algorithms.",
            "Count of cryptographic algorithms available to the kernel. Includes ciphers, hashes, RNGs, and compression algorithms.",
            "Registered crypto algorithm count.\n\nThe kernel crypto API provides algorithms for IPsec, dm-crypt (LUKS), TLS offload, and internal use.\n\n💡 Diagnostic:\n  • Typical count: 50-200 depending on kernel config and loaded modules\n  • Hardware-accelerated algorithms (driver name contains 'aesni', 'ghash-clmulni') → Faster encryption\n  • If dm-crypt/LUKS is slow → Check that aes-ni is available (grep for 'aes' type 'skcipher')",
        ),
        ("crypto", "algorithms") => (
            "Registered crypto algorithm details.",
            "Table of crypto algorithms showing name, type (skcipher, hash, aead, rng), driver implementation, and source module.",
            "Crypto algorithm table.\n\nColumns: name, type, driver, module.\n\n💡 Diagnostic:\n  • Type 'skcipher' → Symmetric cipher (AES, ChaCha20)\n  • Type 'ahash'/'shash' → Hash algorithm (SHA-256, MD5)\n  • Type 'aead' → Authenticated encryption (AES-GCM)\n  • Type 'rng' → Random number generator\n  • Module 'kernel' → Built-in. Named module → Loadable.\n  • Look for hardware-accelerated drivers: 'aesni-intel', 'ghash-clmulni-intel' for x86; 'aes-ce' for ARM",
        ),

        // devices
        ("devices", "device_count") => (
            "Number of registered device drivers.",
            "Count of all registered character and block device drivers. Each driver handles a class of hardware or virtual devices.",
            "Registered device driver count.\n\nIncludes both character devices (terminals, serial ports, /dev/null) and block devices (disk drivers).\n\n💡 Diagnostic:\n  • Character devices are typically more numerous than block devices\n  • Missing expected device → Driver not loaded or hardware not detected\n  • Major number conflicts → Very rare, but indicates a kernel configuration problem",
        ),
        ("devices", "devices") => (
            "Registered device driver details.",
            "Table of all registered devices showing type (Character/Block), major number, and device name.",
            "Registered device driver table.\n\nColumns: type, major number, name.\n\n💡 Diagnostic:\n  • Key character devices: 1=mem, 4=tty, 5=console, 10=misc, 136=pts\n  • Key block devices: 8=sd (SCSI/SATA), 253=device-mapper, 259=blkext (NVMe)\n  • 'Block' type 'sd' missing → SCSI/SATA driver not loaded\n  • 'Block' type 'nvme' missing → NVMe driver not loaded; check `lspci` for NVMe devices",
        ),

        // filesystems
        ("filesystems", "filesystem_count") => (
            "Number of supported filesystem types.",
            "Count of filesystem types the kernel can mount. Includes both disk-based (ext4, xfs) and virtual (proc, sysfs) filesystems.",
            "Supported filesystem type count.\n\n💡 Diagnostic:\n  • Common disk filesystems: ext4, xfs, btrfs, vfat\n  • Common virtual filesystems: proc, sysfs, tmpfs, devtmpfs, cgroup, cgroup2\n  • Missing expected filesystem (e.g., xfs) → Kernel module not loaded; try `modprobe xfs`\n  • 'nodev' flag → Virtual filesystem, not backed by a block device",
        ),
        ("filesystems", "filesystems") => (
            "Supported filesystem type details.",
            "Table of supported filesystem types showing name and whether it requires a block device (nodev=yes means no device needed).",
            "Supported filesystem table.\n\nColumns: name, nodev.\n\n💡 Diagnostic:\n  • nodev='yes' → Virtual/pseudo filesystem (proc, sysfs, tmpfs). No physical disk needed.\n  • nodev='no' → Disk-based filesystem (ext4, xfs, btrfs). Requires a block device.\n  • If you can't mount a filesystem → Check this list first to confirm kernel support\n  • FUSE (Filesystem in Userspace) appears as 'fuseblk' or 'fuse' → Enables user-space filesystem drivers (sshfs, s3fs, rclone)",
        ),

        // iomem
        ("iomem", "region_count") => (
            "Number of I/O memory regions.",
            "Count of physical memory-mapped I/O regions. Each region is reserved by a device driver or the kernel for hardware communication.",
            "I/O memory region count.\n\nThese are physical address ranges mapped to hardware devices (video RAM, NIC buffers, PCI device BARs) or reserved by the kernel.\n\n💡 Diagnostic:\n  • Useful for understanding physical memory layout and device presence\n  • 'System RAM' regions show usable physical memory\n  • 'reserved' regions → BIOS/UEFI reserved areas, ACPI tables\n  • PCI device names appear here — use to verify hardware detection",
        ),
        ("iomem", "regions") => (
            "I/O memory region map.",
            "Table of memory-mapped I/O regions showing address range and description. Shows the physical memory layout of the system.",
            "Physical I/O memory map.\n\nColumns: address_range, description.\n\n💡 Diagnostic:\n  • 'System RAM' → Physical RAM available to the OS\n  • 'Kernel code/data/bss' → Memory used by the kernel itself\n  • PCI device BARs (Base Address Registers) → Hardware MMIO regions\n  • 'ACPI Tables/Non-volatile Storage' → Firmware data regions\n  • Gaps in address space → Reserved by hardware or firmware\n  • Overlapping regions at different indent levels → Hierarchical allocation (parent device → sub-regions)",
        ),

        // ioports
        ("ioports", "region_count") => (
            "Number of I/O port regions.",
            "Count of I/O port address regions. Legacy x86 mechanism for communicating with hardware devices.",
            "I/O port region count.\n\nI/O ports are a legacy x86 mechanism for CPU-to-device communication. Modern devices prefer memory-mapped I/O (see iomem), but many traditional devices still use ports.\n\n💡 Diagnostic:\n  • Mostly relevant on x86/x86_64 systems\n  • Key ranges: 0x0-0xFF (DMA, PIC, timer), 0x3F8 (COM1 serial), 0x1F0 (primary IDE)\n  • On modern systems, PCI devices claim I/O port ranges through BARs",
        ),
        ("ioports", "regions") => (
            "I/O port region map.",
            "Table of I/O port address regions showing port range and the device or subsystem that owns it.",
            "I/O port address map.\n\nColumns: port_range, description.\n\n💡 Diagnostic:\n  • 0x60, 0x64 → Keyboard controller (i8042)\n  • 0x3F8 → COM1 serial port\n  • 0xCF8-0xCFF → PCI configuration space\n  • PCI device port ranges → Claimed by their drivers\n  • Conflicts (same port claimed by multiple drivers) are rare but catastrophic — causes hardware lockups",
        ),

        // consoles
        ("consoles", "console_count") => (
            "Number of registered console devices.",
            "Count of console devices registered with the kernel. Consoles receive kernel log messages (printk output).",
            "Registered console device count.\n\n💡 Diagnostic:\n  • Typical: 1-3 consoles (tty0 for virtual terminal, ttyS0 for serial, sometimes netconsole)\n  • 0 consoles → Kernel messages go nowhere; debugging will be very difficult\n  • netconsole registered → Kernel messages are being sent over the network (useful for remote debugging)",
        ),
        ("consoles", "consoles") => (
            "Registered console device details.",
            "Table of console devices showing name and flags (read/write permissions, preferred console, etc.).",
            "Console device table.\n\nColumns: name, flags.\n\n💡 Diagnostic:\n  • 'E' flag → Enabled\n  • 'W' flag → Can write (output)\n  • 'R' flag → Can read (input)\n  • Preferred console (last one registered) gets kernel panic output\n  • For headless servers: ensure a serial console (ttyS0) is configured for remote crash debugging\n  • Boot parameter 'console=ttyS0,115200' enables serial console",
        ),

        // misc
        ("misc", "device_count") => (
            "Number of misc devices.",
            "Count of miscellaneous character devices. Misc devices share major number 10 and include various kernel features.",
            "Miscellaneous device count.\n\nMisc devices are a grab-bag of character devices that share major number 10. They include hardware watchdogs, random number generators, and various kernel interfaces.\n\n💡 Diagnostic:\n  • Common misc devices: cpu_dma_latency, hpet, hwrng, loop-control, vhost-net\n  • 'watchdog' → Hardware watchdog timer; reboots the system if it hangs\n  • 'fuse' → FUSE (user-space filesystem) support is available",
        ),
        ("misc", "devices") => (
            "Misc device details.",
            "Table of miscellaneous devices showing minor number and device name. All share major number 10.",
            "Miscellaneous device table.\n\nColumns: minor_number, name.\n\n💡 Diagnostic:\n  • Minor number uniquely identifies each misc device under major 10\n  • Look for 'watchdog' (minor 130) → System has hardware watchdog support\n  • 'device-mapper' (minor 236) → LVM/dm-crypt support available\n  • 'kvm' → KVM virtualization support enabled in kernel",
        ),

        // dma
        ("dma", "channel_count") => (
            "Number of DMA channels in use.",
            "Count of ISA DMA channels currently claimed by device drivers. On modern systems, this is often 0 or very small.",
            "ISA DMA channel count.\n\nISA DMA is a legacy mechanism from the IBM PC era. Modern PCI/PCIe devices use bus-mastering DMA instead, which doesn't appear here.\n\n💡 Diagnostic:\n  • 0 channels → Normal on modern systems with no legacy ISA hardware\n  • Channel 4 = cascade (used internally by the DMA controller)\n  • Non-zero on modern systems → Floppy controller emulation or legacy sound card",
        ),
        ("dma", "channels") => (
            "DMA channel details.",
            "Table of active ISA DMA channels showing channel number and the device that claimed it.",
            "ISA DMA channel allocation table.\n\nColumns: channel_number, device_name.\n\n💡 Diagnostic:\n  • Channel 2 = floppy disk controller (legacy)\n  • Channel 4 = cascade (connects the two DMA controllers together)\n  • Modern DMA (PCI bus-mastering) is not shown here — it's managed by individual device drivers\n  • If empty, this is expected on any system without ISA hardware",
        ),

        // timer_list
        ("timer_list", "version") => (
            "Timer list version identifier.",
            "Version of the /proc/timer_list format. Used to ensure compatibility when parsing timer information.",
            "Timer list format version.\n\n💡 This is a metadata field. The important fields are timer_count and clock_count.",
        ),
        ("timer_list", "now") => (
            "Current kernel time in nanoseconds.",
            "The kernel's current time (ktime) in nanoseconds, used as the reference point for all timer expirations.",
            "Current kernel time (ktime_get) in nanoseconds.\n\nThis is the monotonic clock — it never goes backward and isn't affected by NTP adjustments or wall-clock changes.\n\n💡 Diagnostic: Convert to human-readable: divide by 1,000,000,000 to get seconds since boot. Should roughly match uptime.",
        ),
        ("timer_list", "clock_count") => (
            "Number of clock event devices.",
            "Count of clock event devices (hardware timers). Each CPU typically has its own local APIC timer.",
            "Clock event device count.\n\nClock event devices are hardware timers that generate periodic or one-shot interrupts for scheduling, timekeeping, and timer callbacks.\n\n💡 Diagnostic:\n  • Should roughly equal CPU count (each CPU has a local APIC timer)\n  • Additional clock devices: HPET, PIT, TSC deadline timer\n  • Missing clock devices → Kernel may have difficulty scheduling timers accurately",
        ),
        ("timer_list", "timer_count") => (
            "Number of pending kernel timers.",
            "Count of timers currently queued in the kernel. Includes timeouts, delayed work, and periodic callbacks.",
            "Pending kernel timer count.\n\nKernel timers are callbacks scheduled to fire at a future time. Used for TCP retransmission timeouts, device polling, watchdogs, and deferred work.\n\n💡 Diagnostic:\n  • Typical range: 50-500 on a busy server\n  • Very high count (10000+) → Some subsystem is creating many timers; could indicate a network issue (many TCP retransmit timers) or a misbehaving driver\n  • Timers that never fire → Possible timer leak, wasting kernel memory",
        ),

        // schedstat
        ("schedstat", "version") => (
            "Scheduler statistics version.",
            "Version number of the schedstat format. Determines which fields are available in per-CPU stats.",
            "Schedstat format version.\n\n💡 Version 15 is the current format. The version determines how to interpret the per-CPU stat columns.",
        ),
        ("schedstat", "cpu_count") => (
            "CPUs with scheduler statistics.",
            "Number of CPUs reporting scheduler statistics. Should match the total CPU count.",
            "Number of CPUs in the scheduler stats.\n\n💡 Diagnostic: Should match the CPU count from cpuinfo and interrupts. If lower, some CPUs may be offline (`echo 1 > /sys/devices/system/cpu/cpuN/online` to re-enable).",
        ),
        ("schedstat", "cpu_stats") => (
            "Per-CPU scheduler statistics.",
            "Table of per-CPU scheduler metrics: yield count, schedule count, idle count, try-to-wake-up count, and more.",
            "Per-CPU scheduler statistics table.\n\nColumns: cpu, yld_count (yield), sched_count (context switches), sched_goidle (went idle), ttwu_count (try-to-wake-up), ...\n\n💡 Diagnostic:\n  • High sched_count → Many context switches on that CPU. Normal for I/O-heavy workloads.\n  • sched_goidle / sched_count → Idle ratio. High = CPU is underutilized. Low = CPU is always busy.\n  • ttwu_count → How often tasks are woken up. High = many sleeping tasks being activated (I/O completions, lock releases).\n  • Large disparity between CPUs → Workload is not well-balanced; check IRQ affinity and CPU pinning (taskset/cgroups).",
        ),

        // df (disk filesystem usage)
        ("df", "filesystems") => (
            "Filesystem usage table",
            "Table of mounted filesystems with columns: Device, MountPoint, Total, Used, Available, Use%. Pseudo-filesystems (tmpfs, proc, etc.) are excluded.",
            "Filesystem usage table showing real (non-pseudo) filesystems.\n\nColumns: Device, MountPoint, Total, Used, Available, Use%.\n\n💡 Diagnostic:\n  • Use% > 90% → Critical. Logs may fail to write, databases crash.\n  • Use% > 80% → Plan capacity. Set up log rotation, clean temp files.\n  • Available shows space usable by non-root users (accounts for reserved blocks).",
        ),
        ("df", "root_use_pct") => (
            "Root filesystem usage %",
            "Usage percentage of the root (/) filesystem. This is the most critical filesystem — if full, the system may become unresponsive.",
            "Root filesystem usage percentage.\n\n💡 Diagnostic:\n  • > 90% → CRITICAL: Immediate action needed. du -sh /* to find large dirs.\n  • > 80% → WARNING: Plan cleanup. journalctl --vacuum-size=500M, docker system prune.\n  • Steadily increasing → Possible log leak. Check /var/log sizes.",
        ),

        // thermal
        ("thermal", "max_temp") => (
            "Highest CPU/GPU temperature",
            "Maximum temperature across all thermal zones. Above 75°C indicates thermal stress; above 90°C triggers throttling.",
            "Maximum temperature across all thermal zones.\n\n💡 Diagnostic:\n  • > 90°C → CRITICAL: Thermal throttling active. CPU frequency reduced. Check fans.\n  • > 75°C → WARNING: Running hot. Sustained load will push it higher.\n  • < 50°C → Normal idle temperature for most systems.",
        ),

        // file-nr
        ("file-nr", "fd_allocated") => (
            "Allocated file descriptors",
            "Number of file handles currently allocated by the kernel. Includes both in-use and cached (unused) handles.",
            "Number of file handles currently allocated by the kernel.\n\nThis counts all file descriptors allocated, including unused ones kept in the free list for reuse.\n\n💡 Diagnostic: If fd_allocated is close to fd_max, the system may refuse new file/socket opens. Check for FD leaks with lsof.",
        ),
        ("file-nr", "fd_usage_pct") => (
            "File descriptor usage %",
            "Percentage of file descriptors in active use relative to the system maximum. (allocated - unused) / max * 100.",
            "File descriptor usage percentage: (allocated - unused) / max * 100.\n\n💡 Diagnostic:\n  • > 80% → WARNING: FD exhaustion risk. Processes may fail to open files or sockets.\n  • Find leakers: lsof -p <PID> | wc -l for suspect processes.\n  • Raise limit: sysctl -w fs.file-max=<higher_value>.",
        ),
        ("file-nr", "fd_unused") => (
            "Unused (cached) file descriptors",
            "Number of allocated but unused file handles kept in the kernel free list for quick reuse.",
            "Allocated but unused file handles in the kernel free list.\n\nThe kernel pre-allocates file handles and keeps unused ones in a free list to avoid allocation overhead.\n\n💡 Diagnostic: A large fd_unused relative to fd_allocated means the kernel has over-provisioned. This is harmless — the handles are recycled.",
        ),
        ("file-nr", "fd_max") => (
            "Maximum file descriptors (system limit)",
            "Kernel-enforced maximum number of file handles. Reaching this limit causes EMFILE errors for all processes.",
            "System-wide file descriptor maximum (fs.file-max).\n\nThis is the hard ceiling. When fd_allocated approaches fd_max, new open()/socket() calls fail with EMFILE.\n\n💡 Diagnostic:\n  • Default is usually 100000-1000000 depending on RAM\n  • Raise: sysctl -w fs.file-max=<value> or persist in /etc/sysctl.conf\n  • Per-process limit (ulimit -n) is separate and usually lower",
        ),

        // df — additional fields
        ("df", "total_disk") => (
            "Total disk space (root filesystem)",
            "Total capacity of the root (/) filesystem in bytes. Includes space reserved for root user.",
            "Total disk space on the root filesystem.\n\n💡 Diagnostic: If total_disk seems smaller than the physical partition, check for reserved blocks (typically 5% on ext4). Tune with `tune2fs -m <pct> /dev/sdX`.",
        ),
        ("df", "used_disk") => (
            "Used disk space (root filesystem)",
            "Bytes consumed on the root filesystem. Includes files, directories, and filesystem metadata.",
            "Used disk space on the root filesystem.\n\n💡 Diagnostic: If used_disk is near total_disk, the system may become unresponsive. Logs fail to write, databases crash, and package managers break. Immediate cleanup needed.",
        ),
        ("df", "available_disk") => (
            "Available disk space (root filesystem)",
            "Bytes available for non-root users on the root filesystem. Less than total - used because of reserved blocks.",
            "Available disk space on the root filesystem for non-root users.\n\nThis is less than (total - used) because ext4 reserves ~5% for root by default.\n\n💡 Diagnostic:\n  • < 1 GB → CRITICAL on a production system\n  • Rapidly decreasing → Runaway log file or temp file accumulation\n  • To find large files: `du -sh /* | sort -h | tail -10`",
        ),

        // ── net/snmp — additional important counters ──────────────────
        ("net/snmp", "Ip_Forwarding") => (
            "IP forwarding status",
            "Whether this host is forwarding packets (1=yes, acting as router; 2=no, host-only mode).",
            "IP forwarding status.\n\n1 = forwarding enabled (router mode), 2 = forwarding disabled (host mode).\n\n💡 Diagnostic:\n  • Should be 2 on most servers/workstations\n  • Should be 1 on routers, VPN gateways, container hosts with routing\n  • Toggle: sysctl net.ipv4.ip_forward=1",
        ),
        ("net/snmp", "Ip_InHdrErrors") => (
            "IP packets with header errors",
            "Packets discarded due to malformed IP headers (bad checksum, invalid version, truncated).",
            "IP header error drops.\n\n💡 Diagnostic:\n  • Should be 0 or near 0\n  • Non-zero → Corrupted packets on the network, faulty NIC, or attack traffic\n  • Compare with Tcp_InErrs and Udp_InErrors for broader corruption picture",
        ),
        ("net/snmp", "Ip_InAddrErrors") => (
            "IP packets with address errors",
            "Packets discarded because the destination IP was invalid for this host (wrong subnet, broadcast misroute).",
            "IP address error drops — packets sent to wrong destination.\n\n💡 Diagnostic: Non-zero suggests routing issues or misconfigured clients sending traffic to the wrong host.",
        ),
        ("net/snmp", "Ip_ForwDatagrams") => (
            "IP packets forwarded",
            "Packets this host forwarded to another destination. Non-zero means the host is acting as a router.",
            "IP datagrams forwarded to another hop.\n\n💡 Diagnostic:\n  • Should be 0 if ip_forward is disabled\n  • Non-zero with forwarding off → Something is misconfigured\n  • On a router/gateway, this is the core traffic counter",
        ),
        ("net/snmp", "Ip_InDelivers") => (
            "IP packets delivered to protocols",
            "Packets successfully demuxed and delivered to upper-layer protocols (TCP, UDP, ICMP).",
            "IP packets delivered to upper protocols.\n\n💡 Diagnostic: Ip_InReceives - Ip_InDelivers = packets dropped/forwarded at IP layer. Large gap means lots of dropped or forwarded traffic.",
        ),
        ("net/snmp", "Ip_OutDiscards") => (
            "Outbound IP packets discarded",
            "Packets ready to send but discarded (typically due to insufficient buffer space or routing failure).",
            "Outbound IP discards.\n\n💡 Diagnostic: Non-zero → TX path congestion. Check NIC TX ring buffer size and network interface errors.",
        ),
        ("net/snmp", "Ip_OutNoRoutes") => (
            "IP packets with no route",
            "Packets discarded because no route to the destination existed in the routing table.",
            "IP packets dropped due to missing route.\n\n💡 Diagnostic:\n  • Non-zero → Application trying to reach unreachable networks\n  • Check routing table: `ip route show`\n  • Common cause: missing default gateway or VPN tunnel down",
        ),
        ("net/snmp", "Ip_ReasmFails") => (
            "IP reassembly failures",
            "IP fragment reassembly attempts that failed (timeout, missing fragments, or resource exhaustion).",
            "IP fragment reassembly failures.\n\n💡 Diagnostic:\n  • Non-zero → Fragments are being lost in transit (firewall blocking, MTU issues)\n  • Fix: Ensure PMTUD works (don't block ICMP type 3 'fragmentation needed')\n  • Or increase MTU on the problematic path",
        ),
        ("net/snmp", "Tcp_CurrEstab") => (
            "Currently established TCP connections",
            "Snapshot of TCP connections in ESTABLISHED or CLOSE_WAIT state right now. Key capacity indicator.",
            "Current ESTABLISHED + CLOSE_WAIT TCP connections.\n\nThis is a point-in-time gauge (not cumulative like most counters).\n\n💡 Diagnostic:\n  • Baseline this value to understand normal connection load\n  • Steady growth without plateau → Connection leak\n  • Sudden drop → Mass disconnection event or service restart",
        ),
        ("net/snmp", "Tcp_InSegs") => (
            "TCP segments received",
            "Total TCP segments received. Combined with OutSegs, gives the overall TCP throughput picture.",
            "Total TCP segments received.\n\n💡 Diagnostic: Rate of change shows inbound TCP throughput. Compare InSegs rate vs OutSegs rate to see traffic direction bias.",
        ),
        ("net/snmp", "Tcp_OutSegs") => (
            "TCP segments sent",
            "Total TCP segments sent (includes retransmissions). The primary outbound TCP volume counter.",
            "Total TCP segments sent.\n\n💡 Diagnostic: RetransSegs / OutSegs gives retransmission percentage. < 0.1% is excellent, > 1% is problematic.",
        ),
        ("net/snmp", "Tcp_AttemptFails") => (
            "TCP connection attempts failed",
            "Connections that failed during the handshake (SYN sent but no SYN-ACK, or SYN-ACK sent but no final ACK).",
            "Failed TCP connection attempts.\n\n💡 Diagnostic:\n  • High rate → Target hosts unreachable, firewalled, or overloaded\n  • AttemptFails / ActiveOpens = outbound failure rate\n  • Correlate with TcpExt_TCPTimeouts for timeout-related failures",
        ),
        ("net/snmp", "Tcp_EstabResets") => (
            "Established connections reset",
            "ESTABLISHED connections that were reset (RST). Indicates abnormal connection termination.",
            "TCP connections reset from ESTABLISHED state.\n\n💡 Diagnostic:\n  • High rate → Remote hosts crashing, firewall killing connections, or application bugs\n  • Compare with OutRsts to see if this host or remote hosts are sending resets",
        ),
        ("net/snmp", "Tcp_OutRsts") => (
            "TCP RST segments sent",
            "RST (reset) segments sent by this host. Indicates connection refusals or abnormal closures.",
            "TCP RST segments sent.\n\n💡 Diagnostic:\n  • High rate → Many connections to closed ports, or application rejecting connections\n  • Common cause: port scan hitting the host, or service restarting",
        ),
        ("net/snmp", "Tcp_InCsumErrors") => (
            "TCP checksum errors",
            "TCP segments with invalid checksums. Indicates data corruption on the network path.",
            "TCP segments with checksum errors.\n\n💡 Diagnostic:\n  • Must be 0 on a healthy network\n  • Non-zero → NIC offload bug, bad cable, memory corruption\n  • If also seeing Udp_InCsumErrors → Link-level corruption",
        ),
        ("net/snmp", "Udp_InDatagrams") => (
            "UDP datagrams received",
            "Total UDP datagrams successfully received and delivered to applications.",
            "UDP datagrams received and delivered.\n\n💡 Diagnostic: Rate of change shows UDP input throughput. Common UDP consumers: DNS (53), NTP (123), SNMP (161).",
        ),
        ("net/snmp", "Udp_OutDatagrams") => (
            "UDP datagrams sent",
            "Total UDP datagrams sent by this host.",
            "UDP datagrams sent.\n\n💡 Diagnostic: High OutDatagrams with low InDatagrams → Host is a UDP sender (syslog forwarder, DNS server responding). Reverse → UDP consumer.",
        ),
        ("net/snmp", "Udp_NoPorts") => (
            "UDP packets to closed ports",
            "UDP datagrams received for ports with no listening process. Each triggers an ICMP port-unreachable response.",
            "UDP datagrams to ports with no listener.\n\n💡 Diagnostic:\n  • Non-zero is normal (occasional probes, stale DNS replies)\n  • High rate → Port scanning or misconfigured client sending to wrong port\n  • Each generates an ICMP port-unreachable, consuming outbound bandwidth",
        ),
        ("net/snmp", "Udp_RcvbufErrors") => (
            "UDP receive buffer overflows",
            "UDP datagrams dropped because the socket receive buffer was full. Application is too slow to read.",
            "UDP receive buffer overflow drops.\n\n💡 Diagnostic:\n  • Non-zero → Application cannot keep up with incoming UDP rate\n  • Fix: Increase buffer size with SO_RCVBUF or sysctl net.core.rmem_max\n  • Or optimize the receiving application to read faster",
        ),
        ("net/snmp", "Udp_SndbufErrors") => (
            "UDP send buffer overflows",
            "UDP datagrams dropped because the send buffer was full. Sending too fast for the network.",
            "UDP send buffer overflow drops.\n\n💡 Diagnostic:\n  • Application sending faster than the NIC can transmit\n  • Fix: Increase net.core.wmem_max or throttle the sending rate",
        ),
        ("net/snmp", "Udp_InCsumErrors") => (
            "UDP checksum errors",
            "UDP datagrams with invalid checksums. Data corruption in transit.",
            "UDP checksum errors.\n\n💡 Diagnostic: Same implications as TCP checksum errors — NIC/cable/driver issue. Cross-check with Tcp_InCsumErrors.",
        ),
        ("net/snmp", "Icmp_InMsgs") => (
            "ICMP messages received",
            "Total ICMP messages received (ping replies, unreachables, redirects, etc.).",
            "Total inbound ICMP messages.\n\n💡 Diagnostic: High rate may indicate ping flood, PMTUD activity, or network error signaling.",
        ),
        ("net/snmp", "Icmp_InErrors") => (
            "ICMP input errors",
            "ICMP messages received with errors (bad checksum, too short, etc.).",
            "ICMP messages with errors.\n\n💡 Diagnostic: Should be near 0. Non-zero indicates corrupted ICMP packets on the network.",
        ),
        ("net/snmp", "Icmp_InDestUnreachs") => (
            "ICMP Destination Unreachable received",
            "ICMP Destination Unreachable messages received, often indicating remote port/host is unreachable.",
            "ICMP Destination Unreachable messages received.\n\n💡 Diagnostic:\n  • High rate → Many outbound connections failing (firewall drops, host down)\n  • Type 3 code 4 (fragmentation needed) → PMTUD issue, check MTU\n  • Type 3 code 3 (port unreachable) → Remote application not listening",
        ),
        ("net/snmp", "Icmp_OutMsgs") => (
            "ICMP messages sent",
            "Total ICMP messages sent by this host (ping requests, unreachable responses, etc.).",
            "Total outbound ICMP messages.\n\n💡 Diagnostic: High OutMsgs with many Destination Unreachable → This host is rejecting traffic (closed ports generating ICMP unreachable).",
        ),

        // ── net/netstat — additional important counters ───────────────
        ("net/netstat", "TcpExt_SyncookiesSent") => (
            "SYN cookies sent (SYN flood protection)",
            "Count of SYN cookies sent. Non-zero means the SYN queue overflowed and the kernel activated SYN flood protection.",
            "TCP SYN cookies sent.\n\nSYN cookies are a defense against SYN flood attacks. When the SYN queue is full, the kernel encodes connection state in the SYN-ACK sequence number.\n\n💡 Diagnostic:\n  • Non-zero → SYN flood detected or listen backlog too small\n  • If legitimate traffic: increase net.ipv4.tcp_max_syn_backlog\n  • If attack: SYN cookies are working correctly as a defense",
        ),
        ("net/netstat", "TcpExt_SyncookiesRecv") => (
            "SYN cookies received (validated)",
            "Count of SYN cookies successfully validated. These are legitimate connections that went through SYN cookie mode.",
            "Valid SYN cookies received back from clients.\n\n💡 Diagnostic: If SyncookiesSent >> SyncookiesRecv, most SYN-flood traffic was from spoofed IPs (they never complete the handshake).",
        ),
        ("net/netstat", "TcpExt_SyncookiesFailed") => (
            "Invalid SYN cookies received",
            "SYN cookies that failed validation. Could be legitimate clients with mangled packets or attack traffic.",
            "Invalid SYN cookie validation failures.\n\n💡 Diagnostic: High rate alongside SyncookiesSent → Active SYN flood attack with some attempted spoofed completions.",
        ),
        ("net/netstat", "TcpExt_TW") => (
            "TIME_WAIT sockets recycled by timeout",
            "TIME_WAIT sockets that expired naturally after the 2*MSL timeout.",
            "TIME_WAIT sockets expired normally.\n\n💡 Diagnostic: This is the normal cleanup path. Compare with TCPTimeWaitOverflow to see if recycling is keeping up.",
        ),
        ("net/netstat", "TcpExt_PAWSEstab") => (
            "Packets rejected by PAWS on established connections",
            "Segments rejected on established connections by Protection Against Wrapped Sequences. Indicates old duplicate segments or clock issues.",
            "PAWS (Protection Against Wrapped Sequences) rejections on established connections.\n\n💡 Diagnostic:\n  • Occasional is normal (old duplicate segments after route changes)\n  • Sustained high rate → Timestamp clock issue on one side, or middlebox stripping TCP timestamps",
        ),
        ("net/netstat", "TcpExt_DelayedACKs") => (
            "Delayed ACKs sent",
            "ACKs that were delayed to piggyback on data segments. Normal TCP optimization to reduce packet count.",
            "Delayed ACKs sent (piggybacking optimization).\n\n💡 Diagnostic: High count is normal — it means TCP is efficiently batching ACKs. Compare with TCPPureAcks for non-delayed ACK volume.",
        ),
        ("net/netstat", "TcpExt_TCPHPHits") => (
            "TCP header prediction hits (fast path)",
            "Packets processed via the fast path (header prediction). Higher is better — means most traffic follows the common case.",
            "TCP header prediction fast-path hits.\n\n💡 Diagnostic: High HPHits relative to InSegs means the network stack is efficient. Low ratio → Unusual packet patterns forcing slow-path processing.",
        ),
        ("net/netstat", "TcpExt_TCPPureAcks") => (
            "Pure ACKs received (no data)",
            "ACK segments containing no data payload. Common in interactive protocols and after data bursts.",
            "Pure ACKs received (acknowledgment only, no data).\n\n💡 Diagnostic: High pure ACK ratio → Unidirectional data flow (one side sending, other just ACKing). Normal for downloads/uploads.",
        ),
        ("net/netstat", "TcpExt_TCPSackRecovery") => (
            "SACK-based loss recoveries",
            "Times TCP used SACK information to recover from packet loss without a full retransmission timeout.",
            "TCP SACK-based loss recovery events.\n\n💡 Diagnostic:\n  • SACK recovery is much faster than RTO-based recovery\n  • High count → Packet loss is occurring, but SACK is handling it well\n  • Compare with TCPTimeouts — timeouts mean SACK couldn't help",
        ),
        ("net/netstat", "TcpExt_TCPFastRetrans") => (
            "TCP fast retransmits",
            "Segments retransmitted via fast retransmit (3 duplicate ACKs) rather than waiting for timeout.",
            "TCP fast retransmits (triggered by 3 dup-ACKs).\n\n💡 Diagnostic:\n  • Fast retransmit is preferable to timeout — recovery is much quicker\n  • High rate → Frequent packet loss on the network\n  • FastRetrans + SackRecovery working well → Network has loss but TCP is coping",
        ),
        ("net/netstat", "TcpExt_TCPLossProbes") => (
            "TCP Tail Loss Probes sent",
            "TLP (Tail Loss Probe) segments sent to detect loss at the end of a transaction without waiting for full RTO.",
            "TCP Tail Loss Probes (TLP) sent.\n\nTLP is a mechanism to detect tail loss faster than RTO. When the last segment of a burst is lost, TLP retransmits it proactively.\n\n💡 Diagnostic: High TLP count → Many transactions have their final packets lost. Common on lossy networks.",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnData") => (
            "Connections aborted (data in close)",
            "TCP connections aborted because data was received after the connection was closed.",
            "TCP connections aborted due to unexpected data after close.\n\n💡 Diagnostic: Usually indicates a peer sending data after the connection was shut down. Application-level protocol mismatch.",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnClose") => (
            "Connections aborted (close with pending data)",
            "Connections terminated with RST because the application closed the socket with unread data in the buffer.",
            "TCP connections aborted by close() with data pending.\n\n💡 Diagnostic:\n  • Application closed socket without reading all data\n  • Common with HTTP servers aborting slow clients\n  • Results in RST sent to peer",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnTimeout") => (
            "Connections aborted on timeout",
            "TCP connections aborted because retransmission attempts exhausted the timeout limit.",
            "TCP connections aborted due to timeout.\n\n💡 Diagnostic: The final outcome of persistent packet loss — all retransmission attempts failed. Correlates with TCPTimeouts.",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnMemory") => (
            "Connections aborted (memory pressure)",
            "Connections terminated because the system ran out of memory for TCP buffers.",
            "TCP connections killed due to memory pressure.\n\n💡 Diagnostic:\n  • CRITICAL if non-zero: system is dropping connections to survive\n  • Check TCP_mem in sockstat and /proc/sys/net/ipv4/tcp_mem limits\n  • May need more RAM or fewer concurrent connections",
        ),
        ("net/netstat", "TcpExt_TCPMemoryPressures") => (
            "TCP memory pressure events",
            "Times the TCP stack entered memory pressure mode, reducing buffer sizes and potentially dropping connections.",
            "TCP memory pressure mode activations.\n\n💡 Diagnostic:\n  • Non-zero → TCP buffer memory hit the 'pressure' threshold\n  • Kernel reduces per-socket buffer sizes to cope\n  • Fix: Increase tcp_mem limits or add physical RAM",
        ),
        ("net/netstat", "TcpExt_TCPSynRetrans") => (
            "SYN/SYN-ACK retransmits",
            "SYN or SYN-ACK segments retransmitted. Indicates connection establishment failures.",
            "SYN and SYN-ACK retransmissions.\n\n💡 Diagnostic:\n  • High rate → Clients can't reach server (firewall, server overloaded, network loss)\n  • Correlates with ListenDrops if server-side backlog is full\n  • Compare with ActiveOpens/PassiveOpens to get retry ratio",
        ),
        ("net/netstat", "TcpExt_TCPOrigDataSent") => (
            "Original data segments sent",
            "Data segments sent for the first time (excluding retransmissions). Subtract from OutSegs to get retransmit count.",
            "Original (non-retransmit) data segments sent.\n\n💡 Diagnostic: (OutSegs - TCPOrigDataSent - pure ACKs) approximates retransmission volume. Useful for computing retransmission ratio.",
        ),
        ("net/netstat", "TcpExt_TCPKeepAlive") => (
            "TCP keepalive probes sent",
            "Keepalive probes sent on idle connections to verify the peer is still alive.",
            "TCP keepalive probes sent.\n\n💡 Diagnostic:\n  • Normal for long-lived idle connections (database connections, SSH sessions)\n  • Very high rate → Many idle connections with keepalive enabled\n  • Tune: net.ipv4.tcp_keepalive_time (default 7200 sec)",
        ),
        ("net/netstat", "TcpExt_TCPAutoCorking") => (
            "TCP auto-corking events",
            "Times the kernel delayed small writes to combine them into larger segments (Nagle-like optimization).",
            "TCP auto-corking activations.\n\n💡 Diagnostic: Auto-corking reduces small packet overhead by buffering writes when there's already unacknowledged data. High count is normal for write-heavy workloads.",
        ),
        ("net/netstat", "TcpExt_TCPRcvCoalesce") => (
            "TCP receive queue coalescing",
            "Segments coalesced (merged) in the receive queue for efficiency.",
            "TCP segments coalesced in receive queue.\n\n💡 Diagnostic: Coalescing reduces overhead by merging adjacent segments. High values indicate efficient GRO/LRO processing on the NIC.",
        ),
        ("net/netstat", "TcpExt_TCPOFOQueue") => (
            "Out-of-order packets queued",
            "Packets received out of order and queued for reordering. Indicates network path reordering.",
            "Out-of-order TCP packets queued.\n\n💡 Diagnostic:\n  • Some OOO is normal (multipath, load-balanced traffic)\n  • High rate → Significant reordering on the network path\n  • Can trigger spurious fast retransmits if not handled by SACK",
        ),
        ("net/netstat", "TcpExt_TCPChallengeACK") => (
            "Challenge ACKs sent (RFC 5961)",
            "ACKs sent in response to suspicious segments to validate the connection. Security mechanism against blind injection.",
            "TCP Challenge ACKs sent (RFC 5961).\n\n💡 Diagnostic:\n  • Low rate is normal (occasional stale segments)\n  • High rate → Possible blind TCP injection attack attempt\n  • Rate limited by net.ipv4.tcp_challenge_ack_limit",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenActive") => (
            "TCP Fast Open connections initiated",
            "Outbound connections using TCP Fast Open (TFO) to send data in the SYN packet.",
            "TCP Fast Open active connections initiated.\n\n💡 Diagnostic: TFO reduces latency by one RTT on repeat connections. Non-zero means applications are using TFO. Zero means either TFO is disabled or no application requests it.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassive") => (
            "TCP Fast Open connections accepted",
            "Inbound connections accepted with TCP Fast Open data in the SYN packet.",
            "TCP Fast Open passive connections accepted.\n\n💡 Diagnostic: Non-zero → Server is accepting TFO connections. Compare with FastOpenPassiveFail for failure rate.",
        ),
        ("net/netstat", "TcpExt_TCPDelivered") => (
            "TCP segments delivered to application",
            "Total segments successfully delivered to application layer, including original and retransmitted data.",
            "TCP segments delivered to the application.\n\n💡 Diagnostic: This is the 'goodput' counter. Compare with OutSegs for the total packet count to see overhead ratio.",
        ),
        ("net/netstat", "TcpExt_TCPWinProbe") => (
            "TCP window probes sent",
            "Window probes sent when the receive window is zero. The sender is waiting for the receiver to free buffer space.",
            "TCP zero-window probes sent.\n\n💡 Diagnostic:\n  • Non-zero → Receiver is too slow to consume data (zero-window condition)\n  • Persistent → Application backpressure, receiver can't keep up\n  • Check TCPToZeroWindowAdv for how often the receiver advertises zero window",
        ),
        ("net/netstat", "TcpExt_TCPTimeWaitOverflow") => (
            "TIME_WAIT bucket overflows",
            "Times a new TIME_WAIT socket could not be created because the bucket limit was reached.",
            "TIME_WAIT socket overflow events.\n\n💡 Diagnostic:\n  • Non-zero → Too many short-lived connections exhausting TIME_WAIT slots\n  • Enable net.ipv4.tcp_tw_reuse to allow reuse\n  • Consider connection pooling in the application",
        ),
        ("net/netstat", "TcpExt_TCPBacklogDrop") => (
            "Segments dropped from socket backlog",
            "Segments dropped because the per-socket backlog queue was full.",
            "TCP socket backlog drops.\n\n💡 Diagnostic:\n  • Non-zero → Receiving application not processing fast enough\n  • Socket backlog is the queue between NIC and application read()\n  • Fix: Increase SO_RCVBUF or optimize application read loop",
        ),
        ("net/netstat", "TcpExt_TCPSpuriousRTOs") => (
            "Spurious retransmission timeouts",
            "RTOs detected as spurious (the original packet arrived but the RTO fired too early).",
            "Spurious RTO detections.\n\n💡 Diagnostic:\n  • High rate → RTO is too aggressive for the network latency\n  • Causes unnecessary retransmissions and congestion window reduction\n  • Consider tuning or using a different congestion control algorithm",
        ),
        ("net/netstat", "IpExt_InOctets") => (
            "Total bytes received (IP layer)",
            "Total bytes received at the IP layer, including headers. The definitive inbound bandwidth counter.",
            "Total IP-layer inbound bytes.\n\n💡 Diagnostic: Rate of change gives inbound bandwidth. Compare with net/dev total_rx — difference is non-IP traffic (ARP, etc.).",
        ),
        ("net/netstat", "IpExt_OutOctets") => (
            "Total bytes sent (IP layer)",
            "Total bytes sent at the IP layer, including headers. The definitive outbound bandwidth counter.",
            "Total IP-layer outbound bytes.\n\n💡 Diagnostic: Rate of change gives outbound bandwidth. The most accurate bandwidth counter for IP traffic.",
        ),
        ("net/netstat", "IpExt_InMcastPkts") => (
            "Multicast packets received",
            "IP multicast packets received. Common for service discovery (mDNS, SSDP) and cluster communication.",
            "Inbound multicast packets.\n\n💡 Diagnostic: High rate on a server → Cluster/multicast-based service active. On a desktop → mDNS/Bonjour or media streaming.",
        ),
        ("net/netstat", "IpExt_InCsumErrors") => (
            "IP checksum errors",
            "Inbound packets with IP-level checksum errors. Indicates data corruption at the network layer.",
            "IP-layer checksum errors.\n\n💡 Diagnostic:\n  • Must be 0 on a healthy network\n  • Non-zero → Link-level corruption, bad NIC offload, or memory errors\n  • Correlate with TCP/UDP checksum errors for full picture",
        ),

        // ── net/sockstat — remaining fields ──────────────────────────
        ("net/sockstat", "UDPLITE_inuse") => (
            "UDP-Lite sockets in use",
            "Number of UDP-Lite sockets currently in use. UDP-Lite allows partial checksums for error-tolerant media streams.",
            "Active UDP-Lite sockets.\n\n💡 Diagnostic: Usually 0 on most systems. UDP-Lite is used for media streaming where partial data is better than no data.",
        ),
        ("net/sockstat", "RAW_inuse") => (
            "Raw sockets in use",
            "Number of raw IP sockets. Used by tools like ping, traceroute, and custom network protocols.",
            "Active raw IP sockets.\n\n💡 Diagnostic:\n  • Typical: 0-2 on normal systems\n  • ping and traceroute use raw sockets temporarily\n  • Persistent raw sockets → Custom network protocol or monitoring tool\n  • Security concern: raw sockets require root or CAP_NET_RAW",
        ),
        ("net/sockstat", "FRAG_memory") => (
            "IP fragment reassembly memory (bytes)",
            "Bytes of kernel memory used for holding IP fragments awaiting reassembly.",
            "Memory consumed by IP fragment reassembly queue.\n\n💡 Diagnostic:\n  • Usually 0 on modern networks (PMTUD avoids fragmentation)\n  • Large values → Fragmentation attack or broken PMTUD\n  • Limit: net.ipv4.ipfrag_high_thresh",
        ),

        // ── ip/route ─────────────────────────────────────────────────
        ("ip/route", "default_gateway") => (
            "Default gateway IP address",
            "The IP address of the default gateway. All traffic to unknown networks is forwarded here.",
            "Default gateway IP address.\n\n💡 Diagnostic:\n  • '(none)' → No default route configured. Host cannot reach the internet.\n  • Should be reachable: `ping <gateway>`. If not, check cable/link.\n  • Multiple default routes → Failover or policy routing in use",
        ),
        ("ip/route", "route_count") => (
            "Number of IP routes",
            "Total routing entries from 'ip route show'. Includes default, connected, and static routes.",
            "Total IP routing entries.\n\n💡 Diagnostic:\n  • Minimum 1 (the default route) for internet-connected hosts\n  • Very high count → Dynamic routing protocol (BGP/OSPF) active\n  • Compare with /proc/net/route for kernel-level view",
        ),
        ("ip/route", "routes") => (
            "IP routing table",
            "Complete routing table: Destination, Gateway, Device, Protocol, Scope, Metric. Lower metric = preferred route.",
            "IP routing table from 'ip route show'.\n\nColumns: Destination, Gateway, Device, Protocol, Scope, Metric.\n\n💡 Diagnostic:\n  • 'default via X.X.X.X' → Default route. Must exist for internet access.\n  • Protocol 'kernel' → Auto-created for directly connected networks\n  • Protocol 'dhcp' → Learned from DHCP server\n  • Scope 'link' → Directly attached network, no gateway needed\n  • Lower metric = higher priority when multiple routes exist",
        ),

        // ── ip/neighbor ──────────────────────────────────────────────
        ("ip/neighbor", "neighbor_count") => (
            "Total ARP/NDP neighbor entries",
            "Count of all neighbor cache entries (IPv4 ARP and IPv6 NDP). Shows how many hosts this system has communicated with.",
            "ARP/NDP neighbor cache entry count.\n\n💡 Diagnostic:\n  • High count → Many hosts on local network or multicast group\n  • Growing steadily → Network scan or broadcast storm\n  • Check for FAILED entries separately (failed_count)",
        ),
        ("ip/neighbor", "failed_count") => (
            "Neighbors in FAILED state",
            "Neighbors that could not be resolved (ARP/NDP request sent but no reply). Indicates unreachable hosts.",
            "Neighbor entries in FAILED state (unreachable).\n\n💡 Diagnostic:\n  • Non-zero → Hosts on the local network are unreachable\n  • Common causes: host powered off, wrong VLAN, IP conflict\n  • Transient failures are normal; persistent ones indicate a real problem",
        ),
        ("ip/neighbor", "neighbors") => (
            "Neighbor table entries",
            "ARP/NDP neighbor cache: IP address, network device, MAC address, and reachability state.",
            "ARP/NDP neighbor cache contents.\n\nColumns: IP, Device, LLAddr (MAC), State.\n\n💡 Diagnostic:\n  • State REACHABLE → Recently confirmed alive\n  • State STALE → Not verified recently, will re-probe on next use\n  • State FAILED → ARP/NDP resolution failed, host unreachable\n  • State PERMANENT → Statically configured entry\n  • Empty LLAddr with FAILED → Host never responded to ARP",
        ),

        // ── ss (socket summary) ──────────────────────────────────────
        ("ss", "tcp_established") => (
            "Established TCP connections",
            "Number of TCP connections in ESTABLISHED state. The primary indicator of active network sessions.",
            "ESTABLISHED TCP connection count.\n\n💡 Diagnostic:\n  • Baseline this for your workload (web server: hundreds-thousands, database: tens)\n  • Sudden increase → Traffic spike or SYN flood completing\n  • Sudden decrease → Service crash or network partition\n  • Steady growth without plateau → Connection leak in application",
        ),
        ("ss", "tcp_timewait") => (
            "TCP connections in TIME_WAIT",
            "Connections in TIME_WAIT state, cooling down for 2*MSL (typically 60s) after close.",
            "TCP TIME_WAIT socket count.\n\n💡 Diagnostic:\n  • < 5000 → Normal for most workloads\n  • > 30000 → Ephemeral port exhaustion risk\n  • High value → Many short-lived connections; consider connection pooling\n  • Enable net.ipv4.tcp_tw_reuse for busy servers",
        ),
        ("ss", "tcp_orphaned") => (
            "Orphaned TCP connections",
            "TCP connections with no owning process. Still consume kernel memory until timeout.",
            "Orphaned TCP connections.\n\n💡 Diagnostic:\n  • Should be low (< 100)\n  • Growing → Application exits without proper socket cleanup\n  • Limit: /proc/sys/net/ipv4/tcp_max_orphans\n  • Exceeding limit → Kernel forcefully RSTs connections",
        ),
        ("ss", "tcp_closed") => (
            "Closed TCP connections",
            "TCP connections in CLOSED state, waiting to be cleaned up by the kernel.",
            "TCP connections in CLOSED state.\n\n💡 Diagnostic: These are transitional — the kernel will clean them up shortly. High persistent count may indicate a kernel or driver issue.",
        ),
        ("ss", "udp_count") => (
            "Total UDP sockets",
            "Number of all UDP sockets on the system, as reported by 'ss -s'.",
            "Total UDP socket count from ss -s.\n\n💡 Diagnostic:\n  • Typical: 5-30 on a normal server\n  • Includes DNS resolvers, NTP clients, syslog, SNMP\n  • Growing → Possible FD leak in a UDP application",
        ),

        // ── dns (/etc/resolv.conf) ───────────────────────────────────
        ("dns", "nameservers") => (
            "Configured DNS nameservers",
            "DNS server addresses from /etc/resolv.conf. The system queries these servers for name resolution.",
            "DNS nameserver table from /etc/resolv.conf.\n\nColumns: IP address, Type (IPv4/IPv6).\n\n💡 Diagnostic:\n  • Empty → DNS resolution will fail. Check /etc/resolv.conf.\n  • 127.0.0.53 → systemd-resolved is managing DNS\n  • Multiple entries → Failover; first server is tried first\n  • 8.8.8.8 / 1.1.1.1 → Public DNS (Google/Cloudflare)",
        ),
        ("dns", "search_domains") => (
            "DNS search domains",
            "Domains appended to unqualified hostnames for resolution. 'search example.com' means 'host' resolves as 'host.example.com'.",
            "DNS search domain list.\n\n💡 Diagnostic:\n  • Affects short hostname resolution (e.g., 'db' becomes 'db.example.com')\n  • Too many search domains → DNS resolution slower (tries each suffix)\n  • Corporate environments often have multiple search domains",
        ),
        ("dns", "options") => (
            "DNS resolver options",
            "Resolver options from /etc/resolv.conf controlling timeout, retries, and behavior.",
            "DNS resolver options.\n\n💡 Diagnostic:\n  • 'ndots:N' → Queries with fewer than N dots try search domains first\n  • 'timeout:N' → Seconds before retrying another nameserver\n  • 'attempts:N' → Number of retries per nameserver\n  • 'edns0' → EDNS0 support (larger DNS packets)\n  • In containers, high ndots can cause slow DNS resolution",
        ),
        ("dns", "dns_resolution_ms") => (
            "DNS resolution time (localhost test)",
            "Time in milliseconds to resolve 'localhost' via the system resolver. Measures resolver overhead.",
            "DNS resolution latency for 'localhost'.\n\n💡 Diagnostic:\n  • < 1 ms → Normal (resolved from /etc/hosts or nsswitch cache)\n  • > 10 ms → Resolver may be slow (network round-trip to DNS server)\n  • > 100 ms → DNS server is unresponsive or overloaded\n  • This is a baseline test; real-world resolution may be slower for external names",
        ),

        // ── conntrack (connection tracking) ──────────────────────────
        ("conntrack", "conntrack_count") => (
            "Current tracked connections",
            "Number of entries in the netfilter connection tracking table. Each TCP/UDP flow uses one entry.",
            "Current connection tracking table entries.\n\n💡 Diagnostic:\n  • Each established connection (TCP, UDP, ICMP) uses one entry\n  • Approaches conntrack_max → New connections will be dropped!\n  • On busy NAT/firewall boxes, this is a critical capacity metric",
        ),
        ("conntrack", "conntrack_max") => (
            "Maximum tracked connections",
            "Maximum number of entries the connection tracking table can hold. Exceeding this drops new connections.",
            "Connection tracking table maximum (nf_conntrack_max).\n\n💡 Diagnostic:\n  • Default scales with RAM (typically 65536 on 1GB system)\n  • Increase: sysctl -w net.nf_conntrack_max=<value>\n  • Each entry uses ~300 bytes of kernel memory\n  • For a busy firewall/NAT: conntrack_max should be > peak concurrent connections",
        ),
        ("conntrack", "usage_pct") => (
            "Connection tracking table usage %",
            "Percentage of connection tracking table in use. Above 80% indicates risk of new connection drops.",
            "Connection tracking table utilization.\n\n💡 Diagnostic:\n  • < 50% → Healthy headroom\n  • 50-80% → Monitor trend. May need to increase max.\n  • > 80% → WARNING: Approaching limit. New connections may be dropped.\n  • > 95% → CRITICAL: Actively dropping connections. Increase conntrack_max immediately.\n  • Also check if conntrack is even needed — disable if not using NAT/iptables stateful rules",
        ),

        // ── meminfo: remaining fields ────────────────────────────────
        ("meminfo", "Active") => (
            "Recently used memory (less likely reclaimed)",
            "Memory on the active LRU list (recently accessed). Includes both anonymous and file-backed pages. Less likely to be reclaimed by the kernel.",
            "Active memory — recently accessed pages on the active LRU list.\n\nActive = Active(anon) + Active(file). Pages here were accessed recently and won't be reclaimed unless memory pressure is severe.\n\n💡 Diagnostic: Compare Active vs Inactive to see how much memory is 'hot'.",
        ),
        ("meminfo", "Inactive") => (
            "Not recently used memory (reclaim candidate)",
            "Memory on the inactive LRU list (not recently accessed). First candidate for reclaim under memory pressure.",
            "Inactive memory — pages not recently accessed.\n\nInactive = Inactive(anon) + Inactive(file). These pages are reclaimed first when the kernel needs memory.\n\n💡 Diagnostic: Large Inactive is a good buffer against memory pressure.",
        ),
        ("meminfo", "Active(anon)") => (
            "Recently used anonymous memory",
            "Anonymous memory (heap, stack, private mappings) that was recently accessed. Can only be freed by swapping.",
            "Recently accessed anonymous pages (heap, stack, mmap MAP_ANONYMOUS).\n\n💡 Diagnostic: Growing Active(anon) indicates processes are actively using more heap memory.",
        ),
        ("meminfo", "Inactive(anon)") => (
            "Idle anonymous memory (swap-out candidate)",
            "Anonymous memory not recently accessed. First candidates for swap-out under memory pressure.",
            "Idle anonymous pages eligible for swap-out.\n\n💡 Diagnostic:\n  • Large Inactive(anon) with no swap activity → Memory that could be reclaimed via swap if needed\n  • Decreasing Inactive(anon) with increasing swap used → Confirms active swapping",
        ),
        ("meminfo", "Active(file)") => (
            "Recently used file cache",
            "File-backed pages (page cache) recently accessed. Caches file contents for fast re-reads.",
            "Recently used page cache (file contents in memory).\n\n💡 Diagnostic: Large Active(file) means the file cache is working well. Shrinking Active(file) under load means memory pressure is evicting useful cache.",
        ),
        ("meminfo", "Inactive(file)") => (
            "Idle file cache (easy to reclaim)",
            "File-backed pages not recently accessed. Can be reclaimed without I/O (unless dirty).",
            "Idle page cache — file content not recently accessed.\n\n💡 Diagnostic: Large Inactive(file) = good buffer against memory pressure. Near-zero = running tight on reclaimable memory.",
        ),
        ("meminfo", "Unevictable") => (
            "Memory locked in RAM (cannot be reclaimed)",
            "Memory the kernel cannot reclaim: mlock'd pages, ramfs pages, and SHM_LOCK segments.",
            "Non-reclaimable memory — pages locked in RAM.\n\nIncludes mlock(), ramfs, SHM_LOCK.\n\n💡 Diagnostic:\n  • High Unevictable → Check which processes use mlock\n  • Reduces effective memory for caching",
        ),
        ("meminfo", "Mlocked") => (
            "Memory locked with mlock()",
            "Memory explicitly locked into RAM by processes using mlock(). Prevents swap-out.",
            "Memory locked via mlock() or mlockall().\n\nCommon users: databases, real-time audio, cryptographic key storage.\n\n💡 Diagnostic: Check per-process limits with `ulimit -l`",
        ),
        ("meminfo", "SwapCached") => (
            "Swap pages also in RAM",
            "Pages that were swapped out but are still cached in RAM. If accessed again, no disk I/O is needed.",
            "Swap cache — pages present in both swap and RAM.\n\n💡 Diagnostic: Non-zero after memory pressure event is normal recovery behavior.",
        ),
        ("meminfo", "Zswap") => (
            "Memory used by zswap compressed pool",
            "RAM consumed by zswap compressed page pool. Zswap compresses pages before writing to swap.",
            "Memory consumed by zswap's compressed page pool.\n\n💡 Diagnostic: Compare Zswap vs Zswapped for compression ratio (Zswapped / Zswap).",
        ),
        ("meminfo", "Zswapped") => (
            "Original size of pages in zswap",
            "Uncompressed size of pages stored in zswap. Compare with Zswap to see compression ratio.",
            "Original uncompressed size of pages in zswap.\n\n💡 Diagnostic: Compression ratio = Zswapped / Zswap. Higher is better. Typical: 2:1 to 4:1.",
        ),
        ("meminfo", "Dirty") => (
            "Memory waiting to be written to disk",
            "Pages modified in memory but not yet written to disk. Flushed periodically by the kernel.",
            "Dirty pages — modified data not yet flushed to storage.\n\n💡 Diagnostic:\n  • High Dirty → Heavy write workload or slow storage\n  • Dirty above dirty_ratio → write() blocks (write throttling)",
        ),
        ("meminfo", "Writeback") => (
            "Memory actively being written to disk",
            "Pages currently being written back to storage. High values indicate storage is busy.",
            "Pages currently being flushed to storage.\n\n💡 Diagnostic: Normally near zero. Sustained high → Storage device is saturated.",
        ),
        ("meminfo", "AnonPages") => (
            "Non-file-backed pages in processes",
            "Anonymous memory mapped into process page tables. Includes heap, stack, private mappings.",
            "Anonymous pages mapped into process page tables.\n\n💡 Diagnostic: AnonPages growing over time → Possible memory leak. Find culprit: `ps aux --sort=-rss | head`",
        ),
        ("meminfo", "Mapped") => (
            "Files mapped into memory (mmap)",
            "Memory occupied by files mapped via mmap(). Includes shared libraries and memory-mapped files.",
            "Memory-mapped file pages.\n\nIncludes shared libraries (.so), mmap'd files, executable text segments.\n\n💡 Diagnostic: High Mapped → Many processes sharing libraries or large mmap'd database files.",
        ),
        ("meminfo", "Shmem") => (
            "Shared memory (tmpfs, shmem, devtmpfs)",
            "Memory used by shared memory segments, tmpfs, and devtmpfs. Counts as Cached but NOT reclaimable.",
            "Shared memory pages — tmpfs, POSIX shm, SysV shm.\n\nIncluded in Cached but cannot be reclaimed.\n\n💡 Diagnostic: Check `df -h /dev/shm`. Unlike regular cache, Shmem reduces MemAvailable.",
        ),
        ("meminfo", "KReclaimable") => (
            "Kernel memory that can be reclaimed",
            "Total reclaimable kernel memory including SReclaimable slab caches.",
            "Reclaimable kernel memory (superset of SReclaimable).\n\n💡 Diagnostic: Large KReclaimable is healthy — the kernel will release it when RAM is needed.",
        ),
        ("meminfo", "Slab") => (
            "Kernel slab allocator memory total",
            "Total slab memory: SReclaimable + SUnreclaim. Holds fixed-size kernel objects.",
            "Total slab allocator memory.\n\n💡 Diagnostic: Large Slab is normal on fileservers. Growing with high SUnreclaim → Possible kernel memory leak.",
        ),
        ("meminfo", "SReclaimable") => (
            "Reclaimable slab caches",
            "Slab memory that can be freed under pressure. Primarily dentry and inode caches.",
            "Reclaimable slab caches — dentry and inode caches.\n\n💡 Diagnostic: To manually reclaim: `echo 2 > /proc/sys/vm/drop_caches`",
        ),
        ("meminfo", "SUnreclaim") => (
            "Unreclaimable slab memory",
            "Slab memory that cannot be freed. Active kernel data structures that must stay in RAM.",
            "Non-reclaimable slab memory — active kernel objects.\n\n💡 Diagnostic: Steadily growing SUnreclaim → Possible kernel memory leak. Check slabinfo.",
        ),
        ("meminfo", "KernelStack") => (
            "Memory used by kernel thread stacks",
            "Total memory for kernel-mode stacks of all threads. Each thread needs 8-16KB.",
            "Kernel stack memory for all threads.\n\n💡 Diagnostic: KernelStack / 16KB ≈ thread count. Very high → Thread leak.",
        ),
        ("meminfo", "PageTables") => (
            "Memory used by page table entries",
            "Memory for page table entries mapping virtual to physical addresses.",
            "Page table memory.\n\n💡 Diagnostic: High PageTables → Many processes or large virtual memory mappings.",
        ),
        ("meminfo", "SecPageTables") => (
            "Secondary page tables (KVM, IOMMU)",
            "Memory for nested page tables used by KVM and IOMMU.",
            "Secondary page table memory (KVM EPT/NPT, IOMMU).\n\n💡 Diagnostic: High → Many VMs running or IOMMU active.",
        ),
        ("meminfo", "NFS_Unstable") => (
            "NFS pages sent but not committed (legacy)",
            "Always 0 on modern kernels (>= 2.6.38). Deprecated field.",
            "NFS unstable pages — always 0 on modern kernels.\n\n💡 This field is deprecated. Ignore it.",
        ),
        ("meminfo", "Bounce") => (
            "Bounce buffer memory for block I/O",
            "Memory for bounce buffers when devices can't DMA to all physical memory.",
            "Bounce buffer memory. Typically 0 on modern 64-bit systems with IOMMU.\n\n💡 Diagnostic: Non-zero on 64-bit → Check for legacy devices.",
        ),
        ("meminfo", "WritebackTmp") => (
            "Temporary writeback memory (FUSE)",
            "Memory for temporary writeback buffers by FUSE filesystems.",
            "FUSE temporary writeback memory.\n\n💡 Diagnostic: Non-zero only during active FUSE writes (sshfs, s3fs, rclone).",
        ),
        ("meminfo", "CommitLimit") => (
            "Total memory available for allocation",
            "Maximum allocatable memory based on overcommit ratio. (RAM * ratio/100) + Swap.",
            "Memory commit limit based on overcommit settings.\n\n💡 Diagnostic: Only meaningful when overcommit_memory = 2 (strict). Committed_AS > CommitLimit → ENOMEM.",
        ),
        ("meminfo", "Committed_AS") => (
            "Total memory currently committed",
            "Total memory allocated by all processes. May exceed physical RAM due to overcommit.",
            "Total committed virtual memory.\n\n💡 Diagnostic: Committed_AS > RAM + Swap → System is overcommitted. OOM risk if all pages are touched.",
        ),
        ("meminfo", "VmallocTotal") => (
            "Total vmalloc address space",
            "Virtual address range for vmalloc. Extremely large on 64-bit (128TB+).",
            "Total vmalloc address space. Essentially meaningless on 64-bit systems.\n\n💡 Check VmallocUsed for actual consumption.",
        ),
        ("meminfo", "VmallocUsed") => (
            "Memory allocated via vmalloc",
            "Actual vmalloc memory in use. Used by kernel modules, iptables, etc.",
            "Vmalloc memory in use.\n\n💡 Diagnostic: Typically < 100MB. Growing → Possible kernel module memory leak.",
        ),
        ("meminfo", "VmallocChunk") => (
            "Largest free vmalloc block",
            "Largest contiguous free vmalloc block. Irrelevant on 64-bit systems.",
            "Largest contiguous free vmalloc block.\n\n💡 Only relevant on 32-bit kernels where vmalloc space is ~128MB.",
        ),
        ("meminfo", "Percpu") => (
            "Per-CPU data structure memory",
            "Memory for per-CPU variables. Scales linearly with CPU count.",
            "Per-CPU data structure memory.\n\n💡 Diagnostic: Percpu ≈ constant * nr_cpus. Unexpected growth → Module allocating excessive per-CPU data.",
        ),
        ("meminfo", "HardwareCorrupted") => (
            "Memory with hardware errors (ECC)",
            "Memory retired due to hardware errors. No longer usable.",
            "Memory retired due to ECC failures.\n\n💡 Diagnostic: Non-zero → CRITICAL hardware failure. Check edac-util or BIOS logs.",
        ),
        ("meminfo", "AnonHugePages") => (
            "Anonymous memory backed by huge pages",
            "Anonymous memory using THP (2MB). Reduces TLB misses.",
            "Anonymous transparent huge pages (THP).\n\n💡 Diagnostic: Zero → THP disabled or too fragmented. Some databases disable THP due to latency spikes.",
        ),
        ("meminfo", "ShmemHugePages") => (
            "Shared memory backed by huge pages",
            "Shared memory (tmpfs, shmem) using huge pages.",
            "Shared memory huge pages.\n\n💡 Requires tmpfs mounted with 'huge=always' or 'huge=within_size'.",
        ),
        ("meminfo", "ShmemPmdMapped") => (
            "Shared memory mapped with PMD-level huge pages",
            "Shared memory pages mapped at PMD level (2MB granularity).",
            "Shared memory mapped at PMD level.\n\n💡 Subset of ShmemHugePages actually mapped at 2MB granularity.",
        ),
        ("meminfo", "FileHugePages") => (
            "File-backed huge pages",
            "File-backed memory using transparent huge pages.",
            "File-backed transparent huge pages.\n\n💡 Relatively new feature for THP on file-backed pages.",
        ),
        ("meminfo", "FilePmdMapped") => (
            "File-backed PMD-mapped huge pages",
            "File-backed pages mapped at PMD level (2MB).",
            "File-backed pages mapped at PMD level.\n\n💡 Actual 2MB page table entries for file content.",
        ),
        ("meminfo", "CmaTotal") => (
            "Total CMA reserved memory",
            "Memory reserved by CMA for devices needing contiguous DMA buffers.",
            "CMA reserved area.\n\n💡 Diagnostic: 0 → CMA not configured (common on servers).",
        ),
        ("meminfo", "CmaFree") => (
            "Free CMA pages",
            "Unused CMA memory. Available for movable allocations when not used by DMA.",
            "Free CMA memory.\n\n💡 CmaFree = CmaTotal → Reserved but unused by devices.",
        ),
        ("meminfo", "HugePages_Total") => (
            "Total pre-allocated huge pages",
            "Huge pages pre-allocated via /proc/sys/vm/nr_hugepages. Typically 2MB each.",
            "Pre-allocated huge page pool.\n\n💡 Diagnostic: Common users: databases, DPDK, VMs. This memory is NOT available for other uses.",
        ),
        ("meminfo", "HugePages_Free") => (
            "Unallocated huge pages",
            "Huge pages not yet allocated to any process.",
            "Free huge pages.\n\n💡 Diagnostic: Free = Total → No process using huge pages; reserved memory is wasted.",
        ),
        ("meminfo", "HugePages_Rsvd") => (
            "Huge pages reserved but not allocated",
            "Huge pages committed but not yet faulted in.",
            "Reserved huge pages.\n\n💡 Free - Rsvd = truly available for new reservations.",
        ),
        ("meminfo", "HugePages_Surp") => (
            "Surplus huge pages",
            "Huge pages beyond nr_hugepages, created on demand when overcommit allowed.",
            "Surplus huge pages above nr_hugepages.\n\n💡 Freed when no longer needed.",
        ),
        ("meminfo", "Hugepagesize") => (
            "Default huge page size",
            "Size of each huge page, typically 2MB on x86_64.",
            "Default huge page size.\n\n💡 Set at boot via hugepagesz= parameter. Multiple sizes can coexist.",
        ),
        ("meminfo", "Hugetlb") => (
            "Total memory used by huge pages",
            "Total memory consumed by all huge page sizes. Reserved and unavailable for normal use.",
            "Total huge page memory.\n\n💡 Diagnostic: This memory is locked. If large but unused → Reduce nr_hugepages.",
        ),
        ("meminfo", "DirectMap4k") => (
            "Memory mapped with 4KB pages in direct map",
            "Physical memory using 4KB mappings in the kernel's direct map. Higher = more fragmented.",
            "Kernel direct map using 4KB entries.\n\n💡 Diagnostic: High relative to 2M/1G → Direct map fragmentation.",
        ),
        ("meminfo", "DirectMap2M") => (
            "Memory mapped with 2MB pages in direct map",
            "Physical memory using 2MB mappings. More efficient than 4KB.",
            "Kernel direct map using 2MB entries.\n\n💡 Most memory should be mapped at 2MB or 1GB granularity.",
        ),
        ("meminfo", "DirectMap1G") => (
            "Memory mapped with 1GB pages in direct map",
            "Physical memory using 1GB mappings. Most efficient for TLB.",
            "Kernel direct map using 1GB entries.\n\n💡 Requires CPU 'pdpe1gb' flag. 0 on systems without 1GB page support.",
        ),

        // ── pressure: remaining PSI fields ───────────────────────────
        ("pressure", "cpu_some_avg60") => (
            "CPU pressure: some stalled (60s avg)",
            "Percentage of time at least one task stalled on CPU over 60 seconds.",
            "CPU PSI — 60-second average.\n\n💡 Diagnostic: Compare avg10 vs avg60: avg10 >> avg60 = recent spike. avg10 ≈ avg60 = sustained.",
        ),
        ("pressure", "cpu_some_avg300") => (
            "CPU pressure: some stalled (5min avg)",
            "Percentage of time at least one task stalled on CPU over 5 minutes.",
            "CPU PSI — 5-minute average. Baseline indicator.\n\n💡 Diagnostic: > 5% sustained → System consistently CPU-constrained.",
        ),
        ("pressure", "cpu_some_total") => (
            "CPU pressure: total stall time (us)",
            "Cumulative microseconds at least one task stalled on CPU since boot.",
            "Total CPU stall microseconds.\n\n💡 Use delta between snapshots for current rate.",
        ),
        ("pressure", "memory_some_avg60") => (
            "Memory pressure: some stalled (60s avg)",
            "Percentage of time at least one task stalled on memory over 60 seconds.",
            "Memory PSI — 60-second average.\n\n💡 Diagnostic: Any sustained non-zero value means ongoing memory pressure.",
        ),
        ("pressure", "memory_some_avg300") => (
            "Memory pressure: some stalled (5min avg)",
            "Percentage of time at least one task stalled on memory over 5 minutes.",
            "Memory PSI — 5-minute average.\n\n💡 Diagnostic: > 10% sustained → Add RAM. > 40% → Critical thrashing.",
        ),
        ("pressure", "memory_some_total") => (
            "Memory pressure: total stall time (us)",
            "Cumulative microseconds at least one task stalled on memory since boot.",
            "Total memory stall microseconds (some).\n\n💡 Use delta for current rate.",
        ),
        ("pressure", "memory_full_avg10") => (
            "Memory pressure: ALL stalled (10s avg)",
            "Percentage of time ALL non-idle tasks stalled on memory (10s avg).",
            "Memory PSI 'full' — 10-second average. ALL tasks stalled = zero progress.\n\n💡 Diagnostic: > 0% → Very serious. > 10% → System barely functional.",
        ),
        ("pressure", "memory_full_avg60") => (
            "Memory pressure: ALL stalled (60s avg)",
            "Percentage of time ALL tasks stalled on memory (60s avg).",
            "Memory PSI 'full' — 60-second average.\n\n💡 Sustained non-zero → Critical state. Immediate action needed.",
        ),
        ("pressure", "memory_full_avg300") => (
            "Memory pressure: ALL stalled (5min avg)",
            "Percentage of time ALL tasks stalled on memory (5min avg).",
            "Memory PSI 'full' — 5-minute average.\n\n💡 Prolonged non-zero → System in crisis.",
        ),
        ("pressure", "memory_full_total") => (
            "Memory pressure: total full stall time (us)",
            "Cumulative microseconds ALL tasks stalled on memory since boot.",
            "Total memory full-stall microseconds.\n\n💡 full total <= some total. High ratio = stalls affect all tasks.",
        ),
        ("pressure", "io_some_avg60") => (
            "I/O pressure: some stalled (60s avg)",
            "Percentage of time at least one task stalled on I/O over 60 seconds.",
            "I/O PSI — 60-second average.\n\n💡 Diagnostic: Sustained > 5% → Consistent I/O bottleneck.",
        ),
        ("pressure", "io_some_avg300") => (
            "I/O pressure: some stalled (5min avg)",
            "Percentage of time at least one task stalled on I/O over 5 minutes.",
            "I/O PSI — 5-minute average.\n\n💡 Diagnostic: > 20% sustained → Consider SSD or I/O scheduler tuning.",
        ),
        ("pressure", "io_some_total") => (
            "I/O pressure: total stall time (us)",
            "Cumulative microseconds at least one task stalled on I/O since boot.",
            "Total I/O stall microseconds (some).\n\n💡 Use delta for current rate.",
        ),
        ("pressure", "io_full_avg10") => (
            "I/O pressure: ALL stalled (10s avg)",
            "Percentage of time ALL tasks stalled on I/O (10s avg).",
            "I/O PSI 'full' — 10-second average. ALL tasks waiting on I/O.\n\n💡 Diagnostic: > 10% → Severely I/O-bound.",
        ),
        ("pressure", "io_full_avg60") => (
            "I/O pressure: ALL stalled (60s avg)",
            "Percentage of time ALL tasks stalled on I/O (60s avg).",
            "I/O PSI 'full' — 60-second average.\n\n💡 Sustained → Storage cannot keep up. Upgrade to NVMe SSD.",
        ),
        ("pressure", "io_full_avg300") => (
            "I/O pressure: ALL stalled (5min avg)",
            "Percentage of time ALL tasks stalled on I/O (5min avg).",
            "I/O PSI 'full' — 5-minute average.\n\n💡 Prolonged → Fundamental I/O capacity problem.",
        ),
        ("pressure", "io_full_total") => (
            "I/O pressure: total full stall time (us)",
            "Cumulative microseconds ALL tasks stalled on I/O since boot.",
            "Total I/O full-stall microseconds.\n\n💡 High full/some ratio = stalls affect all tasks (single device bottleneck).",
        ),

        // ── vmstat: remaining important fields ───────────────────────
        ("vmstat", "nr_zone_inactive_anon") => (
            "Inactive anon pages per zone",
            "Per-zone inactive anonymous page count.",
            "Per-zone inactive anonymous pages.\n\n💡 Used by kswapd for per-zone reclaim decisions.",
        ),
        ("vmstat", "nr_zone_active_anon") => (
            "Active anon pages per zone",
            "Per-zone active anonymous page count.",
            "Per-zone active anonymous pages.\n\n💡 Used by kswapd for per-zone reclaim decisions.",
        ),
        ("vmstat", "nr_zone_inactive_file") => (
            "Inactive file pages per zone",
            "Per-zone inactive file page count.",
            "Per-zone inactive file pages.\n\n💡 First pages reclaimed per zone under memory pressure.",
        ),
        ("vmstat", "nr_zone_active_file") => (
            "Active file pages per zone",
            "Per-zone active file page count.",
            "Per-zone active file pages.\n\n💡 Tracks hot page cache per zone.",
        ),
        ("vmstat", "nr_zone_unevictable") => (
            "Unevictable pages per zone",
            "Per-zone unevictable (mlock'd, ramfs) pages.",
            "Per-zone unevictable pages.\n\n💡 Reduce effective reclaimable memory in each zone.",
        ),
        ("vmstat", "nr_zone_write_pending") => (
            "Pages with pending writes per zone",
            "Per-zone dirty + writeback pages.",
            "Per-zone write-pending pages.\n\n💡 High values → Write activity concentrated in that zone.",
        ),
        ("vmstat", "nr_mlock") => (
            "Pages locked in memory (mlock)",
            "Total mlock'd pages. Cannot be swapped.",
            "mlock'd pages.\n\n💡 Compare with Mlocked in meminfo.",
        ),
        ("vmstat", "nr_bounce") => (
            "Bounce buffer pages",
            "Bounce buffer pages for legacy DMA. Typically 0.",
            "Bounce buffer pages.\n\n💡 Should be 0 on modern 64-bit systems.",
        ),
        ("vmstat", "nr_zspages") => (
            "Compressed pages (zswap/zram)",
            "Pages in compressed memory pools.",
            "Compressed swap pages.\n\n💡 If zswap enabled, these hold compressed swapped-out data.",
        ),
        ("vmstat", "nr_free_cma") => (
            "Free CMA pages",
            "Free pages in CMA region.",
            "Free CMA pages.\n\n💡 Available for movable allocations when not needed by devices.",
        ),
        ("vmstat", "nr_file_pages") => (
            "Total file-backed pages",
            "Total page cache + swap cache + buffers.",
            "Total file-backed pages.\n\n💡 Large values are healthy — kernel is caching file data.",
        ),
        ("vmstat", "nr_shmem_hugepages") => (
            "Shared memory huge pages",
            "Huge pages for shared memory.",
            "Shared memory huge pages.\n\n💡 Non-zero when tmpfs uses huge pages.",
        ),
        ("vmstat", "nr_shmem_pmdmapped") => (
            "Shared memory PMD-mapped",
            "Shmem pages mapped at PMD level (2MB).",
            "Shmem PMD-mapped pages.\n\n💡 Actually mapped at 2MB granularity.",
        ),
        ("vmstat", "nr_file_hugepages") => (
            "File-backed huge pages",
            "Huge pages for file-backed memory.",
            "File-backed huge pages.\n\n💡 THP applied to file-backed memory.",
        ),
        ("vmstat", "nr_file_pmdmapped") => (
            "File pages mapped by PMD",
            "File pages at PMD level (2MB).",
            "File PMD-mapped pages.\n\n💡 Actual huge page mappings for file content.",
        ),
        ("vmstat", "nr_anon_transparent_hugepages") => (
            "Anonymous transparent huge pages",
            "THP count for anonymous memory. Each = 2MB.",
            "Anonymous THP count.\n\n💡 Count * 2MB = total THP memory. Zero → THP disabled or fragmented.",
        ),
        ("vmstat", "nr_vmscan_write") => (
            "Pages written during reclaim",
            "Pages written by vmscan. Indicates heavy reclaim.",
            "Vmscan writes.\n\n💡 Writing during reclaim = clean pages exhausted.",
        ),
        ("vmstat", "nr_vmscan_immediate_reclaim") => (
            "Pages immediately reclaimed",
            "Pages reclaimed immediately, bypassing LRU aging.",
            "Immediate reclaim pages.\n\n💡 High values → Aggressive reclaim under severe pressure.",
        ),
        ("vmstat", "nr_dirtied") => (
            "Total pages dirtied since boot",
            "Cumulative dirtied page count.",
            "Total dirtied pages.\n\n💡 Delta gives current dirty rate. Compare with nr_written.",
        ),
        ("vmstat", "nr_written") => (
            "Total pages written since boot",
            "Cumulative written page count.",
            "Total written pages.\n\n💡 nr_dirtied - nr_written = dirty backlog.",
        ),
        ("vmstat", "nr_kernel_stack") => (
            "Kernel stack pages",
            "Pages for kernel thread stacks.",
            "Kernel stack pages.\n\n💡 Divide by pages-per-stack to estimate thread count.",
        ),
        ("vmstat", "nr_page_table_pages") => (
            "Page table pages",
            "Pages for page table entries.",
            "Page table pages.\n\n💡 Compare with PageTables in meminfo.",
        ),
        ("vmstat", "nr_swapcached") => (
            "Pages in swap cache",
            "Pages in both RAM and swap.",
            "Swap cache pages.\n\n💡 Non-zero after memory pressure events.",
        ),
        ("vmstat", "nr_dirty_threshold") => (
            "Dirty page threshold",
            "Dirty pages at which writers are throttled.",
            "Dirty throttle threshold.\n\n💡 Tune vm.dirty_ratio if writers experience latency.",
        ),
        ("vmstat", "nr_dirty_background_threshold") => (
            "Background dirty threshold",
            "Dirty pages triggering background writeback.",
            "Background writeback threshold.\n\n💡 Tune vm.dirty_background_ratio.",
        ),
        ("vmstat", "workingset_nodes") => (
            "Workingset shadow nodes",
            "Shadow nodes tracking evicted page access.",
            "Workingset shadow nodes.\n\n💡 Used by kernel's adaptive LRU balancing.",
        ),
        ("vmstat", "workingset_refault_anon") => (
            "Anon workingset refaults",
            "Evicted anon pages faulted back in.",
            "Anon refaults.\n\n💡 High rate → Working set exceeds RAM. Thrashing.",
        ),
        ("vmstat", "workingset_refault_file") => (
            "File workingset refaults",
            "Evicted file pages re-read from disk.",
            "File refaults.\n\n💡 High rate → Page cache too small for file working set.",
        ),
        ("vmstat", "workingset_activate_anon") => (
            "Anon workingset activations",
            "Anon pages promoted via refault detection.",
            "Anon workingset activations.\n\n💡 Refaulted pages placed on active list directly.",
        ),
        ("vmstat", "workingset_activate_file") => (
            "File workingset activations",
            "File pages promoted via refault detection.",
            "File workingset activations.\n\n💡 Protects frequently-accessed file pages.",
        ),
        ("vmstat", "workingset_restore_anon") => (
            "Anon workingset restores",
            "Anon pages restored to active state.",
            "Anon workingset restores.\n\n💡 High values → Churn in anonymous working set.",
        ),
        ("vmstat", "workingset_restore_file") => (
            "File workingset restores",
            "File pages restored to active state.",
            "File workingset restores.\n\n💡 High values → Page cache under pressure.",
        ),
        ("vmstat", "workingset_nodereclaim") => (
            "Shadow nodes reclaimed",
            "Shadow nodes freed under memory pressure.",
            "Shadow node reclaim.\n\n💡 Severe pressure causes loss of workingset tracking.",
        ),
        ("vmstat", "numa_hit") => (
            "NUMA allocations on intended node",
            "Allocations on the intended NUMA node.",
            "NUMA-local allocations.\n\n💡 numa_hit / (hit + miss) = locality ratio. Close to 100% is ideal.",
        ),
        ("vmstat", "numa_miss") => (
            "NUMA allocations on wrong node",
            "Allocations on non-intended NUMA node.",
            "NUMA misses.\n\n💡 High miss rate → Remote memory latency. Use numactl --membind.",
        ),
        ("vmstat", "numa_foreign") => (
            "NUMA foreign allocations",
            "Allocations meant for this node but placed elsewhere.",
            "NUMA foreign.\n\n💡 Complement of numa_miss from the other node.",
        ),
        ("vmstat", "numa_local") => (
            "NUMA local allocations",
            "Allocations on CPU's local NUMA node.",
            "NUMA-local allocations.\n\n💡 Higher is better for performance.",
        ),
        ("vmstat", "numa_other") => (
            "NUMA remote allocations",
            "Allocations on a remote NUMA node.",
            "NUMA remote.\n\n💡 High → Poor memory affinity.",
        ),
        ("vmstat", "pgfree") => (
            "Pages freed",
            "Total pages returned to free pool since boot.",
            "Pages freed.\n\n💡 Should track with pgalloc_*.",
        ),
        ("vmstat", "pgactivate") => (
            "Pages moved to active list",
            "Pages promoted to active LRU due to access.",
            "Pages activated.\n\n💡 High rate → Healthy working set cycling.",
        ),
        ("vmstat", "pgdeactivate") => (
            "Pages moved to inactive list",
            "Pages demoted to inactive LRU during reclaim.",
            "Pages deactivated.\n\n💡 Increasing rate → Memory pressure.",
        ),
        ("vmstat", "pglazyfree") => (
            "Pages marked for lazy freeing",
            "Pages marked MADV_FREE, not yet reclaimed.",
            "Lazy-free pages.\n\n💡 Kernel reclaims only under memory pressure.",
        ),
        ("vmstat", "pglazyfreed") => (
            "Pages actually lazy-freed",
            "MADV_FREE pages reclaimed by the kernel.",
            "Lazy-freed pages.\n\n💡 Reclaimed from the lazy-free pool.",
        ),
        ("vmstat", "pgrefill") => (
            "Pages scanned during LRU refill",
            "Pages scanned refilling inactive from active list.",
            "LRU refill scans.\n\n💡 High rate → Active memory reclaim aging.",
        ),
        ("vmstat", "pgreuse") => (
            "Pages reused via fast path",
            "Pages reused without full allocation path.",
            "Fast-path reuses.\n\n💡 Higher is better.",
        ),
        ("vmstat", "pgsteal_kswapd") => (
            "Pages reclaimed by kswapd",
            "Background reclaim — no process blocked.",
            "kswapd reclaim.\n\n💡 High kswapd with low direct = healthy reclaim.",
        ),
        ("vmstat", "pgsteal_direct") => (
            "Pages reclaimed by direct reclaim",
            "Blocking reclaim — allocating process waits.",
            "Direct reclaim.\n\n💡 High → kswapd can't keep up. Need more RAM.",
        ),
        ("vmstat", "pgscan_kswapd") => (
            "Pages scanned by kswapd",
            "Pages examined during background reclaim.",
            "kswapd scans.\n\n💡 scan - steal = pages not reclaimable.",
        ),
        ("vmstat", "pgscan_direct") => (
            "Pages scanned by direct reclaim",
            "Pages examined during blocking reclaim.",
            "Direct reclaim scans.\n\n💡 High scan/steal ratio → Inefficient reclaim.",
        ),
        ("vmstat", "pgscan_direct_throttle") => (
            "Direct reclaim throttled",
            "Direct reclaim throttled to prevent CPU overuse.",
            "Reclaim throttle events.\n\n💡 Very heavy memory pressure.",
        ),
        ("vmstat", "pgscan_anon") => (
            "Anonymous pages scanned",
            "Anon pages examined for reclaim (requires swap).",
            "Anon page scans.\n\n💡 Expensive — requires swap-out.",
        ),
        ("vmstat", "pgscan_file") => (
            "File pages scanned",
            "File pages examined for reclaim.",
            "File page scans.\n\n💡 Cheaper than anon reclaim.",
        ),
        ("vmstat", "pgsteal_anon") => (
            "Anonymous pages reclaimed",
            "Anon pages swapped out.",
            "Anon pages stolen.\n\n💡 > 0 → Active swapping.",
        ),
        ("vmstat", "pgsteal_file") => (
            "File pages reclaimed",
            "File pages dropped or written back.",
            "File pages stolen.\n\n💡 Healthy reclaim path.",
        ),
        ("vmstat", "zone_reclaim_failed") => (
            "Zone reclaim failures",
            "Zone reclaim couldn't free enough pages.",
            "Zone reclaim failures.\n\n💡 NUMA zones under pressure.",
        ),
        ("vmstat", "pginodesteal") => (
            "Pages freed via inode reclaim",
            "Pages freed when inodes are evicted.",
            "Inode-steal pages.\n\n💡 Inode eviction frees associated page cache.",
        ),
        ("vmstat", "kswapd_inodesteal") => (
            "Inodes reclaimed by kswapd",
            "Inodes evicted during background reclaim.",
            "kswapd inode reclaim.\n\n💡 Frees inode memory + associated page cache.",
        ),
        ("vmstat", "kswapd_low_wmark_hit_quickly") => (
            "kswapd hit low watermark quickly",
            "Free pages dropped to low watermark soon after kswapd.",
            "Low watermark hit quickly.\n\n💡 kswapd not reclaiming enough per cycle.",
        ),
        ("vmstat", "kswapd_high_wmark_hit_quickly") => (
            "kswapd hit high watermark quickly",
            "kswapd quickly restored free pages above high watermark.",
            "High watermark hit quickly.\n\n💡 Healthy — kswapd keeping up.",
        ),
        ("vmstat", "pageoutrun") => (
            "kswapd wakeups",
            "Times kswapd woken for page-out.",
            "kswapd wakeups.\n\n💡 Increasing rate → Frequent memory pressure.",
        ),
        ("vmstat", "pgrotated") => (
            "Pages rotated to LRU tail",
            "Dirty pages rotated during reclaim.",
            "LRU rotations.\n\n💡 Many dirty pages encountered during reclaim.",
        ),
        ("vmstat", "drop_pagecache") => (
            "Page cache drops",
            "Manual page cache drop events.",
            "Page cache drops.\n\n💡 Triggered by drop_caches. Harmful in production.",
        ),
        ("vmstat", "drop_slab") => (
            "Slab cache drops",
            "Manual slab cache drop events.",
            "Slab cache drops.\n\n💡 Triggered by drop_caches.",
        ),
        ("vmstat", "pgmigrate_success") => (
            "Successful page migrations",
            "Pages migrated for NUMA balancing or compaction.",
            "Migration success.\n\n💡 High success rate is good.",
        ),
        ("vmstat", "pgmigrate_fail") => (
            "Failed page migrations",
            "Pages that couldn't be migrated.",
            "Migration failures.\n\n💡 High → Many pinned pages.",
        ),
        ("vmstat", "thp_fault_alloc") => (
            "THP fault allocations",
            "THP allocated on page fault (2MB instead of 4KB).",
            "THP fault alloc.\n\n💡 Higher is better.",
        ),
        ("vmstat", "thp_fault_fallback") => (
            "THP fault fallbacks",
            "THP allocation fell back to 4KB page.",
            "THP fault fallback.\n\n💡 High → Memory too fragmented for THP.",
        ),
        ("vmstat", "thp_collapse_alloc") => (
            "THP collapse allocations",
            "khugepaged collapsed base pages into THP.",
            "khugepaged collapses.\n\n💡 Background merging of 512 base pages into 2MB THP.",
        ),
        ("vmstat", "thp_collapse_alloc_failed") => (
            "Failed THP collapses",
            "khugepaged collapse allocation failed.",
            "Failed collapses.\n\n💡 Fragmentation prevents THP formation.",
        ),
        ("vmstat", "thp_split_page") => (
            "THP page splits",
            "THP split back into 512 base pages.",
            "THP splits.\n\n💡 High with high alloc → Workload doesn't benefit from THP.",
        ),
        ("vmstat", "thp_split_pmd") => (
            "THP PMD splits",
            "PMD entries split from 2MB to 4KB mappings.",
            "THP PMD splits.\n\n💡 Happens during partial unmap or mprotect.",
        ),
        ("vmstat", "thp_zero_page_alloc") => (
            "THP zero page allocations",
            "Huge zero pages for uninitialized memory.",
            "THP zero pages.\n\n💡 Shared read-only zero pages save memory.",
        ),
        ("vmstat", "thp_swpout") => (
            "THP swapped out whole",
            "THP swapped as single 2MB unit.",
            "THP swap-out.\n\n💡 More efficient than splitting first.",
        ),
        ("vmstat", "thp_swpout_fallback") => (
            "THP swap-out fallbacks",
            "THP had to be split before swapping.",
            "THP swap-out fallback.\n\n💡 Less efficient split-then-swap.",
        ),
        ("vmstat", "compact_stall") => (
            "Direct compaction stalls",
            "Process stalled during memory compaction.",
            "Compaction stalls.\n\n💡 High → Fragmentation causing allocation delays.",
        ),
        ("vmstat", "compact_fail") => (
            "Compaction failures",
            "Compaction failed to create contiguous block.",
            "Compaction failures.\n\n💡 High → Severe fragmentation.",
        ),
        ("vmstat", "compact_success") => (
            "Successful compactions",
            "Compaction created requested contiguous block.",
            "Compaction success.\n\n💡 success/(success+fail) = success rate.",
        ),
        ("vmstat", "compact_daemon_wake") => (
            "Compaction daemon wakeups",
            "kcompactd woken for proactive compaction.",
            "kcompactd wakeups.\n\n💡 Tunable via compaction_proactiveness.",
        ),
        ("vmstat", "compact_migrate_scanned") => (
            "Compaction migration scanned",
            "Pages examined for movable pages.",
            "Compaction migration scans.\n\n💡 High with low success → Few movable pages.",
        ),
        ("vmstat", "compact_free_scanned") => (
            "Compaction free space scanned",
            "Pages examined for free target pages.",
            "Compaction free scans.\n\n💡 More scanning = more fragmented.",
        ),
        ("vmstat", "swap_ra") => (
            "Swap read-ahead pages",
            "Pages speculatively read from swap.",
            "Swap read-ahead.\n\n💡 Effective for sequential swap access.",
        ),
        ("vmstat", "swap_ra_hit") => (
            "Swap read-ahead hits",
            "Read-ahead pages actually used.",
            "Swap read-ahead hits.\n\n💡 hit/ra = hit rate. Low → Random access; read-ahead wastes I/O.",
        ),
        ("vmstat", "balloon_inflate") => (
            "Balloon inflate (VM)",
            "Pages returned to hypervisor.",
            "Balloon inflation.\n\n💡 VM returning memory to host.",
        ),
        ("vmstat", "balloon_deflate") => (
            "Balloon deflate (VM)",
            "Pages reclaimed from hypervisor.",
            "Balloon deflation.\n\n💡 VM getting memory back from host.",
        ),
        ("vmstat", "unevictable_pgs_culled") => (
            "Unevictable pages culled",
            "Pages moved to unevictable list.",
            "Pages culled.\n\n💡 Detected as non-evictable (mlock'd, ramfs).",
        ),
        ("vmstat", "unevictable_pgs_scanned") => (
            "Unevictable pages scanned",
            "Unevictable pages encountered during scan.",
            "Unevictable scans.\n\n💡 Wasted work scanning non-reclaimable pages.",
        ),
        ("vmstat", "unevictable_pgs_rescued") => (
            "Unevictable pages rescued",
            "Pages moved back to normal LRU.",
            "Pages rescued.\n\n💡 After munlock, pages rejoin LRU.",
        ),
        ("vmstat", "unevictable_pgs_mlocked") => (
            "Pages mlocked",
            "Pages made unevictable via mlock().",
            "Pages mlocked.\n\n💡 Rate of memory locking.",
        ),
        ("vmstat", "unevictable_pgs_munlocked") => (
            "Pages munlocked",
            "Pages returned to LRU after munlock().",
            "Pages munlocked.\n\n💡 Now eligible for reclaim.",
        ),
        ("vmstat", "direct_map_level2_splits") => (
            "Direct map 2MB splits",
            "2MB direct map entries split into 4KB.",
            "2MB page splits.\n\n💡 Caused by page attribute changes. Increases TLB pressure.",
        ),
        ("vmstat", "direct_map_level3_splits") => (
            "Direct map 1GB splits",
            "1GB direct map entries split into 2MB.",
            "1GB page splits.\n\n💡 Less common but larger TLB impact.",
        ),

        // ── net/snmp — remaining IP fields ───────────────────────────
        ("net/snmp", "Ip_DefaultTTL") => (
            "Default IP Time-To-Live",
            "Default TTL value inserted into outgoing IP packets. Typically 64.",
            "Default IP TTL for outgoing packets.\n\n💡 Diagnostic: Standard value is 64 (Linux default). Lower values cause packets to be dropped before reaching distant hosts.",
        ),
        ("net/snmp", "Ip_InUnknownProtos") => (
            "IP packets with unknown protocol",
            "Packets discarded because the upper-layer protocol was unknown or unsupported.",
            "IP packets with unsupported protocol number.\n\n💡 Diagnostic: Should be 0. Non-zero may indicate custom protocol traffic or attack probes.",
        ),
        ("net/snmp", "Ip_InDiscards") => (
            "Inbound IP packets discarded",
            "Packets discarded despite being valid (e.g., insufficient buffer space). Not counted as errors.",
            "Inbound IP discards (valid packets dropped).\n\n💡 Diagnostic: Non-zero → Resource exhaustion (memory, buffers). Check system memory pressure.",
        ),
        ("net/snmp", "Ip_ReasmTimeout") => (
            "IP reassembly timeouts",
            "Fragment reassembly attempts that timed out. Missing fragments caused the entire datagram to be dropped.",
            "IP fragment reassembly timeouts.\n\n💡 Diagnostic: Non-zero → Fragments arriving but not all pieces arrive in time. Firewall may be blocking some fragments.",
        ),
        ("net/snmp", "Ip_ReasmReqds") => (
            "IP fragments needing reassembly",
            "IP fragments received that needed to be reassembled into complete datagrams.",
            "IP fragments received requiring reassembly.\n\n💡 Diagnostic: Should be near 0 on modern networks with PMTUD. Non-zero → MTU mismatch on some path.",
        ),
        ("net/snmp", "Ip_ReasmOKs") => (
            "IP datagrams reassembled successfully",
            "IP datagrams successfully reconstructed from fragments.",
            "Successful IP fragment reassembly.\n\n💡 Diagnostic: Compare with ReasmReqds and ReasmFails to gauge reassembly success rate.",
        ),
        ("net/snmp", "Ip_FragOKs") => (
            "IP datagrams fragmented successfully",
            "IP datagrams that were successfully fragmented for transmission.",
            "Successful IP fragmentations.\n\n💡 Diagnostic: Non-zero → This host is fragmenting outgoing packets. Consider increasing MTU or enabling PMTUD.",
        ),
        ("net/snmp", "Ip_FragFails") => (
            "IP fragmentation failures (DF set)",
            "IP datagrams that needed fragmentation but had the Don't Fragment flag set.",
            "IP fragmentation failures — DF bit prevented fragmentation.\n\n💡 Diagnostic: Non-zero → Outgoing packets too large for the path MTU and DF is set. PMTUD should generate ICMP 'fragmentation needed' back to the sender.",
        ),
        ("net/snmp", "Ip_FragCreates") => (
            "IP fragments created",
            "Total IP fragments generated by fragmenting larger datagrams.",
            "IP fragments created.\n\n💡 Diagnostic: Each fragmented datagram produces multiple fragments. High values indicate significant fragmentation overhead.",
        ),

        // ── net/snmp — remaining ICMP fields ─────────────────────────
        ("net/snmp", "Icmp_InCsumErrors") => (
            "ICMP checksum errors",
            "ICMP messages received with invalid checksums. Indicates data corruption.",
            "ICMP checksum errors.\n\n💡 Diagnostic: Should be 0. Non-zero → Link-level corruption affecting ICMP packets.",
        ),
        ("net/snmp", "Icmp_InTimeExcds") => (
            "ICMP Time Exceeded received",
            "ICMP Time Exceeded messages received, typically from traceroute or TTL expiry.",
            "ICMP Time Exceeded messages received.\n\n💡 Diagnostic: Normal during traceroute. Persistent high rate without traceroute → Routing loop somewhere.",
        ),
        ("net/snmp", "Icmp_InParmProbs") => (
            "ICMP Parameter Problem received",
            "ICMP Parameter Problem messages indicating malformed IP headers detected by a remote host.",
            "ICMP Parameter Problem messages received.\n\n💡 Diagnostic: Should be 0. Non-zero → This host is sending malformed packets. Check NIC offload settings.",
        ),
        ("net/snmp", "Icmp_InSrcQuenchs") => (
            "ICMP Source Quench received (deprecated)",
            "ICMP Source Quench messages received. Deprecated congestion signaling mechanism.",
            "ICMP Source Quench received.\n\n💡 Diagnostic: Deprecated in RFC 6633. Modern systems ignore these. Should be 0.",
        ),
        ("net/snmp", "Icmp_InRedirects") => (
            "ICMP Redirect received",
            "ICMP Redirect messages received, suggesting a better route exists.",
            "ICMP Redirect messages received.\n\n💡 Diagnostic: Non-zero → A router is telling this host to use a different gateway. May indicate suboptimal routing or ICMP redirect attack.",
        ),
        ("net/snmp", "Icmp_InEchos") => (
            "ICMP Echo Request received (ping)",
            "Ping requests received. Each one triggers an Echo Reply if not filtered.",
            "ICMP Echo Request (ping) received.\n\n💡 Diagnostic: High rate → Someone is pinging this host frequently, or possible ping flood attack.",
        ),
        ("net/snmp", "Icmp_InEchoReps") => (
            "ICMP Echo Reply received",
            "Ping replies received in response to Echo Requests sent by this host.",
            "ICMP Echo Reply received.\n\n💡 Diagnostic: Should correlate with outbound pings. More replies than requests → Unsolicited replies (unusual).",
        ),
        ("net/snmp", "Icmp_InTimestamps") => (
            "ICMP Timestamp Request received",
            "ICMP Timestamp Request messages received. Rarely used in modern networks.",
            "ICMP Timestamp requests received.\n\n💡 Diagnostic: Should be 0 on modern networks. Non-zero may be reconnaissance probing.",
        ),
        ("net/snmp", "Icmp_InTimestampReps") => (
            "ICMP Timestamp Reply received",
            "ICMP Timestamp Reply messages received.",
            "ICMP Timestamp replies received.\n\n💡 Diagnostic: Rarely seen. Used for time synchronization probing.",
        ),
        ("net/snmp", "Icmp_InAddrMasks") => (
            "ICMP Address Mask Request received",
            "ICMP Address Mask Request messages received. Obsolete.",
            "ICMP Address Mask requests.\n\n💡 Diagnostic: Obsolete. Should be 0. Non-zero → Very old equipment or probing.",
        ),
        ("net/snmp", "Icmp_InAddrMaskReps") => (
            "ICMP Address Mask Reply received",
            "ICMP Address Mask Reply messages received. Obsolete.",
            "ICMP Address Mask replies.\n\n💡 Diagnostic: Obsolete protocol feature. Should be 0.",
        ),
        ("net/snmp", "Icmp_OutErrors") => (
            "ICMP messages failed to send",
            "ICMP messages that could not be sent due to errors.",
            "ICMP output errors.\n\n💡 Diagnostic: Non-zero → Kernel failed to send ICMP responses. Check for rate limiting (net.ipv4.icmp_ratelimit).",
        ),
        ("net/snmp", "Icmp_OutDestUnreachs") => (
            "ICMP Destination Unreachable sent",
            "ICMP Destination Unreachable messages sent in response to traffic to closed ports or unreachable destinations.",
            "ICMP Destination Unreachable sent.\n\n💡 Diagnostic: High rate → Many packets arriving for closed ports (port scan or misconfigured clients).",
        ),
        ("net/snmp", "Icmp_OutTimeExcds") => (
            "ICMP Time Exceeded sent",
            "ICMP Time Exceeded messages sent when TTL reaches zero while forwarding.",
            "ICMP Time Exceeded sent.\n\n💡 Diagnostic: Non-zero only if host is forwarding packets. Means forwarded packets had insufficient TTL.",
        ),
        ("net/snmp", "Icmp_OutParmProbs") => (
            "ICMP Parameter Problem sent",
            "ICMP Parameter Problem messages sent for packets with malformed headers.",
            "ICMP Parameter Problem sent.\n\n💡 Diagnostic: Should be 0. Non-zero → Receiving malformed IP packets from peers.",
        ),
        ("net/snmp", "Icmp_OutSrcQuenchs") => (
            "ICMP Source Quench sent (deprecated)",
            "ICMP Source Quench messages sent. Deprecated and should not be generated.",
            "ICMP Source Quench sent.\n\n💡 Diagnostic: Deprecated. Should always be 0 on modern kernels.",
        ),
        ("net/snmp", "Icmp_OutRedirects") => (
            "ICMP Redirect sent",
            "ICMP Redirect messages sent to inform hosts of a better route.",
            "ICMP Redirect sent.\n\n💡 Diagnostic: Non-zero → This host is acting as a router and redirecting traffic. Normal for gateways.",
        ),
        ("net/snmp", "Icmp_OutEchos") => (
            "ICMP Echo Request sent (ping)",
            "Ping requests sent by this host.",
            "ICMP Echo Request (ping) sent.\n\n💡 Diagnostic: Corresponds to outbound ping activity. Monitoring tools often generate these.",
        ),
        ("net/snmp", "Icmp_OutEchoReps") => (
            "ICMP Echo Reply sent",
            "Ping replies sent in response to incoming Echo Requests.",
            "ICMP Echo Reply sent.\n\n💡 Diagnostic: Should match InEchos roughly. Discrepancy → Some pings being filtered.",
        ),
        ("net/snmp", "Icmp_OutTimestamps") => (
            "ICMP Timestamp Request sent",
            "ICMP Timestamp Request messages sent. Rarely used.",
            "ICMP Timestamp requests sent.\n\n💡 Diagnostic: Should be 0 unless a specific tool is using timestamp probing.",
        ),
        ("net/snmp", "Icmp_OutTimestampReps") => (
            "ICMP Timestamp Reply sent",
            "ICMP Timestamp Reply messages sent.",
            "ICMP Timestamp replies sent.\n\n💡 Diagnostic: Matches InTimestamps if not filtered.",
        ),
        ("net/snmp", "Icmp_OutAddrMasks") => (
            "ICMP Address Mask Request sent",
            "ICMP Address Mask Request messages sent. Obsolete.",
            "ICMP Address Mask requests sent.\n\n💡 Diagnostic: Obsolete. Should be 0.",
        ),
        ("net/snmp", "Icmp_OutAddrMaskReps") => (
            "ICMP Address Mask Reply sent",
            "ICMP Address Mask Reply messages sent. Obsolete.",
            "ICMP Address Mask replies sent.\n\n💡 Diagnostic: Obsolete. Should be 0.",
        ),

        // ── net/snmp — remaining TCP fields ──────────────────────────
        ("net/snmp", "Tcp_RtoAlgorithm") => (
            "TCP RTO algorithm",
            "Retransmission timeout algorithm in use. 4 = Van Jacobson (standard).",
            "TCP retransmission timeout algorithm.\n\n💡 Diagnostic: 4 = Van Jacobson's algorithm (RFC 6298). This is the standard on modern Linux.",
        ),
        ("net/snmp", "Tcp_RtoMin") => (
            "TCP minimum RTO (ms)",
            "Minimum retransmission timeout in milliseconds. Lower values enable faster retransmit.",
            "Minimum TCP retransmission timeout.\n\n💡 Diagnostic: Default 200ms. Lower = faster retransmit but more spurious retransmissions on jittery networks.",
        ),
        ("net/snmp", "Tcp_RtoMax") => (
            "TCP maximum RTO (ms)",
            "Maximum retransmission timeout in milliseconds. Caps how long TCP waits before retransmitting.",
            "Maximum TCP retransmission timeout.\n\n💡 Diagnostic: Default 120000ms (120s). Very long — allows TCP to survive extended outages.",
        ),
        ("net/snmp", "Tcp_MaxConn") => (
            "TCP maximum connections",
            "Maximum TCP connections allowed. -1 means dynamically determined by the kernel.",
            "Maximum TCP connections.\n\n💡 Diagnostic: -1 = no fixed limit (kernel manages dynamically). This is the standard value.",
        ),

        // ── net/snmp — remaining UDP fields ──────────────────────────
        ("net/snmp", "Udp_IgnoredMulti") => (
            "UDP multicast datagrams ignored",
            "Multicast UDP datagrams received but ignored (not joined to the multicast group).",
            "Ignored UDP multicast datagrams.\n\n💡 Diagnostic: Normal on networks with multicast traffic. Host received multicast but has no interested listener.",
        ),
        ("net/snmp", "Udp_MemErrors") => (
            "UDP memory allocation failures",
            "UDP datagrams dropped due to memory allocation failures in the kernel.",
            "UDP memory allocation failures.\n\n💡 Diagnostic: Non-zero → Severe memory pressure affecting network stack. Check system memory and TCP/UDP memory limits.",
        ),

        // ── net/snmp — UdpLite fields ────────────────────────────────
        ("net/snmp", "UdpLite_InDatagrams") => (
            "UDP-Lite datagrams received",
            "UDP-Lite datagrams received and delivered to applications. UDP-Lite allows partial checksums.",
            "UDP-Lite datagrams received.\n\n💡 Diagnostic: Usually 0. UDP-Lite is used for error-tolerant media streams where partial data is better than no data.",
        ),
        ("net/snmp", "UdpLite_NoPorts") => (
            "UDP-Lite packets to closed ports",
            "UDP-Lite datagrams received for ports with no listening process.",
            "UDP-Lite datagrams to ports with no listener.\n\n💡 Diagnostic: Should be 0 unless UDP-Lite services are expected.",
        ),
        ("net/snmp", "UdpLite_InErrors") => (
            "UDP-Lite input errors",
            "UDP-Lite datagrams that could not be delivered for any reason.",
            "UDP-Lite input errors.\n\n💡 Diagnostic: Should be 0 on systems not using UDP-Lite.",
        ),
        ("net/snmp", "UdpLite_OutDatagrams") => (
            "UDP-Lite datagrams sent",
            "UDP-Lite datagrams sent by this host.",
            "UDP-Lite datagrams sent.\n\n💡 Diagnostic: Non-zero → An application is using UDP-Lite (rare).",
        ),
        ("net/snmp", "UdpLite_RcvbufErrors") => (
            "UDP-Lite receive buffer overflows",
            "UDP-Lite datagrams dropped due to receive buffer full.",
            "UDP-Lite receive buffer drops.\n\n💡 Diagnostic: Same implications as UDP RcvbufErrors — application too slow to read.",
        ),
        ("net/snmp", "UdpLite_SndbufErrors") => (
            "UDP-Lite send buffer overflows",
            "UDP-Lite datagrams dropped due to send buffer full.",
            "UDP-Lite send buffer drops.\n\n💡 Diagnostic: Application sending faster than the NIC can transmit.",
        ),
        ("net/snmp", "UdpLite_InCsumErrors") => (
            "UDP-Lite checksum errors",
            "UDP-Lite datagrams with checksum errors in the covered portion.",
            "UDP-Lite checksum errors.\n\n💡 Diagnostic: Note that UDP-Lite only checksums a portion of the datagram; this counts errors in that covered portion.",
        ),
        ("net/snmp", "UdpLite_IgnoredMulti") => (
            "UDP-Lite multicast ignored",
            "UDP-Lite multicast datagrams received but ignored.",
            "Ignored UDP-Lite multicast datagrams.\n\n💡 Diagnostic: Similar to UDP IgnoredMulti. Should be 0.",
        ),
        ("net/snmp", "UdpLite_MemErrors") => (
            "UDP-Lite memory allocation failures",
            "UDP-Lite datagrams dropped due to memory allocation failures.",
            "UDP-Lite memory errors.\n\n💡 Diagnostic: Non-zero → Severe memory pressure. Same fix as UDP MemErrors.",
        ),

        // ── net/netstat — remaining TcpExt fields ────────────────────
        ("net/netstat", "TcpExt_EmbryonicRsts") => (
            "RSTs for embryonic connections",
            "RST segments received for connections in SYN_RECV state (half-open).",
            "RSTs received for embryonic (SYN_RECV) connections.\n\n💡 Diagnostic: Non-zero → Clients aborting connections during handshake. Port scan or load balancer health checks.",
        ),
        ("net/netstat", "TcpExt_PruneCalled") => (
            "Socket buffer prune calls",
            "Times the kernel pruned socket buffer memory to reduce memory consumption.",
            "Socket buffer pruning events.\n\n💡 Diagnostic: Non-zero → TCP memory pressure. Kernel is shrinking socket buffers to cope.",
        ),
        ("net/netstat", "TcpExt_RcvPruned") => (
            "Packets pruned from receive queue",
            "Packets dropped from the receive queue due to memory pressure.",
            "Receive queue pruning.\n\n💡 Diagnostic: Non-zero → Application not reading fast enough AND system under memory pressure. Data loss.",
        ),
        ("net/netstat", "TcpExt_OfoPruned") => (
            "Out-of-order packets pruned",
            "Packets dropped from the out-of-order queue due to memory pressure.",
            "Out-of-order queue pruning.\n\n💡 Diagnostic: Non-zero → Severe memory pressure causing loss of reordering data. Retransmissions will follow.",
        ),
        ("net/netstat", "TcpExt_OutOfWindowIcmps") => (
            "ICMP messages dropped (out of window)",
            "ICMP messages dropped because they referenced a TCP segment outside the window.",
            "Out-of-window ICMP drops.\n\n💡 Diagnostic: Security feature preventing ICMP-based blind attacks. Normal to see some.",
        ),
        ("net/netstat", "TcpExt_LockDroppedIcmps") => (
            "ICMP messages dropped (socket locked)",
            "ICMP messages dropped because the target socket was locked.",
            "Locked-socket ICMP drops.\n\n💡 Diagnostic: Transient condition — socket was busy. Usually harmless.",
        ),
        ("net/netstat", "TcpExt_ArpFilter") => (
            "Packets filtered by ARP filter",
            "Packets filtered by the kernel ARP filter mechanism.",
            "ARP filter drops.\n\n💡 Diagnostic: Non-zero → ARP filtering active (arp_filter sysctl). Expected on multi-homed hosts.",
        ),
        ("net/netstat", "TcpExt_TWRecycled") => (
            "TIME_WAIT sockets recycled by timestamp",
            "TIME_WAIT sockets reused early via TCP timestamp validation.",
            "TIME_WAIT recycling via timestamps.\n\n💡 Diagnostic: Enabled by tcp_tw_recycle (deprecated in kernel 4.12). Should be 0 on modern kernels.",
        ),
        ("net/netstat", "TcpExt_TWKilled") => (
            "TIME_WAIT sockets destroyed",
            "TIME_WAIT sockets forcefully destroyed before natural expiry.",
            "TIME_WAIT sockets killed.\n\n💡 Diagnostic: Forced cleanup of TIME_WAIT sockets. May indicate TIME_WAIT bucket pressure.",
        ),
        ("net/netstat", "TcpExt_PAWSActive") => (
            "Active connections rejected by PAWS",
            "Connection attempts rejected by PAWS (Protection Against Wrapped Sequences) on active open.",
            "PAWS rejections on active open.\n\n💡 Diagnostic: Non-zero → Stale connections in TIME_WAIT with old timestamps blocking new connections to the same tuple.",
        ),
        ("net/netstat", "TcpExt_DelayedACKLocked") => (
            "Delayed ACKs further delayed (locked)",
            "Delayed ACKs that were postponed because the socket was locked by the application.",
            "Delayed ACKs postponed by socket lock.\n\n💡 Diagnostic: Application is holding the socket lock during ACK timer. May add latency.",
        ),
        ("net/netstat", "TcpExt_DelayedACKLost") => (
            "Delayed ACK loss detected",
            "Quick ACK mode activated because a delayed ACK was apparently lost.",
            "Delayed ACK loss detection.\n\n💡 Diagnostic: Triggers quick ACK mode. Indicates the delayed ACK mechanism misjudged timing.",
        ),
        ("net/netstat", "TcpExt_TCPHPAcks") => (
            "ACKs via header prediction fast path",
            "ACKs processed via the optimized header prediction path. Higher is better.",
            "Header prediction fast-path ACKs.\n\n💡 Diagnostic: High ratio relative to total ACKs → Efficient TCP stack processing. Low → Unusual packet patterns.",
        ),
        ("net/netstat", "TcpExt_TCPRenoRecovery") => (
            "Reno fast recovery events",
            "TCP loss recoveries using the Reno (non-SACK) fast recovery algorithm.",
            "TCP Reno fast recovery events.\n\n💡 Diagnostic: Reno is the fallback when SACK is not available. SACK recovery is preferred.",
        ),
        ("net/netstat", "TcpExt_TCPSACKReneging") => (
            "SACK data reneged by receiver",
            "Times the receiver reneged on previously SACKed data. The receiver said it had data, then didn't.",
            "SACK reneging events.\n\n💡 Diagnostic: Rare and problematic. Receiver is discarding previously acknowledged data, forcing retransmission.",
        ),
        ("net/netstat", "TcpExt_TCPSACKReorder") => (
            "Reordering detected via SACK",
            "Packet reordering events detected by SACK information.",
            "SACK-detected reordering.\n\n💡 Diagnostic: Non-zero → Network path reorders packets. SACK handles this well. Compare with other reorder counters.",
        ),
        ("net/netstat", "TcpExt_TCPRenoReorder") => (
            "Reordering detected via Reno",
            "Packet reordering events detected by the Reno fast retransmit algorithm.",
            "Reno-detected reordering.\n\n💡 Diagnostic: Reordering detected without SACK. Less precise than SACK-based detection.",
        ),
        ("net/netstat", "TcpExt_TCPTSReorder") => (
            "Reordering detected via timestamp",
            "Packet reordering events detected by TCP timestamp analysis.",
            "Timestamp-detected reordering.\n\n💡 Diagnostic: Another reordering detection method. Non-zero → Path has reordering.",
        ),
        ("net/netstat", "TcpExt_TCPFullUndo") => (
            "Full congestion window undo",
            "Times the congestion window was fully restored after detecting a spurious congestion event.",
            "Full cwnd undo after spurious congestion detection.\n\n💡 Diagnostic: Good — the stack detected that congestion was spurious and restored full throughput.",
        ),
        ("net/netstat", "TcpExt_TCPPartialUndo") => (
            "Partial congestion window undo",
            "Times the congestion window was partially restored after spurious congestion detection.",
            "Partial cwnd undo.\n\n💡 Diagnostic: Some recovery from spurious congestion, but not complete. Less ideal than full undo.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKUndo") => (
            "Undo via DSACK detection",
            "Congestion window restored after DSACK revealed the retransmission was unnecessary.",
            "DSACK-based undo of congestion response.\n\n💡 Diagnostic: DSACK detected that a retransmission was spurious. Good — throughput recovered.",
        ),
        ("net/netstat", "TcpExt_TCPLossUndo") => (
            "Loss detection undo",
            "Times loss detection was found spurious and the congestion state was restored.",
            "Loss detection undo.\n\n💡 Diagnostic: The kernel thought packets were lost but they arrived. Network may have high jitter.",
        ),
        ("net/netstat", "TcpExt_TCPLostRetransmit") => (
            "Retransmitted segments lost again",
            "Segments that were retransmitted but the retransmission was also lost.",
            "Lost retransmissions.\n\n💡 Diagnostic: Very bad — even retransmissions are getting lost. Severe network path problem.",
        ),
        ("net/netstat", "TcpExt_TCPRenoFailures") => (
            "Reno recovery failures",
            "Times Reno fast recovery failed to recover from loss.",
            "Reno recovery failures.\n\n💡 Diagnostic: Reno couldn't recover; fell back to timeout. SACK-capable peers avoid this.",
        ),
        ("net/netstat", "TcpExt_TCPSackFailures") => (
            "SACK recovery failures",
            "Times SACK-based recovery failed to recover from loss.",
            "SACK recovery failures.\n\n💡 Diagnostic: Even SACK couldn't recover — very heavy or sustained packet loss.",
        ),
        ("net/netstat", "TcpExt_TCPLossFailures") => (
            "Loss recovery failures",
            "Times loss-based recovery failed.",
            "Loss recovery failures.\n\n💡 Diagnostic: Loss recovery algorithm couldn't repair the situation. Connection likely timed out.",
        ),
        ("net/netstat", "TcpExt_TCPSlowStartRetrans") => (
            "Retransmits during slow start",
            "Segments retransmitted while in TCP slow start phase.",
            "Slow start retransmissions.\n\n💡 Diagnostic: Loss during slow start is expensive — it resets the congestion window growth.",
        ),
        ("net/netstat", "TcpExt_TCPLossProbeRecovery") => (
            "Recoveries via loss probe",
            "Times a Tail Loss Probe (TLP) successfully triggered recovery without full RTO.",
            "TLP-triggered recoveries.\n\n💡 Diagnostic: TLP working as intended — recovering tail losses faster than RTO.",
        ),
        ("net/netstat", "TcpExt_TCPRenoRecoveryFail") => (
            "Reno recovery entered but failed",
            "Times Reno fast recovery was entered but ultimately failed.",
            "Reno recovery entry failures.\n\n💡 Diagnostic: Entered Reno recovery but loss was too severe. Connection fell back to timeout.",
        ),
        ("net/netstat", "TcpExt_TCPSackRecoveryFail") => (
            "SACK recovery entered but failed",
            "Times SACK recovery was entered but ultimately failed.",
            "SACK recovery entry failures.\n\n💡 Diagnostic: Even SACK recovery couldn't handle the loss pattern. Very heavy packet loss.",
        ),
        ("net/netstat", "TcpExt_TCPRcvCollapsed") => (
            "Receive queue packets collapsed",
            "Packets merged in the receive queue under memory pressure to save space.",
            "Receive queue collapse events.\n\n💡 Diagnostic: Non-zero → Memory pressure causing kernel to merge receive queue entries. Performance impact.",
        ),
        ("net/netstat", "TcpExt_TCPBacklogCoalesce") => (
            "Backlog packets coalesced",
            "Packets merged in the socket backlog for efficiency.",
            "Socket backlog coalescing.\n\n💡 Diagnostic: Normal optimization. Packets merged before application reads them.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOldSent") => (
            "DSACK sent for old data",
            "Duplicate SACK sent for data that was already received (retransmission was unnecessary).",
            "DSACK sent for already-received data.\n\n💡 Diagnostic: Tells the sender its retransmission was unnecessary. Helps the sender detect spurious retransmits.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOfoSent") => (
            "DSACK sent for out-of-order data",
            "Duplicate SACK sent for out-of-order segments.",
            "DSACK sent for out-of-order data.\n\n💡 Diagnostic: Signals that reordering caused duplicate delivery detection.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKRecv") => (
            "DSACKs received",
            "Duplicate SACK blocks received from peers, indicating our retransmission was unnecessary.",
            "DSACK blocks received.\n\n💡 Diagnostic: High count → This host is retransmitting unnecessarily. RTO may be too aggressive.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOfoRecv") => (
            "DSACKs received for out-of-order",
            "Duplicate SACK blocks received indicating out-of-order delivery on the reverse path.",
            "Out-of-order DSACK blocks received.\n\n💡 Diagnostic: Network path to this host has reordering.",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnLinger") => (
            "Connections aborted after linger timeout",
            "Connections aborted because the linger timeout expired with data still pending.",
            "TCP abort on linger timeout.\n\n💡 Diagnostic: Application set SO_LINGER but data couldn't be flushed in time.",
        ),
        ("net/netstat", "TcpExt_TCPAbortFailed") => (
            "Failed connection abort attempts",
            "Attempts to abort a connection that failed.",
            "Failed TCP abort attempts.\n\n💡 Diagnostic: Very rare. Indicates kernel-level issue with connection teardown.",
        ),
        ("net/netstat", "TcpExt_TCPMemoryPressuresChrono") => (
            "TCP memory pressure duration (ms)",
            "Total duration in milliseconds the TCP stack was under memory pressure.",
            "TCP memory pressure duration.\n\n💡 Diagnostic: Non-zero → System spent time in TCP memory pressure mode. Correlates with reduced buffer sizes.",
        ),
        ("net/netstat", "TcpExt_TCPSACKDiscard") => (
            "SACK blocks discarded",
            "SACK blocks that were discarded as invalid or unusable.",
            "Discarded SACK blocks.\n\n💡 Diagnostic: Non-zero → Peer sending invalid SACK information. Possible middlebox or buggy stack.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKIgnoredOld") => (
            "DSACK blocks ignored (old)",
            "DSACK blocks ignored because they referenced old data.",
            "Old DSACK blocks ignored.\n\n💡 Diagnostic: DSACK was too late to be useful. Harmless but indicates delayed feedback.",
        ),
        ("net/netstat", "TcpExt_TCPDSACKIgnoredNoUndo") => (
            "DSACK blocks ignored (no undo possible)",
            "DSACK blocks ignored because undo was not possible at that point.",
            "DSACK ignored (no undo).\n\n💡 Diagnostic: DSACK arrived but congestion state had already moved on. Can't undo the cwnd reduction.",
        ),
        ("net/netstat", "TcpExt_TCPMD5NotFound") => (
            "Missing TCP MD5 signatures",
            "Segments expected to have TCP MD5 signatures but didn't.",
            "Missing TCP-MD5 signatures.\n\n💡 Diagnostic: Non-zero → MD5-protected connection (BGP) receiving unsigned packets. Security concern.",
        ),
        ("net/netstat", "TcpExt_TCPMD5Unexpected") => (
            "Unexpected TCP MD5 signatures",
            "Segments with TCP MD5 signatures that weren't expected.",
            "Unexpected TCP-MD5 signatures.\n\n💡 Diagnostic: Peer is sending MD5-signed packets to a non-MD5 connection. Configuration mismatch.",
        ),
        ("net/netstat", "TcpExt_TCPMD5Failure") => (
            "TCP MD5 signature verification failures",
            "Segments with TCP MD5 signatures that failed verification.",
            "TCP-MD5 signature failures.\n\n💡 Diagnostic: MD5 key mismatch between peers. Check BGP MD5 password configuration.",
        ),
        ("net/netstat", "TcpExt_TCPSackShifted") => (
            "SACK data shifted in retransmit queue",
            "SACK blocks shifted to optimize the retransmit queue layout.",
            "SACK shift operations.\n\n💡 Diagnostic: Internal optimization. High values indicate active SACK processing.",
        ),
        ("net/netstat", "TcpExt_TCPSackMerged") => (
            "SACK blocks merged",
            "Adjacent SACK blocks merged in the retransmit queue.",
            "SACK merge operations.\n\n💡 Diagnostic: Normal optimization to reduce SACK block overhead.",
        ),
        ("net/netstat", "TcpExt_TCPSackShiftFallback") => (
            "SACK shift fallbacks",
            "SACK shift optimization fell back to traditional processing.",
            "SACK shift fallbacks.\n\n💡 Diagnostic: Shift optimization couldn't be applied. Falls back to slower path.",
        ),
        ("net/netstat", "TcpExt_PFMemallocDrop") => (
            "Segments dropped in pfmemalloc",
            "TCP segments dropped because they were received in a pfmemalloc (emergency memory) context.",
            "PF_MEMALLOC drops.\n\n💡 Diagnostic: Non-zero → Extreme memory pressure. Only memory-freeing operations are allowed.",
        ),
        ("net/netstat", "TcpExt_TCPMinTTLDrop") => (
            "Segments dropped below minimum TTL",
            "Segments dropped because the IP TTL was below the socket's minimum TTL threshold.",
            "Minimum TTL drops.\n\n💡 Diagnostic: IP_MINTTL socket option is filtering packets. Used as a lightweight security measure (e.g., BGP GTSM).",
        ),
        ("net/netstat", "TcpExt_TCPDeferAcceptDrop") => (
            "Segments dropped with DEFER_ACCEPT",
            "Segments dropped on sockets with TCP_DEFER_ACCEPT because no data arrived.",
            "TCP_DEFER_ACCEPT drops.\n\n💡 Diagnostic: Server uses DEFER_ACCEPT to delay accept() until data arrives. Bare ACKs are dropped.",
        ),
        ("net/netstat", "TcpExt_IPReversePathFilter") => (
            "Reverse path filter drops",
            "Packets dropped by IP reverse path filtering (anti-spoofing).",
            "Reverse path filter drops.\n\n💡 Diagnostic: Non-zero → Packets arriving on unexpected interfaces. Could be asymmetric routing or spoofing.",
        ),
        ("net/netstat", "TcpExt_TCPReqQFullDoCookies") => (
            "SYN cookies used (queue full)",
            "Times SYN cookies were activated because the request queue was full.",
            "SYN cookies activated by full queue.\n\n💡 Diagnostic: SYN flood protection engaged. Increase tcp_max_syn_backlog if caused by legitimate traffic.",
        ),
        ("net/netstat", "TcpExt_TCPReqQFullDrop") => (
            "SYNs dropped (queue full)",
            "SYN segments dropped because the request queue was full and SYN cookies were not available.",
            "SYN drops from full request queue.\n\n💡 Diagnostic: Connections being refused. Increase tcp_max_syn_backlog or enable SYN cookies.",
        ),
        ("net/netstat", "TcpExt_TCPRetransFail") => (
            "Failed retransmit attempts",
            "TCP retransmission attempts that failed (e.g., could not allocate memory for the retransmit).",
            "Failed retransmit attempts.\n\n💡 Diagnostic: Non-zero → Kernel couldn't even retransmit. Severe resource exhaustion.",
        ),
        ("net/netstat", "TcpExt_TCPOFODrop") => (
            "Out-of-order packets dropped",
            "Packets dropped from the out-of-order queue, losing reordering information.",
            "Out-of-order queue drops.\n\n💡 Diagnostic: Non-zero → Memory pressure forcing loss of OOO data. Will trigger retransmissions.",
        ),
        ("net/netstat", "TcpExt_TCPOFOMerge") => (
            "Out-of-order packets merged",
            "Packets merged in the out-of-order queue to reduce memory usage.",
            "Out-of-order queue merges.\n\n💡 Diagnostic: Normal optimization. Adjacent OOO segments combined.",
        ),
        ("net/netstat", "TcpExt_TCPSYNChallenge") => (
            "SYN segments triggering challenge ACKs",
            "SYN segments on established connections that triggered a challenge ACK response.",
            "SYN challenge events.\n\n💡 Diagnostic: Possible blind SYN injection attack attempt (RFC 5961). Challenge ACK validates the connection.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenActiveFail") => (
            "TCP Fast Open active failures",
            "Failed outbound TCP Fast Open connection attempts.",
            "TFO active connection failures.\n\n💡 Diagnostic: Server may not support TFO, or middlebox is stripping TFO options.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassiveFail") => (
            "TCP Fast Open passive failures",
            "Failed inbound TCP Fast Open accept attempts.",
            "TFO passive accept failures.\n\n💡 Diagnostic: TFO cookie validation failed or server-side TFO error.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenListenOverflow") => (
            "TCP Fast Open listen overflow",
            "TCP Fast Open requests dropped because the listen queue overflowed.",
            "TFO listen overflow.\n\n💡 Diagnostic: Same as ListenOverflow but specific to TFO connections. Increase somaxconn.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenCookieReqd") => (
            "TCP Fast Open cookie required",
            "TCP Fast Open SYNs that required a cookie but didn't have one.",
            "TFO cookie required.\n\n💡 Diagnostic: First connection to a TFO server needs a cookie request round-trip. Subsequent connections are fast.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenBlackhole") => (
            "TCP Fast Open blackhole detected",
            "TCP Fast Open blackhole events where TFO data was silently dropped by a middlebox.",
            "TFO blackhole detection.\n\n💡 Diagnostic: A middlebox (firewall, NAT) is dropping TFO SYN+data. TCP falls back to normal handshake.",
        ),
        ("net/netstat", "TcpExt_TCPSpuriousRtxHostQueues") => (
            "Spurious retransmits from host queues",
            "Retransmissions detected as spurious because packets were delayed in local host queues.",
            "Spurious retransmits from local queuing.\n\n💡 Diagnostic: Local queueing delay (qdisc, NIC ring buffer) caused premature RTO. Not a network problem.",
        ),
        ("net/netstat", "TcpExt_BusyPollRxPackets") => (
            "Packets received via busy polling",
            "Network packets received through busy polling (low-latency polling mode).",
            "Busy poll RX packets.\n\n💡 Diagnostic: Non-zero → Application using SO_BUSY_POLL for low-latency packet reception.",
        ),
        ("net/netstat", "TcpExt_TCPFromZeroWindowAdv") => (
            "Zero-window to non-zero transitions",
            "Times the receive window transitioned from zero back to non-zero.",
            "Zero-to-nonzero window transitions.\n\n💡 Diagnostic: Receiver recovered from zero-window condition. Application caught up with reading.",
        ),
        ("net/netstat", "TcpExt_TCPToZeroWindowAdv") => (
            "Non-zero to zero-window transitions",
            "Times the receive window dropped to zero, stopping the sender.",
            "Window-to-zero transitions.\n\n💡 Diagnostic: Non-zero → Application can't read fast enough. Sender is paused. Optimize application read path.",
        ),
        ("net/netstat", "TcpExt_TCPWantZeroWindowAdv") => (
            "Zero-window advertisement wanted",
            "Times a zero-window advertisement was desired but conditions prevented it.",
            "Zero-window advertisement wanted.\n\n💡 Diagnostic: Kernel wanted to advertise zero window but couldn't. Borderline buffer pressure.",
        ),
        ("net/netstat", "TcpExt_TCPHystartTrainDetect") => (
            "Hystart training phase detections",
            "Congestion detected during the Hystart training phase of cubic congestion control.",
            "Hystart training detections.\n\n💡 Diagnostic: Hystart limits slow-start overshoot. Detections prevent excessive packet bursts.",
        ),
        ("net/netstat", "TcpExt_TCPHystartTrainCwnd") => (
            "Hystart cwnd at training detection",
            "Congestion window size when Hystart training detection triggered.",
            "Hystart training cwnd.\n\n💡 Diagnostic: Average value indicates typical bandwidth-delay product at detection point.",
        ),
        ("net/netstat", "TcpExt_TCPHystartDelayDetect") => (
            "Hystart delay-based detections",
            "Congestion detected via Hystart delay increase measurement.",
            "Hystart delay detections.\n\n💡 Diagnostic: RTT increase triggered exit from slow start. Prevents overshoot.",
        ),
        ("net/netstat", "TcpExt_TCPHystartDelayCwnd") => (
            "Hystart cwnd at delay detection",
            "Congestion window size when Hystart delay detection triggered.",
            "Hystart delay detection cwnd.\n\n💡 Diagnostic: Indicates connection bandwidth at the point where delay-based detection kicked in.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedSynRecv") => (
            "ACKs skipped in SYN-RECV state",
            "ACK segments skipped because the connection was in SYN-RECV state.",
            "ACKs skipped in SYN-RECV.\n\n💡 Diagnostic: Connection not yet established. ACK was premature.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedPAWS") => (
            "ACKs skipped by PAWS",
            "ACK segments skipped by Protection Against Wrapped Sequences check.",
            "ACKs skipped by PAWS.\n\n💡 Diagnostic: Old duplicate ACK detected by timestamp. Normal security mechanism.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedSeq") => (
            "ACKs skipped (sequence mismatch)",
            "ACK segments skipped because the sequence number didn't match expected.",
            "ACKs skipped by sequence check.\n\n💡 Diagnostic: Stale or invalid ACK. May indicate middlebox interference.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedFinWait2") => (
            "ACKs skipped in FIN-WAIT-2",
            "ACK segments skipped because the connection was in FIN-WAIT-2 state.",
            "ACKs skipped in FIN-WAIT-2.\n\n💡 Diagnostic: Connection closing, ACK is irrelevant at this stage.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedTimeWait") => (
            "ACKs skipped in TIME-WAIT",
            "ACK segments skipped because the connection was in TIME-WAIT state.",
            "ACKs skipped in TIME-WAIT.\n\n💡 Diagnostic: Normal — old ACKs arriving for closed connections.",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedChallenge") => (
            "ACKs skipped by challenge processing",
            "ACK segments skipped during challenge ACK processing.",
            "ACKs skipped by challenge ACK.\n\n💡 Diagnostic: Rate limiting challenge ACK responses. Security feature (RFC 5961).",
        ),
        ("net/netstat", "TcpExt_TCPMTUPFail") => (
            "TCP MTU probe failures",
            "TCP path MTU discovery probe failures.",
            "TCP PMTUD probe failures.\n\n💡 Diagnostic: Path MTU probe packet was lost. TCP will retry with smaller size.",
        ),
        ("net/netstat", "TcpExt_TCPMTUPSuccess") => (
            "TCP MTU probe successes",
            "TCP path MTU discovery probes that succeeded in finding a larger MTU.",
            "TCP PMTUD probe successes.\n\n💡 Diagnostic: Successfully increased MSS. Improves throughput by reducing per-packet overhead.",
        ),
        ("net/netstat", "TcpExt_TCPDeliveredCE") => (
            "TCP segments delivered with ECN CE",
            "TCP segments delivered to the application that had the ECN Congestion Experienced mark.",
            "ECN CE-marked segments delivered.\n\n💡 Diagnostic: Non-zero → Network is signaling congestion via ECN. TCP reduces rate accordingly.",
        ),
        ("net/netstat", "TcpExt_TCPAckCompressed") => (
            "Compressed ACKs sent (GRO)",
            "ACKs compressed by GRO (Generic Receive Offload) to reduce processing overhead.",
            "GRO-compressed ACKs.\n\n💡 Diagnostic: Normal NIC offload optimization. Reduces interrupt and processing overhead.",
        ),
        ("net/netstat", "TcpExt_TCPZeroWindowDrop") => (
            "Segments dropped (zero window)",
            "TCP segments dropped because the receive window was zero.",
            "Zero-window segment drops.\n\n💡 Diagnostic: Receiver's buffer is full. Application must read data faster.",
        ),
        ("net/netstat", "TcpExt_TCPRcvQDrop") => (
            "Segments dropped from receive queue",
            "TCP segments dropped from the receive queue.",
            "Receive queue drops.\n\n💡 Diagnostic: Application not reading socket fast enough. Data loss.",
        ),
        ("net/netstat", "TcpExt_TCPWqueueTooBig") => (
            "Write queue exceeded limit",
            "Times the TCP write queue exceeded its size limit.",
            "Write queue overflow.\n\n💡 Diagnostic: Application writing faster than TCP can send. Backpressure needed.",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassiveAltKey") => (
            "TFO passive connections with alternate key",
            "TCP Fast Open passive connections using an alternate cookie key.",
            "TFO alternate key connections.\n\n💡 Diagnostic: Key rotation in progress. New and old keys both accepted temporarily.",
        ),

        // ── net/netstat — remaining IpExt fields ─────────────────────
        ("net/netstat", "IpExt_InNoRoutes") => (
            "Inbound packets with no route",
            "Packets dropped because no route existed for the destination address.",
            "No-route inbound drops.\n\n💡 Diagnostic: Traffic arriving for destinations this host can't reach. Possible misconfigured routing.",
        ),
        ("net/netstat", "IpExt_InTruncatedPkts") => (
            "Inbound truncated packets",
            "Packets dropped because they were truncated (shorter than the IP header indicated).",
            "Truncated packet drops.\n\n💡 Diagnostic: Non-zero → Corrupted packets on the network. Check NIC and cables.",
        ),
        ("net/netstat", "IpExt_OutMcastPkts") => (
            "Multicast packets sent",
            "IP multicast packets sent by this host.",
            "Outbound multicast packets.\n\n💡 Diagnostic: Non-zero → Host is sending multicast traffic (service discovery, cluster communication).",
        ),
        ("net/netstat", "IpExt_InBcastPkts") => (
            "Broadcast packets received",
            "IP broadcast packets received.",
            "Inbound broadcast packets.\n\n💡 Diagnostic: Normal on local networks. High rate → Broadcast storm or chatty protocol.",
        ),
        ("net/netstat", "IpExt_OutBcastPkts") => (
            "Broadcast packets sent",
            "IP broadcast packets sent by this host.",
            "Outbound broadcast packets.\n\n💡 Diagnostic: Common for ARP, DHCP. Excessive → Check for broadcast-heavy applications.",
        ),
        ("net/netstat", "IpExt_InMcastOctets") => (
            "Multicast bytes received",
            "Total bytes received in multicast packets.",
            "Inbound multicast bytes.\n\n💡 Diagnostic: Rate of change shows multicast bandwidth consumption.",
        ),
        ("net/netstat", "IpExt_OutMcastOctets") => (
            "Multicast bytes sent",
            "Total bytes sent in multicast packets.",
            "Outbound multicast bytes.\n\n💡 Diagnostic: Rate shows multicast output bandwidth. High → Video streaming or cluster sync.",
        ),
        ("net/netstat", "IpExt_InBcastOctets") => (
            "Broadcast bytes received",
            "Total bytes received in broadcast packets.",
            "Inbound broadcast bytes.\n\n💡 Diagnostic: High rate → Broadcast-heavy network environment.",
        ),
        ("net/netstat", "IpExt_OutBcastOctets") => (
            "Broadcast bytes sent",
            "Total bytes sent in broadcast packets.",
            "Outbound broadcast bytes.\n\n💡 Diagnostic: Should be small. Large values → Unusual broadcast activity.",
        ),
        ("net/netstat", "IpExt_InNoECTPkts") => (
            "Packets without ECN capability",
            "Inbound packets without ECN-Capable Transport flag set.",
            "Non-ECT inbound packets.\n\n💡 Diagnostic: Most packets on the internet are non-ECT. This is the baseline counter.",
        ),
        ("net/netstat", "IpExt_InECT1Pkts") => (
            "Packets with ECT(1) flag",
            "Inbound packets with ECN-Capable Transport codepoint 1.",
            "ECT(1) inbound packets.\n\n💡 Diagnostic: Used by some ECN implementations. Indicates ECN-capable traffic.",
        ),
        ("net/netstat", "IpExt_InECT0Pkts") => (
            "Packets with ECT(0) flag",
            "Inbound packets with ECN-Capable Transport codepoint 0. The standard ECN marking.",
            "ECT(0) inbound packets.\n\n💡 Diagnostic: Standard ECN marking. Non-zero → ECN-capable peers are communicating.",
        ),
        ("net/netstat", "IpExt_InCEPkts") => (
            "Packets with ECN Congestion Experienced",
            "Inbound packets with the Congestion Experienced ECN flag. Routers set this to signal congestion.",
            "ECN CE-marked inbound packets.\n\n💡 Diagnostic: Non-zero → Network routers are signaling congestion. TCP will reduce sending rate.",
        ),
        ("net/netstat", "IpExt_ReasmOverlaps") => (
            "IP reassembly overlapping fragments",
            "Overlapping fragments detected during IP reassembly. May indicate an attack.",
            "Overlapping IP fragments.\n\n💡 Diagnostic: Non-zero → Possible fragment overlap attack or buggy sender. Linux drops overlapping fragments.",
        ),

        // ── net/netstat — MPTcpExt fields ────────────────────────────
        ("net/netstat", "MPTcpExt_MPCapableSYNRX") => (
            "MPTCP capable SYN received",
            "SYN segments received with MPTCP capability option.",
            "MPTCP capable SYN RX.\n\n💡 Diagnostic: Non-zero → Clients are attempting MPTCP connections.",
        ),
        ("net/netstat", "MPTcpExt_MPCapableSYNTX") => (
            "MPTCP capable SYN sent",
            "SYN segments sent with MPTCP capability option.",
            "MPTCP capable SYN TX.\n\n💡 Diagnostic: Non-zero → This host is initiating MPTCP connections.",
        ),
        ("net/netstat", "MPTcpExt_MPCapableSYNACKRX") => (
            "MPTCP capable SYN-ACK received",
            "SYN-ACK segments received confirming MPTCP support.",
            "MPTCP SYN-ACK RX.\n\n💡 Diagnostic: Server accepted the MPTCP capability.",
        ),
        ("net/netstat", "MPTcpExt_MPCapableACKRX") => (
            "MPTCP capable ACK received",
            "Final ACK of MPTCP handshake received, completing MPTCP setup.",
            "MPTCP ACK RX.\n\n💡 Diagnostic: MPTCP connection fully established.",
        ),
        ("net/netstat", "MPTcpExt_MPCapableFallbackACK") => (
            "MPTCP fallback on ACK",
            "MPTCP fell back to regular TCP on the final ACK.",
            "MPTCP fallback on ACK.\n\n💡 Diagnostic: Middlebox stripped MPTCP options, forcing fallback.",
        ),
        ("net/netstat", "MPTcpExt_MPCapableFallbackSYNACK") => (
            "MPTCP fallback on SYN-ACK",
            "MPTCP fell back to regular TCP on the SYN-ACK.",
            "MPTCP fallback on SYN-ACK.\n\n💡 Diagnostic: Server doesn't support MPTCP or middlebox interference.",
        ),
        ("net/netstat", "MPTcpExt_MPFallbackTokenInit") => (
            "MPTCP token init fallback",
            "MPTCP connection fell back during token initialization.",
            "MPTCP token init fallback.\n\n💡 Diagnostic: Token collision or resource exhaustion.",
        ),
        ("net/netstat", "MPTcpExt_MPTCPRetrans") => (
            "MPTCP retransmissions",
            "MPTCP segments retransmitted at the MPTCP layer.",
            "MPTCP retransmissions.\n\n💡 Diagnostic: Non-zero → Packet loss on MPTCP subflows.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinNoTokenFound") => (
            "MPTCP join: no token found",
            "MPTCP join requests that couldn't find a matching connection token.",
            "MPTCP join token miss.\n\n💡 Diagnostic: Connection no longer exists or token mismatch.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynRx") => (
            "MPTCP join SYN received",
            "MPTCP subflow join SYN segments received.",
            "MPTCP join SYN RX.\n\n💡 Diagnostic: Peers requesting additional subflows.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynAckRx") => (
            "MPTCP join SYN-ACK received",
            "MPTCP subflow join SYN-ACK segments received.",
            "MPTCP join SYN-ACK RX.\n\n💡 Diagnostic: Subflow join handshake progressing.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynAckHMacFailure") => (
            "MPTCP join SYN-ACK HMAC failure",
            "MPTCP join SYN-ACK HMAC authentication failed.",
            "MPTCP join HMAC failure.\n\n💡 Diagnostic: Authentication failure — key mismatch or tampering.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinAckRx") => (
            "MPTCP join ACK received",
            "MPTCP subflow join ACK segments received, completing subflow setup.",
            "MPTCP join ACK RX.\n\n💡 Diagnostic: Subflow join completed.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinAckHMacFailure") => (
            "MPTCP join ACK HMAC failure",
            "MPTCP join ACK HMAC authentication failed.",
            "MPTCP join ACK HMAC failure.\n\n💡 Diagnostic: Subflow join authentication failed.",
        ),
        ("net/netstat", "MPTcpExt_DSSNotMatching") => (
            "MPTCP DSS not matching",
            "MPTCP Data Sequence Signal segments that didn't match expected state.",
            "MPTCP DSS mismatch.\n\n💡 Diagnostic: Data mapping error between MPTCP and TCP layers.",
        ),
        ("net/netstat", "MPTcpExt_InfiniteMapRx") => (
            "MPTCP infinite map received",
            "MPTCP infinite mapping segments received, indicating fallback to regular TCP.",
            "MPTCP infinite map RX.\n\n💡 Diagnostic: Peer is falling back from MPTCP to regular TCP for this connection.",
        ),
        ("net/netstat", "MPTcpExt_OFOQueueTail") => (
            "MPTCP OFO queue tail additions",
            "Segments added to the tail of the MPTCP out-of-order queue.",
            "MPTCP OFO queue tail.\n\n💡 Diagnostic: Out-of-order data being queued for reordering.",
        ),
        ("net/netstat", "MPTcpExt_OFOQueue") => (
            "MPTCP segments queued out-of-order",
            "MPTCP segments queued in the out-of-order queue.",
            "MPTCP OFO queue.\n\n💡 Diagnostic: Normal when multiple subflows have different latencies.",
        ),
        ("net/netstat", "MPTcpExt_OFOMerge") => (
            "MPTCP OFO segments merged",
            "MPTCP out-of-order segments merged together.",
            "MPTCP OFO merge.\n\n💡 Diagnostic: Normal optimization for adjacent OFO segments.",
        ),
        ("net/netstat", "MPTcpExt_NoDSSInWindow") => (
            "MPTCP no DSS in window",
            "MPTCP segments without Data Sequence Signal within the receive window.",
            "MPTCP no DSS in window.\n\n💡 Diagnostic: Mapping gap in MPTCP data.",
        ),
        ("net/netstat", "MPTcpExt_DuplicateData") => (
            "MPTCP duplicate data received",
            "Duplicate data segments received at the MPTCP layer.",
            "MPTCP duplicate data.\n\n💡 Diagnostic: Data retransmitted across different subflows, or subflow retransmission.",
        ),
        ("net/netstat", "MPTcpExt_AddAddr") => (
            "MPTCP ADD_ADDR received",
            "MPTCP ADD_ADDR options received, announcing additional addresses.",
            "MPTCP ADD_ADDR RX.\n\n💡 Diagnostic: Peer is advertising additional network addresses for subflow creation.",
        ),
        ("net/netstat", "MPTcpExt_EchoAdd") => (
            "MPTCP ADD_ADDR echo sent",
            "MPTCP ADD_ADDR echo options sent to acknowledge address addition.",
            "MPTCP ADD_ADDR echo.\n\n💡 Diagnostic: Acknowledging peer's address advertisement.",
        ),
        ("net/netstat", "MPTcpExt_PortAdd") => (
            "MPTCP port-based ADD_ADDR received",
            "MPTCP ADD_ADDR options with port information received.",
            "MPTCP port ADD_ADDR.\n\n💡 Diagnostic: Peer advertising additional port for subflows.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortSynRx") => (
            "MPTCP port-based join SYN received",
            "MPTCP subflow join SYN on a different port.",
            "MPTCP port join SYN RX.\n\n💡 Diagnostic: Subflow join attempt on alternate port.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortSynAckRx") => (
            "MPTCP port-based join SYN-ACK received",
            "MPTCP subflow join SYN-ACK on a different port.",
            "MPTCP port join SYN-ACK RX.\n\n💡 Diagnostic: Port-based subflow join progressing.",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortAckRx") => (
            "MPTCP port-based join ACK received",
            "MPTCP subflow join ACK on a different port, completing setup.",
            "MPTCP port join ACK RX.\n\n💡 Diagnostic: Port-based subflow established.",
        ),
        ("net/netstat", "MPTcpExt_MismatchPortSynRx") => (
            "MPTCP join SYN with mismatched port",
            "MPTCP join SYN received on an unexpected port.",
            "MPTCP port mismatch SYN.\n\n💡 Diagnostic: Port doesn't match expected subflow parameters.",
        ),
        ("net/netstat", "MPTcpExt_MismatchPortAckRx") => (
            "MPTCP join ACK with mismatched port",
            "MPTCP join ACK received on an unexpected port.",
            "MPTCP port mismatch ACK.\n\n💡 Diagnostic: Port mismatch during subflow join.",
        ),
        ("net/netstat", "MPTcpExt_RmAddr") => (
            "MPTCP RM_ADDR received",
            "MPTCP RM_ADDR options received, removing an address.",
            "MPTCP RM_ADDR RX.\n\n💡 Diagnostic: Peer is removing a previously advertised address.",
        ),
        ("net/netstat", "MPTcpExt_RmAddrDrop") => (
            "MPTCP RM_ADDR dropped",
            "MPTCP RM_ADDR options that were dropped.",
            "MPTCP RM_ADDR drops.\n\n💡 Diagnostic: Address removal request couldn't be processed.",
        ),
        ("net/netstat", "MPTcpExt_RmSubflow") => (
            "MPTCP subflows removed",
            "MPTCP subflows removed after RM_ADDR.",
            "MPTCP subflow removal.\n\n💡 Diagnostic: Subflows torn down due to address removal.",
        ),
        ("net/netstat", "MPTcpExt_MPPrioTx") => (
            "MPTCP MP_PRIO sent",
            "MPTCP priority change options sent.",
            "MPTCP MP_PRIO TX.\n\n💡 Diagnostic: Changing subflow priority (backup vs primary).",
        ),
        ("net/netstat", "MPTcpExt_MPPrioRx") => (
            "MPTCP MP_PRIO received",
            "MPTCP priority change options received.",
            "MPTCP MP_PRIO RX.\n\n💡 Diagnostic: Peer changing subflow priority.",
        ),
        ("net/netstat", "MPTcpExt_MPFailTx") => (
            "MPTCP MP_FAIL sent",
            "MPTCP failure notification sent.",
            "MPTCP MP_FAIL TX.\n\n💡 Diagnostic: Notifying peer of MPTCP-level failure.",
        ),
        ("net/netstat", "MPTcpExt_MPFailRx") => (
            "MPTCP MP_FAIL received",
            "MPTCP failure notification received from peer.",
            "MPTCP MP_FAIL RX.\n\n💡 Diagnostic: Peer reporting MPTCP failure. May trigger fallback.",
        ),
        ("net/netstat", "MPTcpExt_MPFastcloseTx") => (
            "MPTCP MP_FASTCLOSE sent",
            "MPTCP fast close options sent to immediately tear down the connection.",
            "MPTCP FASTCLOSE TX.\n\n💡 Diagnostic: Immediate connection teardown requested.",
        ),
        ("net/netstat", "MPTcpExt_MPFastcloseRx") => (
            "MPTCP MP_FASTCLOSE received",
            "MPTCP fast close options received from peer.",
            "MPTCP FASTCLOSE RX.\n\n💡 Diagnostic: Peer requesting immediate connection teardown.",
        ),
        ("net/netstat", "MPTcpExt_MPRstTx") => (
            "MPTCP MP_RST sent",
            "MPTCP reset options sent.",
            "MPTCP RST TX.\n\n💡 Diagnostic: Resetting MPTCP subflow.",
        ),
        ("net/netstat", "MPTcpExt_MPRstRx") => (
            "MPTCP MP_RST received",
            "MPTCP reset options received.",
            "MPTCP RST RX.\n\n💡 Diagnostic: Peer resetting MPTCP subflow.",
        ),
        ("net/netstat", "MPTcpExt_RcvPruned") => (
            "MPTCP received data pruned",
            "MPTCP received data pruned due to memory pressure.",
            "MPTCP receive pruning.\n\n💡 Diagnostic: Memory pressure causing MPTCP data loss.",
        ),
        ("net/netstat", "MPTcpExt_SubflowStale") => (
            "MPTCP stale subflows detected",
            "MPTCP subflows detected as stale (no progress).",
            "MPTCP stale subflows.\n\n💡 Diagnostic: Subflow is not making forward progress. May be rerouted.",
        ),
        ("net/netstat", "MPTcpExt_SubflowRecover") => (
            "MPTCP stale subflows recovered",
            "MPTCP stale subflows that recovered and resumed data transfer.",
            "MPTCP subflow recovery.\n\n💡 Diagnostic: Previously stale subflow is working again.",
        ),
        ("net/netstat", "MPTcpExt_SndWndShared") => (
            "MPTCP shared send window events",
            "MPTCP send window sharing events between subflows.",
            "MPTCP shared send window.\n\n💡 Diagnostic: Window management across subflows.",
        ),
        ("net/netstat", "MPTcpExt_RcvWndShared") => (
            "MPTCP shared receive window events",
            "MPTCP receive window sharing events between subflows.",
            "MPTCP shared receive window.\n\n💡 Diagnostic: Window management across subflows.",
        ),
        ("net/netstat", "MPTcpExt_RcvWndConflictUpdate") => (
            "MPTCP receive window conflict updates",
            "MPTCP receive window conflicts resolved by updating.",
            "MPTCP window conflict update.\n\n💡 Diagnostic: Window negotiation between subflows required adjustment.",
        ),
        ("net/netstat", "MPTcpExt_RcvWndConflict") => (
            "MPTCP receive window conflicts",
            "MPTCP receive window conflicts between subflows.",
            "MPTCP window conflicts.\n\n💡 Diagnostic: Subflows disagree on receive window. Resolved by the MPTCP stack.",
        ),

        _ => return None,
    })
}

fn field_desc_ja(source: &str, field: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match (source, field) {
        // meminfo
        ("meminfo", "MemTotal") => (
            "利用可能な物理メモリ合計",
            "システムに搭載されている物理メモリからカーネル予約領域を引いた量。これがシステムが使える最大メモリ。",
            "利用可能な物理メモリの合計（カーネル予約分を除く）。\n\nMemAvailable が MemTotal の 10% を切ると、OOM Killer がプロセスを強制終了し始める可能性がある。64GB サーバーなら約 6.4GB を下回ると危険。\n\n💡 診断: MemTotal が物理メモリより少ない場合、BIOS のメモリ予約や `dmesg | grep Memory` を確認。",
        ),
        ("meminfo", "MemFree") => (
            "完全未使用のメモリ",
            "何にも使われていないメモリ。Linux はディスクキャッシュに空きメモリを使うので、健全なシステムでもこの値は低い。パニックせず MemAvailable を見ること。",
            "完全に未使用のメモリ — キャッシュにすら使われていない。\n\nよくある誤解:「サーバーのメモリが空いてない！」実は Linux は空きメモリを積極的にディスクキャッシュ（Cached + Buffers）として使う。これは良いこと — 使われないメモリは無駄なメモリ。\n\n💡 診断: MemFree も MemAvailable も低い → 本当のメモリ不足。MemFree だけ低くて MemAvailable は十分 → 健全な状態。",
        ),
        ("meminfo", "MemAvailable") => (
            "新しいプロセスに使えるメモリ",
            "スワップなしで確保できるメモリの推定値。回収可能なキャッシュとバッファを含む。MemFree ではなくこの値を監視すべき。",
            "スワップなしで確保できるメモリのカーネル推定値。\n\nメモリ圧迫度を測る最重要メトリクス。空きメモリ＋回収可能なページキャッシュ − 予約ウォーターマーク。\n\n💡 診断:\n  • MemTotal の 10% 未満 → OOM 危険ゾーン、即調査\n  • 20% 未満 → 警告、メモリリーク確認（RSS が増え続けるプロセス）\n  • 継続的に減少 → メモリリークの可能性大。`ps aux --sort=-rss | head` で犯人を探せ",
        ),
        ("meminfo", "Buffers") => (
            "ブロックデバイスバッファ用メモリ",
            "ブロックデバイスの生 I/O バッファ（ファイルシステムのメタデータ等）。通常は少量。メモリ圧迫時に回収される。",
            "ブロックデバイス I/O バッファ（ファイルシステムメタデータ、スーパーブロック等）。\n\nCached（ページキャッシュ）とは別。Buffers はディスクのメタデータ、Cached はファイルの中身。両方とも回収可能。\n\n💡 Buffers が異常に大きい場合、多数のブロックデバイスか、メタデータ集中操作（`find /` や巨大ディレクトリのリスト表示など）が原因。",
        ),
        ("meminfo", "Cached") => (
            "ファイルキャッシュ用メモリ",
            "ページキャッシュ — ディスクから読んだファイルを次回高速アクセスのためメモリに保持。メモリ圧迫時に回収される。値が高いのは正常で良いこと。",
            "ページキャッシュ — カーネルのファイル内容キャッシュ。\n\nディスクから読んだファイルは全てここにキャッシュされる。健全な Linux で「空きメモリが少ない」ように見えるのはこれのおかげ — カーネルが仕事をしている証拠。\n\n💡 診断: 負荷下で Cached がほぼゼロに落ちたら、スラッシング状態 — RAM 不足でキャッシュを維持できず、大量の I/O が発生している。",
        ),
        ("meminfo", "SwapTotal") => (
            "スワップ領域の合計",
            "利用可能なスワップ領域（ディスクベースの仮想メモリ）。物理 RAM が不足した時に使用される。",
            "設定済みスワップ領域の合計。\n\nスワップは RAM のオーバーフロー先 — 使われてないページがディスクに退避される。多少のスワップ使用は正常だが、激しいスワップ活動（vmstat の si/so を確認）は RAM 不足のサイン。\n\n💡 診断:\n  • SwapTotal = 0 → スワップ未設定。OOM Killer が唯一の安全装置。\n  • SSD 上のスワップ → 許容範囲。HDD 上のスワップ → 圧迫時に深刻な性能劣化。",
        ),
        ("meminfo", "SwapFree") => (
            "空きスワップ領域",
            "未使用のスワップ。時間とともに減少しているなら、メモリ圧迫でスワッピングが発生中。",
            "残りの未使用スワップ。\n\n💡 診断:\n  • SwapTotal - SwapFree > 0 で安定 → 以前スワップアウトされたページがある。正常。\n  • SwapFree が継続的に減少 → アクティブにスワッピング中、本当のメモリ圧迫。\n  • SwapFree = 0 → 次に RAM に収まらないメモリ確保で OOM Killer が発動。",
        ),

        // loadavg
        ("loadavg", "load1") => (
            "1分間の負荷平均",
            "過去1分間の実行可能・割り込み不可プロセスの平均数。CPU数と比較して飽和度を判断する。",
            "1分間の CPU 負荷平均。\n\n負荷平均がカウントするのは: (1) CPU 上で実行中、(2) CPU 待ち、(3) 割り込み不可 I/O 待ち（D 状態）のプロセス数。\n\n💡 診断:\n  • load1 < CPU数 → 余裕あり\n  • load1 ≈ CPU数 → フル稼働、許容範囲\n  • load1 > CPU数 × 2 → 深刻な過負荷。プロセスがキューに溜まっている\n  • load1 >> load15 → 今まさに負荷が急上昇中\n  • load1 << load15 → 直近のスパイクから回復中",
        ),
        ("loadavg", "load5") => (
            "5分間の負荷平均",
            "過去5分間の負荷平均。1分平均より安定したトレンド指標。",
            "5分間の負荷平均 — トレンド指標。\n\nload1 と load5 を比較して方向を見る:\n  • load1 > load5 → 負荷上昇中\n  • load1 < load5 → 負荷減少中\n  • load1 ≈ load5 → 安定",
        ),
        ("loadavg", "load15") => (
            "15分間の負荷平均",
            "過去15分間の負荷平均。ベースラインの負荷を把握するのに最適。",
            "15分間の負荷平均 — ベースライン。\n\nこのシステムの「普通」がわかる。load15 が常に 4.0 前後なら、load1 が 8.0 に跳ねたら注目すべきだが、4.5 なら日常的。\n\n💡 load15 がずっと高いなら、CPU 追加が必要か、永続的な I/O ボトルネック（PSI の pressure データを確認）がある。",
        ),

        // net/tcp
        ("net/tcp", "connections") => (
            "TCP 接続一覧",
            "全 TCP 接続のテーブル。主要な状態: ESTABLISHED（通信中）、TIME_WAIT（切断中）、SYN_SENT（接続試行中）。",
            "システム上の全 TCP 接続。\n\n💡 接続状態別の診断パターン:\n  • SYN_SENT 多数 → 接続先がダウン、FW でドロップ、または DNS が遅い\n  • TIME_WAIT 多数 → 短命な接続を大量処理中。HTTP サーバーでは正常だが、1万超はエフェメラルポート枯渇の恐れ\n  • CLOSE_WAIT 多数 → アプリがソケットを閉じてない。相手は切断済みなのに close() を呼んでない。典型的な FD リーク\n  • 同一 IP への ESTABLISHED 多数 → コネクションプールか永続接続。DB 接続では正常",
        ),

        // processes
        ("processes", "processes") => (
            "プロセス一覧",
            "全プロセス: PID、名前、状態、RSS メモリ、スレッド数、UID。状態: S=休眠、R=実行中、Z=ゾンビ、D=割り込み不可 I/O。",
            "システム上の全プロセス。\n\n💡 診断パターン:\n  • Z（ゾンビ）が蓄積 → 親プロセスが wait() を呼んでない。ゾンビ自体はリソースを消費しないが、バグのある親プロセスを示す\n  • D（ディスク休眠）が停滞 → 割り込み不可 I/O 待ち。NFS ハング、ディスク障害、カーネルドライバのバグが多い。SIGKILL でも殺せない\n  • 1プロセスの RSS が時間とともに増加 → メモリリーク\n  • スレッド数 1000 超 → スレッドリークか、高負荷時のスレッド・パー・コネクション設計\n  • UID 0 のプロセス → root 実行。予期しないものがあればセキュリティ懸念",
        ),

        // pressure
        ("pressure", "cpu_some_avg10") => (
            "CPU 圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクが CPU を待った時間の割合。25% 超は CPU 競合を示す。",
            "CPU PSI（Pressure Stall Information）— 10秒平均。\n\n'some' は少なくとも1つの実行可能タスクが CPU 時間を得られなかったことを意味する。負荷平均より正確 — 停滞時間を直接測定。\n\n💡 診断:\n  • 5% 未満 → 健全、CPU に余裕あり\n  • 5-25% → 中程度の圧力、一部タスクが待機\n  • 25% 超 → 顕著な CPU 競合、性能劣化中\n  • 50% 超 → 深刻 — 半分の時間、タスクが CPU を待っている",
        ),
        ("pressure", "memory_some_avg10") => (
            "メモリ圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクがメモリで停滞した時間の割合。非ゼロはメモリ圧迫またはスワッピングを示す。",
            "メモリ PSI — 10秒平均。\n\nCPU 圧力と違い、メモリ圧力は少しでもあれば注目すべき。メモリ回収やスワッピングが発生中ということ。\n\n💡 診断:\n  • 0% → メモリ圧力なし\n  • 0% 超 → キャッシュ回収かスワップ中。MemAvailable を確認\n  • 10% 超 → 顕著なメモリ圧力。メモリ回収でタスクが停滞\n  • 40% 超 → 危機的。スラッシング状態。性能が深刻に劣化",
        ),
        ("pressure", "io_some_avg10") => (
            "I/O 圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクが I/O で停滞した時間の割合。ディスクボトルネックを示す。",
            "I/O PSI — 10秒平均。\n\nタスクがディスク/ストレージ I/O 待ちでブロックされる頻度を測定。\n\n💡 診断:\n  • 5% 未満 → 正常な I/O 活動\n  • 5-20% → I/O がボトルネックになりつつある。diskstats の await 時間を確認\n  • 20% 超 → 顕著な I/O ボトルネック。SSD アップグレード、I/O スケジューラ調整、RAM 追加（ページキャッシュ拡大）を検討\n  • 負荷スパイクと相関 → I/O バウンドなワークロード。負荷平均は D 状態を含むので、高負荷＋高 I/O 圧力 = CPU ではなく I/O の問題",
        ),

        // uptime
        ("uptime", "uptime") => (
            "システム稼働時間",
            "前回起動からのシステム稼働時間。サスペンド/休止状態の時間もカーネルによっては含まれる。",
            "起動からのシステム稼働時間。\n\nカーネル開始からの壁時計時間。最近のカーネル（3.x+）ではサスペンド中もリセットされないが、動作はカーネルにより異なる。\n\n💡 診断:\n  • 本番サーバーで稼働時間が極端に短い → 予期しない再起動。`dmesg` と `/var/log/kern.log` でパニック/oops を確認。\n  • 極端に長い稼働時間（数ヶ月/数年）→ セキュリティパッチ未適用の可能性。メンテナンスウィンドウを計画すべき。",
        ),
        ("uptime", "idle") => (
            "全 CPU のアイドル時間合計",
            "起動以来の全 CPU のアイドル時間の累計。マルチコアでは各コアの合計なので uptime を超えることがある。",
            "全 CPU コアのアイドル時間の合計。\n\n4コアシステムで idle = 3 * uptime なら、平均して4コア中3コアがアイドルだったことを意味する。\n\n💡 診断: マルチコアで idle < uptime は通常あり得ない — シングルコア VM か、クロックソースの問題の可能性。",
        ),
        ("uptime", "idle_pct") => (
            "アイドル率 (idle / uptime)",
            "アイドル時間と稼働時間の比率。マルチコアでは (idle / uptime * 100) なので、ほぼアイドルなマシンでは 100% を超える。",
            "アイドル率 = idle_time / uptime * 100。\n\nアイドル時間は全 CPU の合計なので、マルチコアでは 100% を超える。4コアのアイドルシステムなら約 400%。\n\n💡 診断:\n  • idle_pct / CPU数 < 20% → システムは高負荷\n  • idle_pct / CPU数 > 80% → システムはほぼアイドル\n  • 時間とともに低下 → ワークロードが増加中",
        ),

        // version
        ("version", "raw") => (
            "カーネルバージョン文字列（完全）",
            "/proc/version の完全な出力。カーネルバージョン、ビルドホスト、コンパイラ、ビルド日時を含む。",
            "カーネルが報告する完全なバージョン文字列。\n\nカーネルバージョン、ビルドした人/ホスト、使用コンパイラ、ビルド日時が含まれる。\n\n💡 アップデート後に正しいカーネルが動作しているか確認したり、サポートチケットに正確なビルド情報を記載する際に有用。",
        ),
        ("version", "kernel_version") => (
            "カーネルバージョン番号",
            "Linux カーネルのバージョン（例: 6.6.87）。利用可能な機能、システムコール対応、ドライバ互換性を決定する。",
            "Linux カーネルバージョン（例: 6.6.87）。\n\nバージョン形式: major.minor.patch。minor で機能セットが決まり、patch にはバグ/セキュリティ修正が含まれる。\n\n💡 診断:\n  • CVE データベースと照合し、重要な脆弱性がパッチ済みか確認。\n  • カーネル 5.x 未満は PSI（圧力ストール情報）、io_uring 等の最新機能が未対応の場合がある。\n  • `uname -r` と比較 — 一致するはず。",
        ),
        ("version", "compiler") => (
            "カーネルビルド用コンパイラ",
            "カーネルのコンパイルに使用されたコンパイラ（通常 GCC）とバージョン。ABI 互換性のデバッグに関連。",
            "このカーネルをビルドしたコンパイラ。\n\n通常は GCC だが、一部ディストリビューションは Clang/LLVM を使用。コンパイラバージョンが重要な理由:\n  • ツリー外カーネルモジュール（NVIDIA ドライバ等）との ABI 互換性\n  • コンパイラ固有の最適化によるパフォーマンスへの影響\n\n💡 カーネルモジュールが 'version magic' エラーでロード失敗する場合、コンパイラ不一致が一般的な原因。",
        ),

        // cmdline
        ("cmdline", "cmdline") => (
            "カーネル起動パラメータ",
            "ブートローダー（GRUB、systemd-boot 等）からカーネルに渡されたコマンドライン。ハードウェア設定、セキュリティ機能、デバッグオプションを制御。",
            "ブートローダーからのカーネル起動コマンドライン。\n\n一般的な重要パラメータ:\n  • root= — ルートファイルシステムデバイス\n  • quiet/splash — 起動メッセージの抑制\n  • nomodeset — カーネルモード設定の無効化（GPU トラブルシューティング）\n  • mitigations=off — CPU 脆弱性緩和の無効化（危険だが高速化）\n  • crashkernel= — kdump 用予約メモリ\n\n💡 診断: パフォーマンスが予想外に低い場合、'mitigations=off' の欠如やオーバーヘッドを生むデバッグパラメータを確認。",
        ),
        ("cmdline", "param_count") => (
            "起動パラメータの数",
            "スペース区切りのカーネル起動パラメータの数。起動設定の複雑さの概要把握に有用。",
            "カーネルコマンドラインのスペース区切りトークン数。\n\n💡 param_count が非常に多い場合、カスタムカーネルチューニングやハードウェア問題の回避策が含まれている可能性。最小構成のシステムでは通常 5-15 個。",
        ),

        // stat
        ("stat", "cpu_user") => (
            "ユーザーモード CPU 時間",
            "起動以来の全 CPU のユーザー空間コード実行時間（jiffies 単位）。アプリケーションコードを含むが、カーネルシステムコールは含まない。",
            "起動以来の累積ユーザーモード CPU 時間（jiffies = 通常 1/100 秒）。\n\nユーザー時間はアプリケーションコードの実行全てを含む。`top` の 'us' 列に相当。\n\n💡 診断:\n  • user% が高く system% が低い → アプリケーションが CPU バウンド（計算集中型）\n  • cpu_system と比較: user >> system は計算ワークロードでは正常\n  • 絶対値ではなくスナップショット間の変化率を監視すべき",
        ),
        ("stat", "cpu_system") => (
            "カーネルモード CPU 時間",
            "起動以来の全 CPU のカーネルコード実行時間。システムコール、割り込み、カーネルスレッドを含む。",
            "起動以来の累積カーネルモード CPU 時間。\n\nシステム時間 = システムコール、割り込みハンドラ、カーネルスレッドでの時間。`top` の 'sy' 列に相当。\n\n💡 診断:\n  • system% が高い → 大量のシステムコール（I/O、コンテキストスイッチ、ネットワーク処理が多い）\n  • system > user → 異常。大量の小さい read() 等、バッファリングされない I/O が原因の可能性\n  • system% の突然のスパイク → 割り込みストームやカーネルドライバの問題を確認",
        ),
        ("stat", "cpu_idle") => (
            "CPU アイドル時間",
            "起動以来の全 CPU のアイドル時間。高い値は CPU に余裕があることを意味する。",
            "起動以来の累積 CPU アイドル時間。\n\nCPU が何もしておらず、実行可能なタスクもなかった時間。\n\n💡 診断:\n  • アイドルが常にほぼ 0 → CPU が飽和状態\n  • スナップショット間のアイドル率を比較: (idle_delta / total_delta * 100) でリアルタイムのアイドル% がわかる\n  • マルチコアでは合計値なので、1秒あたり最大アイドル = コア数 × 100 jiffies",
        ),
        ("stat", "cpu_iowait") => (
            "I/O 待ち CPU 時間",
            "CPU が未処理の I/O を待ってアイドルだった時間。高い iowait はストレージのボトルネックを示す。",
            "I/O 完了待ちの CPU 時間。\n\niowait は CPU がやることがなく、かつ未完了の I/O があったことを意味する。アイドルのサブセット — CPU はアイドルだがディスクでブロックされている。\n\n💡 診断:\n  • 高い iowait → CPU ではなくストレージがボトルネック\n  • 忙しいシステムでは iowait が低く見える場合がある（I/O 中に他のタスクが CPU を使うため）\n  • より正確な I/O ボトルネック信号は pressure の io_some_avg10 と比較\n  • スパイク → 大きなシーケンシャル読み書きやファイルシステムジャーナルのフラッシュ",
        ),
        ("stat", "cpu_usage_pct") => (
            "総合 CPU 使用率（累積）",
            "起動以来の CPU が有用な作業に費やした時間の割合。(total - idle - iowait) / total * 100 で計算。",
            "起動以来の累積 CPU 使用率。\n\n計算式: (busy_time / total_time * 100)、busy = user + nice + system + irq + softirq + steal。\n\n💡 注意: これは起動以来の累積平均であり、リアルタイムの使用率ではない。23時間アイドル後に1時間 100% 稼働したシステムではここに約 4% と表示される。",
        ),
        ("stat", "forks_total") => (
            "起動以来のフォーク（プロセス生成）回数",
            "起動以来 fork()/clone() が呼ばれた回数。高いレートは多数の短命プロセスを示す。",
            "起動以来の fork()/clone() 呼び出し回数。\n\nプロセスやスレッドの生成ごとにこのカウンタが増加する。\n\n💡 診断:\n  • 高いフォークレート（スナップショット間のデルタ）→ シェルスクリプトが多数のサブプロセスを生成、cron ジョブ、またはフォーク爆弾\n  • context_switches と比較: 高フォーク + 高コンテキストスイッチ = 多数の短命プロセス\n  • 緩やかな増加は正常。突然のスパイクは調査が必要",
        ),
        ("stat", "procs_running") => (
            "現在 CPU 上で実行中のプロセス数",
            "R（実行中/実行可能）状態のプロセス数。CPU 数を常に超えている場合、CPU が過負荷。",
            "現在 R（実行中）状態のプロセス数。\n\nCPU 上でアクティブに実行中か、CPU 時間待ちのランキューにいるプロセス。\n\n💡 診断:\n  • procs_running <= CPU数 → 正常、CPU が全ランナーを処理可能\n  • procs_running > CPU数 → タスクが CPU 時間待ちでキューイング中\n  • 持続的に高い → CPU ボトルネック、負荷平均と相関",
        ),
        ("stat", "procs_blocked") => (
            "I/O でブロック中のプロセス数",
            "D（割り込み不可スリープ）状態のプロセス数。I/O 待ちで、シグナルでも中断できない。",
            "D 状態（割り込み不可スリープ）のプロセス数。ディスク、NFS、デバイス I/O 待ち。\n\n💡 診断:\n  • procs_blocked > 0 が一時的 → I/O 操作中は正常\n  • procs_blocked が持続的に高い → I/O ボトルネック。diskstats と pressure を確認。\n  • D 状態プロセスが数分間スタック → NFS ハング、ディスク故障、カーネルドライバのバグ。SIGKILL でも殺せない。",
        ),
        ("stat", "context_switches") => (
            "起動以来のコンテキストスイッチ回数",
            "起動以来の CPU コンテキストスイッチ数。各スイッチでプロセスの状態を保存/復元する。非常に高いレートは過度なマルチタスクのオーバーヘッドを示す。",
            "起動以来の CPU コンテキストスイッチ合計。\n\nコンテキストスイッチは CPU がプロセス/スレッドを切り替える時に発生。自発的（ブロッキング I/O）と非自発的（プリエンプション）の両方がカウントされる。\n\n💡 診断:\n  • 正常レート: ワークロードにより 1000-50000/秒\n  • 100000/秒超 → 高い。多数のスレッドが競合するか過度な I/O 操作\n  • 高い system% CPU と相関 — 各スイッチにカーネルオーバーヘッドがある\n  • スナップショット間のデルタで現在のレートを把握",
        ),

        // cpuinfo
        ("cpuinfo", "logical_cpus") => (
            "論理 CPU 数（スレッド数）",
            "OS から見える論理プロセッサの合計。ハイパースレッディングを含む — 2つの論理 CPU が1つの物理コアを共有する場合がある。",
            "OS から見える論理 CPU（ハードウェアスレッド）の合計。\n\nハイパースレッディング/SMT 有効時: logical_cpus = physical_cores * 2（通常）。HT 無効時: logical_cpus = physical_cores。\n\n💡 診断:\n  • cores_per_socket と比較してハイパースレッディングを検出\n  • 負荷平均と比較する数はこの値\n  • 期待より少ない場合、BIOS 設定やカーネルパラメータ (maxcpus=, nr_cpus=) を確認",
        ),
        ("cpuinfo", "model") => (
            "CPU モデル名",
            "プロセッサが報告する完全な CPU モデル識別子。ブランド、世代、バリアント情報を含む。",
            "プロセッサの CPUID 命令から取得した CPU モデル名。\n\n例: 'Intel(R) Core(TM) i9-13900K'、'AMD EPYC 9654'。\n\n💡 用途:\n  • ハードウェア世代と期待されるパフォーマンスの特定\n  • 必要な命令セット（AVX-512 等）のサポート確認\n  • VM が正しい CPU モデルを公開しているか検証",
        ),
        ("cpuinfo", "frequency") => (
            "現在の CPU 周波数 (MHz)",
            "CPU の現在の動作周波数。周波数スケーリング（ターボブースト、省電力）により変動する場合がある。",
            "現在の CPU 周波数（MHz）。\n\n最近の CPU は負荷に応じて動的に周波数を調整する（P-state）。報告される値は:\n  • 軽負荷時のベース周波数（省電力）\n  • 高負荷時のターボ/ブースト周波数\n  • サーマルスロットリングで制限された周波数\n\n💡 診断:\n  • 定格より大幅に低い → ガバナーを確認: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`\n  • 'powersave' ガバナー → パフォーマンスが制限される。ベンチマークには 'performance' に切替。\n  • ターボに達しない → サーマルスロットリング。CPU 温度を確認。",
        ),
        ("cpuinfo", "cache_size") => (
            "CPU キャッシュサイズ (L2/L3)",
            "コアあたりの最終レベルキャッシュサイズ。大きいキャッシュはメモリ集中型ワークロードのパフォーマンスを向上させる。",
            "/proc/cpuinfo が報告する最終レベルキャッシュサイズ。\n\n通常はコア間で共有される L3 キャッシュ、またはアーキテクチャによってはコアごとの L2。\n\n💡 診断:\n  • 大きいキャッシュ = ワーキングセットが収まる場合のパフォーマンス向上\n  • データベースワークロードは大きな L3 キャッシュから大きな恩恵\n  • 詳細なキャッシュトポロジは `/sys/devices/system/cpu/cpu0/cache/` を確認",
        ),
        ("cpuinfo", "cores_per_socket") => (
            "ソケットあたりの物理コア数",
            "ソケットあたりの物理 CPU コア数。ハイパースレッディングでは各コアが 2 つの論理 CPU として表示される。",
            "ソケットあたりの物理 CPU コア。\n\n物理コアは独自の実行ユニットを持つ。ハイパースレッディングは実行リソースを共有して物理コアあたり 2 つの論理 CPU を作る。\n\n💡 診断:\n  • logical_cpus / cores_per_socket = 2 → ハイパースレッディング有効\n  • logical_cpus / cores_per_socket = 1 → ハイパースレッディング無効または非対応\n  • HPC/レイテンシ重視のワークロードでは、HT 無効化でパフォーマンスが向上する場合がある",
        ),
        ("cpuinfo", "key_flags") => (
            "主要 CPU 機能フラグ",
            "重要な CPU 機能フラグ: SSE/AVX（SIMD）、aes（暗号化）、vmx/svm（仮想化）、ht（ハイパースレッディング）、lm（64ビット）。",
            "/proc/cpuinfo からの主要 CPU 機能フラグ。\n\n注目すべきフラグ:\n  • sse/sse2/avx/avx2/avx512f → SIMD 命令セット（数値計算ワークロードに重要）\n  • aes → ハードウェア AES 暗号化（TLS パフォーマンスに重要）\n  • vmx (Intel) / svm (AMD) → ハードウェア仮想化サポート\n  • ht → ハイパースレッディング対応\n  • lm → ロングモード（64ビットサポート）\n  • nx → No-eXecute ビット（セキュリティ: データページの実行防止）\n  • hypervisor → VM 内で動作中\n\n💡 診断: 'hypervisor' フラグがある → これは VM。'vmx'/'svm' がない → ネストされた仮想化は利用不可。",
        ),

        // vmstat
        ("vmstat", "pgfault") => (
            "起動以来のページフォルト合計",
            "マイナー + メジャーページフォルトの合計。マイナーはメモリから解決、メジャーはディスク I/O が必要。",
            "起動以来のページフォルト合計（マイナー + メジャー）。\n\nマイナーフォルト: ページはメモリにあるがプロセスのページテーブルにマップされていない。即座に解決 — 正常で頻繁。\nメジャーフォルト: ページをディスクから読む必要がある。コストが高い。\n\n💡 診断: pgfault が高いのは正常。実際に I/O を発生させる pgmajfault に注目すべき。",
        ),
        ("vmstat", "pgmajfault") => (
            "メジャーページフォルト（ディスク I/O が必要）",
            "ディスクからの読み込みが必要だったページフォルト。各フォルトがプロセスを停滞させる。高いレートは RAM 不足やコールドキャッシュを示す。",
            "起動以来のメジャーページフォルト — 各フォルトでディスク I/O が必要。\n\nメジャーフォルトは要求されたページが RAM になく、ストレージから取得する必要があったことを意味する。フォルトしたプロセスは停滞する。\n\n💡 診断:\n  • pgmajfault レート（デルタ/秒）が高い → ワーキングセットに対して RAM 不足、またはコールドスタート\n  • 高い iowait と I/O 圧力と相関\n  • 起動直後はアプリケーション読み込みでメジャーフォルトが急増 — これは正常\n  • 持続的に高いレート → RAM 追加またはワーキングセットサイズの削減",
        ),
        ("vmstat", "pgpgin") => (
            "ディスクからページイン",
            "起動以来にブロックデバイスからメモリに読み込まれたページ数。通常のファイル I/O とデマンドページングを含む。",
            "ディスクからページインされたページ数（1KB 単位）。\n\nファイル I/O（read() システムコールによるページキャッシュ充填）とデマンドページング（実行可能ページの読み込み）の両方を含む。\n\n💡 pgpgin レートと pgpgout レートを比較: pgpgin >> pgpgout なら読み込み集中型ワークロード。",
        ),
        ("vmstat", "pgpgout") => (
            "ディスクへページアウト",
            "起動以来にメモリからブロックデバイスに書き込まれたページ数。ダーティページのライトバックとスワップアウトを含む。",
            "ディスクへページアウトされたページ数（1KB 単位）。\n\nダーティページキャッシュのライトバック（正常）とスワップのページアウト（メモリ圧迫）を含む。\n\n💡 診断: 通常のライトバックとスワップ圧力を区別するには、pswpout を別途確認。pgpgout が高く pswpout が低い = 通常のファイル書き込み。",
        ),
        ("vmstat", "pswpin") => (
            "スワップからページイン",
            "スワップからメモリに読み戻されたページ数。非ゼロは以前スワップアウトしたページを取り戻していることを意味する。",
            "スワップ領域からページインされたページ数。\n\nスワップインは、メモリ圧迫で以前スワップアウトされたページにプロセスがアクセスした時に発生。\n\n💡 診断:\n  • pswpin レート > 0 → スワップから積極的に読み込み中。パフォーマンスへの影響はスワップデバイスの速度に依存。\n  • 高 pswpin + 高 pswpout → スラッシング — ページが常にスワップイン/アウトされている。危機的状態。\n  • pswpin はあるが pswpout がない → 過去のメモリ圧迫からページを回復中。一時的かもしれない。",
        ),
        ("vmstat", "pswpout") => (
            "スワップへページアウト",
            "RAM からスワップに移されたページ数。アクティブなスワップアウトは現在のメモリ圧迫を示す。",
            "スワップ領域へページアウトされたページ数。\n\nスワップアウトはカーネルが RAM を確保する必要があり、回収可能なキャッシュを使い果たした時に発生。\n\n💡 診断:\n  • pswpout レート > 0 → 今まさにメモリ圧迫中\n  • 持続的に高い pswpout → システムにもっと RAM が必要\n  • 断続的な pswpout → 一時的なメモリスパイク、許容範囲の場合もある",
        ),
        ("vmstat", "nr_free_pages") => (
            "空きメモリページ数",
            "完全に空きのページ数。低い値は正常 — Linux は空きメモリをキャッシュに使う。",
            "未使用（空き）メモリページ数。\n\nmeminfo の MemFree と同様。Linux は空きメモリをページキャッシュに使うので、低い値は期待通り。\n\n💡 診断:\n  • 空きページが 'min' ウォーターマーク以下 → カーネルがダイレクトリクレームに入り、メモリ確保が停滞する可能性\n  • ゾーンごとの空き vs ウォーターマーク比較は zoneinfo を確認",
        ),
        ("vmstat", "nr_active_anon") => (
            "アクティブな匿名ページ数",
            "アクティブ LRU リスト上の匿名（ファイル非連動）ページ。最近アクセスされたヒープ/スタックページ。",
            "アクティブ LRU（Least Recently Used）リスト上の匿名ページ。\n\n匿名ページ = プロセスのヒープ、スタック、mmap(MAP_ANONYMOUS)。'アクティブ' は最近アクセスされたことを意味する。\n\n💡 これらのページはスワッピングでのみ解放できる。高い active_anon = プロセスが多くのヒープメモリを使用中。",
        ),
        ("vmstat", "nr_inactive_anon") => (
            "非アクティブな匿名ページ数",
            "最近アクセスされていない匿名ページ。メモリ圧迫時にスワップアウトの候補になる。",
            "非アクティブ LRU リスト上の匿名ページ — 最近アクセスされていない。\n\nメモリ圧迫が発生した時に最初にスワップアウトされる候補。\n\n💡 診断: 大きな inactive_anon でスワップ活動なし → 必要に応じて回収可能なメモリ。大きな inactive_anon でスワップが活発 → これらのページがスワップアウトされている。",
        ),
        ("vmstat", "nr_active_file") => (
            "アクティブなファイルページ数",
            "最近アクセスされたファイル連動ページ（ページキャッシュのホットページ）。再読み込みを高速化するファイル内容をキャッシュ。",
            "アクティブ LRU 上のファイル連動ページ — 最近使用されたページキャッシュ。\n\n最近読み取られたファイル内容をキャッシュし、ディスク読み取りを回避して I/O パフォーマンスを向上させる。\n\n💡 健全なシステムは大きなアクティブファイルキャッシュを持つ。負荷下で active_file が縮小するのは、メモリ圧迫でキャッシュが追い出されていることを意味する。",
        ),
        ("vmstat", "nr_inactive_file") => (
            "非アクティブなファイルページ数",
            "最近アクセスされていないファイル連動ページ。I/O なしで（ダーティでなければ）素早く回収可能。",
            "非アクティブ LRU 上のファイル連動ページ — 最近アクセスされていない。\n\nメモリ圧迫時に最初に回収される。回収コストは低い（ダーティでなければページを捨てるだけ）。\n\n💡 診断: 大きな inactive_file = メモリ圧迫に対する良いバッファ。小さな inactive_file = スワッピング開始前に回収できる余地が少ない。",
        ),
        ("vmstat", "nr_dirty") => (
            "ダーティページ（変更済み、未書き込み）",
            "メモリ上で変更されたがまだディスクに書き込まれていないページ。カーネルのライトバック機構で書き込まれる。",
            "まだディスクにフラッシュされていない変更済みページ。\n\nカーネルは定期的にダーティページをフラッシュする（/proc/sys/vm/dirty_writeback_centisecs で制御）。システムがクラッシュすると、ダーティページは失われる。\n\n💡 診断:\n  • 高い nr_dirty → 書き込み集中ワークロードまたは低速ストレージ\n  • nr_dirty が dirty_ratio を超える → write() がページフラッシュまでブロックされる（書き込みスロットリング）\n  • 持続的に高い → ストレージが書き込みレートに追いつけない",
        ),
        ("vmstat", "nr_writeback") => (
            "ディスクに書き込み中のページ",
            "現在ストレージにフラッシュ中のページ。高い値は激しい I/O 活動を示す。",
            "現在ストレージにライトバック中のページ。\n\nこれらのページはディスクへ転送中。数値はストレージ速度と書き込み量に依存。\n\n💡 診断:\n  • nr_writeback > 0 がほとんどの時間 → 常時書き込み圧力\n  • 非常に高い nr_writeback → ストレージデバイスが飽和、書き込みが滞留\n  • デバイスレベルの I/O メトリクスは diskstats を確認",
        ),
        ("vmstat", "nr_slab_reclaimable") => (
            "回収可能なスラブページ",
            "メモリ圧迫時に解放可能なカーネルスラブアロケータのページ。dentry キャッシュと inode キャッシュを含む。",
            "回収可能なスラブメモリ（カーネルキャッシュ）。\n\n主に dentry キャッシュ（ディレクトリエントリ）と inode キャッシュ。ファイルシステム操作を高速化し、メモリが必要な時に解放される。\n\n💡 診断: ファイルサーバーで回収可能スラブが非常に大きいのは正常 — 数百万のディレクトリエントリをキャッシュしている。カーネルは圧迫時に自動的に縮小する。",
        ),
        ("vmstat", "nr_slab_unreclaimable") => (
            "回収不可能なスラブページ",
            "解放できないカーネルスラブページ。メモリに保持し続ける必要があるアクティブなカーネルデータ構造。",
            "回収不可能なスラブメモリ — アクティブなカーネルオブジェクト。\n\nアクティブに使用中のカーネルデータ構造（タスク構造体、ネットワークバッファ等）で、解放できない。\n\n💡 診断:\n  • 回収不可能スラブが着実に増加 → カーネルメモリリークの可能性\n  • slabinfo でどの特定キャッシュが増加しているか確認\n  • 多数のネットワーク接続で回収不可能スラブが大きい → ネットワークバッファメモリ",
        ),
        ("vmstat", "oom_kill") => (
            "OOM Killer 発動回数",
            "起動以来 OOM（Out of Memory）Killer が発動した回数。各発動でメモリ解放のためプロセスが強制終了される。",
            "起動以来の OOM（Out of Memory）Killer 発動回数。\n\nOOM Killer はカーネルの最終手段 — 全メモリ（RAM + スワップ）が枯渇した時、生存のためにプロセスを強制終了する。\n\n💡 診断:\n  • 非ゼロの値は全て調査に値する\n  • `dmesg | grep -i oom` でどのプロセスが殺されたか、理由の詳細を確認\n  • OOM 防止: スワップ追加、RAM 増設、cgroups でメモリ制限を設定\n  • 重要プロセスの保護: `echo -1000 > /proc/<pid>/oom_score_adj`",
        ),
        ("vmstat", "nr_mapped") => (
            "ページテーブルにマップされたページ数",
            "少なくとも1つのプロセスの仮想アドレス空間にマップされているページ数。ファイル連動と匿名の両方のマップページを含む。",
            "プロセスのページテーブルにマップされたページ数。\n\nmmap されたファイルや共有ライブラリなど、プロセスの仮想メモリマッピングを通じてアクティブに参照されているページ。\n\n💡 診断:\n  • 総メモリに対して nr_mapped が高い → 多くのプロセスがライブラリやmmap ファイルを共有\n  • 着実に増加 → メモリマップファイルのリークや共有メモリ使用量の増加の可能性\n  • nr_anon_pages + nr_file_pages と比較して全体像を把握",
        ),
        ("vmstat", "nr_shmem") => (
            "共有メモリページ数（tmpfs, shmem）",
            "共有メモリセグメント、tmpfs ファイルシステム、POSIX 共有メモリで使用されるページ数。meminfo の Cached に含まれる。",
            "共有メモリページ — tmpfs、POSIX shmem、SysV 共有メモリ。\n\nこれらのページは meminfo の Cached に含まれるが、通常のページキャッシュのように回収可能ではない — 明示的に解放されるまで持続する。\n\n💡 診断:\n  • 高い nr_shmem → tmpfs マウントを確認（`df -h /dev/shm`、tmpfs の場合 `/tmp`）\n  • 主な消費者: データベース（PostgreSQL 共有バッファ）、Web ブラウザ、Docker オーバーレイ\n  • 通常のキャッシュと異なり、shmem は MemAvailable にカウントされる",
        ),
        ("vmstat", "nr_anon_pages") => (
            "匿名ページの合計",
            "プロセスのヒープ、スタック、プライベート匿名マッピングに割り当てられたページ数。プロセスの終了またはスワッピングでのみ解放される。",
            "使用中の匿名（ファイル非連動）ページの合計。\n\n匿名ページはプロセスのプライベートデータを保持: ヒープ（malloc）、スタック、mmap(MAP_ANONYMOUS|MAP_PRIVATE)。\n\n💡 診断:\n  • nr_anon_pages が時間とともに増加 → 1つ以上のプロセスにメモリリークの可能性\n  • 大きな nr_anon_pages + 低い MemAvailable → プロセスが RAM の大部分を消費\n  • 犯人を見つける: `ps aux --sort=-rss | head` または /proc/<pid>/smaps_rollup を確認",
        ),
        ("vmstat", "allocstall_normal") => (
            "Normal ゾーンのメモリ確保停滞回数",
            "Normal ゾーンからのメモリ確保でプロセスが停滞した回数。ダイレクトリクレームが発動し、レイテンシスパイクを引き起こす。",
            "Normal メモリゾーンでのメモリ確保停滞回数。\n\n空きページが low ウォーターマーク以下に低下すると、新規メモリ確保でダイレクトリクレームが発動 — 確保しようとするプロセスがカーネルのページ解放を待つ必要がある。これがレイテンシスパイクを引き起こす。\n\n💡 診断:\n  • allocstall が増加 → システムがメモリ圧迫下にある\n  • アプリケーションの高レイテンシと相関\n  • 高い停滞率 → kswapd が追いつけない。RAM の追加を検討\n  • pgscan_direct と pgsteal_direct も併せて確認して全体像を把握",
        ),

        // buddyinfo
        ("buddyinfo", "zone_count") => (
            "メモリゾーン数",
            "バディアロケータのメモリゾーン数。一般的なゾーン: DMA、DMA32、Normal、オプションで Movable。",
            "バディアロケータが追跡するメモリゾーン数。\n\n典型的なゾーン:\n  • DMA — 最初の 16MB、レガシー ISA デバイス用\n  • DMA32 — 最初の 4GB、32ビット DMA デバイス用\n  • Normal — メインメモリゾーン\n  • Movable — 移行可能なページ（メモリホットプラグ用）\n\n💡 NUMA システムでは各ノードにゾーンセットがあるため、ゾーンが増える。",
        ),
        ("buddyinfo", "zones") => (
            "ゾーン・オーダー別の空きページチャンク",
            "メモリ断片化データ: 各ゾーンで各オーダー（0-10）の空きチャンク数を表示。オーダー N = 2^N 連続ページ（各 4KB）。",
            "バディアロケータのゾーン・オーダー別空きページ数。\n\nオーダー 0 = 4KB、オーダー 1 = 8KB、... オーダー 10 = 4MB。高いオーダーほど大きな連続空きブロック。\n\n💡 診断:\n  • オーダー 0 のページは多いが高次オーダーがゼロ → メモリが断片化。大きなメモリ確保が失敗するかコンパクションが必要。\n  • 全オーダーでゼロ → ゾーンが枯渇\n  • ヒュージページに重要: 2MB ヒュージページにはオーダー 9 が必要（x86）。オーダー 9 が 0 なら透過的ヒュージページは失敗する。\n  • `echo 1 > /proc/sys/vm/compact_memory` でコンパクションを実行可能。",
        ),

        // zoneinfo
        ("zoneinfo", "zone_count") => (
            "メモリゾーン数",
            "詳細なウォーターマーク情報を持つメモリゾーン数。各ゾーンに回収動作を制御する min/low/high ウォーターマークがある。",
            "zoneinfo のメモリゾーン数。\n\n各ゾーンにはページ回収を制御するウォーターマークがある:\n  • min — これ以下ではメモリ確保が停滞（ダイレクトリクレーム）\n  • low — kswapd がバックグラウンド回収を開始\n  • high — kswapd が回収を停止\n\n💡 複数ノードの NUMA システムではゾーンが増える。",
        ),
        ("zoneinfo", "zones") => (
            "ゾーン別メモリ詳細（free, min, low, high）",
            "メモリゾーンのテーブル。空きページ数とウォーターマーク閾値を表示。free が 'low' を下回るとバックグラウンド回収が開始。",
            "ゾーン別メモリ詳細情報。\n\n列: ゾーン識別子、空きページ、min ウォーターマーク、low ウォーターマーク、high ウォーターマーク。\n\n💡 診断:\n  • free < min → ダイレクトリクレームが活発。メモリ確保が停滞 — プロセスがブロックされる可能性。\n  • free < low → kswapd 実行中（バックグラウンド回収）。中程度の圧迫下では正常。\n  • free > high → このゾーンにメモリ圧迫なし。\n  • 各ゾーンを個別に確認 — メモリ圧迫はゾーン固有の場合がある（例: Normal は余裕だが DMA32 が枯渇）。",
        ),

        // slabinfo
        ("slabinfo", "cache_count") => (
            "スラブキャッシュ数",
            "カーネルスラブアロケータのキャッシュ総数。各キャッシュは特定の種類のカーネルオブジェクト（inode、dentry、バッファ等）を提供。",
            "カーネルのアクティブなスラブキャッシュ数。\n\nスラブアロケータは固定サイズのカーネルオブジェクトの効率的な割り当てを提供する。各キャッシュは同じ型のオブジェクトをプール。\n\n💡 診断: キャッシュ数が非常に多い場合、多数のカーネルモジュールがロードされ、それぞれ独自のキャッシュを登録している可能性。",
        ),
        ("slabinfo", "caches") => (
            "スラブキャッシュ詳細",
            "スラブキャッシュのテーブル: 名前、アクティブオブジェクト数、総オブジェクト数、オブジェクトサイズ、スラブあたりオブジェクト数、スラブあたりページ数。",
            "スラブキャッシュの詳細統計。\n\n列: キャッシュ名、アクティブオブジェクト数、総割り当てオブジェクト数、オブジェクトサイズ（バイト）、スラブページあたりオブジェクト数、スラブあたりページ数。\n\n💡 診断:\n  • dentry キャッシュが非常に大きい → 多数のファイルシステムパスをキャッシュ中。ファイルサーバーでは正常。\n  • inode_cache が増加 → 多数のユニークファイルにアクセス中。\n  • active_objs << num_objs → メモリの無駄。多数の事前割り当てだが未使用のオブジェクト。\n  • 不明なキャッシュが増加 → カーネルモジュールのメモリリークの可能性。\n  • スラブキャッシュの回収: `echo 2 > /proc/sys/vm/drop_caches`（dentry+inode のみ）。",
        ),

        // pagetypeinfo
        ("pagetypeinfo", "entry_count") => (
            "ページタイプ情報エントリ数",
            "ページタイプ内訳のエントリ数。各エントリはゾーンごとの移行タイプ別空きページ数を表示。",
            "ページタイプ情報のエントリ数。\n\nエントリは NUMA ノード、メモリゾーン、移行タイプ（Unmovable、Movable、Reclaimable 等）別に分類される。\n\n💡 移行タイプはデフラグに影響: Movable ページは連続ブロックを作るために再配置可能。",
        ),
        ("pagetypeinfo", "entries") => (
            "ゾーン別ページ割り当てタイプ内訳",
            "移行タイプ（Unmovable、Movable、Reclaimable）別、ゾーン別、オーダー別の空きページ数。コンパクション用のページ分類を表示。",
            "詳細なページ割り当てタイプ情報。\n\n各オーダーの空きページが移行タイプ別にどう分布しているかを表示:\n  • Unmovable — 再配置不可（カーネルの割り当て）\n  • Movable — コンパクション用に移行可能（ユーザーページ）\n  • Reclaimable — 解放可能（ページキャッシュ、スラブ）\n\n💡 診断:\n  • Unmovable の断片が多い → コンパクション困難。断片化が持続する。\n  • Movable ページが支配的 → 透過的ヒュージページとコンパクションに良い。\n  • buddyinfo と合わせて断片化の原因を理解。",
        ),

        // swaps
        ("swaps", "total_size") => (
            "利用可能なスワップ領域合計",
            "全スワップ領域（ファイルとパーティション）の合計サイズ。RAM のオーバーフロー先の最大容量。",
            "全スワップ領域の合計スワップ容量。\n\nスワップは物理 RAM を超えて仮想メモリを拡張する。最近アクセスされていないページがスワップに移動され、RAM を解放する。\n\n💡 診断:\n  • 0 バイト → スワップ未設定。RAM が満杯になったら OOM しかない。\n  • 推奨: 大容量 RAM のシステムでも安全策として最低 1-2GB のスワップ。",
        ),
        ("swaps", "total_used") => (
            "使用中のスワップ領域",
            "現在使用中のスワップ量。多少の使用は正常。絶対値よりトレンドを監視すべき。",
            "現在使用中のスワップ容量。\n\n多少のスワップ使用は正常で問題を示さない — カーネルがアイドルページを先行してスワップアウトした可能性がある。\n\n💡 診断:\n  • 使用スワップが安定 → 以前スワップアウトされたページがそのまま。正常。\n  • 使用スワップが増加 → アクティブなメモリ圧迫。MemAvailable を確認。\n  • 使用スワップが合計に近い → 危険ゾーン。次のメモリ確保で OOM が発動する可能性。\n  • どのプロセスがスワップを使用しているか: `grep VmSwap /proc/*/status | sort -k2 -n`",
        ),
        ("swaps", "usage_pct") => (
            "スワップ使用率",
            "スワップ合計に対する使用中の割合。50% 以下なら通常問題なし。80% 超は調査が必要。",
            "スワップ合計に対する使用率。\n\n💡 診断:\n  • 0% → スワップ未使用（またはスワップ未設定）\n  • 50% 未満 → 正常、特に安定していれば\n  • 50-80% → 高め。トレンドを監視。\n  • 80% 超 → 高い。メモリ需要が増えると OOM の可能性。\n  • 100% → スワップ満杯。新たなメモリ需要は OOM Killer を発動させる。",
        ),
        ("swaps", "swap_areas") => (
            "個別スワップ領域の詳細",
            "スワップ領域のテーブル: ファイル名/デバイス、タイプ（パーティション/ファイル）、サイズ、使用量、優先度。高い優先度のものから先に使用される。",
            "個別スワップ領域の詳細。\n\n列: デバイス/ファイルパス、タイプ（パーティションまたはファイル）、サイズ、使用量、優先度。\n\n💡 診断:\n  • 優先度が使用順序を決定 — 高い優先度のものから先に使用される\n  • 複数のスワップ領域が同じ優先度 → ラウンドロビン（ストライプ）で分散、パフォーマンス向上\n  • SSD 上のスワップ → 許容範囲のパフォーマンス。HDD 上のスワップ → 圧迫時に大きなレイテンシ。\n  • スワップファイル vs パーティション → ファイルはやや遅いがリサイズが容易",
        ),

        // df (ディスク使用状況)
        ("df", "filesystems") => (
            "ファイルシステム使用状況テーブル",
            "マウント済みファイルシステムのテーブル: デバイス、マウントポイント、合計、使用量、空き、使用率%。疑似ファイルシステム (tmpfs, proc 等) は除外。",
            "実ファイルシステムの使用状況テーブル。\n\n列: デバイス、マウントポイント、合計、使用量、空き、使用率%。\n\n💡 診断:\n  • 使用率 > 90% → 危険。ログ書き込み失敗、DB クラッシュの可能性。\n  • 使用率 > 80% → 容量計画を。ログローテーション、不要ファイル削除。\n  • 「空き」は非 root ユーザーが使える容量（予約ブロックを考慮）。",
        ),
        ("df", "root_use_pct") => (
            "ルートファイルシステム使用率 %",
            "ルート (/) ファイルシステムの使用率。最も重要なファイルシステム — 満杯になるとシステムが応答不能になる可能性。",
            "ルートファイルシステムの使用率。\n\n💡 診断:\n  • > 90% → 危険: 即座に対処。du -sh /* で大きなディレクトリを特定。\n  • > 80% → 警告: 整理を計画。journalctl --vacuum-size=500M, docker system prune。\n  • 増加傾向 → ログリークの可能性。/var/log のサイズを確認。",
        ),

        // thermal (温度)
        ("thermal", "max_temp") => (
            "最高CPU/GPU温度",
            "全サーマルゾーンの最高温度。75°C以上は熱ストレス、90°C以上はスロットリング発動。",
            "全サーマルゾーンの最高温度。\n\n💡 診断:\n  • > 90°C → 危険: サーマルスロットリング発動。CPU周波数低下。ファンを確認。\n  • > 75°C → 警告: 高温状態。負荷が続くとさらに上昇。\n  • < 50°C → 多くのシステムで正常なアイドル温度。",
        ),

        // file-nr (ファイルディスクリプタ)
        ("file-nr", "fd_allocated") => (
            "割り当て済みファイルディスクリプタ数",
            "カーネルが現在割り当てているファイルハンドル数。使用中のものと未使用（キャッシュ）のものを含む。",
            "カーネルが現在割り当てているファイルハンドル数。\n\n未使用のものはフリーリストに保持され再利用される。\n\n💡 診断: fd_allocated が fd_max に近い場合、新しいファイルやソケットが開けなくなる。lsof で FD リークを確認。",
        ),
        ("file-nr", "fd_usage_pct") => (
            "ファイルディスクリプタ使用率 %",
            "システム最大値に対するFD使用率。(allocated - unused) / max * 100。",
            "ファイルディスクリプタ使用率: (allocated - unused) / max * 100。\n\n💡 診断:\n  • > 80% → 警告: FD枯渇の危険。ファイルやソケットが開けなくなる可能性。\n  • リーク元の特定: lsof -p <PID> | wc -l で疑わしいプロセスを調査。\n  • 上限引き上げ: sysctl -w fs.file-max=<大きい値>。",
        ),

        // ── グループ 3: ネットワーク ──────────────────────────────────

        // net/dev
        ("net/dev", "total_rx") => (
            "全インターフェースの受信バイト合計。",
            "起動以降の全ネットワークインターフェースの累積受信バイト数。ループバックトラフィックを含む。",
            "全インターフェースの受信バイト合計。\n\nこのカウンタは再起動でリセットされる。2つのスナップショットを比較してスループットを算出。ループバック（lo）を含むため、外部通信のみ必要な場合は lo を差し引く。\n\n💡 診断: total_rx が total_tx より大幅に速く増加 → ホストはコンシューマ（ダウンロード、DB 読み取り）。逆パターンはサーバー役割（Web サーバー、NFS エクスポート）を示唆。",
        ),
        ("net/dev", "total_tx") => (
            "全インターフェースの送信バイト合計。",
            "起動以降の全ネットワークインターフェースの累積送信バイト数。ループバックトラフィックを含む。",
            "全インターフェースの送信バイト合計。\n\ntotal_rx と同様だが送信側。TX が多く RX が少ない場合、コンテンツ配信ワークロードを示す。\n\n💡 診断: アプリケーション変更なしに TX が急増 → 侵害されたホストがデータを流出、または DDoS 増幅攻撃に参加している可能性。",
        ),
        ("net/dev", "interface_count") => (
            "ネットワークインターフェース数。",
            "/proc/net/dev に見える全ネットワークインターフェース数。ループバック、仮想、物理インターフェースを含む。",
            "ネットワークインターフェースの総数。\n\nlo（ループバック）、物理 NIC、ブリッジ、veth ペア（コンテナ）、tun/tap デバイス（VPN）、ボンドインターフェースを含む。\n\n💡 診断: 予想外に多い場合、コンテナの増殖を確認（各コンテナが veth ペアを追加）。予想外に少ない場合、NIC ドライバのロード失敗 — `dmesg | grep -i eth` を確認。",
        ),
        ("net/dev", "interfaces") => (
            "インターフェース別トラフィック統計。",
            "全ネットワークインターフェースのテーブル。名前、受信バイト数、受信パケット数、送信バイト数、送信パケット数を表示。",
            "インターフェース別ネットワークトラフィック内訳。\n\nカラム: 名前、RX バイト、RX パケット、TX バイト、TX パケット。\n\n💡 診断:\n  • RX/TX エラーが多い → ケーブル問題、デュプレックス不一致、またはドライバのバグ\n  • トラフィックゼロのインターフェース → リンクダウンの可能性; `ip link show` で確認\n  • lo のトラフィックが多い → localhost 上のプロセス間通信が活発（DB では一般的）\n  • パケット数多いがバイト数少ない → 小さなパケットが大量、SYN フラッドやおしゃべりなプロトコルの可能性",
        ),
        ("net/udp", "socket_count") => (
            "UDP ソケット数。",
            "システム上の全 UDP ソケット数。TCP と異なり UDP はコネクションレスで、各エントリはバインド済みソケットを表す。",
            "UDP ソケットの総数。\n\nUDP ソケットには TCP のような接続状態がない。各エントリはローカルポートにバインドされたソケット。\n\n💡 診断:\n  • 多数のローカルポートで高カウント → DNS 増幅や UDP スキャンの可能性\n  • 一般的な正当な UDP 利用: DNS (ポート 53)、NTP (123)、SNMP (161)、syslog (514)\n  • socket_count が増加し続ける → UDP アプリケーションの FD リークの可能性",
        ),
        ("net/udp", "sockets") => (
            "UDP ソケット一覧。",
            "全 UDP ソケットのテーブル。ローカルアドレス、リモートアドレス、状態、所有プロセスの UID を表示。",
            "システム上の全 UDP ソケット。\n\nカラム: local_addr、remote_addr、state、uid。\n\n💡 診断:\n  • リモートアドレス 0.0.0.0:0 → リスニング中（特定のピアに接続していない）\n  • 同じポートに多数のソケット → 複数プロセスまたは SO_REUSEPORT\n  • UID 0 のソケット → root で実行中; 想定通りか確認\n  • UDP には信頼性保証がない — パケットロスはこのレベルでは見えない。net/snmp の Udp_InErrors でドロップ検出。",
        ),
        ("net/unix", "socket_count") => (
            "Unix ドメインソケット数。",
            "全 Unix ドメインソケットの数。同一ホスト上の高速プロセス間通信に使用。",
            "Unix ドメインソケットの総数。\n\nUnix ソケットはローカル通信の優先 IPC メカニズム（TCP ループバックより高速）。DB、ディスプレイサーバー、systemd が多用。\n\n💡 診断:\n  • 高カウントは systemd ベースの現代システムでは正常（200+ は一般的）\n  • 継続的に増加 → ソケットリークの可能性\n  • パスが空のソケット → 抽象名前空間ソケット（@ プレフィックス付き）",
        ),
        ("net/unix", "sockets") => (
            "Unix ドメインソケット詳細。",
            "全 Unix ドメインソケットのテーブル。参照カウント、タイプ、状態、inode、パスを表示。",
            "全 Unix ドメインソケット。\n\nカラム: refcount、type、state、inode、path。\n\n💡 診断:\n  • Type 1 = STREAM（TCP 的）、Type 2 = DGRAM（UDP 的）、Type 5 = SEQPACKET\n  • 既知のパスのソケット: /var/run/dbus/system_bus_socket（D-Bus）、/var/run/docker.sock（Docker）\n  • 1つのソケットに高い refcount → 多数のプロセスが共有（D-Bus では正常）",
        ),
        ("net/arp", "entry_count") => (
            "ARP テーブルエントリ数。",
            "カーネル ARP キャッシュの IP-MAC アドレスマッピング数。各エントリはローカルネットワーク上の最近通信した隣接ホストを表す。",
            "ARP キャッシュエントリ数。\n\nARP テーブルは同一 L2 ネットワーク上のホストの IPv4 アドレスを MAC アドレスにマッピング。\n\n💡 診断:\n  • 非常に多い（1000+）→ 大規模フラットネットワーク、または ARP ポイズニング/スキャン\n  • 増加し続ける → ARP ストームまたはネットワークスキャンの可能性",
        ),
        ("net/arp", "entries") => (
            "ARP テーブルエントリ。",
            "ARP キャッシュエントリのテーブル。IP アドレス、MAC アドレス、フラグ、ネットワークデバイスを表示。",
            "ARP キャッシュの内容。\n\nカラム: ip、hw_addr、flags、device。\n\n💡 診断:\n  • Flags 0x2 = 完了（解決済み）、0x6 = 完了+永続（静的エントリ）\n  • 異なる IP に同じ MAC → ARP スプーフィングまたはネットワーク設定ミス\n  • 00:00:00:00:00:00 MAC → 未解決エントリ; L2 でホストに到達不可",
        ),
        ("net/route", "route_count") => (
            "ルーティングテーブルエントリ数。",
            "カーネル IPv4 ルーティングテーブルのエントリ数。デフォルトゲートウェイ、直接接続ネットワーク、静的ルートを含む。",
            "IPv4 ルーティングテーブルエントリ数。\n\n💡 診断:\n  • 0 ルート → ネットワーク未設定; ホストが孤立\n  • デフォルトルート（0.0.0.0 宛先）がない → 非ローカルネットワークに到達不可\n  • 非常に多い → 複雑なルーティング設定、またはダイナミックルーティングプロトコルがルートを注入",
        ),
        ("net/route", "routes") => (
            "カーネルルーティングテーブル。",
            "ルーティングエントリのテーブル。インターフェース、宛先、ゲートウェイ、マスク、フラグ、メトリックを表示。",
            "IPv4 カーネルルーティングテーブル。\n\nカラム: iface、destination、gateway、mask、flags、metric。\n\n💡 診断:\n  • 宛先 0.0.0.0 でマスク 0.0.0.0 → デフォルトルート\n  • ゲートウェイ 0.0.0.0 → 直接接続ネットワーク\n  • フラグ: U=アップ、G=ゲートウェイ、H=ホストルート",
        ),
        ("net/sockstat", "sockets_used") => (
            "使用中のソケット総数。",
            "カーネルが割り当てた全種類のソケット数。ネットワーク活動の全体的な指標。",
            "全プロトコルの割り当て済みソケット総数。\n\n💡 診断:\n  • 継続的に増加 → ソケット/FD リークの可能性\n  • システム全体の制限: /proc/sys/fs/file-max",
        ),
        ("net/sockstat", "TCP_inuse") => (
            "使用中の TCP ソケット数。",
            "現在使用中の TCP ソケット数（TIME_WAIT 以外の全状態）。",
            "現在使用中の TCP ソケット。\n\n💡 診断:\n  • TCP_tw と比較 — tw >> inuse なら短命な接続が多い\n  • 急増はトラフィックスパイクまたは接続リーク",
        ),
        ("net/sockstat", "TCP_orphan") => (
            "孤立 TCP ソケット数。",
            "どのプロセスにも属さない TCP ソケット。カーネルメモリを消費しクリーンアップ待ち。",
            "孤立 TCP 接続。\n\n💡 診断:\n  • 高い孤立数 → アプリクラッシュまたは不正終了\n  • 制限: /proc/sys/net/ipv4/tcp_max_orphans",
        ),
        ("net/sockstat", "TCP_tw") => (
            "TIME_WAIT の TCP ソケット数。",
            "TIME_WAIT 状態の TCP 接続。遅延パケット待機中。忙しい HTTP サーバーでは正常。",
            "TCP TIME_WAIT ソケット数。\n\n💡 診断:\n  • 5000 未満 → 正常\n  • 30000 超 → エフェメラルポート枯渇の可能性\n  • tcp_tw_reuse を有効化して再利用を許可",
        ),
        ("net/sockstat", "TCP_alloc") => (
            "割り当て済み TCP ソケット数。",
            "カーネルが割り当てた TCP ソケットの総数。全状態を含む。",
            "割り当て済み TCP ソケット総数。\n\n💡 診断:\n  • alloc >> inuse + tw → 遷移状態のソケットが多い\n  • TCP メモリ制限: /proc/sys/net/ipv4/tcp_mem",
        ),
        ("net/sockstat", "TCP_mem") => (
            "TCP メモリ使用量（ページ）。",
            "全 TCP ソケットが消費するカーネルメモリページ数。各ページは通常 4KB。",
            "TCP メモリ消費量（カーネルページ単位）。\n\n💡 診断:\n  • /proc/sys/net/ipv4/tcp_mem のしきい値と比較\n  • 'high' 超過 → 接続ドロップ",
        ),
        ("net/sockstat", "UDP_inuse") => (
            "使用中の UDP ソケット数。",
            "現在使用中の UDP ソケット数。DNS、NTP、ロギング等を含む。",
            "アクティブな UDP ソケット。\n\n💡 診断:\n  • 一般的な値: 静かなサーバーで 5-20\n  • UDP には輻輳制御がない",
        ),
        ("net/sockstat", "UDP_mem") => (
            "UDP メモリ使用量（ページ）。",
            "UDP ソケットが消費するカーネルメモリページ数。",
            "UDP メモリ消費量。\n\n💡 診断:\n  • メモリ圧迫下では受信 UDP パケットを無言でドロップ\n  • net/snmp の Udp_RcvbufErrors を確認",
        ),
        ("net/sockstat", "FRAG_inuse") => (
            "IP フラグメント再構成エントリ数。",
            "非ゼロ値はフラグメント化されたパケットの受信を示す。",
            "IP フラグメント再構成キューエントリ。\n\n💡 診断:\n  • 現代のネットワークでは通常 0\n  • 非ゼロ → MTU 不一致か PMTUD がブロック",
        ),
        ("net/snmp", "Tcp_ActiveOpens") => (
            "TCP 接続開始数（クライアント側）。",
            "このホストが初期 SYN を送信した TCP 接続の累積数。",
            "TCP アクティブオープン。\n\n💡 診断:\n  • Tcp_PassiveOpens と比較してクライアント/サーバー役割を判断\n  • Tcp_AttemptFails / Tcp_ActiveOpens = 接続失敗率",
        ),
        ("net/snmp", "Tcp_PassiveOpens") => (
            "TCP 接続受付数（サーバー側）。",
            "listen/accept で受け付けた TCP 接続の累積数。",
            "TCP パッシブオープン。\n\n💡 診断:\n  • PassiveOpens >> ActiveOpens → 主にサーバー役割\n  • レートの突然の低下 → クライアントがサービスに到達不可",
        ),
        ("net/snmp", "Tcp_RetransSegs") => (
            "TCP セグメント再送数。",
            "再送された TCP セグメントの累積数。パケットロスを示す。",
            "TCP 再送カウンタ。\n\n💡 診断:\n  • RetransSegs / OutSegs = 再送率\n  • 1% 超 → 深刻なパケットロス",
        ),
        ("net/snmp", "Tcp_InErrs") => (
            "エラーのある TCP セグメント受信数。",
            "チェックサムエラー等の TCP セグメント数。データ破損を示す。",
            "TCP 入力エラー。\n\n💡 診断:\n  • 0 に近いはず\n  • 非ゼロ → NIC 障害、不良ケーブル、またはドライバのバグ",
        ),
        ("net/snmp", "Udp_InErrors") => (
            "配信できなかった UDP データグラム数。",
            "配信できなかった受信 UDP データグラムの累積数。",
            "UDP 入力エラー。\n\n💡 診断:\n  • 一般的な原因: リスンするプロセスなし、受信バッファオーバーフロー\n  • Udp_RcvbufErrors でバッファオーバーフローを確認",
        ),
        ("net/snmp", "Ip_InReceives") => (
            "受信 IP データグラム総数。",
            "全受信 IP データグラムの累積数。最上位入力カウンタ。",
            "IP 入力データグラム総数。\n\n💡 診断:\n  • 変化率がネットワーク入力負荷を示す\n  • ルーターでなければ InReceives ≈ InDelivers",
        ),
        ("net/snmp", "Ip_OutRequests") => (
            "送信 IP データグラム総数。",
            "送信のために渡された全 IP データグラムの累積数。",
            "IP 出力データグラム総数。\n\n💡 診断:\n  • OutRequests >> InReceives → サーバー役割\n  • 突然のスパイク → 新ワークロードか侵害の可能性",
        ),
        ("net/netstat", "TcpExt_ListenOverflows") => (
            "リスンキューオーバーフロー回数。",
            "リスンバックログキューが満杯だった回数。",
            "TCP リスンキューオーバーフロー。\n\n💡 診断:\n  • 通常は 0 であるべき\n  • 非ゼロ → net.core.somaxconn を増加",
        ),
        ("net/netstat", "TcpExt_ListenDrops") => (
            "リスンキューからのドロップ数。",
            "リスンキュー満杯で接続がドロップされた数。",
            "TCP リスンキュードロップ。\n\n💡 診断:\n  • 非ゼロ = クライアントが拒否されている\n  • レート（ドロップ/秒）を監視",
        ),
        ("net/netstat", "TcpExt_TCPTimeouts") => (
            "TCP 接続タイムアウト数。",
            "応答待ちでタイムアウトした TCP 接続数。",
            "TCP タイムアウトイベント。\n\n💡 診断:\n  • 高タイムアウト + 高再送 → 永続的なネットワーク経路障害\n  • 時間帯との相関を確認（輻輳パターン）",
        ),
        ("net/wireless", "interface_count") => (
            "無線インターフェース数。",
            "カーネルが検出した無線ネットワークインターフェースの数。",
            "無線インターフェース数。\n\n💡 診断:\n  • 0 → 無線ハードウェア未検出またはドライバ未ロード\n  • 通常は Wi-Fi 付きラップトップで 1",
        ),
        ("net/wireless", "interfaces") => (
            "無線インターフェース統計。",
            "無線インターフェースのテーブル。状態、リンク品質、信号レベル、ノイズレベル。",
            "インターフェース別無線統計。\n\nカラム: iface、status、link quality、signal level（dBm）、noise level（dBm）。\n\n💡 診断:\n  • 信号レベル: -30=優秀、-67=良好、-70=普通、-80=弱い、-90=使用不可\n  • SNR > 25 dB → 良好; < 15 dB → 不良",
        ),

        // ── グループ 4: ストレージ ────────────────────────────────────
        ("mounts", "count") => (
            "マウント済みファイルシステム数。",
            "全マウント済みファイルシステムの数。仮想ファイルシステムを含む。",
            "マウント済みファイルシステムの総数。\n\n💡 診断:\n  • 一般的な Linux: 30-60 マウント\n  • 200+ → コンテナホストまたは NFS 多用環境",
        ),
        ("mounts", "mounts") => (
            "マウント済みファイルシステム詳細。",
            "デバイス、マウントポイント、ファイルシステムタイプ、マウントオプションを表示。",
            "現在マウントされている全ファイルシステム。\n\nカラム: device、mountpoint、fstype、options。\n\n💡 診断:\n  • 'ro' オプション → 読み取り専用に再マウント、ディスクエラーの可能性\n  • NFS 'hard' → サーバー到達不能でハング\n  • tmpfs → RAM ベース; サイズ制限を確認",
        ),
        ("partitions", "count") => (
            "ブロックデバイスパーティション数。",
            "カーネルが認識した全パーティション数。ディスク全体とサブパーティションを含む。",
            "ブロックデバイスパーティション数。\n\n💡 診断:\n  • 期待される全ディスクが表示されているか確認\n  • ディスクが見えない → ハードウェア障害またはドライバの問題",
        ),
        ("partitions", "partitions") => (
            "パーティションテーブル詳細。",
            "名前、サイズ、メジャー/マイナーデバイス番号を表示。",
            "ブロックデバイスパーティションテーブル。\n\nカラム: name、size、major、minor。\n\n💡 診断:\n  • メジャー 8=SCSI/SATA、259=NVMe、253=device-mapper\n  • ループデバイス（メジャー 7）→ Snap パッケージまたは ISO",
        ),
        ("diskstats", "active_devices") => (
            "I/O 活動のあるデバイス数。",
            "起動以降に読み書き操作があったブロックデバイスの数。",
            "アクティブなブロックデバイス数。\n\n💡 診断:\n  • 多数の非アクティブデバイスは正常\n  • 期待されるデバイスの I/O がゼロ → 未使用またはワークロード未開始",
        ),
        ("diskstats", "devices") => (
            "デバイス別 I/O 統計。",
            "名前、読み取り数、読み取りバイト、書き込み数、書き込みバイト、処理中 I/O を表示。I/O ボトルネック特定に重要。",
            "デバイス別ディスク I/O 統計。\n\nカラム: name、reads、bytes read、writes、bytes written、in-flight。\n\n💡 診断:\n  • in-flight > 0 持続 → アクティブ I/O 負荷。HDD で >2 は飽和を示唆。\n  • Reads >> Writes → 読み取り中心。RAM 増設でページキャッシュ拡大。\n  • SSD: 高い書き込み量は寿命を劣化。SMART データを確認。\n  • PSI io_some_avg10 と相関させて I/O 停滞を判断。",
        ),
        ("locks", "lock_count") => (
            "アクティブなファイルロック数。",
            "全アクティブファイルロック数（POSIX/FLOCK）。",
            "アクティブなファイルロック数。\n\n💡 診断:\n  • 一般的な値: 10-50\n  • アプリがハング → ロック待ちか確認。PID カラムが保持者を特定。",
        ),
        ("locks", "locks") => (
            "アクティブなファイルロック詳細。",
            "タイプ、モード、PID、inode 情報、バイト範囲を表示。",
            "全アクティブファイルロック。\n\nカラム: type、mode、pid、inode_info、range_start、range_end。\n\n💡 診断:\n  • WRITE ロックは全アクセスをブロック\n  • デッドロック: プロセス A が X を保持し Y を待機、B が Y を保持し X を待機",
        ),

        // ── グループ 5: セキュリティ & カーネル ──────────────────────
        ("modules", "module_count") => (
            "ロード済みカーネルモジュール数。",
            "現在ロードされているカーネルモジュール数。",
            "ロード済みカーネルモジュール数。\n\n💡 診断:\n  • 一般的: 50-150 モジュール\n  • 予期しないモジュール → ルートキットの可能性\n  • 期待されるモジュールがない → `modprobe <name>` を試す",
        ),
        ("modules", "modules") => (
            "ロード済みカーネルモジュール詳細。",
            "名前、メモリサイズ、参照カウント、依存関係、状態を表示。",
            "全ロード済みカーネルモジュール。\n\nカラム: name、size、refcount、dependencies、state。\n\n💡 診断:\n  • refcount > 0 → アンロード不可\n  • refcount = 0 → `rmmod` で安全にアンロード可能\n  • State 'Live' → 正常",
        ),
        ("interrupts", "cpu_count") => (
            "割り込みを処理する CPU 数。",
            "割り込みテーブルに見える CPU 数。",
            "割り込みテーブルからの CPU 数。\n\n💡 診断: 割り込みが CPU 間でバランスしているか確認。`irqbalance` デーモンを使用。",
        ),
        ("interrupts", "irq_count") => (
            "IRQ ライン数。",
            "異なる割り込み要求ラインの数。",
            "IRQ ライン総数。\n\n💡 診断:\n  • 特定 IRQ のカウントが非常に高い → そのデバイスが非常にアクティブ\n  • 共有 IRQ はレガシーシステムで性能問題の可能性",
        ),
        ("interrupts", "interrupts") => (
            "ハードウェア割り込みカウンタ。",
            "全割り込みラインの IRQ 番号、合計カウント、説明を表示。",
            "IRQ 別割り込みカウンタ。\n\nカラム: irq、total_count、description。\n\n💡 診断:\n  • NIC 割り込みの高カウント = 高ネットワークスループット\n  • CPU 間の不均衡 → `irqbalance` を実行",
        ),
        ("softirqs", "softirq_count") => (
            "ソフトウェア割り込みタイプ数。",
            "ソフトウェア割り込みタイプの数（通常 10）。",
            "ソフトウェア割り込みタイプ数。\n\nLinux 固定セット: HI、TIMER、NET_TX、NET_RX、BLOCK、IRQ_POLL、TASKLET、SCHED、HRTIMER、RCU。\n\n💡 カウント自体は固定。タイプ別活動が重要。",
        ),
        ("softirqs", "softirqs") => (
            "ソフトウェア割り込みカウンタ。",
            "タイプ別の全 CPU 呼び出し回数合計。NET_RX/TX はネットワーク、TIMER はカーネルタイマー。",
            "タイプ別ソフトウェア割り込みカウンタ。\n\nカラム: name、total_count。\n\n💡 診断:\n  • NET_RX 高い → インバウンドネットワーク処理\n  • SCHED → ビジーなマルチスレッドシステムで高い\n  • CPU 間の不均衡 → IRQ アフィニティ問題",
        ),
        ("cgroups", "controller_count") => (
            "cgroup コントローラ数。",
            "利用可能な cgroup コントローラの数。リソース管理に使用。",
            "利用可能な cgroup コントローラ数。\n\n💡 診断:\n  • 一般的: cpu、memory、blkio、pids\n  • コントローラがない → カーネル未サポートまたは未マウント",
        ),
        ("cgroups", "controllers") => (
            "cgroup コントローラ詳細。",
            "名前、階層 ID、cgroup 数、有効状態を表示。",
            "cgroup コントローラテーブル。\n\nカラム: name、hierarchy、num_cgroups、enabled。\n\n💡 診断:\n  • hierarchy = 0 → 未使用\n  • memory 未有効 → cgroup ごとのメモリ制限なし",
        ),
        ("crypto", "algorithm_count") => (
            "登録済み暗号アルゴリズム数。",
            "カーネルで利用可能な暗号アルゴリズム数。",
            "登録済み暗号アルゴリズム数。\n\n💡 診断:\n  • 一般的: 50-200\n  • ハードウェアアクセラレーション（'aesni' 等）→ 高速暗号化",
        ),
        ("crypto", "algorithms") => (
            "登録済み暗号アルゴリズム詳細。",
            "名前、タイプ、ドライバ実装、ソースモジュールを表示。",
            "暗号アルゴリズムテーブル。\n\nカラム: name、type、driver、module。\n\n💡 診断:\n  • 'skcipher' → 対称暗号\n  • 'ahash'/'shash' → ハッシュ\n  • Module 'kernel' → ビルトイン",
        ),
        ("devices", "device_count") => (
            "登録済みデバイスドライバ数。",
            "全登録済みキャラクタ/ブロックデバイスドライバ数。",
            "登録済みデバイスドライバ数。\n\n💡 診断:\n  • 期待されるデバイスがない → ドライバ未ロード\n  • キャラクタデバイスは通常ブロックより多い",
        ),
        ("devices", "devices") => (
            "登録済みデバイスドライバ詳細。",
            "タイプ、メジャー番号、デバイス名を表示。",
            "登録済みデバイスドライバテーブル。\n\nカラム: type、major、name。\n\n💡 診断:\n  • メジャー 8=sd（SCSI/SATA）、253=device-mapper、259=NVMe",
        ),
        ("filesystems", "filesystem_count") => (
            "サポートされるファイルシステムタイプ数。",
            "カーネルがマウントできるファイルシステムタイプ数。",
            "サポートされるファイルシステムタイプ数。\n\n💡 診断:\n  • 期待される FS がない → `modprobe <fsname>` を試す\n  • 'nodev' → 仮想ファイルシステム",
        ),
        ("filesystems", "filesystems") => (
            "サポートされるファイルシステムタイプ詳細。",
            "名前とブロックデバイス必要性（nodev）を表示。",
            "サポートされるファイルシステムテーブル。\n\nカラム: name、nodev。\n\n💡 診断:\n  • nodev='yes' → 仮想 FS、物理ディスク不要\n  • nodev='no' → ディスクベース FS\n  • FUSE → ユーザースペース FS ドライバ",
        ),
        ("iomem", "region_count") => (
            "I/O メモリ領域数。",
            "物理メモリマップ I/O 領域数。",
            "I/O メモリ領域数。\n\n💡 診断:\n  • 'System RAM' 領域は使用可能な物理メモリ\n  • PCI デバイス名でハードウェア検出を確認",
        ),
        ("iomem", "regions") => (
            "I/O メモリ領域マップ。",
            "アドレス範囲と説明のテーブル。",
            "物理 I/O メモリマップ。\n\nカラム: address_range、description。\n\n💡 診断:\n  • 'System RAM' → OS が利用可能な RAM\n  • 'Kernel code/data' → カーネル自体が使用",
        ),
        ("ioports", "region_count") => (
            "I/O ポート領域数。",
            "I/O ポートアドレス領域数。レガシー x86 メカニズム。",
            "I/O ポート領域数。\n\n💡 診断:\n  • 主に x86/x86_64 で関連\n  • 現代のシステムでは PCI デバイスが BAR を通じて要求",
        ),
        ("ioports", "regions") => (
            "I/O ポート領域マップ。",
            "ポート範囲と所有デバイスを表示。",
            "I/O ポートアドレスマップ。\n\nカラム: port_range、description。\n\n💡 診断:\n  • 0x60/0x64 → キーボードコントローラ\n  • 0x3F8 → COM1 シリアルポート",
        ),
        ("consoles", "console_count") => (
            "登録済みコンソールデバイス数。",
            "カーネルに登録されたコンソールデバイスの数。",
            "登録済みコンソールデバイス数。\n\n💡 診断:\n  • 一般的: 1-3 コンソール\n  • 0 → カーネルメッセージ出力先なし",
        ),
        ("consoles", "consoles") => (
            "登録済みコンソールデバイス詳細。",
            "名前とフラグを表示。",
            "コンソールデバイステーブル。\n\nカラム: name、flags。\n\n💡 診断:\n  • 'E'=有効、'W'=書き込み可、'R'=読み取り可\n  • ヘッドレスサーバー: シリアルコンソール ttyS0 を設定",
        ),
        ("misc", "device_count") => (
            "misc デバイス数。",
            "メジャー番号 10 を共有する misc キャラクタデバイスの数。",
            "misc デバイス数。\n\n💡 診断:\n  • 'watchdog' → ハードウェアウォッチドッグ\n  • 'fuse' → FUSE サポート利用可能",
        ),
        ("misc", "devices") => (
            "misc デバイス詳細。",
            "マイナー番号とデバイス名を表示。",
            "misc デバイステーブル。\n\nカラム: minor_number、name。\n\n💡 診断:\n  • 'watchdog' (130) → ウォッチドッグサポート\n  • 'kvm' → KVM 仮想化有効",
        ),
        ("dma", "channel_count") => (
            "使用中の DMA チャネル数。",
            "ISA DMA チャネル数。現代システムでは 0 が多い。",
            "ISA DMA チャネル数。\n\n💡 診断:\n  • 0 → レガシーハードウェアなし、正常\n  • 現代の DMA（PCI バスマスタリング）はここに表示されない",
        ),
        ("dma", "channels") => (
            "DMA チャネル詳細。",
            "チャネル番号と要求したデバイスを表示。",
            "ISA DMA チャネル割り当てテーブル。\n\nカラム: channel_number、device_name。\n\n💡 診断:\n  • チャネル 4 = カスケード\n  • 空の場合は ISA ハードウェアなしで想定通り",
        ),
        ("timer_list", "version") => (
            "タイマーリストバージョン。",
            "/proc/timer_list フォーマットのバージョン。",
            "タイマーリストフォーマットバージョン。\n\n💡 メタデータフィールド。",
        ),
        ("timer_list", "now") => (
            "現在のカーネル時間（ナノ秒）。",
            "カーネルの現在時間（ktime）。全タイマー満了の参照点。",
            "現在のカーネル時間（ktime_get）。\n\n単調時計 — NTP 調整の影響を受けない。\n\n💡 1,000,000,000 で割って起動からの秒数を取得。",
        ),
        ("timer_list", "clock_count") => (
            "クロックイベントデバイス数。",
            "ハードウェアタイマーの数。各 CPU にローカル APIC タイマー。",
            "クロックイベントデバイス数。\n\n💡 診断:\n  • CPU 数とほぼ等しいはず\n  • 追加: HPET、PIT、TSC デッドラインタイマー",
        ),
        ("timer_list", "timer_count") => (
            "保留中のカーネルタイマー数。",
            "現在キューに入っているタイマー数。",
            "保留中のカーネルタイマー数。\n\n💡 診断:\n  • 一般的: 50-500\n  • 10000+ → タイマー作成しすぎ（ネットワーク問題またはドライバ異常）",
        ),
        ("schedstat", "version") => (
            "スケジューラ統計バージョン。",
            "schedstat フォーマットのバージョン番号。",
            "schedstat フォーマットバージョン。\n\n💡 バージョン 15 が現在のフォーマット。",
        ),
        ("schedstat", "cpu_count") => (
            "スケジューラ統計のある CPU 数。",
            "スケジューラ統計を報告する CPU 数。",
            "スケジューラ統計の CPU 数。\n\n💡 cpuinfo の CPU 数と一致するはず。少ない場合は一部がオフライン。",
        ),
        ("schedstat", "cpu_stats") => (
            "CPU 別スケジューラ統計。",
            "yield 回数、スケジュール回数、アイドル回数、try-to-wake-up 回数等。",
            "CPU 別スケジューラ統計テーブル。\n\nカラム: cpu、yld_count、sched_count、sched_goidle、ttwu_count、...\n\n💡 診断:\n  • 高い sched_count → 多数のコンテキストスイッチ\n  • sched_goidle / sched_count → アイドル率\n  • CPU 間の大きな格差 → ワークロード不均衡",
        ),

        // file-nr — 追加フィールド
        ("file-nr", "fd_unused") => (
            "未使用（キャッシュ）ファイルディスクリプタ数",
            "割り当て済みだが未使用のファイルハンドル数。再利用のためカーネルのフリーリストに保持。",
            "カーネルのフリーリストに保持されている未使用ファイルハンドル。\n\n💡 診断: fd_allocated に対して fd_unused が多い場合、カーネルが事前に多く割り当てている。無害 — ハンドルは再利用される。",
        ),
        ("file-nr", "fd_max") => (
            "ファイルディスクリプタ最大数（システム上限）",
            "カーネルが割り当てるファイルハンドルの最大数。この上限に達すると全プロセスで EMFILE エラーが発生。",
            "システム全体の FD 最大数 (fs.file-max)。\n\nfd_allocated が fd_max に近づくと、新しい open()/socket() が EMFILE で失敗する。\n\n💡 診断:\n  • デフォルトは RAM に応じて 100000-1000000\n  • 引き上げ: sysctl -w fs.file-max=<値> または /etc/sysctl.conf に永続化\n  • プロセスごとの上限 (ulimit -n) は別で、通常より低い",
        ),

        // df — 追加フィールド
        ("df", "total_disk") => (
            "ディスク容量合計（ルート FS）",
            "ルート (/) ファイルシステムの合計容量（バイト）。root ユーザー用予約領域を含む。",
            "ルートファイルシステムの合計ディスク容量。\n\n💡 診断: total_disk が物理パーティションより小さい場合、予約ブロック（ext4 で通常 5%）を確認。`tune2fs -m <pct> /dev/sdX` で調整可能。",
        ),
        ("df", "used_disk") => (
            "使用ディスク容量（ルート FS）",
            "ルートファイルシステムの使用済みバイト数。ファイル、ディレクトリ、ファイルシステムメタデータを含む。",
            "ルートファイルシステムの使用ディスク容量。\n\n💡 診断: used_disk が total_disk に近い場合、システムが応答不能になる可能性。ログ書き込み失敗、DB クラッシュ、パッケージマネージャ障害。即座のクリーンアップが必要。",
        ),
        ("df", "available_disk") => (
            "利用可能ディスク容量（ルート FS）",
            "ルートファイルシステムの非 root ユーザーが利用可能なバイト数。予約ブロックのため total - used より少ない。",
            "ルートファイルシステムの利用可能ディスク容量（非 root ユーザー向け）。\n\next4 のデフォルトでは約 5% が root 用に予約されるため (total - used) より少ない。\n\n💡 診断:\n  • 1 GB 未満 → 本番システムでは危険\n  • 急速に減少 → ログファイルやテンポラリファイルの蓄積\n  • 大きなファイルを見つける: `du -sh /* | sort -h | tail -10`",
        ),

        // ── net/snmp — 追加の重要カウンタ ────────────────────────────
        ("net/snmp", "Ip_Forwarding") => (
            "IP フォワーディング状態",
            "ホストがパケット転送を行うか (1=有効 ルーターモード、2=無効 ホストモード)。",
            "IP フォワーディング状態。\n\n1 = 転送有効（ルーターモード）、2 = 転送無効（ホストモード）。\n\n💡 診断:\n  • ほとんどのサーバー/ワークステーションでは 2\n  • ルーター、VPN ゲートウェイ、コンテナホストでは 1\n  • 切替: sysctl net.ipv4.ip_forward=1",
        ),
        ("net/snmp", "Ip_InHdrErrors") => (
            "IP ヘッダーエラーのパケット数",
            "不正な IP ヘッダー（チェックサムエラー、無効なバージョン等）によりドロップされたパケット。",
            "IP ヘッダーエラーによるドロップ。\n\n💡 診断:\n  • 0 または 0 に近いはず\n  • 非ゼロ → ネットワーク上の破損パケット、NIC 障害、攻撃トラフィック",
        ),
        ("net/snmp", "Ip_InAddrErrors") => (
            "IP アドレスエラーのパケット数",
            "宛先 IP がこのホストに対して無効だったためドロップされたパケット。",
            "IP アドレスエラーによるドロップ。\n\n💡 診断: 非ゼロはルーティングの問題、または誤設定されたクライアントが間違ったホストにトラフィックを送信していることを示唆。",
        ),
        ("net/snmp", "Ip_ForwDatagrams") => (
            "IP 転送パケット数",
            "別の宛先に転送されたパケット。非ゼロはホストがルーターとして動作していることを意味。",
            "別ホップに転送された IP データグラム。\n\n💡 診断:\n  • ip_forward 無効時は 0 であるべき\n  • 転送無効で非ゼロ → 設定ミス\n  • ルーター/ゲートウェイではコアトラフィックカウンタ",
        ),
        ("net/snmp", "Ip_InDelivers") => (
            "上位プロトコルに配信された IP パケット",
            "上位レイヤープロトコル (TCP、UDP、ICMP) にデマルチプレクスされ配信されたパケット。",
            "上位プロトコルに配信された IP パケット。\n\n💡 診断: Ip_InReceives - Ip_InDelivers = IP 層でドロップ/転送されたパケット。大きな差 = 多数のドロップまたは転送トラフィック。",
        ),
        ("net/snmp", "Ip_OutDiscards") => (
            "送信 IP パケットの破棄数",
            "送信準備ができたが破棄されたパケット（バッファ不足やルーティング失敗が原因）。",
            "送信 IP パケットの破棄。\n\n💡 診断: 非ゼロ → TX パスの輻輳。NIC の TX リングバッファサイズとインターフェースエラーを確認。",
        ),
        ("net/snmp", "Ip_OutNoRoutes") => (
            "ルートなしの IP パケット",
            "ルーティングテーブルに宛先へのルートがなかったためドロップされたパケット。",
            "ルート不在による IP パケットドロップ。\n\n💡 診断:\n  • 非ゼロ → アプリケーションが到達不能なネットワークに接続を試行\n  • ルーティングテーブルを確認: `ip route show`\n  • 一般的な原因: デフォルトゲートウェイ未設定または VPN トンネルダウン",
        ),
        ("net/snmp", "Ip_ReasmFails") => (
            "IP 再構成失敗数",
            "IP フラグメント再構成の試行が失敗した回数（タイムアウト、フラグメント欠落、リソース枯渇）。",
            "IP フラグメント再構成の失敗。\n\n💡 診断:\n  • 非ゼロ → フラグメントが転送中に失われている（ファイアウォールブロック、MTU 問題）\n  • 対処: PMTUD が動作するようにする（ICMP type 3 をブロックしない）",
        ),
        ("net/snmp", "Tcp_CurrEstab") => (
            "現在確立中の TCP 接続数",
            "ESTABLISHED または CLOSE_WAIT 状態の TCP 接続のスナップショット。重要な容量指標。",
            "現在の ESTABLISHED + CLOSE_WAIT TCP 接続数。\n\nポイントインタイムのゲージ（他のカウンタと異なり累積ではない）。\n\n💡 診断:\n  • ベースラインを把握して通常の接続負荷を理解\n  • プラトーなしの継続的増加 → 接続リーク\n  • 急激な減少 → 大量切断イベントまたはサービス再起動",
        ),
        ("net/snmp", "Tcp_InSegs") => (
            "TCP セグメント受信数",
            "受信した TCP セグメントの合計。OutSegs と合わせて TCP スループットの全体像を把握。",
            "受信 TCP セグメント合計。\n\n💡 診断: 変化率がインバウンド TCP スループットを示す。InSegs レート vs OutSegs レートでトラフィック方向の偏りを確認。",
        ),
        ("net/snmp", "Tcp_OutSegs") => (
            "TCP セグメント送信数",
            "送信した TCP セグメントの合計（再送を含む）。主要なアウトバウンド TCP ボリュームカウンタ。",
            "送信 TCP セグメント合計。\n\n💡 診断: RetransSegs / OutSegs で再送率を算出。0.1% 未満は優秀、1% 超は問題。",
        ),
        ("net/snmp", "Tcp_AttemptFails") => (
            "TCP 接続試行失敗数",
            "ハンドシェイク中に失敗した接続（SYN 送信後に SYN-ACK なし等）。",
            "TCP 接続試行の失敗。\n\n💡 診断:\n  • 高レート → 対象ホストに到達不能、FW ブロック、または過負荷\n  • AttemptFails / ActiveOpens = アウトバウンド失敗率",
        ),
        ("net/snmp", "Tcp_EstabResets") => (
            "確立済み接続のリセット",
            "ESTABLISHED 接続がリセット (RST) された数。異常な接続終了を示す。",
            "ESTABLISHED 状態からリセットされた TCP 接続。\n\n💡 診断:\n  • 高レート → リモートホストのクラッシュ、FW による接続切断、アプリのバグ\n  • OutRsts と比較してリセット送信元を特定",
        ),
        ("net/snmp", "Tcp_OutRsts") => (
            "TCP RST セグメント送信数",
            "このホストが送信した RST セグメント。接続拒否や異常切断を示す。",
            "TCP RST セグメント送信数。\n\n💡 診断:\n  • 高レート → 閉じたポートへの多数の接続、またはアプリケーションの接続拒否\n  • 一般的な原因: ポートスキャン、またはサービス再起動",
        ),
        ("net/snmp", "Tcp_InCsumErrors") => (
            "TCP チェックサムエラー",
            "チェックサムが無効な TCP セグメント。ネットワーク経路上のデータ破損を示す。",
            "TCP チェックサムエラーのセグメント。\n\n💡 診断:\n  • 健全なネットワークでは 0 であるべき\n  • 非ゼロ → NIC オフロードバグ、不良ケーブル、メモリ破損\n  • Udp_InCsumErrors も確認 → リンクレベルの破損",
        ),
        ("net/snmp", "Udp_InDatagrams") => (
            "UDP データグラム受信数",
            "アプリケーションに正常に配信された UDP データグラムの合計。",
            "受信・配信された UDP データグラム。\n\n💡 診断: 変化率が UDP 入力スループットを示す。一般的な UDP 利用: DNS (53)、NTP (123)、SNMP (161)。",
        ),
        ("net/snmp", "Udp_OutDatagrams") => (
            "UDP データグラム送信数",
            "このホストが送信した UDP データグラムの合計。",
            "送信 UDP データグラム。\n\n💡 診断: OutDatagrams が多く InDatagrams が少ない → UDP 送信者（syslog フォワーダー、DNS サーバー応答）。逆 → UDP コンシューマ。",
        ),
        ("net/snmp", "Udp_NoPorts") => (
            "閉じたポートへの UDP パケット",
            "リスナーがないポートに受信した UDP データグラム。各パケットで ICMP ポート到達不能応答を生成。",
            "リスナーなしのポートへの UDP データグラム。\n\n💡 診断:\n  • 非ゼロは正常（散発的なプローブ、古い DNS 応答）\n  • 高レート → ポートスキャンまたは誤設定クライアント\n  • 各パケットで ICMP ポート到達不能を生成し帯域幅を消費",
        ),
        ("net/snmp", "Udp_RcvbufErrors") => (
            "UDP 受信バッファオーバーフロー",
            "ソケット受信バッファが満杯のためドロップされた UDP データグラム。アプリの読み取りが遅い。",
            "UDP 受信バッファオーバーフローによるドロップ。\n\n💡 診断:\n  • 非ゼロ → アプリが UDP 受信レートに追いつけない\n  • 対処: SO_RCVBUF または sysctl net.core.rmem_max でバッファ拡大\n  • またはアプリの読み取り処理を最適化",
        ),
        ("net/snmp", "Udp_SndbufErrors") => (
            "UDP 送信バッファオーバーフロー",
            "送信バッファが満杯のためドロップされた UDP データグラム。送信速度がネットワークを超過。",
            "UDP 送信バッファオーバーフローによるドロップ。\n\n💡 診断:\n  • アプリが NIC の送信速度を超えて送信\n  • 対処: net.core.wmem_max を増加、または送信レートを抑制",
        ),
        ("net/snmp", "Udp_InCsumErrors") => (
            "UDP チェックサムエラー",
            "チェックサムが無効な UDP データグラム。転送中のデータ破損。",
            "UDP チェックサムエラー。\n\n💡 診断: TCP チェックサムエラーと同じ意味 — NIC/ケーブル/ドライバの問題。Tcp_InCsumErrors と相互確認。",
        ),
        ("net/snmp", "Icmp_InMsgs") => (
            "ICMP メッセージ受信数",
            "受信した ICMP メッセージの合計（ping 応答、到達不能、リダイレクト等）。",
            "インバウンド ICMP メッセージ合計。\n\n💡 診断: 高レートは ping フラッド、PMTUD 活動、またはネットワークエラーシグナリングを示す可能性。",
        ),
        ("net/snmp", "Icmp_InErrors") => (
            "ICMP 入力エラー",
            "エラーのある ICMP メッセージ（不正チェックサム、短すぎる等）。",
            "エラーのある ICMP メッセージ。\n\n💡 診断: 0 に近いはず。非ゼロはネットワーク上の破損した ICMP パケットを示す。",
        ),
        ("net/snmp", "Icmp_InDestUnreachs") => (
            "ICMP 宛先到達不能受信数",
            "受信した ICMP 宛先到達不能メッセージ。リモートポート/ホストへの到達不能を示す。",
            "受信した ICMP 宛先到達不能メッセージ。\n\n💡 診断:\n  • 高レート → 多数のアウトバウンド接続が失敗（FW ドロップ、ホストダウン）\n  • Type 3 code 4 → PMTUD 問題、MTU を確認\n  • Type 3 code 3 → リモートアプリがリッスンしていない",
        ),
        ("net/snmp", "Icmp_OutMsgs") => (
            "ICMP メッセージ送信数",
            "このホストが送信した ICMP メッセージの合計（ping 要求、到達不能応答等）。",
            "アウトバウンド ICMP メッセージ合計。\n\n💡 診断: 宛先到達不能を多く含む高い OutMsgs → このホストがトラフィックを拒否中（閉じたポートが ICMP 到達不能を生成）。",
        ),

        // ── net/netstat — 追加の重要カウンタ ─────────────────────────
        ("net/netstat", "TcpExt_SyncookiesSent") => (
            "SYN Cookie 送信数（SYN フラッド防御）",
            "SYN Cookie が送信された回数。非ゼロは SYN キューがオーバーフローし SYN フラッド防御が作動したことを意味。",
            "TCP SYN Cookie 送信数。\n\nSYN Cookie は SYN フラッド攻撃に対する防御。SYN キューが満杯の時、カーネルが SYN-ACK シーケンス番号に接続状態をエンコード。\n\n💡 診断:\n  • 非ゼロ → SYN フラッド検知またはリスンバックログが小さすぎる\n  • 正当なトラフィックの場合: net.ipv4.tcp_max_syn_backlog を増加",
        ),
        ("net/netstat", "TcpExt_SyncookiesRecv") => (
            "SYN Cookie 受信数（検証済み）",
            "正常に検証された SYN Cookie の数。SYN Cookie モードを通過した正当な接続。",
            "クライアントから返された有効な SYN Cookie。\n\n💡 診断: SyncookiesSent >> SyncookiesRecv の場合、ほとんどの SYN フラッドトラフィックは偽装 IP から（ハンドシェイクを完了しない）。",
        ),
        ("net/netstat", "TcpExt_SyncookiesFailed") => (
            "無効な SYN Cookie 受信数",
            "検証に失敗した SYN Cookie。パケットが改変された正当なクライアントか攻撃トラフィック。",
            "無効な SYN Cookie 検証失敗。\n\n💡 診断: SyncookiesSent と共に高レート → 偽装完了を試みるアクティブな SYN フラッド攻撃。",
        ),
        ("net/netstat", "TcpExt_TW") => (
            "タイムアウトでリサイクルされた TIME_WAIT ソケット",
            "2*MSL タイムアウト後に自然に期限切れとなった TIME_WAIT ソケット。",
            "正常に期限切れした TIME_WAIT ソケット。\n\n💡 診断: これは通常のクリーンアップパス。TCPTimeWaitOverflow と比較してリサイクルが追いついているか確認。",
        ),
        ("net/netstat", "TcpExt_PAWSEstab") => (
            "PAWS により拒否された確立済み接続のパケット",
            "PAWS (Protection Against Wrapped Sequences) により確立済み接続で拒否されたセグメント。",
            "確立済み接続での PAWS 拒否。\n\n💡 診断:\n  • 散発的は正常（ルート変更後の古い重複セグメント）\n  • 持続的な高レート → 片側のタイムスタンプクロック問題、またはミドルボックスが TCP タイムスタンプを除去",
        ),
        ("net/netstat", "TcpExt_DelayedACKs") => (
            "遅延 ACK 送信数",
            "データセグメントにピギーバックするために遅延された ACK。パケット数を削減する通常の TCP 最適化。",
            "遅延 ACK 送信数（ピギーバック最適化）。\n\n💡 診断: 高い値は正常 — TCP が ACK を効率的にバッチ処理していることを意味する。",
        ),
        ("net/netstat", "TcpExt_TCPHPHits") => (
            "TCP ヘッダー予測ヒット（高速パス）",
            "高速パス（ヘッダー予測）で処理されたパケット。高いほど良い — 大部分が一般的なケースに従うことを意味。",
            "TCP ヘッダー予測高速パスヒット。\n\n💡 診断: InSegs に対して HPHits が高いほどネットワークスタックが効率的。低い比率 → 異常なパケットパターンが低速パス処理を強制。",
        ),
        ("net/netstat", "TcpExt_TCPPureAcks") => (
            "純粋 ACK 受信数（データなし）",
            "データペイロードを含まない ACK セグメント。対話型プロトコルやデータバースト後に一般的。",
            "純粋 ACK 受信（確認応答のみ、データなし）。\n\n💡 診断: 純粋 ACK の比率が高い → 一方向のデータフロー（片側が送信、もう片側が ACK のみ）。ダウンロード/アップロードでは正常。",
        ),
        ("net/netstat", "TcpExt_TCPSackRecovery") => (
            "SACK ベースのロスリカバリ",
            "TCP が SACK 情報を使ってフル再送タイムアウトなしにパケットロスから回復した回数。",
            "TCP SACK ベースのロスリカバリイベント。\n\n💡 診断:\n  • SACK リカバリは RTO ベースより遥かに高速\n  • 高い値 → パケットロスが発生しているが SACK がうまく処理\n  • TCPTimeouts と比較 — タイムアウトは SACK では対処できなかったことを意味",
        ),
        ("net/netstat", "TcpExt_TCPFastRetrans") => (
            "TCP 高速再送",
            "タイムアウト待ちの代わりに高速再送（3重複 ACK）で再送されたセグメント。",
            "TCP 高速再送（3 重複 ACK でトリガー）。\n\n💡 診断:\n  • 高速再送はタイムアウトより望ましい — リカバリが遥かに速い\n  • 高レート → ネットワーク上の頻繁なパケットロス",
        ),
        ("net/netstat", "TcpExt_TCPLossProbes") => (
            "TCP テールロスプローブ送信数",
            "フル RTO を待たずにトランザクション末尾のロスを検出する TLP セグメント。",
            "TCP テールロスプローブ (TLP) 送信数。\n\nTLP は RTO より速くテールロスを検出するメカニズム。バーストの最後のセグメントが失われた場合、TLP が先行して再送。\n\n💡 診断: 高い TLP 数 → 多数のトランザクションで最後のパケットが失われている。",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnData") => (
            "接続中断（クローズ後のデータ）",
            "接続クローズ後にデータが受信されたため中断された TCP 接続。",
            "クローズ後の予期しないデータによる TCP 接続中断。\n\n💡 診断: 通常は接続シャットダウン後にピアがデータを送信したことを示す。アプリケーションレベルのプロトコル不一致。",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnClose") => (
            "接続中断（保留データありでクローズ）",
            "未読データがバッファにある状態でアプリがソケットをクローズしたため RST で終了。",
            "保留データありで close() による TCP 接続中断。\n\n💡 診断:\n  • アプリが全データを読まずにソケットをクローズ\n  • HTTP サーバーが遅いクライアントを中断する場合に一般的\n  • ピアに RST が送信される",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnTimeout") => (
            "タイムアウトによる接続中断",
            "再送試行がタイムアウト制限を超えたため中断された TCP 接続。",
            "タイムアウトによる TCP 接続中断。\n\n💡 診断: 持続的なパケットロスの最終結果 — 全再送試行が失敗。TCPTimeouts と相関。",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnMemory") => (
            "メモリ圧迫による接続中断",
            "TCP バッファのメモリ不足のため終了された接続。",
            "メモリ圧迫による TCP 接続強制終了。\n\n💡 診断:\n  • 非ゼロなら危険: システムが生き残るために接続を切断\n  • sockstat の TCP_mem と /proc/sys/net/ipv4/tcp_mem 制限を確認\n  • RAM 追加または同時接続数の削減が必要",
        ),
        ("net/netstat", "TcpExt_TCPMemoryPressures") => (
            "TCP メモリ圧力イベント",
            "TCP スタックがメモリ圧力モードに入り、バッファサイズを削減し接続をドロップする可能性がある回数。",
            "TCP メモリ圧力モードの発動。\n\n💡 診断:\n  • 非ゼロ → TCP バッファメモリが 'pressure' しきい値に到達\n  • カーネルがソケットごとのバッファサイズを縮小して対処\n  • 対処: tcp_mem 制限の引き上げ、または物理 RAM の追加",
        ),
        ("net/netstat", "TcpExt_TCPSynRetrans") => (
            "SYN/SYN-ACK 再送数",
            "再送された SYN または SYN-ACK セグメント。接続確立の失敗を示す。",
            "SYN および SYN-ACK の再送。\n\n💡 診断:\n  • 高レート → クライアントがサーバーに到達不能（FW、サーバー過負荷、ネットワークロス）\n  • サーバー側バックログが満杯の場合 ListenDrops と相関",
        ),
        ("net/netstat", "TcpExt_TCPOrigDataSent") => (
            "オリジナルデータセグメント送信数",
            "初めて送信されたデータセグメント（再送を除く）。OutSegs から引いて再送数を算出可能。",
            "オリジナル（非再送）データセグメント送信数。\n\n💡 診断: (OutSegs - TCPOrigDataSent - 純粋 ACK) で再送ボリュームを概算。再送率の計算に有用。",
        ),
        ("net/netstat", "TcpExt_TCPKeepAlive") => (
            "TCP キープアライブプローブ送信数",
            "アイドル接続でピアの生存を確認するためのキープアライブプローブ。",
            "TCP キープアライブプローブ送信数。\n\n💡 診断:\n  • 長時間アイドルの接続（DB 接続、SSH セッション）では正常\n  • 非常に高いレート → キープアライブ有効の多数のアイドル接続\n  • 調整: net.ipv4.tcp_keepalive_time (デフォルト 7200 秒)",
        ),
        ("net/netstat", "TcpExt_TCPAutoCorking") => (
            "TCP 自動コーキングイベント",
            "小さな書き込みを大きなセグメントに結合するためカーネルが遅延させた回数。",
            "TCP 自動コーキング発動。\n\n💡 診断: 自動コーキングは未確認データがある時に書き込みをバッファリングして小パケットのオーバーヘッドを削減。書き込み集中ワークロードでは高い値は正常。",
        ),
        ("net/netstat", "TcpExt_TCPRcvCoalesce") => (
            "TCP 受信キュー結合",
            "効率化のため受信キューで結合（マージ）されたセグメント。",
            "受信キューで結合された TCP セグメント。\n\n💡 診断: 結合は隣接セグメントをマージしてオーバーヘッドを削減。高い値は NIC の GRO/LRO が効率的に処理していることを示す。",
        ),
        ("net/netstat", "TcpExt_TCPOFOQueue") => (
            "順序外パケットのキュー追加",
            "順序外で受信しリオーダリングのためキューに追加されたパケット。ネットワーク経路のリオーダリングを示す。",
            "順序外 TCP パケットのキュー追加。\n\n💡 診断:\n  • 一部の順序外は正常（マルチパス、ロードバランスされたトラフィック）\n  • 高レート → ネットワーク経路で顕著なリオーダリング\n  • SACK で処理されないと偽の高速再送をトリガーする可能性",
        ),
        ("net/netstat", "TcpExt_TCPChallengeACK") => (
            "チャレンジ ACK 送信数 (RFC 5961)",
            "接続を検証するため不審なセグメントに応答して送信された ACK。ブラインドインジェクション対策。",
            "TCP チャレンジ ACK 送信 (RFC 5961)。\n\n💡 診断:\n  • 低レートは正常（散発的な古いセグメント）\n  • 高レート → ブラインド TCP インジェクション攻撃の可能性\n  • net.ipv4.tcp_challenge_ack_limit でレート制限",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenActive") => (
            "TCP Fast Open 接続開始数",
            "SYN パケットにデータを含めて送信する TCP Fast Open (TFO) を使ったアウトバウンド接続。",
            "TCP Fast Open アクティブ接続開始数。\n\n💡 診断: TFO はリピート接続で 1 RTT のレイテンシを削減。非ゼロならアプリが TFO を使用中。ゼロは TFO 無効またはアプリが要求していないことを意味。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassive") => (
            "TCP Fast Open 接続受付数",
            "SYN パケットの TCP Fast Open データ付きで受け付けたインバウンド接続。",
            "TCP Fast Open パッシブ接続受付数。\n\n💡 診断: 非ゼロ → サーバーが TFO 接続を受付中。FastOpenPassiveFail と比較して失敗率を確認。",
        ),
        ("net/netstat", "TcpExt_TCPDelivered") => (
            "アプリケーションに配信された TCP セグメント",
            "アプリケーション層に正常配信されたセグメント合計。オリジナルと再送データを含む。",
            "アプリケーションに配信された TCP セグメント。\n\n💡 診断: グッドプットカウンタ。OutSegs の合計パケット数と比較してオーバーヘッド比率を確認。",
        ),
        ("net/netstat", "TcpExt_TCPWinProbe") => (
            "TCP ウィンドウプローブ送信数",
            "受信ウィンドウがゼロの時に送信されたウィンドウプローブ。送信者が受信者のバッファ解放を待機。",
            "TCP ゼロウィンドウプローブ送信数。\n\n💡 診断:\n  • 非ゼロ → 受信者のデータ消費が遅い（ゼロウィンドウ状態）\n  • 持続的 → アプリケーションバックプレッシャー\n  • TCPToZeroWindowAdv で受信者がゼロウィンドウを通知する頻度を確認",
        ),
        ("net/netstat", "TcpExt_TCPTimeWaitOverflow") => (
            "TIME_WAIT バケットオーバーフロー",
            "バケット制限に達して新しい TIME_WAIT ソケットを作成できなかった回数。",
            "TIME_WAIT ソケットオーバーフローイベント。\n\n💡 診断:\n  • 非ゼロ → 短命接続が多すぎて TIME_WAIT スロットを枯渇\n  • net.ipv4.tcp_tw_reuse を有効化して再利用を許可\n  • アプリケーションでコネクションプーリングを検討",
        ),
        ("net/netstat", "TcpExt_TCPBacklogDrop") => (
            "ソケットバックログからのドロップ",
            "ソケットごとのバックログキューが満杯のためドロップされたセグメント。",
            "TCP ソケットバックログドロップ。\n\n💡 診断:\n  • 非ゼロ → 受信アプリの処理が追いつかない\n  • ソケットバックログは NIC とアプリの read() の間のキュー\n  • 対処: SO_RCVBUF の増加またはアプリの読み取りループの最適化",
        ),
        ("net/netstat", "TcpExt_TCPSpuriousRTOs") => (
            "偽の再送タイムアウト",
            "偽と検出された RTO（元のパケットは到着していたが RTO が早すぎた）。",
            "偽 RTO 検出。\n\n💡 診断:\n  • 高レート → ネットワークレイテンシに対して RTO が過度に積極的\n  • 不要な再送と輻輳ウィンドウ縮小を引き起こす",
        ),
        ("net/netstat", "IpExt_InOctets") => (
            "受信バイト合計（IP 層）",
            "IP 層で受信した合計バイト数（ヘッダー含む）。決定的なインバウンド帯域幅カウンタ。",
            "IP 層インバウンド合計バイト。\n\n💡 診断: 変化率がインバウンド帯域幅を示す。net/dev の total_rx と比較 — 差分は非 IP トラフィック（ARP 等）。",
        ),
        ("net/netstat", "IpExt_OutOctets") => (
            "送信バイト合計（IP 層）",
            "IP 層で送信した合計バイト数（ヘッダー含む）。決定的なアウトバウンド帯域幅カウンタ。",
            "IP 層アウトバウンド合計バイト。\n\n💡 診断: 変化率がアウトバウンド帯域幅を示す。IP トラフィックの最も正確な帯域幅カウンタ。",
        ),
        ("net/netstat", "IpExt_InMcastPkts") => (
            "マルチキャストパケット受信数",
            "受信した IP マルチキャストパケット。サービスディスカバリ (mDNS, SSDP) やクラスタ通信で一般的。",
            "インバウンドマルチキャストパケット。\n\n💡 診断: サーバーで高レート → クラスタ/マルチキャストベースのサービスがアクティブ。デスクトップ → mDNS/Bonjour またはメディアストリーミング。",
        ),
        ("net/netstat", "IpExt_InCsumErrors") => (
            "IP チェックサムエラー",
            "IP レベルのチェックサムエラーのインバウンドパケット。ネットワーク層のデータ破損を示す。",
            "IP 層チェックサムエラー。\n\n💡 診断:\n  • 健全なネットワークでは 0 であるべき\n  • 非ゼロ → リンクレベルの破損、不良 NIC オフロード、メモリエラー\n  • TCP/UDP チェックサムエラーと相関させて全体像を把握",
        ),

        // ── net/sockstat — 残りのフィールド ──────────────────────────
        ("net/sockstat", "UDPLITE_inuse") => (
            "使用中の UDP-Lite ソケット数",
            "現在使用中の UDP-Lite ソケット数。部分チェックサムでエラー許容メディアストリーム用。",
            "アクティブな UDP-Lite ソケット。\n\n💡 診断: ほとんどのシステムで通常 0。UDP-Lite は部分データの方がデータなしより良いメディアストリーミングで使用。",
        ),
        ("net/sockstat", "RAW_inuse") => (
            "使用中の Raw ソケット数",
            "Raw IP ソケットの数。ping、traceroute、カスタムネットワークプロトコルで使用。",
            "アクティブな Raw IP ソケット。\n\n💡 診断:\n  • 通常: 0-2\n  • ping や traceroute が一時的に使用\n  • 永続的な Raw ソケット → カスタムプロトコルまたは監視ツール\n  • セキュリティ上の注意: Raw ソケットは root または CAP_NET_RAW が必要",
        ),
        ("net/sockstat", "FRAG_memory") => (
            "IP フラグメント再構成メモリ（バイト）",
            "再構成待ちの IP フラグメントを保持するためのカーネルメモリ（バイト）。",
            "IP フラグメント再構成キューが消費するメモリ。\n\n💡 診断:\n  • 現代のネットワークでは通常 0（PMTUD がフラグメント化を回避）\n  • 大きな値 → フラグメント攻撃または PMTUD の故障\n  • 制限: net.ipv4.ipfrag_high_thresh",
        ),

        // ── ip/route ─────────────────────────────────────────────────
        ("ip/route", "default_gateway") => (
            "デフォルトゲートウェイ IP アドレス",
            "デフォルトゲートウェイの IP アドレス。未知のネットワークへのトラフィックは全てここに転送。",
            "デフォルトゲートウェイ IP アドレス。\n\n💡 診断:\n  • '(none)' → デフォルトルート未設定。インターネットに到達不可。\n  • 到達可能であるべき: `ping <ゲートウェイ>`。不可なら配線/リンクを確認。\n  • 複数のデフォルトルート → フェイルオーバーまたはポリシールーティング使用中",
        ),
        ("ip/route", "route_count") => (
            "IP ルート数",
            "'ip route show' からのルーティングエントリ合計。デフォルト、接続済み、静的ルートを含む。",
            "IP ルーティングエントリ合計。\n\n💡 診断:\n  • インターネット接続ホストでは最低 1（デフォルトルート）\n  • 非常に多い → ダイナミックルーティングプロトコル (BGP/OSPF) がアクティブ",
        ),
        ("ip/route", "routes") => (
            "IP ルーティングテーブル",
            "完全なルーティングテーブル: 宛先、ゲートウェイ、デバイス、プロトコル、スコープ、メトリック。低いメトリック = 優先ルート。",
            "'ip route show' からの IP ルーティングテーブル。\n\nカラム: Destination、Gateway、Device、Protocol、Scope、Metric。\n\n💡 診断:\n  • 'default via X.X.X.X' → デフォルトルート。インターネットアクセスに必須。\n  • Protocol 'kernel' → 直接接続ネットワーク用に自動作成\n  • Protocol 'dhcp' → DHCP サーバーから学習\n  • Scope 'link' → 直接接続ネットワーク、ゲートウェイ不要\n  • 低いメトリック = 複数ルート存在時の優先度が高い",
        ),

        // ── ip/neighbor ──────────────────────────────────────────────
        ("ip/neighbor", "neighbor_count") => (
            "ARP/NDP ネイバーエントリ合計",
            "全ネイバーキャッシュエントリ (IPv4 ARP と IPv6 NDP) の数。このシステムが通信したホスト数を示す。",
            "ARP/NDP ネイバーキャッシュエントリ数。\n\n💡 診断:\n  • 高い値 → ローカルネットワーク上のホストが多い\n  • 継続的に増加 → ネットワークスキャンまたはブロードキャストストーム\n  • FAILED エントリは failed_count で別途確認",
        ),
        ("ip/neighbor", "failed_count") => (
            "FAILED 状態のネイバー数",
            "解決できなかったネイバー（ARP/NDP 要求を送信したが応答なし）。到達不能ホストを示す。",
            "FAILED 状態のネイバーエントリ（到達不能）。\n\n💡 診断:\n  • 非ゼロ → ローカルネットワーク上のホストに到達不能\n  • 一般的な原因: ホスト電源オフ、VLAN 誤り、IP 競合\n  • 一時的な失敗は正常; 持続的なものは実際の問題を示す",
        ),
        ("ip/neighbor", "neighbors") => (
            "ネイバーテーブルエントリ",
            "ARP/NDP ネイバーキャッシュ: IP アドレス、ネットワークデバイス、MAC アドレス、到達可能性状態。",
            "ARP/NDP ネイバーキャッシュの内容。\n\nカラム: IP、Device、LLAddr (MAC)、State。\n\n💡 診断:\n  • State REACHABLE → 最近生存確認済み\n  • State STALE → 最近未確認、次回使用時に再プローブ\n  • State FAILED → ARP/NDP 解決失敗、ホスト到達不能\n  • State PERMANENT → 静的に設定されたエントリ\n  • FAILED で空の LLAddr → ホストが ARP に応答しなかった",
        ),

        // ── ss（ソケットサマリ）────────────────────────────────────
        ("ss", "tcp_established") => (
            "確立済み TCP 接続数",
            "ESTABLISHED 状態の TCP 接続数。アクティブなネットワークセッションの主要指標。",
            "ESTABLISHED TCP 接続数。\n\n💡 診断:\n  • ワークロードに応じたベースラインを設定（Web サーバー: 数百-数千、DB: 数十）\n  • 急増 → トラフィックスパイクまたは SYN フラッドの完了\n  • 急減 → サービスクラッシュまたはネットワーク分断\n  • プラトーなしの継続増加 → アプリの接続リーク",
        ),
        ("ss", "tcp_timewait") => (
            "TIME_WAIT の TCP 接続数",
            "クローズ後 2*MSL（通常 60 秒）のクールダウン中の TIME_WAIT 状態の接続。",
            "TCP TIME_WAIT ソケット数。\n\n💡 診断:\n  • 5000 未満 → ほとんどのワークロードで正常\n  • 30000 超 → エフェメラルポート枯渇の危険\n  • 高い値 → 短命接続が多い; コネクションプーリングを検討\n  • 忙しいサーバーでは net.ipv4.tcp_tw_reuse を有効化",
        ),
        ("ss", "tcp_orphaned") => (
            "孤立 TCP 接続数",
            "所有プロセスのない TCP 接続。タイムアウトまでカーネルメモリを消費。",
            "孤立 TCP 接続。\n\n💡 診断:\n  • 低い値であるべき（100 未満）\n  • 増加 → アプリが適切なソケットクリーンアップなしに終了\n  • 制限: /proc/sys/net/ipv4/tcp_max_orphans\n  • 制限超過 → カーネルが強制的に接続を RST",
        ),
        ("ss", "tcp_closed") => (
            "クローズ済み TCP 接続数",
            "CLOSED 状態の TCP 接続。カーネルによるクリーンアップ待ち。",
            "CLOSED 状態の TCP 接続。\n\n💡 診断: これらは一時的 — カーネルが間もなくクリーンアップする。持続的に高い値はカーネルまたはドライバの問題を示す可能性。",
        ),
        ("ss", "udp_count") => (
            "UDP ソケット合計",
            "'ss -s' が報告するシステム上の全 UDP ソケット数。",
            "ss -s からの UDP ソケット合計数。\n\n💡 診断:\n  • 一般的: 通常のサーバーで 5-30\n  • DNS リゾルバ、NTP クライアント、syslog、SNMP を含む\n  • 増加 → UDP アプリケーションの FD リークの可能性",
        ),

        // ── dns (/etc/resolv.conf) ───────────────────────────────────
        ("dns", "nameservers") => (
            "設定済み DNS ネームサーバー",
            "/etc/resolv.conf からの DNS サーバーアドレス。名前解決時にこれらのサーバーに問い合わせ。",
            "/etc/resolv.conf からの DNS ネームサーバーテーブル。\n\nカラム: IP アドレス、タイプ (IPv4/IPv6)。\n\n💡 診断:\n  • 空 → DNS 名前解決が失敗する。/etc/resolv.conf を確認。\n  • 127.0.0.53 → systemd-resolved が DNS を管理\n  • 複数エントリ → フェイルオーバー; 最初のサーバーが優先\n  • 8.8.8.8 / 1.1.1.1 → パブリック DNS (Google/Cloudflare)",
        ),
        ("dns", "search_domains") => (
            "DNS 検索ドメイン",
            "非修飾ホスト名に付加されるドメイン。'search example.com' は 'host' を 'host.example.com' として解決。",
            "DNS 検索ドメインリスト。\n\n💡 診断:\n  • 短いホスト名の解決に影響（例: 'db' が 'db.example.com' になる）\n  • 検索ドメインが多すぎる → DNS 解決が遅い（各サフィックスを試行）\n  • 企業環境では複数の検索ドメインが一般的",
        ),
        ("dns", "options") => (
            "DNS リゾルバオプション",
            "/etc/resolv.conf からのリゾルバオプション。タイムアウト、リトライ、動作を制御。",
            "DNS リゾルバオプション。\n\n💡 診断:\n  • 'ndots:N' → ドットが N 個未満のクエリは検索ドメインを先に試行\n  • 'timeout:N' → 別のネームサーバーに再試行するまでの秒数\n  • 'attempts:N' → ネームサーバーごとのリトライ回数\n  • コンテナ内で高い ndots は DNS 解決を遅くする可能性",
        ),
        ("dns", "dns_resolution_ms") => (
            "DNS 解決時間（localhost テスト）",
            "'localhost' をシステムリゾルバで解決する時間（ミリ秒）。リゾルバのオーバーヘッドを測定。",
            "'localhost' の DNS 解決レイテンシ。\n\n💡 診断:\n  • 1 ms 未満 → 正常（/etc/hosts または nsswitch キャッシュから解決）\n  • 10 ms 超 → リゾルバが遅い可能性（DNS サーバーへのネットワーク往復）\n  • 100 ms 超 → DNS サーバーが応答しないか過負荷\n  • これはベースラインテスト; 外部名の実際の解決はより遅い可能性",
        ),

        // ── conntrack（コネクション追跡）─────────────────────────────
        ("conntrack", "conntrack_count") => (
            "現在の追跡接続数",
            "netfilter コネクション追跡テーブルのエントリ数。各 TCP/UDP フローが 1 エントリを使用。",
            "現在のコネクション追跡テーブルエントリ。\n\n💡 診断:\n  • 各確立済み接続（TCP、UDP、ICMP）が 1 エントリを使用\n  • conntrack_max に近づく → 新規接続がドロップされる！\n  • 忙しい NAT/FW ボックスではこれが重要な容量メトリクス",
        ),
        ("conntrack", "conntrack_max") => (
            "最大追跡接続数",
            "コネクション追跡テーブルの最大エントリ数。超過すると新規接続をドロップ。",
            "コネクション追跡テーブル最大値 (nf_conntrack_max)。\n\n💡 診断:\n  • デフォルトは RAM に応じてスケール（1GB システムで通常 65536）\n  • 引き上げ: sysctl -w net.nf_conntrack_max=<値>\n  • 各エントリは約 300 バイトのカーネルメモリを使用\n  • 忙しい FW/NAT: conntrack_max > ピーク同時接続数",
        ),
        ("conntrack", "usage_pct") => (
            "コネクション追跡テーブル使用率 %",
            "コネクション追跡テーブルの使用割合。80% 超は新規接続ドロップの危険を示す。",
            "コネクション追跡テーブルの使用率。\n\n💡 診断:\n  • 50% 未満 → 健全な余裕\n  • 50-80% → トレンドを監視。max の引き上げが必要かも。\n  • 80% 超 → 警告: 上限に接近。新規接続がドロップされる可能性。\n  • 95% 超 → 危険: 接続を積極的にドロップ中。即座に conntrack_max を増加。\n  • conntrack が必要かも確認 — NAT/iptables ステートフルルールを使わないなら無効化",
        ),

        // ── meminfo: 残りのフィールド ─────────────────────────────────
        ("meminfo", "Active") => (
            "最近使用されたメモリ",
            "アクティブ LRU 上のメモリ。最近アクセスされ回収されにくい。",
            "アクティブメモリ = Active(anon) + Active(file)。\n\n💡 Active vs Inactive を比較してメモリの「ホット」度を確認。",
        ),
        ("meminfo", "Inactive") => (
            "最近未使用のメモリ（回収候補）",
            "非アクティブ LRU 上のメモリ。メモリ圧迫時に最初に回収。",
            "非アクティブメモリ = Inactive(anon) + Inactive(file)。\n\n💡 大きな Inactive はメモリ圧迫への良いバッファ。",
        ),
        ("meminfo", "Active(anon)") => (
            "最近使用された匿名メモリ",
            "最近アクセスされたヒープ/スタックメモリ。スワッピングでのみ解放。",
            "アクティブな匿名ページ。\n\n💡 増加中 → プロセスがヒープメモリを積極的に使用中。",
        ),
        ("meminfo", "Inactive(anon)") => (
            "非アクティブな匿名メモリ",
            "最近アクセスされていない匿名メモリ。スワップアウト候補。",
            "非アクティブな匿名ページ。\n\n💡 スワップ活動なしで大きい → 必要時にスワップで回収可能。",
        ),
        ("meminfo", "Active(file)") => (
            "最近使用されたファイルキャッシュ",
            "最近アクセスされたページキャッシュ。再読み込み高速化。",
            "アクティブなファイルページ。\n\n💡 負荷下で縮小 → メモリ圧迫でキャッシュが追い出されている。",
        ),
        ("meminfo", "Inactive(file)") => (
            "非アクティブなファイルキャッシュ",
            "最近アクセスされていないファイルページ。回収が容易。",
            "非アクティブなファイルページ。\n\n💡 大きい = 圧迫時の良いバッファ。ほぼゼロ = 回収可能メモリが逼迫。",
        ),
        ("meminfo", "Unevictable") => (
            "回収不可のロックされたメモリ",
            "mlock、ramfs、SHM_LOCK で RAM に固定されたメモリ。",
            "回収不可メモリ。\n\n💡 高い → mlock 使用プロセスを確認。キャッシュ可能メモリが減少。",
        ),
        ("meminfo", "Mlocked") => (
            "mlock でロックされたメモリ",
            "mlock() で RAM に固定。スワップアウト防止。",
            "mlock メモリ。\n\n💡 DB、リアルタイムオーディオ、暗号鍵保存で使用。",
        ),
        ("meminfo", "SwapCached") => (
            "RAM にもあるスワップページ",
            "スワップアウト後も RAM にキャッシュ。再アクセス時に I/O 不要。",
            "スワップキャッシュ。\n\n💡 メモリ圧迫後の非ゼロは正常な回復動作。",
        ),
        ("meminfo", "Zswap") => (
            "zswap 圧縮プール使用量",
            "zswap の圧縮ページプールが消費する RAM。",
            "zswap 圧縮プール。\n\n💡 Zswapped / Zswap = 圧縮率。高いほど効果的。",
        ),
        ("meminfo", "Zswapped") => (
            "zswap 内ページの元サイズ",
            "zswap に格納されたページの非圧縮サイズ。",
            "zswap 元サイズ。\n\n💡 一般的な圧縮率: 2:1〜4:1。",
        ),
        ("meminfo", "Dirty") => (
            "ディスク書き込み待ちメモリ",
            "変更済みだが未書き込みのページ。カーネルが定期フラッシュ。",
            "ダーティページ。\n\n💡 高い → 書き込み集中か低速ストレージ。dirty_ratio 超過 → write() がブロック。",
        ),
        ("meminfo", "Writeback") => (
            "ディスクに書き込み中のメモリ",
            "現在ストレージにフラッシュ中のページ。",
            "ライトバック中ページ。\n\n💡 通常はほぼゼロ。持続的に高い → ストレージ飽和。",
        ),
        ("meminfo", "AnonPages") => (
            "プロセスの匿名ページ",
            "ページテーブルにマップされた匿名メモリ。ヒープ、スタック等。",
            "匿名ページ。\n\n💡 時間とともに増加 → メモリリークの可能性。`ps aux --sort=-rss | head` で犯人探し。",
        ),
        ("meminfo", "Mapped") => (
            "メモリマップされたファイル",
            "mmap() でマップされたファイル。共有ライブラリ等を含む。",
            "メモリマップファイルページ。\n\n💡 高い → 多数のプロセスがライブラリを共有、または大きな mmap DB ファイル。",
        ),
        ("meminfo", "Shmem") => (
            "共有メモリ (tmpfs, shmem)",
            "共有メモリ、tmpfs、devtmpfs。Cached に含まれるが回収不可。",
            "共有メモリページ。\n\n💡 `df -h /dev/shm` で確認。通常のキャッシュと異なり MemAvailable を減少させる。",
        ),
        ("meminfo", "KReclaimable") => (
            "回収可能なカーネルメモリ",
            "SReclaimable を含む回収可能なカーネルメモリ合計。",
            "回収可能カーネルメモリ。\n\n💡 大きいのは健全 — 必要時にカーネルが解放。",
        ),
        ("meminfo", "Slab") => (
            "スラブアロケータメモリ合計",
            "SReclaimable + SUnreclaim。カーネルオブジェクト用。",
            "スラブメモリ合計。\n\n💡 SUnreclaim が増加中 → カーネルメモリリークの可能性。",
        ),
        ("meminfo", "SReclaimable") => (
            "回収可能なスラブキャッシュ",
            "回収可能なスラブ。主に dentry/inode キャッシュ。",
            "回収可能スラブ。\n\n💡 `echo 2 > /proc/sys/vm/drop_caches` で手動回収。",
        ),
        ("meminfo", "SUnreclaim") => (
            "回収不可のスラブメモリ",
            "解放できないカーネルデータ構造。",
            "回収不可スラブ。\n\n💡 増加中 → カーネルメモリリークの可能性。slabinfo を確認。",
        ),
        ("meminfo", "KernelStack") => (
            "カーネルスタックメモリ",
            "全スレッドのカーネルモードスタック。各 8-16KB。",
            "カーネルスタック。\n\n💡 KernelStack / 16KB ≈ スレッド数。非常に高い → スレッドリーク。",
        ),
        ("meminfo", "PageTables") => (
            "ページテーブルメモリ",
            "仮想→物理アドレス変換テーブル用メモリ。",
            "ページテーブル。\n\n💡 高い → プロセス数が多いか大きな仮想メモリマッピング。",
        ),
        ("meminfo", "SecPageTables") => (
            "二次ページテーブル (KVM, IOMMU)",
            "KVM と IOMMU 用のネストされたページテーブル。",
            "二次ページテーブル。\n\n💡 高い → 多数の VM が稼働中または IOMMU が活発。",
        ),
        ("meminfo", "NFS_Unstable") => (
            "NFS 未コミットページ (廃止)",
            "最新カーネルでは常に 0。廃止フィールド。",
            "NFS 未コミットページ — 常に 0。\n\n💡 廃止。無視して可。",
        ),
        ("meminfo", "Bounce") => (
            "バウンスバッファメモリ",
            "DMA デバイスが全物理メモリにアクセスできない場合のバッファ。",
            "バウンスバッファ。最新 64 ビットシステムでは通常 0。\n\n💡 非ゼロ → レガシーデバイスを確認。",
        ),
        ("meminfo", "WritebackTmp") => (
            "FUSE 一時ライトバックメモリ",
            "FUSE ファイルシステムの一時バッファ。",
            "FUSE 一時ライトバック。\n\n💡 FUSE 書き込み中のみ非ゼロ（sshfs、s3fs 等）。",
        ),
        ("meminfo", "CommitLimit") => (
            "メモリ確保上限",
            "オーバーコミット比率に基づく最大確保可能メモリ。",
            "コミットリミット。\n\n💡 overcommit_memory = 2 (厳格) の場合のみ有意。",
        ),
        ("meminfo", "Committed_AS") => (
            "コミット済みメモリ合計",
            "全プロセスの確保済みメモリ合計。物理 RAM を超えることがある。",
            "コミット済み仮想メモリ。\n\n💡 Committed_AS > RAM + Swap → オーバーコミット状態。OOM リスクあり。",
        ),
        ("meminfo", "VmallocTotal") => (
            "vmalloc アドレス空間合計",
            "vmalloc 仮想アドレス範囲。64 ビットでは極大。",
            "vmalloc 総アドレス空間。64 ビットでは事実上無意味。\n\n💡 VmallocUsed を確認。",
        ),
        ("meminfo", "VmallocUsed") => (
            "vmalloc 使用量",
            "実際に使用中の vmalloc メモリ。カーネルモジュール等。",
            "vmalloc 使用量。\n\n💡 通常 100MB 未満。増加中 → カーネルモジュールのメモリリークの可能性。",
        ),
        ("meminfo", "VmallocChunk") => (
            "vmalloc 最大空きブロック",
            "vmalloc の最大連続空きブロック。64 ビットでは無関係。",
            "vmalloc 最大空きブロック。\n\n💡 32 ビットカーネルでのみ関連。",
        ),
        ("meminfo", "Percpu") => (
            "Per-CPU データ構造メモリ",
            "CPU ごとの変数用メモリ。CPU 数に比例。",
            "Per-CPU メモリ。\n\n💡 ≈ 定数 × CPU 数。",
        ),
        ("meminfo", "HardwareCorrupted") => (
            "ハードウェアエラーメモリ (ECC)",
            "ECC エラーで退避されたメモリ。使用不可。",
            "ハードウェアエラーメモリ。\n\n💡 非ゼロ → 危険: ハードウェア障害。交換を計画。",
        ),
        ("meminfo", "AnonHugePages") => (
            "匿名ヒュージページ",
            "THP (2MB) を使用する匿名メモリ。TLB ミス削減。",
            "匿名 THP。\n\n💡 ゼロ → THP 無効または断片化。一部 DB は THP を無効化。",
        ),
        ("meminfo", "ShmemHugePages") => (
            "共有メモリヒュージページ",
            "ヒュージページを使用する共有メモリ。",
            "共有メモリヒュージページ。\n\n💡 tmpfs の huge=always 設定が必要。",
        ),
        ("meminfo", "ShmemPmdMapped") => (
            "共有メモリ PMD マップ",
            "PMD レベル（2MB）でマップされた共有メモリ。",
            "共有メモリ PMD マップ。\n\n💡 ShmemHugePages のうち実際に 2MB でマップされた部分。",
        ),
        ("meminfo", "FileHugePages") => (
            "ファイルヒュージページ",
            "THP を使用するファイル連動メモリ。",
            "ファイル THP。\n\n💡 比較的新しい機能。",
        ),
        ("meminfo", "FilePmdMapped") => (
            "ファイル PMD マップ",
            "PMD レベルでマップされたファイルページ。",
            "ファイル PMD マップ。\n\n💡 ファイルコンテンツの実際の 2MB マッピング。",
        ),
        ("meminfo", "CmaTotal") => (
            "CMA 予約メモリ合計",
            "連続メモリアロケータの予約メモリ。DMA デバイス用。",
            "CMA 予約領域。\n\n💡 0 → CMA 未設定（サーバーでは一般的）。",
        ),
        ("meminfo", "CmaFree") => (
            "CMA 空きページ",
            "CMA 予約領域の未使用メモリ。",
            "CMA 空きメモリ。\n\n💡 CmaFree = CmaTotal → 予約済みだがデバイス未使用。",
        ),
        ("meminfo", "HugePages_Total") => (
            "ヒュージページ総数",
            "事前確保されたヒュージページ数。各 2MB。",
            "ヒュージページプール。\n\n💡 DB、DPDK、VM で使用。このメモリは他に使えない。",
        ),
        ("meminfo", "HugePages_Free") => (
            "未使用ヒュージページ",
            "プロセスに未割り当てのヒュージページ。",
            "空きヒュージページ。\n\n💡 Free = Total → 予約メモリが無駄。",
        ),
        ("meminfo", "HugePages_Rsvd") => (
            "予約済み未割り当てヒュージページ",
            "コミット済みだが未フォールトのヒュージページ。",
            "予約済みヒュージページ。\n\n💡 Free - Rsvd = 新規予約可能なページ数。",
        ),
        ("meminfo", "HugePages_Surp") => (
            "超過ヒュージページ",
            "nr_hugepages を超えて確保されたヒュージページ。",
            "超過ヒュージページ。\n\n💡 不要になれば解放される。",
        ),
        ("meminfo", "Hugepagesize") => (
            "デフォルトヒュージページサイズ",
            "各ヒュージページのサイズ。x86_64 で通常 2MB。",
            "ヒュージページサイズ。\n\n💡 hugepagesz= カーネルパラメータで設定。",
        ),
        ("meminfo", "Hugetlb") => (
            "ヒュージページ使用メモリ合計",
            "全ヒュージページサイズの合計メモリ。他用途に使用不可。",
            "ヒュージページメモリ合計。\n\n💡 大きいが未使用 → nr_hugepages を削減。",
        ),
        ("meminfo", "DirectMap4k") => (
            "直接マップ 4KB",
            "カーネル直接マップで 4KB ページを使用するメモリ。",
            "直接マップ 4KB エントリ。\n\n💡 2M/1G に比べて多い → 直接マップの断片化。",
        ),
        ("meminfo", "DirectMap2M") => (
            "直接マップ 2MB",
            "カーネル直接マップで 2MB ページを使用するメモリ。",
            "直接マップ 2MB エントリ。\n\n💡 大部分のメモリがここにあるべき。",
        ),
        ("meminfo", "DirectMap1G") => (
            "直接マップ 1GB",
            "カーネル直接マップで 1GB ページを使用するメモリ。",
            "直接マップ 1GB エントリ。\n\n💡 CPU が pdpe1gb フラグ対応なら利用可能。",
        ),

        // ── pressure: 残りの PSI フィールド ───────────────────────────
        ("pressure", "cpu_some_avg60") => (
            "CPU 圧力: 一部停滞 (60秒平均)",
            "60秒間で少なくとも1タスクが CPU 待ちの時間割合。",
            "CPU PSI 60秒平均。\n\n💡 avg10 >> avg60 = 直近スパイク。avg10 ≈ avg60 = 持続的。",
        ),
        ("pressure", "cpu_some_avg300") => (
            "CPU 圧力: 一部停滞 (5分平均)",
            "5分間で少なくとも1タスクが CPU 待ちの時間割合。",
            "CPU PSI 5分平均。ベースライン指標。\n\n💡 5% 超が持続 → CPU リソース不足。",
        ),
        ("pressure", "cpu_some_total") => (
            "CPU 圧力: 累積停滞時間 (us)",
            "起動以来の CPU 停滞マイクロ秒累計。",
            "CPU 停滞累計。\n\n💡 スナップショット間のデルタで現在レートを算出。",
        ),
        ("pressure", "memory_some_avg60") => (
            "メモリ圧力: 一部停滞 (60秒平均)",
            "60秒間でメモリ待ち時間の割合。",
            "メモリ PSI 60秒平均。\n\n💡 持続的に非ゼロ → 継続的なメモリ圧迫。",
        ),
        ("pressure", "memory_some_avg300") => (
            "メモリ圧力: 一部停滞 (5分平均)",
            "5分間でメモリ待ち時間の割合。",
            "メモリ PSI 5分平均。\n\n💡 10% 超 → RAM 追加。40% 超 → スラッシング。",
        ),
        ("pressure", "memory_some_total") => (
            "メモリ圧力: 累積停滞時間 (us)",
            "起動以来のメモリ停滞マイクロ秒累計。",
            "メモリ停滞累計 (some)。\n\n💡 デルタで現在レートを算出。",
        ),
        ("pressure", "memory_full_avg10") => (
            "メモリ圧力: 全停滞 (10秒平均)",
            "全タスクがメモリで停滞した時間割合 (10秒)。",
            "メモリ PSI 'full' 10秒平均。全タスク停滞 = 進捗ゼロ。\n\n💡 > 0% → 深刻。> 10% → システムがほぼ機能不全。",
        ),
        ("pressure", "memory_full_avg60") => (
            "メモリ圧力: 全停滞 (60秒平均)",
            "全タスクがメモリで停滞した時間割合 (60秒)。",
            "メモリ PSI 'full' 60秒平均。\n\n💡 持続的に非ゼロ → 危機的。即対処が必要。",
        ),
        ("pressure", "memory_full_avg300") => (
            "メモリ圧力: 全停滞 (5分平均)",
            "全タスクがメモリで停滞した時間割合 (5分)。",
            "メモリ PSI 'full' 5分平均。\n\n💡 長時間非ゼロ → システムが危機的状態。",
        ),
        ("pressure", "memory_full_total") => (
            "メモリ圧力: 全停滞累計 (us)",
            "全タスクがメモリで停滞したマイクロ秒累計。",
            "メモリ全停滞累計。\n\n💡 full ≤ some。高い比率 = 停滞時に全タスクが影響。",
        ),
        ("pressure", "io_some_avg60") => (
            "I/O 圧力: 一部停滞 (60秒平均)",
            "60秒間で I/O 待ち時間の割合。",
            "I/O PSI 60秒平均。\n\n💡 5% 超が持続 → 一貫した I/O ボトルネック。",
        ),
        ("pressure", "io_some_avg300") => (
            "I/O 圧力: 一部停滞 (5分平均)",
            "5分間で I/O 待ち時間の割合。",
            "I/O PSI 5分平均。\n\n💡 20% 超 → SSD アップグレードまたは I/O スケジューラ調整を検討。",
        ),
        ("pressure", "io_some_total") => (
            "I/O 圧力: 累積停滞時間 (us)",
            "起動以来の I/O 停滞マイクロ秒累計。",
            "I/O 停滞累計 (some)。\n\n💡 デルタで現在レートを算出。",
        ),
        ("pressure", "io_full_avg10") => (
            "I/O 圧力: 全停滞 (10秒平均)",
            "全タスクが I/O で停滞した時間割合 (10秒)。",
            "I/O PSI 'full' 10秒平均。全タスクが I/O 待ち。\n\n💡 > 10% → 深刻な I/O バウンド。",
        ),
        ("pressure", "io_full_avg60") => (
            "I/O 圧力: 全停滞 (60秒平均)",
            "全タスクが I/O で停滞した時間割合 (60秒)。",
            "I/O PSI 'full' 60秒平均。\n\n💡 持続 → ストレージが追いつけない。NVMe SSD へアップグレード。",
        ),
        ("pressure", "io_full_avg300") => (
            "I/O 圧力: 全停滞 (5分平均)",
            "全タスクが I/O で停滞した時間割合 (5分)。",
            "I/O PSI 'full' 5分平均。\n\n💡 長期 → 根本的な I/O 容量問題。",
        ),
        ("pressure", "io_full_total") => (
            "I/O 圧力: 全停滞累計 (us)",
            "全タスクが I/O で停滞したマイクロ秒累計。",
            "I/O 全停滞累計。\n\n💡 full/some 比が高い = 単一デバイスのボトルネック。",
        ),

        // ── vmstat: 残りの重要フィールド ──────────────────────────────
        ("vmstat", "nr_zone_inactive_anon") => (
            "ゾーン別非アクティブ匿名ページ",
            "ゾーンごとの非アクティブ匿名ページ数。",
            "ゾーン別非アクティブ匿名ページ。\n\n💡 kswapd のゾーン別回収判断に使用。",
        ),
        ("vmstat", "nr_zone_active_anon") => (
            "ゾーン別アクティブ匿名ページ",
            "ゾーンごとのアクティブ匿名ページ数。",
            "ゾーン別アクティブ匿名ページ。\n\n💡 kswapd のゾーン別回収判断に使用。",
        ),
        ("vmstat", "nr_zone_inactive_file") => (
            "ゾーン別非アクティブファイルページ",
            "ゾーンごとの非アクティブファイルページ数。",
            "ゾーン別非アクティブファイルページ。\n\n💡 各ゾーンで最初に回収される。",
        ),
        ("vmstat", "nr_zone_active_file") => (
            "ゾーン別アクティブファイルページ",
            "ゾーンごとのアクティブファイルページ数。",
            "ゾーン別アクティブファイルページ。\n\n💡 ゾーン別のホットなページキャッシュ。",
        ),
        ("vmstat", "nr_zone_unevictable") => (
            "ゾーン別回収不可ページ",
            "ゾーンごとの回収不可ページ数。",
            "ゾーン別回収不可ページ。\n\n💡 各ゾーンの実効回収可能メモリを減少。",
        ),
        ("vmstat", "nr_zone_write_pending") => (
            "ゾーン別書き込み保留ページ",
            "ゾーンごとのダーティ+ライトバックページ数。",
            "ゾーン別書き込み保留。\n\n💡 高い → そのゾーンに書き込み集中。",
        ),
        ("vmstat", "nr_mlock") => (
            "mlock ロックページ",
            "mlock で固定されたページ数。",
            "mlock ページ。\n\n💡 meminfo の Mlocked と比較。",
        ),
        ("vmstat", "nr_bounce") => (
            "バウンスバッファページ",
            "レガシー DMA 用バウンスバッファ。通常 0。",
            "バウンスバッファ。\n\n💡 64ビットでは 0 が正常。",
        ),
        ("vmstat", "nr_zspages") => (
            "圧縮ページ (zswap/zram)",
            "圧縮メモリプール内のページ。",
            "圧縮スワップページ。\n\n💡 zswap 有効時、圧縮データを RAM に保持。",
        ),
        ("vmstat", "nr_free_cma") => (
            "CMA 空きページ",
            "CMA 領域の空きページ。",
            "CMA 空きページ。\n\n💡 デバイス未使用時は移動可能割り当てに利用可。",
        ),
        ("vmstat", "nr_file_pages") => (
            "ファイルページ合計",
            "ページキャッシュ+スワップキャッシュ+バッファの合計。",
            "ファイルページ合計。\n\n💡 大きい値は健全。",
        ),
        ("vmstat", "nr_shmem_hugepages") => (
            "共有メモリヒュージページ",
            "共有メモリ用ヒュージページ数。",
            "共有メモリヒュージページ。\n\n💡 tmpfs でヒュージページ使用時に非ゼロ。",
        ),
        ("vmstat", "nr_shmem_pmdmapped") => (
            "共有メモリ PMD マップ",
            "PMD レベルでマップされた共有メモリページ。",
            "共有メモリ PMD マップ。\n\n💡 2MB 粒度で実際にマップ。",
        ),
        ("vmstat", "nr_file_hugepages") => (
            "ファイルヒュージページ",
            "ファイル連動メモリ用ヒュージページ。",
            "ファイルヒュージページ。\n\n💡 ファイル連動 THP。",
        ),
        ("vmstat", "nr_file_pmdmapped") => (
            "ファイル PMD マップ",
            "PMD レベルのファイルページ。",
            "ファイル PMD マップ。\n\n💡 ファイルコンテンツのヒュージページマッピング。",
        ),
        ("vmstat", "nr_anon_transparent_hugepages") => (
            "匿名 THP",
            "匿名メモリの THP 数。各 2MB。",
            "匿名 THP。\n\n💡 数 × 2MB = THP メモリ合計。ゼロ → THP 無効か断片化。",
        ),
        ("vmstat", "nr_vmscan_write") => (
            "回収時書き込みページ",
            "vmscan がディスクに書き込んだページ。",
            "vmscan 書き込み。\n\n💡 クリーンページ枯渇を示す。",
        ),
        ("vmstat", "nr_vmscan_immediate_reclaim") => (
            "即時回収ページ",
            "LRU エージングをバイパスして即回収。",
            "即時回収。\n\n💡 高い → 深刻なメモリ圧迫下の積極的回収。",
        ),
        ("vmstat", "nr_dirtied") => (
            "起動以来のダーティページ合計",
            "累積ダーティ化ページ数。",
            "ダーティ化合計。\n\n💡 デルタで現在の汚染レートを把握。",
        ),
        ("vmstat", "nr_written") => (
            "起動以来の書き込みページ合計",
            "累積書き込みページ数。",
            "書き込み合計。\n\n💡 nr_dirtied - nr_written = ダーティバックログ。",
        ),
        ("vmstat", "nr_kernel_stack") => (
            "カーネルスタックページ",
            "カーネルスレッドスタック用ページ。",
            "カーネルスタック。\n\n💡 スタックあたりページ数で割ってスレッド数を推定。",
        ),
        ("vmstat", "nr_page_table_pages") => (
            "ページテーブルページ",
            "ページテーブルエントリ用ページ。",
            "ページテーブルページ。\n\n💡 meminfo の PageTables と比較。",
        ),
        ("vmstat", "nr_swapcached") => (
            "スワップキャッシュページ",
            "RAM とスワップ両方にあるページ。",
            "スワップキャッシュ。\n\n💡 メモリ圧迫後に非ゼロ。",
        ),
        ("vmstat", "nr_dirty_threshold") => (
            "ダーティページ閾値",
            "ライターがスロットルされるダーティページ数。",
            "ダーティスロットル閾値。\n\n💡 vm.dirty_ratio で調整。",
        ),
        ("vmstat", "nr_dirty_background_threshold") => (
            "バックグラウンドダーティ閾値",
            "バックグラウンドライトバック開始閾値。",
            "バックグラウンドライトバック閾値。\n\n💡 vm.dirty_background_ratio で調整。",
        ),
        ("vmstat", "workingset_nodes") => (
            "ワーキングセットシャドウノード",
            "退避ページアクセスパターン追跡ノード。",
            "シャドウノード。\n\n💡 カーネルの適応的 LRU バランシングに使用。",
        ),
        ("vmstat", "workingset_refault_anon") => (
            "匿名ワーキングセットリフォルト",
            "退避された匿名ページへの再アクセス。",
            "匿名リフォルト。\n\n💡 高レート → ワーキングセットが RAM を超過。スラッシング。",
        ),
        ("vmstat", "workingset_refault_file") => (
            "ファイルワーキングセットリフォルト",
            "退避されたファイルページのディスク再読み込み。",
            "ファイルリフォルト。\n\n💡 高レート → ページキャッシュ不足。",
        ),
        ("vmstat", "workingset_activate_anon") => (
            "匿名ワーキングセット活性化",
            "リフォルト検出で匿名ページをアクティブリストに昇格。",
            "匿名活性化。\n\n💡 再退避を防止。",
        ),
        ("vmstat", "workingset_activate_file") => (
            "ファイルワーキングセット活性化",
            "リフォルト検出でファイルページをアクティブリストに昇格。",
            "ファイル活性化。\n\n💡 頻繁アクセスページを保護。",
        ),
        ("vmstat", "workingset_restore_anon") => (
            "匿名ワーキングセット復元",
            "匿名ページのアクティブ状態復元。",
            "匿名復元。\n\n💡 高い → 匿名ワーキングセットの変動が激しい。",
        ),
        ("vmstat", "workingset_restore_file") => (
            "ファイルワーキングセット復元",
            "ファイルページのアクティブ状態復元。",
            "ファイル復元。\n\n💡 高い → ページキャッシュが圧迫下。",
        ),
        ("vmstat", "workingset_nodereclaim") => (
            "シャドウノード回収",
            "メモリ圧迫でシャドウノードを回収。",
            "シャドウノード回収。\n\n💡 深刻な圧迫でワーキングセット追跡データを喪失。",
        ),
        ("vmstat", "numa_hit") => (
            "NUMA 意図ノード割り当て",
            "意図した NUMA ノードでの割り当て。",
            "NUMA ヒット。\n\n💡 hit / (hit + miss) = ローカリティ率。100% に近いほど良い。",
        ),
        ("vmstat", "numa_miss") => (
            "NUMA 非意図ノード割り当て",
            "意図しない NUMA ノードでの割り当て。",
            "NUMA ミス。\n\n💡 高い → リモートメモリレイテンシ。numactl --membind を使用。",
        ),
        ("vmstat", "numa_foreign") => (
            "NUMA 外部割り当て",
            "このノード向けだが別ノードに配置された割り当て。",
            "NUMA 外部。\n\n💡 他ノードの numa_miss の対。",
        ),
        ("vmstat", "numa_local") => (
            "NUMA ローカル割り当て",
            "CPU のローカル NUMA ノードでの割り当て。",
            "NUMA ローカル。\n\n💡 高いほどパフォーマンスが良い。",
        ),
        ("vmstat", "numa_other") => (
            "NUMA リモート割り当て",
            "リモート NUMA ノードでの割り当て。",
            "NUMA リモート。\n\n💡 高い → メモリアフィニティが悪い。",
        ),
        ("vmstat", "pgfree") => (
            "解放ページ数",
            "起動以来の解放ページ合計。",
            "解放ページ。\n\n💡 pgalloc_* と連動するはず。",
        ),
        ("vmstat", "pgactivate") => (
            "アクティブリスト昇格ページ",
            "アクセスにより非アクティブ→アクティブに昇格。",
            "ページ活性化。\n\n💡 健全なワーキングセットサイクル。",
        ),
        ("vmstat", "pgdeactivate") => (
            "非アクティブリスト降格ページ",
            "回収時にアクティブ→非アクティブに降格。",
            "ページ非活性化。\n\n💡 レート増加 → メモリ圧迫。",
        ),
        ("vmstat", "pglazyfree") => (
            "遅延解放マークページ",
            "MADV_FREE でマークされたが未回収のページ。",
            "遅延解放ページ。\n\n💡 メモリ圧迫時のみカーネルが回収。",
        ),
        ("vmstat", "pglazyfreed") => (
            "実際に遅延解放されたページ",
            "MADV_FREE ページがカーネルにより回収。",
            "遅延解放済みページ。\n\n💡 カーネルがメモリを必要とした結果。",
        ),
        ("vmstat", "pgrefill") => (
            "LRU 補充スキャンページ",
            "アクティブ→非アクティブの補充時にスキャン。",
            "LRU 補充スキャン。\n\n💡 高レート → 活発なメモリ回収エージング。",
        ),
        ("vmstat", "pgreuse") => (
            "高速パス再利用ページ",
            "完全な割り当てパスを経ずに再利用。",
            "高速パス再利用。\n\n💡 高いほど良い。",
        ),
        ("vmstat", "pgsteal_kswapd") => (
            "kswapd 回収ページ",
            "バックグラウンド回収。プロセスをブロックしない。",
            "kswapd 回収。\n\n💡 kswapd 高 + direct 低 = 健全な回収。",
        ),
        ("vmstat", "pgsteal_direct") => (
            "ダイレクト回収ページ",
            "ブロッキング回収。確保プロセスが待機。",
            "ダイレクト回収。\n\n💡 高い → kswapd が追いつけない。RAM 追加を検討。",
        ),
        ("vmstat", "pgscan_kswapd") => (
            "kswapd スキャンページ",
            "バックグラウンド回収でスキャン。",
            "kswapd スキャン。\n\n💡 スキャン - 回収 = 回収不可ページ。",
        ),
        ("vmstat", "pgscan_direct") => (
            "ダイレクト回収スキャンページ",
            "ブロッキング回収でスキャン。",
            "ダイレクト回収スキャン。\n\n💡 高いスキャン/回収比 → 非効率な回収。",
        ),
        ("vmstat", "pgscan_direct_throttle") => (
            "ダイレクト回収スロットル",
            "CPU 過負荷防止でスロットル。",
            "回収スロットル。\n\n💡 非常に重いメモリ圧迫。",
        ),
        ("vmstat", "pgscan_anon") => (
            "匿名ページスキャン",
            "回収候補としてスキャンされた匿名ページ。",
            "匿名スキャン。\n\n💡 コストが高い — スワップアウトが必要。",
        ),
        ("vmstat", "pgscan_file") => (
            "ファイルページスキャン",
            "回収候補としてスキャンされたファイルページ。",
            "ファイルスキャン。\n\n💡 匿名回収より低コスト。",
        ),
        ("vmstat", "pgsteal_anon") => (
            "匿名ページ回収",
            "スワップアウトされた匿名ページ。",
            "匿名回収。\n\n💡 > 0 → アクティブにスワッピング中。",
        ),
        ("vmstat", "pgsteal_file") => (
            "ファイルページ回収",
            "ドロップまたはライトバックされたファイルページ。",
            "ファイル回収。\n\n💡 健全な回収パス。",
        ),
        ("vmstat", "zone_reclaim_failed") => (
            "ゾーン回収失敗",
            "ゾーン回収でページ不足。",
            "ゾーン回収失敗。\n\n💡 NUMA ゾーンの圧迫。",
        ),
        ("vmstat", "pginodesteal") => (
            "inode 回収によるページ解放",
            "inode 退避でページも解放。",
            "inode スチール。\n\n💡 inode 退避で関連ページキャッシュも解放。",
        ),
        ("vmstat", "kswapd_inodesteal") => (
            "kswapd inode 回収",
            "バックグラウンド回収で inode を退避。",
            "kswapd inode 回収。\n\n💡 inode メモリと関連ページキャッシュを解放。",
        ),
        ("vmstat", "kswapd_low_wmark_hit_quickly") => (
            "kswapd low ウォーターマーク即到達",
            "kswapd 完了後すぐに low ウォーターマークに到達。",
            "low 即到達。\n\n💡 kswapd の回収量が不足。",
        ),
        ("vmstat", "kswapd_high_wmark_hit_quickly") => (
            "kswapd high ウォーターマーク即到達",
            "kswapd が素早く high ウォーターマークを回復。",
            "high 即到達。\n\n💡 健全 — kswapd が需要に追いついている。",
        ),
        ("vmstat", "pageoutrun") => (
            "kswapd 起動回数",
            "kswapd がページアウトのために起動された回数。",
            "kswapd 起動。\n\n💡 増加レート → 頻繁なメモリ圧迫。",
        ),
        ("vmstat", "pgrotated") => (
            "LRU 末尾ローテーションページ",
            "回収時にダーティだったページを LRU 末尾に移動。",
            "LRU ローテーション。\n\n💡 回収時に多数のダーティページに遭遇。",
        ),
        ("vmstat", "drop_pagecache") => (
            "ページキャッシュドロップ",
            "手動のページキャッシュ破棄イベント。",
            "ページキャッシュドロップ。\n\n💡 drop_caches で発動。本番では有害。",
        ),
        ("vmstat", "drop_slab") => (
            "スラブキャッシュドロップ",
            "手動のスラブキャッシュ破棄イベント。",
            "スラブドロップ。\n\n💡 drop_caches で発動。",
        ),
        ("vmstat", "pgmigrate_success") => (
            "ページ移行成功",
            "NUMA バランシングやコンパクションで移行成功。",
            "移行成功。\n\n💡 成功率が高いほど良い。",
        ),
        ("vmstat", "pgmigrate_fail") => (
            "ページ移行失敗",
            "移行できなかったページ。",
            "移行失敗。\n\n💡 高い → 固定ページが多い。",
        ),
        ("vmstat", "thp_fault_alloc") => (
            "THP フォルト割り当て",
            "ページフォルトで THP (2MB) を割り当て。",
            "THP フォルト割り当て。\n\n💡 高いほど良い。",
        ),
        ("vmstat", "thp_fault_fallback") => (
            "THP フォルトフォールバック",
            "THP 割り当て失敗で 4KB にフォールバック。",
            "THP フォールバック。\n\n💡 高い → 断片化で THP 割り当て不可。",
        ),
        ("vmstat", "thp_collapse_alloc") => (
            "THP 折りたたみ割り当て",
            "khugepaged がベースページを THP に統合。",
            "khugepaged 統合。\n\n💡 512 ベースページを 2MB THP に統合。",
        ),
        ("vmstat", "thp_collapse_alloc_failed") => (
            "THP 折りたたみ失敗",
            "khugepaged 統合の割り当て失敗。",
            "統合失敗。\n\n💡 断片化で THP 形成不可。",
        ),
        ("vmstat", "thp_split_page") => (
            "THP ページ分割",
            "THP を 512 ベースページに分割。",
            "THP 分割。\n\n💡 高い + 高い alloc → THP の恩恵が少ないワークロード。",
        ),
        ("vmstat", "thp_split_pmd") => (
            "THP PMD 分割",
            "PMD エントリを 2MB→4KB マッピングに分割。",
            "THP PMD 分割。\n\n💡 部分的 unmap や mprotect で発生。",
        ),
        ("vmstat", "thp_zero_page_alloc") => (
            "THP ゼロページ割り当て",
            "未初期化メモリ用の共有ゼロページ。",
            "THP ゼロページ。\n\n💡 共有読み取り専用でメモリ節約。",
        ),
        ("vmstat", "thp_swpout") => (
            "THP 一括スワップアウト",
            "THP を 2MB 単位でスワップアウト。",
            "THP スワップアウト。\n\n💡 分割より効率的。",
        ),
        ("vmstat", "thp_swpout_fallback") => (
            "THP スワップアウトフォールバック",
            "THP を分割してからスワップアウト。",
            "THP スワップアウトフォールバック。\n\n💡 分割が必要で非効率。",
        ),
        ("vmstat", "compact_stall") => (
            "コンパクション停滞",
            "メモリコンパクションでプロセスが停滞。",
            "コンパクション停滞。\n\n💡 高い → 断片化が割り当て遅延を引き起こす。",
        ),
        ("vmstat", "compact_fail") => (
            "コンパクション失敗",
            "連続ブロック作成に失敗。",
            "コンパクション失敗。\n\n💡 高い → 深刻な断片化。",
        ),
        ("vmstat", "compact_success") => (
            "コンパクション成功",
            "連続ブロック作成に成功。",
            "コンパクション成功。\n\n💡 success/(success+fail) = 成功率。",
        ),
        ("vmstat", "compact_daemon_wake") => (
            "コンパクションデーモン起動",
            "kcompactd のプロアクティブコンパクション起動。",
            "kcompactd 起動。\n\n💡 compaction_proactiveness で調整可能。",
        ),
        ("vmstat", "compact_migrate_scanned") => (
            "コンパクション移行スキャン",
            "移動可能ページを探してスキャン。",
            "移行スキャン。\n\n💡 高い + 低成功 → 移動可能ページが少ない。",
        ),
        ("vmstat", "compact_free_scanned") => (
            "コンパクション空き領域スキャン",
            "空きターゲットページを探してスキャン。",
            "空き領域スキャン。\n\n💡 多いスキャン = より断片化。",
        ),
        ("vmstat", "swap_ra") => (
            "スワップ先読みページ",
            "投機的にスワップから先読み。",
            "スワップ先読み。\n\n💡 シーケンシャルアクセスで効果的。",
        ),
        ("vmstat", "swap_ra_hit") => (
            "スワップ先読みヒット",
            "先読みページの実際の使用。",
            "先読みヒット。\n\n💡 hit/ra = ヒット率。低い → ランダムアクセスで I/O が無駄。",
        ),
        ("vmstat", "balloon_inflate") => (
            "バルーンインフレート (VM)",
            "ハイパーバイザにメモリ返却。",
            "バルーンインフレーション。\n\n💡 VM がホストにメモリを返却中。",
        ),
        ("vmstat", "balloon_deflate") => (
            "バルーンデフレート (VM)",
            "ハイパーバイザからメモリ取得。",
            "バルーンデフレーション。\n\n💡 VM がメモリを取り戻し中。",
        ),
        ("vmstat", "unevictable_pgs_culled") => (
            "回収不可ページ選別",
            "回収不可と判定され LRU から移動。",
            "選別。\n\n💡 mlock や ramfs で回収不可と判定。",
        ),
        ("vmstat", "unevictable_pgs_scanned") => (
            "回収不可ページスキャン",
            "LRU スキャン中に遭遇した回収不可ページ。",
            "回収不可スキャン。\n\n💡 回収不可ページのスキャンは無駄な作業。",
        ),
        ("vmstat", "unevictable_pgs_rescued") => (
            "回収不可ページ救出",
            "回収不可リストから通常 LRU に復帰。",
            "ページ救出。\n\n💡 munlock 後に LRU エージングに復帰。",
        ),
        ("vmstat", "unevictable_pgs_mlocked") => (
            "mlock 化ページ",
            "mlock() で回収不可にされたページ。",
            "mlock 化。\n\n💡 メモリロックのレートを追跡。",
        ),
        ("vmstat", "unevictable_pgs_munlocked") => (
            "munlock 化ページ",
            "munlock() で通常 LRU に復帰したページ。",
            "munlock 化。\n\n💡 回収対象に復帰。",
        ),
        ("vmstat", "direct_map_level2_splits") => (
            "直接マップ 2MB 分割",
            "2MB エントリを 4KB に分割。",
            "2MB ページ分割。\n\n💡 ページ属性変更で発生。TLB 圧力増加。",
        ),
        ("vmstat", "direct_map_level3_splits") => (
            "直接マップ 1GB 分割",
            "1GB エントリを 2MB に分割。",
            "1GB ページ分割。\n\n💡 より大きな TLB への影響。",
        ),

        // ── net/snmp — 残りの IP フィールド ──────────────────────────
        ("net/snmp", "Ip_DefaultTTL") => (
            "デフォルト IP TTL",
            "送信 IP パケットに設定されるデフォルト TTL 値。通常 64。",
            "送信パケットのデフォルト IP TTL。\n\n💡 診断: 標準値は 64。低い値は遠いホストに到達できなくなる。",
        ),
        ("net/snmp", "Ip_InUnknownProtos") => (
            "未知プロトコルの IP パケット",
            "上位プロトコルが不明またはサポートされていないため破棄されたパケット。",
            "未対応プロトコル番号の IP パケット。\n\n💡 診断: 0 であるべき。非ゼロはカスタムプロトコルか攻撃の可能性。",
        ),
        ("net/snmp", "Ip_InDiscards") => (
            "受信 IP パケット破棄",
            "有効だが破棄されたパケット（バッファ不足等）。エラーとしてはカウントされない。",
            "受信 IP パケット破棄（有効だが破棄）。\n\n💡 診断: 非ゼロ → リソース枯渇（メモリ、バッファ）。システムメモリ圧力を確認。",
        ),
        ("net/snmp", "Ip_ReasmTimeout") => (
            "IP 再組み立てタイムアウト",
            "フラグメント再組み立てがタイムアウト。欠けたフラグメントによりデータグラム全体が破棄。",
            "IP フラグメント再組み立てタイムアウト。\n\n💡 診断: 非ゼロ → フラグメントが到着するが全ピースが時間内に揃わない。FW がフラグメントをブロックしている可能性。",
        ),
        ("net/snmp", "Ip_ReasmReqds") => (
            "再組み立てが必要な IP フラグメント",
            "完全なデータグラムに再組み立てが必要な IP フラグメントの受信数。",
            "再組み立てが必要な IP フラグメント。\n\n💡 診断: PMTUD が機能する現代のネットワークでは 0 に近いはず。非ゼロ → 経路上の MTU 不一致。",
        ),
        ("net/snmp", "Ip_ReasmOKs") => (
            "IP データグラム再組み立て成功",
            "フラグメントから正常に再構成された IP データグラム数。",
            "IP フラグメント再組み立て成功。\n\n💡 診断: ReasmReqds と ReasmFails と比較して成功率を確認。",
        ),
        ("net/snmp", "Ip_FragOKs") => (
            "IP データグラム断片化成功",
            "送信のために正常に断片化された IP データグラム数。",
            "IP 断片化成功。\n\n💡 診断: 非ゼロ → このホストが送信パケットを断片化中。MTU 増加か PMTUD 有効化を検討。",
        ),
        ("net/snmp", "Ip_FragFails") => (
            "IP 断片化失敗（DF ビット設定）",
            "断片化が必要だが Don't Fragment フラグが設定されていた IP データグラム。",
            "IP 断片化失敗 — DF ビットにより断片化不可。\n\n💡 診断: 非ゼロ → 経路 MTU に対してパケットが大きすぎ、DF が設定済み。PMTUD が ICMP 'fragmentation needed' を返すはず。",
        ),
        ("net/snmp", "Ip_FragCreates") => (
            "IP フラグメント生成数",
            "大きなデータグラムの断片化により生成された IP フラグメントの総数。",
            "IP フラグメント生成。\n\n💡 診断: 各断片化データグラムが複数のフラグメントを生成。高い値は大きな断片化オーバーヘッドを示す。",
        ),

        // ── net/snmp — 残りの ICMP フィールド ────────────────────────
        ("net/snmp", "Icmp_InCsumErrors") => (
            "ICMP チェックサムエラー",
            "チェックサムが無効な ICMP メッセージの受信数。データ破損を示す。",
            "ICMP チェックサムエラー。\n\n💡 診断: 0 であるべき。非ゼロ → リンクレベルの破損。",
        ),
        ("net/snmp", "Icmp_InTimeExcds") => (
            "ICMP 時間超過受信",
            "traceroute や TTL 期限切れによる ICMP 時間超過メッセージ受信数。",
            "ICMP 時間超過メッセージ受信。\n\n💡 診断: traceroute 中は正常。traceroute なしで高レート → ルーティングループの可能性。",
        ),
        ("net/snmp", "Icmp_InParmProbs") => (
            "ICMP パラメータ問題受信",
            "リモートホストが検出した不正 IP ヘッダを示す ICMP パラメータ問題メッセージ。",
            "ICMP パラメータ問題受信。\n\n💡 診断: 0 であるべき。非ゼロ → このホストが不正パケットを送信中。NIC オフロード設定を確認。",
        ),
        ("net/snmp", "Icmp_InSrcQuenchs") => (
            "ICMP ソースクエンチ受信（非推奨）",
            "受信した ICMP ソースクエンチメッセージ。非推奨の輻輳通知メカニズム。",
            "ICMP ソースクエンチ受信。\n\n💡 診断: RFC 6633 で非推奨。現代のシステムでは無視される。0 であるべき。",
        ),
        ("net/snmp", "Icmp_InRedirects") => (
            "ICMP リダイレクト受信",
            "より良い経路が存在することを示す ICMP リダイレクトメッセージの受信数。",
            "ICMP リダイレクト受信。\n\n💡 診断: 非ゼロ → ルーターがこのホストに別のゲートウェイを使うよう指示。最適でないルーティングか ICMP リダイレクト攻撃の可能性。",
        ),
        ("net/snmp", "Icmp_InEchos") => (
            "ICMP エコー要求受信（ping）",
            "受信した ping 要求数。フィルタされなければエコー応答をトリガー。",
            "ICMP エコー要求（ping）受信。\n\n💡 診断: 高レート → 頻繁な ping、または ping フラッド攻撃の可能性。",
        ),
        ("net/snmp", "Icmp_InEchoReps") => (
            "ICMP エコー応答受信",
            "このホストが送信したエコー要求に対する ping 応答の受信数。",
            "ICMP エコー応答受信。\n\n💡 診断: 送信した ping と相関するはず。応答 > 要求 → 未要求の応答（異常）。",
        ),
        ("net/snmp", "Icmp_InTimestamps") => (
            "ICMP タイムスタンプ要求受信",
            "ICMP タイムスタンプ要求メッセージの受信数。現代のネットワークではまれ。",
            "ICMP タイムスタンプ要求受信。\n\n💡 診断: 現代のネットワークでは 0 であるべき。非ゼロは偵察プロービングの可能性。",
        ),
        ("net/snmp", "Icmp_InTimestampReps") => (
            "ICMP タイムスタンプ応答受信",
            "ICMP タイムスタンプ応答メッセージの受信数。",
            "ICMP タイムスタンプ応答受信。\n\n💡 診断: まれ。時刻同期プロービングに使用。",
        ),
        ("net/snmp", "Icmp_InAddrMasks") => (
            "ICMP アドレスマスク要求受信",
            "ICMP アドレスマスク要求メッセージの受信数。廃止済み。",
            "ICMP アドレスマスク要求。\n\n💡 診断: 廃止済み。0 であるべき。非ゼロ → 非常に古い機器かプロービング。",
        ),
        ("net/snmp", "Icmp_InAddrMaskReps") => (
            "ICMP アドレスマスク応答受信",
            "ICMP アドレスマスク応答メッセージの受信数。廃止済み。",
            "ICMP アドレスマスク応答。\n\n💡 診断: 廃止済み。0 であるべき。",
        ),
        ("net/snmp", "Icmp_OutErrors") => (
            "ICMP 送信エラー",
            "エラーにより送信できなかった ICMP メッセージ数。",
            "ICMP 出力エラー。\n\n💡 診断: 非ゼロ → カーネルが ICMP 応答を送信できなかった。レート制限（net.ipv4.icmp_ratelimit）を確認。",
        ),
        ("net/snmp", "Icmp_OutDestUnreachs") => (
            "ICMP 宛先到達不能送信",
            "閉じたポートや到達不可の宛先へのトラフィックに応答して送信された ICMP 宛先到達不能メッセージ。",
            "ICMP 宛先到達不能送信。\n\n💡 診断: 高レート → 閉じたポートへのパケットが多い（ポートスキャンまたは設定ミスのクライアント）。",
        ),
        ("net/snmp", "Icmp_OutTimeExcds") => (
            "ICMP 時間超過送信",
            "転送中に TTL がゼロに達した際に送信された ICMP 時間超過メッセージ。",
            "ICMP 時間超過送信。\n\n💡 診断: ホストがパケットを転送している場合のみ非ゼロ。転送パケットの TTL が不足。",
        ),
        ("net/snmp", "Icmp_OutParmProbs") => (
            "ICMP パラメータ問題送信",
            "不正ヘッダのパケットに対して送信された ICMP パラメータ問題メッセージ。",
            "ICMP パラメータ問題送信。\n\n💡 診断: 0 であるべき。非ゼロ → ピアから不正 IP パケットを受信中。",
        ),
        ("net/snmp", "Icmp_OutSrcQuenchs") => (
            "ICMP ソースクエンチ送信（非推奨）",
            "送信された ICMP ソースクエンチメッセージ。非推奨で生成されるべきでない。",
            "ICMP ソースクエンチ送信。\n\n💡 診断: 非推奨。現代のカーネルでは常に 0。",
        ),
        ("net/snmp", "Icmp_OutRedirects") => (
            "ICMP リダイレクト送信",
            "ホストにより良い経路を通知するために送信された ICMP リダイレクトメッセージ。",
            "ICMP リダイレクト送信。\n\n💡 診断: 非ゼロ → このホストがルーターとして動作しトラフィックをリダイレクト中。ゲートウェイでは正常。",
        ),
        ("net/snmp", "Icmp_OutEchos") => (
            "ICMP エコー要求送信（ping）",
            "このホストが送信した ping 要求数。",
            "ICMP エコー要求（ping）送信。\n\n💡 診断: 送信 ping 活動に対応。監視ツールがこれらを生成することが多い。",
        ),
        ("net/snmp", "Icmp_OutEchoReps") => (
            "ICMP エコー応答送信",
            "受信エコー要求に対して送信された ping 応答数。",
            "ICMP エコー応答送信。\n\n💡 診断: InEchos とほぼ一致するはず。差異 → 一部の ping がフィルタされている。",
        ),
        ("net/snmp", "Icmp_OutTimestamps") => (
            "ICMP タイムスタンプ要求送信",
            "送信された ICMP タイムスタンプ要求メッセージ。まれ。",
            "ICMP タイムスタンプ要求送信。\n\n💡 診断: タイムスタンププロービング使用ツールがない限り 0 であるべき。",
        ),
        ("net/snmp", "Icmp_OutTimestampReps") => (
            "ICMP タイムスタンプ応答送信",
            "送信された ICMP タイムスタンプ応答メッセージ。",
            "ICMP タイムスタンプ応答送信。\n\n💡 診断: フィルタされなければ InTimestamps と一致。",
        ),
        ("net/snmp", "Icmp_OutAddrMasks") => (
            "ICMP アドレスマスク要求送信",
            "送信された ICMP アドレスマスク要求メッセージ。廃止済み。",
            "ICMP アドレスマスク要求送信。\n\n💡 診断: 廃止済み。0 であるべき。",
        ),
        ("net/snmp", "Icmp_OutAddrMaskReps") => (
            "ICMP アドレスマスク応答送信",
            "送信された ICMP アドレスマスク応答メッセージ。廃止済み。",
            "ICMP アドレスマスク応答送信。\n\n💡 診断: 廃止済み。0 であるべき。",
        ),

        // ── net/snmp — 残りの TCP フィールド ─────────────────────────
        ("net/snmp", "Tcp_RtoAlgorithm") => (
            "TCP RTO アルゴリズム",
            "使用中の再送タイムアウトアルゴリズム。4 = Van Jacobson（標準）。",
            "TCP 再送タイムアウトアルゴリズム。\n\n💡 診断: 4 = Van Jacobson アルゴリズム（RFC 6298）。現代の Linux では標準。",
        ),
        ("net/snmp", "Tcp_RtoMin") => (
            "TCP 最小 RTO（ms）",
            "最小再送タイムアウト（ミリ秒）。低いほど再送が速い。",
            "TCP 最小再送タイムアウト。\n\n💡 診断: デフォルト 200ms。低い → 再送が速いが、ジッタのあるネットワークでは不要な再送が増える。",
        ),
        ("net/snmp", "Tcp_RtoMax") => (
            "TCP 最大 RTO（ms）",
            "最大再送タイムアウト（ミリ秒）。TCP が再送まで待つ上限。",
            "TCP 最大再送タイムアウト。\n\n💡 診断: デフォルト 120000ms（120秒）。非常に長い — 長時間の障害でも TCP が耐えられる。",
        ),
        ("net/snmp", "Tcp_MaxConn") => (
            "TCP 最大接続数",
            "許容される最大 TCP 接続数。-1 はカーネルが動的に決定。",
            "TCP 最大接続数。\n\n💡 診断: -1 = 固定制限なし（カーネルが動的管理）。これが標準値。",
        ),

        // ── net/snmp — 残りの UDP フィールド ─────────────────────────
        ("net/snmp", "Udp_IgnoredMulti") => (
            "無視された UDP マルチキャスト",
            "受信したが無視されたマルチキャスト UDP データグラム（マルチキャストグループ未参加）。",
            "無視された UDP マルチキャスト。\n\n💡 診断: マルチキャストトラフィックがあるネットワークでは正常。",
        ),
        ("net/snmp", "Udp_MemErrors") => (
            "UDP メモリ割り当て失敗",
            "カーネルのメモリ割り当て失敗により破棄された UDP データグラム。",
            "UDP メモリ割り当て失敗。\n\n💡 診断: 非ゼロ → 深刻なメモリ圧迫がネットワークスタックに影響。システムメモリと TCP/UDP メモリ制限を確認。",
        ),

        // ── net/snmp — UdpLite フィールド ────────────────────────────
        ("net/snmp", "UdpLite_InDatagrams") => (
            "UDP-Lite データグラム受信",
            "アプリケーションに配信された UDP-Lite データグラム数。UDP-Lite は部分チェックサムを許容。",
            "UDP-Lite データグラム受信。\n\n💡 診断: 通常 0。UDP-Lite は部分データが無データより良いエラー耐性メディアストリームに使用。",
        ),
        ("net/snmp", "UdpLite_NoPorts") => (
            "UDP-Lite 閉ポートへのパケット",
            "リスンするプロセスのないポートに受信した UDP-Lite データグラム。",
            "リスナーなしの UDP-Lite データグラム。\n\n💡 診断: UDP-Lite サービスが予定されていない限り 0 であるべき。",
        ),
        ("net/snmp", "UdpLite_InErrors") => (
            "UDP-Lite 入力エラー",
            "何らかの理由で配信できなかった UDP-Lite データグラム。",
            "UDP-Lite 入力エラー。\n\n💡 診断: UDP-Lite を使用していないシステムでは 0 であるべき。",
        ),
        ("net/snmp", "UdpLite_OutDatagrams") => (
            "UDP-Lite データグラム送信",
            "このホストが送信した UDP-Lite データグラム。",
            "UDP-Lite データグラム送信。\n\n💡 診断: 非ゼロ → アプリケーションが UDP-Lite を使用中（まれ）。",
        ),
        ("net/snmp", "UdpLite_RcvbufErrors") => (
            "UDP-Lite 受信バッファオーバーフロー",
            "受信バッファ満杯により破棄された UDP-Lite データグラム。",
            "UDP-Lite 受信バッファドロップ。\n\n💡 診断: UDP RcvbufErrors と同じ影響 — アプリの読み取りが遅い。",
        ),
        ("net/snmp", "UdpLite_SndbufErrors") => (
            "UDP-Lite 送信バッファオーバーフロー",
            "送信バッファ満杯により破棄された UDP-Lite データグラム。",
            "UDP-Lite 送信バッファドロップ。\n\n💡 診断: アプリが NIC の送信能力を超えて送信中。",
        ),
        ("net/snmp", "UdpLite_InCsumErrors") => (
            "UDP-Lite チェックサムエラー",
            "カバー部分にチェックサムエラーがある UDP-Lite データグラム。",
            "UDP-Lite チェックサムエラー。\n\n💡 診断: UDP-Lite はデータグラムの一部のみチェックサム計算; これはそのカバー部分のエラー数。",
        ),
        ("net/snmp", "UdpLite_IgnoredMulti") => (
            "UDP-Lite マルチキャスト無視",
            "受信したが無視された UDP-Lite マルチキャストデータグラム。",
            "無視された UDP-Lite マルチキャスト。\n\n💡 診断: UDP IgnoredMulti と同様。0 であるべき。",
        ),
        ("net/snmp", "UdpLite_MemErrors") => (
            "UDP-Lite メモリ割り当て失敗",
            "メモリ割り当て失敗により破棄された UDP-Lite データグラム。",
            "UDP-Lite メモリエラー。\n\n💡 診断: 非ゼロ → 深刻なメモリ圧迫。UDP MemErrors と同じ対処法。",
        ),

        // ── net/netstat — 残りの TcpExt フィールド ───────────────────
        ("net/netstat", "TcpExt_EmbryonicRsts") => (
            "胚接続への RST",
            "SYN_RECV 状態（ハーフオープン）の接続に受信した RST セグメント。",
            "胚（SYN_RECV）接続への RST。\n\n💡 診断: 非ゼロ → ハンドシェイク中にクライアントが接続を中断。ポートスキャンまたはロードバランサのヘルスチェック。",
        ),
        ("net/netstat", "TcpExt_PruneCalled") => (
            "ソケットバッファプルーン呼び出し",
            "メモリ消費を削減するためカーネルがソケットバッファメモリをプルーニングした回数。",
            "ソケットバッファプルーニング。\n\n💡 診断: 非ゼロ → TCP メモリ圧迫。カーネルがソケットバッファを縮小中。",
        ),
        ("net/netstat", "TcpExt_RcvPruned") => (
            "受信キューからプルーニングされたパケット",
            "メモリ圧迫により受信キューから破棄されたパケット。",
            "受信キュープルーニング。\n\n💡 診断: 非ゼロ → アプリの読み取りが追いつかず、かつシステムがメモリ圧迫下。データロス。",
        ),
        ("net/netstat", "TcpExt_OfoPruned") => (
            "順序外パケットのプルーニング",
            "メモリ圧迫により順序外キューから破棄されたパケット。",
            "順序外キュープルーニング。\n\n💡 診断: 非ゼロ → 深刻なメモリ圧迫で並べ替えデータを喪失。再送が続く。",
        ),
        ("net/netstat", "TcpExt_OutOfWindowIcmps") => (
            "ウィンドウ外 ICMP のドロップ",
            "TCP セグメントがウィンドウ外を参照していたため破棄された ICMP メッセージ。",
            "ウィンドウ外 ICMP ドロップ。\n\n💡 診断: ICMP ベースのブラインド攻撃を防ぐセキュリティ機能。若干あるのは正常。",
        ),
        ("net/netstat", "TcpExt_LockDroppedIcmps") => (
            "ソケットロック中 ICMP ドロップ",
            "ターゲットソケットがロックされていたため破棄された ICMP メッセージ。",
            "ロックソケット ICMP ドロップ。\n\n💡 診断: 一時的な状態 — ソケットがビジー。通常無害。",
        ),
        ("net/netstat", "TcpExt_ArpFilter") => (
            "ARP フィルタによるパケットフィルタ",
            "カーネル ARP フィルタメカニズムによりフィルタされたパケット。",
            "ARP フィルタドロップ。\n\n💡 診断: 非ゼロ → ARP フィルタリングがアクティブ（arp_filter sysctl）。マルチホームホストでは想定内。",
        ),
        ("net/netstat", "TcpExt_TWRecycled") => (
            "タイムスタンプによる TIME_WAIT リサイクル",
            "TCP タイムスタンプ検証により早期に再利用された TIME_WAIT ソケット。",
            "タイムスタンプによる TIME_WAIT リサイクル。\n\n💡 診断: tcp_tw_recycle で有効化（カーネル 4.12 で非推奨）。現代のカーネルでは 0 であるべき。",
        ),
        ("net/netstat", "TcpExt_TWKilled") => (
            "TIME_WAIT ソケット強制破棄",
            "自然な期限切れ前に強制的に破棄された TIME_WAIT ソケット。",
            "TIME_WAIT ソケット強制破棄。\n\n💡 診断: TIME_WAIT バケット圧力を示す可能性。",
        ),
        ("net/netstat", "TcpExt_PAWSActive") => (
            "PAWS によるアクティブ接続拒否",
            "アクティブオープン時に PAWS（Wrapped Sequences 防止）により拒否された接続試行。",
            "アクティブオープンでの PAWS 拒否。\n\n💡 診断: 非ゼロ → TIME_WAIT 内の古いタイムスタンプの接続が同一タプルの新規接続をブロック。",
        ),
        ("net/netstat", "TcpExt_DelayedACKLocked") => (
            "遅延 ACK 追加遅延（ロック）",
            "ソケットがアプリケーションによりロックされていたため延期された遅延 ACK。",
            "ソケットロックによる遅延 ACK 延期。\n\n💡 診断: アプリが ACK タイマー中にソケットロックを保持。レイテンシ増加の可能性。",
        ),
        ("net/netstat", "TcpExt_DelayedACKLost") => (
            "遅延 ACK ロス検出",
            "遅延 ACK が紛失したと思われるためクイック ACK モードが有効化。",
            "遅延 ACK ロス検出。\n\n💡 診断: クイック ACK モードをトリガー。遅延 ACK メカニズムがタイミングを誤判断。",
        ),
        ("net/netstat", "TcpExt_TCPHPAcks") => (
            "ヘッダ予測高速パス ACK",
            "最適化されたヘッダ予測パスで処理された ACK。高いほど良い。",
            "ヘッダ予測高速パス ACK。\n\n💡 診断: 全 ACK に対する比率が高い → 効率的な TCP スタック処理。低い → 通常でないパケットパターン。",
        ),
        ("net/netstat", "TcpExt_TCPRenoRecovery") => (
            "Reno 高速回復イベント",
            "Reno（非 SACK）高速回復アルゴリズムによる TCP ロス回復。",
            "TCP Reno 高速回復。\n\n💡 診断: SACK が利用できない場合のフォールバック。SACK 回復が推奨。",
        ),
        ("net/netstat", "TcpExt_TCPSACKReneging") => (
            "受信者による SACK データ撤回",
            "受信者が以前 SACK したデータを撤回した回数。データがあると言ったのに無かった。",
            "SACK 撤回イベント。\n\n💡 診断: まれで問題あり。受信者が以前確認済みのデータを破棄し、再送を強制。",
        ),
        ("net/netstat", "TcpExt_TCPSACKReorder") => (
            "SACK による順序入れ替え検出",
            "SACK 情報により検出されたパケット順序入れ替えイベント。",
            "SACK 検出の順序入れ替え。\n\n💡 診断: 非ゼロ → ネットワーク経路がパケットを並べ替え。SACK はこれを適切に処理。",
        ),
        ("net/netstat", "TcpExt_TCPRenoReorder") => (
            "Reno による順序入れ替え検出",
            "Reno 高速再送アルゴリズムにより検出されたパケット順序入れ替え。",
            "Reno 検出の順序入れ替え。\n\n💡 診断: SACK なしの順序入れ替え検出。SACK ベースより精度が低い。",
        ),
        ("net/netstat", "TcpExt_TCPTSReorder") => (
            "タイムスタンプによる順序入れ替え検出",
            "TCP タイムスタンプ分析により検出された順序入れ替え。",
            "タイムスタンプ検出の順序入れ替え。\n\n💡 診断: もう一つの順序入れ替え検出方法。非ゼロ → 経路に順序入れ替えあり。",
        ),
        ("net/netstat", "TcpExt_TCPFullUndo") => (
            "輻輳ウィンドウ完全アンドゥ",
            "誤った輻輳検出後に輻輳ウィンドウが完全に復元された回数。",
            "誤輻輳検出後の完全 cwnd アンドゥ。\n\n💡 診断: 良い — 輻輳が誤検出と判断され、フルスループットに復元。",
        ),
        ("net/netstat", "TcpExt_TCPPartialUndo") => (
            "輻輳ウィンドウ部分アンドゥ",
            "誤った輻輳検出後に輻輳ウィンドウが部分的に復元された回数。",
            "部分 cwnd アンドゥ。\n\n💡 診断: 誤輻輳から部分回復したが完全ではない。完全アンドゥより非効率。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKUndo") => (
            "DSACK によるアンドゥ",
            "DSACK が再送が不要だったことを示した後の輻輳ウィンドウ復元。",
            "DSACK ベースの輻輳応答アンドゥ。\n\n💡 診断: DSACK が再送が誤りだと検出。良い — スループットが回復。",
        ),
        ("net/netstat", "TcpExt_TCPLossUndo") => (
            "ロス検出アンドゥ",
            "ロス検出が誤りと判明し、輻輳状態が復元された回数。",
            "ロス検出アンドゥ。\n\n💡 診断: カーネルがパケットロスと判断したが到着した。ネットワークのジッタが大きい可能性。",
        ),
        ("net/netstat", "TcpExt_TCPLostRetransmit") => (
            "再送セグメントの再ロス",
            "再送されたがその再送もロスされたセグメント。",
            "ロスされた再送。\n\n💡 診断: 非常に悪い — 再送すらロスされている。深刻なネットワーク経路問題。",
        ),
        ("net/netstat", "TcpExt_TCPRenoFailures") => (
            "Reno 回復失敗",
            "Reno 高速回復がロスから回復できなかった回数。",
            "Reno 回復失敗。\n\n💡 診断: Reno が回復できずタイムアウトにフォールバック。SACK 対応ピアではこれを回避。",
        ),
        ("net/netstat", "TcpExt_TCPSackFailures") => (
            "SACK 回復失敗",
            "SACK ベースの回復がロスから回復できなかった回数。",
            "SACK 回復失敗。\n\n💡 診断: SACK でも回復できなかった — 非常に重い、または持続的なパケットロス。",
        ),
        ("net/netstat", "TcpExt_TCPLossFailures") => (
            "ロス回復失敗",
            "ロスベースの回復が失敗した回数。",
            "ロス回復失敗。\n\n💡 診断: ロス回復アルゴリズムが修復できなかった。接続はタイムアウトした可能性大。",
        ),
        ("net/netstat", "TcpExt_TCPSlowStartRetrans") => (
            "スロースタート中の再送",
            "TCP スロースタートフェーズ中に再送されたセグメント。",
            "スロースタート再送。\n\n💡 診断: スロースタート中のロスは高コスト — 輻輳ウィンドウの成長がリセットされる。",
        ),
        ("net/netstat", "TcpExt_TCPLossProbeRecovery") => (
            "ロスプローブによる回復",
            "Tail Loss Probe (TLP) が完全 RTO なしで回復をトリガーした回数。",
            "TLP トリガーの回復。\n\n💡 診断: TLP が意図通り動作 — テールロスを RTO より速く回復。",
        ),
        ("net/netstat", "TcpExt_TCPRenoRecoveryFail") => (
            "Reno 回復参入後の失敗",
            "Reno 高速回復に入ったが最終的に失敗した回数。",
            "Reno 回復参入失敗。\n\n💡 診断: Reno 回復に入ったがロスが重すぎた。タイムアウトにフォールバック。",
        ),
        ("net/netstat", "TcpExt_TCPSackRecoveryFail") => (
            "SACK 回復参入後の失敗",
            "SACK 回復に入ったが最終的に失敗した回数。",
            "SACK 回復参入失敗。\n\n💡 診断: SACK 回復でもロスパターンを処理できなかった。非常に重いパケットロス。",
        ),
        ("net/netstat", "TcpExt_TCPRcvCollapsed") => (
            "受信キューのパケット統合",
            "メモリ圧迫下でスペース節約のため受信キュー内のパケットが統合された回数。",
            "受信キュー統合イベント。\n\n💡 診断: 非ゼロ → メモリ圧迫により受信キューエントリを統合。パフォーマンスへの影響あり。",
        ),
        ("net/netstat", "TcpExt_TCPBacklogCoalesce") => (
            "バックログパケット統合",
            "効率化のためソケットバックログ内でパケットが統合された回数。",
            "ソケットバックログ統合。\n\n💡 診断: 正常な最適化。アプリが読み取る前にパケットを統合。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOldSent") => (
            "旧データに対する DSACK 送信",
            "既に受信済みのデータに対して送信された重複 SACK（再送は不要だった）。",
            "受信済みデータへの DSACK 送信。\n\n💡 診断: 送信者に再送が不要だったことを通知。送信者が誤再送を検出するのに役立つ。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOfoSent") => (
            "順序外データに対する DSACK 送信",
            "順序外セグメントに対して送信された重複 SACK。",
            "順序外データへの DSACK 送信。\n\n💡 診断: 順序入れ替えによる重複配信検出のシグナル。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKRecv") => (
            "DSACK 受信",
            "ピアから受信した重複 SACK ブロック。このホストの再送が不要だったことを示す。",
            "DSACK ブロック受信。\n\n💡 診断: 高い数 → このホストが不必要に再送中。RTO が積極的すぎる可能性。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKOfoRecv") => (
            "順序外 DSACK 受信",
            "逆方向の順序外配信を示す重複 SACK ブロックの受信。",
            "順序外 DSACK ブロック受信。\n\n💡 診断: このホストへのネットワーク経路に順序入れ替えあり。",
        ),
        ("net/netstat", "TcpExt_TCPAbortOnLinger") => (
            "リンガータイムアウト後の接続中断",
            "リンガータイムアウトが期限切れしデータが保留中のまま接続が中断。",
            "リンガータイムアウトでの TCP 中断。\n\n💡 診断: アプリが SO_LINGER を設定したが、データを時間内にフラッシュできなかった。",
        ),
        ("net/netstat", "TcpExt_TCPAbortFailed") => (
            "接続中断試行の失敗",
            "接続を中断する試みが失敗した回数。",
            "TCP 中断試行の失敗。\n\n💡 診断: 非常にまれ。接続切断のカーネルレベルの問題。",
        ),
        ("net/netstat", "TcpExt_TCPMemoryPressuresChrono") => (
            "TCP メモリ圧迫持続時間（ms）",
            "TCP スタックがメモリ圧迫下にあった合計持続時間（ミリ秒）。",
            "TCP メモリ圧迫持続時間。\n\n💡 診断: 非ゼロ → TCP メモリ圧迫モードで時間を消費。バッファサイズ縮小と相関。",
        ),
        ("net/netstat", "TcpExt_TCPSACKDiscard") => (
            "SACK ブロック破棄",
            "無効または使用不可として破棄された SACK ブロック。",
            "破棄された SACK ブロック。\n\n💡 診断: 非ゼロ → ピアが無効な SACK 情報を送信。ミドルボックスかバグのあるスタックの可能性。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKIgnoredOld") => (
            "DSACK ブロック無視（古い）",
            "古いデータを参照していたため無視された DSACK ブロック。",
            "古い DSACK ブロック無視。\n\n💡 診断: DSACK が遅すぎて役に立たなかった。無害だが遅延フィードバックを示す。",
        ),
        ("net/netstat", "TcpExt_TCPDSACKIgnoredNoUndo") => (
            "DSACK ブロック無視（アンドゥ不可）",
            "その時点でアンドゥが不可能だったため無視された DSACK ブロック。",
            "DSACK 無視（アンドゥ不可）。\n\n💡 診断: DSACK が到着したが輻輳状態は既に進行。cwnd 削減をアンドゥできない。",
        ),
        ("net/netstat", "TcpExt_TCPMD5NotFound") => (
            "TCP MD5 署名の欠落",
            "TCP MD5 署名が期待されていたが付いていなかったセグメント。",
            "TCP-MD5 署名の欠落。\n\n💡 診断: 非ゼロ → MD5 保護接続（BGP）が未署名パケットを受信。セキュリティ上の懸念。",
        ),
        ("net/netstat", "TcpExt_TCPMD5Unexpected") => (
            "予期しない TCP MD5 署名",
            "期待されていない TCP MD5 署名付きセグメント。",
            "予期しない TCP-MD5 署名。\n\n💡 診断: ピアが非 MD5 接続に MD5 署名パケットを送信。設定の不一致。",
        ),
        ("net/netstat", "TcpExt_TCPMD5Failure") => (
            "TCP MD5 署名検証失敗",
            "TCP MD5 署名の検証に失敗したセグメント。",
            "TCP-MD5 署名失敗。\n\n💡 診断: ピア間の MD5 キー不一致。BGP MD5 パスワード設定を確認。",
        ),
        ("net/netstat", "TcpExt_TCPSackShifted") => (
            "再送キューでの SACK データシフト",
            "再送キューレイアウトを最適化するためシフトされた SACK ブロック。",
            "SACK シフト操作。\n\n💡 診断: 内部最適化。高い値はアクティブな SACK 処理を示す。",
        ),
        ("net/netstat", "TcpExt_TCPSackMerged") => (
            "SACK ブロック統合",
            "再送キューで統合された隣接 SACK ブロック。",
            "SACK 統合操作。\n\n💡 診断: SACK ブロックオーバーヘッドを削減する正常な最適化。",
        ),
        ("net/netstat", "TcpExt_TCPSackShiftFallback") => (
            "SACK シフトフォールバック",
            "SACK シフト最適化が従来の処理にフォールバック。",
            "SACK シフトフォールバック。\n\n💡 診断: シフト最適化が適用できなかった。低速パスにフォールバック。",
        ),
        ("net/netstat", "TcpExt_PFMemallocDrop") => (
            "pfmemalloc でのセグメントドロップ",
            "pfmemalloc（緊急メモリ）コンテキストで受信されたため破棄された TCP セグメント。",
            "PF_MEMALLOC ドロップ。\n\n💡 診断: 非ゼロ → 極度のメモリ圧迫。メモリ解放操作のみ許可。",
        ),
        ("net/netstat", "TcpExt_TCPMinTTLDrop") => (
            "最小 TTL 未満のセグメントドロップ",
            "IP TTL がソケットの最小 TTL 閾値を下回っていたため破棄されたセグメント。",
            "最小 TTL ドロップ。\n\n💡 診断: IP_MINTTL ソケットオプションがパケットをフィルタ中。軽量セキュリティ対策（BGP GTSM 等）として使用。",
        ),
        ("net/netstat", "TcpExt_TCPDeferAcceptDrop") => (
            "DEFER_ACCEPT でのセグメントドロップ",
            "TCP_DEFER_ACCEPT を設定したソケットでデータが到着しなかったため破棄されたセグメント。",
            "TCP_DEFER_ACCEPT ドロップ。\n\n💡 診断: サーバーが DEFER_ACCEPT を使用してデータ到着まで accept() を遅延。データなし ACK は破棄。",
        ),
        ("net/netstat", "TcpExt_IPReversePathFilter") => (
            "逆経路フィルタドロップ",
            "IP 逆経路フィルタリング（スプーフィング防止）により破棄されたパケット。",
            "逆経路フィルタドロップ。\n\n💡 診断: 非ゼロ → 予期しないインターフェースにパケットが到着。非対称ルーティングかスプーフィングの可能性。",
        ),
        ("net/netstat", "TcpExt_TCPReqQFullDoCookies") => (
            "キュー満杯で SYN Cookie 使用",
            "リクエストキューが満杯のため SYN Cookie が有効化された回数。",
            "フルキューによる SYN Cookie 有効化。\n\n💡 診断: SYN フラッド保護が起動。正当なトラフィックが原因なら tcp_max_syn_backlog を増加。",
        ),
        ("net/netstat", "TcpExt_TCPReqQFullDrop") => (
            "キュー満杯で SYN ドロップ",
            "リクエストキューが満杯で SYN Cookie が利用できなかったため破棄された SYN セグメント。",
            "フルリクエストキューからの SYN ドロップ。\n\n💡 診断: 接続が拒否されている。tcp_max_syn_backlog を増加するか SYN Cookie を有効化。",
        ),
        ("net/netstat", "TcpExt_TCPRetransFail") => (
            "再送試行の失敗",
            "失敗した TCP 再送試行（メモリ割り当て不可等）。",
            "再送試行失敗。\n\n💡 診断: 非ゼロ → カーネルが再送すらできなかった。深刻なリソース枯渇。",
        ),
        ("net/netstat", "TcpExt_TCPOFODrop") => (
            "順序外パケットのドロップ",
            "順序外キューから破棄されたパケット。並べ替え情報が失われる。",
            "順序外キュードロップ。\n\n💡 診断: 非ゼロ → メモリ圧迫で OOO データを喪失。再送をトリガー。",
        ),
        ("net/netstat", "TcpExt_TCPOFOMerge") => (
            "順序外パケットの統合",
            "メモリ使用量削減のため順序外キュー内で統合されたパケット。",
            "順序外キュー統合。\n\n💡 診断: 正常な最適化。隣接する OOO セグメントを結合。",
        ),
        ("net/netstat", "TcpExt_TCPSYNChallenge") => (
            "チャレンジ ACK をトリガーした SYN",
            "確立済み接続上の SYN セグメントがチャレンジ ACK 応答をトリガー。",
            "SYN チャレンジイベント。\n\n💡 診断: ブラインド SYN インジェクション攻撃の試み（RFC 5961）の可能性。チャレンジ ACK が接続を検証。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenActiveFail") => (
            "TCP Fast Open アクティブ失敗",
            "送信 TCP Fast Open 接続試行の失敗。",
            "TFO アクティブ接続失敗。\n\n💡 診断: サーバーが TFO をサポートしていないか、ミドルボックスが TFO オプションを除去。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassiveFail") => (
            "TCP Fast Open パッシブ失敗",
            "受信 TCP Fast Open 受付試行の失敗。",
            "TFO パッシブ受付失敗。\n\n💡 診断: TFO Cookie 検証失敗またはサーバー側 TFO エラー。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenListenOverflow") => (
            "TCP Fast Open リスンオーバーフロー",
            "リスンキューがオーバーフローしたため破棄された TCP Fast Open リクエスト。",
            "TFO リスンオーバーフロー。\n\n💡 診断: ListenOverflow と同じだが TFO 接続固有。somaxconn を増加。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenCookieReqd") => (
            "TCP Fast Open Cookie 要求",
            "Cookie が必要だが持っていなかった TCP Fast Open SYN。",
            "TFO Cookie 要求。\n\n💡 診断: TFO サーバーへの初回接続は Cookie リクエストの往復が必要。以降の接続は高速。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenBlackhole") => (
            "TCP Fast Open ブラックホール検出",
            "TFO データがミドルボックスにより無言でドロップされた TCP Fast Open ブラックホールイベント。",
            "TFO ブラックホール検出。\n\n💡 診断: ミドルボックス（FW、NAT）が TFO SYN+データをドロップ。TCP は通常のハンドシェイクにフォールバック。",
        ),
        ("net/netstat", "TcpExt_TCPSpuriousRtxHostQueues") => (
            "ホストキューによる誤再送",
            "ローカルホストキューでのパケット遅延により誤りと検出された再送。",
            "ローカルキューイングによる誤再送。\n\n💡 診断: ローカルキューイング遅延（qdisc、NIC リングバッファ）が早期 RTO を引き起こした。ネットワークの問題ではない。",
        ),
        ("net/netstat", "TcpExt_BusyPollRxPackets") => (
            "ビジーポーリングで受信したパケット",
            "ビジーポーリング（低レイテンシポーリングモード）により受信したネットワークパケット。",
            "ビジーポール RX パケット。\n\n💡 診断: 非ゼロ → アプリが SO_BUSY_POLL を使用して低レイテンシパケット受信中。",
        ),
        ("net/netstat", "TcpExt_TCPFromZeroWindowAdv") => (
            "ゼロウィンドウから非ゼロへの遷移",
            "受信ウィンドウがゼロから非ゼロに遷移した回数。",
            "ゼロ→非ゼロウィンドウ遷移。\n\n💡 診断: 受信者がゼロウィンドウ状態から回復。アプリが読み取りに追いついた。",
        ),
        ("net/netstat", "TcpExt_TCPToZeroWindowAdv") => (
            "非ゼロからゼロウィンドウへの遷移",
            "受信ウィンドウがゼロに低下し送信者を停止させた回数。",
            "ウィンドウ→ゼロ遷移。\n\n💡 診断: 非ゼロ → アプリが十分速く読み取れない。送信者が一時停止。アプリの読み取りパスを最適化。",
        ),
        ("net/netstat", "TcpExt_TCPWantZeroWindowAdv") => (
            "ゼロウィンドウ広告希望",
            "ゼロウィンドウ広告が望ましかったが条件が揃わなかった回数。",
            "ゼロウィンドウ広告希望。\n\n💡 診断: カーネルがゼロウィンドウを広告したかったができなかった。バッファ圧力の境界線。",
        ),
        ("net/netstat", "TcpExt_TCPHystartTrainDetect") => (
            "Hystart トレーニングフェーズ検出",
            "cubic 輻輳制御の Hystart トレーニングフェーズ中に検出された輻輳。",
            "Hystart トレーニング検出。\n\n💡 診断: Hystart がスロースタートの行き過ぎを制限。検出により過剰なパケットバーストを防止。",
        ),
        ("net/netstat", "TcpExt_TCPHystartTrainCwnd") => (
            "Hystart トレーニング検出時の cwnd",
            "Hystart トレーニング検出がトリガーされた時の輻輳ウィンドウサイズ。",
            "Hystart トレーニング cwnd。\n\n💡 診断: 平均値は検出時点の典型的な帯域幅遅延積を示す。",
        ),
        ("net/netstat", "TcpExt_TCPHystartDelayDetect") => (
            "Hystart 遅延ベース検出",
            "Hystart 遅延増加測定による輻輳検出。",
            "Hystart 遅延検出。\n\n💡 診断: RTT 増加がスロースタートからの離脱をトリガー。行き過ぎを防止。",
        ),
        ("net/netstat", "TcpExt_TCPHystartDelayCwnd") => (
            "Hystart 遅延検出時の cwnd",
            "Hystart 遅延検出がトリガーされた時の輻輳ウィンドウサイズ。",
            "Hystart 遅延検出 cwnd。\n\n💡 診断: 遅延ベース検出が介入した時点の接続帯域幅を示す。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedSynRecv") => (
            "SYN-RECV 状態でのスキップ ACK",
            "接続が SYN-RECV 状態だったためスキップされた ACK セグメント。",
            "SYN-RECV でのスキップ ACK。\n\n💡 診断: 接続がまだ確立されていない。ACK が早すぎた。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedPAWS") => (
            "PAWS によるスキップ ACK",
            "PAWS チェックによりスキップされた ACK セグメント。",
            "PAWS によるスキップ ACK。\n\n💡 診断: タイムスタンプにより古い重複 ACK を検出。正常なセキュリティメカニズム。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedSeq") => (
            "シーケンス不一致スキップ ACK",
            "シーケンス番号が期待値と一致しなかったためスキップされた ACK セグメント。",
            "シーケンスチェックによるスキップ ACK。\n\n💡 診断: 古いまたは無効な ACK。ミドルボックスの干渉の可能性。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedFinWait2") => (
            "FIN-WAIT-2 でのスキップ ACK",
            "接続が FIN-WAIT-2 状態だったためスキップされた ACK セグメント。",
            "FIN-WAIT-2 でのスキップ ACK。\n\n💡 診断: 接続が切断中。この段階では ACK は無関係。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedTimeWait") => (
            "TIME-WAIT でのスキップ ACK",
            "接続が TIME-WAIT 状態だったためスキップされた ACK セグメント。",
            "TIME-WAIT でのスキップ ACK。\n\n💡 診断: 正常 — 閉じた接続に対する古い ACK の到着。",
        ),
        ("net/netstat", "TcpExt_TCPACKSkippedChallenge") => (
            "チャレンジ処理によるスキップ ACK",
            "チャレンジ ACK 処理中にスキップされた ACK セグメント。",
            "チャレンジ ACK によるスキップ ACK。\n\n💡 診断: チャレンジ ACK 応答のレート制限。セキュリティ機能（RFC 5961）。",
        ),
        ("net/netstat", "TcpExt_TCPMTUPFail") => (
            "TCP MTU プローブ失敗",
            "TCP パス MTU 探索プローブの失敗。",
            "TCP PMTUD プローブ失敗。\n\n💡 診断: パス MTU プローブパケットがロスト。TCP はより小さいサイズで再試行。",
        ),
        ("net/netstat", "TcpExt_TCPMTUPSuccess") => (
            "TCP MTU プローブ成功",
            "より大きな MTU を見つけることに成功した TCP パス MTU 探索プローブ。",
            "TCP PMTUD プローブ成功。\n\n💡 診断: MSS の増加に成功。パケットあたりのオーバーヘッド削減でスループットが向上。",
        ),
        ("net/netstat", "TcpExt_TCPDeliveredCE") => (
            "ECN CE マーク付き TCP セグメント配信",
            "ECN Congestion Experienced マークが付いたアプリケーションに配信された TCP セグメント。",
            "ECN CE マーク付きセグメント配信。\n\n💡 診断: 非ゼロ → ネットワークが ECN を通じて輻輳をシグナリング。TCP はレートを適応的に削減。",
        ),
        ("net/netstat", "TcpExt_TCPAckCompressed") => (
            "圧縮 ACK 送信（GRO）",
            "処理オーバーヘッド削減のため GRO（Generic Receive Offload）により圧縮された ACK。",
            "GRO 圧縮 ACK。\n\n💡 診断: 正常な NIC オフロード最適化。割り込みと処理オーバーヘッドを削減。",
        ),
        ("net/netstat", "TcpExt_TCPZeroWindowDrop") => (
            "ゼロウィンドウによるセグメントドロップ",
            "受信ウィンドウがゼロだったため破棄された TCP セグメント。",
            "ゼロウィンドウセグメントドロップ。\n\n💡 診断: 受信者のバッファが満杯。アプリがデータをより速く読み取る必要あり。",
        ),
        ("net/netstat", "TcpExt_TCPRcvQDrop") => (
            "受信キューからのセグメントドロップ",
            "受信キューから破棄された TCP セグメント。",
            "受信キュードロップ。\n\n💡 診断: アプリがソケットを十分速く読み取っていない。データロス。",
        ),
        ("net/netstat", "TcpExt_TCPWqueueTooBig") => (
            "書き込みキュー制限超過",
            "TCP 書き込みキューがサイズ制限を超過した回数。",
            "書き込みキューオーバーフロー。\n\n💡 診断: アプリが TCP の送信能力を超えて書き込み中。バックプレッシャーが必要。",
        ),
        ("net/netstat", "TcpExt_TCPFastOpenPassiveAltKey") => (
            "TFO 代替キーでのパッシブ接続",
            "代替 Cookie キーを使用した TCP Fast Open パッシブ接続。",
            "TFO 代替キー接続。\n\n💡 診断: キーローテーション中。新旧両方のキーが一時的に受け入れ。",
        ),

        // ── net/netstat — 残りの IpExt フィールド ────────────────────
        ("net/netstat", "IpExt_InNoRoutes") => (
            "ルートなし受信パケット",
            "宛先アドレスへのルートが存在しなかったため破棄されたパケット。",
            "ルートなし受信ドロップ。\n\n💡 診断: このホストが到達できない宛先へのトラフィックが到着。ルーティング設定ミスの可能性。",
        ),
        ("net/netstat", "IpExt_InTruncatedPkts") => (
            "切り詰められた受信パケット",
            "IP ヘッダの示す長さより短かったため破棄されたパケット。",
            "切り詰めパケットドロップ。\n\n💡 診断: 非ゼロ → ネットワーク上の破損パケット。NIC とケーブルを確認。",
        ),
        ("net/netstat", "IpExt_OutMcastPkts") => (
            "マルチキャストパケット送信",
            "このホストが送信した IP マルチキャストパケット。",
            "送信マルチキャストパケット。\n\n💡 診断: 非ゼロ → マルチキャストトラフィックを送信中（サービスディスカバリ、クラスタ通信）。",
        ),
        ("net/netstat", "IpExt_InBcastPkts") => (
            "ブロードキャストパケット受信",
            "受信した IP ブロードキャストパケット。",
            "受信ブロードキャスト。\n\n💡 診断: ローカルネットワークでは正常。高レート → ブロードキャストストームか過度なプロトコル。",
        ),
        ("net/netstat", "IpExt_OutBcastPkts") => (
            "ブロードキャストパケット送信",
            "このホストが送信した IP ブロードキャストパケット。",
            "送信ブロードキャスト。\n\n💡 診断: ARP、DHCP で一般的。過剰 → ブロードキャスト多用アプリを確認。",
        ),
        ("net/netstat", "IpExt_InMcastOctets") => (
            "マルチキャストバイト受信",
            "マルチキャストパケットで受信した合計バイト数。",
            "受信マルチキャストバイト。\n\n💡 診断: 変化率がマルチキャスト帯域幅消費を示す。",
        ),
        ("net/netstat", "IpExt_OutMcastOctets") => (
            "マルチキャストバイト送信",
            "マルチキャストパケットで送信した合計バイト数。",
            "送信マルチキャストバイト。\n\n💡 診断: レートがマルチキャスト出力帯域幅を示す。高い → ビデオストリーミングやクラスタ同期。",
        ),
        ("net/netstat", "IpExt_InBcastOctets") => (
            "ブロードキャストバイト受信",
            "ブロードキャストパケットで受信した合計バイト数。",
            "受信ブロードキャストバイト。\n\n💡 診断: 高レート → ブロードキャスト多用のネットワーク環境。",
        ),
        ("net/netstat", "IpExt_OutBcastOctets") => (
            "ブロードキャストバイト送信",
            "ブロードキャストパケットで送信した合計バイト数。",
            "送信ブロードキャストバイト。\n\n💡 診断: 小さいはず。大きい値 → 異常なブロードキャスト活動。",
        ),
        ("net/netstat", "IpExt_InNoECTPkts") => (
            "ECN 非対応パケット",
            "ECN-Capable Transport フラグが設定されていない受信パケット。",
            "非 ECT 受信パケット。\n\n💡 診断: インターネット上のほとんどのパケットは非 ECT。ベースラインカウンタ。",
        ),
        ("net/netstat", "IpExt_InECT1Pkts") => (
            "ECT(1) フラグ付きパケット",
            "ECN-Capable Transport コードポイント 1 の受信パケット。",
            "ECT(1) 受信パケット。\n\n💡 診断: 一部の ECN 実装で使用。ECN 対応トラフィックを示す。",
        ),
        ("net/netstat", "IpExt_InECT0Pkts") => (
            "ECT(0) フラグ付きパケット",
            "ECN-Capable Transport コードポイント 0（標準 ECN マーキング）の受信パケット。",
            "ECT(0) 受信パケット。\n\n💡 診断: 標準 ECN マーキング。非ゼロ → ECN 対応ピアが通信中。",
        ),
        ("net/netstat", "IpExt_InCEPkts") => (
            "ECN 輻輳経験フラグ付きパケット",
            "Congestion Experienced ECN フラグ付き受信パケット。ルーターが輻輳をシグナリング。",
            "ECN CE マーク付き受信パケット。\n\n💡 診断: 非ゼロ → ネットワークルーターが輻輳をシグナリング。TCP は送信レートを削減。",
        ),
        ("net/netstat", "IpExt_ReasmOverlaps") => (
            "IP 再組み立てオーバーラップフラグメント",
            "IP 再組み立て中に検出されたオーバーラップフラグメント。攻撃の可能性。",
            "オーバーラップ IP フラグメント。\n\n💡 診断: 非ゼロ → フラグメントオーバーラップ攻撃かバグのある送信者の可能性。Linux はオーバーラップフラグメントを破棄。",
        ),

        // ── net/netstat — MPTcpExt フィールド ────────────────────────
        ("net/netstat", "MPTcpExt_MPCapableSYNRX") => (
            "MPTCP 対応 SYN 受信",
            "MPTCP 対応オプション付き SYN セグメントの受信数。",
            "MPTCP 対応 SYN 受信。\n\n💡 診断: 非ゼロ → クライアントが MPTCP 接続を試行中。",
        ),
        ("net/netstat", "MPTcpExt_MPCapableSYNTX") => (
            "MPTCP 対応 SYN 送信",
            "MPTCP 対応オプション付き SYN セグメントの送信数。",
            "MPTCP 対応 SYN 送信。\n\n💡 診断: 非ゼロ → このホストが MPTCP 接続を開始中。",
        ),
        ("net/netstat", "MPTcpExt_MPCapableSYNACKRX") => (
            "MPTCP 対応 SYN-ACK 受信",
            "MPTCP サポートを確認する SYN-ACK セグメントの受信数。",
            "MPTCP SYN-ACK 受信。\n\n💡 診断: サーバーが MPTCP 対応を受理。",
        ),
        ("net/netstat", "MPTcpExt_MPCapableACKRX") => (
            "MPTCP 対応 ACK 受信",
            "MPTCP ハンドシェイクの最終 ACK 受信。MPTCP セットアップ完了。",
            "MPTCP ACK 受信。\n\n💡 診断: MPTCP 接続が完全に確立。",
        ),
        ("net/netstat", "MPTcpExt_MPCapableFallbackACK") => (
            "MPTCP ACK でのフォールバック",
            "最終 ACK で MPTCP が通常の TCP にフォールバック。",
            "MPTCP ACK フォールバック。\n\n💡 診断: ミドルボックスが MPTCP オプションを除去し、フォールバックを強制。",
        ),
        ("net/netstat", "MPTcpExt_MPCapableFallbackSYNACK") => (
            "MPTCP SYN-ACK でのフォールバック",
            "SYN-ACK で MPTCP が通常の TCP にフォールバック。",
            "MPTCP SYN-ACK フォールバック。\n\n💡 診断: サーバーが MPTCP 非対応かミドルボックスの干渉。",
        ),
        ("net/netstat", "MPTcpExt_MPFallbackTokenInit") => (
            "MPTCP トークン初期化フォールバック",
            "トークン初期化中に MPTCP 接続がフォールバック。",
            "MPTCP トークン初期化フォールバック。\n\n💡 診断: トークン衝突またはリソース枯渇。",
        ),
        ("net/netstat", "MPTcpExt_MPTCPRetrans") => (
            "MPTCP 再送",
            "MPTCP レイヤーで再送された MPTCP セグメント。",
            "MPTCP 再送。\n\n💡 診断: 非ゼロ → MPTCP サブフローでパケットロス。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinNoTokenFound") => (
            "MPTCP ジョイン: トークン未検出",
            "一致する接続トークンが見つからなかった MPTCP ジョインリクエスト。",
            "MPTCP ジョイントークンミス。\n\n💡 診断: 接続がもう存在しないかトークン不一致。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynRx") => (
            "MPTCP ジョイン SYN 受信",
            "MPTCP サブフロージョイン SYN セグメントの受信。",
            "MPTCP ジョイン SYN 受信。\n\n💡 診断: ピアが追加サブフローをリクエスト中。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynAckRx") => (
            "MPTCP ジョイン SYN-ACK 受信",
            "MPTCP サブフロージョイン SYN-ACK セグメントの受信。",
            "MPTCP ジョイン SYN-ACK 受信。\n\n💡 診断: サブフロージョインハンドシェイクが進行中。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinSynAckHMacFailure") => (
            "MPTCP ジョイン SYN-ACK HMAC 失敗",
            "MPTCP ジョイン SYN-ACK HMAC 認証の失敗。",
            "MPTCP ジョイン HMAC 失敗。\n\n💡 診断: 認証失敗 — キー不一致または改ざん。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinAckRx") => (
            "MPTCP ジョイン ACK 受信",
            "MPTCP サブフロージョイン ACK セグメントの受信。サブフロー設定完了。",
            "MPTCP ジョイン ACK 受信。\n\n💡 診断: サブフロージョインが完了。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinAckHMacFailure") => (
            "MPTCP ジョイン ACK HMAC 失敗",
            "MPTCP ジョイン ACK HMAC 認証の失敗。",
            "MPTCP ジョイン ACK HMAC 失敗。\n\n💡 診断: サブフロージョイン認証の失敗。",
        ),
        ("net/netstat", "MPTcpExt_DSSNotMatching") => (
            "MPTCP DSS 不一致",
            "期待される状態と一致しなかった MPTCP データシーケンスシグナルセグメント。",
            "MPTCP DSS 不一致。\n\n💡 診断: MPTCP レイヤーと TCP レイヤー間のデータマッピングエラー。",
        ),
        ("net/netstat", "MPTcpExt_InfiniteMapRx") => (
            "MPTCP 無限マップ受信",
            "MPTCP 無限マッピングセグメント受信。通常の TCP へのフォールバックを示す。",
            "MPTCP 無限マップ受信。\n\n💡 診断: ピアがこの接続で MPTCP から通常の TCP にフォールバック中。",
        ),
        ("net/netstat", "MPTcpExt_OFOQueueTail") => (
            "MPTCP OFO キュー末尾追加",
            "MPTCP 順序外キューの末尾に追加されたセグメント。",
            "MPTCP OFO キュー末尾。\n\n💡 診断: 順序外データが並べ替えのためにキューイング中。",
        ),
        ("net/netstat", "MPTcpExt_OFOQueue") => (
            "MPTCP 順序外キューセグメント",
            "MPTCP 順序外キューに入れられたセグメント。",
            "MPTCP OFO キュー。\n\n💡 診断: 複数サブフローのレイテンシが異なる場合は正常。",
        ),
        ("net/netstat", "MPTcpExt_OFOMerge") => (
            "MPTCP OFO セグメント統合",
            "統合された MPTCP 順序外セグメント。",
            "MPTCP OFO 統合。\n\n💡 診断: 隣接 OFO セグメントの正常な最適化。",
        ),
        ("net/netstat", "MPTcpExt_NoDSSInWindow") => (
            "MPTCP ウィンドウ内に DSS なし",
            "受信ウィンドウ内にデータシーケンスシグナルのない MPTCP セグメント。",
            "MPTCP ウィンドウ内 DSS なし。\n\n💡 診断: MPTCP データのマッピングギャップ。",
        ),
        ("net/netstat", "MPTcpExt_DuplicateData") => (
            "MPTCP 重複データ受信",
            "MPTCP レイヤーで受信した重複データセグメント。",
            "MPTCP 重複データ。\n\n💡 診断: 異なるサブフロー間でのデータ再送、またはサブフロー再送。",
        ),
        ("net/netstat", "MPTcpExt_AddAddr") => (
            "MPTCP ADD_ADDR 受信",
            "追加アドレスを通知する MPTCP ADD_ADDR オプションの受信。",
            "MPTCP ADD_ADDR 受信。\n\n💡 診断: ピアがサブフロー作成用の追加ネットワークアドレスを広告中。",
        ),
        ("net/netstat", "MPTcpExt_EchoAdd") => (
            "MPTCP ADD_ADDR エコー送信",
            "アドレス追加を確認する MPTCP ADD_ADDR エコーオプションの送信。",
            "MPTCP ADD_ADDR エコー。\n\n💡 診断: ピアのアドレス広告を確認中。",
        ),
        ("net/netstat", "MPTcpExt_PortAdd") => (
            "MPTCP ポートベース ADD_ADDR 受信",
            "ポート情報付き MPTCP ADD_ADDR オプションの受信。",
            "MPTCP ポート ADD_ADDR。\n\n💡 診断: ピアがサブフロー用の追加ポートを広告中。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortSynRx") => (
            "MPTCP ポートベースジョイン SYN 受信",
            "異なるポートでの MPTCP サブフロージョイン SYN。",
            "MPTCP ポートジョイン SYN 受信。\n\n💡 診断: 代替ポートでのサブフロージョイン試行。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortSynAckRx") => (
            "MPTCP ポートベースジョイン SYN-ACK 受信",
            "異なるポートでの MPTCP サブフロージョイン SYN-ACK。",
            "MPTCP ポートジョイン SYN-ACK 受信。\n\n💡 診断: ポートベースサブフロージョインが進行中。",
        ),
        ("net/netstat", "MPTcpExt_MPJoinPortAckRx") => (
            "MPTCP ポートベースジョイン ACK 受信",
            "異なるポートでの MPTCP サブフロージョイン ACK。セットアップ完了。",
            "MPTCP ポートジョイン ACK 受信。\n\n💡 診断: ポートベースサブフローが確立。",
        ),
        ("net/netstat", "MPTcpExt_MismatchPortSynRx") => (
            "MPTCP ポート不一致ジョイン SYN",
            "予期しないポートで受信した MPTCP ジョイン SYN。",
            "MPTCP ポート不一致 SYN。\n\n💡 診断: ポートが期待されるサブフローパラメータと一致しない。",
        ),
        ("net/netstat", "MPTcpExt_MismatchPortAckRx") => (
            "MPTCP ポート不一致ジョイン ACK",
            "予期しないポートで受信した MPTCP ジョイン ACK。",
            "MPTCP ポート不一致 ACK。\n\n💡 診断: サブフロージョイン中のポート不一致。",
        ),
        ("net/netstat", "MPTcpExt_RmAddr") => (
            "MPTCP RM_ADDR 受信",
            "アドレスを削除する MPTCP RM_ADDR オプションの受信。",
            "MPTCP RM_ADDR 受信。\n\n💡 診断: ピアが以前に広告したアドレスを削除中。",
        ),
        ("net/netstat", "MPTcpExt_RmAddrDrop") => (
            "MPTCP RM_ADDR ドロップ",
            "破棄された MPTCP RM_ADDR オプション。",
            "MPTCP RM_ADDR ドロップ。\n\n💡 診断: アドレス削除リクエストを処理できなかった。",
        ),
        ("net/netstat", "MPTcpExt_RmSubflow") => (
            "MPTCP サブフロー削除",
            "RM_ADDR 後に削除された MPTCP サブフロー。",
            "MPTCP サブフロー削除。\n\n💡 診断: アドレス削除によりサブフローが切断。",
        ),
        ("net/netstat", "MPTcpExt_MPPrioTx") => (
            "MPTCP MP_PRIO 送信",
            "送信された MPTCP 優先度変更オプション。",
            "MPTCP MP_PRIO 送信。\n\n💡 診断: サブフロー優先度（バックアップ vs プライマリ）の変更。",
        ),
        ("net/netstat", "MPTcpExt_MPPrioRx") => (
            "MPTCP MP_PRIO 受信",
            "受信した MPTCP 優先度変更オプション。",
            "MPTCP MP_PRIO 受信。\n\n💡 診断: ピアがサブフロー優先度を変更中。",
        ),
        ("net/netstat", "MPTcpExt_MPFailTx") => (
            "MPTCP MP_FAIL 送信",
            "送信された MPTCP 障害通知。",
            "MPTCP MP_FAIL 送信。\n\n💡 診断: ピアに MPTCP レベルの障害を通知。",
        ),
        ("net/netstat", "MPTcpExt_MPFailRx") => (
            "MPTCP MP_FAIL 受信",
            "ピアから受信した MPTCP 障害通知。",
            "MPTCP MP_FAIL 受信。\n\n💡 診断: ピアが MPTCP 障害を報告。フォールバックをトリガーする可能性。",
        ),
        ("net/netstat", "MPTcpExt_MPFastcloseTx") => (
            "MPTCP MP_FASTCLOSE 送信",
            "接続を即座に切断する MPTCP 高速クローズオプションの送信。",
            "MPTCP FASTCLOSE 送信。\n\n💡 診断: 即座の接続切断をリクエスト。",
        ),
        ("net/netstat", "MPTcpExt_MPFastcloseRx") => (
            "MPTCP MP_FASTCLOSE 受信",
            "ピアから受信した MPTCP 高速クローズオプション。",
            "MPTCP FASTCLOSE 受信。\n\n💡 診断: ピアが即座の接続切断をリクエスト。",
        ),
        ("net/netstat", "MPTcpExt_MPRstTx") => (
            "MPTCP MP_RST 送信",
            "送信された MPTCP リセットオプション。",
            "MPTCP RST 送信。\n\n💡 診断: MPTCP サブフローをリセット。",
        ),
        ("net/netstat", "MPTcpExt_MPRstRx") => (
            "MPTCP MP_RST 受信",
            "受信した MPTCP リセットオプション。",
            "MPTCP RST 受信。\n\n💡 診断: ピアが MPTCP サブフローをリセット。",
        ),
        ("net/netstat", "MPTcpExt_RcvPruned") => (
            "MPTCP 受信データプルーニング",
            "メモリ圧迫により MPTCP 受信データがプルーニング。",
            "MPTCP 受信プルーニング。\n\n💡 診断: メモリ圧迫により MPTCP データロス。",
        ),
        ("net/netstat", "MPTcpExt_SubflowStale") => (
            "MPTCP 停滞サブフロー検出",
            "進行しないと検出された MPTCP サブフロー。",
            "MPTCP 停滞サブフロー。\n\n💡 診断: サブフローが前進していない。ルート変更の可能性。",
        ),
        ("net/netstat", "MPTcpExt_SubflowRecover") => (
            "MPTCP 停滞サブフロー回復",
            "回復してデータ転送を再開した MPTCP 停滞サブフロー。",
            "MPTCP サブフロー回復。\n\n💡 診断: 以前停滞していたサブフローが再び動作中。",
        ),
        ("net/netstat", "MPTcpExt_SndWndShared") => (
            "MPTCP 共有送信ウィンドウイベント",
            "サブフロー間の MPTCP 送信ウィンドウ共有イベント。",
            "MPTCP 共有送信ウィンドウ。\n\n💡 診断: サブフロー間のウィンドウ管理。",
        ),
        ("net/netstat", "MPTcpExt_RcvWndShared") => (
            "MPTCP 共有受信ウィンドウイベント",
            "サブフロー間の MPTCP 受信ウィンドウ共有イベント。",
            "MPTCP 共有受信ウィンドウ。\n\n💡 診断: サブフロー間のウィンドウ管理。",
        ),
        ("net/netstat", "MPTcpExt_RcvWndConflictUpdate") => (
            "MPTCP 受信ウィンドウ競合更新",
            "更新により解決された MPTCP 受信ウィンドウ競合。",
            "MPTCP ウィンドウ競合更新。\n\n💡 診断: サブフロー間のウィンドウネゴシエーションが調整を必要とした。",
        ),
        ("net/netstat", "MPTcpExt_RcvWndConflict") => (
            "MPTCP 受信ウィンドウ競合",
            "サブフロー間の MPTCP 受信ウィンドウ競合。",
            "MPTCP ウィンドウ競合。\n\n💡 診断: サブフローが受信ウィンドウについて不一致。MPTCP スタックが解決。",
        ),

        _ => return None,
    })
}

/// Returns related metrics for a given source+field.
/// Format: Vec<(source, field, reason)>
pub fn see_also(
    locale: Locale,
    source: &str,
    field: &str,
) -> Vec<(&'static str, &'static str, &'static str)> {
    match locale {
        Locale::En => see_also_en(source, field),
        Locale::Ja => see_also_ja(source, field),
    }
}

fn see_also_en(source: &str, field: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match (source, field) {
        // meminfo fields
        ("meminfo", "MemAvailable") => vec![
            ("meminfo", "MemFree", "Actual free memory"),
            ("meminfo", "Cached", "Reclaimable cache"),
            ("meminfo", "SwapFree", "Remaining swap"),
            ("vmstat", "pgfault", "Page fault frequency"),
            ("pressure", "memory_some_avg10", "Memory pressure"),
        ],
        ("meminfo", "MemFree") => vec![
            ("meminfo", "MemAvailable", "Effective available memory"),
            ("meminfo", "Cached", "Reclaimable cache"),
            ("meminfo", "Buffers", "Buffer cache"),
            ("vmstat", "pgfault", "Page fault frequency"),
        ],
        ("meminfo", "MemTotal") => vec![
            ("meminfo", "MemAvailable", "Available memory"),
            ("meminfo", "SwapTotal", "Total swap space"),
            ("processes", "processes", "Per-process memory usage"),
        ],
        ("meminfo", "Cached") => vec![
            ("meminfo", "MemAvailable", "Available including cache"),
            ("meminfo", "Dirty", "Pending writes"),
            ("meminfo", "SReclaimable", "Reclaimable slab"),
            ("vmstat", "pgfault", "Page fault rate"),
        ],
        ("meminfo", "SwapFree") => vec![
            ("meminfo", "SwapTotal", "Total swap space"),
            ("meminfo", "MemAvailable", "Available memory"),
            ("vmstat", "pswpin", "Swap-in pages"),
            ("vmstat", "pswpout", "Swap-out pages"),
        ],
        ("meminfo", "SwapTotal") => vec![
            ("meminfo", "SwapFree", "Available swap"),
            ("vmstat", "pswpin", "Swap-in activity"),
            ("vmstat", "pswpout", "Swap-out activity"),
        ],
        ("meminfo", "Dirty") => vec![
            ("diskstats", "devices", "Disk I/O"),
            ("pressure", "io_some_avg10", "I/O pressure"),
            ("meminfo", "Writeback", "Writeback in progress"),
        ],

        // loadavg
        ("loadavg", "load1") => vec![
            ("stat", "cpu_user", "CPU usage"),
            ("pressure", "cpu_some_avg10", "CPU pressure"),
            ("stat", "procs_running", "Running process count"),
            ("loadavg", "load15", "Long-term trend"),
        ],
        ("loadavg", "load5") => vec![
            ("loadavg", "load1", "Short-term load"),
            ("loadavg", "load15", "Long-term load"),
            ("stat", "cpu_user", "CPU usage"),
        ],
        ("loadavg", "load15") => vec![
            ("loadavg", "load1", "Short-term trend"),
            ("stat", "cpu_user", "CPU usage"),
            ("pressure", "cpu_some_avg10", "CPU pressure"),
        ],

        // stat
        ("stat", "cpu_user") => vec![
            ("stat", "cpu_system", "Kernel CPU time"),
            ("stat", "cpu_iowait", "I/O wait time"),
            ("loadavg", "load1", "Load average"),
            ("pressure", "cpu_some_avg10", "CPU pressure"),
        ],
        ("stat", "cpu_iowait") => vec![
            ("pressure", "io_some_avg10", "I/O pressure"),
            ("diskstats", "devices", "Disk I/O stats"),
            ("meminfo", "Dirty", "Pending writes"),
            ("vmstat", "pgmajfault", "Major page faults"),
        ],
        ("stat", "procs_running") => vec![
            ("loadavg", "load1", "Load average"),
            ("stat", "cpu_user", "CPU usage"),
            ("pressure", "cpu_some_avg10", "CPU pressure"),
        ],
        ("stat", "cpu_system") => vec![
            ("stat", "cpu_user", "User CPU time"),
            ("stat", "ctxt", "Context switches"),
            ("interrupts", "interrupt_total", "Hardware interrupts"),
        ],

        // net/tcp
        ("net/tcp", "connections") => vec![
            ("net/sockstat", "TCP_tw", "TIME_WAIT count"),
            ("net/snmp", "Tcp_RetransSegs", "Retransmissions"),
            ("ss", "tcp_orphaned", "Orphaned sockets"),
            ("file-nr", "fd_usage_pct", "FD usage"),
        ],

        // processes
        ("processes", "processes") => vec![
            ("stat", "procs_running", "Running count"),
            ("loadavg", "load1", "Load average"),
            ("meminfo", "MemAvailable", "Available memory"),
            ("vmstat", "oom_kill", "OOM kill count"),
        ],

        // pressure
        ("pressure", "cpu_some_avg10") => vec![
            ("loadavg", "load1", "Load average"),
            ("stat", "cpu_user", "CPU user time"),
            ("stat", "procs_running", "Running processes"),
        ],
        ("pressure", "memory_some_avg10") => vec![
            ("meminfo", "MemAvailable", "Available memory"),
            ("vmstat", "pswpout", "Swap-out activity"),
            ("vmstat", "pgmajfault", "Major page faults"),
            ("vmstat", "oom_kill", "OOM kill count"),
        ],
        ("pressure", "io_some_avg10") => vec![
            ("diskstats", "devices", "Disk I/O"),
            ("stat", "cpu_iowait", "I/O wait CPU"),
            ("meminfo", "Dirty", "Pending writes"),
        ],

        // df
        ("df", "root_use_pct") => vec![
            ("diskstats", "devices", "Disk I/O"),
            ("pressure", "io_some_avg10", "I/O pressure"),
            ("meminfo", "Dirty", "Pending write pages"),
        ],

        // vmstat
        ("vmstat", "pgfault") => vec![
            ("vmstat", "pgmajfault", "Major page faults"),
            ("meminfo", "MemAvailable", "Available memory"),
            ("pressure", "memory_some_avg10", "Memory pressure"),
        ],
        ("vmstat", "pgmajfault") => vec![
            ("meminfo", "MemAvailable", "Available memory"),
            ("vmstat", "pswpin", "Swap-in"),
            ("pressure", "memory_some_avg10", "Memory pressure"),
        ],
        ("vmstat", "pswpin") => vec![
            ("vmstat", "pswpout", "Swap-out"),
            ("meminfo", "SwapFree", "Available swap"),
            ("meminfo", "MemAvailable", "Available memory"),
        ],
        ("vmstat", "pswpout") => vec![
            ("vmstat", "pswpin", "Swap-in"),
            ("meminfo", "SwapFree", "Available swap"),
            ("pressure", "memory_some_avg10", "Memory pressure"),
        ],
        ("vmstat", "oom_kill") => vec![
            ("meminfo", "MemAvailable", "Available memory"),
            ("meminfo", "SwapFree", "Available swap"),
            ("vmstat", "pswpout", "Swap-out activity"),
            ("processes", "processes", "Process list"),
        ],

        // thermal
        ("thermal", "max_temp") => vec![
            ("stat", "cpu_user", "CPU usage"),
            ("loadavg", "load1", "Load average"),
            ("pressure", "cpu_some_avg10", "CPU pressure"),
        ],

        // file-nr
        ("file-nr", "fd_usage_pct") => vec![
            ("net/tcp", "connections", "TCP connections"),
            ("processes", "processes", "Process list"),
            ("net/sockstat", "TCP_tw", "TIME_WAIT sockets"),
        ],

        // net/sockstat
        ("net/sockstat", "TCP_tw") => vec![
            ("net/tcp", "connections", "TCP connections"),
            ("net/snmp", "Tcp_RetransSegs", "Retransmissions"),
            ("ss", "tcp_orphaned", "Orphaned sockets"),
        ],

        // net/snmp
        ("net/snmp", "Tcp_RetransSegs") => vec![
            ("net/tcp", "connections", "TCP connections"),
            ("net/snmp", "Tcp_OutSegs", "TCP segments sent"),
            ("net/dev", "tx_bytes", "Network TX"),
        ],

        // ss
        ("ss", "tcp_orphaned") => vec![
            ("net/tcp", "connections", "TCP connections"),
            ("file-nr", "fd_usage_pct", "FD usage"),
            ("net/sockstat", "TCP_tw", "TIME_WAIT count"),
        ],

        // diskstats
        ("diskstats", "devices") => vec![
            ("df", "root_use_pct", "Disk usage"),
            ("pressure", "io_some_avg10", "I/O pressure"),
            ("stat", "cpu_iowait", "I/O wait CPU"),
        ],

        _ => vec![],
    }
}

fn see_also_ja(source: &str, field: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match (source, field) {
        // meminfo fields
        ("meminfo", "MemAvailable") => vec![
            ("meminfo", "MemFree", "実際の空きメモリ"),
            ("meminfo", "Cached", "回収可能なキャッシュ"),
            ("meminfo", "SwapFree", "スワップ残量"),
            ("vmstat", "pgfault", "ページフォルト頻度"),
            ("pressure", "memory_some_avg10", "メモリ圧力"),
        ],
        ("meminfo", "MemFree") => vec![
            ("meminfo", "MemAvailable", "実効利用可能メモリ"),
            ("meminfo", "Cached", "回収可能なキャッシュ"),
            ("meminfo", "Buffers", "バッファキャッシュ"),
            ("vmstat", "pgfault", "ページフォルト頻度"),
        ],
        ("meminfo", "MemTotal") => vec![
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("meminfo", "SwapTotal", "スワップ合計"),
            ("processes", "processes", "プロセス別メモリ使用量"),
        ],
        ("meminfo", "Cached") => vec![
            ("meminfo", "MemAvailable", "キャッシュ込みの利用可能量"),
            ("meminfo", "Dirty", "書き込み待ちページ"),
            ("meminfo", "SReclaimable", "回収可能スラブ"),
            ("vmstat", "pgfault", "ページフォルト率"),
        ],
        ("meminfo", "SwapFree") => vec![
            ("meminfo", "SwapTotal", "スワップ合計"),
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("vmstat", "pswpin", "スワップインページ"),
            ("vmstat", "pswpout", "スワップアウトページ"),
        ],
        ("meminfo", "SwapTotal") => vec![
            ("meminfo", "SwapFree", "スワップ残量"),
            ("vmstat", "pswpin", "スワップイン活動"),
            ("vmstat", "pswpout", "スワップアウト活動"),
        ],
        ("meminfo", "Dirty") => vec![
            ("diskstats", "devices", "ディスクI/O"),
            ("pressure", "io_some_avg10", "I/O圧力"),
            ("meminfo", "Writeback", "書き戻し中"),
        ],

        // loadavg
        ("loadavg", "load1") => vec![
            ("stat", "cpu_user", "CPU使用率"),
            ("pressure", "cpu_some_avg10", "CPU圧力"),
            ("stat", "procs_running", "実行中プロセス数"),
            ("loadavg", "load15", "長期トレンド"),
        ],
        ("loadavg", "load5") => vec![
            ("loadavg", "load1", "短期負荷"),
            ("loadavg", "load15", "長期負荷"),
            ("stat", "cpu_user", "CPU使用率"),
        ],
        ("loadavg", "load15") => vec![
            ("loadavg", "load1", "短期トレンド"),
            ("stat", "cpu_user", "CPU使用率"),
            ("pressure", "cpu_some_avg10", "CPU圧力"),
        ],

        // stat
        ("stat", "cpu_user") => vec![
            ("stat", "cpu_system", "カーネルCPU時間"),
            ("stat", "cpu_iowait", "I/O待ち時間"),
            ("loadavg", "load1", "負荷平均"),
            ("pressure", "cpu_some_avg10", "CPU圧力"),
        ],
        ("stat", "cpu_iowait") => vec![
            ("pressure", "io_some_avg10", "I/O圧力"),
            ("diskstats", "devices", "ディスクI/O統計"),
            ("meminfo", "Dirty", "書き込み待ちページ"),
            ("vmstat", "pgmajfault", "メジャーページフォルト"),
        ],
        ("stat", "procs_running") => vec![
            ("loadavg", "load1", "負荷平均"),
            ("stat", "cpu_user", "CPU使用率"),
            ("pressure", "cpu_some_avg10", "CPU圧力"),
        ],
        ("stat", "cpu_system") => vec![
            ("stat", "cpu_user", "ユーザーCPU時間"),
            ("stat", "ctxt", "コンテキストスイッチ"),
            ("interrupts", "interrupt_total", "ハードウェア割り込み"),
        ],

        // net/tcp
        ("net/tcp", "connections") => vec![
            ("net/sockstat", "TCP_tw", "TIME_WAIT数"),
            ("net/snmp", "Tcp_RetransSegs", "再送数"),
            ("ss", "tcp_orphaned", "孤立ソケット"),
            ("file-nr", "fd_usage_pct", "FD使用率"),
        ],

        // processes
        ("processes", "processes") => vec![
            ("stat", "procs_running", "実行中プロセス数"),
            ("loadavg", "load1", "負荷平均"),
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("vmstat", "oom_kill", "OOMキル回数"),
        ],

        // pressure
        ("pressure", "cpu_some_avg10") => vec![
            ("loadavg", "load1", "負荷平均"),
            ("stat", "cpu_user", "CPUユーザー時間"),
            ("stat", "procs_running", "実行中プロセス数"),
        ],
        ("pressure", "memory_some_avg10") => vec![
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("vmstat", "pswpout", "スワップアウト活動"),
            ("vmstat", "pgmajfault", "メジャーページフォルト"),
            ("vmstat", "oom_kill", "OOMキル回数"),
        ],
        ("pressure", "io_some_avg10") => vec![
            ("diskstats", "devices", "ディスクI/O"),
            ("stat", "cpu_iowait", "I/O待ちCPU"),
            ("meminfo", "Dirty", "書き込み待ちページ"),
        ],

        // df
        ("df", "root_use_pct") => vec![
            ("diskstats", "devices", "ディスクI/O"),
            ("pressure", "io_some_avg10", "I/O圧力"),
            ("meminfo", "Dirty", "書き込み待ちページ"),
        ],

        // vmstat
        ("vmstat", "pgfault") => vec![
            ("vmstat", "pgmajfault", "メジャーページフォルト"),
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("pressure", "memory_some_avg10", "メモリ圧力"),
        ],
        ("vmstat", "pgmajfault") => vec![
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("vmstat", "pswpin", "スワップイン"),
            ("pressure", "memory_some_avg10", "メモリ圧力"),
        ],
        ("vmstat", "pswpin") => vec![
            ("vmstat", "pswpout", "スワップアウト"),
            ("meminfo", "SwapFree", "スワップ残量"),
            ("meminfo", "MemAvailable", "利用可能メモリ"),
        ],
        ("vmstat", "pswpout") => vec![
            ("vmstat", "pswpin", "スワップイン"),
            ("meminfo", "SwapFree", "スワップ残量"),
            ("pressure", "memory_some_avg10", "メモリ圧力"),
        ],
        ("vmstat", "oom_kill") => vec![
            ("meminfo", "MemAvailable", "利用可能メモリ"),
            ("meminfo", "SwapFree", "スワップ残量"),
            ("vmstat", "pswpout", "スワップアウト活動"),
            ("processes", "processes", "プロセス一覧"),
        ],

        // thermal
        ("thermal", "max_temp") => vec![
            ("stat", "cpu_user", "CPU使用率"),
            ("loadavg", "load1", "負荷平均"),
            ("pressure", "cpu_some_avg10", "CPU圧力"),
        ],

        // file-nr
        ("file-nr", "fd_usage_pct") => vec![
            ("net/tcp", "connections", "TCP接続"),
            ("processes", "processes", "プロセス一覧"),
            ("net/sockstat", "TCP_tw", "TIME_WAITソケット"),
        ],

        // net/sockstat
        ("net/sockstat", "TCP_tw") => vec![
            ("net/tcp", "connections", "TCP接続"),
            ("net/snmp", "Tcp_RetransSegs", "再送数"),
            ("ss", "tcp_orphaned", "孤立ソケット"),
        ],

        // net/snmp
        ("net/snmp", "Tcp_RetransSegs") => vec![
            ("net/tcp", "connections", "TCP接続"),
            ("net/snmp", "Tcp_OutSegs", "TCP送信セグメント"),
            ("net/dev", "tx_bytes", "ネットワーク送信"),
        ],

        // ss
        ("ss", "tcp_orphaned") => vec![
            ("net/tcp", "connections", "TCP接続"),
            ("file-nr", "fd_usage_pct", "FD使用率"),
            ("net/sockstat", "TCP_tw", "TIME_WAIT数"),
        ],

        // diskstats
        ("diskstats", "devices") => vec![
            ("df", "root_use_pct", "ディスク使用率"),
            ("pressure", "io_some_avg10", "I/O圧力"),
            ("stat", "cpu_iowait", "I/O待ちCPU"),
        ],

        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[&str] = &[
        T::SOURCE,
        T::DRILL_IN,
        T::BACK,
        T::DIFF,
        T::SEARCH,
        T::REFRESH,
        T::GRAPH,
        T::AUTO,
        T::EXPORT,
        T::QUIT,
        T::HELP,
        T::LANG,
        T::AGO,
        T::SNAPS,
        T::AXIS,
        T::AXIS_AUTO,
        T::AXIS_ZERO,
        T::VIEW_OVERVIEW,
        T::VIEW_DETAIL,
        T::VIEW_DIFF,
        T::VIEW_TABLE,
        T::VIEW_GRAPH,
        T::VIEW_DASHBOARD,
        T::VIEW_WELCOME,
        T::WELCOME_NAV,
        T::WELCOME_DRILL,
        T::WELCOME_DIFF,
        T::WELCOME_SEARCH,
        T::WELCOME_GRAPH,
        T::WELCOME_HELP,
        T::WELCOME_LANG,
        T::WELCOME_CTA,
        T::FIELD,
        T::VALUE,
        T::UNIT,
        T::DESCRIPTION,
        T::OLD,
        T::NEW,
        T::NO_DATA,
        T::NO_CHANGES,
        T::NO_TABLE_DATA,
        T::EXPORTED,
        T::EXPORT_FAILED,
        T::SEARCHING,
    ];

    const ALL_SOURCES: &[&str] = &[
        "meminfo",
        "uptime",
        "loadavg",
        "version",
        "cpuinfo",
        "stat",
        "mounts",
        "partitions",
        "diskstats",
        "processes",
        "swaps",
        "net/dev",
        "net/tcp",
        "net/udp",
        "net/unix",
        "net/arp",
        "net/route",
        "net/sockstat",
        "net/snmp",
        "net/netstat",
        "net/wireless",
        "vmstat",
        "buddyinfo",
        "zoneinfo",
        "slabinfo",
        "pagetypeinfo",
        "modules",
        "interrupts",
        "softirqs",
        "schedstat",
        "timer_list",
        "pressure",
        "cgroups",
        "cmdline",
        "consoles",
        "crypto",
        "devices",
        "filesystems",
        "iomem",
        "ioports",
        "locks",
        "misc",
        "dma",
        "df",
        "thermal",
        "file-nr",
        "gpu",
        "systemd",
    ];

    // T8: All EN keys return non-"?" values
    #[test]
    fn all_en_keys_have_translations() {
        for key in ALL_KEYS {
            let val = t(Locale::En, key);
            assert_ne!(val, "?", "EN translation missing for key '{}'", key);
        }
    }

    // T9: All JA keys return non-"?" values
    #[test]
    fn all_ja_keys_have_translations() {
        for key in ALL_KEYS {
            let val = t(Locale::Ja, key);
            assert_ne!(val, "?", "JA translation missing for key '{}'", key);
        }
    }

    // T10: source_description returns non-default for all known sources
    #[test]
    fn source_descriptions_complete_en() {
        for source in ALL_SOURCES {
            let desc = source_description(Locale::En, source);
            assert_ne!(
                desc, "System information source",
                "EN source_description missing for '{}'",
                source
            );
            assert!(
                !desc.is_empty(),
                "EN source_description empty for '{}'",
                source
            );
        }
    }

    #[test]
    fn source_descriptions_complete_ja() {
        for source in ALL_SOURCES {
            let desc = source_description(Locale::Ja, source);
            assert_ne!(
                desc, "システム情報ソース",
                "JA source_description missing for '{}'",
                source
            );
            assert!(
                !desc.is_empty(),
                "JA source_description empty for '{}'",
                source
            );
        }
    }

    #[test]
    fn locale_from_str_variants() {
        assert_eq!(Locale::from_str("ja"), Locale::Ja);
        assert_eq!(Locale::from_str("jp"), Locale::Ja);
        assert_eq!(Locale::from_str("japanese"), Locale::Ja);
        assert_eq!(Locale::from_str("en"), Locale::En);
        assert_eq!(Locale::from_str("unknown"), Locale::En);
        assert_eq!(Locale::from_str(""), Locale::En);
    }

    #[test]
    fn locale_next_toggles() {
        assert_eq!(Locale::En.next(), Locale::Ja);
        assert_eq!(Locale::Ja.next(), Locale::En);
    }

    #[test]
    fn locale_name_returns_short_code() {
        assert_eq!(Locale::En.name(), "EN");
        assert_eq!(Locale::Ja.name(), "JA");
    }
}
