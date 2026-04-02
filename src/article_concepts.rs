use crate::article::{ArticleKind, ArticleLink, EducationArticle};

const ARTICLE_OPS_METHOD: EducationArticle = EducationArticle {
    id: "concept.ops-methodology",
    kind: ArticleKind::Concept,
    title_en: "Operations Methodology",
    title_ja: "運用の進め方",
    body_en: r#"What is this?
A practical loop for turning a symptom into evidence, then evidence into action.

Why it matters
Without a method, teams jump to fixes before they know what layer is failing.

How to use
1. Name the symptom in one sentence.
2. Split the stack into app, kernel, device, and hardware.
3. Read at least one capacity metric and one pressure or latency metric.
4. Verify whether the issue is local, repeated, or workload-wide.

Common mistakes
- Starting from the noisiest graph
- Fixing symptoms without checking blast radius
- Treating a single metric as proof

Diagnostic flow
- User impact first -> look for latency and pressure
- Resource saturation -> look for headroom and queueing
- Change-related -> compare against the last deploy or config change
"#,
    body_ja: r#"これは何か
症状を証拠に変え、証拠を行動に変えるための実務的な進め方です。

なぜ重要か
方法がないと、どの層が壊れているか分からないまま修正に飛びつきます。

どう使うか
1. 症状を一文で言う
2. アプリ、カーネル、デバイス、ハードウェアに分ける
3. 容量メトリクスと圧力/遅延メトリクスを少なくとも1つずつ見る
4. 影響が局所か、再現するか、全体かを確かめる

よくある誤り
- いちばん派手なグラフから始める
- 影響範囲を見ずに修正する
- 単一メトリクスを証拠だと思い込む

診断フロー
- 利用者影響がある -> 遅延と圧力を見る
- 資源飽和がある -> 余裕とキューを確認する
- 変更起因が疑わしい -> 直近のデプロイや設定変更と比べる
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Cross-metric reading",
            label_ja: "クロス指標の読み方",
            id: "concept.cross-metric-reading",
        },
        ArticleLink::Article {
            label_en: "Incident workflow",
            label_ja: "インシデント手順",
            id: "concept.incident-workflow",
        },
        ArticleLink::Metric {
            label_en: "MemAvailable",
            label_ja: "MemAvailable",
            source: "meminfo",
            field: "MemAvailable",
        },
    ],
};

const ARTICLE_CROSS_METRIC: EducationArticle = EducationArticle {
    id: "concept.cross-metric-reading",
    kind: ArticleKind::Concept,
    title_en: "Cross-metric Reading",
    title_ja: "クロスメトリクスの読み方",
    body_en: r#"What is this?
The habit of reading metrics in pairs or triples instead of in isolation.

Why it matters
Every metric has blind spots. Pairing counters reveals whether a value is healthy, noisy, or misleading.

How to use
- Capacity + pressure: MemAvailable with PSI, utilization with queue depth
- Average + tail: mean latency with p95/p99 or max
- Count + rate: event count with per-second growth
- Symptom + cause: user latency with CPU, memory, IO, or network signals

Common mistakes
- Looking only at a single gauge
- Comparing values with different units
- Reading a rising count without checking rate

Diagnostic flow
1. Start with the symptom metric.
2. Add a pressure metric to test contention.
3. Add a capacity metric to test headroom.
4. Add a rate or tail metric to test burstiness.
"#,
    body_ja: r#"これは何か
単一メトリクスではなく、2つか3つの指標を組にして読む習慣です。

なぜ重要か
どのメトリクスにも死角があります。組み合わせると、健全・ノイズ・誤解の区別ができます。

どう使うか
- 容量 + 圧力: MemAvailable と PSI、利用率とキュー深さ
- 平均 + 尾部: 平均遅延と p95/p99 や max
- 件数 + 速度: 件数と 1秒あたりの増加
- 症状 + 原因: 利用者遅延と CPU/メモリ/IO/ネットワーク

よくある誤り
- 1つのゲージだけを見る
- 単位の違う値を比べる
- 件数の増加を rate で確認しない

診断フロー
1. 症状のメトリクスから始める
2. 圧力メトリクスを足して競合を確認する
3. 容量メトリクスを足して余裕を確認する
4. rate か tail を足してバースト性を確認する
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Latency analysis",
            label_ja: "遅延分析",
            id: "concept.latency-analysis",
        },
        ArticleLink::Article {
            label_en: "Bottleneck triage",
            label_ja: "ボトルネック切り分け",
            id: "concept.bottleneck-triage",
        },
        ArticleLink::Metric {
            label_en: "memory_some_avg10",
            label_ja: "memory_some_avg10",
            source: "pressure",
            field: "memory_some_avg10",
        },
    ],
};

