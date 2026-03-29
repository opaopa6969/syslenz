use crate::i18n::Locale;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Memory,
    CpuLoad,
    Network,
    Storage,
    Process,
    Hardware,
}

impl Category {
    pub fn all() -> &'static [Category] {
        &[
            Category::Memory,
            Category::CpuLoad,
            Category::Network,
            Category::Storage,
            Category::Process,
            Category::Hardware,
        ]
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Category::Memory => "MEM",
            Category::CpuLoad => "CPU",
            Category::Network => "NET",
            Category::Storage => "DSK",
            Category::Process => "PRC",
            Category::Hardware => "HW",
        }
    }

    pub fn name(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Category::Memory, Locale::En) => "Memory",
            (Category::Memory, Locale::Ja) => "メモリ",
            (Category::CpuLoad, Locale::En) => "CPU / Load",
            (Category::CpuLoad, Locale::Ja) => "CPU / 負荷",
            (Category::Network, Locale::En) => "Network",
            (Category::Network, Locale::Ja) => "ネットワーク",
            (Category::Storage, Locale::En) => "Storage",
            (Category::Storage, Locale::Ja) => "ストレージ",
            (Category::Process, Locale::En) => "Process",
            (Category::Process, Locale::Ja) => "プロセス",
            (Category::Hardware, Locale::En) => "Hardware",
            (Category::Hardware, Locale::Ja) => "ハードウェア",
        }
    }

    pub fn related_sources(&self) -> &'static [&'static str] {
        match self {
            Category::Memory => &[
                "meminfo", "vmstat", "swaps", "buddyinfo", "pressure",
                "zoneinfo", "slabinfo", "pagetypeinfo",
            ],
            Category::CpuLoad => &[
                "stat", "loadavg", "cpuinfo", "pressure", "schedstat",
                "softirqs", "interrupts",
            ],
            Category::Network => &[
                "net/dev", "net/tcp", "net/udp", "net/unix", "net/arp",
                "net/route", "net/sockstat", "net/snmp", "net/netstat",
                "net/wireless",
            ],
            Category::Storage => &[
                "diskstats", "df", "mounts", "partitions", "pressure", "locks",
            ],
            Category::Process => &["processes", "file-nr", "stat"],
            Category::Hardware => &["thermal", "cpuinfo"],
        }
    }
}

// =============================================================================
// Learning Paths — guided sequences through categories
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LearningPath {
    /// Start here: understand the big picture of Linux resource monitoring.
    Beginner,
    /// Deep-dive into performance bottleneck diagnosis across subsystems.
    PerformanceDiagnosis,
    /// Server health and capacity planning workflows.
    ServerHealth,
}

impl LearningPath {
    pub fn all() -> &'static [LearningPath] {
        &[
            LearningPath::Beginner,
            LearningPath::PerformanceDiagnosis,
            LearningPath::ServerHealth,
        ]
    }

    pub fn name(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (LearningPath::Beginner, Locale::En) => "Beginner: System Basics",
            (LearningPath::Beginner, Locale::Ja) => "初級：システムの基礎",
            (LearningPath::PerformanceDiagnosis, Locale::En) => "Performance Diagnosis",
            (LearningPath::PerformanceDiagnosis, Locale::Ja) => "パフォーマンス診断",
            (LearningPath::ServerHealth, Locale::En) => "Server Health & Capacity",
            (LearningPath::ServerHealth, Locale::Ja) => "サーバー健全性と容量計画",
        }
    }

    pub fn description(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (LearningPath::Beginner, Locale::En) => "\
Start with Memory to understand caching and MemAvailable, then move to \
CPU/Load for utilization vs demand, then Storage and Process basics. \
Finish with Network and Hardware for a complete picture.",
            (LearningPath::Beginner, Locale::Ja) => "\
まずメモリでキャッシュとMemAvailableを理解し、次にCPU/負荷で使用率と\
需要の違いを学び、ストレージとプロセスの基礎に進みます。最後にネットワーク\
とハードウェアで全体像を完成させます。",
            (LearningPath::PerformanceDiagnosis, Locale::En) => "\
Learn to trace slowness from symptoms to root cause: CPU load and \
pressure, then I/O saturation in Storage, memory pressure and swap \
storms, process contention, and thermal throttling in Hardware.",
            (LearningPath::PerformanceDiagnosis, Locale::Ja) => "\
症状から根本原因まで遅延を追跡する方法を学びます：CPU負荷と圧力、\
ストレージのI/O飽和、メモリ圧力とスワップストーム、プロセス競合、\
ハードウェアのサーマルスロットリング。",
            (LearningPath::ServerHealth, Locale::En) => "\
Focus on capacity indicators: disk space trends in Storage, memory \
headroom, file descriptor limits in Process, NIC saturation in \
Network, and thermal envelope in Hardware.",
            (LearningPath::ServerHealth, Locale::Ja) => "\
容量指標に注目：ストレージのディスク容量推移、メモリの余裕、プロセスの\
ファイルディスクリプタ上限、ネットワークのNIC飽和度、ハードウェアの\
熱的余裕。",
        }
    }

    /// Returns the ordered sequence of categories for this learning path.
    pub fn categories(&self) -> &'static [Category] {
        match self {
            LearningPath::Beginner => &[
                Category::Memory,
                Category::CpuLoad,
                Category::Storage,
                Category::Process,
                Category::Network,
                Category::Hardware,
            ],
            LearningPath::PerformanceDiagnosis => &[
                Category::CpuLoad,
                Category::Storage,
                Category::Memory,
                Category::Process,
                Category::Hardware,
            ],
            LearningPath::ServerHealth => &[
                Category::Storage,
                Category::Memory,
                Category::Process,
                Category::Network,
                Category::Hardware,
            ],
        }
    }
}

// =============================================================================
// Category educational content
// =============================================================================

pub struct CategoryContent {
    pub overview: &'static str,
    pub story: &'static str,
    pub diagnostic_flow: &'static str,
    pub common_issues: &'static str,
}

