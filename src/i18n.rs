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

    // View names
    pub const VIEW_OVERVIEW: &str = "view_overview";
    pub const VIEW_DETAIL: &str = "view_detail";
    pub const VIEW_DIFF: &str = "view_diff";
    pub const VIEW_TABLE: &str = "view_table";
    pub const VIEW_GRAPH: &str = "view_graph";

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

        "view_overview" => "OVERVIEW",
        "view_detail" => "DETAIL",
        "view_diff" => "DIFF",
        "view_table" => "TABLE",
        "view_graph" => "GRAPH",

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

        "view_overview" => "概要",
        "view_detail" => "詳細",
        "view_diff" => "差分",
        "view_table" => "テーブル",
        "view_graph" => "グラフ",

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
        "ip/neighbor" => "ARP/NDP neighbor cache from 'ip neighbor show': IP-to-MAC mappings and states",
        "ss" => "Socket statistics summary from 'ss -s': TCP established/timewait/orphaned, UDP counts",
        "dns" => "DNS configuration from /etc/resolv.conf: nameservers, search domains, resolution timing",
        "conntrack" => "Connection tracking table usage: current count, max limit, utilization percentage",
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
        "ip/route" => "IPルーティングテーブル (ip route show): デフォルトゲートウェイ、経路、メトリクス",
        "ip/neighbor" => "ARP/NDPネイバーキャッシュ (ip neighbor show): IP-MAC対応と状態",
        "ss" => "ソケット統計サマリ (ss -s): TCP確立/タイムウェイト/孤立、UDP数",
        "dns" => "DNS設定 (/etc/resolv.conf): ネームサーバ、検索ドメイン、名前解決時間",
        "conntrack" => "コネクション追跡テーブル使用状況: 現在数、上限、使用率",
        _ => "システム情報ソース",
    }
}