const ARTICLE_HARDWARE: EducationArticle = EducationArticle {
    id: "concept.hardware-components",
    kind: ArticleKind::Concept,
    title_en: "Hardware Components",
    title_ja: "ハードウェア構成要素",
    body_en: r#"What is this?
A map of the physical pieces that bound system performance: CPU, memory, storage, NICs, power, and thermals.

Why it matters
If you do not know the component, you cannot tell whether the limit is silicon, firmware, or software.

How to use
- CPU: cores, clocks, cache, SMT, thermal headroom
- Memory: RAM size, bandwidth, fragmentation, swap exposure
- Storage: device latency, queue depth, media type, endurance
- Network: NIC speed, drops, offload behavior, packet path

Common mistakes
- Treating all hardware as interchangeable
- Assuming faster storage fixes every IO problem
- Ignoring thermals and power limits on busy systems

Diagnostic flow
- CPU symptoms -> check frequency, throttling, and run queue
- Memory symptoms -> check headroom, reclaim, and fragmentation
- IO symptoms -> check device queue and tail latency
- Network symptoms -> check drops, retransmits, and link speed
"#,
    body_ja: r#"これは何か
CPU、メモリ、ストレージ、NIC、電源、温度など、システム性能を決める物理要素の地図です。

なぜ重要か
どの部品が限界か分からないと、制約がシリコンなのか、ファームウェアなのか、ソフトウェアなのか判定できません。

どう使うか
- CPU: コア数、クロック、キャッシュ、SMT、熱的余裕
- メモリ: 容量、帯域、断片化、スワップ露出
- ストレージ: デバイス遅延、キュー深さ、媒体種別、耐久性
- ネットワーク: NIC 速度、ドロップ、オフロード、経路

よくある誤り
- すべてのハードウェアを同じものとして扱う
- 高速ストレージで何でも解決すると考える
- 高負荷時の温度や電力制限を無視する

診断フロー
- CPU 症状 -> 周波数、スロットリング、ランキューを確認
- メモリ症状 -> 余裕、回収、断片化を確認
- IO 症状 -> デバイスキューと尾部遅延を確認
- ネットワーク症状 -> ドロップ、再送、リンク速度を確認
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Driver / kernel / OS boundary",
            label_ja: "ドライバ・カーネル・OS境界",
            id: "concept.driver-kernel-os-boundary",
        },
        ArticleLink::Metric {
            label_en: "thermal_temp",
            label_ja: "thermal_temp",
            source: "thermal",
            field: "temp",
        },
        ArticleLink::Metric {
            label_en: "cpu_mhz",
            label_ja: "cpu_mhz",
            source: "cpuinfo",
            field: "mhz",
        },
    ],
};

const ARTICLE_BOUNDARY: EducationArticle = EducationArticle {
    id: "concept.driver-kernel-os-boundary",
    kind: ArticleKind::Concept,
    title_en: "Driver, Kernel, and OS Boundary",
    title_ja: "ドライバ・カーネル・OSの境界",
    body_en: r#"What is this?
The seam between user space, the kernel, device drivers, and the hardware device itself.

Why it matters
Many incidents are caused by crossing the wrong layer: an app issue that looks like IO, or a driver issue that looks like CPU load.

How to use
- User space: threads, sockets, files, allocators
- Kernel: sched, VM, VFS, networking stack, block layer
- Driver: device programming, interrupts, DMA, queues
- Hardware: controller, media, bus, firmware, thermal limits

Common mistakes
- Blaming the app for a device timeout
- Blaming the disk for a filesystem or scheduler issue
- Forgetting that interrupts and DMA shift work into kernel space

Diagnostic flow
1. Check whether the symptom appears in one process or system-wide.
2. Check whether kernel pressure or interrupts rise with the symptom.
3. Check whether the device itself shows queueing or error signals.
"#,
    body_ja: r#"これは何か
ユーザー空間、カーネル、デバイスドライバ、実機の間にある境界です。

なぜ重要か
多くの障害は境界の見誤りで起きます。アプリの問題がIOに見えたり、ドライバの問題がCPU負荷に見えたりします。

どう使うか
- ユーザー空間: スレッド、ソケット、ファイル、アロケータ
- カーネル: scheduler、VM、VFS、ネットワーク、ブロック層
- ドライバ: デバイス制御、割り込み、DMA、キュー
- ハードウェア: コントローラ、媒体、バス、ファームウェア、温度制限

よくある誤り
- デバイスタイムアウトをアプリのせいにする
- ファイルシステムや scheduler の問題をディスクのせいにする
- 割り込みや DMA がカーネル側へ仕事を移すことを忘れる

診断フロー
1. 症状が単一プロセスか全体かを確認する
2. 圧力や割り込みが症状と同時に増えるかを見る
3. デバイス自体にキューやエラーがあるか確認する
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Hardware components",
            label_ja: "ハードウェア構成要素",
            id: "concept.hardware-components",
        },
        ArticleLink::Article {
            label_en: "Storage latency",
            label_ja: "ストレージ遅延",
            id: "concept.storage-latency",
        },
        ArticleLink::Metric {
            label_en: "irq_rate",
            label_ja: "irq_rate",
            source: "interrupts",
            field: "irq_rate",
        },
    ],
};

