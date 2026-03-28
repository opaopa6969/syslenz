use crate::i18n::Locale;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Category {
    Memory,
    CpuLoad,
    Network,
    Storage,
    Process,
}

impl Category {
    pub fn all() -> &'static [Category] {
        &[
            Category::Memory,
            Category::CpuLoad,
            Category::Network,
            Category::Storage,
            Category::Process,
        ]
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Category::Memory => "MEM",
            Category::CpuLoad => "CPU",
            Category::Network => "NET",
            Category::Storage => "DSK",
            Category::Process => "PRC",
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
        }
    }
}

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
        // STORAGE / PROCESS — Placeholder (Phase 2)
        // =====================================================================
        (Category::Storage, Locale::En) => CategoryContent {
            overview: "Storage category content is planned for Phase 2.",
            story: "Storage cross-source narrative coming soon.",
            diagnostic_flow: "Storage diagnostic flow coming soon.",
            common_issues: "Storage common issues coming soon.",
        },
        (Category::Storage, Locale::Ja) => CategoryContent {
            overview: "ストレージカテゴリのコンテンツはフェーズ2で追加予定です。",
            story: "ストレージのソース横断ナラティブは準備中です。",
            diagnostic_flow: "ストレージの診断フローは準備中です。",
            common_issues: "ストレージのよくある問題は準備中です。",
        },
        (Category::Process, Locale::En) => CategoryContent {
            overview: "Process category content is planned for Phase 2.",
            story: "Process cross-source narrative coming soon.",
            diagnostic_flow: "Process diagnostic flow coming soon.",
            common_issues: "Process common issues coming soon.",
        },
        (Category::Process, Locale::Ja) => CategoryContent {
            overview: "プロセスカテゴリのコンテンツはフェーズ2で追加予定です。",
            story: "プロセスのソース横断ナラティブは準備中です。",
            diagnostic_flow: "プロセスの診断フローは準備中です。",
            common_issues: "プロセスのよくある問題は準備中です。",
        },
    }
}