/// Field-level descriptions with detail levels.
/// Returns (normal, detailed, extra_detailed) for the given source+field.
/// Returns None if no override exists (fall back to parser's hardcoded description).
pub fn field_description(locale: Locale, source: &str, field: &str) -> Option<(&'static str, &'static str, &'static str)> {
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
            "Total usable RAM (physical memory minus kernel-reserved regions).\n\nWhen MemAvailable drops below 10% of MemTotal, the OOM Killer may start terminating processes. On a 64GB server, that means trouble below ~6.4GB available.\n\n💡 Diagnostic: If MemTotal seems lower than your physical RAM, check BIOS memory reservations or `dmesg | grep Memory`."
        ),
        ("meminfo", "MemFree") => (
            "Free memory (completely unused)",
            "Memory not used by anything. This is often low on healthy systems because Linux uses free RAM for disk cache. Don't panic if it's low — check MemAvailable instead.",
            "Completely unused memory — not even used for caches.\n\nA common misconception: 'My server has no free memory!' In reality, Linux aggressively uses free RAM as disk cache (Cached + Buffers). This is good — unused RAM is wasted RAM.\n\n💡 Diagnostic: If both MemFree AND MemAvailable are low, then you have a real memory problem. If only MemFree is low but MemAvailable is fine, your system is healthy."
        ),
        ("meminfo", "MemAvailable") => (
            "Available memory for new processes",
            "Estimated memory available without swapping. This accounts for reclaimable cache and buffers. This is the number you should watch, not MemFree.",
            "The kernel's estimate of how much memory can be allocated without swapping.\n\nThis is THE metric to watch for memory pressure. It includes free memory plus reclaimable page cache minus reserved watermarks.\n\n💡 Diagnostic:\n  • < 10% of MemTotal → OOM danger zone, investigate immediately\n  • < 20% → Warning, check for memory leaks (RSS growing processes)\n  • Steadily decreasing → Likely a memory leak. Run `ps aux --sort=-rss | head` to find the culprit."
        ),
        ("meminfo", "Buffers") => (
            "Memory used for block device buffers",
            "Memory used for raw block device I/O (metadata like filesystem structures). Usually small. Will be reclaimed under memory pressure.",
            "Raw block device I/O buffers (filesystem metadata, superblocks, etc.).\n\nThis is separate from page cache (Cached). Buffers hold disk metadata, while Cached holds file content. Both are reclaimable.\n\n💡 If Buffers is unusually large, you may have many block devices or heavy metadata operations (like `find /` or massive directory listings)."
        ),
        ("meminfo", "Cached") => (
            "Memory used for file cache",
            "Page cache — files read from disk are kept in memory for faster subsequent access. This is reclaimable under memory pressure. High values are normal and good.",
            "Page cache — the kernel's file content cache.\n\nEvery file read from disk is cached here. This is why 'free' memory looks low on a healthy Linux system — the kernel is doing its job by caching frequently accessed files.\n\n💡 Diagnostic: If Cached drops to near-zero while the system is under load, you're thrashing — the kernel can't keep files in cache because RAM is genuinely exhausted. This causes massive I/O amplification."
        ),
        ("meminfo", "SwapTotal") => (
            "Total swap space",
            "Total swap space available (disk-based virtual memory). Used when physical RAM is exhausted.",
            "Total configured swap space.\n\nSwap acts as overflow for RAM — inactive pages get written to disk to free up physical memory. Some swap usage is normal, but heavy swap activity (check `vmstat si/so`) means RAM is insufficient.\n\n💡 Diagnostic:\n  • SwapTotal = 0 → No swap configured. OOM Killer is your only safety net.\n  • Swap on SSD → Acceptable. Swap on spinning disk → Severe performance hit under pressure."
        ),
        ("meminfo", "SwapFree") => (
            "Free swap space",
            "Unused swap space. If this is decreasing over time, the system is under memory pressure and actively swapping.",
            "Remaining unused swap.\n\n💡 Diagnostic:\n  • SwapTotal - SwapFree > 0 but stable → Some pages were swapped out once, normal.\n  • SwapFree steadily decreasing → Active swapping, system is under real memory pressure.\n  • SwapFree = 0 → Next allocation that can't fit in RAM triggers OOM Killer."
        ),

        // loadavg
        ("loadavg", "load1") => (
            "1-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 1 minute. Compare with CPU count to assess saturation.",
            "1-minute CPU load average.\n\nLoad average counts processes that are: (1) currently running on a CPU, or (2) waiting for CPU time, or (3) in uninterruptible I/O wait (D state).\n\n💡 Diagnostic:\n  • load1 < CPU_count → System has spare capacity\n  • load1 ≈ CPU_count → Fully utilized, acceptable\n  • load1 > CPU_count × 2 → Severely overloaded. Processes are queuing.\n  • load1 >> load15 → Load is spiking RIGHT NOW\n  • load1 << load15 → Load is recovering from a recent spike"
        ),
        ("loadavg", "load5") => (
            "5-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 5 minutes. More stable than 1-minute average.",
            "5-minute load average — the 'trend' indicator.\n\nCompare load1 vs load5 to see direction:\n  • load1 > load5 → Load is increasing\n  • load1 < load5 → Load is decreasing\n  • load1 ≈ load5 → Stable load"
        ),
        ("loadavg", "load15") => (
            "15-minute load average",
            "Average number of processes in runnable or uninterruptible state over the last 15 minutes. Best for understanding baseline system load.",
            "15-minute load average — the 'baseline'.\n\nThis tells you what 'normal' looks like for this system. If load15 is always around 4.0, then load1 jumping to 8.0 is notable but load1 at 4.5 is routine.\n\n💡 If load15 has been high for a long time, the system may need more CPUs, or there's a persistent I/O bottleneck (check `pressure` for PSI data)."
        ),

        // net/tcp
        ("net/tcp", "connections") => (
            "Active TCP connections",
            "Table of all TCP connections with local/remote addresses and states. Key states: ESTABLISHED (active), TIME_WAIT (closing), SYN_SENT (connecting).",
            "All TCP connections on the system.\n\n💡 Diagnostic patterns by connection state:\n  • Many SYN_SENT → Outbound connections timing out. Target host is down, firewall dropping packets, or DNS is slow.\n  • Many TIME_WAIT → Server is handling lots of short-lived connections. Normal for HTTP servers, but excessive counts (>10000) may exhaust ephemeral ports.\n  • Many CLOSE_WAIT → YOUR application isn't closing sockets. This is a bug — the remote end closed, but your code never called close(). Classic FD leak.\n  • Many ESTABLISHED to same IP → Connection pooling or persistent connections. Normal for database connections."
        ),

        // processes
        ("processes", "processes") => (
            "Running processes",
            "Table of all processes: PID, name, state, RSS memory, threads, UID. States: S=sleeping, R=running, Z=zombie, D=uninterruptible I/O.",
            "All processes on the system.\n\n💡 Diagnostic patterns:\n  • Z (Zombie) processes accumulating → Parent process not calling wait(). The zombies themselves use no resources, but indicate a buggy parent.\n  • D (Disk sleep) processes stuck → Uninterruptible I/O wait. Often NFS hangs, disk failures, or kernel driver bugs. These can't be killed with SIGKILL.\n  • High RSS on a single process growing over time → Memory leak. Compare RSS now vs. 1 hour ago.\n  • Many threads (>1000) on one process → Thread leak or thread-per-connection architecture under high load.\n  • UID 0 processes → Running as root. Security concern if unexpected."
        ),

        // pressure (PSI)
        ("pressure", "cpu_some_avg10") => (
            "CPU pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on CPU in the last 10 seconds. Values above 25% indicate CPU contention.",
            "CPU PSI (Pressure Stall Information) — 10-second average.\n\n'some' means at least one runnable task couldn't get CPU time. This is more precise than load average because it directly measures stall time.\n\n💡 Diagnostic:\n  • < 5% → Healthy, plenty of CPU headroom\n  • 5-25% → Moderate pressure, some tasks waiting\n  • > 25% → Significant CPU contention, performance degrading\n  • > 50% → Severe — half the time, tasks are waiting for CPU"
        ),
        ("pressure", "memory_some_avg10") => (
            "Memory pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on memory in the last 10 seconds. Non-zero values indicate memory pressure or active swapping.",
            "Memory PSI — 10-second average.\n\nUnlike CPU pressure, ANY non-zero memory pressure is noteworthy. It means the system is actively reclaiming memory or swapping.\n\n💡 Diagnostic:\n  • 0% → No memory pressure at all\n  • > 0% → System is reclaiming cache or swapping. Check MemAvailable.\n  • > 10% → Significant memory pressure. Tasks are stalling due to memory reclaim.\n  • > 40% → Critical. System is thrashing. Performance severely degraded."
        ),
        ("pressure", "io_some_avg10") => (
            "I/O pressure: some tasks stalled (10s avg)",
            "Percentage of time at least one task was stalled on I/O in the last 10 seconds. Indicates disk bottleneck.",
            "I/O PSI — 10-second average.\n\nMeasures how often tasks are blocked waiting for disk/storage I/O.\n\n💡 Diagnostic:\n  • < 5% → Normal I/O activity\n  • 5-20% → I/O is becoming a bottleneck. Check diskstats for high await times.\n  • > 20% → Significant I/O bottleneck. Consider: SSD upgrade, I/O scheduler tuning, reducing write amplification, or adding RAM for larger page cache.\n  • Spikes correlating with load spikes → I/O-bound workload. Load average includes D-state processes, so high load + high I/O pressure = I/O problem, not CPU."
        ),

        // uptime
        ("uptime", "uptime") => (
            "System uptime since boot",
            "Total time the system has been running since last boot. Includes time spent in suspend/hibernate on some kernels.",
            "System uptime since boot.\n\nThis is the wall-clock time since the kernel started. It does NOT reset on suspend on modern kernels (3.x+), but behavior varies.\n\n💡 Diagnostic:\n  • Very short uptime on a production server → Unexpected reboot. Check `dmesg` and `/var/log/kern.log` for panic/oops.\n  • Extremely long uptime (months/years) → Kernel may be missing critical security patches. Plan a maintenance window."
        ),
        ("uptime", "idle") => (
            "Total idle time across all CPUs",
            "Cumulative time all CPUs spent idle since boot. On multi-core systems, this can exceed uptime because it sums all cores.",
            "Total idle time summed across all CPU cores.\n\nOn a 4-core system, if idle = 3 * uptime, that means on average 3 out of 4 cores were idle.\n\n💡 Diagnostic: idle < uptime on a multi-core system is impossible — if you see this, the value may be from a single-core VM or something is wrong with the clock source."
        ),
        ("uptime", "idle_pct") => (
            "Idle percentage (idle / uptime)",
            "Ratio of total idle time to uptime. On multi-core systems this is (idle / uptime * 100), so values above 100% are normal for mostly-idle machines.",
            "Idle percentage = idle_time / uptime * 100.\n\nBecause idle is summed across all CPUs, this can exceed 100% on multi-core systems. A 4-core idle system would show ~400%.\n\n💡 Diagnostic:\n  • idle_pct / cpu_count < 20% → System is heavily loaded\n  • idle_pct / cpu_count > 80% → System is mostly idle\n  • Values dropping over time → Workload is increasing"
        ),

        // version
        ("version", "raw") => (
            "Full kernel version string",
            "Complete /proc/version output including kernel version, build host, compiler, and build date.",
            "The full kernel version string as reported by the kernel.\n\nContains kernel version, who built it, which compiler was used, and when.\n\n💡 Useful for verifying that the correct kernel is running after an update, or for support tickets that need the exact kernel build info."
        ),
        ("version", "kernel_version") => (
            "Kernel version number",
            "The Linux kernel version (e.g., 6.6.87). This determines available features, syscall support, and driver compatibility.",
            "The Linux kernel version (e.g., 6.6.87).\n\nVersion format: major.minor.patch. The minor number determines the feature set, and patches contain bug/security fixes.\n\n💡 Diagnostic:\n  • Check against CVE databases to ensure critical vulnerabilities are patched.\n  • Kernel < 5.x may lack modern features like PSI (pressure stall info), io_uring, etc.\n  • Compare with `uname -r` — they should match."
        ),
        ("version", "compiler") => (
            "Compiler used to build the kernel",
            "The compiler (typically GCC) and its version used to compile the running kernel. Relevant for debugging ABI compatibility.",
            "The compiler that built this kernel.\n\nTypically GCC, but some distributions use Clang/LLVM. The compiler version matters for:\n  • ABI compatibility with out-of-tree kernel modules (e.g., NVIDIA drivers)\n  • Compiler-specific optimizations affecting performance\n\n💡 If kernel modules fail to load with 'version magic' errors, compiler mismatch is a common cause."
        ),

        // cmdline
        ("cmdline", "cmdline") => (
            "Kernel boot parameters",
            "The command line passed to the kernel at boot by the bootloader (GRUB, systemd-boot, etc.). Controls hardware settings, security features, and debug options.",
            "Kernel boot command line from the bootloader.\n\nCommon important parameters:\n  • root= — Root filesystem device\n  • quiet/splash — Suppress boot messages\n  • nomodeset — Disable kernel mode setting (GPU troubleshooting)\n  • mitigations=off — Disable CPU vulnerability mitigations (insecure but faster)\n  • crashkernel= — Memory reserved for kdump\n\n💡 Diagnostic: If performance is unexpectedly poor, check for missing 'mitigations=off' or debug parameters that add overhead."
        ),
        ("cmdline", "param_count") => (
            "Number of boot parameters",
            "Count of space-separated kernel boot parameters. Useful for a quick overview of boot config complexity.",
            "Number of space-separated tokens in the kernel command line.\n\n💡 A very large param_count may indicate custom kernel tuning or workarounds for hardware issues. Minimal systems typically have 5-15 parameters."
        ),

        // stat
        ("stat", "cpu_user") => (
            "CPU time in user mode",
            "Cumulative time (in jiffies) all CPUs spent executing user-space code since boot. Includes application code but not kernel syscalls.",
            "Cumulative user-mode CPU time since boot (jiffies = typically 1/100th sec).\n\nUser time includes all application code execution. This is the 'us' column in `top`.\n\n💡 Diagnostic:\n  • High user% with low system% → Application is CPU-bound (computation-heavy)\n  • Compare with cpu_system: user >> system is normal for compute workloads\n  • Watch the rate of change between snapshots, not the absolute value"
        ),
        ("stat", "cpu_system") => (
            "CPU time in kernel mode",
            "Cumulative time all CPUs spent in kernel (system) code since boot. Includes syscalls, interrupts, and kernel threads.",
            "Cumulative kernel-mode CPU time since boot.\n\nSystem time = time spent in syscalls, interrupt handlers, and kernel threads. The 'sy' column in `top`.\n\n💡 Diagnostic:\n  • High system% → Heavy syscall usage (lots of I/O, context switches, or network ops)\n  • system > user → Unusual. Could indicate excessive syscalls (e.g., millions of tiny reads instead of buffered I/O)\n  • Sudden system% spike → Check for interrupt storms or kernel driver issues"
        ),
        ("stat", "cpu_idle") => (
            "CPU idle time",
            "Cumulative time all CPUs spent idle since boot. High idle means the CPU has spare capacity.",
            "Cumulative CPU idle time since boot.\n\nThe CPU was doing nothing and no tasks were runnable.\n\n💡 Diagnostic:\n  • Idle steadily near 0 → CPU is fully saturated\n  • Compare idle rate between snapshots: (idle_delta / total_delta * 100) gives real-time idle%\n  • On multi-core: this is summed, so max idle per second = number_of_cores * 100 jiffies"
        ),
        ("stat", "cpu_iowait") => (
            "CPU time waiting for I/O",
            "Cumulative time CPUs spent idle while waiting for outstanding I/O. High iowait indicates a storage bottleneck.",
            "CPU time spent waiting for I/O completion.\n\niowait means the CPU had nothing to do AND there was outstanding I/O. This is a subset of idle — the CPU is idle, but blocked on disk.\n\n💡 Diagnostic:\n  • High iowait → Storage is the bottleneck, not CPU\n  • iowait can appear low on busy systems because other tasks fill the CPU while I/O completes\n  • Compare with pressure io_some_avg10 for a more accurate I/O bottleneck signal\n  • Spikes → Large sequential reads/writes or filesystem journal flushes"
        ),
        ("stat", "cpu_usage_pct") => (
            "Overall CPU usage percentage (cumulative)",
            "Percentage of CPU time spent doing useful work since boot. Calculated as (total - idle - iowait) / total * 100.",
            "Cumulative CPU usage percentage since boot.\n\nThis is (busy_time / total_time * 100) where busy = user + nice + system + irq + softirq + steal.\n\n💡 Note: This is a cumulative average since boot, not real-time usage. For real-time CPU%, compare delta values between two snapshots. A system that was idle for 23 hours then 100% busy for 1 hour will show ~4% here."
        ),
        ("stat", "forks_total") => (
            "Total forks (process creations) since boot",
            "Number of times fork()/clone() has been called since boot. A high rate indicates many short-lived processes.",
            "Total fork()/clone() calls since boot.\n\nEvery process or thread creation increments this counter.\n\n💡 Diagnostic:\n  • High fork rate (delta between snapshots) → Shell scripts spawning many subprocesses, cron jobs, or a fork-bomb\n  • Compare with context_switches: high forks + high context switches = lots of short-lived processes\n  • Steady growth is normal; sudden spikes warrant investigation"
        ),
        ("stat", "procs_running") => (
            "Processes currently running on CPU",
            "Number of processes in the R (running/runnable) state right now. Consistently above CPU count means the CPU is overloaded.",
            "Number of processes currently in R (running) state.\n\nThese are either actively executing on a CPU or in the run queue waiting for CPU time.\n\n💡 Diagnostic:\n  • procs_running <= cpu_count → Normal, CPUs can service all runners\n  • procs_running > cpu_count → Tasks are queuing for CPU time\n  • Persistently high → CPU bottleneck, correlates with load average"
        ),
        ("stat", "procs_blocked") => (
            "Processes blocked on I/O",
            "Number of processes in the D (uninterruptible sleep) state. These are waiting for I/O and cannot be interrupted, even by signals.",
            "Processes in D state (uninterruptible sleep), blocked on I/O.\n\nThese processes are waiting for disk, network filesystem (NFS), or device I/O.\n\n💡 Diagnostic:\n  • procs_blocked > 0 temporarily → Normal during I/O operations\n  • procs_blocked persistently high → I/O bottleneck. Check diskstats and pressure.\n  • D-state processes stuck for minutes → NFS hang, dead disk, or kernel driver bug. These CANNOT be killed with SIGKILL."
        ),
        ("stat", "context_switches") => (
            "Total context switches since boot",
            "Number of CPU context switches since boot. Each switch saves/restores process state. A very high rate can indicate excessive multitasking overhead.",
            "Total CPU context switches since boot.\n\nA context switch happens when the CPU changes from one process/thread to another. Both voluntary (blocking I/O) and involuntary (preemption) switches are counted.\n\n💡 Diagnostic:\n  • Normal rate: 1000-50000/sec depending on workload\n  • > 100000/sec → High. Many threads contending or excessive I/O operations.\n  • Correlates with high system% CPU — each switch has kernel overhead\n  • Compare delta between snapshots for the current rate"
        ),

        // cpuinfo
        ("cpuinfo", "logical_cpus") => (
            "Number of logical CPUs (threads)",
            "Total logical processors visible to the OS. Includes hyperthreading — two logical CPUs may share one physical core.",
            "Total logical CPUs (hardware threads) visible to the OS.\n\nWith Hyperthreading/SMT enabled, logical_cpus = physical_cores * 2 (typically). Without HT, logical_cpus = physical_cores.\n\n💡 Diagnostic:\n  • Compare with cores_per_socket to detect Hyperthreading\n  • This is the number you compare load average against\n  • If lower than expected, check BIOS settings or kernel parameters (maxcpus=, nr_cpus=)"
        ),
        ("cpuinfo", "model") => (
            "CPU model name",
            "Full CPU model identifier as reported by the processor. Includes brand, generation, and variant information.",
            "CPU model name from the processor's CPUID instruction.\n\nExamples: 'Intel(R) Core(TM) i9-13900K', 'AMD EPYC 9654'.\n\n💡 Useful for:\n  • Identifying hardware generation and expected performance\n  • Checking if the CPU supports required instruction sets (AVX-512, etc.)\n  • Verifying that VMs are exposing the correct CPU model"
        ),
        ("cpuinfo", "frequency") => (
            "Current CPU frequency in MHz",
            "Current operating frequency of the CPU. May vary due to frequency scaling (turbo boost, power saving).",
            "Current CPU frequency in MHz.\n\nModern CPUs dynamically adjust frequency based on load (P-states). The reported value may be:\n  • Base frequency under light load (power saving)\n  • Turbo/boost frequency under heavy load\n  • Capped by thermal throttling\n\n💡 Diagnostic:\n  • Frequency much lower than rated → Check governor: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`\n  • 'powersave' governor → May limit performance. Switch to 'performance' for benchmarks.\n  • Frequency not reaching turbo → Thermal throttling. Check CPU temperature."
        ),
        ("cpuinfo", "cache_size") => (
            "CPU cache size (L2/L3)",
            "Size of the last-level cache reported per core. Larger caches improve performance for memory-intensive workloads.",
            "Last-level cache size as reported by /proc/cpuinfo.\n\nThis is typically the L3 cache shared across cores, or L2 per-core on some architectures.\n\n💡 Diagnostic:\n  • Larger cache = better performance for working sets that fit\n  • Database workloads benefit significantly from large L3 cache\n  • For detailed cache topology, check `/sys/devices/system/cpu/cpu0/cache/`"
        ),
        ("cpuinfo", "cores_per_socket") => (
            "Physical cores per CPU socket",
            "Number of physical CPU cores per socket. With Hyperthreading, each core appears as 2 logical CPUs.",
            "Physical CPU cores per socket.\n\nPhysical cores have their own execution units. Hyperthreading creates 2 logical CPUs per physical core by sharing execution resources.\n\n💡 Diagnostic:\n  • logical_cpus / cores_per_socket = 2 → Hyperthreading is enabled\n  • logical_cpus / cores_per_socket = 1 → Hyperthreading is disabled or unavailable\n  • For HPC/latency-sensitive workloads, disabling HT can sometimes improve performance"
        ),
        ("cpuinfo", "key_flags") => (
            "Key CPU feature flags",
            "Important CPU capability flags: SSE/AVX (SIMD), aes (encryption), vmx/svm (virtualization), ht (hyperthreading), lm (64-bit).",
            "Key CPU feature flags from /proc/cpuinfo.\n\nNotable flags:\n  • sse/sse2/avx/avx2/avx512f → SIMD instruction sets (important for numeric workloads)\n  • aes → Hardware AES encryption (critical for TLS performance)\n  • vmx (Intel) / svm (AMD) → Hardware virtualization support\n  • ht → Hyperthreading capable\n  • lm → Long mode (64-bit support)\n  • nx → No-eXecute bit (security: prevents executing data pages)\n  • hypervisor → Running inside a VM\n\n💡 Diagnostic: 'hypervisor' flag present → This is a VM. Missing 'vmx'/'svm' → Nested virtualization not available."
        ),

        // vmstat
        ("vmstat", "pgfault") => (
            "Total page faults since boot",
            "Total minor + major page faults. Minor faults are resolved from memory; major faults require disk I/O.",
            "Total page faults (minor + major) since boot.\n\nMinor faults: page exists in memory but not mapped in the process's page table. Resolved instantly — normal and frequent.\nMajor faults: page must be read from disk. These are expensive.\n\n💡 Diagnostic: High pgfault is normal. Focus on pgmajfault for real I/O-causing faults."
        ),
        ("vmstat", "pgmajfault") => (
            "Major page faults (required disk I/O)",
            "Page faults that required reading from disk. Each one causes a process stall. High rates indicate insufficient RAM or cold cache.",
            "Major page faults since boot — each one required disk I/O.\n\nA major fault means the requested page was not in RAM and had to be fetched from storage. This stalls the faulting process.\n\n💡 Diagnostic:\n  • High pgmajfault rate (delta/sec) → RAM insufficient for working set, or cold start\n  • Correlates with high iowait and I/O pressure\n  • After a fresh boot, major faults spike as applications load — this is normal\n  • Persistent high rate → Add more RAM or reduce working set size"
        ),
        ("vmstat", "pgpgin") => (
            "Pages paged in from disk",
            "Total pages read from block devices into memory since boot. Includes normal file I/O and demand paging.",
            "Pages paged in from disk (in 1KB units).\n\nIncludes both file I/O (read() syscall causing page cache fill) and demand paging (loading executable pages).\n\n💡 Compare pgpgin rate with pgpgout rate: if pgpgin >> pgpgout, the workload is read-heavy."
        ),
        ("vmstat", "pgpgout") => (
            "Pages paged out to disk",
            "Total pages written from memory to block devices since boot. Includes dirty page writeback and swap-out.",
            "Pages paged out to disk (in 1KB units).\n\nIncludes dirty page cache writeback (normal) and swap page-out (memory pressure).\n\n💡 Diagnostic: To distinguish normal writeback from swap pressure, check pswpout separately. High pgpgout with low pswpout = normal file writes."
        ),
        ("vmstat", "pswpin") => (
            "Pages swapped in from swap",
            "Pages read back from swap into memory. Non-zero means the system previously swapped out and is now retrieving those pages.",
            "Pages swapped in from swap space.\n\nSwap-in happens when a process accesses a page that was previously swapped out due to memory pressure.\n\n💡 Diagnostic:\n  • pswpin rate > 0 → System is actively reading from swap. Performance impact depends on swap device speed.\n  • High pswpin + high pswpout → Thrashing — pages are being constantly swapped in and out. Critical condition.\n  • pswpin but no current pswpout → Recovering pages from a past memory pressure event. May be transient."
        ),
        ("vmstat", "pswpout") => (
            "Pages swapped out to swap",
            "Pages moved from RAM to swap. Active swap-out indicates current memory pressure.",
            "Pages swapped out to swap space.\n\nSwap-out happens when the kernel needs to free RAM and has exhausted reclaimable caches.\n\n💡 Diagnostic:\n  • pswpout rate > 0 → Active memory pressure RIGHT NOW\n  • High sustained pswpout → System needs more RAM\n  • Intermittent pswpout → Temporary memory spikes, may be acceptable"
        ),
        ("vmstat", "nr_free_pages") => (
            "Free memory pages",
            "Number of completely free pages in the system. Low values are normal — Linux uses free pages for caching.",
            "Number of free (unused) memory pages.\n\nSimilar to MemFree in meminfo. Low values are expected because Linux uses free memory for page cache.\n\n💡 Diagnostic:\n  • Free pages below the 'min' watermark → Kernel enters direct reclaim, allocations may stall\n  • Check zoneinfo for per-zone free vs. watermark comparison"
        ),
        ("vmstat", "nr_active_anon") => (
            "Active anonymous pages",
            "Anonymous (non-file-backed) pages on the active LRU list. These are recently accessed heap/stack pages.",
            "Anonymous pages on the active LRU (Least Recently Used) list.\n\nAnonymous pages = process heap, stack, mmap(MAP_ANONYMOUS). 'Active' means recently accessed.\n\n💡 These pages can only be freed by swapping. High active anon = processes using lots of heap memory."
        ),
        ("vmstat", "nr_inactive_anon") => (
            "Inactive anonymous pages",
            "Anonymous pages not recently accessed. These are candidates for swap-out under memory pressure.",
            "Anonymous pages on the inactive LRU list — not recently accessed.\n\nThese are first candidates for swap-out when memory pressure occurs.\n\n💡 Diagnostic: Large inactive_anon with no swap activity → Memory that could be reclaimed if needed. Large inactive_anon with active swapping → These pages are being swapped out."
        ),
        ("vmstat", "nr_active_file") => (
            "Active file-backed pages",
            "File-backed pages recently accessed (page cache hot pages). These cache file contents for fast re-reads.",
            "File-backed pages on the active LRU — recently used page cache.\n\nThese pages cache file contents that were recently read. They improve I/O performance by avoiding disk reads.\n\n💡 Healthy systems have a large active file cache. Shrinking active_file under load means cache is being evicted due to memory pressure."
        ),
        ("vmstat", "nr_inactive_file") => (
            "Inactive file-backed pages",
            "File-backed pages not recently accessed. Can be quickly reclaimed without I/O (unless dirty).",
            "File-backed pages on the inactive LRU — not recently accessed.\n\nThese are the first to be reclaimed under memory pressure, and reclaim is cheap (just drop the page, unless dirty).\n\n💡 Diagnostic: Large inactive_file = good buffer against memory pressure. Small inactive_file = less room for reclaim before swapping starts."
        ),
        ("vmstat", "nr_dirty") => (
            "Dirty pages (modified, not yet written)",
            "Pages modified in memory but not yet written to disk. These will be written by the kernel's writeback mechanism.",
            "Pages with modifications not yet flushed to disk.\n\nThe kernel periodically flushes dirty pages (controlled by /proc/sys/vm/dirty_writeback_centisecs). If the system crashes, dirty pages are lost.\n\n💡 Diagnostic:\n  • High nr_dirty → Heavy write workload or slow storage\n  • nr_dirty above dirty_ratio → Processes will block on write() until pages are flushed (write throttling)\n  • Persistently high → Storage cannot keep up with write rate"
        ),
        ("vmstat", "nr_writeback") => (
            "Pages currently being written to disk",
            "Pages actively being flushed to storage right now. High values indicate heavy I/O activity.",
            "Pages currently being written back to storage.\n\nThese pages are in-flight to disk. The number depends on storage speed and write volume.\n\n💡 Diagnostic:\n  • nr_writeback > 0 most of the time → Constant write pressure\n  • Very high nr_writeback → Storage device is saturated, writes are backing up\n  • Check with diskstats for device-level I/O metrics"
        ),
        ("vmstat", "nr_slab_reclaimable") => (
            "Reclaimable slab pages",
            "Kernel slab allocator pages that can be freed under memory pressure. Includes dentry cache and inode cache.",
            "Reclaimable slab memory (kernel caches).\n\nPrimarily dentry cache (directory entries) and inode cache. These speed up filesystem operations and are released when memory is needed.\n\n💡 Diagnostic: Very large reclaimable slab on a fileserver is normal — it's caching millions of directory entries. The kernel will shrink it automatically under pressure."
        ),
        ("vmstat", "nr_slab_unreclaimable") => (
            "Unreclaimable slab pages",
            "Kernel slab pages that cannot be freed. These are active kernel data structures that must remain in memory.",
            "Non-reclaimable slab memory — active kernel objects.\n\nThese are kernel data structures in active use (task structs, network buffers, etc.) that cannot be freed.\n\n💡 Diagnostic:\n  • Steadily growing unreclaimable slab → Possible kernel memory leak\n  • Check slabinfo for which specific caches are growing\n  • Large unreclaimable slab with many network connections → Network buffer memory"
        ),
        ("vmstat", "oom_kill") => (
            "OOM killer invocations",
            "Number of times the OOM (Out of Memory) killer has been triggered since boot. Each invocation kills a process to free memory.",
            "OOM (Out of Memory) killer invocations since boot.\n\nThe OOM killer is the kernel's last resort — when all memory (RAM + swap) is exhausted, it kills a process to survive.\n\n💡 Diagnostic:\n  • Any non-zero value deserves investigation\n  • Check `dmesg | grep -i oom` for details on which processes were killed and why\n  • Prevent OOM: add swap, increase RAM, set memory limits via cgroups\n  • Protect critical processes: `echo -1000 > /proc/<pid>/oom_score_adj`"
        ),

        // buddyinfo
        ("buddyinfo", "zone_count") => (
            "Number of memory zones",
            "Count of memory zones in the buddy allocator. Common zones: DMA, DMA32, Normal, and optionally Movable.",
            "Number of memory zones tracked by the buddy allocator.\n\nTypical zones:\n  • DMA — First 16MB, for legacy ISA devices\n  • DMA32 — First 4GB, for 32-bit DMA devices\n  • Normal — Main memory zone\n  • Movable — Pages that can be migrated (for memory hotplug)\n\n💡 More zones on NUMA systems (one set per NUMA node)."
        ),
        ("buddyinfo", "zones") => (
            "Free page chunks per zone and order",
            "Memory fragmentation data: for each zone, shows free chunks at each order (0-10). Order N = 2^N contiguous pages (4KB each).",
            "Buddy allocator free page counts per zone per order.\n\nOrder 0 = 4KB, Order 1 = 8KB, ... Order 10 = 4MB. Higher orders represent larger contiguous free blocks.\n\n💡 Diagnostic:\n  • Many order-0 pages but zero higher orders → Memory is fragmented. Large allocations will fail or require compaction.\n  • Zero pages at all orders in a zone → Zone is exhausted\n  • Important for huge pages: 2MB huge pages need order-9 (x86). If order-9 count is 0, transparent huge pages will fail.\n  • Run `echo 1 > /proc/sys/vm/compact_memory` to trigger compaction."
        ),

        // zoneinfo
        ("zoneinfo", "zone_count") => (
            "Number of memory zones",
            "Count of memory zones with detailed watermark information. Each zone has min/low/high watermarks controlling reclaim behavior.",
            "Number of memory zones in zoneinfo.\n\nEach zone has watermarks that control page reclaim:\n  • min — Below this, allocation stalls (direct reclaim)\n  • low — kswapd starts background reclaim\n  • high — kswapd stops reclaiming\n\n💡 More zones appear on NUMA systems with multiple nodes."
        ),
        ("zoneinfo", "zones") => (
            "Per-zone memory details (free, min, low, high)",
            "Table of memory zones showing free pages and watermark thresholds. When free drops below 'low', background reclaim starts.",
            "Detailed per-zone memory information.\n\nColumns: zone identifier, free pages, min watermark, low watermark, high watermark.\n\n💡 Diagnostic:\n  • free < min → Direct reclaim active. Allocations are stalling — processes may block.\n  • free < low → kswapd is running (background reclaim). Normal under moderate pressure.\n  • free > high → No memory pressure in this zone.\n  • Check each zone independently — memory pressure can be zone-specific (e.g., DMA32 exhausted while Normal is fine)."
        ),

        // slabinfo
        ("slabinfo", "cache_count") => (
            "Number of slab caches",
            "Total number of kernel slab allocator caches. Each cache serves a specific type of kernel object (inodes, dentries, buffers, etc.).",
            "Number of active slab caches in the kernel.\n\nThe slab allocator provides efficient allocation for fixed-size kernel objects. Each cache pools objects of the same type.\n\n💡 Diagnostic: A very high cache count may indicate many loaded kernel modules, each registering their own caches."
        ),
        ("slabinfo", "caches") => (
            "Slab cache details",
            "Table of slab caches: name, active objects, total objects, object size, objects per slab, pages per slab.",
            "Detailed slab cache statistics.\n\nColumns: cache name, active objects, total allocated objects, object size (bytes), objects per slab page, pages per slab.\n\n💡 Diagnostic:\n  • dentry cache very large → Lots of filesystem paths cached. Normal on fileservers.\n  • inode_cache growing → Many unique files accessed.\n  • active_objs << num_objs → Memory waste. Many pre-allocated but unused objects.\n  • Look for unknown caches growing → Possible kernel module memory leak.\n  • To reclaim slab caches: `echo 2 > /proc/sys/vm/drop_caches` (dentry+inode only)."
        ),

        // pagetypeinfo
        ("pagetypeinfo", "entry_count") => (
            "Number of pagetype info entries",
            "Count of entries in the page type breakdown. Each entry shows free page counts per migration type per zone.",
            "Number of pagetype info entries.\n\nEntries are broken down by NUMA node, memory zone, and migration type (Unmovable, Movable, Reclaimable, etc.).\n\n💡 Migration types affect defragmentation: Movable pages can be relocated to create contiguous blocks."
        ),
        ("pagetypeinfo", "entries") => (
            "Page allocation type breakdown per zone",
            "Free page counts by migration type (Unmovable, Movable, Reclaimable) per zone per order. Shows how pages are categorized for compaction.",
            "Detailed page allocation type info.\n\nShows how free pages at each order are distributed across migration types:\n  • Unmovable — Cannot be relocated (kernel allocations)\n  • Movable — Can be migrated for compaction (user pages)\n  • Reclaimable — Can be freed (page cache, slab)\n\n💡 Diagnostic:\n  • Lots of Unmovable fragments → Hard to compact. Fragmentation will persist.\n  • Movable pages dominate → Good for transparent huge pages and compaction.\n  • Use with buddyinfo to understand fragmentation causes."
        ),

        // swaps
        ("swaps", "total_size") => (
            "Total swap space available",
            "Combined size of all swap areas (files and partitions). This is the maximum overflow space for RAM.",
            "Total swap space across all swap areas.\n\nSwap extends virtual memory beyond physical RAM. Pages that haven't been accessed recently get moved to swap to free up RAM.\n\n💡 Diagnostic:\n  • 0 bytes → No swap configured. OOM is the only option when RAM is full.\n  • Recommended: at least 1-2GB of swap even on large-RAM systems as a safety net."
        ),
        ("swaps", "total_used") => (
            "Used swap space",
            "Amount of swap currently in use. Some swap usage is normal; watch the trend rather than the absolute value.",
            "Currently used swap space.\n\nSome swap usage is normal and doesn't indicate a problem — the kernel may have swapped out idle pages proactively.\n\n💡 Diagnostic:\n  • Stable used swap → Pages were swapped out once and remain there. Normal.\n  • Growing used swap → Active memory pressure. Check MemAvailable.\n  • Used swap close to total → Danger zone. Next allocation may trigger OOM.\n  • To see which processes use swap: `grep VmSwap /proc/*/status | sort -k2 -n`"
        ),
        ("swaps", "usage_pct") => (
            "Swap usage percentage",
            "Percentage of total swap currently in use. Below 50% is usually fine; above 80% warrants investigation.",
            "Swap usage as a percentage of total swap.\n\n💡 Diagnostic:\n  • 0% → No swap in use (or no swap configured)\n  • < 50% → Normal, especially if stable\n  • 50-80% → Elevated. Monitor the trend.\n  • > 80% → High. System may OOM if memory demand increases.\n  • 100% → Swap is full. Any new memory demand triggers OOM Killer."
        ),
        ("swaps", "swap_areas") => (
            "Individual swap area details",
            "Table of swap areas: filename/device, type (partition/file), size, used, priority. Higher priority areas are used first.",
            "Individual swap area details.\n\nColumns: device/file path, type (partition or file), size, used, priority.\n\n💡 Diagnostic:\n  • Priority determines usage order — higher priority areas are used first\n  • Multiple swap areas with same priority → Round-robin (striped) across them for better performance\n  • Swap on SSD → Acceptable performance. Swap on HDD → Major latency under pressure.\n  • Swap file vs partition → Files are slightly slower but easier to resize"
        ),

        // ── Group 3: Network ──────────────────────────────────────────

        // net/dev
        ("net/dev", "total_rx") => (
            "Total bytes received across all interfaces.",
            "Cumulative receive byte counter for every network interface since boot. Includes loopback traffic.",
            "Total received bytes summed across all interfaces.\n\nThis counter resets on reboot. Compare two snapshots to derive throughput. Includes loopback (lo) traffic, so subtract lo if you need external-only numbers.\n\n💡 Diagnostic: If total_rx grows much faster than total_tx, the host is a consumer (downloads, database reads). The reverse pattern suggests a serving role (web server, NFS export)."
        ),
        ("net/dev", "total_tx") => (
            "Total bytes transmitted across all interfaces.",
            "Cumulative transmit byte counter for every network interface since boot. Includes loopback traffic.",
            "Total transmitted bytes summed across all interfaces.\n\nSame as total_rx but for the send path. Heavy TX with low RX often indicates a content-serving workload.\n\n💡 Diagnostic: A sudden spike in TX with no corresponding application change may indicate a compromised host exfiltrating data or participating in a DDoS amplification attack."
        ),
        ("net/dev", "interface_count") => (
            "Number of network interfaces.",
            "Count of all network interfaces visible in /proc/net/dev, including loopback, virtual, and physical interfaces.",
            "Total network interface count.\n\nIncludes lo (loopback), physical NICs, bridges, veth pairs (containers), tun/tap devices (VPN), and bond interfaces.\n\n💡 Diagnostic: If this number is unexpectedly high, check for container sprawl (each container adds a veth pair). If unexpectedly low, a NIC driver may have failed to load — check `dmesg | grep -i eth`."
        ),
        ("net/dev", "interfaces") => (
            "Per-interface traffic statistics.",
            "Table of all network interfaces showing name, received bytes, received packets, transmitted bytes, and transmitted packets.",
            "Per-interface network traffic breakdown.\n\nColumns: name, RX bytes, RX packets, TX bytes, TX packets.\n\n💡 Diagnostic:\n  • High RX/TX errors → Cable issues, duplex mismatch, or driver bugs\n  • One interface with zero traffic → Link may be down; check `ip link show`\n  • lo with heavy traffic → Lots of inter-process communication on localhost (common for databases)\n  • Large packet counts but small byte counts → Many small packets, possible SYN flood or chatty protocol"
        ),

        // net/udp
        ("net/udp", "socket_count") => (
            "Number of open UDP sockets.",
            "Count of all UDP sockets on the system. Unlike TCP, UDP is connectionless, so each entry represents a bound socket.",
            "Total UDP socket count.\n\nUDP sockets don't have connection states like TCP. Each entry is a socket bound to a local port, optionally associated with a remote address.\n\n💡 Diagnostic:\n  • High count with many different local ports → Possible DNS amplification or UDP-based scanning\n  • Common legitimate UDP users: DNS (port 53), NTP (port 123), SNMP (port 161), syslog (port 514)\n  • If socket_count keeps growing → Possible FD leak in a UDP application"
        ),
        ("net/udp", "sockets") => (
            "Active UDP sockets.",
            "Table of all UDP sockets showing local address, remote address, state, and UID of the owning process.",
            "All UDP sockets on the system.\n\nColumns: local_addr, remote_addr, state, uid.\n\n💡 Diagnostic:\n  • Remote address 0.0.0.0:0 → Socket is listening (not connected to a specific peer)\n  • Many sockets bound to same port → Multiple processes or SO_REUSEPORT\n  • UID 0 sockets → Running as root; check if expected\n  • UDP has no built-in reliability — packet loss is invisible at this level. Use net/snmp Udp_InErrors for drop detection."
        ),

        // net/unix
        ("net/unix", "socket_count") => (
            "Number of Unix domain sockets.",
            "Count of all Unix domain sockets. These are used for fast inter-process communication on the same host.",
            "Total Unix domain socket count.\n\nUnix sockets are the preferred IPC mechanism for local communication (much faster than TCP loopback). Databases, display servers, and systemd all use them heavily.\n\n💡 Diagnostic:\n  • High count is normal on a modern systemd-based system (200+ is typical)\n  • Steadily growing count → Possible socket leak; a process is creating sockets without closing them\n  • Look for sockets with empty path → Abstract namespace sockets (prefixed with @)"
        ),
        ("net/unix", "sockets") => (
            "Unix domain socket details.",
            "Table of all Unix domain sockets showing reference count, type, state, inode, and path.",
            "All Unix domain sockets.\n\nColumns: refcount, type, state, inode, path.\n\n💡 Diagnostic:\n  • Type 1 = STREAM (like TCP), Type 2 = DGRAM (like UDP), Type 5 = SEQPACKET\n  • Sockets with well-known paths: /var/run/dbus/system_bus_socket (D-Bus), /run/systemd/journal/stdout (journald), /var/run/docker.sock (Docker)\n  • High refcount on a single socket → Many processes sharing it (normal for D-Bus)"
        ),

        // net/arp
        ("net/arp", "entry_count") => (
            "Number of ARP table entries.",
            "Count of IP-to-MAC address mappings in the kernel ARP cache. Each entry represents a recently-communicated neighbor on the local network.",
            "ARP cache entry count.\n\nThe ARP table maps IPv4 addresses to MAC (hardware) addresses for hosts on the same L2 network segment.\n\n💡 Diagnostic:\n  • Very high count (1000+) → Large flat network or ARP table poisoning/scanning\n  • Count keeps growing → Possible ARP storm or network scan in progress\n  • Stale entries (flags=0x0) → Neighbor went offline; kernel will eventually expire them"
        ),
        ("net/arp", "entries") => (
            "ARP table entries.",
            "Table of ARP cache entries showing IP address, hardware (MAC) address, flags, and network device.",
            "ARP cache contents.\n\nColumns: ip, hw_addr, flags, device.\n\n💡 Diagnostic:\n  • Flags 0x2 = complete (resolved), 0x6 = complete+permanent (static entry)\n  • Duplicate MAC for different IPs → ARP spoofing or misconfigured network\n  • 00:00:00:00:00:00 MAC → Unresolved entry; host is unreachable at L2\n  • Multiple entries on unexpected interfaces → Possible VLAN or routing misconfiguration"
        ),

        // net/route
        ("net/route", "route_count") => (
            "Number of routing table entries.",
            "Count of entries in the kernel IPv4 routing table. Includes default gateway, directly connected networks, and static routes.",
            "IPv4 routing table entry count.\n\nEach entry tells the kernel how to reach a network: which interface to use and which gateway to forward through.\n\n💡 Diagnostic:\n  • 0 routes → No networking configured; host is isolated\n  • Missing default route (0.0.0.0 destination) → Host cannot reach the internet or other non-local networks\n  • Very high count → Complex routing setup or dynamic routing protocol (OSPF, BGP) injecting routes"
        ),
        ("net/route", "routes") => (
            "Kernel routing table.",
            "Table of routing entries showing interface, destination, gateway, mask, flags, and metric. The default route has destination 0.0.0.0.",
            "IPv4 kernel routing table.\n\nColumns: iface, destination, gateway, mask, flags, metric.\n\n💡 Diagnostic:\n  • Destination 0.0.0.0 with mask 0.0.0.0 → Default route (gateway of last resort)\n  • Gateway 0.0.0.0 → Directly connected network, no gateway needed\n  • Multiple default routes with different metrics → Failover configuration; lower metric = preferred\n  • Flags: U=up, G=gateway, H=host route. Missing U flag means route is down."
        ),

        // net/sockstat
        ("net/sockstat", "sockets_used") => (
            "Total sockets in use.",
            "Total count of all socket types currently allocated by the kernel. This is a high-level indicator of network activity.",
            "Total allocated sockets across all protocols.\n\nThis is the grand total — TCP + UDP + RAW + FRAG + Unix domain sockets.\n\n💡 Diagnostic:\n  • Steadily growing → Possible socket/FD leak\n  • Compare with `ulimit -n` to check how close you are to the per-process FD limit\n  • System-wide limit: /proc/sys/fs/file-max"
        ),
        ("net/sockstat", "TCP_inuse") => (
            "TCP sockets in use.",
            "Number of TCP sockets currently in use (all states except TIME_WAIT). A key indicator of active network connections.",
            "TCP sockets currently in use.\n\nIncludes ESTABLISHED, SYN_SENT, SYN_RECV, FIN_WAIT, CLOSE_WAIT, LAST_ACK, LISTEN states — everything except TIME_WAIT (tracked separately).\n\n💡 Diagnostic:\n  • Compare with TCP_tw (TIME_WAIT) — if tw >> inuse, you have lots of short-lived connections\n  • Approaching /proc/sys/net/ipv4/tcp_max_orphans → Risk of connection drops\n  • Baseline this value; a sudden jump indicates a traffic spike or connection leak"
        ),
        ("net/sockstat", "TCP_orphan") => (
            "Orphaned TCP sockets.",
            "TCP sockets not attached to any process. These consume kernel memory and are waiting to be cleaned up.",
            "Orphaned TCP connections — no longer owned by any user-space process.\n\nOrphans happen when a process closes a socket that still has data in flight. The kernel keeps the socket alive to complete the TCP teardown.\n\n💡 Diagnostic:\n  • High orphan count → Application crashing or exiting without clean shutdown\n  • Limit: /proc/sys/net/ipv4/tcp_max_orphans (default ~16384)\n  • Exceeding the limit causes the kernel to aggressively reset connections, dropping data"
        ),
        ("net/sockstat", "TCP_tw") => (
            "TCP sockets in TIME_WAIT.",
            "TCP connections in TIME_WAIT state, waiting for late packets before fully closing. High counts are normal for busy HTTP servers.",
            "TCP TIME_WAIT socket count.\n\nTIME_WAIT is a normal TCP state — after closing, the socket waits 2*MSL (typically 60 seconds) to catch delayed packets.\n\n💡 Diagnostic:\n  • < 5000 → Normal\n  • 5000-30000 → Heavy short-lived connection workload; consider connection pooling\n  • > 30000 → May exhaust ephemeral ports (check /proc/sys/net/ipv4/ip_local_port_range)\n  • Enable tcp_tw_reuse (sysctl) to allow reuse of TIME_WAIT sockets for new outbound connections"
        ),
        ("net/sockstat", "TCP_alloc") => (
            "TCP sockets allocated.",
            "Total TCP sockets allocated by the kernel, including sockets in all states. This is the total TCP memory footprint.",
            "Total allocated TCP sockets.\n\nThis includes every TCP socket in any state (inuse + TIME_WAIT + orphan + listen).\n\n💡 Diagnostic:\n  • alloc >> inuse + tw → Many sockets in transitional states or kernel overhead\n  • Memory usage per socket: roughly 1-2 KB for basic, more with large buffers\n  • Total TCP memory limits: /proc/sys/net/ipv4/tcp_mem (in pages)"
        ),
        ("net/sockstat", "TCP_mem") => (
            "TCP memory usage (pages).",
            "Kernel memory pages consumed by all TCP sockets. Each page is typically 4KB.",
            "TCP memory consumption in kernel pages.\n\nMultiply by page size (usually 4096 bytes) to get bytes. This memory is used for socket buffers, control structures, and data in flight.\n\n💡 Diagnostic:\n  • Compare with /proc/sys/net/ipv4/tcp_mem thresholds (low, pressure, high)\n  • When usage exceeds the 'pressure' threshold, the kernel starts reducing buffer sizes\n  • When usage exceeds 'high', new allocations may fail → connection drops"
        ),
        ("net/sockstat", "UDP_inuse") => (
            "UDP sockets in use.",
            "Number of UDP sockets currently in use. Includes DNS resolvers, NTP clients, logging daemons, and game servers.",
            "Active UDP sockets.\n\nUDP is stateless at the protocol level, so 'in use' means a socket is bound and ready to send/receive.\n\n💡 Diagnostic:\n  • Typical values: 5-20 on a quiet server\n  • High values → Many UDP services or a DNS/NTP heavy workload\n  • UDP has no congestion control — a flood of UDP traffic can overwhelm the network without any backpressure"
        ),
        ("net/sockstat", "UDP_mem") => (
            "UDP memory usage (pages).",
            "Kernel memory pages consumed by UDP sockets. Usually much smaller than TCP since UDP has no connection state or retransmission buffers.",
            "UDP memory consumption in kernel pages.\n\nMultiply by page size (usually 4096) to get bytes.\n\n💡 Diagnostic:\n  • Compare with /proc/sys/net/ipv4/udp_mem limits\n  • If UDP_mem is high, check for applications with large receive buffers (e.g., video streaming receivers)\n  • Under memory pressure, the kernel drops incoming UDP packets silently — check net/snmp Udp_RcvbufErrors"
        ),
        ("net/sockstat", "FRAG_inuse") => (
            "IP fragment reassembly sockets.",
            "Number of IP fragment reassembly entries. Non-zero values indicate fragmented packets are being received.",
            "IP fragment reassembly queue entries.\n\nIP fragmentation occurs when a packet exceeds the MTU. The kernel holds fragments here until all pieces arrive for reassembly.\n\n💡 Diagnostic:\n  • Usually 0 on modern networks with path MTU discovery\n  • Non-zero → Some path has an MTU mismatch or PMTUD is blocked (ICMP filtered)\n  • Persistent high values → Possible fragmentation attack; check net/snmp Ip_ReasmFails"
        ),

        // net/snmp — key protocol counters
        ("net/snmp", "Tcp_ActiveOpens") => (
            "TCP connections initiated (client-side).",
            "Cumulative count of TCP connections where this host sent the initial SYN. Indicates outbound connection activity.",
            "TCP active opens — connections initiated by this host.\n\nEvery time your system connects to a remote server (HTTP request, database query, SSH), this counter increments.\n\n💡 Diagnostic:\n  • Compare with Tcp_PassiveOpens to understand if the host is primarily a client or server\n  • High rate of ActiveOpens → Busy client workload or connection churn (not pooling connections)\n  • Tcp_AttemptFails / Tcp_ActiveOpens = connection failure rate"
        ),
        ("net/snmp", "Tcp_PassiveOpens") => (
            "TCP connections accepted (server-side).",
            "Cumulative count of TCP connections accepted via listen/accept. Indicates inbound connection activity.",
            "TCP passive opens — connections accepted by listening sockets.\n\nEvery incoming client connection (web request, SSH login, database client) increments this.\n\n💡 Diagnostic:\n  • High PassiveOpens → Busy server\n  • PassiveOpens >> ActiveOpens → Primarily a server role\n  • Sudden drop in PassiveOpens rate → Clients can't reach the service (firewall, DNS, or service crash)"
        ),
        ("net/snmp", "Tcp_RetransSegs") => (
            "TCP segments retransmitted.",
            "Cumulative count of TCP segments retransmitted. Non-zero growth indicates packet loss on the network.",
            "TCP retransmission counter.\n\nRetransmissions occur when a sent segment is not acknowledged within the timeout. This is THE key indicator of network quality issues.\n\n💡 Diagnostic:\n  • RetransSegs / OutSegs = retransmission rate\n  • < 0.1% → Excellent network\n  • 0.1-1% → Moderate packet loss, noticeable latency\n  • > 1% → Severe packet loss. Check for: congested links, faulty cables, overloaded switches, MTU issues\n  • Sudden spike → Network event (link flap, route change, congestion)"
        ),
        ("net/snmp", "Tcp_InErrs") => (
            "TCP segments received with errors.",
            "Cumulative count of TCP segments received with checksum or other errors. Indicates data corruption in transit.",
            "TCP input errors — segments with invalid checksums or other protocol errors.\n\n💡 Diagnostic:\n  • Should be 0 or very close to 0\n  • Non-zero → Data corruption on the network path\n  • Possible causes: faulty NIC, bad cable, memory bit-flip (ECC failure), buggy network driver\n  • Compare with Udp_InErrors to see if the problem is protocol-specific or link-wide"
        ),
        ("net/snmp", "Udp_InErrors") => (
            "UDP datagrams that could not be delivered.",
            "Cumulative count of received UDP datagrams that could not be delivered (no matching socket, buffer overflow, checksum error).",
            "UDP input errors — datagrams that arrived but could not be delivered to an application.\n\n💡 Diagnostic:\n  • Common causes: no process listening on the destination port, receive buffer overflow\n  • Growing steadily → An application is too slow to read its UDP socket, causing kernel to drop packets\n  • Check Udp_RcvbufErrors specifically for buffer overflow drops\n  • For DNS servers: high InErrors = clients are sending queries faster than the server can process"
        ),
        ("net/snmp", "Ip_InReceives") => (
            "Total IP datagrams received.",
            "Cumulative count of all IP datagrams received, including those with errors. The top-level input counter for all network traffic.",
            "Total IP input datagrams — every packet that arrived at this host.\n\nThis is the grand total input counter before any protocol demuxing (TCP, UDP, ICMP).\n\n💡 Diagnostic:\n  • Rate of change (packets/sec) indicates overall network input load\n  • Ip_InReceives - Ip_InDelivers = packets dropped or forwarded\n  • If the host is not a router, InReceives should roughly equal InDelivers"
        ),
        ("net/snmp", "Ip_OutRequests") => (
            "Total IP datagrams sent.",
            "Cumulative count of all IP datagrams handed to the network layer for transmission. The top-level output counter.",
            "Total IP output datagrams — every packet sent by this host.\n\n💡 Diagnostic:\n  • Rate of change indicates overall network output load\n  • Ip_OutRequests >> Ip_InReceives → Host is generating more traffic than receiving (serving role)\n  • Sudden spike → New workload, backup job, or possibly compromised host"
        ),

        // net/netstat — extended TCP/IP stats
        ("net/netstat", "TcpExt_ListenOverflows") => (
            "Times the listen queue overflowed.",
            "Count of times a SYN was received but the listen backlog queue was full. Indicates the server cannot accept connections fast enough.",
            "TCP listen queue overflows.\n\nWhen a client SYN arrives and the server's accept queue (backlog) is full, this counter increments and the connection is dropped.\n\n💡 Diagnostic:\n  • Should be 0 under normal operation\n  • Non-zero → Server is too slow calling accept(), or backlog is too small\n  • Fix: Increase net.core.somaxconn and the application's listen backlog parameter\n  • Also check TcpExt_ListenDrops for total drops"
        ),
        ("net/netstat", "TcpExt_ListenDrops") => (
            "Connections dropped from listen queue.",
            "Count of connections dropped because the listen queue was full. Clients experience connection timeouts when this happens.",
            "TCP listen queue drops — connections lost because the server couldn't keep up.\n\n💡 Diagnostic:\n  • Non-zero = clients are being turned away\n  • Monitor the rate (drops/sec) rather than the absolute value\n  • Common during traffic spikes or application GC pauses\n  • Fix: tune somaxconn, optimize accept() path, reduce per-request latency"
        ),
        ("net/netstat", "TcpExt_TCPTimeouts") => (
            "TCP connection timeouts.",
            "Count of TCP connections that timed out waiting for a response. Indicates network issues or unresponsive remote hosts.",
            "TCP timeout events.\n\nA timeout occurs when a sent segment receives no ACK within the retransmission timeout (RTO). After several retries, the connection is aborted.\n\n💡 Diagnostic:\n  • Correlates with Tcp_RetransSegs but represents the final failure\n  • High timeouts + high retransmissions → Persistent network path failure\n  • High timeouts to specific hosts → Those hosts or the path to them is unreliable\n  • Check if timeouts correlate with specific times of day (congestion patterns)"
        ),

        // net/wireless
        ("net/wireless", "interface_count") => (
            "Number of wireless interfaces.",
            "Count of wireless (Wi-Fi) network interfaces detected by the kernel.",
            "Wireless interface count.\n\n💡 Diagnostic:\n  • 0 → No wireless hardware detected, or driver not loaded (check `lspci` and `modules`)\n  • Typically 1 on a laptop/desktop with Wi-Fi\n  • Multiple interfaces → Wi-Fi card with multiple radios or USB Wi-Fi adapter added"
        ),
        ("net/wireless", "interfaces") => (
            "Wireless interface statistics.",
            "Table of wireless interfaces with signal quality metrics: status, link quality, signal level, and noise level.",
            "Per-interface wireless statistics.\n\nColumns: iface, status, link quality, signal level (dBm), noise level (dBm).\n\n💡 Diagnostic:\n  • Link quality: higher is better (max varies by driver, often 70)\n  • Signal level (dBm): -30 = excellent, -67 = good, -70 = fair, -80 = weak, -90 = unusable\n  • Noise level: lower (more negative) is better\n  • SNR (signal - noise) > 25 dB → Good; < 15 dB → Poor, expect packet loss"
        ),

        // ── Group 4: Storage ──────────────────────────────────────────

        // mounts
        ("mounts", "count") => (
            "Number of mounted filesystems.",
            "Total count of all mounted filesystems, including virtual filesystems like proc, sysfs, tmpfs, and cgroup mounts.",
            "Total mounted filesystem count.\n\nIncludes physical disks, network mounts (NFS/CIFS), and virtual/pseudo filesystems (proc, sysfs, tmpfs, devtmpfs, cgroup).\n\n💡 Diagnostic:\n  • Typical Linux system: 30-60 mounts (many are virtual)\n  • Very high count (200+) → Container host with many mount namespaces, or NFS-heavy environment\n  • Missing expected mount → Filesystem failed to mount at boot; check `dmesg` and `systemctl --failed`"
        ),
        ("mounts", "mounts") => (
            "Mounted filesystem details.",
            "Table of all mounted filesystems showing device, mountpoint, filesystem type, and mount options.",
            "All currently mounted filesystems.\n\nColumns: device, mountpoint, fstype, options.\n\n💡 Diagnostic:\n  • Look for 'ro' in options → Filesystem remounted read-only, usually due to disk errors\n  • 'noatime' option → Reduces I/O by not updating access timestamps (good for SSDs)\n  • 'errors=remount-ro' → ext4 default; filesystem goes read-only on errors rather than continuing with corruption\n  • NFS mounts with 'hard' option → Processes will hang indefinitely if NFS server is unreachable\n  • tmpfs mounts → RAM-backed; check their size limits to avoid memory exhaustion"
        ),

        // partitions
        ("partitions", "count") => (
            "Number of block device partitions.",
            "Total count of all block device partitions recognized by the kernel, including whole disks and their sub-partitions.",
            "Block device partition count.\n\nIncludes whole disks (sda, nvme0n1) and their partitions (sda1, nvme0n1p1), plus device-mapper entries (dm-0), loop devices, and RAM disks.\n\n💡 Diagnostic:\n  • Check that all expected disks appear — a missing disk may indicate hardware failure or driver issue\n  • Unexpected entries → Hot-plugged USB device or new virtual disk\n  • Size of 0 → Partition table entry exists but has no allocated space"
        ),
        ("partitions", "partitions") => (
            "Partition table details.",
            "Table of all block device partitions showing name, size, and major/minor device numbers.",
            "Block device partition table.\n\nColumns: name, size, major, minor.\n\n💡 Diagnostic:\n  • Major 8 = SCSI/SATA disks (sd*), Major 259 = NVMe, Major 253 = device-mapper (LVM/LUKS)\n  • Compare partition sizes with `df` output — large partitions with small filesystems may have wasted space\n  • Loop devices (major 7) → Snap packages or mounted ISO images"
        ),

        // diskstats
        ("diskstats", "active_devices") => (
            "Devices with I/O activity.",
            "Number of block devices that have had at least one read or write operation since boot. Filters out inactive devices.",
            "Active block device count — devices with non-zero I/O.\n\nOnly devices with at least one completed read or write are counted, filtering out inactive loop devices and unused partitions.\n\n💡 Diagnostic:\n  • Compare with total partition count — many inactive devices is normal (loop devices, unused partitions)\n  • If an expected device shows zero I/O → It may not be in use, or the workload hasn't started"
        ),
        ("diskstats", "devices") => (
            "Per-device I/O statistics.",
            "Table of active block devices showing name, read count, bytes read, write count, bytes written, and in-flight I/O operations. Critical for identifying I/O bottlenecks.",
            "Per-device disk I/O statistics.\n\nColumns: name, reads completed, bytes read, writes completed, bytes written, I/O in-flight.\n\n💡 Diagnostic:\n  • in-flight > 0 sustained → Disk is under active I/O load. High sustained in-flight on HDD (>2) suggests saturation.\n  • Reads >> Writes → Read-heavy workload (database scans, file serving). Increase RAM for better page cache.\n  • Writes >> Reads → Write-heavy workload (logging, database inserts). Check if write-back caching is enabled.\n  • Compare whole disk (sda) vs partition (sda1) — the disk-level stats include all partitions.\n  • For SSDs: high write volume degrades lifespan. Check SMART data with `smartctl -a /dev/sdX`.\n  • Correlate with I/O pressure (PSI io_some_avg10) to determine if I/O is actually stalling processes."
        ),

        // locks
        ("locks", "lock_count") => (
            "Number of active file locks.",
            "Count of all active file locks (POSIX and FLOCK types). Locks coordinate access between processes to prevent data corruption.",
            "Active file lock count.\n\nIncludes both POSIX locks (fcntl-based, byte-range granularity) and FLOCK locks (flock-based, whole-file). Also includes lease locks used by Samba.\n\n💡 Diagnostic:\n  • Typical values: 10-50 on a normal server\n  • Very high count → Database or file server under heavy concurrent access\n  • If an application hangs → Check if it's waiting on a lock held by another process. The PID column identifies the holder."
        ),
        ("locks", "locks") => (
            "Active file lock details.",
            "Table of all active file locks showing type (POSIX/FLOCK), mode (READ/WRITE), PID of holder, inode info, and byte range.",
            "All active file locks.\n\nColumns: type, mode, pid, inode_info, range_start, range_end.\n\n💡 Diagnostic:\n  • WRITE locks block all other access — if a process holds a WRITE lock and is stuck, other processes will hang\n  • Range 0 to EOF → Whole-file lock\n  • Multiple READ locks on same inode → Shared read access, normal for concurrent readers\n  • To find which file an inode belongs to: `find / -inum <inode_number>`\n  • Deadlock: Process A holds lock on file X, waits for file Y; Process B holds Y, waits for X"
        ),

        // ── Group 5: Security & Kernel ────────────────────────────────

        // modules
        ("modules", "module_count") => (
            "Number of loaded kernel modules.",
            "Count of currently loaded kernel modules (drivers, filesystem modules, etc.). Modules extend kernel functionality without recompilation.",
            "Loaded kernel module count.\n\nKernel modules are dynamically loadable pieces of kernel code: device drivers, filesystem implementations, network protocols, crypto algorithms.\n\n💡 Diagnostic:\n  • Typical Linux server: 50-150 modules\n  • Unexpected module → Possible rootkit. Compare with known-good list.\n  • Missing expected module → Driver not loaded; try `modprobe <name>` and check `dmesg` for errors"
        ),
        ("modules", "modules") => (
            "Loaded kernel module details.",
            "Table of loaded kernel modules showing name, memory size, reference count, dependencies, and state (Live/Loading/Unloading).",
            "All loaded kernel modules.\n\nColumns: name, size (bytes), refcount, dependencies, state.\n\n💡 Diagnostic:\n  • refcount > 0 → Module is in use and cannot be unloaded\n  • refcount = 0 → Module can be safely unloaded with `rmmod`\n  • State 'Live' → Normal. 'Loading' or 'Unloading' should be transient; if stuck, the module has a bug.\n  • Dependencies show module relationships — unloading requires removing dependents first\n  • Large module size → Significant kernel memory usage. GPU drivers (nvidia, amdgpu) are often the largest."
        ),

        // interrupts
        ("interrupts", "cpu_count") => (
            "Number of CPUs handling interrupts.",
            "Count of CPUs visible in the interrupt table. Each CPU independently handles hardware interrupts.",
            "CPU count from the interrupt table.\n\nThis should match the CPU count from cpuinfo. If it doesn't, some CPUs may be offline.\n\n💡 Diagnostic: Check if interrupts are balanced across CPUs. If one CPU handles most interrupts, performance may suffer. Use `irqbalance` daemon or set IRQ affinity manually."
        ),
        ("interrupts", "irq_count") => (
            "Number of IRQ lines.",
            "Count of distinct interrupt request lines (hardware and software). Each IRQ represents a device or subsystem that can interrupt the CPU.",
            "Total IRQ line count.\n\n💡 Diagnostic:\n  • Common IRQs: timer (IRQ 0), keyboard (IRQ 1), NIC, disk controller, USB\n  • Very high counts of a specific IRQ → That device is very active or misbehaving\n  • Shared IRQs (multiple devices on same line) can cause performance issues on legacy systems"
        ),
        ("interrupts", "interrupts") => (
            "Hardware interrupt counters.",
            "Table of all interrupt lines showing IRQ number, total count across all CPUs, and description of the interrupt source.",
            "Per-IRQ interrupt counters.\n\nColumns: irq, total_count, description.\n\n💡 Diagnostic:\n  • NIC interrupts (usually labeled with driver name like 'ixgbe', 'mlx5') — high count = high network throughput\n  • Timer interrupts (LOC, local APIC timer) — should be roughly equal across CPUs\n  • NMI (Non-Maskable Interrupt) > 0 → Hardware watchdog or performance monitoring event\n  • Spurious IRQ (SPU) > 0 → Hardware issue, usually harmless but worth noting\n  • Unbalanced interrupt distribution across CPUs → Run `irqbalance` or manually set /proc/irq/N/smp_affinity"
        ),

        // softirqs
        ("softirqs", "softirq_count") => (
            "Number of software interrupt types.",
            "Count of distinct softirq types (typically 10). Softirqs handle deferred work like network packet processing and timer callbacks.",
            "Software interrupt type count.\n\nLinux has a fixed set of softirq types: HI, TIMER, NET_TX, NET_RX, BLOCK, IRQ_POLL, TASKLET, SCHED, HRTIMER, RCU.\n\n💡 Diagnostic: The count itself is fixed. What matters is the per-type activity in the softirqs table below."
        ),
        ("softirqs", "softirqs") => (
            "Software interrupt counters.",
            "Table of softirq types and their total invocation count across all CPUs. NET_RX and NET_TX handle network traffic; TIMER handles kernel timers.",
            "Per-type software interrupt counters.\n\nColumns: name, total_count (summed across all CPUs).\n\n💡 Diagnostic:\n  • NET_RX very high → Heavy inbound network traffic processing\n  • NET_TX very high → Heavy outbound network traffic processing\n  • TIMER → Kernel timer callbacks; should be proportional to uptime and HZ\n  • SCHED → Scheduler events; high on busy multi-threaded systems\n  • RCU → Read-Copy-Update synchronization; high is normal on active systems\n  • TASKLET → Deferred work from interrupt handlers; if disproportionately high, a driver may be inefficient\n  • Unbalanced softirq counts across CPUs (check per-CPU view) → IRQ affinity issue; RX processing stuck on one core"
        ),

        // cgroups
        ("cgroups", "controller_count") => (
            "Number of cgroup controllers.",
            "Count of available cgroup (control group) controllers. Cgroups partition processes for resource management (CPU, memory, I/O limits).",
            "Available cgroup controller count.\n\nCgroup controllers manage resource allocation: cpu, memory, blkio (disk I/O), pids, cpuset, devices, etc.\n\n💡 Diagnostic:\n  • Common controllers: cpu, cpuacct, memory, blkio, pids, freezer, devices, net_cls\n  • Missing controllers → Kernel compiled without support, or not mounted\n  • cgroups v2 unified hierarchy vs v1 per-controller hierarchies — check hierarchy column"
        ),
        ("cgroups", "controllers") => (
            "Cgroup controller details.",
            "Table of cgroup controllers showing name, hierarchy ID, number of cgroups in that hierarchy, and whether the controller is enabled.",
            "Cgroup controller table.\n\nColumns: name, hierarchy, num_cgroups, enabled.\n\n💡 Diagnostic:\n  • hierarchy = 0 → Controller not attached to any hierarchy (not in use)\n  • enabled = 1 → Controller is available; 0 → Disabled (kernel parameter or not compiled in)\n  • High num_cgroups → Many containers/services using this controller (normal on container hosts)\n  • memory controller not enabled → No per-cgroup memory limits; a runaway container can consume all host memory"
        ),

        // crypto
        ("crypto", "algorithm_count") => (
            "Number of registered crypto algorithms.",
            "Count of cryptographic algorithms available to the kernel. Includes ciphers, hashes, RNGs, and compression algorithms.",
            "Registered crypto algorithm count.\n\nThe kernel crypto API provides algorithms for IPsec, dm-crypt (LUKS), TLS offload, and internal use.\n\n💡 Diagnostic:\n  • Typical count: 50-200 depending on kernel config and loaded modules\n  • Hardware-accelerated algorithms (driver name contains 'aesni', 'ghash-clmulni') → Faster encryption\n  • If dm-crypt/LUKS is slow → Check that aes-ni is available (grep for 'aes' type 'skcipher')"
        ),
        ("crypto", "algorithms") => (
            "Registered crypto algorithm details.",
            "Table of crypto algorithms showing name, type (skcipher, hash, aead, rng), driver implementation, and source module.",
            "Crypto algorithm table.\n\nColumns: name, type, driver, module.\n\n💡 Diagnostic:\n  • Type 'skcipher' → Symmetric cipher (AES, ChaCha20)\n  • Type 'ahash'/'shash' → Hash algorithm (SHA-256, MD5)\n  • Type 'aead' → Authenticated encryption (AES-GCM)\n  • Type 'rng' → Random number generator\n  • Module 'kernel' → Built-in. Named module → Loadable.\n  • Look for hardware-accelerated drivers: 'aesni-intel', 'ghash-clmulni-intel' for x86; 'aes-ce' for ARM"
        ),

        // devices
        ("devices", "device_count") => (
            "Number of registered device drivers.",
            "Count of all registered character and block device drivers. Each driver handles a class of hardware or virtual devices.",
            "Registered device driver count.\n\nIncludes both character devices (terminals, serial ports, /dev/null) and block devices (disk drivers).\n\n💡 Diagnostic:\n  • Character devices are typically more numerous than block devices\n  • Missing expected device → Driver not loaded or hardware not detected\n  • Major number conflicts → Very rare, but indicates a kernel configuration problem"
        ),
        ("devices", "devices") => (
            "Registered device driver details.",
            "Table of all registered devices showing type (Character/Block), major number, and device name.",
            "Registered device driver table.\n\nColumns: type, major number, name.\n\n💡 Diagnostic:\n  • Key character devices: 1=mem, 4=tty, 5=console, 10=misc, 136=pts\n  • Key block devices: 8=sd (SCSI/SATA), 253=device-mapper, 259=blkext (NVMe)\n  • 'Block' type 'sd' missing → SCSI/SATA driver not loaded\n  • 'Block' type 'nvme' missing → NVMe driver not loaded; check `lspci` for NVMe devices"
        ),

        // filesystems
        ("filesystems", "filesystem_count") => (
            "Number of supported filesystem types.",
            "Count of filesystem types the kernel can mount. Includes both disk-based (ext4, xfs) and virtual (proc, sysfs) filesystems.",
            "Supported filesystem type count.\n\n💡 Diagnostic:\n  • Common disk filesystems: ext4, xfs, btrfs, vfat\n  • Common virtual filesystems: proc, sysfs, tmpfs, devtmpfs, cgroup, cgroup2\n  • Missing expected filesystem (e.g., xfs) → Kernel module not loaded; try `modprobe xfs`\n  • 'nodev' flag → Virtual filesystem, not backed by a block device"
        ),
        ("filesystems", "filesystems") => (
            "Supported filesystem type details.",
            "Table of supported filesystem types showing name and whether it requires a block device (nodev=yes means no device needed).",
            "Supported filesystem table.\n\nColumns: name, nodev.\n\n💡 Diagnostic:\n  • nodev='yes' → Virtual/pseudo filesystem (proc, sysfs, tmpfs). No physical disk needed.\n  • nodev='no' → Disk-based filesystem (ext4, xfs, btrfs). Requires a block device.\n  • If you can't mount a filesystem → Check this list first to confirm kernel support\n  • FUSE (Filesystem in Userspace) appears as 'fuseblk' or 'fuse' → Enables user-space filesystem drivers (sshfs, s3fs, rclone)"
        ),

        // iomem
        ("iomem", "region_count") => (
            "Number of I/O memory regions.",
            "Count of physical memory-mapped I/O regions. Each region is reserved by a device driver or the kernel for hardware communication.",
            "I/O memory region count.\n\nThese are physical address ranges mapped to hardware devices (video RAM, NIC buffers, PCI device BARs) or reserved by the kernel.\n\n💡 Diagnostic:\n  • Useful for understanding physical memory layout and device presence\n  • 'System RAM' regions show usable physical memory\n  • 'reserved' regions → BIOS/UEFI reserved areas, ACPI tables\n  • PCI device names appear here — use to verify hardware detection"
        ),
        ("iomem", "regions") => (
            "I/O memory region map.",
            "Table of memory-mapped I/O regions showing address range and description. Shows the physical memory layout of the system.",
            "Physical I/O memory map.\n\nColumns: address_range, description.\n\n💡 Diagnostic:\n  • 'System RAM' → Physical RAM available to the OS\n  • 'Kernel code/data/bss' → Memory used by the kernel itself\n  • PCI device BARs (Base Address Registers) → Hardware MMIO regions\n  • 'ACPI Tables/Non-volatile Storage' → Firmware data regions\n  • Gaps in address space → Reserved by hardware or firmware\n  • Overlapping regions at different indent levels → Hierarchical allocation (parent device → sub-regions)"
        ),

        // ioports
        ("ioports", "region_count") => (
            "Number of I/O port regions.",
            "Count of I/O port address regions. Legacy x86 mechanism for communicating with hardware devices.",
            "I/O port region count.\n\nI/O ports are a legacy x86 mechanism for CPU-to-device communication. Modern devices prefer memory-mapped I/O (see iomem), but many traditional devices still use ports.\n\n💡 Diagnostic:\n  • Mostly relevant on x86/x86_64 systems\n  • Key ranges: 0x0-0xFF (DMA, PIC, timer), 0x3F8 (COM1 serial), 0x1F0 (primary IDE)\n  • On modern systems, PCI devices claim I/O port ranges through BARs"
        ),
        ("ioports", "regions") => (
            "I/O port region map.",
            "Table of I/O port address regions showing port range and the device or subsystem that owns it.",
            "I/O port address map.\n\nColumns: port_range, description.\n\n💡 Diagnostic:\n  • 0x60, 0x64 → Keyboard controller (i8042)\n  • 0x3F8 → COM1 serial port\n  • 0xCF8-0xCFF → PCI configuration space\n  • PCI device port ranges → Claimed by their drivers\n  • Conflicts (same port claimed by multiple drivers) are rare but catastrophic — causes hardware lockups"
        ),

        // consoles
        ("consoles", "console_count") => (
            "Number of registered console devices.",
            "Count of console devices registered with the kernel. Consoles receive kernel log messages (printk output).",
            "Registered console device count.\n\n💡 Diagnostic:\n  • Typical: 1-3 consoles (tty0 for virtual terminal, ttyS0 for serial, sometimes netconsole)\n  • 0 consoles → Kernel messages go nowhere; debugging will be very difficult\n  • netconsole registered → Kernel messages are being sent over the network (useful for remote debugging)"
        ),
        ("consoles", "consoles") => (
            "Registered console device details.",
            "Table of console devices showing name and flags (read/write permissions, preferred console, etc.).",
            "Console device table.\n\nColumns: name, flags.\n\n💡 Diagnostic:\n  • 'E' flag → Enabled\n  • 'W' flag → Can write (output)\n  • 'R' flag → Can read (input)\n  • Preferred console (last one registered) gets kernel panic output\n  • For headless servers: ensure a serial console (ttyS0) is configured for remote crash debugging\n  • Boot parameter 'console=ttyS0,115200' enables serial console"
        ),

        // misc
        ("misc", "device_count") => (
            "Number of misc devices.",
            "Count of miscellaneous character devices. Misc devices share major number 10 and include various kernel features.",
            "Miscellaneous device count.\n\nMisc devices are a grab-bag of character devices that share major number 10. They include hardware watchdogs, random number generators, and various kernel interfaces.\n\n💡 Diagnostic:\n  • Common misc devices: cpu_dma_latency, hpet, hwrng, loop-control, vhost-net\n  • 'watchdog' → Hardware watchdog timer; reboots the system if it hangs\n  • 'fuse' → FUSE (user-space filesystem) support is available"
        ),
        ("misc", "devices") => (
            "Misc device details.",
            "Table of miscellaneous devices showing minor number and device name. All share major number 10.",
            "Miscellaneous device table.\n\nColumns: minor_number, name.\n\n💡 Diagnostic:\n  • Minor number uniquely identifies each misc device under major 10\n  • Look for 'watchdog' (minor 130) → System has hardware watchdog support\n  • 'device-mapper' (minor 236) → LVM/dm-crypt support available\n  • 'kvm' → KVM virtualization support enabled in kernel"
        ),

        // dma
        ("dma", "channel_count") => (
            "Number of DMA channels in use.",
            "Count of ISA DMA channels currently claimed by device drivers. On modern systems, this is often 0 or very small.",
            "ISA DMA channel count.\n\nISA DMA is a legacy mechanism from the IBM PC era. Modern PCI/PCIe devices use bus-mastering DMA instead, which doesn't appear here.\n\n💡 Diagnostic:\n  • 0 channels → Normal on modern systems with no legacy ISA hardware\n  • Channel 4 = cascade (used internally by the DMA controller)\n  • Non-zero on modern systems → Floppy controller emulation or legacy sound card"
        ),
        ("dma", "channels") => (
            "DMA channel details.",
            "Table of active ISA DMA channels showing channel number and the device that claimed it.",
            "ISA DMA channel allocation table.\n\nColumns: channel_number, device_name.\n\n💡 Diagnostic:\n  • Channel 2 = floppy disk controller (legacy)\n  • Channel 4 = cascade (connects the two DMA controllers together)\n  • Modern DMA (PCI bus-mastering) is not shown here — it's managed by individual device drivers\n  • If empty, this is expected on any system without ISA hardware"
        ),

        // timer_list
        ("timer_list", "version") => (
            "Timer list version identifier.",
            "Version of the /proc/timer_list format. Used to ensure compatibility when parsing timer information.",
            "Timer list format version.\n\n💡 This is a metadata field. The important fields are timer_count and clock_count."
        ),
        ("timer_list", "now") => (
            "Current kernel time in nanoseconds.",
            "The kernel's current time (ktime) in nanoseconds, used as the reference point for all timer expirations.",
            "Current kernel time (ktime_get) in nanoseconds.\n\nThis is the monotonic clock — it never goes backward and isn't affected by NTP adjustments or wall-clock changes.\n\n💡 Diagnostic: Convert to human-readable: divide by 1,000,000,000 to get seconds since boot. Should roughly match uptime."
        ),
        ("timer_list", "clock_count") => (
            "Number of clock event devices.",
            "Count of clock event devices (hardware timers). Each CPU typically has its own local APIC timer.",
            "Clock event device count.\n\nClock event devices are hardware timers that generate periodic or one-shot interrupts for scheduling, timekeeping, and timer callbacks.\n\n💡 Diagnostic:\n  • Should roughly equal CPU count (each CPU has a local APIC timer)\n  • Additional clock devices: HPET, PIT, TSC deadline timer\n  • Missing clock devices → Kernel may have difficulty scheduling timers accurately"
        ),
        ("timer_list", "timer_count") => (
            "Number of pending kernel timers.",
            "Count of timers currently queued in the kernel. Includes timeouts, delayed work, and periodic callbacks.",
            "Pending kernel timer count.\n\nKernel timers are callbacks scheduled to fire at a future time. Used for TCP retransmission timeouts, device polling, watchdogs, and deferred work.\n\n💡 Diagnostic:\n  • Typical range: 50-500 on a busy server\n  • Very high count (10000+) → Some subsystem is creating many timers; could indicate a network issue (many TCP retransmit timers) or a misbehaving driver\n  • Timers that never fire → Possible timer leak, wasting kernel memory"
        ),

        // schedstat
        ("schedstat", "version") => (
            "Scheduler statistics version.",
            "Version number of the schedstat format. Determines which fields are available in per-CPU stats.",
            "Schedstat format version.\n\n💡 Version 15 is the current format. The version determines how to interpret the per-CPU stat columns."
        ),
        ("schedstat", "cpu_count") => (
            "CPUs with scheduler statistics.",
            "Number of CPUs reporting scheduler statistics. Should match the total CPU count.",
            "Number of CPUs in the scheduler stats.\n\n💡 Diagnostic: Should match the CPU count from cpuinfo and interrupts. If lower, some CPUs may be offline (`echo 1 > /sys/devices/system/cpu/cpuN/online` to re-enable)."
        ),
        ("schedstat", "cpu_stats") => (
            "Per-CPU scheduler statistics.",
            "Table of per-CPU scheduler metrics: yield count, schedule count, idle count, try-to-wake-up count, and more.",
            "Per-CPU scheduler statistics table.\n\nColumns: cpu, yld_count (yield), sched_count (context switches), sched_goidle (went idle), ttwu_count (try-to-wake-up), ...\n\n💡 Diagnostic:\n  • High sched_count → Many context switches on that CPU. Normal for I/O-heavy workloads.\n  • sched_goidle / sched_count → Idle ratio. High = CPU is underutilized. Low = CPU is always busy.\n  • ttwu_count → How often tasks are woken up. High = many sleeping tasks being activated (I/O completions, lock releases).\n  • Large disparity between CPUs → Workload is not well-balanced; check IRQ affinity and CPU pinning (taskset/cgroups)."
        ),

        // df (disk filesystem usage)
        ("df", "filesystems") => (
            "Filesystem usage table",
            "Table of mounted filesystems with columns: Device, MountPoint, Total, Used, Available, Use%. Pseudo-filesystems (tmpfs, proc, etc.) are excluded.",
            "Filesystem usage table showing real (non-pseudo) filesystems.\n\nColumns: Device, MountPoint, Total, Used, Available, Use%.\n\n💡 Diagnostic:\n  • Use% > 90% → Critical. Logs may fail to write, databases crash.\n  • Use% > 80% → Plan capacity. Set up log rotation, clean temp files.\n  • Available shows space usable by non-root users (accounts for reserved blocks)."
        ),
        ("df", "root_use_pct") => (
            "Root filesystem usage %",
            "Usage percentage of the root (/) filesystem. This is the most critical filesystem — if full, the system may become unresponsive.",
            "Root filesystem usage percentage.\n\n💡 Diagnostic:\n  • > 90% → CRITICAL: Immediate action needed. du -sh /* to find large dirs.\n  • > 80% → WARNING: Plan cleanup. journalctl --vacuum-size=500M, docker system prune.\n  • Steadily increasing → Possible log leak. Check /var/log sizes."
        ),

        // thermal
        ("thermal", "max_temp") => (
            "Highest CPU/GPU temperature",
            "Maximum temperature across all thermal zones. Above 75°C indicates thermal stress; above 90°C triggers throttling.",
            "Maximum temperature across all thermal zones.\n\n💡 Diagnostic:\n  • > 90°C → CRITICAL: Thermal throttling active. CPU frequency reduced. Check fans.\n  • > 75°C → WARNING: Running hot. Sustained load will push it higher.\n  • < 50°C → Normal idle temperature for most systems."
        ),

        // file-nr
        ("file-nr", "fd_allocated") => (
            "Allocated file descriptors",
            "Number of file handles currently allocated by the kernel. Includes both in-use and cached (unused) handles.",
            "Number of file handles currently allocated by the kernel.\n\nThis counts all file descriptors allocated, including unused ones kept in the free list for reuse.\n\n💡 Diagnostic: If fd_allocated is close to fd_max, the system may refuse new file/socket opens. Check for FD leaks with lsof."
        ),
        ("file-nr", "fd_usage_pct") => (
            "File descriptor usage %",
            "Percentage of file descriptors in active use relative to the system maximum. (allocated - unused) / max * 100.",
            "File descriptor usage percentage: (allocated - unused) / max * 100.\n\n💡 Diagnostic:\n  • > 80% → WARNING: FD exhaustion risk. Processes may fail to open files or sockets.\n  • Find leakers: lsof -p <PID> | wc -l for suspect processes.\n  • Raise limit: sysctl -w fs.file-max=<higher_value>."
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
            "利用可能な物理メモリの合計（カーネル予約分を除く）。\n\nMemAvailable が MemTotal の 10% を切ると、OOM Killer がプロセスを強制終了し始める可能性がある。64GB サーバーなら約 6.4GB を下回ると危険。\n\n💡 診断: MemTotal が物理メモリより少ない場合、BIOS のメモリ予約や `dmesg | grep Memory` を確認。"
        ),
        ("meminfo", "MemFree") => (
            "完全未使用のメモリ",
            "何にも使われていないメモリ。Linux はディスクキャッシュに空きメモリを使うので、健全なシステムでもこの値は低い。パニックせず MemAvailable を見ること。",
            "完全に未使用のメモリ — キャッシュにすら使われていない。\n\nよくある誤解:「サーバーのメモリが空いてない！」実は Linux は空きメモリを積極的にディスクキャッシュ（Cached + Buffers）として使う。これは良いこと — 使われないメモリは無駄なメモリ。\n\n💡 診断: MemFree も MemAvailable も低い → 本当のメモリ不足。MemFree だけ低くて MemAvailable は十分 → 健全な状態。"
        ),
        ("meminfo", "MemAvailable") => (
            "新しいプロセスに使えるメモリ",
            "スワップなしで確保できるメモリの推定値。回収可能なキャッシュとバッファを含む。MemFree ではなくこの値を監視すべき。",
            "スワップなしで確保できるメモリのカーネル推定値。\n\nメモリ圧迫度を測る最重要メトリクス。空きメモリ＋回収可能なページキャッシュ − 予約ウォーターマーク。\n\n💡 診断:\n  • MemTotal の 10% 未満 → OOM 危険ゾーン、即調査\n  • 20% 未満 → 警告、メモリリーク確認（RSS が増え続けるプロセス）\n  • 継続的に減少 → メモリリークの可能性大。`ps aux --sort=-rss | head` で犯人を探せ"
        ),
        ("meminfo", "Buffers") => (
            "ブロックデバイスバッファ用メモリ",
            "ブロックデバイスの生 I/O バッファ（ファイルシステムのメタデータ等）。通常は少量。メモリ圧迫時に回収される。",
            "ブロックデバイス I/O バッファ（ファイルシステムメタデータ、スーパーブロック等）。\n\nCached（ページキャッシュ）とは別。Buffers はディスクのメタデータ、Cached はファイルの中身。両方とも回収可能。\n\n💡 Buffers が異常に大きい場合、多数のブロックデバイスか、メタデータ集中操作（`find /` や巨大ディレクトリのリスト表示など）が原因。"
        ),
        ("meminfo", "Cached") => (
            "ファイルキャッシュ用メモリ",
            "ページキャッシュ — ディスクから読んだファイルを次回高速アクセスのためメモリに保持。メモリ圧迫時に回収される。値が高いのは正常で良いこと。",
            "ページキャッシュ — カーネルのファイル内容キャッシュ。\n\nディスクから読んだファイルは全てここにキャッシュされる。健全な Linux で「空きメモリが少ない」ように見えるのはこれのおかげ — カーネルが仕事をしている証拠。\n\n💡 診断: 負荷下で Cached がほぼゼロに落ちたら、スラッシング状態 — RAM 不足でキャッシュを維持できず、大量の I/O が発生している。"
        ),
        ("meminfo", "SwapTotal") => (
            "スワップ領域の合計",
            "利用可能なスワップ領域（ディスクベースの仮想メモリ）。物理 RAM が不足した時に使用される。",
            "設定済みスワップ領域の合計。\n\nスワップは RAM のオーバーフロー先 — 使われてないページがディスクに退避される。多少のスワップ使用は正常だが、激しいスワップ活動（vmstat の si/so を確認）は RAM 不足のサイン。\n\n💡 診断:\n  • SwapTotal = 0 → スワップ未設定。OOM Killer が唯一の安全装置。\n  • SSD 上のスワップ → 許容範囲。HDD 上のスワップ → 圧迫時に深刻な性能劣化。"
        ),
        ("meminfo", "SwapFree") => (
            "空きスワップ領域",
            "未使用のスワップ。時間とともに減少しているなら、メモリ圧迫でスワッピングが発生中。",
            "残りの未使用スワップ。\n\n💡 診断:\n  • SwapTotal - SwapFree > 0 で安定 → 以前スワップアウトされたページがある。正常。\n  • SwapFree が継続的に減少 → アクティブにスワッピング中、本当のメモリ圧迫。\n  • SwapFree = 0 → 次に RAM に収まらないメモリ確保で OOM Killer が発動。"
        ),

        // loadavg
        ("loadavg", "load1") => (
            "1分間の負荷平均",
            "過去1分間の実行可能・割り込み不可プロセスの平均数。CPU数と比較して飽和度を判断する。",
            "1分間の CPU 負荷平均。\n\n負荷平均がカウントするのは: (1) CPU 上で実行中、(2) CPU 待ち、(3) 割り込み不可 I/O 待ち（D 状態）のプロセス数。\n\n💡 診断:\n  • load1 < CPU数 → 余裕あり\n  • load1 ≈ CPU数 → フル稼働、許容範囲\n  • load1 > CPU数 × 2 → 深刻な過負荷。プロセスがキューに溜まっている\n  • load1 >> load15 → 今まさに負荷が急上昇中\n  • load1 << load15 → 直近のスパイクから回復中"
        ),
        ("loadavg", "load5") => (
            "5分間の負荷平均",
            "過去5分間の負荷平均。1分平均より安定したトレンド指標。",
            "5分間の負荷平均 — トレンド指標。\n\nload1 と load5 を比較して方向を見る:\n  • load1 > load5 → 負荷上昇中\n  • load1 < load5 → 負荷減少中\n  • load1 ≈ load5 → 安定"
        ),
        ("loadavg", "load15") => (
            "15分間の負荷平均",
            "過去15分間の負荷平均。ベースラインの負荷を把握するのに最適。",
            "15分間の負荷平均 — ベースライン。\n\nこのシステムの「普通」がわかる。load15 が常に 4.0 前後なら、load1 が 8.0 に跳ねたら注目すべきだが、4.5 なら日常的。\n\n💡 load15 がずっと高いなら、CPU 追加が必要か、永続的な I/O ボトルネック（PSI の pressure データを確認）がある。"
        ),

        // net/tcp
        ("net/tcp", "connections") => (
            "TCP 接続一覧",
            "全 TCP 接続のテーブル。主要な状態: ESTABLISHED（通信中）、TIME_WAIT（切断中）、SYN_SENT（接続試行中）。",
            "システム上の全 TCP 接続。\n\n💡 接続状態別の診断パターン:\n  • SYN_SENT 多数 → 接続先がダウン、FW でドロップ、または DNS が遅い\n  • TIME_WAIT 多数 → 短命な接続を大量処理中。HTTP サーバーでは正常だが、1万超はエフェメラルポート枯渇の恐れ\n  • CLOSE_WAIT 多数 → アプリがソケットを閉じてない。相手は切断済みなのに close() を呼んでない。典型的な FD リーク\n  • 同一 IP への ESTABLISHED 多数 → コネクションプールか永続接続。DB 接続では正常"
        ),

        // processes
        ("processes", "processes") => (
            "プロセス一覧",
            "全プロセス: PID、名前、状態、RSS メモリ、スレッド数、UID。状態: S=休眠、R=実行中、Z=ゾンビ、D=割り込み不可 I/O。",
            "システム上の全プロセス。\n\n💡 診断パターン:\n  • Z（ゾンビ）が蓄積 → 親プロセスが wait() を呼んでない。ゾンビ自体はリソースを消費しないが、バグのある親プロセスを示す\n  • D（ディスク休眠）が停滞 → 割り込み不可 I/O 待ち。NFS ハング、ディスク障害、カーネルドライバのバグが多い。SIGKILL でも殺せない\n  • 1プロセスの RSS が時間とともに増加 → メモリリーク\n  • スレッド数 1000 超 → スレッドリークか、高負荷時のスレッド・パー・コネクション設計\n  • UID 0 のプロセス → root 実行。予期しないものがあればセキュリティ懸念"
        ),

        // pressure
        ("pressure", "cpu_some_avg10") => (
            "CPU 圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクが CPU を待った時間の割合。25% 超は CPU 競合を示す。",
            "CPU PSI（Pressure Stall Information）— 10秒平均。\n\n'some' は少なくとも1つの実行可能タスクが CPU 時間を得られなかったことを意味する。負荷平均より正確 — 停滞時間を直接測定。\n\n💡 診断:\n  • 5% 未満 → 健全、CPU に余裕あり\n  • 5-25% → 中程度の圧力、一部タスクが待機\n  • 25% 超 → 顕著な CPU 競合、性能劣化中\n  • 50% 超 → 深刻 — 半分の時間、タスクが CPU を待っている"
        ),
        ("pressure", "memory_some_avg10") => (
            "メモリ圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクがメモリで停滞した時間の割合。非ゼロはメモリ圧迫またはスワッピングを示す。",
            "メモリ PSI — 10秒平均。\n\nCPU 圧力と違い、メモリ圧力は少しでもあれば注目すべき。メモリ回収やスワッピングが発生中ということ。\n\n💡 診断:\n  • 0% → メモリ圧力なし\n  • 0% 超 → キャッシュ回収かスワップ中。MemAvailable を確認\n  • 10% 超 → 顕著なメモリ圧力。メモリ回収でタスクが停滞\n  • 40% 超 → 危機的。スラッシング状態。性能が深刻に劣化"
        ),
        ("pressure", "io_some_avg10") => (
            "I/O 圧力: 一部タスク停滞 (10秒平均)",
            "過去10秒で少なくとも1つのタスクが I/O で停滞した時間の割合。ディスクボトルネックを示す。",
            "I/O PSI — 10秒平均。\n\nタスクがディスク/ストレージ I/O 待ちでブロックされる頻度を測定。\n\n💡 診断:\n  • 5% 未満 → 正常な I/O 活動\n  • 5-20% → I/O がボトルネックになりつつある。diskstats の await 時間を確認\n  • 20% 超 → 顕著な I/O ボトルネック。SSD アップグレード、I/O スケジューラ調整、RAM 追加（ページキャッシュ拡大）を検討\n  • 負荷スパイクと相関 → I/O バウンドなワークロード。負荷平均は D 状態を含むので、高負荷＋高 I/O 圧力 = CPU ではなく I/O の問題"
        ),

        // uptime
        ("uptime", "uptime") => (
            "システム稼働時間",
            "前回起動からのシステム稼働時間。サスペンド/休止状態の時間もカーネルによっては含まれる。",
            "起動からのシステム稼働時間。\n\nカーネル開始からの壁時計時間。最近のカーネル（3.x+）ではサスペンド中もリセットされないが、動作はカーネルにより異なる。\n\n💡 診断:\n  • 本番サーバーで稼働時間が極端に短い → 予期しない再起動。`dmesg` と `/var/log/kern.log` でパニック/oops を確認。\n  • 極端に長い稼働時間（数ヶ月/数年）→ セキュリティパッチ未適用の可能性。メンテナンスウィンドウを計画すべき。"
        ),
        ("uptime", "idle") => (
            "全 CPU のアイドル時間合計",
            "起動以来の全 CPU のアイドル時間の累計。マルチコアでは各コアの合計なので uptime を超えることがある。",
            "全 CPU コアのアイドル時間の合計。\n\n4コアシステムで idle = 3 * uptime なら、平均して4コア中3コアがアイドルだったことを意味する。\n\n💡 診断: マルチコアで idle < uptime は通常あり得ない — シングルコア VM か、クロックソースの問題の可能性。"
        ),
        ("uptime", "idle_pct") => (
            "アイドル率 (idle / uptime)",
            "アイドル時間と稼働時間の比率。マルチコアでは (idle / uptime * 100) なので、ほぼアイドルなマシンでは 100% を超える。",
            "アイドル率 = idle_time / uptime * 100。\n\nアイドル時間は全 CPU の合計なので、マルチコアでは 100% を超える。4コアのアイドルシステムなら約 400%。\n\n💡 診断:\n  • idle_pct / CPU数 < 20% → システムは高負荷\n  • idle_pct / CPU数 > 80% → システムはほぼアイドル\n  • 時間とともに低下 → ワークロードが増加中"
        ),

        // version
        ("version", "raw") => (
            "カーネルバージョン文字列（完全）",
            "/proc/version の完全な出力。カーネルバージョン、ビルドホスト、コンパイラ、ビルド日時を含む。",
            "カーネルが報告する完全なバージョン文字列。\n\nカーネルバージョン、ビルドした人/ホスト、使用コンパイラ、ビルド日時が含まれる。\n\n💡 アップデート後に正しいカーネルが動作しているか確認したり、サポートチケットに正確なビルド情報を記載する際に有用。"
        ),
        ("version", "kernel_version") => (
            "カーネルバージョン番号",
            "Linux カーネルのバージョン（例: 6.6.87）。利用可能な機能、システムコール対応、ドライバ互換性を決定する。",
            "Linux カーネルバージョン（例: 6.6.87）。\n\nバージョン形式: major.minor.patch。minor で機能セットが決まり、patch にはバグ/セキュリティ修正が含まれる。\n\n💡 診断:\n  • CVE データベースと照合し、重要な脆弱性がパッチ済みか確認。\n  • カーネル 5.x 未満は PSI（圧力ストール情報）、io_uring 等の最新機能が未対応の場合がある。\n  • `uname -r` と比較 — 一致するはず。"
        ),
        ("version", "compiler") => (
            "カーネルビルド用コンパイラ",
            "カーネルのコンパイルに使用されたコンパイラ（通常 GCC）とバージョン。ABI 互換性のデバッグに関連。",
            "このカーネルをビルドしたコンパイラ。\n\n通常は GCC だが、一部ディストリビューションは Clang/LLVM を使用。コンパイラバージョンが重要な理由:\n  • ツリー外カーネルモジュール（NVIDIA ドライバ等）との ABI 互換性\n  • コンパイラ固有の最適化によるパフォーマンスへの影響\n\n💡 カーネルモジュールが 'version magic' エラーでロード失敗する場合、コンパイラ不一致が一般的な原因。"
        ),

        // cmdline
        ("cmdline", "cmdline") => (
            "カーネル起動パラメータ",
            "ブートローダー（GRUB、systemd-boot 等）からカーネルに渡されたコマンドライン。ハードウェア設定、セキュリティ機能、デバッグオプションを制御。",
            "ブートローダーからのカーネル起動コマンドライン。\n\n一般的な重要パラメータ:\n  • root= — ルートファイルシステムデバイス\n  • quiet/splash — 起動メッセージの抑制\n  • nomodeset — カーネルモード設定の無効化（GPU トラブルシューティング）\n  • mitigations=off — CPU 脆弱性緩和の無効化（危険だが高速化）\n  • crashkernel= — kdump 用予約メモリ\n\n💡 診断: パフォーマンスが予想外に低い場合、'mitigations=off' の欠如やオーバーヘッドを生むデバッグパラメータを確認。"
        ),
        ("cmdline", "param_count") => (
            "起動パラメータの数",
            "スペース区切りのカーネル起動パラメータの数。起動設定の複雑さの概要把握に有用。",
            "カーネルコマンドラインのスペース区切りトークン数。\n\n💡 param_count が非常に多い場合、カスタムカーネルチューニングやハードウェア問題の回避策が含まれている可能性。最小構成のシステムでは通常 5-15 個。"
        ),

        // stat
        ("stat", "cpu_user") => (
            "ユーザーモード CPU 時間",
            "起動以来の全 CPU のユーザー空間コード実行時間（jiffies 単位）。アプリケーションコードを含むが、カーネルシステムコールは含まない。",
            "起動以来の累積ユーザーモード CPU 時間（jiffies = 通常 1/100 秒）。\n\nユーザー時間はアプリケーションコードの実行全てを含む。`top` の 'us' 列に相当。\n\n💡 診断:\n  • user% が高く system% が低い → アプリケーションが CPU バウンド（計算集中型）\n  • cpu_system と比較: user >> system は計算ワークロードでは正常\n  • 絶対値ではなくスナップショット間の変化率を監視すべき"
        ),
        ("stat", "cpu_system") => (
            "カーネルモード CPU 時間",
            "起動以来の全 CPU のカーネルコード実行時間。システムコール、割り込み、カーネルスレッドを含む。",
            "起動以来の累積カーネルモード CPU 時間。\n\nシステム時間 = システムコール、割り込みハンドラ、カーネルスレッドでの時間。`top` の 'sy' 列に相当。\n\n💡 診断:\n  • system% が高い → 大量のシステムコール（I/O、コンテキストスイッチ、ネットワーク処理が多い）\n  • system > user → 異常。大量の小さい read() 等、バッファリングされない I/O が原因の可能性\n  • system% の突然のスパイク → 割り込みストームやカーネルドライバの問題を確認"
        ),
        ("stat", "cpu_idle") => (
            "CPU アイドル時間",
            "起動以来の全 CPU のアイドル時間。高い値は CPU に余裕があることを意味する。",
            "起動以来の累積 CPU アイドル時間。\n\nCPU が何もしておらず、実行可能なタスクもなかった時間。\n\n💡 診断:\n  • アイドルが常にほぼ 0 → CPU が飽和状態\n  • スナップショット間のアイドル率を比較: (idle_delta / total_delta * 100) でリアルタイムのアイドル% がわかる\n  • マルチコアでは合計値なので、1秒あたり最大アイドル = コア数 × 100 jiffies"
        ),
        ("stat", "cpu_iowait") => (
            "I/O 待ち CPU 時間",
            "CPU が未処理の I/O を待ってアイドルだった時間。高い iowait はストレージのボトルネックを示す。",
            "I/O 完了待ちの CPU 時間。\n\niowait は CPU がやることがなく、かつ未完了の I/O があったことを意味する。アイドルのサブセット — CPU はアイドルだがディスクでブロックされている。\n\n💡 診断:\n  • 高い iowait → CPU ではなくストレージがボトルネック\n  • 忙しいシステムでは iowait が低く見える場合がある（I/O 中に他のタスクが CPU を使うため）\n  • より正確な I/O ボトルネック信号は pressure の io_some_avg10 と比較\n  • スパイク → 大きなシーケンシャル読み書きやファイルシステムジャーナルのフラッシュ"
        ),
        ("stat", "cpu_usage_pct") => (
            "総合 CPU 使用率（累積）",
            "起動以来の CPU が有用な作業に費やした時間の割合。(total - idle - iowait) / total * 100 で計算。",
            "起動以来の累積 CPU 使用率。\n\n計算式: (busy_time / total_time * 100)、busy = user + nice + system + irq + softirq + steal。\n\n💡 注意: これは起動以来の累積平均であり、リアルタイムの使用率ではない。23時間アイドル後に1時間 100% 稼働したシステムではここに約 4% と表示される。"
        ),
        ("stat", "forks_total") => (
            "起動以来のフォーク（プロセス生成）回数",
            "起動以来 fork()/clone() が呼ばれた回数。高いレートは多数の短命プロセスを示す。",
            "起動以来の fork()/clone() 呼び出し回数。\n\nプロセスやスレッドの生成ごとにこのカウンタが増加する。\n\n💡 診断:\n  • 高いフォークレート（スナップショット間のデルタ）→ シェルスクリプトが多数のサブプロセスを生成、cron ジョブ、またはフォーク爆弾\n  • context_switches と比較: 高フォーク + 高コンテキストスイッチ = 多数の短命プロセス\n  • 緩やかな増加は正常。突然のスパイクは調査が必要"
        ),
        ("stat", "procs_running") => (
            "現在 CPU 上で実行中のプロセス数",
            "R（実行中/実行可能）状態のプロセス数。CPU 数を常に超えている場合、CPU が過負荷。",
            "現在 R（実行中）状態のプロセス数。\n\nCPU 上でアクティブに実行中か、CPU 時間待ちのランキューにいるプロセス。\n\n💡 診断:\n  • procs_running <= CPU数 → 正常、CPU が全ランナーを処理可能\n  • procs_running > CPU数 → タスクが CPU 時間待ちでキューイング中\n  • 持続的に高い → CPU ボトルネック、負荷平均と相関"
        ),
        ("stat", "procs_blocked") => (
            "I/O でブロック中のプロセス数",
            "D（割り込み不可スリープ）状態のプロセス数。I/O 待ちで、シグナルでも中断できない。",
            "D 状態（割り込み不可スリープ）のプロセス数。ディスク、NFS、デバイス I/O 待ち。\n\n💡 診断:\n  • procs_blocked > 0 が一時的 → I/O 操作中は正常\n  • procs_blocked が持続的に高い → I/O ボトルネック。diskstats と pressure を確認。\n  • D 状態プロセスが数分間スタック → NFS ハング、ディスク故障、カーネルドライバのバグ。SIGKILL でも殺せない。"
        ),
        ("stat", "context_switches") => (
            "起動以来のコンテキストスイッチ回数",
            "起動以来の CPU コンテキストスイッチ数。各スイッチでプロセスの状態を保存/復元する。非常に高いレートは過度なマルチタスクのオーバーヘッドを示す。",
            "起動以来の CPU コンテキストスイッチ合計。\n\nコンテキストスイッチは CPU がプロセス/スレッドを切り替える時に発生。自発的（ブロッキング I/O）と非自発的（プリエンプション）の両方がカウントされる。\n\n💡 診断:\n  • 正常レート: ワークロードにより 1000-50000/秒\n  • 100000/秒超 → 高い。多数のスレッドが競合するか過度な I/O 操作\n  • 高い system% CPU と相関 — 各スイッチにカーネルオーバーヘッドがある\n  • スナップショット間のデルタで現在のレートを把握"
        ),

        // cpuinfo
        ("cpuinfo", "logical_cpus") => (
            "論理 CPU 数（スレッド数）",
            "OS から見える論理プロセッサの合計。ハイパースレッディングを含む — 2つの論理 CPU が1つの物理コアを共有する場合がある。",
            "OS から見える論理 CPU（ハードウェアスレッド）の合計。\n\nハイパースレッディング/SMT 有効時: logical_cpus = physical_cores * 2（通常）。HT 無効時: logical_cpus = physical_cores。\n\n💡 診断:\n  • cores_per_socket と比較してハイパースレッディングを検出\n  • 負荷平均と比較する数はこの値\n  • 期待より少ない場合、BIOS 設定やカーネルパラメータ (maxcpus=, nr_cpus=) を確認"
        ),
        ("cpuinfo", "model") => (
            "CPU モデル名",
            "プロセッサが報告する完全な CPU モデル識別子。ブランド、世代、バリアント情報を含む。",
            "プロセッサの CPUID 命令から取得した CPU モデル名。\n\n例: 'Intel(R) Core(TM) i9-13900K'、'AMD EPYC 9654'。\n\n💡 用途:\n  • ハードウェア世代と期待されるパフォーマンスの特定\n  • 必要な命令セット（AVX-512 等）のサポート確認\n  • VM が正しい CPU モデルを公開しているか検証"
        ),
        ("cpuinfo", "frequency") => (
            "現在の CPU 周波数 (MHz)",
            "CPU の現在の動作周波数。周波数スケーリング（ターボブースト、省電力）により変動する場合がある。",
            "現在の CPU 周波数（MHz）。\n\n最近の CPU は負荷に応じて動的に周波数を調整する（P-state）。報告される値は:\n  • 軽負荷時のベース周波数（省電力）\n  • 高負荷時のターボ/ブースト周波数\n  • サーマルスロットリングで制限された周波数\n\n💡 診断:\n  • 定格より大幅に低い → ガバナーを確認: `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`\n  • 'powersave' ガバナー → パフォーマンスが制限される。ベンチマークには 'performance' に切替。\n  • ターボに達しない → サーマルスロットリング。CPU 温度を確認。"
        ),
        ("cpuinfo", "cache_size") => (
            "CPU キャッシュサイズ (L2/L3)",
            "コアあたりの最終レベルキャッシュサイズ。大きいキャッシュはメモリ集中型ワークロードのパフォーマンスを向上させる。",
            "/proc/cpuinfo が報告する最終レベルキャッシュサイズ。\n\n通常はコア間で共有される L3 キャッシュ、またはアーキテクチャによってはコアごとの L2。\n\n💡 診断:\n  • 大きいキャッシュ = ワーキングセットが収まる場合のパフォーマンス向上\n  • データベースワークロードは大きな L3 キャッシュから大きな恩恵\n  • 詳細なキャッシュトポロジは `/sys/devices/system/cpu/cpu0/cache/` を確認"
        ),
        ("cpuinfo", "cores_per_socket") => (
            "ソケットあたりの物理コア数",
            "ソケットあたりの物理 CPU コア数。ハイパースレッディングでは各コアが 2 つの論理 CPU として表示される。",
            "ソケットあたりの物理 CPU コア。\n\n物理コアは独自の実行ユニットを持つ。ハイパースレッディングは実行リソースを共有して物理コアあたり 2 つの論理 CPU を作る。\n\n💡 診断:\n  • logical_cpus / cores_per_socket = 2 → ハイパースレッディング有効\n  • logical_cpus / cores_per_socket = 1 → ハイパースレッディング無効または非対応\n  • HPC/レイテンシ重視のワークロードでは、HT 無効化でパフォーマンスが向上する場合がある"
        ),
        ("cpuinfo", "key_flags") => (
            "主要 CPU 機能フラグ",
            "重要な CPU 機能フラグ: SSE/AVX（SIMD）、aes（暗号化）、vmx/svm（仮想化）、ht（ハイパースレッディング）、lm（64ビット）。",
            "/proc/cpuinfo からの主要 CPU 機能フラグ。\n\n注目すべきフラグ:\n  • sse/sse2/avx/avx2/avx512f → SIMD 命令セット（数値計算ワークロードに重要）\n  • aes → ハードウェア AES 暗号化（TLS パフォーマンスに重要）\n  • vmx (Intel) / svm (AMD) → ハードウェア仮想化サポート\n  • ht → ハイパースレッディング対応\n  • lm → ロングモード（64ビットサポート）\n  • nx → No-eXecute ビット（セキュリティ: データページの実行防止）\n  • hypervisor → VM 内で動作中\n\n💡 診断: 'hypervisor' フラグがある → これは VM。'vmx'/'svm' がない → ネストされた仮想化は利用不可。"
        ),

        // vmstat
        ("vmstat", "pgfault") => (
            "起動以来のページフォルト合計",
            "マイナー + メジャーページフォルトの合計。マイナーはメモリから解決、メジャーはディスク I/O が必要。",
            "起動以来のページフォルト合計（マイナー + メジャー）。\n\nマイナーフォルト: ページはメモリにあるがプロセスのページテーブルにマップされていない。即座に解決 — 正常で頻繁。\nメジャーフォルト: ページをディスクから読む必要がある。コストが高い。\n\n💡 診断: pgfault が高いのは正常。実際に I/O を発生させる pgmajfault に注目すべき。"
        ),
        ("vmstat", "pgmajfault") => (
            "メジャーページフォルト（ディスク I/O が必要）",
            "ディスクからの読み込みが必要だったページフォルト。各フォルトがプロセスを停滞させる。高いレートは RAM 不足やコールドキャッシュを示す。",
            "起動以来のメジャーページフォルト — 各フォルトでディスク I/O が必要。\n\nメジャーフォルトは要求されたページが RAM になく、ストレージから取得する必要があったことを意味する。フォルトしたプロセスは停滞する。\n\n💡 診断:\n  • pgmajfault レート（デルタ/秒）が高い → ワーキングセットに対して RAM 不足、またはコールドスタート\n  • 高い iowait と I/O 圧力と相関\n  • 起動直後はアプリケーション読み込みでメジャーフォルトが急増 — これは正常\n  • 持続的に高いレート → RAM 追加またはワーキングセットサイズの削減"
        ),
        ("vmstat", "pgpgin") => (
            "ディスクからページイン",
            "起動以来にブロックデバイスからメモリに読み込まれたページ数。通常のファイル I/O とデマンドページングを含む。",
            "ディスクからページインされたページ数（1KB 単位）。\n\nファイル I/O（read() システムコールによるページキャッシュ充填）とデマンドページング（実行可能ページの読み込み）の両方を含む。\n\n💡 pgpgin レートと pgpgout レートを比較: pgpgin >> pgpgout なら読み込み集中型ワークロード。"
        ),
        ("vmstat", "pgpgout") => (
            "ディスクへページアウト",
            "起動以来にメモリからブロックデバイスに書き込まれたページ数。ダーティページのライトバックとスワップアウトを含む。",
            "ディスクへページアウトされたページ数（1KB 単位）。\n\nダーティページキャッシュのライトバック（正常）とスワップのページアウト（メモリ圧迫）を含む。\n\n💡 診断: 通常のライトバックとスワップ圧力を区別するには、pswpout を別途確認。pgpgout が高く pswpout が低い = 通常のファイル書き込み。"
        ),
        ("vmstat", "pswpin") => (
            "スワップからページイン",
            "スワップからメモリに読み戻されたページ数。非ゼロは以前スワップアウトしたページを取り戻していることを意味する。",
            "スワップ領域からページインされたページ数。\n\nスワップインは、メモリ圧迫で以前スワップアウトされたページにプロセスがアクセスした時に発生。\n\n💡 診断:\n  • pswpin レート > 0 → スワップから積極的に読み込み中。パフォーマンスへの影響はスワップデバイスの速度に依存。\n  • 高 pswpin + 高 pswpout → スラッシング — ページが常にスワップイン/アウトされている。危機的状態。\n  • pswpin はあるが pswpout がない → 過去のメモリ圧迫からページを回復中。一時的かもしれない。"
        ),
        ("vmstat", "pswpout") => (
            "スワップへページアウト",
            "RAM からスワップに移されたページ数。アクティブなスワップアウトは現在のメモリ圧迫を示す。",
            "スワップ領域へページアウトされたページ数。\n\nスワップアウトはカーネルが RAM を確保する必要があり、回収可能なキャッシュを使い果たした時に発生。\n\n💡 診断:\n  • pswpout レート > 0 → 今まさにメモリ圧迫中\n  • 持続的に高い pswpout → システムにもっと RAM が必要\n  • 断続的な pswpout → 一時的なメモリスパイク、許容範囲の場合もある"
        ),
        ("vmstat", "nr_free_pages") => (
            "空きメモリページ数",
            "完全に空きのページ数。低い値は正常 — Linux は空きメモリをキャッシュに使う。",
            "未使用（空き）メモリページ数。\n\nmeminfo の MemFree と同様。Linux は空きメモリをページキャッシュに使うので、低い値は期待通り。\n\n💡 診断:\n  • 空きページが 'min' ウォーターマーク以下 → カーネルがダイレクトリクレームに入り、メモリ確保が停滞する可能性\n  • ゾーンごとの空き vs ウォーターマーク比較は zoneinfo を確認"
        ),
        ("vmstat", "nr_active_anon") => (
            "アクティブな匿名ページ数",
            "アクティブ LRU リスト上の匿名（ファイル非連動）ページ。最近アクセスされたヒープ/スタックページ。",
            "アクティブ LRU（Least Recently Used）リスト上の匿名ページ。\n\n匿名ページ = プロセスのヒープ、スタック、mmap(MAP_ANONYMOUS)。'アクティブ' は最近アクセスされたことを意味する。\n\n💡 これらのページはスワッピングでのみ解放できる。高い active_anon = プロセスが多くのヒープメモリを使用中。"
        ),
        ("vmstat", "nr_inactive_anon") => (
            "非アクティブな匿名ページ数",
            "最近アクセスされていない匿名ページ。メモリ圧迫時にスワップアウトの候補になる。",
            "非アクティブ LRU リスト上の匿名ページ — 最近アクセスされていない。\n\nメモリ圧迫が発生した時に最初にスワップアウトされる候補。\n\n💡 診断: 大きな inactive_anon でスワップ活動なし → 必要に応じて回収可能なメモリ。大きな inactive_anon でスワップが活発 → これらのページがスワップアウトされている。"
        ),
        ("vmstat", "nr_active_file") => (
            "アクティブなファイルページ数",
            "最近アクセスされたファイル連動ページ（ページキャッシュのホットページ）。再読み込みを高速化するファイル内容をキャッシュ。",
            "アクティブ LRU 上のファイル連動ページ — 最近使用されたページキャッシュ。\n\n最近読み取られたファイル内容をキャッシュし、ディスク読み取りを回避して I/O パフォーマンスを向上させる。\n\n💡 健全なシステムは大きなアクティブファイルキャッシュを持つ。負荷下で active_file が縮小するのは、メモリ圧迫でキャッシュが追い出されていることを意味する。"
        ),
        ("vmstat", "nr_inactive_file") => (
            "非アクティブなファイルページ数",
            "最近アクセスされていないファイル連動ページ。I/O なしで（ダーティでなければ）素早く回収可能。",
            "非アクティブ LRU 上のファイル連動ページ — 最近アクセスされていない。\n\nメモリ圧迫時に最初に回収される。回収コストは低い（ダーティでなければページを捨てるだけ）。\n\n💡 診断: 大きな inactive_file = メモリ圧迫に対する良いバッファ。小さな inactive_file = スワッピング開始前に回収できる余地が少ない。"
        ),
        ("vmstat", "nr_dirty") => (
            "ダーティページ（変更済み、未書き込み）",
            "メモリ上で変更されたがまだディスクに書き込まれていないページ。カーネルのライトバック機構で書き込まれる。",
            "まだディスクにフラッシュされていない変更済みページ。\n\nカーネルは定期的にダーティページをフラッシュする（/proc/sys/vm/dirty_writeback_centisecs で制御）。システムがクラッシュすると、ダーティページは失われる。\n\n💡 診断:\n  • 高い nr_dirty → 書き込み集中ワークロードまたは低速ストレージ\n  • nr_dirty が dirty_ratio を超える → write() がページフラッシュまでブロックされる（書き込みスロットリング）\n  • 持続的に高い → ストレージが書き込みレートに追いつけない"
        ),
        ("vmstat", "nr_writeback") => (
            "ディスクに書き込み中のページ",
            "現在ストレージにフラッシュ中のページ。高い値は激しい I/O 活動を示す。",
            "現在ストレージにライトバック中のページ。\n\nこれらのページはディスクへ転送中。数値はストレージ速度と書き込み量に依存。\n\n💡 診断:\n  • nr_writeback > 0 がほとんどの時間 → 常時書き込み圧力\n  • 非常に高い nr_writeback → ストレージデバイスが飽和、書き込みが滞留\n  • デバイスレベルの I/O メトリクスは diskstats を確認"
        ),
        ("vmstat", "nr_slab_reclaimable") => (
            "回収可能なスラブページ",
            "メモリ圧迫時に解放可能なカーネルスラブアロケータのページ。dentry キャッシュと inode キャッシュを含む。",
            "回収可能なスラブメモリ（カーネルキャッシュ）。\n\n主に dentry キャッシュ（ディレクトリエントリ）と inode キャッシュ。ファイルシステム操作を高速化し、メモリが必要な時に解放される。\n\n💡 診断: ファイルサーバーで回収可能スラブが非常に大きいのは正常 — 数百万のディレクトリエントリをキャッシュしている。カーネルは圧迫時に自動的に縮小する。"
        ),
        ("vmstat", "nr_slab_unreclaimable") => (
            "回収不可能なスラブページ",
            "解放できないカーネルスラブページ。メモリに保持し続ける必要があるアクティブなカーネルデータ構造。",
            "回収不可能なスラブメモリ — アクティブなカーネルオブジェクト。\n\nアクティブに使用中のカーネルデータ構造（タスク構造体、ネットワークバッファ等）で、解放できない。\n\n💡 診断:\n  • 回収不可能スラブが着実に増加 → カーネルメモリリークの可能性\n  • slabinfo でどの特定キャッシュが増加しているか確認\n  • 多数のネットワーク接続で回収不可能スラブが大きい → ネットワークバッファメモリ"
        ),
        ("vmstat", "oom_kill") => (
            "OOM Killer 発動回数",
            "起動以来 OOM（Out of Memory）Killer が発動した回数。各発動でメモリ解放のためプロセスが強制終了される。",
            "起動以来の OOM（Out of Memory）Killer 発動回数。\n\nOOM Killer はカーネルの最終手段 — 全メモリ（RAM + スワップ）が枯渇した時、生存のためにプロセスを強制終了する。\n\n💡 診断:\n  • 非ゼロの値は全て調査に値する\n  • `dmesg | grep -i oom` でどのプロセスが殺されたか、理由の詳細を確認\n  • OOM 防止: スワップ追加、RAM 増設、cgroups でメモリ制限を設定\n  • 重要プロセスの保護: `echo -1000 > /proc/<pid>/oom_score_adj`"
        ),

        // buddyinfo
        ("buddyinfo", "zone_count") => (
            "メモリゾーン数",
            "バディアロケータのメモリゾーン数。一般的なゾーン: DMA、DMA32、Normal、オプションで Movable。",
            "バディアロケータが追跡するメモリゾーン数。\n\n典型的なゾーン:\n  • DMA — 最初の 16MB、レガシー ISA デバイス用\n  • DMA32 — 最初の 4GB、32ビット DMA デバイス用\n  • Normal — メインメモリゾーン\n  • Movable — 移行可能なページ（メモリホットプラグ用）\n\n💡 NUMA システムでは各ノードにゾーンセットがあるため、ゾーンが増える。"
        ),
        ("buddyinfo", "zones") => (
            "ゾーン・オーダー別の空きページチャンク",
            "メモリ断片化データ: 各ゾーンで各オーダー（0-10）の空きチャンク数を表示。オーダー N = 2^N 連続ページ（各 4KB）。",
            "バディアロケータのゾーン・オーダー別空きページ数。\n\nオーダー 0 = 4KB、オーダー 1 = 8KB、... オーダー 10 = 4MB。高いオーダーほど大きな連続空きブロック。\n\n💡 診断:\n  • オーダー 0 のページは多いが高次オーダーがゼロ → メモリが断片化。大きなメモリ確保が失敗するかコンパクションが必要。\n  • 全オーダーでゼロ → ゾーンが枯渇\n  • ヒュージページに重要: 2MB ヒュージページにはオーダー 9 が必要（x86）。オーダー 9 が 0 なら透過的ヒュージページは失敗する。\n  • `echo 1 > /proc/sys/vm/compact_memory` でコンパクションを実行可能。"
        ),

        // zoneinfo
        ("zoneinfo", "zone_count") => (
            "メモリゾーン数",
            "詳細なウォーターマーク情報を持つメモリゾーン数。各ゾーンに回収動作を制御する min/low/high ウォーターマークがある。",
            "zoneinfo のメモリゾーン数。\n\n各ゾーンにはページ回収を制御するウォーターマークがある:\n  • min — これ以下ではメモリ確保が停滞（ダイレクトリクレーム）\n  • low — kswapd がバックグラウンド回収を開始\n  • high — kswapd が回収を停止\n\n💡 複数ノードの NUMA システムではゾーンが増える。"
        ),
        ("zoneinfo", "zones") => (
            "ゾーン別メモリ詳細（free, min, low, high）",
            "メモリゾーンのテーブル。空きページ数とウォーターマーク閾値を表示。free が 'low' を下回るとバックグラウンド回収が開始。",
            "ゾーン別メモリ詳細情報。\n\n列: ゾーン識別子、空きページ、min ウォーターマーク、low ウォーターマーク、high ウォーターマーク。\n\n💡 診断:\n  • free < min → ダイレクトリクレームが活発。メモリ確保が停滞 — プロセスがブロックされる可能性。\n  • free < low → kswapd 実行中（バックグラウンド回収）。中程度の圧迫下では正常。\n  • free > high → このゾーンにメモリ圧迫なし。\n  • 各ゾーンを個別に確認 — メモリ圧迫はゾーン固有の場合がある（例: Normal は余裕だが DMA32 が枯渇）。"
        ),

        // slabinfo
        ("slabinfo", "cache_count") => (
            "スラブキャッシュ数",
            "カーネルスラブアロケータのキャッシュ総数。各キャッシュは特定の種類のカーネルオブジェクト（inode、dentry、バッファ等）を提供。",
            "カーネルのアクティブなスラブキャッシュ数。\n\nスラブアロケータは固定サイズのカーネルオブジェクトの効率的な割り当てを提供する。各キャッシュは同じ型のオブジェクトをプール。\n\n💡 診断: キャッシュ数が非常に多い場合、多数のカーネルモジュールがロードされ、それぞれ独自のキャッシュを登録している可能性。"
        ),
        ("slabinfo", "caches") => (
            "スラブキャッシュ詳細",
            "スラブキャッシュのテーブル: 名前、アクティブオブジェクト数、総オブジェクト数、オブジェクトサイズ、スラブあたりオブジェクト数、スラブあたりページ数。",
            "スラブキャッシュの詳細統計。\n\n列: キャッシュ名、アクティブオブジェクト数、総割り当てオブジェクト数、オブジェクトサイズ（バイト）、スラブページあたりオブジェクト数、スラブあたりページ数。\n\n💡 診断:\n  • dentry キャッシュが非常に大きい → 多数のファイルシステムパスをキャッシュ中。ファイルサーバーでは正常。\n  • inode_cache が増加 → 多数のユニークファイルにアクセス中。\n  • active_objs << num_objs → メモリの無駄。多数の事前割り当てだが未使用のオブジェクト。\n  • 不明なキャッシュが増加 → カーネルモジュールのメモリリークの可能性。\n  • スラブキャッシュの回収: `echo 2 > /proc/sys/vm/drop_caches`（dentry+inode のみ）。"
        ),

        // pagetypeinfo
        ("pagetypeinfo", "entry_count") => (
            "ページタイプ情報エントリ数",
            "ページタイプ内訳のエントリ数。各エントリはゾーンごとの移行タイプ別空きページ数を表示。",
            "ページタイプ情報のエントリ数。\n\nエントリは NUMA ノード、メモリゾーン、移行タイプ（Unmovable、Movable、Reclaimable 等）別に分類される。\n\n💡 移行タイプはデフラグに影響: Movable ページは連続ブロックを作るために再配置可能。"
        ),
        ("pagetypeinfo", "entries") => (
            "ゾーン別ページ割り当てタイプ内訳",
            "移行タイプ（Unmovable、Movable、Reclaimable）別、ゾーン別、オーダー別の空きページ数。コンパクション用のページ分類を表示。",
            "詳細なページ割り当てタイプ情報。\n\n各オーダーの空きページが移行タイプ別にどう分布しているかを表示:\n  • Unmovable — 再配置不可（カーネルの割り当て）\n  • Movable — コンパクション用に移行可能（ユーザーページ）\n  • Reclaimable — 解放可能（ページキャッシュ、スラブ）\n\n💡 診断:\n  • Unmovable の断片が多い → コンパクション困難。断片化が持続する。\n  • Movable ページが支配的 → 透過的ヒュージページとコンパクションに良い。\n  • buddyinfo と合わせて断片化の原因を理解。"
        ),

        // swaps
        ("swaps", "total_size") => (
            "利用可能なスワップ領域合計",
            "全スワップ領域（ファイルとパーティション）の合計サイズ。RAM のオーバーフロー先の最大容量。",
            "全スワップ領域の合計スワップ容量。\n\nスワップは物理 RAM を超えて仮想メモリを拡張する。最近アクセスされていないページがスワップに移動され、RAM を解放する。\n\n💡 診断:\n  • 0 バイト → スワップ未設定。RAM が満杯になったら OOM しかない。\n  • 推奨: 大容量 RAM のシステムでも安全策として最低 1-2GB のスワップ。"
        ),
        ("swaps", "total_used") => (
            "使用中のスワップ領域",
            "現在使用中のスワップ量。多少の使用は正常。絶対値よりトレンドを監視すべき。",
            "現在使用中のスワップ容量。\n\n多少のスワップ使用は正常で問題を示さない — カーネルがアイドルページを先行してスワップアウトした可能性がある。\n\n💡 診断:\n  • 使用スワップが安定 → 以前スワップアウトされたページがそのまま。正常。\n  • 使用スワップが増加 → アクティブなメモリ圧迫。MemAvailable を確認。\n  • 使用スワップが合計に近い → 危険ゾーン。次のメモリ確保で OOM が発動する可能性。\n  • どのプロセスがスワップを使用しているか: `grep VmSwap /proc/*/status | sort -k2 -n`"
        ),
        ("swaps", "usage_pct") => (
            "スワップ使用率",
            "スワップ合計に対する使用中の割合。50% 以下なら通常問題なし。80% 超は調査が必要。",
            "スワップ合計に対する使用率。\n\n💡 診断:\n  • 0% → スワップ未使用（またはスワップ未設定）\n  • 50% 未満 → 正常、特に安定していれば\n  • 50-80% → 高め。トレンドを監視。\n  • 80% 超 → 高い。メモリ需要が増えると OOM の可能性。\n  • 100% → スワップ満杯。新たなメモリ需要は OOM Killer を発動させる。"
        ),
        ("swaps", "swap_areas") => (
            "個別スワップ領域の詳細",
            "スワップ領域のテーブル: ファイル名/デバイス、タイプ（パーティション/ファイル）、サイズ、使用量、優先度。高い優先度のものから先に使用される。",
            "個別スワップ領域の詳細。\n\n列: デバイス/ファイルパス、タイプ（パーティションまたはファイル）、サイズ、使用量、優先度。\n\n💡 診断:\n  • 優先度が使用順序を決定 — 高い優先度のものから先に使用される\n  • 複数のスワップ領域が同じ優先度 → ラウンドロビン（ストライプ）で分散、パフォーマンス向上\n  • SSD 上のスワップ → 許容範囲のパフォーマンス。HDD 上のスワップ → 圧迫時に大きなレイテンシ。\n  • スワップファイル vs パーティション → ファイルはやや遅いがリサイズが容易"
        ),

        // df (ディスク使用状況)
        ("df", "filesystems") => (
            "ファイルシステム使用状況テーブル",
            "マウント済みファイルシステムのテーブル: デバイス、マウントポイント、合計、使用量、空き、使用率%。疑似ファイルシステム (tmpfs, proc 等) は除外。",
            "実ファイルシステムの使用状況テーブル。\n\n列: デバイス、マウントポイント、合計、使用量、空き、使用率%。\n\n💡 診断:\n  • 使用率 > 90% → 危険。ログ書き込み失敗、DB クラッシュの可能性。\n  • 使用率 > 80% → 容量計画を。ログローテーション、不要ファイル削除。\n  • 「空き」は非 root ユーザーが使える容量（予約ブロックを考慮）。"
        ),
        ("df", "root_use_pct") => (
            "ルートファイルシステム使用率 %",
            "ルート (/) ファイルシステムの使用率。最も重要なファイルシステム — 満杯になるとシステムが応答不能になる可能性。",
            "ルートファイルシステムの使用率。\n\n💡 診断:\n  • > 90% → 危険: 即座に対処。du -sh /* で大きなディレクトリを特定。\n  • > 80% → 警告: 整理を計画。journalctl --vacuum-size=500M, docker system prune。\n  • 増加傾向 → ログリークの可能性。/var/log のサイズを確認。"
        ),

        // thermal (温度)
        ("thermal", "max_temp") => (
            "最高CPU/GPU温度",
            "全サーマルゾーンの最高温度。75°C以上は熱ストレス、90°C以上はスロットリング発動。",
            "全サーマルゾーンの最高温度。\n\n💡 診断:\n  • > 90°C → 危険: サーマルスロットリング発動。CPU周波数低下。ファンを確認。\n  • > 75°C → 警告: 高温状態。負荷が続くとさらに上昇。\n  • < 50°C → 多くのシステムで正常なアイドル温度。"
        ),

        // file-nr (ファイルディスクリプタ)
        ("file-nr", "fd_allocated") => (
            "割り当て済みファイルディスクリプタ数",
            "カーネルが現在割り当てているファイルハンドル数。使用中のものと未使用（キャッシュ）のものを含む。",
            "カーネルが現在割り当てているファイルハンドル数。\n\n未使用のものはフリーリストに保持され再利用される。\n\n💡 診断: fd_allocated が fd_max に近い場合、新しいファイルやソケットが開けなくなる。lsof で FD リークを確認。"
        ),
        ("file-nr", "fd_usage_pct") => (
            "ファイルディスクリプタ使用率 %",
            "システム最大値に対するFD使用率。(allocated - unused) / max * 100。",
            "ファイルディスクリプタ使用率: (allocated - unused) / max * 100。\n\n💡 診断:\n  • > 80% → 警告: FD枯渇の危険。ファイルやソケットが開けなくなる可能性。\n  • リーク元の特定: lsof -p <PID> | wc -l で疑わしいプロセスを調査。\n  • 上限引き上げ: sysctl -w fs.file-max=<大きい値>。"
        ),


        // ── グループ 3: ネットワーク ──────────────────────────────────

        // net/dev
        ("net/dev", "total_rx") => (
            "全インターフェースの受信バイト合計。",
            "起動以降の全ネットワークインターフェースの累積受信バイト数。ループバックトラフィックを含む。",
            "全インターフェースの受信バイト合計。\n\nこのカウンタは再起動でリセットされる。2つのスナップショットを比較してスループットを算出。ループバック（lo）を含むため、外部通信のみ必要な場合は lo を差し引く。\n\n💡 診断: total_rx が total_tx より大幅に速く増加 → ホストはコンシューマ（ダウンロード、DB 読み取り）。逆パターンはサーバー役割（Web サーバー、NFS エクスポート）を示唆。"
        ),
        ("net/dev", "total_tx") => (
            "全インターフェースの送信バイト合計。",
            "起動以降の全ネットワークインターフェースの累積送信バイト数。ループバックトラフィックを含む。",
            "全インターフェースの送信バイト合計。\n\ntotal_rx と同様だが送信側。TX が多く RX が少ない場合、コンテンツ配信ワークロードを示す。\n\n💡 診断: アプリケーション変更なしに TX が急増 → 侵害されたホストがデータを流出、または DDoS 増幅攻撃に参加している可能性。"
        ),
        ("net/dev", "interface_count") => (
            "ネットワークインターフェース数。",
            "/proc/net/dev に見える全ネットワークインターフェース数。ループバック、仮想、物理インターフェースを含む。",
            "ネットワークインターフェースの総数。\n\nlo（ループバック）、物理 NIC、ブリッジ、veth ペア（コンテナ）、tun/tap デバイス（VPN）、ボンドインターフェースを含む。\n\n💡 診断: 予想外に多い場合、コンテナの増殖を確認（各コンテナが veth ペアを追加）。予想外に少ない場合、NIC ドライバのロード失敗 — `dmesg | grep -i eth` を確認。"
        ),
        ("net/dev", "interfaces") => (
            "インターフェース別トラフィック統計。",
            "全ネットワークインターフェースのテーブル。名前、受信バイト数、受信パケット数、送信バイト数、送信パケット数を表示。",
            "インターフェース別ネットワークトラフィック内訳。\n\nカラム: 名前、RX バイト、RX パケット、TX バイト、TX パケット。\n\n💡 診断:\n  • RX/TX エラーが多い → ケーブル問題、デュプレックス不一致、またはドライバのバグ\n  • トラフィックゼロのインターフェース → リンクダウンの可能性; `ip link show` で確認\n  • lo のトラフィックが多い → localhost 上のプロセス間通信が活発（DB では一般的）\n  • パケット数多いがバイト数少ない → 小さなパケットが大量、SYN フラッドやおしゃべりなプロトコルの可能性"
        ),
        ("net/udp", "socket_count") => ("UDP ソケット数。", "システム上の全 UDP ソケット数。TCP と異なり UDP はコネクションレスで、各エントリはバインド済みソケットを表す。", "UDP ソケットの総数。\n\nUDP ソケットには TCP のような接続状態がない。各エントリはローカルポートにバインドされたソケット。\n\n💡 診断:\n  • 多数のローカルポートで高カウント → DNS 増幅や UDP スキャンの可能性\n  • 一般的な正当な UDP 利用: DNS (ポート 53)、NTP (123)、SNMP (161)、syslog (514)\n  • socket_count が増加し続ける → UDP アプリケーションの FD リークの可能性"),
        ("net/udp", "sockets") => ("UDP ソケット一覧。", "全 UDP ソケットのテーブル。ローカルアドレス、リモートアドレス、状態、所有プロセスの UID を表示。", "システム上の全 UDP ソケット。\n\nカラム: local_addr、remote_addr、state、uid。\n\n💡 診断:\n  • リモートアドレス 0.0.0.0:0 → リスニング中（特定のピアに接続していない）\n  • 同じポートに多数のソケット → 複数プロセスまたは SO_REUSEPORT\n  • UID 0 のソケット → root で実行中; 想定通りか確認\n  • UDP には信頼性保証がない — パケットロスはこのレベルでは見えない。net/snmp の Udp_InErrors でドロップ検出。"),
        ("net/unix", "socket_count") => ("Unix ドメインソケット数。", "全 Unix ドメインソケットの数。同一ホスト上の高速プロセス間通信に使用。", "Unix ドメインソケットの総数。\n\nUnix ソケットはローカル通信の優先 IPC メカニズム（TCP ループバックより高速）。DB、ディスプレイサーバー、systemd が多用。\n\n💡 診断:\n  • 高カウントは systemd ベースの現代システムでは正常（200+ は一般的）\n  • 継続的に増加 → ソケットリークの可能性\n  • パスが空のソケット → 抽象名前空間ソケット（@ プレフィックス付き）"),
        ("net/unix", "sockets") => ("Unix ドメインソケット詳細。", "全 Unix ドメインソケットのテーブル。参照カウント、タイプ、状態、inode、パスを表示。", "全 Unix ドメインソケット。\n\nカラム: refcount、type、state、inode、path。\n\n💡 診断:\n  • Type 1 = STREAM（TCP 的）、Type 2 = DGRAM（UDP 的）、Type 5 = SEQPACKET\n  • 既知のパスのソケット: /var/run/dbus/system_bus_socket（D-Bus）、/var/run/docker.sock（Docker）\n  • 1つのソケットに高い refcount → 多数のプロセスが共有（D-Bus では正常）"),
        ("net/arp", "entry_count") => ("ARP テーブルエントリ数。", "カーネル ARP キャッシュの IP-MAC アドレスマッピング数。各エントリはローカルネットワーク上の最近通信した隣接ホストを表す。", "ARP キャッシュエントリ数。\n\nARP テーブルは同一 L2 ネットワーク上のホストの IPv4 アドレスを MAC アドレスにマッピング。\n\n💡 診断:\n  • 非常に多い（1000+）→ 大規模フラットネットワーク、または ARP ポイズニング/スキャン\n  • 増加し続ける → ARP ストームまたはネットワークスキャンの可能性"),
        ("net/arp", "entries") => ("ARP テーブルエントリ。", "ARP キャッシュエントリのテーブル。IP アドレス、MAC アドレス、フラグ、ネットワークデバイスを表示。", "ARP キャッシュの内容。\n\nカラム: ip、hw_addr、flags、device。\n\n💡 診断:\n  • Flags 0x2 = 完了（解決済み）、0x6 = 完了+永続（静的エントリ）\n  • 異なる IP に同じ MAC → ARP スプーフィングまたはネットワーク設定ミス\n  • 00:00:00:00:00:00 MAC → 未解決エントリ; L2 でホストに到達不可"),
        ("net/route", "route_count") => ("ルーティングテーブルエントリ数。", "カーネル IPv4 ルーティングテーブルのエントリ数。デフォルトゲートウェイ、直接接続ネットワーク、静的ルートを含む。", "IPv4 ルーティングテーブルエントリ数。\n\n💡 診断:\n  • 0 ルート → ネットワーク未設定; ホストが孤立\n  • デフォルトルート（0.0.0.0 宛先）がない → 非ローカルネットワークに到達不可\n  • 非常に多い → 複雑なルーティング設定、またはダイナミックルーティングプロトコルがルートを注入"),
        ("net/route", "routes") => ("カーネルルーティングテーブル。", "ルーティングエントリのテーブル。インターフェース、宛先、ゲートウェイ、マスク、フラグ、メトリックを表示。", "IPv4 カーネルルーティングテーブル。\n\nカラム: iface、destination、gateway、mask、flags、metric。\n\n💡 診断:\n  • 宛先 0.0.0.0 でマスク 0.0.0.0 → デフォルトルート\n  • ゲートウェイ 0.0.0.0 → 直接接続ネットワーク\n  • フラグ: U=アップ、G=ゲートウェイ、H=ホストルート"),
        ("net/sockstat", "sockets_used") => ("使用中のソケット総数。", "カーネルが割り当てた全種類のソケット数。ネットワーク活動の全体的な指標。", "全プロトコルの割り当て済みソケット総数。\n\n💡 診断:\n  • 継続的に増加 → ソケット/FD リークの可能性\n  • システム全体の制限: /proc/sys/fs/file-max"),
        ("net/sockstat", "TCP_inuse") => ("使用中の TCP ソケット数。", "現在使用中の TCP ソケット数（TIME_WAIT 以外の全状態）。", "現在使用中の TCP ソケット。\n\n💡 診断:\n  • TCP_tw と比較 — tw >> inuse なら短命な接続が多い\n  • 急増はトラフィックスパイクまたは接続リーク"),
        ("net/sockstat", "TCP_orphan") => ("孤立 TCP ソケット数。", "どのプロセスにも属さない TCP ソケット。カーネルメモリを消費しクリーンアップ待ち。", "孤立 TCP 接続。\n\n💡 診断:\n  • 高い孤立数 → アプリクラッシュまたは不正終了\n  • 制限: /proc/sys/net/ipv4/tcp_max_orphans"),
        ("net/sockstat", "TCP_tw") => ("TIME_WAIT の TCP ソケット数。", "TIME_WAIT 状態の TCP 接続。遅延パケット待機中。忙しい HTTP サーバーでは正常。", "TCP TIME_WAIT ソケット数。\n\n💡 診断:\n  • 5000 未満 → 正常\n  • 30000 超 → エフェメラルポート枯渇の可能性\n  • tcp_tw_reuse を有効化して再利用を許可"),
        ("net/sockstat", "TCP_alloc") => ("割り当て済み TCP ソケット数。", "カーネルが割り当てた TCP ソケットの総数。全状態を含む。", "割り当て済み TCP ソケット総数。\n\n💡 診断:\n  • alloc >> inuse + tw → 遷移状態のソケットが多い\n  • TCP メモリ制限: /proc/sys/net/ipv4/tcp_mem"),
        ("net/sockstat", "TCP_mem") => ("TCP メモリ使用量（ページ）。", "全 TCP ソケットが消費するカーネルメモリページ数。各ページは通常 4KB。", "TCP メモリ消費量（カーネルページ単位）。\n\n💡 診断:\n  • /proc/sys/net/ipv4/tcp_mem のしきい値と比較\n  • 'high' 超過 → 接続ドロップ"),
        ("net/sockstat", "UDP_inuse") => ("使用中の UDP ソケット数。", "現在使用中の UDP ソケット数。DNS、NTP、ロギング等を含む。", "アクティブな UDP ソケット。\n\n💡 診断:\n  • 一般的な値: 静かなサーバーで 5-20\n  • UDP には輻輳制御がない"),
        ("net/sockstat", "UDP_mem") => ("UDP メモリ使用量（ページ）。", "UDP ソケットが消費するカーネルメモリページ数。", "UDP メモリ消費量。\n\n💡 診断:\n  • メモリ圧迫下では受信 UDP パケットを無言でドロップ\n  • net/snmp の Udp_RcvbufErrors を確認"),
        ("net/sockstat", "FRAG_inuse") => ("IP フラグメント再構成エントリ数。", "非ゼロ値はフラグメント化されたパケットの受信を示す。", "IP フラグメント再構成キューエントリ。\n\n💡 診断:\n  • 現代のネットワークでは通常 0\n  • 非ゼロ → MTU 不一致か PMTUD がブロック"),
        ("net/snmp", "Tcp_ActiveOpens") => ("TCP 接続開始数（クライアント側）。", "このホストが初期 SYN を送信した TCP 接続の累積数。", "TCP アクティブオープン。\n\n💡 診断:\n  • Tcp_PassiveOpens と比較してクライアント/サーバー役割を判断\n  • Tcp_AttemptFails / Tcp_ActiveOpens = 接続失敗率"),
        ("net/snmp", "Tcp_PassiveOpens") => ("TCP 接続受付数（サーバー側）。", "listen/accept で受け付けた TCP 接続の累積数。", "TCP パッシブオープン。\n\n💡 診断:\n  • PassiveOpens >> ActiveOpens → 主にサーバー役割\n  • レートの突然の低下 → クライアントがサービスに到達不可"),
        ("net/snmp", "Tcp_RetransSegs") => ("TCP セグメント再送数。", "再送された TCP セグメントの累積数。パケットロスを示す。", "TCP 再送カウンタ。\n\n💡 診断:\n  • RetransSegs / OutSegs = 再送率\n  • 1% 超 → 深刻なパケットロス"),
        ("net/snmp", "Tcp_InErrs") => ("エラーのある TCP セグメント受信数。", "チェックサムエラー等の TCP セグメント数。データ破損を示す。", "TCP 入力エラー。\n\n💡 診断:\n  • 0 に近いはず\n  • 非ゼロ → NIC 障害、不良ケーブル、またはドライバのバグ"),
        ("net/snmp", "Udp_InErrors") => ("配信できなかった UDP データグラム数。", "配信できなかった受信 UDP データグラムの累積数。", "UDP 入力エラー。\n\n💡 診断:\n  • 一般的な原因: リスンするプロセスなし、受信バッファオーバーフロー\n  • Udp_RcvbufErrors でバッファオーバーフローを確認"),
        ("net/snmp", "Ip_InReceives") => ("受信 IP データグラム総数。", "全受信 IP データグラムの累積数。最上位入力カウンタ。", "IP 入力データグラム総数。\n\n💡 診断:\n  • 変化率がネットワーク入力負荷を示す\n  • ルーターでなければ InReceives ≈ InDelivers"),
        ("net/snmp", "Ip_OutRequests") => ("送信 IP データグラム総数。", "送信のために渡された全 IP データグラムの累積数。", "IP 出力データグラム総数。\n\n💡 診断:\n  • OutRequests >> InReceives → サーバー役割\n  • 突然のスパイク → 新ワークロードか侵害の可能性"),
        ("net/netstat", "TcpExt_ListenOverflows") => ("リスンキューオーバーフロー回数。", "リスンバックログキューが満杯だった回数。", "TCP リスンキューオーバーフロー。\n\n💡 診断:\n  • 通常は 0 であるべき\n  • 非ゼロ → net.core.somaxconn を増加"),
        ("net/netstat", "TcpExt_ListenDrops") => ("リスンキューからのドロップ数。", "リスンキュー満杯で接続がドロップされた数。", "TCP リスンキュードロップ。\n\n💡 診断:\n  • 非ゼロ = クライアントが拒否されている\n  • レート（ドロップ/秒）を監視"),
        ("net/netstat", "TcpExt_TCPTimeouts") => ("TCP 接続タイムアウト数。", "応答待ちでタイムアウトした TCP 接続数。", "TCP タイムアウトイベント。\n\n💡 診断:\n  • 高タイムアウト + 高再送 → 永続的なネットワーク経路障害\n  • 時間帯との相関を確認（輻輳パターン）"),
        ("net/wireless", "interface_count") => ("無線インターフェース数。", "カーネルが検出した無線ネットワークインターフェースの数。", "無線インターフェース数。\n\n💡 診断:\n  • 0 → 無線ハードウェア未検出またはドライバ未ロード\n  • 通常は Wi-Fi 付きラップトップで 1"),
        ("net/wireless", "interfaces") => ("無線インターフェース統計。", "無線インターフェースのテーブル。状態、リンク品質、信号レベル、ノイズレベル。", "インターフェース別無線統計。\n\nカラム: iface、status、link quality、signal level（dBm）、noise level（dBm）。\n\n💡 診断:\n  • 信号レベル: -30=優秀、-67=良好、-70=普通、-80=弱い、-90=使用不可\n  • SNR > 25 dB → 良好; < 15 dB → 不良"),

        // ── グループ 4: ストレージ ────────────────────────────────────
        ("mounts", "count") => ("マウント済みファイルシステム数。", "全マウント済みファイルシステムの数。仮想ファイルシステムを含む。", "マウント済みファイルシステムの総数。\n\n💡 診断:\n  • 一般的な Linux: 30-60 マウント\n  • 200+ → コンテナホストまたは NFS 多用環境"),
        ("mounts", "mounts") => ("マウント済みファイルシステム詳細。", "デバイス、マウントポイント、ファイルシステムタイプ、マウントオプションを表示。", "現在マウントされている全ファイルシステム。\n\nカラム: device、mountpoint、fstype、options。\n\n💡 診断:\n  • 'ro' オプション → 読み取り専用に再マウント、ディスクエラーの可能性\n  • NFS 'hard' → サーバー到達不能でハング\n  • tmpfs → RAM ベース; サイズ制限を確認"),
        ("partitions", "count") => ("ブロックデバイスパーティション数。", "カーネルが認識した全パーティション数。ディスク全体とサブパーティションを含む。", "ブロックデバイスパーティション数。\n\n💡 診断:\n  • 期待される全ディスクが表示されているか確認\n  • ディスクが見えない → ハードウェア障害またはドライバの問題"),
        ("partitions", "partitions") => ("パーティションテーブル詳細。", "名前、サイズ、メジャー/マイナーデバイス番号を表示。", "ブロックデバイスパーティションテーブル。\n\nカラム: name、size、major、minor。\n\n💡 診断:\n  • メジャー 8=SCSI/SATA、259=NVMe、253=device-mapper\n  • ループデバイス（メジャー 7）→ Snap パッケージまたは ISO"),
        ("diskstats", "active_devices") => ("I/O 活動のあるデバイス数。", "起動以降に読み書き操作があったブロックデバイスの数。", "アクティブなブロックデバイス数。\n\n💡 診断:\n  • 多数の非アクティブデバイスは正常\n  • 期待されるデバイスの I/O がゼロ → 未使用またはワークロード未開始"),
        ("diskstats", "devices") => ("デバイス別 I/O 統計。", "名前、読み取り数、読み取りバイト、書き込み数、書き込みバイト、処理中 I/O を表示。I/O ボトルネック特定に重要。", "デバイス別ディスク I/O 統計。\n\nカラム: name、reads、bytes read、writes、bytes written、in-flight。\n\n💡 診断:\n  • in-flight > 0 持続 → アクティブ I/O 負荷。HDD で >2 は飽和を示唆。\n  • Reads >> Writes → 読み取り中心。RAM 増設でページキャッシュ拡大。\n  • SSD: 高い書き込み量は寿命を劣化。SMART データを確認。\n  • PSI io_some_avg10 と相関させて I/O 停滞を判断。"),
        ("locks", "lock_count") => ("アクティブなファイルロック数。", "全アクティブファイルロック数（POSIX/FLOCK）。", "アクティブなファイルロック数。\n\n💡 診断:\n  • 一般的な値: 10-50\n  • アプリがハング → ロック待ちか確認。PID カラムが保持者を特定。"),
        ("locks", "locks") => ("アクティブなファイルロック詳細。", "タイプ、モード、PID、inode 情報、バイト範囲を表示。", "全アクティブファイルロック。\n\nカラム: type、mode、pid、inode_info、range_start、range_end。\n\n💡 診断:\n  • WRITE ロックは全アクセスをブロック\n  • デッドロック: プロセス A が X を保持し Y を待機、B が Y を保持し X を待機"),

        // ── グループ 5: セキュリティ & カーネル ──────────────────────
        ("modules", "module_count") => ("ロード済みカーネルモジュール数。", "現在ロードされているカーネルモジュール数。", "ロード済みカーネルモジュール数。\n\n💡 診断:\n  • 一般的: 50-150 モジュール\n  • 予期しないモジュール → ルートキットの可能性\n  • 期待されるモジュールがない → `modprobe <name>` を試す"),
        ("modules", "modules") => ("ロード済みカーネルモジュール詳細。", "名前、メモリサイズ、参照カウント、依存関係、状態を表示。", "全ロード済みカーネルモジュール。\n\nカラム: name、size、refcount、dependencies、state。\n\n💡 診断:\n  • refcount > 0 → アンロード不可\n  • refcount = 0 → `rmmod` で安全にアンロード可能\n  • State 'Live' → 正常"),
        ("interrupts", "cpu_count") => ("割り込みを処理する CPU 数。", "割り込みテーブルに見える CPU 数。", "割り込みテーブルからの CPU 数。\n\n💡 診断: 割り込みが CPU 間でバランスしているか確認。`irqbalance` デーモンを使用。"),
        ("interrupts", "irq_count") => ("IRQ ライン数。", "異なる割り込み要求ラインの数。", "IRQ ライン総数。\n\n💡 診断:\n  • 特定 IRQ のカウントが非常に高い → そのデバイスが非常にアクティブ\n  • 共有 IRQ はレガシーシステムで性能問題の可能性"),
        ("interrupts", "interrupts") => ("ハードウェア割り込みカウンタ。", "全割り込みラインの IRQ 番号、合計カウント、説明を表示。", "IRQ 別割り込みカウンタ。\n\nカラム: irq、total_count、description。\n\n💡 診断:\n  • NIC 割り込みの高カウント = 高ネットワークスループット\n  • CPU 間の不均衡 → `irqbalance` を実行"),
        ("softirqs", "softirq_count") => ("ソフトウェア割り込みタイプ数。", "ソフトウェア割り込みタイプの数（通常 10）。", "ソフトウェア割り込みタイプ数。\n\nLinux 固定セット: HI、TIMER、NET_TX、NET_RX、BLOCK、IRQ_POLL、TASKLET、SCHED、HRTIMER、RCU。\n\n💡 カウント自体は固定。タイプ別活動が重要。"),
        ("softirqs", "softirqs") => ("ソフトウェア割り込みカウンタ。", "タイプ別の全 CPU 呼び出し回数合計。NET_RX/TX はネットワーク、TIMER はカーネルタイマー。", "タイプ別ソフトウェア割り込みカウンタ。\n\nカラム: name、total_count。\n\n💡 診断:\n  • NET_RX 高い → インバウンドネットワーク処理\n  • SCHED → ビジーなマルチスレッドシステムで高い\n  • CPU 間の不均衡 → IRQ アフィニティ問題"),
        ("cgroups", "controller_count") => ("cgroup コントローラ数。", "利用可能な cgroup コントローラの数。リソース管理に使用。", "利用可能な cgroup コントローラ数。\n\n💡 診断:\n  • 一般的: cpu、memory、blkio、pids\n  • コントローラがない → カーネル未サポートまたは未マウント"),
        ("cgroups", "controllers") => ("cgroup コントローラ詳細。", "名前、階層 ID、cgroup 数、有効状態を表示。", "cgroup コントローラテーブル。\n\nカラム: name、hierarchy、num_cgroups、enabled。\n\n💡 診断:\n  • hierarchy = 0 → 未使用\n  • memory 未有効 → cgroup ごとのメモリ制限なし"),
        ("crypto", "algorithm_count") => ("登録済み暗号アルゴリズム数。", "カーネルで利用可能な暗号アルゴリズム数。", "登録済み暗号アルゴリズム数。\n\n💡 診断:\n  • 一般的: 50-200\n  • ハードウェアアクセラレーション（'aesni' 等）→ 高速暗号化"),
        ("crypto", "algorithms") => ("登録済み暗号アルゴリズム詳細。", "名前、タイプ、ドライバ実装、ソースモジュールを表示。", "暗号アルゴリズムテーブル。\n\nカラム: name、type、driver、module。\n\n💡 診断:\n  • 'skcipher' → 対称暗号\n  • 'ahash'/'shash' → ハッシュ\n  • Module 'kernel' → ビルトイン"),
        ("devices", "device_count") => ("登録済みデバイスドライバ数。", "全登録済みキャラクタ/ブロックデバイスドライバ数。", "登録済みデバイスドライバ数。\n\n💡 診断:\n  • 期待されるデバイスがない → ドライバ未ロード\n  • キャラクタデバイスは通常ブロックより多い"),
        ("devices", "devices") => ("登録済みデバイスドライバ詳細。", "タイプ、メジャー番号、デバイス名を表示。", "登録済みデバイスドライバテーブル。\n\nカラム: type、major、name。\n\n💡 診断:\n  • メジャー 8=sd（SCSI/SATA）、253=device-mapper、259=NVMe"),
        ("filesystems", "filesystem_count") => ("サポートされるファイルシステムタイプ数。", "カーネルがマウントできるファイルシステムタイプ数。", "サポートされるファイルシステムタイプ数。\n\n💡 診断:\n  • 期待される FS がない → `modprobe <fsname>` を試す\n  • 'nodev' → 仮想ファイルシステム"),
        ("filesystems", "filesystems") => ("サポートされるファイルシステムタイプ詳細。", "名前とブロックデバイス必要性（nodev）を表示。", "サポートされるファイルシステムテーブル。\n\nカラム: name、nodev。\n\n💡 診断:\n  • nodev='yes' → 仮想 FS、物理ディスク不要\n  • nodev='no' → ディスクベース FS\n  • FUSE → ユーザースペース FS ドライバ"),
        ("iomem", "region_count") => ("I/O メモリ領域数。", "物理メモリマップ I/O 領域数。", "I/O メモリ領域数。\n\n💡 診断:\n  • 'System RAM' 領域は使用可能な物理メモリ\n  • PCI デバイス名でハードウェア検出を確認"),
        ("iomem", "regions") => ("I/O メモリ領域マップ。", "アドレス範囲と説明のテーブル。", "物理 I/O メモリマップ。\n\nカラム: address_range、description。\n\n💡 診断:\n  • 'System RAM' → OS が利用可能な RAM\n  • 'Kernel code/data' → カーネル自体が使用"),
        ("ioports", "region_count") => ("I/O ポート領域数。", "I/O ポートアドレス領域数。レガシー x86 メカニズム。", "I/O ポート領域数。\n\n💡 診断:\n  • 主に x86/x86_64 で関連\n  • 現代のシステムでは PCI デバイスが BAR を通じて要求"),
        ("ioports", "regions") => ("I/O ポート領域マップ。", "ポート範囲と所有デバイスを表示。", "I/O ポートアドレスマップ。\n\nカラム: port_range、description。\n\n💡 診断:\n  • 0x60/0x64 → キーボードコントローラ\n  • 0x3F8 → COM1 シリアルポート"),
        ("consoles", "console_count") => ("登録済みコンソールデバイス数。", "カーネルに登録されたコンソールデバイスの数。", "登録済みコンソールデバイス数。\n\n💡 診断:\n  • 一般的: 1-3 コンソール\n  • 0 → カーネルメッセージ出力先なし"),
        ("consoles", "consoles") => ("登録済みコンソールデバイス詳細。", "名前とフラグを表示。", "コンソールデバイステーブル。\n\nカラム: name、flags。\n\n💡 診断:\n  • 'E'=有効、'W'=書き込み可、'R'=読み取り可\n  • ヘッドレスサーバー: シリアルコンソール ttyS0 を設定"),
        ("misc", "device_count") => ("misc デバイス数。", "メジャー番号 10 を共有する misc キャラクタデバイスの数。", "misc デバイス数。\n\n💡 診断:\n  • 'watchdog' → ハードウェアウォッチドッグ\n  • 'fuse' → FUSE サポート利用可能"),
        ("misc", "devices") => ("misc デバイス詳細。", "マイナー番号とデバイス名を表示。", "misc デバイステーブル。\n\nカラム: minor_number、name。\n\n💡 診断:\n  • 'watchdog' (130) → ウォッチドッグサポート\n  • 'kvm' → KVM 仮想化有効"),
        ("dma", "channel_count") => ("使用中の DMA チャネル数。", "ISA DMA チャネル数。現代システムでは 0 が多い。", "ISA DMA チャネル数。\n\n💡 診断:\n  • 0 → レガシーハードウェアなし、正常\n  • 現代の DMA（PCI バスマスタリング）はここに表示されない"),
        ("dma", "channels") => ("DMA チャネル詳細。", "チャネル番号と要求したデバイスを表示。", "ISA DMA チャネル割り当てテーブル。\n\nカラム: channel_number、device_name。\n\n💡 診断:\n  • チャネル 4 = カスケード\n  • 空の場合は ISA ハードウェアなしで想定通り"),
        ("timer_list", "version") => ("タイマーリストバージョン。", "/proc/timer_list フォーマットのバージョン。", "タイマーリストフォーマットバージョン。\n\n💡 メタデータフィールド。"),
        ("timer_list", "now") => ("現在のカーネル時間（ナノ秒）。", "カーネルの現在時間（ktime）。全タイマー満了の参照点。", "現在のカーネル時間（ktime_get）。\n\n単調時計 — NTP 調整の影響を受けない。\n\n💡 1,000,000,000 で割って起動からの秒数を取得。"),
        ("timer_list", "clock_count") => ("クロックイベントデバイス数。", "ハードウェアタイマーの数。各 CPU にローカル APIC タイマー。", "クロックイベントデバイス数。\n\n💡 診断:\n  • CPU 数とほぼ等しいはず\n  • 追加: HPET、PIT、TSC デッドラインタイマー"),
        ("timer_list", "timer_count") => ("保留中のカーネルタイマー数。", "現在キューに入っているタイマー数。", "保留中のカーネルタイマー数。\n\n💡 診断:\n  • 一般的: 50-500\n  • 10000+ → タイマー作成しすぎ（ネットワーク問題またはドライバ異常）"),
        ("schedstat", "version") => ("スケジューラ統計バージョン。", "schedstat フォーマットのバージョン番号。", "schedstat フォーマットバージョン。\n\n💡 バージョン 15 が現在のフォーマット。"),
        ("schedstat", "cpu_count") => ("スケジューラ統計のある CPU 数。", "スケジューラ統計を報告する CPU 数。", "スケジューラ統計の CPU 数。\n\n💡 cpuinfo の CPU 数と一致するはず。少ない場合は一部がオフライン。"),
        ("schedstat", "cpu_stats") => ("CPU 別スケジューラ統計。", "yield 回数、スケジュール回数、アイドル回数、try-to-wake-up 回数等。", "CPU 別スケジューラ統計テーブル。\n\nカラム: cpu、yld_count、sched_count、sched_goidle、ttwu_count、...\n\n💡 診断:\n  • 高い sched_count → 多数のコンテキストスイッチ\n  • sched_goidle / sched_count → アイドル率\n  • CPU 間の大きな格差 → ワークロード不均衡"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KEYS: &[&str] = &[
        T::SOURCE, T::DRILL_IN, T::BACK, T::DIFF, T::SEARCH,
        T::REFRESH, T::GRAPH, T::AUTO, T::EXPORT, T::QUIT,
        T::HELP, T::LANG, T::AGO, T::SNAPS,
        T::VIEW_OVERVIEW, T::VIEW_DETAIL, T::VIEW_DIFF, T::VIEW_TABLE, T::VIEW_GRAPH,
        T::FIELD, T::VALUE, T::UNIT, T::DESCRIPTION, T::OLD, T::NEW,
        T::NO_DATA, T::NO_CHANGES, T::NO_TABLE_DATA,
        T::EXPORTED, T::EXPORT_FAILED, T::SEARCHING,
    ];

    const ALL_SOURCES: &[&str] = &[
        "meminfo", "uptime", "loadavg", "version", "cpuinfo", "stat",
        "mounts", "partitions", "diskstats", "processes", "swaps",
        "net/dev", "net/tcp", "net/udp", "net/unix", "net/arp",
        "net/route", "net/sockstat", "net/snmp", "net/netstat", "net/wireless",
        "vmstat", "buddyinfo", "zoneinfo", "slabinfo", "pagetypeinfo",
        "modules", "interrupts", "softirqs", "schedstat", "timer_list",
        "pressure", "cgroups", "cmdline", "consoles", "crypto",
        "devices", "filesystems", "iomem", "ioports", "locks",
        "misc", "dma", "df", "thermal", "file-nr",
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
            assert_ne!(desc, "System information source",
                "EN source_description missing for '{}'", source);
            assert!(!desc.is_empty(), "EN source_description empty for '{}'", source);
        }
    }

    #[test]
    fn source_descriptions_complete_ja() {
        for source in ALL_SOURCES {
            let desc = source_description(Locale::Ja, source);
            assert_ne!(desc, "システム情報ソース",
                "JA source_description missing for '{}'", source);
            assert!(!desc.is_empty(), "JA source_description empty for '{}'", source);
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