const ARTICLE_LATENCY: EducationArticle = EducationArticle {
    id: "concept.latency-analysis",
    kind: ArticleKind::Concept,
    title_en: "Latency Analysis",
    title_ja: "遅延分析",
    body_en: r#"What is this?
The practice of breaking latency into service time, queue time, retry time, and waiting time.

Why it matters
Average latency hides tail pain. The user feels the slowest requests, not the median.

How to use
- Separate work from waiting
- Compare average with p95/p99 or max
- Look for queue growth before throughput collapse
- Check retries and timeouts, because they amplify tail latency

Common mistakes
- Optimizing mean while tail grows
- Ignoring retries that multiply load
- Confusing fast failures with healthy service

Diagnostic flow
1. Confirm whether the pain is in compute, queueing, storage, or network.
2. Compare mean and tail together.
3. Check pressure and saturation on the suspected resource.
4. Correlate with recent change and retry behavior.
"#,
    body_ja: r#"これは何か
遅延を、処理時間・待ち時間・再試行時間・停止時間に分けて考える方法です。

なぜ重要か
平均遅延だけでは尾部のつらさが見えません。利用者が感じるのは中央値ではなく、遅いリクエストです。

どう使うか
- 仕事時間と待ち時間を分ける
- 平均と p95/p99 か max を並べて見る
- スループット低下の前にキュー増加を見る
- 再試行とタイムアウトを確認する。負荷を増幅します

よくある誤り
- 平均だけ最適化して尾部が悪化する
- 再試行が負荷を増やすことを無視する
- 早く失敗することを健全だと誤解する

診断フロー
1. 問題が compute、queue、storage、network のどれかを確かめる
2. 平均と尾部を同時に比べる
3. 疑わしい資源の圧力と飽和を確認する
4. 直近の変更と retry 挙動を結びつける
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Queueing vs saturation",
            label_ja: "キューと飽和",
            id: "concept.queueing-vs-saturation",
        },
        ArticleLink::Article {
            label_en: "Storage latency",
            label_ja: "ストレージ遅延",
            id: "concept.storage-latency",
        },
        ArticleLink::Metric {
            label_en: "io_some_avg10",
            label_ja: "io_some_avg10",
            source: "pressure",
            field: "io_some_avg10",
        },
    ],
};

const ARTICLE_TRIAGE: EducationArticle = EducationArticle {
    id: "concept.bottleneck-triage",
    kind: ArticleKind::Concept,
    title_en: "Bottleneck Triage",
    title_ja: "ボトルネック切り分け",
    body_en: r#"What is this?
A fast method for separating CPU, memory, storage, network, and process contention.

Why it matters
If you triage in the wrong order, you waste time staring at symptoms that are downstream of the real limit.

How to use
1. Decide whether the system is slow, stuck, or dropping work.
2. Check pressure first to see whether the resource is contended.
3. Check capacity second to see whether the system still has headroom.
4. Check tails and retries last to estimate how bad the user impact is.

Common mistakes
- Starting with root-cause speculation
- Assuming one subsystem can explain every symptom
- Missing the difference between saturation and failure

Diagnostic flow
- Slow requests -> latency analysis
- High run queue -> CPU contention
- Low headroom -> memory or storage saturation
- Retries or drops -> network or IO path issues
"#,
    body_ja: r#"これは何か
CPU、メモリ、ストレージ、ネットワーク、プロセス競合を素早く切り分ける方法です。

なぜ重要か
順番を誤ると、本当の制限の下流にある症状を延々と見続けることになります。

どう使うか
1. 遅いのか、止まっているのか、落としているのかを決める
2. まず圧力を見て競合の有無を確認する
3. 次に容量を見て余裕を確認する
4. 最後に尾部と retry で利用者影響を見積もる

よくある誤り
- 根本原因の推測から始める
- 1つのサブシステムで全部説明できると思う
- 飽和と失敗の違いを見落とす

診断フロー
- 遅いリクエスト -> 遅延分析
- ランキューが高い -> CPU 競合
- 余裕が少ない -> メモリかストレージの飽和
- retry や drop が多い -> ネットワークか IO 経路の問題
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Cross-metric reading",
            label_ja: "クロスメトリクスの読み方",
            id: "concept.cross-metric-reading",
        },
        ArticleLink::Article {
            label_en: "Latency analysis",
            label_ja: "遅延分析",
            id: "concept.latency-analysis",
        },
        ArticleLink::Metric {
            label_en: "cpu_some_avg10",
            label_ja: "cpu_some_avg10",
            source: "pressure",
            field: "cpu_some_avg10",
        },
    ],
};

const ARTICLE_INCIDENT: EducationArticle = EducationArticle {
    id: "concept.incident-workflow",
    kind: ArticleKind::Concept,
    title_en: "Incident Workflow",
    title_ja: "インシデント対応の流れ",
    body_en: r#"What is this?
The sequence of steps that keeps a bad event from becoming a worse one.

Why it matters
An incident is about control and learning, not only about the first fix.

How to use
- Detect: decide whether the issue is active and user-visible
- Stabilize: reduce blast radius, roll back, throttle, or isolate
- Diagnose: gather the smallest set of metrics that can explain the symptom
- Recover: restore service before perfect root cause analysis
- Learn: record change, cause, mitigation, and follow-up owner

Common mistakes
- Spending too long on root cause before service is stable
- Skipping timestamps and change context
- Forgetting to write down what was actually observed

Diagnostic flow
1. Protect users first.
2. Capture evidence second.
3. Restore service third.
4. Write the postmortem last.
"#,
    body_ja: r#"これは何か
悪い出来事を、もっと悪い事態にしないための一連の手順です。

なぜ重要か
インシデントは最初の修正だけでなく、制御と学習が重要です。

どう使うか
- 検知: 問題が進行中で、利用者に見えているか決める
- 安定化: 影響範囲を狭める、ロールバックする、制限する、分離する
- 診断: 症状を説明できる最小限のメトリクスを集める
- 復旧: 完璧な原因分析より先にサービスを戻す
- 学習: 変更、原因、緩和策、担当者を残す

よくある誤り
- サービスが安定する前に根本原因に時間を使いすぎる
- タイムスタンプと変更文脈を残さない
- 実際に観測した内容を書き残さない

診断フロー
1. まず利用者を守る
2. 次に証拠を集める
3. その後でサービスを復旧する
4. 最後にポストモーテムを書く
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Operations methodology",
            label_ja: "運用の進め方",
            id: "concept.ops-methodology",
        },
        ArticleLink::Article {
            label_en: "SLO thinking",
            label_ja: "SLO の考え方",
            id: "concept.slo-thinking",
        },
        ArticleLink::Metric {
            label_en: "memory_some_avg10",
            label_ja: "memory_some_avg10",
            source: "pressure",
            field: "memory_some_avg10",
        },
    ],
};

