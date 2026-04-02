# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:27:15+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 12834
- duplicated lines: 76088
- duplication ratio: 85.57%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 11489
- duplicated lines: 46130
- duplication ratio: 80.06%

#### Exclusion regex
- `^(## (NAME|WHY NOW|EVIDENCE ORDER|SEE ALSO|Incident Forensics|Failure Archetype Matrix|Counterfactual Branches|Man-Page Crosswalk|Unix Internals Lens|Metric Snapshot|Action Loop)|## (なぜ今読むか|証拠の順序|障害フォレンジクス|失敗類型マトリクス|反事実分岐|manページ・クロスウォーク|Unix内部システム視点|メトリクス概要|アクションループ)|### (Evidence Capture|Decision Record|Drill Steps|Debrief Questions|エビデンス取得|証拠取得|判断記録|ドリル手順|振り返り設問)|[0-9]+\..*|- (Field in focus|注目field|Disproof attempt|反証試行):.*|Use related links in the article overlay to continue the same evidence chain\.|記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。)$`

### Top 20 repeated lines (Raw)
- [689] Use related links in the article overlay to continue the same evidence chain.
- [689] 4. Apply reversible mitigation and confirm trend recovery.
- [689] 3. Cross-check one sibling signal and one cross-layer signal.
- [689] 2. Verify this article's primary signal trend.
- [689] 1. Fix user-visible symptom and time window first.
- [689] - Disproof attempt: identify one alternate cause and log why it failed.
- [689] ### Evidence Capture
- [689] ### Decision Record
- [689] ## WHY NOW
- [689] ## SEE ALSO
- [689] ## NAME
- [689] ## Man-Page Crosswalk
- [689] ## Incident Forensics
- [689] ## Failure Archetype Matrix
- [689] ## EVIDENCE ORDER
- [689] ## Counterfactual Branches
- [660] ## Unix Internals Lens
- [615] 4. Keep concise evidence notes for postmortem reuse.
- [615] 2. Apply reversible mitigation.
- [615] ## Metric Snapshot

### Top 20 repeated lines (Framework-Excluded)
- [202] - Archetype B (Network): anomaly amplification driven by wake-delay and queue coupling.
- [179] This field is a manifestation of **extended networking edge conditions**.
- [179] - Typical trigger: hidden retry loops and pathological connection patterns.
- [179] - Kernel path: drop/retry/error pathways not obvious in throughput charts.
- [179] - Cross-check: protocol counters plus application timeout/error rates.
- [172] Case C: Reversible mitigation provided faster learning than invasive change.
- [172] Case B: Cross-source correlation reversed the initial diagnosis.
- [172] Case A: First anomaly came from this field trend, not absolute value.
- [167] - Scheduler lens: Scheduler: verify whether reclaim-side waiting inflated wake latency during vmstat spikes.
- [167] - Root-cause declaration from one vmstat reclaim/scan snapshot.
- [167] - Revalidate vmstat-related thresholds after release, kernel, or allocator-behavior changes.
- [167] - Process lens: Process: check runnable vs blocked expansion around vmstat inflection windows.
- [167] - Prefer explicit timestamps and vmstat reclaim-phase notes; narratives drift without phase context.
- [167] - Pair this field with one scheduler signal and one writeback or swap signal to test cross-layer consistency.
- [167] - Medium: correlated movement without clear vmstat reclaim/scan boundary
- [167] - Link this article to at least two neighboring vmstat or memory-path articles in your runbook.
- [167] - Kernel path: reclaim, compaction, page fault handling, swap policy.
- [167] - Keep one baseline note per environment, workload phase, and vmstat reclaim mode.
- [167] - Interrupt or IO lens: Storage or IO: validate writeback or page-in side effects around vmstat shifts.
- [167] - Ignoring post-mitigation recovery shape in vmstat timeline.

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 10534
- duplicated lines: 78389
- duplication ratio: 88.15%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 9188
- duplicated lines: 48432
- duplication ratio: 84.05%

#### Exclusion regex
- `^(## (NAME|WHY NOW|EVIDENCE ORDER|SEE ALSO|Incident Forensics|Failure Archetype Matrix|Counterfactual Branches|Man-Page Crosswalk|Unix Internals Lens|Metric Snapshot|Action Loop)|## (なぜ今読むか|証拠の順序|障害フォレンジクス|失敗類型マトリクス|反事実分岐|manページ・クロスウォーク|Unix内部システム視点|メトリクス概要|アクションループ)|### (Evidence Capture|Decision Record|Drill Steps|Debrief Questions|エビデンス取得|証拠取得|判断記録|ドリル手順|振り返り設問)|[0-9]+\..*|- (Field in focus|注目field|Disproof attempt|反証試行):.*|Use related links in the article overlay to continue the same evidence chain\.|記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。)$`

### Top 20 repeated lines (Raw)
- [689] 記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。
- [689] 4. 可逆な緩和策を当て、回復トレンドを検証する。
- [689] 3. 隣接シグナル1つと跨層シグナル1つで照合する。
- [689] 2. この記事の主シグナルのトレンドを確認する。
- [689] 1. 先にユーザー症状と時刻窓を固定する。
- [689] - 反証試行: 代替原因を1つ立て、なぜ棄却したかを記録する。
- [689] ### 証拠取得
- [689] ### 判断記録
- [689] ## 障害フォレンジクス
- [689] ## 証拠の順序
- [689] ## 失敗類型マトリクス
- [689] ## 反事実分岐
- [689] ## なぜ今読むか
- [689] ## manページ・クロスウォーク
- [689] ## SEE ALSO
- [689] ## NAME
- [660] ## Unix内部システム視点
- [615] 2. 可逆な緩和策を適用する。
- [615] ## メトリクス概要
- [615] ## アクションループ

### Top 20 repeated lines (Framework-Excluded)
- [202] - 類型B（ネットワーク）: wake遅延とqueue結合で異常が増幅する状態。
- [179] この field は **拡張ネットワーク異常経路の発露** です。
- [179] - 併読: protocol counters と app timeout/error。
- [179] - 主な契機: 隠れた再試行ループ、病的接続パターン。
- [179] - カーネル経路: throughput だけでは見えない drop/retry/error 系。
- [172] ケースC: 侵襲的変更より可逆対策で学習速度が上がった。
- [172] ケースB: クロスソース照合で初期診断が反転した。
- [172] ケースA: 絶対値ではなくトレンドが最初の異常を示した。
- [167] この field は **仮想メモリ状態遷移の発露** です。
- [167] - 類型C: vmstat scan/reclaim 位相変化が遅れてユーザー影響へ出る状態。
- [167] - 類型B: vmstat reclaim 振動により短い回復と再悪化を繰り返す状態。
- [167] - 類型A: 利用率は安全に見えるのに vmstat reclaim 圧力で stall が上がる状態。
- [167] - 領域: 仮想メモリと reclaim 挙動
- [167] - 強い: 時系列順・跨層・vmstat reclaim整合トレンド
- [167] - 弱い: vmstat reclaim/scan 文脈なしの一点値
- [167] - 単一の vmstat reclaim/scan スナップショットで根因断定する。
- [167] - 前提時刻と vmstat reclaim 位相を明示してください。位相情報なしでは再利用性が下がります。
- [167] - 分岐4: 一時回復後に再悪化。 -> vmstat reclaim と retry のフィードバックループ確認。
- [167] - 分岐3: 対策後に両方悪化。 -> 即ロールバックし vmstat 証拠スナップショットを保全。
- [167] - 分岐2: vmstat トレンドは改善したが症状は不変。 -> CPUまたはIOの並行連鎖を探索。