pub fn get_content(category: Category, locale: Locale) -> CategoryContent {
    match (category, locale) {
        // =====================================================================
        // MEMORY — English
        // =====================================================================
        (Category::Memory, Locale::En) => CategoryContent {
            overview: "\
Memory is the most misunderstood Linux subsystem. On a healthy Linux \
system, nearly all RAM appears \"used\" because the kernel aggressively \
caches disk data. The key insight: \"used\" does not mean \"unavailable.\" \
MemAvailable (not MemFree) is the true indicator of how much memory \
applications can still allocate without triggering swapping.",

            story: "\
\"Where did all my RAM go?\" — The Memory Story\n\
\n\
Start at meminfo. MemTotal shows your total physical RAM. MemFree is \
the truly idle portion — often surprisingly small. Do not panic.\n\
\n\
Linux uses free RAM for Cached (page cache for file data) and Buffers \
(metadata cache). These are automatically reclaimed when applications \
need memory. This is normal and desirable — idle RAM is wasted RAM.\n\
\n\
MemAvailable is the kernel's estimate of memory available for new \
allocations without swapping. It combines MemFree + reclaimable caches \
+ reclaimable Slab. If MemAvailable is above 10% of MemTotal, you are \
generally fine.\n\
\n\
When MemAvailable drops low, the kernel begins swapping. Check swaps: \
if SwapUsed is growing, real memory pressure exists. Confirm with \
vmstat: rising si (swap-in) and so (swap-out) values mean the system \
is actively moving pages between RAM and disk. Also check pgmajfault \
— major page faults indicate the kernel is fetching pages from disk \
that were evicted from RAM.\n\
\n\
Finally, pressure (PSI) gives a direct answer: if memory_some_avg10 > 0, \
tasks are currently stalled waiting for memory. buddyinfo reveals memory \
fragmentation — if high-order free blocks are all zero, the kernel \
cannot allocate large contiguous regions even if total free memory seems \
adequate.",

            diagnostic_flow: "\
Step 1: Check meminfo → MemAvailable\n\
  If > 10% of MemTotal → Memory is OK. Stop here.\n\
  If < 10% of MemTotal → Continue to Step 2.\n\
\n\
Step 2: Check meminfo → Cached + Buffers\n\
  If large → Caches are using RAM (normal). Kernel will reclaim if needed.\n\
  If small → Real memory shortage. Continue to Step 3.\n\
\n\
Step 3: Check swaps → SwapUsed\n\
  If 0 or stable → Memory is tight but not swapping yet.\n\
  If growing → Active swapping. Continue to Step 4.\n\
\n\
Step 4: Check vmstat → si, so, pgmajfault\n\
  If si/so > 0 → Swap thrashing. Performance will degrade.\n\
  If pgmajfault rising → Pages being fetched from disk constantly.\n\
\n\
Step 5: Check pressure → memory_some_avg10\n\
  If > 0 → Tasks are stalling on memory. This is real impact.\n\
  If 0 → Swapping exists but not yet causing task stalls.",

            common_issues: "\
OOM Killer: When the kernel runs out of memory and swap, it invokes \
the Out-Of-Memory killer to terminate processes. Check dmesg for \
\"Out of memory\" messages. Prevent by monitoring MemAvailable trend.\n\
\n\
\"My RAM is full!\" (False Alarm): MemFree is low but MemAvailable is \
healthy. Cached + Buffers are using RAM for disk cache. This is Linux \
working correctly — do not restart services or add swap.\n\
\n\
Memory Leaks: A process gradually consumes more RSS over time without \
releasing it. Check processes table for RSS growth. MemAvailable will \
slowly decline. Fix the leaking application.\n\
\n\
Swap Storms: SwapUsed is large and vmstat si/so are constantly active. \
The system spends more time swapping than doing useful work. Solutions: \
add RAM, reduce workload, or tune vm.swappiness.\n\
\n\
Cache Pressure (Slab Growth): slabinfo shows kernel caches growing. \
Dentry and inode caches can grow large on systems with many files. \
Usually reclaimable, but check SReclaimable vs SUnreclaim in meminfo.",
        },

        // =====================================================================
        // MEMORY — Japanese
        // =====================================================================
        (Category::Memory, Locale::Ja) => CategoryContent {
            overview: "\
メモリはLinuxで最も誤解されるサブシステムです。健全なLinuxシステムでは、\
ほぼ全てのRAMが「使用中」に見えます。カーネルがディスクデータを積極的に\
キャッシュするためです。重要な点：「使用中」は「利用不可」ではありません。\
MemAvailable（MemFreeではない）が、スワップを発生させずにアプリケーションが\
確保できるメモリの真の指標です。",

            story: "\
「RAMはどこに消えた？」 — メモリのストーリー\n\
\n\
まず meminfo を確認します。MemTotal が物理RAMの総量です。MemFree は\
完全に未使用の部分で、驚くほど少ないことが多いです。慌てないでください。\n\
\n\
Linux は空きRAMを Cached（ファイルデータのページキャッシュ）と Buffers\
（メタデータキャッシュ）に使います。これらはアプリケーションがメモリを必要と\
すれば自動的に回収されます。正常で望ましい動作です — 空きRAMは無駄なRAMです。\n\
\n\
MemAvailable はカーネルが計算した「スワップなしで新規割り当て可能なメモリ量」\
の推定値です。MemFree + 回収可能なキャッシュ + 回収可能なSlabの合計です。\
MemAvailable が MemTotal の10%以上あれば、通常は問題ありません。\n\
\n\
MemAvailable が低下すると、カーネルはスワップを開始します。swaps を確認：\
SwapUsed が増加していれば、実際のメモリ圧力があります。vmstat で確認：\
si（スワップイン）と so（スワップアウト）の値が上昇していれば、システムは\
RAMとディスク間でページを活発に移動しています。pgmajfault も確認 — メジャー\
ページフォルトは、RAMから追い出されたページをディスクから取得していることを\
示します。\n\
\n\
最後に、pressure（PSI）が直接的な答えを与えます：memory_some_avg10 > 0 なら、\
タスクが現在メモリ待ちで停滞しています。buddyinfo はメモリの断片化を示します\
 — 高オーダーの空きブロックが全て0なら、合計空きメモリが十分に見えても、\
カーネルは大きな連続領域を確保できません。",

            diagnostic_flow: "\
ステップ1: meminfo → MemAvailable を確認\n\
  MemTotal の10%以上 → メモリは問題なし。ここで終了。\n\
  MemTotal の10%未満 → ステップ2へ。\n\
\n\
ステップ2: meminfo → Cached + Buffers を確認\n\
  大きい → キャッシュがRAMを使用中（正常）。必要時にカーネルが回収。\n\
  小さい → 本当のメモリ不足。ステップ3へ。\n\
\n\
ステップ3: swaps → SwapUsed を確認\n\
  0 または安定 → メモリは逼迫しているがスワップはまだ発生していない。\n\
  増加中 → アクティブにスワップ中。ステップ4へ。\n\
\n\
ステップ4: vmstat → si, so, pgmajfault を確認\n\
  si/so > 0 → スワップスラッシング。性能が低下する。\n\
  pgmajfault 増加中 → ページが常にディスクから取得されている。\n\
\n\
ステップ5: pressure → memory_some_avg10 を確認\n\
  > 0 → タスクがメモリ待ちで停滞中。実際の影響あり。\n\
  0 → スワップは存在するがタスクの停滞は未発生。",

            common_issues: "\
OOM Killer: カーネルがメモリとスワップを使い果たすと、OOM Killerが起動して\
プロセスを強制終了します。dmesg で「Out of memory」メッセージを確認。\
MemAvailable の推移を監視して予防しましょう。\n\
\n\
「RAMがいっぱい！」（誤警報）: MemFree は低いが MemAvailable は健全。\
Cached + Buffers がディスクキャッシュとしてRAMを使用中。これはLinuxの正常な\
動作です — サービスの再起動やスワップの追加は不要です。\n\
\n\
メモリリーク: プロセスが時間とともにRSSを徐々に消費し、解放しない。\
processes テーブルでRSSの増加を確認。MemAvailable が徐々に低下します。\
リークしているアプリケーションを修正してください。\n\
\n\
スワップストーム: SwapUsed が大きく、vmstat の si/so が常にアクティブ。\
システムが有用な作業よりスワップに時間を費やしている状態。対処法：RAMの追加、\
ワークロードの削減、または vm.swappiness の調整。\n\
\n\
キャッシュ圧力（Slab増大）: slabinfo でカーネルキャッシュの増大を確認。\
多数のファイルがあるシステムでは dentry と inode キャッシュが大きくなります。\
通常は回収可能ですが、meminfo の SReclaimable と SUnreclaim を確認。",
        },

        // =====================================================================
        // CPU/LOAD — English
        // =====================================================================
        (Category::CpuLoad, Locale::En) => CategoryContent {
            overview: "\
CPU metrics tell you how busy your processors are and what they are \
doing. Load average is the most commonly checked but least understood \
metric: it measures demand, not utilization. A load of 4.0 on a 4-core \
system means each core has roughly one task — busy but not overloaded. \
Understanding the difference between load average, CPU utilization \
breakdown, and PSI pressure is essential for diagnosing performance.",

            story: "\
\"Why is my server slow?\" — The CPU Story\n\
\n\
Start at loadavg. The three numbers (1min, 5min, 15min) show average \
demand — the number of tasks wanting CPU time plus tasks in \
uninterruptible I/O wait. Compare load to your CPU count (from cpuinfo): \
if load1 < number of cores, the system is not overloaded.\n\
\n\
But load average does not tell you what the CPU is doing. Switch to \
stat for the breakdown: cpu_user (application code), cpu_system (kernel), \
cpu_idle (nothing to do), cpu_iowait (waiting for disk), cpu_steal \
(hypervisor took your cycles). High cpu_user means application-bound. \
High cpu_iowait means disk-bound — the CPU is idle waiting for I/O, \
not actually busy.\n\
\n\
A common mistake: high iowait looks like CPU is busy, but it actually \
means the CPU is waiting for slow storage. The fix is faster disks, \
not more CPU.\n\
\n\
Next, check pressure (PSI). cpu_some_avg10 > 0 means tasks are actively \
stalling because no CPU time is available. This is the most direct \
measure of CPU contention — load average can be misleading (it includes \
I/O wait), but PSI measures actual stalls.\n\
\n\
For deeper analysis, schedstat shows per-CPU scheduling statistics: \
time spent running tasks, time spent idle, and number of context \
switches. High context_switches in stat may indicate too many threads \
competing. softirqs and interrupts show kernel overhead — network-heavy \
servers often show high NET_RX softirqs.",

            diagnostic_flow: "\
Step 1: Check loadavg → load1\n\
  Compare to CPU count (cpuinfo → processor count).\n\
  If load1 < cores → CPU is not the bottleneck. Stop here.\n\
  If load1 > cores → Continue to Step 2.\n\
\n\
Step 2: Check stat → cpu_user, cpu_system, cpu_iowait\n\
  If cpu_iowait is high → Problem is I/O, not CPU. Check Storage.\n\
  If cpu_user is high → Application is CPU-bound. Continue to Step 3.\n\
  If cpu_system is high → Kernel overhead. Check interrupts/softirqs.\n\
\n\
Step 3: Check pressure → cpu_some_avg10\n\
  If > 0 → Tasks are stalling for CPU. Real contention.\n\
  If 0 → Load is high but tasks are getting scheduled OK.\n\
\n\
Step 4: Check stat → context_switches\n\
  If very high (>100k/s) → Too many threads competing.\n\
  Consider reducing thread counts or consolidating workloads.\n\
\n\
Step 5: Check schedstat for per-CPU imbalance\n\
  If one CPU is much busier than others → Workload affinity issue.",

            common_issues: "\
CPU Saturation: load1 exceeds core count and cpu_some_avg10 > 0. \
Tasks are queuing for CPU time. Solutions: optimize application code, \
reduce thread count, add CPU cores, or distribute load.\n\
\n\
iowait Misinterpretation: High iowait is often mistaken for CPU \
being busy. In reality, the CPU has nothing to do while waiting for \
disk I/O. Fix the storage bottleneck, not the CPU.\n\
\n\
Context Switch Storms: Extremely high context_switches (>500k/s) with \
degraded performance. Too many threads or processes are competing for \
time slices. Reduce concurrency or increase thread affinity.\n\
\n\
Steal Time: In virtual machines, cpu_steal > 0 means the hypervisor \
is taking CPU cycles for other VMs. Your instance is not getting the \
CPU time it needs. Solution: resize VM or reduce neighbor contention.",
        },

        // =====================================================================
        // CPU/LOAD — Japanese
        // =====================================================================
        (Category::CpuLoad, Locale::Ja) => CategoryContent {
            overview: "\
CPUメトリクスは、プロセッサがどれほど忙しく、何をしているかを示します。\
ロードアベレージは最も頻繁に確認されますが、最も誤解されるメトリクスです：\
使用率ではなく需要を測定します。4コアシステムでロード4.0は各コアにほぼ1つの\
タスクがある状態 — 忙しいが過負荷ではありません。ロードアベレージ、CPU使用率\
の内訳、PSI圧力の違いを理解することが性能診断に不可欠です。",

            story: "\
「なぜサーバーが遅い？」 — CPUのストーリー\n\
\n\
まず loadavg を確認します。3つの数値（1分、5分、15分）は平均需要を示します\
 — CPU時間を求めるタスク数 + 中断不可能なI/O待ちのタスク数です。\
ロードをCPU数（cpuinfo から）と比較：load1 < コア数なら、システムは過負荷\
ではありません。\n\
\n\
しかしロードアベレージはCPUが何をしているかは教えてくれません。stat で内訳を\
確認：cpu_user（アプリケーションコード）、cpu_system（カーネル）、cpu_idle\
（何もしていない）、cpu_iowait（ディスク待ち）、cpu_steal（ハイパーバイザが\
サイクルを取得）。cpu_user が高ければアプリケーションバウンド。cpu_iowait が\
高ければディスクバウンド — CPUはI/O待ちで遊んでおり、実際には忙しくありません。\n\
\n\
よくある間違い：高い iowait はCPUが忙しく見えますが、実際にはCPUは遅い\
ストレージを待っている状態です。修正すべきはCPUではなく、より高速なディスクです。\n\
\n\
次に pressure（PSI）を確認。cpu_some_avg10 > 0 は、CPU時間が利用できないため\
タスクが実際に停滞していることを意味します。CPU競合の最も直接的な指標です — \
ロードアベレージはI/O待ちを含むため誤解を招きますが、PSIは実際の停滞を測定します。\n\
\n\
より深い分析には、schedstat がCPUごとのスケジューリング統計を示します：\
タスク実行時間、アイドル時間、コンテキストスイッチ数。stat の context_switches \
が高い場合、スレッドが多すぎる可能性があります。softirqs と interrupts は\
カーネルのオーバーヘッドを示します — ネットワーク負荷の高いサーバーでは \
NET_RX softirqs が高くなります。",

            diagnostic_flow: "\
ステップ1: loadavg → load1 を確認\n\
  CPU数（cpuinfo → processor count）と比較。\n\
  load1 < コア数 → CPUはボトルネックではない。ここで終了。\n\
  load1 > コア数 → ステップ2へ。\n\
\n\
ステップ2: stat → cpu_user, cpu_system, cpu_iowait を確認\n\
  cpu_iowait が高い → 問題はI/O、CPUではない。ストレージを確認。\n\
  cpu_user が高い → アプリケーションがCPUバウンド。ステップ3へ。\n\
  cpu_system が高い → カーネルオーバーヘッド。interrupts/softirqsを確認。\n\
\n\
ステップ3: pressure → cpu_some_avg10 を確認\n\
  > 0 → タスクがCPU待ちで停滞。実際の競合あり。\n\
  0 → ロードは高いがタスクはスケジュールされている。\n\
\n\
ステップ4: stat → context_switches を確認\n\
  非常に高い（>10万/秒） → スレッドが多すぎる。\n\
  スレッド数の削減やワークロードの統合を検討。\n\
\n\
ステップ5: schedstat でCPUごとの偏りを確認\n\
  1つのCPUが他より著しく忙しい → ワークロードアフィニティの問題。",

            common_issues: "\
CPU飽和: load1 がコア数を超え、cpu_some_avg10 > 0。タスクがCPU時間を待って\
キューイング中。対処法：アプリケーションコードの最適化、スレッド数の削減、\
CPUコアの追加、または負荷分散。\n\
\n\
iowait の誤解: 高い iowait はCPUが忙しいと誤解されがち。実際にはCPUは\
ディスクI/O待ちで何もしていない。CPUではなくストレージのボトルネックを修正。\n\
\n\
コンテキストスイッチストーム: 非常に高い context_switches（>50万/秒）と\
性能低下。スレッドやプロセスが多すぎてタイムスライスを奪い合っている。\
並行度の削減やスレッドアフィニティの向上を検討。\n\
\n\
スチールタイム: 仮想マシンで cpu_steal > 0 はハイパーバイザが他のVMのために\
CPUサイクルを取得していることを意味する。インスタンスが必要なCPU時間を得ら\
れていない。対処法：VMのリサイズまたは隣接VM競合の削減。",
        },

        // =====================================================================
        // NETWORK — English
        // =====================================================================
        (Category::Network, Locale::En) => CategoryContent {
            overview: "\
Linux networking spans multiple layers from hardware NICs through the \
kernel network stack to userspace sockets. Problems can occur at any \
layer: dropped packets at the NIC, connection state issues in TCP, \
port exhaustion in the socket layer, or routing misconfigurations. \
Understanding which /proc/net source corresponds to which layer is \
the key to efficient network troubleshooting.",

            story: "\
\"The Life of a Packet\" — The Network Story\n\
\n\
A packet arrives at the NIC. net/dev tracks this: RX bytes, packets, \
errors, and drops per interface. If rx_errors or rx_drop are non-zero, \
packets are being lost at the hardware/driver level before the kernel \
even processes them. Check for NIC ring buffer overflows or driver \
issues.\n\
\n\
The kernel processes the packet through the network stack. net/snmp \
provides protocol-level counters (IP, TCP, UDP, ICMP). InReceives, \
OutRequests, InErrors — these show the health of each protocol layer. \
net/netstat has extended TCP statistics like TCPRetransSegs (segments \
retransmitted) which indicates network quality issues.\n\
\n\
For TCP connections, net/tcp lists every socket: local address, remote \
address, and state (ESTABLISHED, TIME_WAIT, CLOSE_WAIT, etc.). A surge \
in TIME_WAIT means many short-lived connections are closing. A surge in \
CLOSE_WAIT means the application is not closing sockets properly — this \
is a bug. net/udp shows UDP sockets similarly but without states.\n\
\n\
net/sockstat provides socket allocation summaries: total sockets in use, \
TCP sockets allocated, UDP in use, and orphaned sockets. If TCP alloc \
is growing unboundedly, you may have a connection leak. If orphans are \
high, sockets are in limbo — their owning process has gone but the \
socket lingers.\n\
\n\
Finally, net/arp shows the ARP cache (IP-to-MAC mappings) and net/route \
shows the kernel routing table. Stale ARP entries can cause intermittent \
connectivity; incorrect routes cause packets to go to the wrong gateway.",

            diagnostic_flow: "\
Step 1: Check net/dev → rx_errors, rx_drop, tx_errors, tx_drop\n\
  If non-zero → Packet loss at NIC level. Check hardware/drivers.\n\
  If all zero → Continue to Step 2.\n\
\n\
Step 2: Check net/sockstat → TCP alloc, orphans\n\
  If TCP alloc growing over time → Connection leak. App not closing sockets.\n\
  If orphans > 0 → Sockets without owning process. Investigate.\n\
  If normal → Continue to Step 3.\n\
\n\
Step 3: Check net/tcp → connection states\n\
  Many TIME_WAIT → Short-lived connections, may need connection pooling.\n\
  Many CLOSE_WAIT → App bug: not calling close() on sockets.\n\
  Many SYN_RECV → Possible SYN flood attack or slow backend.\n\
\n\
Step 4: Check net/snmp → TCPRetransSegs, InErrors\n\
  If TCPRetransSegs rising → Network quality issue (packet loss, latency).\n\
  If InErrors > 0 → Protocol-level errors. Check network path.\n\
\n\
Step 5: Check net/route and net/arp\n\
  Verify default gateway is correct.\n\
  Check for stale or incomplete ARP entries.",

            common_issues: "\
Connection Leaks: net/sockstat TCP alloc keeps growing. The application \
opens connections but never closes them. Check net/tcp for accumulating \
ESTABLISHED or CLOSE_WAIT sockets owned by the same process.\n\
\n\
Port Exhaustion: Unable to create new connections. The system has run \
out of ephemeral ports (default range: 32768-60999). Check net/tcp for \
too many TIME_WAIT sockets. Solutions: tune net.ipv4.tcp_tw_reuse, \
enable connection pooling, widen ephemeral port range.\n\
\n\
Packet Drops at NIC: net/dev shows rx_drop or tx_drop increasing. \
Causes: NIC ring buffer too small (ethtool -G), interrupt coalescing \
issues, or CPU too busy to drain packets. Check softirqs for NET_RX \
backlog.\n\
\n\
Retransmissions: net/snmp TCPRetransSegs climbing steadily. Indicates \
packet loss on the network path. Could be congestion, faulty switch, \
MTU mismatch, or WiFi interference. Use traceroute to find the lossy \
hop.",
        },

        // =====================================================================
        // NETWORK — Japanese
        // =====================================================================
        (Category::Network, Locale::Ja) => CategoryContent {
            overview: "\
Linuxネットワーキングは、ハードウェアNICからカーネルネットワークスタック、\
ユーザー空間ソケットまで複数のレイヤーにまたがります。問題はどのレイヤーでも\
発生し得ます：NICでのパケットドロップ、TCPの接続状態の問題、ソケット層での\
ポート枯渇、ルーティングの設定ミスなど。どの /proc/net ソースがどのレイヤーに\
対応するかを理解することが、効率的なネットワークトラブルシューティングの鍵です。",

            story: "\
「パケットの一生」 — ネットワークのストーリー\n\
\n\
パケットがNICに到着します。net/dev がこれを追跡：インターフェースごとの\
RXバイト数、パケット数、エラー、ドロップ。rx_errors や rx_drop がゼロでなければ、\
カーネルが処理する前にハードウェア/ドライバレベルでパケットが失われています。\
NICリングバッファのオーバーフローやドライバの問題を確認してください。\n\
\n\
カーネルがネットワークスタックを通じてパケットを処理します。net/snmp は\
プロトコルレベルのカウンター（IP、TCP、UDP、ICMP）を提供します。InReceives、\
OutRequests、InErrors — これらが各プロトコル層の健全性を示します。\
net/netstat には TCPRetransSegs（再送セグメント数）などの拡張TCP統計があり、\
ネットワーク品質の問題を示します。\n\
\n\
TCP接続については、net/tcp が全ソケットをリスト：ローカルアドレス、リモート\
アドレス、状態（ESTABLISHED、TIME_WAIT、CLOSE_WAITなど）。TIME_WAIT の急増は\
多くの短命な接続が閉じていることを意味します。CLOSE_WAIT の急増はアプリケーション\
がソケットを正しく閉じていないことを意味します — これはバグです。net/udp は\
UDPソケットを同様に表示しますが状態はありません。\n\
\n\
net/sockstat はソケット割り当ての要約を提供：使用中の合計ソケット数、\
TCP割り当て数、UDP使用数、孤立ソケット数。TCP alloc が際限なく増加していれば、\
コネクションリークの可能性があります。孤立ソケットが多ければ、所有プロセスが\
なくなったソケットが残存しています。\n\
\n\
最後に、net/arp はARPキャッシュ（IPからMACへのマッピング）、net/route は\
カーネルルーティングテーブルを示します。古いARPエントリは間欠的な接続問題の\
原因に、不正なルートはパケットが間違ったゲートウェイに送られる原因になります。",

            diagnostic_flow: "\
ステップ1: net/dev → rx_errors, rx_drop, tx_errors, tx_drop を確認\n\
  ゼロでない → NICレベルでパケットロス。ハードウェア/ドライバを確認。\n\
  全てゼロ → ステップ2へ。\n\
\n\
ステップ2: net/sockstat → TCP alloc, orphans を確認\n\
  TCP alloc が増加し続ける → コネクションリーク。アプリがソケットを閉じていない。\n\
  orphans > 0 → 所有プロセスのないソケット。調査が必要。\n\
  正常 → ステップ3へ。\n\
\n\
ステップ3: net/tcp → 接続状態を確認\n\
  TIME_WAIT が多い → 短命な接続。コネクションプーリングの検討を。\n\
  CLOSE_WAIT が多い → アプリのバグ：ソケットの close() を呼んでいない。\n\
  SYN_RECV が多い → SYNフラッド攻撃またはバックエンドの遅延の可能性。\n\
\n\
ステップ4: net/snmp → TCPRetransSegs, InErrors を確認\n\
  TCPRetransSegs 上昇中 → ネットワーク品質の問題（パケットロス、遅延）。\n\
  InErrors > 0 → プロトコルレベルのエラー。ネットワーク経路を確認。\n\
\n\
ステップ5: net/route と net/arp を確認\n\
  デフォルトゲートウェイが正しいか確認。\n\
  古いまたは不完全なARPエントリがないか確認。",

            common_issues: "\
コネクションリーク: net/sockstat の TCP alloc が増加し続ける。アプリケーションが\
接続を開くが閉じない。net/tcp で同じプロセスが所有する ESTABLISHED または \
CLOSE_WAIT ソケットの蓄積を確認。\n\
\n\
ポート枯渇: 新しい接続を作成できない。エフェメラルポート（デフォルト範囲：\
32768-60999）が枯渇。net/tcp で TIME_WAIT ソケットが多すぎないか確認。\
対処法：net.ipv4.tcp_tw_reuse の調整、コネクションプーリングの有効化、\
エフェメラルポート範囲の拡大。\n\
\n\
NICでのパケットドロップ: net/dev の rx_drop または tx_drop が増加。\
原因：NICリングバッファが小さすぎる（ethtool -G）、割り込みコアレッシングの\
問題、またはCPUがパケットを処理しきれない。softirqs の NET_RX バックログを\
確認。\n\
\n\
再送: net/snmp の TCPRetransSegs が着実に上昇。ネットワーク経路でのパケット\
ロスを示す。輻輳、スイッチの故障、MTUの不一致、またはWiFiの干渉の可能性。\
traceroute でロスの発生箇所を特定。",
        },

        // =====================================================================
        // STORAGE — English
        // =====================================================================
        (Category::Storage, Locale::En) => CategoryContent {
            overview: "\
Disk I/O is one of the most common bottlenecks in modern systems. \
Unlike CPU and memory which operate in nanoseconds, storage devices \
operate in microseconds (SSDs) to milliseconds (HDDs). Linux tracks \
disk activity through diskstats (per-device I/O counters), df (filesystem \
space usage), mounts (mounted filesystems and their options), partitions \
(block device layout), and pressure (PSI for I/O stalls). Understanding \
these together reveals whether your storage is the bottleneck and why.",

            story: "\
\"Why is my disk slow?\" — The Storage Story\n\
\n\
Start at diskstats. Each block device shows reads_completed, \
writes_completed, read_sectors, write_sectors, and critically: \
io_in_progress and weighted_io_ms. If io_in_progress is consistently \
above 0, the device has a queue of pending I/O requests. The \
weighted_io_ms counter accumulates time-weighted I/O operations — \
dividing this by total operations gives average I/O latency.\n\
\n\
Next check df for filesystem space usage. A filesystem above 90% full \
can degrade performance significantly: the filesystem driver must search \
harder for free blocks, and fragmentation increases. At 100%, writes \
fail entirely and applications crash. The available space (not just \
used) matters because ext4 reserves 5% for root by default.\n\
\n\
Check mounts for filesystem types and mount options. A filesystem \
mounted without noatime generates extra write I/O for every read \
(updating access timestamps). Write-heavy workloads on ext4 should \
check if data=journal mode is causing double-writes. The filesystem \
type itself matters: ext4, xfs, and btrfs have very different \
performance characteristics.\n\
\n\
Finally, pressure (PSI) for I/O gives a direct answer: if \
io_some_avg10 > 0, tasks are currently stalling because they are \
waiting for disk I/O. If io_full_avg10 > 0, the entire system is \
stalled on I/O — nothing useful is happening while waiting for disk.",

            diagnostic_flow: "\
Step 1: Check pressure → io_some_avg10\n\
  If 0 → I/O is not currently a bottleneck. Stop here.\n\
  If > 0 → Tasks are stalling on I/O. Continue to Step 2.\n\
\n\
Step 2: Check diskstats → io_in_progress, weighted_io_ms\n\
  If io_in_progress consistently > 0 → Device has I/O queue.\n\
  Calculate avg latency = weighted_io_ms / total_ops.\n\
  If latency > 20ms on SSD → Device is struggling. Continue to Step 3.\n\
  If latency > 100ms on HDD → Expected for spinning disk under load.\n\
\n\
Step 3: Check df → filesystem usage percentage\n\
  If > 95% → Nearly full. Free space urgently. This causes slowdowns.\n\
  If > 90% → Getting tight. Plan cleanup or expansion.\n\
  If < 90% → Space is not the issue. Continue to Step 4.\n\
\n\
Step 4: Check mounts → mount options and filesystem type\n\
  If noatime missing → Add noatime to reduce write amplification.\n\
  If data=journal on ext4 → Consider data=ordered for better write throughput.\n\
  If using HDD → Consider migrating to SSD for latency-sensitive workloads.\n\
\n\
Step 5: Check diskstats → read vs write ratio\n\
  If reads dominate → Check if page cache (meminfo Cached) is too small.\n\
  If writes dominate → Check for write-heavy applications or logging.",

            common_issues: "\
Disk Full: df shows 100% usage or 0 available. Applications fail with \
\"No space left on device.\" Find large files with du, check for deleted \
files held open by processes (lsof), and clean up logs. Remember ext4 \
reserves 5% for root — tune with tune2fs -m if needed.\n\
\n\
I/O Saturation: diskstats shows io_in_progress consistently > 0 and \
io_some_avg10 > 0 in pressure. The device cannot keep up with demand. \
Solutions: upgrade to faster storage (HDD to SSD), spread I/O across \
multiple devices (RAID or separate mount points), or reduce I/O demand \
(add RAM for better page cache, batch writes).\n\
\n\
Filesystem Corruption: Unexpected reboots or hardware failures can \
leave filesystems in an inconsistent state. Symptoms: I/O errors in \
dmesg, files or directories inaccessible. Run fsck on unmounted \
filesystems. For critical data, always use journaling filesystems \
(ext4, xfs) which recover automatically from most crashes.\n\
\n\
Slow Writes (Write Amplification): Writes are slower than expected. \
Common causes: missing noatime mount option (every read triggers a \
write), journaling overhead (data=journal doubles write volume), \
filesystem nearly full (fragmentation), or SSD nearing end of life \
(check SMART attributes via smartctl).",
        },

        // =====================================================================
        // STORAGE — Japanese
        // =====================================================================
        (Category::Storage, Locale::Ja) => CategoryContent {
            overview: "\
ディスクI/Oは現代のシステムで最も一般的なボトルネックの一つです。\
ナノ秒単位で動作するCPUやメモリと異なり、ストレージデバイスは\
マイクロ秒（SSD）からミリ秒（HDD）単位で動作します。Linuxは\
diskstats（デバイスごとのI/Oカウンター）、df（ファイルシステムの\
容量使用状況）、mounts（マウントされたファイルシステムとオプション）、\
partitions（ブロックデバイスのレイアウト）、pressure（I/Oストールの\
PSI）を通じてディスク活動を追跡します。これらを組み合わせて理解する\
ことで、ストレージがボトルネックかどうか、そしてその理由が明らかになります。",

            story: "\
「なぜディスクが遅い？」 — ストレージのストーリー\n\
\n\
まず diskstats を確認します。各ブロックデバイスは reads_completed、\
writes_completed、read_sectors、write_sectors、そして重要な \
io_in_progress と weighted_io_ms を表示します。io_in_progress が\
常に0より大きければ、デバイスには保留中のI/Oリクエストのキューがあります。\
weighted_io_ms カウンターは時間加重I/O操作を蓄積します — これを\
総操作数で割ると平均I/Oレイテンシが得られます。\n\
\n\
次に df でファイルシステムの容量使用状況を確認します。90%以上使用中の\
ファイルシステムは性能が大幅に低下します：ファイルシステムドライバが\
空きブロックの検索に苦労し、断片化が増加します。100%で書き込みが完全に\
失敗し、アプリケーションがクラッシュします。利用可能な容量（使用量だけ\
でなく）が重要です。ext4はデフォルトで5%をrootに予約しているためです。\n\
\n\
mounts でファイルシステムの種類とマウントオプションを確認します。\
noatime なしでマウントされたファイルシステムは、すべての読み取りで\
余分な書き込みI/O（アクセスタイムスタンプの更新）が発生します。\
ext4での書き込み負荷が高いワークロードでは、data=journal モードが\
二重書き込みを引き起こしていないか確認してください。ファイルシステムの\
種類自体が重要です：ext4、xfs、btrfsは非常に異なる性能特性を持ちます。\n\
\n\
最後に、I/Oの pressure（PSI）が直接的な答えを与えます：\
io_some_avg10 > 0 なら、タスクがディスクI/O待ちで停滞しています。\
io_full_avg10 > 0 なら、システム全体がI/Oで停滞しています — \
ディスク待ちの間、有用な処理は何も行われていません。",

            diagnostic_flow: "\
ステップ1: pressure → io_some_avg10 を確認\n\
  0 → I/Oは現在ボトルネックではない。ここで終了。\n\
  > 0 → タスクがI/O待ちで停滞中。ステップ2へ。\n\
\n\
ステップ2: diskstats → io_in_progress, weighted_io_ms を確認\n\
  io_in_progress が常に > 0 → デバイスにI/Oキューあり。\n\
  平均レイテンシ = weighted_io_ms / 総操作数 を計算。\n\
  SSDでレイテンシ > 20ms → デバイスが苦しんでいる。ステップ3へ。\n\
  HDDでレイテンシ > 100ms → 負荷下の回転ディスクでは想定内。\n\
\n\
ステップ3: df → ファイルシステム使用率を確認\n\
  > 95% → ほぼ満杯。緊急に空き容量を確保。速度低下の原因。\n\
  > 90% → 逼迫中。クリーンアップまたは拡張を計画。\n\
  < 90% → 容量は問題ではない。ステップ4へ。\n\
\n\
ステップ4: mounts → マウントオプションとファイルシステムタイプを確認\n\
  noatime がない → noatime を追加して書き込み増幅を削減。\n\
  ext4で data=journal → 書き込みスループット向上のため data=ordered を検討。\n\
  HDD使用中 → レイテンシ重視のワークロードにはSSDへの移行を検討。\n\
\n\
ステップ5: diskstats → 読み取りと書き込みの比率を確認\n\
  読み取りが優勢 → ページキャッシュ（meminfo の Cached）が小さすぎないか確認。\n\
  書き込みが優勢 → 書き込み負荷の高いアプリやログ出力を確認。",

            common_issues: "\
ディスク満杯: df が使用率100%または利用可能0を表示。アプリケーションが\
「No space left on device」で失敗。du で大きなファイルを探し、プロセスが\
保持している削除済みファイルを確認（lsof）、ログをクリーンアップ。ext4は\
5%をrootに予約していることを忘れずに — 必要に応じて tune2fs -m で調整。\n\
\n\
I/O飽和: diskstats の io_in_progress が常に > 0、pressure の \
io_some_avg10 > 0。デバイスが需要に追いつけない。対処法：より高速な\
ストレージへのアップグレード（HDDからSSD）、複数デバイスへのI/O分散\
（RAIDまたは別マウントポイント）、またはI/O需要の削減（ページキャッシュ\
改善のためRAM追加、書き込みのバッチ処理）。\n\
\n\
ファイルシステム破損: 予期しない再起動やハードウェア障害により\
ファイルシステムが不整合な状態になることがあります。症状：dmesgでの\
I/Oエラー、ファイルやディレクトリへのアクセス不能。アンマウントした\
ファイルシステムでfsckを実行。重要なデータには常にジャーナリング\
ファイルシステム（ext4、xfs）を使用 — ほとんどのクラッシュから\
自動的に回復します。\n\
\n\
遅い書き込み（書き込み増幅）: 書き込みが予想より遅い。一般的な原因：\
noatime マウントオプションの欠如（すべての読み取りが書き込みを誘発）、\
ジャーナリングのオーバーヘッド（data=journal が書き込み量を倍増）、\
ファイルシステムがほぼ満杯（断片化）、またはSSDの寿命末期（smartctl で\
SMART属性を確認）。",
        },

        // =====================================================================
        // PROCESS — English
        // =====================================================================
        (Category::Process, Locale::En) => CategoryContent {
            overview: "\
Every program running on Linux is a process. Processes have states \
(Running, Sleeping, Zombie, Stopped), consume resources (CPU, memory, \
file descriptors), and are organized in a parent-child tree rooted at \
PID 1. The kernel tracks process creation (fork), termination, context \
switches, and resource limits. Understanding the process lifecycle and \
common failure modes is essential for diagnosing system overload, \
resource leaks, and application failures.",

            story: "\
\"Too many processes\" — The Process Story\n\
\n\
Start at processes (the /proc-level summary). The total number of \
running processes and threads tells you system-wide concurrency. \
Compare to ulimits and system maximums — if the count is near the \
kernel.pid_max limit, new processes cannot be created and fork() \
calls fail with EAGAIN.\n\
\n\
Next, check stat for system-wide counters. processes_forks shows the \
cumulative number of fork() calls since boot — if this is increasing \
rapidly (thousands per second), something is spawning processes at an \
alarming rate. context_switches counts how often the CPU switches \
between tasks. High context switches (hundreds of thousands per second) \
indicate too many competing threads or very short-lived processes.\n\
\n\
Then check file-nr (file descriptor usage). The three numbers show: \
allocated file handles, free file handles (usually 0 on modern \
kernels), and the system maximum. If allocated approaches the maximum, \
processes will fail to open files, sockets, or pipes with \"Too many \
open files.\" Each leaked file descriptor brings the system closer to \
this limit.\n\
\n\
Look for zombie processes (state Z in the process table). Zombies are \
finished processes whose parent has not called wait() to collect their \
exit status. A few zombies are harmless, but hundreds indicate a \
buggy parent process. Zombies consume a PID slot and a small amount \
of kernel memory — in extreme cases they can exhaust the PID space.",

            diagnostic_flow: "\
Step 1: Check processes → total count\n\
  Compare to kernel.pid_max (default 32768 or 4194304).\n\
  If near limit → PID exhaustion imminent. Continue to Step 2.\n\
  If well below → Process count is OK. Skip to Step 3.\n\
\n\
Step 2: Check stat → processes_forks rate\n\
  If forks/sec > 1000 → Fork storm or runaway process spawning.\n\
  Identify the parent process creating children rapidly.\n\
  If normal → Process count grew slowly. Check for zombies.\n\
\n\
Step 3: Check processes → zombie count\n\
  If zombies > 10 → Parent process not reaping children.\n\
  Identify the parent (PPID) and fix or restart it.\n\
  If zombies = 0 → No zombie issue. Continue to Step 4.\n\
\n\
Step 4: Check file-nr → allocated vs maximum\n\
  If allocated > 80% of max → File descriptor pressure.\n\
  Find processes with many open FDs (ls /proc/PID/fd | wc -l).\n\
  If normal → FD usage is healthy. Continue to Step 5.\n\
\n\
Step 5: Check stat → context_switches rate\n\
  If > 500k/sec with degraded performance → Too many threads.\n\
  Reduce concurrency or increase CPU resources.",

            common_issues: "\
Fork Bombs: A process recursively spawns copies of itself, consuming \
all available PIDs within seconds. stat shows processes_forks exploding. \
The system becomes unresponsive as the kernel spends all time creating \
and scheduling processes. Prevention: set ulimit -u (max user processes) \
to a reasonable value. Recovery: if you can get a shell, kill the parent \
process tree.\n\
\n\
Zombie Accumulation: Hundreds of zombie (Z state) processes appear. \
The parent process is not calling wait()/waitpid() to reap child \
exit statuses. Zombies do not consume CPU or memory but do consume PID \
slots. Fix the parent application. As a workaround, killing the parent \
causes zombies to be reparented to init, which reaps them.\n\
\n\
File Descriptor Leaks: file-nr allocated count grows over time. An \
application opens files, sockets, or pipes but never closes them. \
Eventually the process hits its per-process limit (ulimit -n, default \
1024) or the system hits the global limit (file-nr max). Symptoms: \
\"Too many open files\" errors. Find the leaking process and fix the \
application code.\n\
\n\
OOM Kills: When the system runs out of memory, the OOM killer selects \
processes to terminate based on their memory usage (oom_score). Check \
dmesg for \"Out of memory: Killed process\" messages. The killed process \
may restart and get killed again in a loop. Fix by reducing memory \
usage, adding RAM, or tuning oom_score_adj to protect critical processes.",
        },

        // =====================================================================
        // PROCESS — Japanese
        // =====================================================================
        (Category::Process, Locale::Ja) => CategoryContent {
            overview: "\
Linuxで実行されるすべてのプログラムはプロセスです。プロセスには状態\
（実行中、スリープ、ゾンビ、停止）があり、リソース（CPU、メモリ、\
ファイルディスクリプタ）を消費し、PID 1を根とする親子ツリーに\
組織化されています。カーネルはプロセスの生成（fork）、終了、\
コンテキストスイッチ、リソース制限を追跡します。プロセスのライフ\
サイクルと一般的な障害モードを理解することは、システム過負荷、\
リソースリーク、アプリケーション障害の診断に不可欠です。",

            story: "\
「プロセスが多すぎる」 — プロセスのストーリー\n\
\n\
まず processes（/procレベルの要約）を確認します。実行中のプロセスと\
スレッドの総数がシステム全体の並行性を示します。ulimits やシステムの\
最大値と比較 — カウントが kernel.pid_max の上限に近い場合、新しい\
プロセスを作成できず、fork() 呼び出しが EAGAIN で失敗します。\n\
\n\
次に stat でシステム全体のカウンターを確認します。processes_forks は\
ブート以降の累計 fork() 呼び出し数を示します — これが急速に増加\
（毎秒数千回）している場合、何かが驚くべき速度でプロセスを生成して\
います。context_switches はCPUがタスク間で切り替わった回数です。\
高いコンテキストスイッチ（毎秒数十万回）は、競合するスレッドが多すぎるか、\
非常に短命なプロセスを示します。\n\
\n\
次に file-nr（ファイルディスクリプタの使用状況）を確認します。3つの\
数値は：割り当て済みファイルハンドル数、空きファイルハンドル数（現代の\
カーネルでは通常0）、システム最大値です。割り当て済みが最大値に近づくと、\
プロセスはファイル、ソケット、パイプを開けなくなり「Too many open files」\
で失敗します。リークされたファイルディスクリプタの一つ一つがシステムを\
この上限に近づけます。\n\
\n\
ゾンビプロセス（プロセステーブルで状態Z）を探します。ゾンビは終了した\
プロセスで、親が wait() を呼んで終了ステータスを回収していないものです。\
少数のゾンビは無害ですが、数百のゾンビはバグのある親プロセスを示します。\
ゾンビはPIDスロットと少量のカーネルメモリを消費します — 極端な場合、\
PID空間を枯渇させることがあります。",

            diagnostic_flow: "\
ステップ1: processes → 総数を確認\n\
  kernel.pid_max（デフォルト 32768 または 4194304）と比較。\n\
  上限に近い → PID枯渇が差し迫っている。ステップ2へ。\n\
  十分下回る → プロセス数は問題なし。ステップ3へスキップ。\n\
\n\
ステップ2: stat → processes_forks のレートを確認\n\
  forks/秒 > 1000 → フォークストームまたは暴走プロセス生成。\n\
  子プロセスを急速に生成している親プロセスを特定。\n\
  正常 → プロセス数はゆっくり増加。ゾンビを確認。\n\
\n\
ステップ3: processes → ゾンビ数を確認\n\
  ゾンビ > 10 → 親プロセスが子を回収していない。\n\
  親（PPID）を特定し、修正または再起動。\n\
  ゾンビ = 0 → ゾンビの問題なし。ステップ4へ。\n\
\n\
ステップ4: file-nr → 割り当て済み vs 最大値を確認\n\
  割り当て済み > 最大値の80% → ファイルディスクリプタ圧力。\n\
  多くのFDを開いているプロセスを特定（ls /proc/PID/fd | wc -l）。\n\
  正常 → FD使用量は健全。ステップ5へ。\n\
\n\
ステップ5: stat → context_switches のレートを確認\n\
  > 50万/秒で性能低下 → スレッドが多すぎる。\n\
  並行度の削減またはCPUリソースの追加。",

            common_issues: "\
フォーク爆弾: プロセスが再帰的に自身のコピーを生成し、数秒で利用可能な\
全PIDを消費。stat の processes_forks が爆発的に増加。カーネルがプロセスの\
作成とスケジューリングに全時間を費やし、システムが応答不能になる。\
予防策：ulimit -u（ユーザーあたりの最大プロセス数）を適切な値に設定。\
回復：シェルが取得できれば、親プロセスツリーをkill。\n\
\n\
ゾンビの蓄積: 数百のゾンビ（Z状態）プロセスが出現。親プロセスが \
wait()/waitpid() を呼んで子の終了ステータスを回収していない。ゾンビは\
CPUやメモリは消費しないがPIDスロットを消費。親アプリケーションを修正。\
回避策として親をkillすると、ゾンビはinitに再ペアレントされ回収される。\n\
\n\
ファイルディスクリプタリーク: file-nr の割り当て済み数が時間とともに増加。\
アプリケーションがファイル、ソケット、パイプを開くが閉じない。最終的に\
プロセスがプロセスごとの上限（ulimit -n、デフォルト1024）またはシステムが\
グローバル上限（file-nr max）に到達。症状：「Too many open files」エラー。\
リークしているプロセスを見つけてアプリケーションコードを修正。\n\
\n\
OOM Kill: システムがメモリ不足になると、OOM killerがメモリ使用量\
（oom_score）に基づいてプロセスを選択して終了。dmesg で「Out of memory: \
Killed process」メッセージを確認。killされたプロセスが再起動してまた\
killされるループに陥ることがある。対処法：メモリ使用量の削減、RAMの追加、\
または oom_score_adj の調整で重要なプロセスを保護。",
        },

        // =====================================================================
        // HARDWARE — English
        // =====================================================================
        (Category::Hardware, Locale::En) => CategoryContent {
            overview: "\
Hardware monitoring covers CPU temperature, frequency scaling, and \
thermal throttling. Modern CPUs dynamically adjust their clock frequency \
based on workload demand and thermal conditions. When temperatures \
exceed safe thresholds, the kernel throttles CPU frequency to prevent \
damage — trading performance for safety. Monitoring thermal zones and \
CPU frequency together reveals whether your system is performing \
optimally or silently degraded by heat.",

            story: "\
\"Why is my server throttling?\" — The Hardware Story\n\
\n\
Start at thermal zone data. Each thermal zone reports a temperature \
in millidegrees Celsius (divide by 1000 for human-readable values). \
Zone types include x86_pkg_temp (CPU package), acpitz (ACPI thermal), \
and others depending on hardware. Compare current temperatures to the \
trip points defined for each zone: passive trip points trigger frequency \
throttling, critical trip points trigger emergency shutdown.\n\
\n\
Next, check cpuinfo for current CPU frequency. Each core reports its \
clock speed in MHz. Compare to the maximum frequency the CPU supports. \
If the current frequency is well below maximum during a CPU-intensive \
workload, the CPU is being throttled. The frequency governor \
(performance, powersave, ondemand, schedutil) controls how aggressively \
the CPU scales frequency.\n\
\n\
The connection between thermal and frequency is direct: as temperature \
rises toward the passive trip point, the kernel reduces maximum allowed \
frequency. This creates a feedback loop — lower frequency means less \
heat generation, stabilizing temperature at the cost of throughput. \
On well-cooled systems this never triggers. On systems with inadequate \
cooling, it can silently cut performance by 50% or more.\n\
\n\
In virtual machines and containers, thermal data may not be available \
(the hypervisor manages hardware). If thermal zones are empty, \
performance issues are more likely caused by CPU steal time or \
resource limits than by thermal throttling.",

            diagnostic_flow: "\
Step 1: Check thermal zones → current temperature\n\
  If < 70C → Temperature is normal. Skip to Step 3.\n\
  If 70-85C → Warm but usually acceptable. Continue to Step 2.\n\
  If > 85C → Hot. Likely throttling. Continue to Step 2.\n\
\n\
Step 2: Check thermal zones → trip points\n\
  Compare current temp to passive trip point.\n\
  If current >= passive → CPU is being frequency-throttled.\n\
  If current >= critical → System at risk of emergency shutdown.\n\
\n\
Step 3: Check cpuinfo → current frequency vs max frequency\n\
  If current ~= max during load → No throttling. CPU is fine.\n\
  If current << max during load → Throttling active. Check thermals.\n\
  If governor = powersave → Frequency intentionally limited.\n\
\n\
Step 4: Investigate cooling\n\
  If physical server → Check fans, airflow, thermal paste, dust.\n\
  If VM → Thermal data may be unavailable; check steal time instead.\n\
  If laptop → Ensure vents are not blocked, consider a cooling pad.\n\
\n\
Step 5: Check for sustained throttling pattern\n\
  If throttling only under peak load → May be acceptable.\n\
  If throttling at idle → Cooling system is failing. Urgent action needed.",

            common_issues: "\
Overheating: Thermal zone temperature exceeds 85C consistently. CPU \
frequency drops well below maximum. Causes: failed or degraded fans, \
blocked airflow, dust accumulation in heatsinks, or ambient temperature \
too high (server room AC failure). Monitor temperature trends over time \
to distinguish transient spikes from sustained overheating.\n\
\n\
Broken or Degraded Fans: Temperature rises gradually over weeks or \
months as fans wear out or accumulate dust. Performance degrades \
silently as thermal throttling kicks in. Regular physical inspection \
and cleaning prevents this. For servers, IPMI/BMC fan speed sensors \
provide early warning.\n\
\n\
Thermal Paste Degradation: Over 3-5 years, thermal paste between CPU \
and heatsink dries out, increasing thermal resistance. Symptoms: higher \
temperatures than when the system was new, despite same workload and \
ambient conditions. Solution: clean and reapply quality thermal paste.\n\
\n\
Frequency Governor Misconfiguration: CPU is stuck at low frequency \
even when temperature is cool. The governor is set to \"powersave\" \
instead of \"ondemand\" or \"schedutil.\" Check \
/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor. For servers, \
\"performance\" governor gives consistent throughput at the cost of \
higher power consumption.",
        },

        // =====================================================================
        // HARDWARE — Japanese
        // =====================================================================
        (Category::Hardware, Locale::Ja) => CategoryContent {
            overview: "\
ハードウェアモニタリングは、CPU温度、周波数スケーリング、サーマル\
スロットリングをカバーします。現代のCPUはワークロードの需要と熱条件に\
基づいてクロック周波数を動的に調整します。温度が安全な閾値を超えると、\
カーネルは損傷を防ぐためにCPU周波数を抑制します — 性能を犠牲にして\
安全性を確保します。サーマルゾーンとCPU周波数を合わせて監視することで、\
システムが最適に動作しているか、熱によって静かに性能低下しているかが\
明らかになります。",

            story: "\
「なぜサーバーがスロットリングしている？」 — ハードウェアのストーリー\n\
\n\
まずサーマルゾーンデータを確認します。各サーマルゾーンはミリ度セルシウスで\
温度を報告します（人間が読める値にするには1000で割る）。ゾーンタイプには \
x86_pkg_temp（CPUパッケージ）、acpitz（ACPIサーマル）などがあり、\
ハードウェアに依存します。現在の温度を各ゾーンに定義されたトリップポイントと\
比較：パッシブトリップポイントは周波数スロットリングを発動し、クリティカル\
トリップポイントは緊急シャットダウンを発動します。\n\
\n\
次に cpuinfo で現在のCPU周波数を確認します。各コアがクロック速度をMHzで\
報告します。CPUがサポートする最大周波数と比較してください。CPU集約型の\
ワークロード中に現在の周波数が最大値を大きく下回っていれば、CPUは\
スロットリングされています。周波数ガバナー（performance、powersave、\
ondemand、schedutil）がCPUの周波数スケーリングの積極性を制御します。\n\
\n\
サーマルと周波数の関係は直接的です：温度がパッシブトリップポイントに\
向かって上昇すると、カーネルは許容最大周波数を下げます。これが\
フィードバックループを作ります — 低い周波数は発熱を減らし、スループットの\
犠牲で温度を安定させます。冷却が十分なシステムではこれは発動しません。\
冷却が不十分なシステムでは、性能が静かに50%以上低下することがあります。\n\
\n\
仮想マシンやコンテナでは、サーマルデータが利用できない場合があります\
（ハイパーバイザがハードウェアを管理）。サーマルゾーンが空の場合、性能の\
問題はサーマルスロットリングよりCPUスチールタイムやリソース制限が原因の\
可能性が高いです。",

            diagnostic_flow: "\
ステップ1: サーマルゾーン → 現在の温度を確認\n\
  < 70C → 温度は正常。ステップ3へスキップ。\n\
  70-85C → 暖かいが通常は許容範囲。ステップ2へ。\n\
  > 85C → 高温。スロットリングの可能性大。ステップ2へ。\n\
\n\
ステップ2: サーマルゾーン → トリップポイントを確認\n\
  現在の温度をパッシブトリップポイントと比較。\n\
  現在 >= パッシブ → CPUが周波数スロットリング中。\n\
  現在 >= クリティカル → システムが緊急シャットダウンの危険あり。\n\
\n\
ステップ3: cpuinfo → 現在の周波数 vs 最大周波数を確認\n\
  負荷時に現在 ~= 最大 → スロットリングなし。CPUは正常。\n\
  負荷時に現在 << 最大 → スロットリングが有効。サーマルを確認。\n\
  ガバナー = powersave → 意図的に周波数を制限中。\n\
\n\
ステップ4: 冷却を調査\n\
  物理サーバー → ファン、エアフロー、サーマルペースト、ほこりを確認。\n\
  VM → サーマルデータが利用できない場合あり。スチールタイムを確認。\n\
  ノートPC → 通気口がふさがっていないか確認、冷却パッドを検討。\n\
\n\
ステップ5: 持続的なスロットリングパターンを確認\n\
  ピーク負荷時のみスロットリング → 許容できる場合あり。\n\
  アイドル時にスロットリング → 冷却システムが故障中。緊急対応が必要。",

            common_issues: "\
オーバーヒート: サーマルゾーンの温度が常に85Cを超過。CPU周波数が最大値を\
大きく下回る。原因：故障または劣化したファン、ふさがったエアフロー、\
ヒートシンクのほこり蓄積、または周囲温度が高すぎる（サーバールームの\
空調故障）。一時的なスパイクと持続的なオーバーヒートを区別するため、\
温度推移を経時的に監視。\n\
\n\
故障または劣化したファン: ファンが摩耗したりほこりが溜まると、数週間から\
数ヶ月かけて温度が徐々に上昇。サーマルスロットリングが作動し、性能が\
静かに低下。定期的な物理的点検と清掃で予防可能。サーバーでは \
IPMI/BMC のファン速度センサーが早期警告を提供。\n\
\n\
サーマルペーストの劣化: 3-5年でCPUとヒートシンク間のサーマルペーストが\
乾燥し、熱抵抗が増加。症状：同じワークロードと環境条件にもかかわらず、\
新品時より高い温度。対処法：品質の良いサーマルペーストで清掃・再塗布。\n\
\n\
周波数ガバナーの設定ミス: 温度が低いにもかかわらずCPUが低周波数に固定。\
ガバナーが「ondemand」や「schedutil」ではなく「powersave」に設定されている。\
/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor を確認。サーバー\
では「performance」ガバナーが電力消費の増加と引き換えに一貫したスループット\
を提供。",
        },
    }
}