const ARTICLE_SLO: EducationArticle = EducationArticle {
    id: "concept.slo-thinking",
    kind: ArticleKind::Concept,
    title_en: "SLO Thinking",
    title_ja: "SLO の考え方",
    body_en: r#"What is this?
Thinking in service level objectives, not just in infrastructure metrics.

Why it matters
Infrastructure health matters because it affects user outcomes. SLOs keep the conversation tied to user impact.

How to use
- Define the service that users actually care about
- Choose SLIs that reflect correctness, latency, or availability
- Set an objective that leaves room for normal variance
- Track error budget burn so you know when to pause risky change

Common mistakes
- Using internal metrics as the objective itself
- Setting objectives without a failure budget
- Treating every alert as an SLO violation

Diagnostic flow
1. Is the user-facing SLI degraded?
2. Is the error budget burning fast or slow?
3. Which subsystem is the best explanation for the SLI movement?
4. Should you ship, pause, or mitigate?
"#,
    body_ja: r#"これは何か
インフラの数字ではなく、サービス目標で考える方法です。

なぜ重要か
インフラの健全性は利用者結果に影響するから意味があります。SLO は議論を利用者影響に結びつけます。

どう使うか
- 利用者が本当に気にするサービスを定義する
- 正しさ、遅延、可用性を表す SLI を選ぶ
- 通常のばらつきを吸収できる目標を置く
- エラーバジェット消費を追って危険な変更を止める

よくある誤り
- 内部メトリクスをそのまま目的にする
- 失敗予算なしで目標を決める
- すべてのアラートを SLO 侵害だと思う

診断フロー
1. 利用者向け SLI が悪化しているか
2. エラーバジェットは速く減っているか、遅いか
3. どのサブシステムが SLI 変化の説明として最適か
4. 続行、停止、緩和のどれにするか
"#,
    links: &[
        ArticleLink::Article {
            label_en: "Incident workflow",
            label_ja: "インシデント対応の流れ",
            id: "concept.incident-workflow",
        },
        ArticleLink::Article {
            label_en: "Latency analysis",
            label_ja: "遅延分析",
            id: "concept.latency-analysis",
        },
        ArticleLink::Metric {
            label_en: "load1",
            label_ja: "load1",
            source: "loadavg",
            field: "load1",
        },
    ],
};

const ARTICLE_CAPACITY: EducationArticle = EducationArticle {
    id: "concept.capacity-vs-utilization",
    kind: ArticleKind::Concept,
    title_en: "Capacity vs Utilization",
    title_ja: "容量と利用率の違い",
    body_en: r#"What is this?
The difference between how busy a resource looks and how much safe room it really has.

Why it matters
High utilization does not always mean trouble, and low utilization does not always mean safety.

How to use
- Capacity asks: how much headroom remains?
- Utilization asks: how much is currently in use?
- Pressure asks: is anyone waiting because of contention?

Common mistakes
- Using utilization alone to predict failure
- Ignoring burstiness and queue growth
- Assuming a flat average means safe headroom

Diagnostic flow
1. Measure headroom.
2. Measure contention.
3. Compare recent trend with peak behavior.
4. Decide whether to add capacity, rebalance, or reduce demand.
"#,
    body_ja: r#"これは何か
ある資源がどれだけ忙しく見えるかと、実際にどれだけ余裕があるかの違いです。

なぜ重要か
利用率が高いから危険とは限らず、低いから安全とも限りません。

どう使うか
- 容量: どれだけ余裕が残っているか
- 利用率: 今どれだけ使っているか
- 圧力: 競合で待っているものがあるか

よくある誤り
- 利用率だけで失敗を予測する
- バーストとキュー増加を無視する
- 平均が平らなら余裕があると思う

診断フロー
1. 余裕を測る
2. 競合を測る
3. 直近の傾向とピーク時の挙動を比べる
4. 増設、分散、需要削減のどれが必要か決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "MemAvailable",
            label_ja: "MemAvailable",
            source: "meminfo",
            field: "MemAvailable",
        },
        ArticleLink::Metric {
            label_en: "load1",
            label_ja: "load1",
            source: "loadavg",
            field: "load1",
        },
        ArticleLink::Article {
            label_en: "Queueing vs saturation",
            label_ja: "キューと飽和",
            id: "concept.queueing-vs-saturation",
        },
    ],
};

