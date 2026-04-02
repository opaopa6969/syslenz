use crate::article::{ArticleKind, ArticleLink, EducationArticle};

const LINK_READING_METRICS: ArticleLink = ArticleLink::Article {
    label_en: "Reading metrics basics",
    label_ja: "メトリクスの読み方の基本",
    id: "concept.reading-metrics",
};

const LINK_PRESSURE_STALL: ArticleLink = ArticleLink::Article {
    label_en: "Pressure Stall Information",
    label_ja: "PSI プレッシャー",
    id: "concept.pressure-stall",
};

macro_rules! metric_article {
    ($id:expr, $title_en:expr, $title_ja:expr, $body_en:expr, $body_ja:expr, [$($link:expr),* $(,)?]) => {
        EducationArticle {
            id: $id,
            kind: ArticleKind::Metric,
            title_en: $title_en,
            title_ja: $title_ja,
            body_en: $body_en,
            body_ja: $body_ja,
            links: &[$($link),*],
        }
    };
}

pub const ARTICLES_METRICS: &[crate::article::EducationArticle] = &[
    metric_article!(
        "meminfo.MemAvailable",
        "MemAvailable",
        "利用可能メモリ",
        r#"What is this?
Kernel-estimated memory that can be used without immediate reclaim or swap.

Why it matters
This is the best quick signal for memory headroom. MemFree alone can look scary on a healthy system.

How to read
- Healthy: comfortably above 10% of MemTotal.
- Watch: 10-20% and trending down.
- Risk: below 10% with pswpout or PSI increasing.

Next check
Compare with MemFree, pswpout, and pressure.memory_full_avg10."#,
        r#"これは何か
カーネルが「すぐに回収やスワップを発生させずに使える」と見積もるメモリ量です。

なぜ重要か
空きメモリの見た目より、実際の余裕をよく表します。MemFree だけで判断すると誤りやすいです。

どう読むか
- 健全: MemTotal の 10% 以上
- 注意: 10〜20% で低下傾向
- 危険: 10% 未満で pswpout や PSI が増加

次に確認
MemFree、pswpout、pressure.memory_full_avg10 と合わせて見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemFree",
                label_ja: "MemFree",
                source: "meminfo",
                field: "MemFree",
            },
            ArticleLink::Metric {
                label_en: "memory_full_avg10",
                label_ja: "memory_full_avg10",
                source: "pressure",
                field: "memory_full_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "meminfo.MemFree",
        "MemFree",
        "空きメモリ",
        r#"What is this?
RAM that is not allocated to anything right now.

Why it matters
Low MemFree is normal on Linux because the kernel uses RAM for cache. Do not treat MemFree as a shortage signal by itself.

How to read
- If MemAvailable is healthy, low MemFree is usually fine.
- If both MemFree and MemAvailable fall, pressure is real.
- If swap activity rises too, reclaim is no longer enough.

Next check
Look at Cached, MemAvailable, and swap activity together."#,
        r#"これは何か
いま何にも使われていない RAM です。

なぜ重要か
Linux ではキャッシュに RAM を使うので、MemFree が少ないこと自体は普通です。MemFree 単独では判断しません。

どう読むか
- MemAvailable が十分なら、MemFree が少なくても問題ないことが多い
- MemFree と MemAvailable が同時に下がるなら圧力あり
- スワップも増えるなら回収だけでは足りない

次に確認
Cached、MemAvailable、スワップ動作を合わせて見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemAvailable",
                label_ja: "MemAvailable",
                source: "meminfo",
                field: "MemAvailable",
            },
            ArticleLink::Metric {
                label_en: "Cached",
                label_ja: "Cached",
                source: "meminfo",
                field: "Cached",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "meminfo.Cached",
        "Cached",
        "ページキャッシュ",
        r#"What is this?
File-backed memory kept in RAM so reads do not have to hit disk again.

Why it matters
Cached RAM is not wasted RAM. It is usually the first reclaimable pool when memory pressure rises.

How to read
- Large Cached on a file server is often good.
- Falling Cached plus rising pswpout means the kernel is running out of cheap reclaim.
- Dirty pages mixed into the picture mean writeback may be lagging.

Next check
Compare with nr_dirty, nr_writeback, and MemAvailable."#,
        r#"これは何か
ファイル由来のデータを再読込しなくて済むように RAM に置いている領域です。

なぜ重要か
Cached は無駄ではありません。メモリ圧力が高まったとき、まず回収対象になることが多いです。

どう読むか
- ファイルサーバーで Cached が大きいのはよくある
- Cached が減り、pswpout が増えるなら回収余地が減っている
- Dirty も多いなら書き戻しが追いついていない可能性

次に確認
nr_dirty、nr_writeback、MemAvailable と合わせて見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemAvailable",
                label_ja: "MemAvailable",
                source: "meminfo",
                field: "MemAvailable",
            },
            ArticleLink::Metric {
                label_en: "nr_dirty",
                label_ja: "nr_dirty",
                source: "vmstat",
                field: "nr_dirty",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "vmstat.nr_dirty",
        "nr_dirty",
        "Dirty ページ",
        r#"What is this?
Pages that have been modified in RAM but not written back to storage yet.

Why it matters
A growing dirty page count means the system is building writeback pressure. That can turn into stalls later.

How to read
- Short spikes are normal during bursts.
- Sustained growth means writeback is behind.
- If pressure.io_some_avg10 also rises, the storage path is feeling it.

Next check
Watch nr_writeback and diskstats.active_devices."#,
        r#"これは何か
RAM 上で変更されたが、まだストレージへ書き戻されていないページです。

なぜ重要か
Dirty が増え続けると、あとから書き戻し圧力として現れます。

どう読むか
- 短いスパイクはバーストなら普通
- 継続的な増加は書き戻し遅れ
- pressure.io_some_avg10 も上がるなら I/O 影響あり

次に確認
nr_writeback と diskstats.active_devices を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "nr_writeback",
                label_ja: "nr_writeback",
                source: "vmstat",
                field: "nr_writeback",
            },
            ArticleLink::Metric {
                label_en: "io_some_avg10",
                label_ja: "io_some_avg10",
                source: "pressure",
                field: "io_some_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "vmstat.nr_writeback",
        "nr_writeback",
        "書き込み中ページ",
        r#"What is this?
Pages currently being written to storage.

Why it matters
This is the in-flight part of writeback. If it stays elevated, the storage path cannot keep up.

How to read
- A small amount is normal.
- Sustained elevation points to slow disks or a heavy dirty-page backlog.
- Pair it with PSI to decide whether users are seeing stalls.

Next check
Compare with diskstats.active_devices and pressure.io_some_avg10."#,
        r#"これは何か
いまストレージへ書き込み中のページです。

なぜ重要か
これが高止まりするなら、ストレージが追いついていません。

どう読むか
- 少量なら普通
- 継続的に高いなら遅いディスクか Dirty の滞留
- PSI と合わせると、実際に停滞しているか分かります

次に確認
diskstats.active_devices と pressure.io_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "diskstats active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            ArticleLink::Metric {
                label_en: "io_some_avg10",
                label_ja: "io_some_avg10",
                source: "pressure",
                field: "io_some_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "vmstat.pswpin",
        "pswpin",
        "スワップイン",
        r#"What is this?
Pages brought back from swap into RAM.

Why it matters
Any sustained pswpin means the system is paying swap I/O to recover working sets.

How to read
- Occasional spikes can be harmless after a burst.
- Repeated growth means memory pressure is real.
- If pswpout also rises, the system is thrashing between RAM and swap.

Next check
Compare with MemAvailable and pswpout."#,
        r#"これは何か
スワップから RAM に戻したページ数です。

なぜ重要か
pswpin が続くと、作業セットを戻すためにスワップ I/O を払っています。

どう読むか
- 一時的なスパイクなら許容されることもある
- 継続増加ならメモリ圧力あり
- pswpout も増えるなら RAM と swap を行き来している

次に確認
MemAvailable と pswpout を合わせて見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemAvailable",
                label_ja: "MemAvailable",
                source: "meminfo",
                field: "MemAvailable",
            },
            ArticleLink::Metric {
                label_en: "pswpout",
                label_ja: "pswpout",
                source: "vmstat",
                field: "pswpout",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "vmstat.pswpout",
        "pswpout",
        "スワップアウト",
        r#"What is this?
Pages evicted from RAM to swap.

Why it matters
Growing pswpout is a direct sign that RAM pressure has crossed from warning into action.

How to read
- Some historical swap-out can be fine.
- New growth while the workload is active means reclaim is failing to keep up.
- If pressure.memory_full_avg10 is non-zero, the impact is visible.

Next check
Look at MemAvailable and pressure.memory_full_avg10."#,
        r#"これは何か
RAM から swap へ追い出されたページ数です。

なぜ重要か
pswpout の増加は、RAM 圧力が実際に動き始めた証拠です。

どう読むか
- 過去のスワップアウトが少しある程度なら問題ないこともある
- ワークロード稼働中に増えるなら回収が追いついていない
- pressure.memory_full_avg10 が非ゼロなら影響が見えています

次に確認
MemAvailable と pressure.memory_full_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemAvailable",
                label_ja: "MemAvailable",
                source: "meminfo",
                field: "MemAvailable",
            },
            ArticleLink::Metric {
                label_en: "memory_full_avg10",
                label_ja: "memory_full_avg10",
                source: "pressure",
                field: "memory_full_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "vmstat.pgmajfault",
        "pgmajfault",
        "大ページフォルト",
        r#"What is this?
Page faults that needed disk I/O to resolve.

Why it matters
Major faults are much more expensive than minor faults. A steady rise usually means the system is fetching data from disk under pressure.

How to read
- Near-zero on steady workloads is ideal.
- Bursts during startup or cold cache warm-up are normal.
- Continuous growth points to memory pressure or working sets that do not fit.

Next check
Look at diskstats.active_devices and pressure.io_some_avg10."#,
        r#"これは何か
解決にディスク I/O が必要だったページフォルトです。

なぜ重要か
minor fault よりずっと重く、継続増加はメモリ不足やキャッシュ不足を示します。

どう読むか
- 安定稼働中はほぼゼロが理想
- 起動時やコールドキャッシュでは一時的増加は普通
- 継続増加はメモリ圧力や作業セット不足

次に確認
diskstats.active_devices と pressure.io_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "diskstats active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            ArticleLink::Metric {
                label_en: "io_some_avg10",
                label_ja: "io_some_avg10",
                source: "pressure",
                field: "io_some_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "stat.cpu_user",
        "cpu_user",
        "ユーザー CPU",
        r#"What is this?
Time spent running application code in user mode.

Why it matters
This is the useful work side of CPU usage. It is better than staring at load alone.

How to read
- Rising user time with stable latency is usually fine.
- User time plus rising loadavg can mean the host is simply busy.
- User time without matching throughput can point to inefficient code.

Next check
Compare with loadavg.load_1min and pressure.cpu_some_avg10."#,
        r#"これは何か
ユーザーモードでアプリケーションコードを実行していた時間です。

なぜ重要か
CPU 使用率を見るなら、実際の仕事をしている側です。負荷平均だけを見るより役に立ちます。

どう読むか
- latency が安定しているなら上昇は普通
- loadavg も上がるなら単純に忙しいだけの可能性
- スループットが増えないのに増えるなら非効率なコードの疑い

次に確認
loadavg.load_1min と pressure.cpu_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "load_1min",
                label_ja: "load_1min",
                source: "loadavg",
                field: "load_1min",
            },
            ArticleLink::Metric {
                label_en: "cpu_some_avg10",
                label_ja: "cpu_some_avg10",
                source: "pressure",
                field: "cpu_some_avg10",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "stat.cpu_system",
        "cpu_system",
        "カーネル CPU",
        r#"What is this?
Time spent inside kernel mode on behalf of workloads.

Why it matters
A high system share can mean network, storage, or syscall-heavy activity.

How to read
- System time rising together with iowait often means storage pressure.
- System time with high packet rates can mean network processing overhead.
- System time alone is not bad if throughput also rises.

Next check
Look at cpu_iowait, net/snmp counters, and loadavg."#,
        r#"これは何か
ワークロードのためにカーネルモードで使った時間です。

なぜ重要か
system が高いと、ネットワークやストレージ、syscall が重い可能性があります。

どう読むか
- iowait と一緒に上がるならストレージ圧力の疑い
- パケット数増加と一緒ならネットワーク処理負荷の可能性
- スループットも増えているなら悪いとは限らない

次に確認
cpu_iowait、net/snmp のカウンタ、loadavg を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "cpu_iowait",
                label_ja: "cpu_iowait",
                source: "stat",
                field: "cpu_iowait",
            },
            ArticleLink::Metric {
                label_en: "context_switches",
                label_ja: "context_switches",
                source: "stat",
                field: "context_switches",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "stat.cpu_iowait",
        "cpu_iowait",
        "I/O 待ち CPU",
        r#"What is this?
Time the CPU spent idle while waiting for storage operations to complete.

Why it matters
iowait is not real CPU work. It is a symptom of blocked storage or filesystem work.

How to read
- Small values can be normal.
- Persistent growth means the system is waiting on disk.
- Pair it with pressure.io_some_avg10 to see whether tasks are stalling.

Next check
Compare with diskstats.active_devices and pressure.io_some_avg10."#,
        r#"これは何か
ストレージ操作の完了待ちで CPU が空いていた時間です。

なぜ重要か
iowait は CPU の仕事ではなく、ストレージやファイルシステム待ちの症状です。

どう読むか
- 小さい値なら普通
- 継続的に高いならディスク待ち
- pressure.io_some_avg10 と合わせると停滞の有無が分かる

次に確認
diskstats.active_devices と pressure.io_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "diskstats active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            ArticleLink::Metric {
                label_en: "io_some_avg10",
                label_ja: "io_some_avg10",
                source: "pressure",
                field: "io_some_avg10",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "stat.procs_running",
        "procs_running",
        "実行中プロセス",
        r#"What is this?
Processes or threads that are runnable right now.

Why it matters
This is a direct view of scheduler demand. If it stays above available CPU capacity, latency climbs.

How to read
- Small spikes are fine.
- Sustained elevation plus high loadavg means CPU demand is real.
- If pressure.cpu_some_avg10 rises too, tasks are actually waiting.

Next check
Compare with loadavg.running_threads and pressure.cpu_some_avg10."#,
        r#"これは何か
今すぐ実行可能なプロセス／スレッド数です。

なぜ重要か
スケジューラの需要を直接見られます。CPU 余力を超えて続くと遅延が増えます。

どう読むか
- 小さいスパイクは問題ない
- 高止まりと loadavg 上昇が一緒なら CPU 需要が本物
- pressure.cpu_some_avg10 も上がるなら、実際に待たされている

次に確認
loadavg.running_threads と pressure.cpu_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "running_threads",
                label_ja: "running_threads",
                source: "loadavg",
                field: "running_threads",
            },
            ArticleLink::Metric {
                label_en: "cpu_some_avg10",
                label_ja: "cpu_some_avg10",
                source: "pressure",
                field: "cpu_some_avg10",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "stat.context_switches",
        "context_switches",
        "コンテキスト切替",
        r#"What is this?
Total voluntary and involuntary context switches since boot.

Why it matters
This is a scheduler churn signal. It helps explain why the CPU may feel busy even when raw utilization is moderate.

How to read
- Steady growth is normal on multi-process systems.
- Sudden jumps can point to lock contention or too many short-lived threads.
- Use it with process_count to see whether the system is thrashing.

Next check
Compare with process_count and loadavg.load_1min."#,
        r#"これは何か
起動以来のコンテキスト切替の総数です。

なぜ重要か
スケジューラの入れ替え量が分かります。CPU 使用率が中程度でも、切替が多いと重く感じます。

どう読むか
- 多プロセス環境では継続増加は普通
- 急増ならロック競合や短命スレッドの疑い
- process_count と合わせると過度な揺れが見える

次に確認
process_count と loadavg.load_1min を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "process_count",
                label_ja: "process_count",
                source: "processes",
                field: "process_count",
            },
            ArticleLink::Metric {
                label_en: "load_1min",
                label_ja: "load_1min",
                source: "loadavg",
                field: "load_1min",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "loadavg.load_1min",
        "load_1min",
        "1 分負荷",
        r#"What is this?
The average number of runnable or uninterruptible tasks over the last minute.

Why it matters
Load is demand, not utilization. It tells you whether the scheduler queue is getting crowded.

How to read
- Compare it with CPU count.
- A load near CPU count is often acceptable.
- A load above CPU count plus rising PSI means real contention.

Next check
Look at procs_running and pressure.cpu_some_avg10."#,
        r#"これは何か
過去 1 分の runnable / uninterruptible タスクの平均です。

なぜ重要か
負荷は使用率ではなく需要です。スケジューラの行列が詰まっているかを示します。

どう読むか
- CPU 数と比べる
- CPU 数前後なら多くの環境で許容範囲
- CPU 数超え + PSI 上昇なら競合あり

次に確認
procs_running と pressure.cpu_some_avg10 を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "procs_running",
                label_ja: "procs_running",
                source: "stat",
                field: "procs_running",
            },
            ArticleLink::Metric {
                label_en: "cpu_some_avg10",
                label_ja: "cpu_some_avg10",
                source: "pressure",
                field: "cpu_some_avg10",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "loadavg.running_threads",
        "running_threads",
        "実行可能スレッド数",
        r#"What is this?
The instantaneous runnable thread count from /proc/loadavg.

Why it matters
This is the live queue depth, not a smoothed average. It moves faster than load_1min.

How to read
- Use it to catch spikes that the average hides.
- If it stays above CPU count, the queue is backing up.
- Pair it with context_switches for scheduler churn.

Next check
Compare with procs_running and process_count."#,
        r#"これは何か
/proc/loadavg に出る、いま実行可能なスレッド数です。

なぜ重要か
平均化されていない即時値なので、load_1min より速くスパイクを拾えます。

どう読むか
- 平均値に隠れた急上昇を見る
- CPU 数を超えて続くならキューが詰まっている
- context_switches と合わせると切替負荷も見える

次に確認
procs_running と process_count を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "procs_running",
                label_ja: "procs_running",
                source: "stat",
                field: "procs_running",
            },
            ArticleLink::Metric {
                label_en: "process_count",
                label_ja: "process_count",
                source: "processes",
                field: "process_count",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "pressure.cpu_some_avg10",
        "cpu_some_avg10",
        "CPU 圧力",
        r#"What is this?
The fraction of the last 10 seconds where at least one task was stalled waiting for CPU time.

Why it matters
This is a demand-side metric. It tells you whether users are waiting, not just whether CPUs are busy.

How to read
- Any sustained non-zero value means contention exists.
- Rising loadavg with zero PSI is queueing without user-visible pain yet.
- Rising PSI means the pain is real.

Next check
Compare with loadavg.load_1min and stat.procs_running."#,
        r#"これは何か
直近 10 秒で、少なくとも 1 つのタスクが CPU 待ちだった時間割合です。

なぜ重要か
需要側の指標なので、CPU が忙しいかではなく、人が待っているかを示します。

どう読むか
- 持続的に非ゼロなら競合あり
- loadavg 上昇だけで PSI がゼロなら、まだ体感影響前かもしれない
- PSI も上がるなら実害がある

次に確認
loadavg.load_1min と stat.procs_running を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "load_1min",
                label_ja: "load_1min",
                source: "loadavg",
                field: "load_1min",
            },
            ArticleLink::Metric {
                label_en: "procs_running",
                label_ja: "procs_running",
                source: "stat",
                field: "procs_running",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "pressure.memory_full_avg10",
        "memory_full_avg10",
        "メモリ全停滞",
        r#"What is this?
The fraction of the last 10 seconds where all non-idle tasks were stalled on memory.

Why it matters
This is severe memory pressure. It means the whole system is being held up by memory reclaim or swap.

How to read
- Any non-zero value is serious.
- If it persists, the machine is effectively degraded.
- Correlate it with pswpout and pgmajfault.

Next check
Look at MemAvailable and vmstat.pswpout."#,
        r#"これは何か
直近 10 秒で、全ての非アイドルタスクがメモリ待ちで停滞した時間割合です。

なぜ重要か
深刻なメモリ圧力で、システム全体が回収や swap に足を取られています。

どう読むか
- 非ゼロなら重大
- 持続するなら実質的に性能劣化状態
- pswpout や pgmajfault と相関を見る

次に確認
MemAvailable と vmstat.pswpout を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "MemAvailable",
                label_ja: "MemAvailable",
                source: "meminfo",
                field: "MemAvailable",
            },
            ArticleLink::Metric {
                label_en: "pswpout",
                label_ja: "pswpout",
                source: "vmstat",
                field: "pswpout",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "pressure.io_some_avg10",
        "io_some_avg10",
        "I/O 圧力",
        r#"What is this?
The fraction of the last 10 seconds where at least one task was stalled waiting for I/O.

Why it matters
This is the cleanest signal that storage latency is affecting tasks.

How to read
- Rising dirty pages plus this metric usually means writeback is behind.
- If it stays above zero, tasks are waiting on storage.
- Spikes can come from log bursts or backup windows.

Next check
Compare with diskstats.active_devices and cpu_iowait."#,
        r#"これは何か
直近 10 秒で、少なくとも 1 つのタスクが I/O 待ちで止まっていた時間割合です。

なぜ重要か
ストレージ遅延が実際にタスクへ影響しているかを最も直接的に示します。

どう読むか
- Dirty 増加と一緒なら書き戻し遅れの可能性
- ゼロを超えて続くなら I/O 待ちあり
- ログ大量書き込みやバックアップ時間帯のスパイクもある

次に確認
diskstats.active_devices と cpu_iowait を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "diskstats active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            ArticleLink::Metric {
                label_en: "cpu_iowait",
                label_ja: "cpu_iowait",
                source: "stat",
                field: "cpu_iowait",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "pressure.io_full_avg10",
        "io_full_avg10",
        "I/O 全停滞",
        r#"What is this?
The fraction of the last 10 seconds where all non-idle tasks were stalled on I/O.

Why it matters
This is a strong sign that storage is the bottleneck, not just a noisy background signal.

How to read
- Any non-zero value is bad.
- Sustained elevation means the host is effectively I/O bound.
- Pair it with writeback and diskstats to separate device saturation from workload bursts.

Next check
Look at nr_writeback and diskstats.active_devices."#,
        r#"これは何か
直近 10 秒で、全ての非アイドルタスクが I/O 待ちだった時間割合です。

なぜ重要か
単なるノイズではなく、ストレージが本当にボトルネックである強い兆候です。

どう読むか
- 非ゼロなら悪い
- 持続上昇なら I/O 偏重
- writeback と diskstats で機器飽和かバーストかを切り分ける

次に確認
nr_writeback と diskstats.active_devices を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "nr_writeback",
                label_ja: "nr_writeback",
                source: "vmstat",
                field: "nr_writeback",
            },
            ArticleLink::Metric {
                label_en: "diskstats active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            LINK_PRESSURE_STALL,
        ]
    ),
    metric_article!(
        "diskstats.active_devices",
        "active_devices",
        "稼働中デバイス数",
        r#"What is this?
The number of block devices that have seen real I/O activity since boot.

Why it matters
It is a quick way to tell whether storage load is concentrated on a few devices or spread across many.

How to read
- Low count with high I/O pressure points to a few hot disks.
- High count with low pressure can be background noise.
- Use the device table for the real breakdown.

Next check
Compare with pressure.io_some_avg10 and root disk usage."#,
        r#"これは何か
起動以来、実際に I/O があったブロックデバイス数です。

なぜ重要か
負荷が少数のディスクに集中しているのか、広く分散しているのかが分かります。

どう読むか
- 少数で I/O 圧力が高いならホットディスク集中
- 多いが圧力が低いならバックグラウンドノイズの可能性
- 明細表で本当の内訳を見る

次に確認
pressure.io_some_avg10 と root ディスク使用率を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "io_some_avg10",
                label_ja: "io_some_avg10",
                source: "pressure",
                field: "io_some_avg10",
            },
            ArticleLink::Metric {
                label_en: "available_disk",
                label_ja: "available_disk",
                source: "df",
                field: "available_disk",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "df.available_disk",
        "available_disk",
        "利用可能ディスク容量",
        r#"What is this?
Bytes available for new writes on the root filesystem.

Why it matters
This is the operational free space number. It is the one that prevents log loss and failed deploys.

How to read
- Keep a healthy buffer, not just a few megabytes.
- Fast downward trends matter more than a single snapshot.
- If it keeps falling, find the writer before the filesystem fills.

Next check
Compare with root_use_pct and file descriptor usage."#,
        r#"これは何か
ルートファイルシステムで新規書き込みに使える容量です。

なぜ重要か
運用上の空き容量です。ログ消失やデプロイ失敗を防ぐ指標になります。

どう読むか
- 数 MB ではなく十分な余裕を持つ
- 1 回の値より低下トレンドが重要
- 減り続けるなら書き込み元を探す

次に確認
root_use_pct と FD 使用率を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "root_use_pct",
                label_ja: "root_use_pct",
                source: "df",
                field: "root_use_pct",
            },
            ArticleLink::Metric {
                label_en: "fd_allocated",
                label_ja: "fd_allocated",
                source: "file-nr",
                field: "fd_allocated",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "df.root_use_pct",
        "root_use_pct",
        "ルート使用率",
        r#"What is this?
Percentage of the root filesystem that is in use.

Why it matters
This is the easiest disk-capacity alarm to understand. Once the root filesystem fills, many services fail in ugly ways.

How to read
- Watch the trend, not just the absolute number.
- 80% can already be uncomfortable on busy hosts.
- 90%+ is usually a cleanup or expansion task.

Next check
Compare with available_disk and diskstats.active_devices."#,
        r#"これは何か
ルートファイルシステムの使用率です。

なぜ重要か
一番分かりやすい容量アラームです。ルートが埋まると多くのサービスが壊れます。

どう読むか
- 絶対値だけでなく推移を見る
- 忙しいホストでは 80% でも厳しいことがある
- 90% 超は整理か増設が必要なことが多い

次に確認
available_disk と diskstats.active_devices を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "available_disk",
                label_ja: "available_disk",
                source: "df",
                field: "available_disk",
            },
            ArticleLink::Metric {
                label_en: "active_devices",
                label_ja: "active_devices",
                source: "diskstats",
                field: "active_devices",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "processes.process_count",
        "process_count",
        "プロセス数",
        r#"What is this?
The total number of processes currently visible under /proc.

Why it matters
A growing process count can signal a fork storm, container churn, or a process leak.

How to read
- A sudden jump is worth checking immediately.
- A slow climb over time can be a leak.
- Pair it with context_switches if the host feels busy.

Next check
Compare with file descriptor usage and procs_running."#,
        r#"これは何か
/proc に見える現在の総プロセス数です。

なぜ重要か
増え続けるなら fork 嵐、コンテナの増減、プロセスリークの可能性があります。

どう読むか
- 急増ならすぐ確認
- ゆっくり増え続けるならリークの疑い
- busy に感じるなら context_switches も見る

次に確認
FD 使用率と procs_running を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "fd_allocated",
                label_ja: "fd_allocated",
                source: "file-nr",
                field: "fd_allocated",
            },
            ArticleLink::Metric {
                label_en: "procs_running",
                label_ja: "procs_running",
                source: "stat",
                field: "procs_running",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "file-nr.fd_allocated",
        "fd_allocated",
        "割り当て済み FD",
        r#"What is this?
File handles currently allocated by the kernel.

Why it matters
This is the system-wide file descriptor pool. When it gets close to the limit, opens and sockets start failing.

How to read
- Growing over time can indicate a leak.
- Some reuse is normal, so do not panic over short spikes.
- Compare with fd_usage_pct to understand how close you are to exhaustion.

Next check
Look at fd_usage_pct and process_count."#,
        r#"これは何か
カーネルが現在割り当てているファイルハンドル数です。

なぜ重要か
システム全体の FD プールです。上限に近づくと open() や socket() が失敗し始めます。

どう読むか
- 時間とともに増えるならリークの可能性
- 短いスパイクは再利用で普通なこともある
- fd_usage_pct で上限までの余裕を見る

次に確認
fd_usage_pct と process_count を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "fd_usage_pct",
                label_ja: "fd_usage_pct",
                source: "file-nr",
                field: "fd_usage_pct",
            },
            ArticleLink::Metric {
                label_en: "process_count",
                label_ja: "process_count",
                source: "processes",
                field: "process_count",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "file-nr.fd_usage_pct",
        "fd_usage_pct",
        "FD 使用率",
        r#"What is this?
The percentage of the system-wide file handle limit that is currently in active use.

Why it matters
This is the most actionable file-nr number. It tells you when the host is approaching EMFILE risk.

How to read
- Under 50%: usually fine.
- Around 80%: start looking for leaks.
- Near the limit: open() and socket() failures become likely.

Next check
Compare with fd_allocated and process_count."#,
        r#"これは何か
システム全体のファイルハンドル上限に対する実使用率です。

なぜ重要か
最も実務的な file-nr の数字です。EMFILE リスクを直接見られます。

どう読むか
- 50% 未満: たいてい問題なし
- 80% 前後: リーク調査を開始
- 上限近く: open() や socket() 失敗が起きやすい

次に確認
fd_allocated と process_count を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "fd_allocated",
                label_ja: "fd_allocated",
                source: "file-nr",
                field: "fd_allocated",
            },
            ArticleLink::Metric {
                label_en: "process_count",
                label_ja: "process_count",
                source: "processes",
                field: "process_count",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "net/dev.total_rx",
        "total_rx",
        "受信バイト総量",
        r#"What is this?
Cumulative received bytes across all network interfaces.

Why it matters
This is the simplest host-level network throughput signal. Use deltas between snapshots to estimate receive rate.

How to read
- Growth over time is expected on active hosts.
- Compare it with total_tx to see whether the host is mostly consuming or serving data.
- If packet drops also rise, traffic quality is the issue, not just volume.

Next check
Compare with total_tx and Tcp_RetransSegs."#,
        r#"これは何か
全ネットワークインターフェースを合計した受信バイトの累積値です。

なぜ重要か
ホスト全体のネットワークスループットを一番簡単に見る指標です。スナップショット差分で受信レートを出します。

どう読むか
- 稼働中ホストなら増加は普通
- total_tx と比べると、受信中心か送信中心かが分かる
- ドロップも増えるなら、量だけでなく品質の問題

次に確認
total_tx と Tcp_RetransSegs を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "total_tx",
                label_ja: "total_tx",
                source: "net/dev",
                field: "total_tx",
            },
            ArticleLink::Metric {
                label_en: "Tcp_RetransSegs",
                label_ja: "Tcp_RetransSegs",
                source: "net/snmp",
                field: "Tcp_RetransSegs",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "net/dev.total_tx",
        "total_tx",
        "送信バイト総量",
        r#"What is this?
Cumulative transmitted bytes across all network interfaces.

Why it matters
This shows outbound network volume. It is useful for spotting serving workloads, backups, or unexpected exfiltration.

How to read
- High TX with low RX often means the host is serving data.
- Compare with total_rx to understand traffic direction.
- Sudden jumps deserve attention if the workload did not change.

Next check
Compare with total_rx and Udp_InErrors."#,
        r#"これは何か
全ネットワークインターフェースを合計した送信バイトの累積値です。

なぜ重要か
送信量の把握に使います。配信系、バックアップ、想定外の外向き通信を見つけるのに役立ちます。

どう読むか
- TX が高く RX が低いなら配信系の可能性
- total_rx と比べると通信の向きが分かる
- ワークロード変更なしの急増は要確認

次に確認
total_rx と Udp_InErrors を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "total_rx",
                label_ja: "total_rx",
                source: "net/dev",
                field: "total_rx",
            },
            ArticleLink::Metric {
                label_en: "Udp_InErrors",
                label_ja: "Udp_InErrors",
                source: "net/snmp",
                field: "Udp_InErrors",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "net/snmp.Tcp_RetransSegs",
        "Tcp_RetransSegs",
        "TCP 再送",
        r#"What is this?
Total TCP segments retransmitted because acknowledgements did not arrive in time.

Why it matters
Retransmissions usually mean packet loss, congestion, or an overloaded peer.

How to read
- Use the rate, not the absolute value.
- A small background rate is normal on busy networks.
- Rising retransmits plus listen drops can mean multiple network bottlenecks at once.

Next check
Compare with Udp_InErrors and TcpExt_ListenDrops."#,
        r#"これは何か
ACK が間に合わず再送された TCP セグメント数です。

なぜ重要か
パケットロス、輻輳、相手側の過負荷を示すことが多いです。

どう読むか
- 絶対値ではなく増加率を見る
- 忙しいネットワークなら少しの背景再送は普通
- 再送と listen drop が同時なら、複数のボトルネックがある可能性

次に確認
Udp_InErrors と TcpExt_ListenDrops を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "Udp_InErrors",
                label_ja: "Udp_InErrors",
                source: "net/snmp",
                field: "Udp_InErrors",
            },
            ArticleLink::Metric {
                label_en: "TcpExt_ListenDrops",
                label_ja: "TcpExt_ListenDrops",
                source: "net/netstat",
                field: "TcpExt_ListenDrops",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "net/snmp.Udp_InErrors",
        "Udp_InErrors",
        "UDP 受信エラー",
        r#"What is this?
UDP datagrams that could not be delivered successfully.

Why it matters
UDP has no retransmission. Once packets are dropped here, the application never gets them.

How to read
- Any persistent growth deserves a look.
- Receive-buffer exhaustion is a common cause.
- Check this alongside packet rate and application ingest speed.

Next check
Compare with Tcp_RetransSegs and listen queue drops."#,
        r#"これは何か
正常に配信できなかった UDP データグラムです。

なぜ重要か
UDP には再送がありません。ここで落ちるとアプリは取りこぼします。

どう読むか
- 持続増加は要確認
- 受信バッファ不足がよくある原因
- パケット量とアプリの取り込み速度と一緒に見る

次に確認
Tcp_RetransSegs と listen queue drop を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "Tcp_RetransSegs",
                label_ja: "Tcp_RetransSegs",
                source: "net/snmp",
                field: "Tcp_RetransSegs",
            },
            ArticleLink::Metric {
                label_en: "TcpExt_ListenDrops",
                label_ja: "TcpExt_ListenDrops",
                source: "net/netstat",
                field: "TcpExt_ListenDrops",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "net/netstat.TcpExt_ListenDrops",
        "TcpExt_ListenDrops",
        "Listen queue drop",
        r#"What is this?
Connections dropped because the listen queue was full.

Why it matters
This is an application-facing failure mode. Clients are turned away when the server cannot accept fast enough.

How to read
- Any non-zero rate is a problem to investigate.
- Traffic spikes can overwhelm a too-small backlog.
- Garbage-collection pauses or slow accept loops can show up here.

Next check
Compare with Tcp_RetransSegs and procs_running."#,
        r#"これは何か
listen queue がいっぱいで落とされた接続です。

なぜ重要か
クライアントに見える失敗です。サーバーが速く accept できないと、接続が拒否されます。

どう読むか
- 非ゼロなら調査対象
- トラフィック急増で backlog が小さいと溢れる
- GC 停止や遅い accept ループでも出る

次に確認
Tcp_RetransSegs と procs_running を見ます。"#,
        [
            ArticleLink::Metric {
                label_en: "Tcp_RetransSegs",
                label_ja: "Tcp_RetransSegs",
                source: "net/snmp",
                field: "Tcp_RetransSegs",
            },
            ArticleLink::Metric {
                label_en: "procs_running",
                label_ja: "procs_running",
                source: "stat",
                field: "procs_running",
            },
            LINK_READING_METRICS,
        ]
    ),
    metric_article!(
        "thermal.max_temp",
        "max_temp",
        "最高温度",
        r#"What is this?
The hottest thermal zone currently reported by the system.

Why it matters
Thermal limits can silently reduce CPU speed before you notice a clear failure.

How to read
- Below 50°C is usually comfortable.
- Around 75°C is warm enough to watch.
- Above 90°C usually means throttling risk or active throttling.

Next check
Compare with cpu_user and cpu_iowait to see whether heat is workload-driven."#,
        r#"これは何か
システム内で今いちばん高温なサーマルゾーンです。

なぜ重要か
明確な故障になる前に、温度制限で CPU が遅くなることがあります。

どう読むか
- 50°C 未満ならだいたい余裕あり
- 75°C 前後なら注意
- 90°C 超はスロットリングの危険または実発生

次に確認
cpu_user と cpu_iowait を見て、負荷由来か確認します。"#,
        [
            ArticleLink::Metric {
                label_en: "cpu_user",
                label_ja: "cpu_user",
                source: "stat",
                field: "cpu_user",
            },
            ArticleLink::Metric {
                label_en: "cpu_iowait",
                label_ja: "cpu_iowait",
                source: "stat",
                field: "cpu_iowait",
            },
            LINK_READING_METRICS,
        ]
    ),
];