/// Learning Breadcrumbs: "What to learn next" for a given field.
/// Returns a list of (source, field, reason) tuples ordered by learning progression.
/// Only defined for key fields (SEE ALSO 31 fields base).
/// Shown only at EXTRA help level.
pub fn breadcrumbs(locale: Locale, source: &str, field: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match locale {
        Locale::En => breadcrumbs_en(source, field),
        Locale::Ja => breadcrumbs_ja(source, field),
    }
}

fn breadcrumbs_en(source: &str, field: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match (source, field) {
        // Memory learning path: MemAvailable → MemFree → Cached → Slab → vmstat dirty pages
        ("meminfo", "MemAvailable") => vec![
            ("meminfo", "MemFree", "Learn why MemAvailable != MemFree"),
            ("meminfo", "SReclaimable", "Understand reclaimable kernel cache"),
            ("vmstat", "nr_dirty", "Explore dirty page write pressure"),
        ],
        ("meminfo", "MemFree") => vec![
            ("meminfo", "Cached", "See what makes MemAvailable > MemFree"),
            ("meminfo", "Buffers", "Buffer cache vs page cache"),
            ("meminfo", "SReclaimable", "Kernel slab memory that can be freed"),
        ],
        ("meminfo", "Cached") => vec![
            ("meminfo", "SReclaimable", "Kernel cache vs page cache"),
            ("vmstat", "pgfault", "How cache misses cause page faults"),
            ("meminfo", "Dirty", "Cached pages waiting to be written"),
        ],
        ("meminfo", "SwapFree") => vec![
            ("vmstat", "pswpin", "Swap-in rate shows active swap pressure"),
            ("vmstat", "pswpout", "Swap-out rate shows memory exhaustion"),
            ("pressure", "memory_some_avg10", "PSI: are tasks stalled on memory?"),
        ],
        ("meminfo", "Dirty") => vec![
            ("vmstat", "nr_dirty", "Kernel-level dirty page tracking"),
            ("vmstat", "nr_writeback", "Pages being written to disk right now"),
            ("diskstats", "write_ios_total", "Resulting disk write operations"),
        ],
        // CPU learning path: load → stat → pressure → scheduling
        ("loadavg", "load_1min") => vec![
            ("stat", "procs_running", "Actual runnable processes right now"),
            ("cpuinfo", "cpu_count", "Compare load to core count"),
            ("pressure", "cpu_some_avg10", "PSI: are tasks stalled on CPU?"),
        ],
        ("stat", "procs_running") => vec![
            ("loadavg", "load_1min", "Smoothed average over 1 minute"),
            ("stat", "procs_blocked", "Processes blocked on I/O"),
            ("schedstat", "total_run_time", "Scheduler-level run time stats"),
        ],
        ("stat", "procs_blocked") => vec![
            ("pressure", "io_some_avg10", "PSI: are tasks stalled on I/O?"),
            ("diskstats", "io_in_progress", "Pending disk I/O operations"),
            ("stat", "procs_running", "Compare blocked vs running ratio"),
        ],
        // Network learning path: bytes → errors → connections
        ("net_dev", "rx_bytes") => vec![
            ("net_dev", "rx_packets", "Packet count gives you packet size"),
            ("net_dev", "rx_errors", "Errors indicate hardware/driver issues"),
            ("net_snmp", "InSegs", "TCP segment count for protocol-level view"),
        ],
        ("net_dev", "tx_bytes") => vec![
            ("net_dev", "tx_packets", "Packet count for average packet size"),
            ("net_dev", "tx_errors", "Transmission errors"),
            ("net_tcp", "established", "Active TCP connections generating traffic"),
        ],
        ("net_tcp", "established") => vec![
            ("net_tcp", "time_wait", "Connections closing: high count = short-lived conns"),
            ("net_tcp", "listen", "Listening ports: your exposed services"),
            ("net_sockstat", "TCP_inuse", "Total TCP socket usage"),
        ],
        // Storage learning path: diskstats → pressure → cache
        ("diskstats", "read_ios_total") => vec![
            ("diskstats", "read_time_ms", "Time spent reading: latency indicator"),
            ("meminfo", "Cached", "File cache reduces physical reads"),
            ("pressure", "io_some_avg10", "PSI: overall I/O pressure"),
        ],
        // Pressure learning path
        ("pressure", "memory_some_avg10") => vec![
            ("meminfo", "MemAvailable", "Check actual memory availability"),
            ("pressure", "memory_full_avg10", "Full stall: ALL tasks waiting"),
            ("vmstat", "pgfault", "Page faults indicate cache misses"),
        ],
        ("pressure", "cpu_some_avg10") => vec![
            ("loadavg", "load_1min", "Load average for longer-term trend"),
            ("stat", "procs_running", "Actual runnable processes"),
            ("pressure", "io_some_avg10", "I/O pressure often accompanies CPU"),
        ],
        ("pressure", "io_some_avg10") => vec![
            ("stat", "procs_blocked", "Blocked processes confirm I/O stalls"),
            ("diskstats", "io_in_progress", "Active disk I/O operations"),
            ("pressure", "io_full_avg10", "Full stall: ALL tasks waiting on I/O"),
        ],
        _ => vec![],
    }
}