const ARTICLE_QUEUEING: EducationArticle = EducationArticle {
    id: "concept.queueing-vs-saturation",
    kind: ArticleKind::Concept,
    title_en: "Queueing vs Saturation",
    title_ja: "キューと飽和",
    body_en: r#"What is this?
Two different failure shapes: work piling up in line, or the resource itself being full.

Why it matters
Queueing can exist before saturation. By the time saturation is obvious, tail latency may already be bad.

How to use
- Queueing shows up as waiting time, run queue, backlog, or retry accumulation
- Saturation shows up as a lack of headroom, throttling, or blocked progress
- The same utilization level can be safe on one workload and unstable on another

Common mistakes
- Equating any queue with overload
- Equating high utilization with saturation
- Ignoring service time changes that turn a small queue into a large one

Diagnostic flow
1. Check whether work is waiting.
2. Check whether the system still has spare capacity.
3. Check whether retries are increasing the queue.
4. Check whether the queue is a symptom or the cause.
"#,
    body_ja: r#"これは何か
仕事が列にたまる状態と、資源自体が満杯になる状態の違いです。

なぜ重要か
飽和の前にキューは生まれます。飽和が見えた時には、すでに尾部遅延が悪化していることがあります。

どう使うか
- キュー: 待ち時間、ランキュー、バックログ、retry の蓄積
- 飽和: 余裕不足、スロットリング、進行停止
- 同じ利用率でも、ワークロード次第で安全にも不安定にもなる

よくある誤り
- すべてのキューを過負荷とみなす
- 高利用率をそのまま飽和とみなす
- サービス時間の変化で小さなキューが大きくなることを無視する

診断フロー
1. 仕事が待っているか確認する
2. まだ余裕があるか確認する
3. retry がキューを増やしていないか確認する
4. キューが原因か結果かを見極める
"#,
    links: &[
        ArticleLink::Article {
            label_en: "CPU run queue",
            label_ja: "CPU ランキュー",
            id: "concept.cpu-runqueue",
        },
        ArticleLink::Article {
            label_en: "Latency analysis",
            label_ja: "遅延分析",
            id: "concept.latency-analysis",
        },
        ArticleLink::Metric {
            label_en: "cpu_some_avg10",
            label_ja: "cpu_some_avg10",
            source: "pressure",
            field: "cpu_some_avg10",
        },
    ],
};

const ARTICLE_CPU_RUNQUEUE: EducationArticle = EducationArticle {
    id: "concept.cpu-runqueue",
    kind: ArticleKind::Concept,
    title_en: "CPU Run Queue",
    title_ja: "CPU ランキュー",
    body_en: r#"What is this?
The set of runnable tasks waiting for CPU time.

Why it matters
A growing run queue means work is ready but cannot execute immediately. That is a direct signal of contention.

How to use
- Compare runnable tasks with CPU count
- Look for context-switch churn and load average growth
- Check whether the load is compute-bound or waiting on something else

Common mistakes
- Treating load average as the same thing as CPU usage
- Ignoring runnable tasks on machines with many cores
- Missing steal time or throttling on virtualized systems

Diagnostic flow
1. Check load and runnable count.
2. Check CPU pressure and utilization.
3. Check whether a single process or all tasks are waiting.
4. Check whether the real issue is CPU, IO, or lock contention.
"#,
    body_ja: r#"これは何か
CPU 実行待ちの runnable task の集合です。

なぜ重要か
ランキューが増えると、仕事は準備できていてもすぐ実行できません。これは競合の直接的なサインです。

どう使うか
- runnable task 数と CPU 数を比べる
- context switch の増加と load average の伸びを見る
- 負荷が compute bound か、別の待ちがあるか確認する

よくある誤り
- load average を CPU 使用率と同じものだと思う
- 多コア環境の runnable task を無視する
- 仮想化環境の steal time や throttling を見落とす

診断フロー
1. load と runnable 数を確認する
2. CPU 圧力と利用率を確認する
3. 単一プロセスか全体かを確認する
4. 本当の原因が CPU、IO、ロックのどれか確認する
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "load1",
            label_ja: "load1",
            source: "loadavg",
            field: "load1",
        },
        ArticleLink::Metric {
            label_en: "procs_running",
            label_ja: "procs_running",
            source: "stat",
            field: "procs_running",
        },
        ArticleLink::Article {
            label_en: "Throttling",
            label_ja: "スロットリング",
            id: "concept.throttling",
        },
    ],
};