fn breadcrumbs_ja(source: &str, field: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match (source, field) {
        ("meminfo", "MemAvailable") => vec![
            ("meminfo", "MemFree", "MemAvailable と MemFree の違いを理解する"),
            ("meminfo", "SReclaimable", "回収可能なカーネルキャッシュを学ぶ"),
            ("vmstat", "nr_dirty", "dirty page による書き込み圧力を調べる"),
        ],
        ("meminfo", "MemFree") => vec![
            ("meminfo", "Cached", "MemAvailable > MemFree になる理由を知る"),
            ("meminfo", "Buffers", "バッファキャッシュとページキャッシュの違い"),
            ("meminfo", "SReclaimable", "解放可能なカーネル slab メモリ"),
        ],
        ("meminfo", "Cached") => vec![
            ("meminfo", "SReclaimable", "カーネルキャッシュとページキャッシュの関係"),
            ("vmstat", "pgfault", "キャッシュミスがページフォルトを引き起こす仕組み"),
            ("meminfo", "Dirty", "書き込み待ちのキャッシュページ"),
        ],
        ("meminfo", "SwapFree") => vec![
            ("vmstat", "pswpin", "スワップイン率で実際の圧力を確認"),
            ("vmstat", "pswpout", "スワップアウト率はメモリ枯渇のサイン"),
            ("pressure", "memory_some_avg10", "PSI: タスクがメモリ待ちか？"),
        ],
        ("meminfo", "Dirty") => vec![
            ("vmstat", "nr_dirty", "カーネルレベルの dirty ページ追跡"),
            ("vmstat", "nr_writeback", "今まさにディスクに書き込み中のページ"),
            ("diskstats", "write_ios_total", "結果としてのディスク書き込み操作"),
        ],
        ("loadavg", "load_1min") => vec![
            ("stat", "procs_running", "現在実行可能なプロセス数"),
            ("cpuinfo", "cpu_count", "load をコア数と比較する"),
            ("pressure", "cpu_some_avg10", "PSI: タスクが CPU 待ちか？"),
        ],
        ("stat", "procs_running") => vec![
            ("loadavg", "load_1min", "1分間の平滑化平均"),
            ("stat", "procs_blocked", "I/O 待ちでブロック中のプロセス"),
            ("schedstat", "total_run_time", "スケジューラレベルの実行時間統計"),
        ],
        ("stat", "procs_blocked") => vec![
            ("pressure", "io_some_avg10", "PSI: タスクが I/O 待ちか？"),
            ("diskstats", "io_in_progress", "保留中のディスク I/O 操作"),
            ("stat", "procs_running", "ブロック vs 実行の比率を比較"),
        ],
        ("net_dev", "rx_bytes") => vec![
            ("net_dev", "rx_packets", "パケット数からパケットサイズがわかる"),
            ("net_dev", "rx_errors", "エラーはハードウェア/ドライバの問題を示す"),
            ("net_snmp", "InSegs", "TCP セグメント数でプロトコルレベルの状況を見る"),
        ],
        ("net_dev", "tx_bytes") => vec![
            ("net_dev", "tx_packets", "パケット数から平均パケットサイズを算出"),
            ("net_dev", "tx_errors", "送信エラー"),
            ("net_tcp", "established", "トラフィックを生成しているTCP接続"),
        ],
        ("net_tcp", "established") => vec![
            ("net_tcp", "time_wait", "クローズ中の接続: 多い=短命な接続"),
            ("net_tcp", "listen", "リッスンポート: 公開中のサービス"),
            ("net_sockstat", "TCP_inuse", "TCP ソケットの総使用数"),
        ],
        ("diskstats", "read_ios_total") => vec![
            ("diskstats", "read_time_ms", "読み取り時間: レイテンシ指標"),
            ("meminfo", "Cached", "ファイルキャッシュが物理読み取りを削減"),
            ("pressure", "io_some_avg10", "PSI: 全体的な I/O 圧力"),
        ],
        ("pressure", "memory_some_avg10") => vec![
            ("meminfo", "MemAvailable", "実際のメモリ空き状況を確認"),
            ("pressure", "memory_full_avg10", "完全停止: 全タスクがメモリ待ち"),
            ("vmstat", "pgfault", "ページフォルトはキャッシュミスを示す"),
        ],
        ("pressure", "cpu_some_avg10") => vec![
            ("loadavg", "load_1min", "長期トレンドの load average"),
            ("stat", "procs_running", "実行可能プロセス数"),
            ("pressure", "io_some_avg10", "CPU 圧力には I/O 圧力が伴うことが多い"),
        ],
        ("pressure", "io_some_avg10") => vec![
            ("stat", "procs_blocked", "ブロックされたプロセスが I/O 停止を確認"),
            ("diskstats", "io_in_progress", "進行中のディスク I/O 操作"),
            ("pressure", "io_full_avg10", "完全停止: 全タスクが I/O 待ち"),
        ],
        _ => vec![],
    }
}