const ARTICLE_MEMORY_PRESSURE: EducationArticle = EducationArticle {
    id: "concept.memory-pressure",
    kind: ArticleKind::Concept,
    title_en: "Memory Pressure",
    title_ja: "メモリ圧力",
    body_en: r#"What is this?
The point where memory demand starts causing reclaim, stalls, or swap activity.

Why it matters
Memory pressure is often visible before an OOM event. PSI and swap activity show impact earlier than a crash.

How to use
- Check MemAvailable for headroom
- Check Cached and Slab for reclaimable memory
- Check swap usage and PSI for user-visible stalls

Common mistakes
- Assuming low MemFree means pressure
- Ignoring cache reclaimability
- Watching only total used memory

Diagnostic flow
1. Confirm whether MemAvailable is falling.
2. Check whether swap is growing.
3. Check whether memory PSI is non-zero.
4. Decide whether to add RAM, reduce demand, or tune reclaim behavior.
"#,
    body_ja: r#"これは何か
メモリ需要が回収、停止、スワップを引き起こし始める地点です。

なぜ重要か
メモリ圧力は OOM より前に見えることが多いです。PSI とスワップ活動はクラッシュより早く影響を示します。

どう使うか
- 余裕を見るために MemAvailable を確認する
- 回収可能性を見るために Cached と Slab を確認する
- 利用者影響を見るために swap と PSI を確認する

よくある誤り
- MemFree が低いだけで圧力と判断する
- キャッシュの回収可能性を無視する
- 合計使用量だけを見る

診断フロー
1. MemAvailable が減っているか確認する
2. swap が増えているか確認する
3. memory PSI が非ゼロか確認する
4. RAM 増設、需要削減、回収調整のどれかを決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "MemAvailable",
            label_ja: "MemAvailable",
            source: "meminfo",
            field: "MemAvailable",
        },
        ArticleLink::Metric {
            label_en: "SwapFree",
            label_ja: "SwapFree",
            source: "meminfo",
            field: "SwapFree",
        },
        ArticleLink::Metric {
            label_en: "memory_some_avg10",
            label_ja: "memory_some_avg10",
            source: "pressure",
            field: "memory_some_avg10",
        },
    ],
};

const ARTICLE_STORAGE: EducationArticle = EducationArticle {
    id: "concept.storage-latency",
    kind: ArticleKind::Concept,
    title_en: "Storage Latency",
    title_ja: "ストレージ遅延",
    body_en: r#"What is this?
The time it takes for a block device and its filesystem path to complete reads, writes, and flushes.

Why it matters
Storage often looks idle by bandwidth but still hurts latency when the queue is deep or the device is busy with small IO.

How to use
- Separate sequential throughput from latency
- Watch average, tail, and queue depth together
- Check fsync, flush, and write amplification behavior

Common mistakes
- Thinking bandwidth alone explains disk performance
- Ignoring small random IO
- Missing that the filesystem layer can amplify device delays

Diagnostic flow
1. Confirm whether the symptom is read, write, or flush heavy.
2. Check queue depth and tail latency.
3. Check pressure and retries.
4. Decide whether the bottleneck is filesystem, driver, or device media.
"#,
    body_ja: r#"これは何か
ブロックデバイスとファイルシステム経路が、読み書きや flush を終えるまでの時間です。

なぜ重要か
帯域が空いて見えても、キューが深かったり小さな IO が多かったりすると遅延は悪化します。

どう使うか
- シーケンシャル帯域と遅延を分ける
- 平均、尾部、キュー深さを一緒に見る
- fsync、flush、書き込み増幅を見る

よくある誤り
- 帯域だけでディスク性能を判断する
- 小さなランダム IO を無視する
- ファイルシステム層が遅延を増幅することを見落とす

診断フロー
1. read/write/flush のどれが重いか確認する
2. キュー深さと尾部遅延を確認する
3. 圧力と retry を確認する
4. ボトルネックが filesystem、driver、media のどれか決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "io_some_avg10",
            label_ja: "io_some_avg10",
            source: "pressure",
            field: "io_some_avg10",
        },
        ArticleLink::Metric {
            label_en: "await",
            label_ja: "await",
            source: "diskstats",
            field: "await",
        },
        ArticleLink::Article {
            label_en: "Driver / kernel / OS boundary",
            label_ja: "ドライバ・カーネル・OS境界",
            id: "concept.driver-kernel-os-boundary",
        },
    ],
};

const ARTICLE_NETWORK: EducationArticle = EducationArticle {
    id: "concept.network-path",
    kind: ArticleKind::Concept,
    title_en: "Network Path",
    title_ja: "ネットワーク経路",
    body_en: r#"What is this?
The full path from application socket to NIC, switch, remote host, and back again.

Why it matters
Network issues often appear as app slowness, timeouts, or retries long before packet loss becomes obvious.

How to use
- Break the path into local stack, link, and remote path
- Check drops, retransmits, and RTT together
- Compare throughput with small-request latency

Common mistakes
- Blaming the network for an app backlog
- Looking at bandwidth when the problem is retransmission
- Forgetting that DNS, TLS, and connect time are part of network latency

Diagnostic flow
1. See whether the pain is connection setup, payload transfer, or packet recovery.
2. Check interface errors and retransmits.
3. Check whether one peer or all peers are affected.
4. Decide whether the limit is host, link, switch, or remote service.
"#,
    body_ja: r#"これは何か
アプリの socket から NIC、スイッチ、相手ホストまでの経路全体です。

なぜ重要か
ネットワーク問題は、パケットロスが明らかになる前に、アプリの遅さや timeout、retry として見えます。

どう使うか
- ローカルスタック、リンク、リモート経路に分ける
- drop、再送、RTT を一緒に見る
- 帯域と小さな要求の遅延を比べる

よくある誤り
- アプリの backlog をネットワークのせいにする
- 問題が再送なのに帯域だけを見る
- DNS、TLS、connect time がネットワーク遅延の一部であることを忘れる

診断フロー
1. 接続確立、転送、再送のどれが痛いか見る
2. interface エラーと再送を確認する
3. 1つの相手だけか、全体かを確認する
4. 制限が host、link、switch、remote service のどれか決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "rx_dropped",
            label_ja: "rx_dropped",
            source: "net/dev",
            field: "rx_dropped",
        },
        ArticleLink::Metric {
            label_en: "retransmits",
            label_ja: "retransmits",
            source: "net/tcp",
            field: "retransmits",
        },
        ArticleLink::Article {
            label_en: "Cross-metric reading",
            label_ja: "クロスメトリクスの読み方",
            id: "concept.cross-metric-reading",
        },
    ],
};

const ARTICLE_PROCESS: EducationArticle = EducationArticle {
    id: "concept.process-contention",
    kind: ArticleKind::Concept,
    title_en: "Process Contention",
    title_ja: "プロセス競合",
    body_en: r#"What is this?
The situation where processes compete for CPU, locks, file descriptors, or memory inside user space.

Why it matters
An application can look healthy at the process level while one hot thread or lock stalls the whole service.

How to use
- Check process counts, thread counts, and file descriptor growth
- Look for lock contention and context-switch spikes
- Compare a hot process with the system-wide picture

Common mistakes
- Assuming more processes means more throughput
- Looking only at total RSS or total CPU
- Missing a single lock that serializes everything

Diagnostic flow
1. Find the hottest process.
2. Check whether it is CPU-bound, lock-bound, or IO-bound.
3. Check whether other processes are blocked behind it.
4. Check whether the limit is application design or operating system resources.
"#,
    body_ja: r#"これは何か
CPU、ロック、ファイルディスクリプタ、メモリをユーザー空間のプロセス同士で奪い合う状態です。

なぜ重要か
アプリ全体は元気に見えても、1つのホットスレッドやロックが全体を止めることがあります。

どう使うか
- プロセス数、スレッド数、FD の増加を見る
- ロック競合と context switch の増加を見る
- ホットなプロセスと全体像を比べる

よくある誤り
- プロセスが多いほど速いと思う
- 合計 RSS や合計 CPU だけを見る
- 1つのロックが全体を直列化することを見落とす

診断フロー
1. 最も重いプロセスを見つける
2. CPU、lock、IO のどれに縛られているか確認する
3. 他のプロセスがその背後で止まっていないか確認する
4. 制約がアプリ設計か OS 資源かを判断する
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "procs_running",
            label_ja: "procs_running",
            source: "stat",
            field: "procs_running",
        },
        ArticleLink::Metric {
            label_en: "context_switches",
            label_ja: "context_switches",
            source: "stat",
            field: "context_switches",
        },
        ArticleLink::Article {
            label_en: "CPU run queue",
            label_ja: "CPU ランキュー",
            id: "concept.cpu-runqueue",
        },
    ],
};

const ARTICLE_RECLAIM: EducationArticle = EducationArticle {
    id: "concept.reclaim-vs-cache",
    kind: ArticleKind::Concept,
    title_en: "Reclaim vs Cache",
    title_ja: "回収とキャッシュ",
    body_en: r#"What is this?
The difference between memory that is immediately free, memory that can be reclaimed, and memory that is genuinely committed.

Why it matters
Linux uses RAM aggressively for cache. A large page cache is usually good, not a leak.

How to use
- Treat Cached and Buffers as potential headroom
- Treat Slab as partly reclaimable, partly sticky
- Treat MemAvailable as the best quick estimate

Common mistakes
- Calling every rise in used memory a leak
- Restarting services because MemFree looks low
- Ignoring whether reclaim is cheap or expensive

Diagnostic flow
1. Check whether the memory is free, cached, or committed.
2. Check whether reclaim cost is growing.
3. Check whether swap is being used.
4. Decide whether the issue is cache growth, true leak, or workload increase.
"#,
    body_ja: r#"これは何か
すぐ使える空き、回収できる空き、本当に確保済みのメモリの違いです。

なぜ重要か
Linux は RAM をキャッシュに積極利用します。大きな page cache は普通で、リークとは限りません。

どう使うか
- Cached と Buffers を回収余地として扱う
- Slab は一部回収可能、一部は粘ると考える
- MemAvailable を素早い推定値として使う

よくある誤り
- used memory の増加を全部リークだと思う
- MemFree が低いだけで再起動する
- 回収コストが安いか高いかを見ない

診断フロー
1. free / cached / committed のどれか確認する
2. 回収コストが増えているか確認する
3. swap が使われているか確認する
4. キャッシュ増加、真のリーク、需要増加のどれか決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "Cached",
            label_ja: "Cached",
            source: "meminfo",
            field: "Cached",
        },
        ArticleLink::Metric {
            label_en: "MemAvailable",
            label_ja: "MemAvailable",
            source: "meminfo",
            field: "MemAvailable",
        },
        ArticleLink::Article {
            label_en: "Memory pressure",
            label_ja: "メモリ圧力",
            id: "concept.memory-pressure",
        },
    ],
};