/// Generate a "Did you know?" tip using live system data.
/// Returns a localized tip string that incorporates actual values from the snapshot.
pub fn generate_tip(snapshot: &crate::proc::Snapshot, locale: Locale) -> String {
    use crate::proc::FieldValue;

    // Helper to get a numeric field value from snapshot
    let get_val = |source: &str, field: &str| -> Option<f64> {
        snapshot.entries.get(source).and_then(|e| {
            e.fields.iter().find(|f| f.name == field).and_then(|f| match &f.value {
                FieldValue::Integer(n) => Some(*n as f64),
                FieldValue::Float(n) => Some(*n),
                _ => None,
            })
        })
    };

    let get_str = |source: &str, field: &str| -> Option<String> {
        snapshot.entries.get(source).and_then(|e| {
            e.fields.iter().find(|f| f.name == field).and_then(|f| match &f.value {
                FieldValue::Text(s) => Some(s.clone()),
                _ => None,
            })
        })
    };

    // Build tips dynamically based on available data
    let mut tips: Vec<String> = Vec::new();

    // Memory tips
    if let (Some(total), Some(avail)) = (get_val("meminfo", "MemTotal"), get_val("meminfo", "MemAvailable")) {
        let pct = (avail / total * 100.0) as u64;
        let total_gb = total / 1024.0 / 1024.0;
        if locale == Locale::Ja {
            tips.push(format!(
                "MemAvailable ({:.1}GB, {}%) は実際に使える空きメモリです。MemFree とは違い、\
                カーネルキャッシュの回収可能分も含みます。20% 以下になるとパフォーマンスに影響が出始めます。",
                avail / 1024.0 / 1024.0, pct
            ));
            tips.push(format!(
                "このマシンの総メモリは {:.1}GB。MemTotal はカーネルが予約した分を除いた値なので、\
                物理メモリより少し小さくなります。",
                total_gb
            ));
        } else {
            tips.push(format!(
                "MemAvailable ({:.1}GB, {}%) is the actual usable free memory. Unlike MemFree, \
                it includes reclaimable kernel caches. Below 20% may impact performance.",
                avail / 1024.0 / 1024.0, pct
            ));
            tips.push(format!(
                "This machine has {:.1}GB total memory. MemTotal is slightly less than physical RAM \
                because the kernel reserves some memory before reporting.",
                total_gb
            ));
        }
    }

    if let (Some(swap_total), Some(swap_free)) = (get_val("meminfo", "SwapTotal"), get_val("meminfo", "SwapFree")) {
        if swap_total > 0.0 {
            let swap_used_pct = ((swap_total - swap_free) / swap_total * 100.0) as u64;
            if locale == Locale::Ja {
                tips.push(format!(
                    "スワップ使用率は {}%。スワップが使われること自体は問題ではありませんが、\
                    頻繁なスワップイン/アウト (vmstat の pswpin/pswpout) はパフォーマンス低下のサインです。",
                    swap_used_pct
                ));
            } else {
                tips.push(format!(
                    "Swap usage is at {}%. Swap use alone isn't a problem, but frequent swap in/out \
                    (vmstat pswpin/pswpout) signals performance degradation.",
                    swap_used_pct
                ));
            }
        }
    }

    // CPU / Load tips
    if let (Some(load1), Some(num_cpus)) = (get_val("loadavg", "load_1min"), get_val("cpuinfo", "cpu_count")) {
        let threshold = num_cpus;
        if locale == Locale::Ja {
            tips.push(format!(
                "load average ({:.2}) が CPU コア数 ({}) を超えると、プロセスが CPU 待ちに\
                なっていることを意味します。現在の比率: {:.0}%。",
                load1, num_cpus as u64, load1 / threshold * 100.0
            ));
        } else {
            tips.push(format!(
                "When load average ({:.2}) exceeds CPU core count ({}), processes are waiting for CPU. \
                Current ratio: {:.0}%.",
                load1, num_cpus as u64, load1 / threshold * 100.0
            ));
        }
    }

    if let Some(procs_running) = get_val("stat", "procs_running") {
        if locale == Locale::Ja {
            tips.push(format!(
                "現在 {} 個のプロセスが実行中 (procs_running)。この数が CPU コア数を\
                大きく超えている場合、CPU バウンドな負荷がかかっています。",
                procs_running as u64
            ));
        } else {
            tips.push(format!(
                "Currently {} processes are running (procs_running). When this significantly exceeds \
                CPU core count, the system is CPU-bound.",
                procs_running as u64
            ));
        }
    }

    if let Some(ctxt) = get_val("stat", "ctxt") {
        if locale == Locale::Ja {
            tips.push(format!(
                "コンテキストスイッチ回数 (ctxt) は累計 {:.0} 回。1秒あたりの変化量 (差分ビュー d キー) \
                が数万回/秒なら正常、数十万回/秒超なら過負荷の可能性。",
                ctxt
            ));
        } else {
            tips.push(format!(
                "Context switches (ctxt) total: {:.0}. Use diff view (d key) to see per-second rate. \
                Tens of thousands/sec is normal; hundreds of thousands may indicate overload.",
                ctxt
            ));
        }
    }

    // Network tips
    if let Some(rx) = get_val("net_dev", "rx_bytes_total") {
        let rx_gb = rx / 1024.0 / 1024.0 / 1024.0;
        if locale == Locale::Ja {
            tips.push(format!(
                "起動以来の総受信データ量: {:.2}GB。差分ビュー (d キー) で1秒あたりの\
                受信速度を確認できます。net_dev の rx_bytes と tx_bytes を比較すると\
                トラフィックの方向性がわかります。",
                rx_gb
            ));
        } else {
            tips.push(format!(
                "Total received data since boot: {:.2}GB. Use diff view (d key) for per-second rates. \
                Compare rx_bytes vs tx_bytes in net_dev to understand traffic direction.",
                rx_gb
            ));
        }
    }

    // Storage tips
    if let Some(reads) = get_val("diskstats", "read_ios_total") {
        if locale == Locale::Ja {
            tips.push(format!(
                "ディスク読み取り I/O 操作回数: {:.0}。読み取りが多い場合は\
                ページキャッシュ (meminfo の Cached) を確認 — キャッシュが\
                効いていれば物理 I/O を減らせます。",
                reads
            ));
        } else {
            tips.push(format!(
                "Disk read I/O operations: {:.0}. If reads are high, check page cache \
                (meminfo Cached) — effective caching reduces physical I/O.",
                reads
            ));
        }
    }

    // Process tips
    if let Some(procs_blocked) = get_val("stat", "procs_blocked") {
        if procs_blocked > 0.0 {
            if locale == Locale::Ja {
                tips.push(format!(
                    "現在 {} 個のプロセスが I/O 待ちでブロック中 (procs_blocked)。\
                    ディスクやネットワーク I/O のボトルネックを示唆しています。\
                    diskstats や pressure の io を確認してみてください。",
                    procs_blocked as u64
                ));
            } else {
                tips.push(format!(
                    "{} process(es) blocked on I/O (procs_blocked). This suggests disk or network \
                    I/O bottleneck. Check diskstats and pressure io metrics.",
                    procs_blocked as u64
                ));
            }
        }
    }

    // Uptime tip
    if let Some(uptime) = get_val("uptime", "uptime_seconds") {
        let days = (uptime / 86400.0) as u64;
        if locale == Locale::Ja {
            tips.push(format!(
                "このマシンは {} 日間稼働中。長期稼働ではメモリリークや\
                ファイルディスクリプタ枯渇に注意。/proc/slabinfo で\
                カーネルメモリの増加傾向を確認できます。",
                days
            ));
        } else {
            tips.push(format!(
                "This machine has been up for {} days. For long-running systems, watch for memory leaks \
                and file descriptor exhaustion. Check /proc/slabinfo for kernel memory growth trends.",
                days
            ));
        }
    }

    // Kernel version tip
    if let Some(version) = get_str("version", "version_string") {
        if locale == Locale::Ja {
            tips.push(format!(
                "カーネル: {}。カーネルバージョンによって利用可能なメトリクスが\
                変わります。例えば PSI (Pressure Stall Information) は Linux 4.20+ で導入。",
                version.split_whitespace().take(3).collect::<Vec<_>>().join(" ")
            ));
        } else {
            tips.push(format!(
                "Kernel: {}. Available metrics depend on kernel version. For example, \
                PSI (Pressure Stall Information) was introduced in Linux 4.20+.",
                version.split_whitespace().take(3).collect::<Vec<_>>().join(" ")
            ));
        }
    }

    // General syslenz tips (static but useful)
    if locale == Locale::Ja {
        tips.push("? キーでヘルプレベルを切り替え: OFF → NORMAL → DETAILED → EXTRA。\
            EXTRA では SEE ALSO リンクやコンテキストヒントが表示されます。".into());
        tips.push("d キーで差分ビューに切り替え。カウンタ値の1秒あたりの変化量が\
            一目でわかります。ネットワーク速度やディスク I/O の確認に便利。".into());
        tips.push("X キーで診断ビュー。システムの問題を自動検出して対処法を提案します。\
            Enter キーで関連メトリクスに直接ジャンプ可能。".into());
        tips.push("C キーでカテゴリガイド。メモリ、CPU、ネットワーク等のカテゴリ別に\
            関連ソースと学習コンテンツをまとめて表示します。".into());
    } else {
        tips.push("Press ? to cycle help levels: OFF → NORMAL → DETAILED → EXTRA. \
            EXTRA shows SEE ALSO links and contextual hints.".into());
        tips.push("Press d for diff view. See per-second changes for counter values at a glance. \
            Great for checking network speed and disk I/O rates.".into());
        tips.push("Press X for diagnostics view. Auto-detects system issues and suggests fixes. \
            Press Enter to jump directly to related metrics.".into());
        tips.push("Press C for category guide. View related sources and learning content \
            organized by category: Memory, CPU, Network, and more.".into());
    }

    // Select one tip based on a simple hash of timestamp
    if tips.is_empty() {
        return String::new();
    }
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Change tip every 10 minutes
    let idx = (seed / 600) as usize % tips.len();
    tips.into_iter().nth(idx).unwrap_or_default()
}