const ARTICLE_FRAGMENTATION: EducationArticle = EducationArticle {
    id: "concept.fragmentation",
    kind: ArticleKind::Concept,
    title_en: "Fragmentation",
    title_ja: "断片化",
    body_en: r#"What is this?
The state where free memory exists, but not in the sizes or locations that the kernel needs.

Why it matters
Fragmentation can break large allocations, huge pages, and contiguous DMA even when total free memory looks fine.

How to use
- Look at free blocks by order, not only total free bytes
- Check whether large allocations fail while small allocations succeed
- Compare long-lived memory patterns with recent spikes

Common mistakes
- Thinking fragmentation only matters when memory is nearly full
- Ignoring high-order allocation failures
- Confusing lack of capacity with lack of contiguity

Diagnostic flow
1. Confirm whether total memory is actually low.
2. Check whether high-order blocks are scarce.
3. Check whether the workload needs large contiguous chunks.
4. Decide whether compaction, reservation, or redesign is needed.
"#,
    body_ja: r#"これは何か
空きメモリはあるが、カーネルが必要とするサイズや位置にまとまっていない状態です。

なぜ重要か
断片化は、合計空きが十分でも、大きな割り当てや huge page、連続 DMA を壊します。

どう使うか
- 総空きだけでなく order 別の空きを見る
- 小さい割り当ては通るのに大きいものが失敗するか確認する
- 長寿命メモリの形と最近のスパイクを比べる

よくある誤り
- メモリがほぼ満杯のときだけ重要だと思う
- 高 order の割り当て失敗を無視する
- 容量不足と連続性不足を混同する

診断フロー
1. 合計メモリが本当に少ないか確認する
2. 高 order ブロックが不足していないか確認する
3. ワークロードが大きな連続領域を必要とするか確認する
4. compaction、予約、設計変更のどれが必要か決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "order0_free",
            label_ja: "order0_free",
            source: "buddyinfo",
            field: "order0_free",
        },
        ArticleLink::Metric {
            label_en: "free_pages",
            label_ja: "free_pages",
            source: "pagetypeinfo",
            field: "free_pages",
        },
        ArticleLink::Article {
            label_en: "Hardware components",
            label_ja: "ハードウェア構成要素",
            id: "concept.hardware-components",
        },
    ],
};

const ARTICLE_THROTTLING: EducationArticle = EducationArticle {
    id: "concept.throttling",
    kind: ArticleKind::Concept,
    title_en: "Throttling",
    title_ja: "スロットリング",
    body_en: r#"What is this?
An intentional limit that slows execution to stay within thermal, power, cgroup, or policy boundaries.

Why it matters
Throttling can look like random slowness unless you check the limiter itself.

How to use
- Check whether CPU frequency falls under load
- Check whether thermal or power limits are engaged
- Check whether cgroup or quota limits are active

Common mistakes
- Treating throttling as normal latency
- Ignoring power and thermal envelopes on laptops and dense servers
- Looking only at average CPU usage when frequency is capped

Diagnostic flow
1. See whether performance drops with rising heat or power draw.
2. Check whether the cap is imposed by hardware, firmware, or policy.
3. Check whether latency follows the cap exactly.
4. Decide whether to cool, reconfigure, or move the workload.
"#,
    body_ja: r#"これは何か
温度、電力、cgroup、ポリシーの境界を守るために意図的に実行を遅くする制限です。

なぜ重要か
制限器そのものを見ないと、スロットリングはただの遅さに見えます。

どう使うか
- 負荷時に CPU 周波数が下がるか確認する
- thermal や power limit が効いているか確認する
- cgroup や quota が有効か確認する

よくある誤り
- スロットリングを通常遅延だと思う
- ノート PC や高密度サーバーの電力・温度上限を無視する
- 周波数が抑えられているのに平均 CPU 使用率だけを見る

診断フロー
1. 温度や消費電力の増加とともに性能が下がるか見る
2. 制限が hardware、firmware、policy のどれか確認する
3. 遅延が制限にぴったり追従するか見る
4. 冷却、再設定、移設のどれが必要か決める
"#,
    links: &[
        ArticleLink::Metric {
            label_en: "thermal_temp",
            label_ja: "thermal_temp",
            source: "thermal",
            field: "temp",
        },
        ArticleLink::Metric {
            label_en: "cpu_mhz",
            label_ja: "cpu_mhz",
            source: "cpuinfo",
            field: "mhz",
        },
        ArticleLink::Article {
            label_en: "Latency analysis",
            label_ja: "遅延分析",
            id: "concept.latency-analysis",
        },
    ],
};

pub const ARTICLES_CONCEPTS: &[EducationArticle] = &[
    ARTICLE_OPS_METHOD,
    ARTICLE_CROSS_METRIC,
    ARTICLE_HARDWARE,
    ARTICLE_BOUNDARY,
    ARTICLE_LATENCY,
    ARTICLE_TRIAGE,
    ARTICLE_INCIDENT,
    ARTICLE_SLO,
    ARTICLE_CAPACITY,
    ARTICLE_QUEUEING,
    ARTICLE_CPU_RUNQUEUE,
    ARTICLE_MEMORY_PRESSURE,
    ARTICLE_STORAGE,
    ARTICLE_NETWORK,
    ARTICLE_PROCESS,
    ARTICLE_RECLAIM,
    ARTICLE_FRAGMENTATION,
    ARTICLE_THROTTLING,
];
