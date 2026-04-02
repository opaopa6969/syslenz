# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:19:49+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 8662
- duplicated lines: 80260
- duplication ratio: 90.26%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 7317
- duplicated lines: 50302
- duplication ratio: 87.30%

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
- [202] - Archetype B: anomaly amplification driven by wake-delay and queue coupling.
- [179] This field is a manifestation of **extended networking edge conditions**.
- [179] - Typical trigger: hidden retry loops and pathological connection patterns.
- [179] - Kernel path: drop/retry/error pathways not obvious in throughput charts.
- [179] - Cross-check: protocol counters plus application timeout/error rates.
- [172] Case C: Reversible mitigation provided faster learning than invasive change.
- [172] Case B: Cross-source correlation reversed the initial diagnosis.
- [172] Case A: First anomaly came from this field trend, not absolute value.
- [167] This field is strongest for separating vmstat reclaim signal from side-effect noise when scan and reclaim trends diverge.
- [167] This field is a manifestation of **virtual memory state transitions**.
- [167] Read this when vmstat reclaim and scan behavior is noisy and you need a fast, defensible decision.
- [167] If you cannot tell reclaim pressure from side effect, use this article to order evidence before tuning.
- [167] - Which vmstat-side step produced the highest confidence gain?
- [167] - Which step failed to reduce uncertainty about reclaim pressure versus side effect?
- [167] - What vmstat instrumentation change would accelerate this drill next time?
- [167] - Weak: isolated value with no vmstat reclaim/scan context
- [167] - Typical trigger: memory pressure or workload phase shift.
- [167] - This section mirrors man-page flow for vmstat reclaim analysis: definition -> scan/reclaim context -> failure branches -> evidence order.
- [167] - T0: user symptom confirmed with vmstat-side context
- [167] - T-5m: first vmstat reclaim or scan anomaly candidate

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 8650
- duplicated lines: 80273
- duplication ratio: 90.27%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 7304
- duplicated lines: 50316
- duplication ratio: 87.32%

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
- [211] ### シニアレビューでよく問われる点
- [211] ### エピソード: ダッシュボードの安心感とユーザー痛みがずれた日
- [202] - 類型B: wake遅延とqueue結合で異常が増幅する状態。
- [179] この field は **拡張ネットワーク異常経路の発露** です。
- [179] - 併読: protocol counters と app timeout/error。
- [179] - 主な契機: 隠れた再試行ループ、病的接続パターン。
- [179] - カーネル経路: throughput だけでは見えない drop/retry/error 系。
- [172] ケースC: 侵襲的変更より可逆対策で学習速度が上がった。
- [172] ケースB: クロスソース照合で初期診断が反転した。
- [172] ケースA: 絶対値ではなくトレンドが最初の異常を示した。
- [167] この field は、vmstat の reclaim 主信号と副作用ノイズを、scan/reclaimトレンド分岐時に分離するのに最も有効です。
- [167] この field は **仮想メモリ状態遷移の発露** です。
- [167] vmstat の reclaim/scan 挙動がノイジーで、短時間に防御可能な判断が必要な時に読みます。
- [167] reclaim圧力と副作用の切り分けが曖昧なら、調整前に証拠順序をここで固定します。
- [167] - 類型C: vmstat scan/reclaim 位相変化が遅れてユーザー影響へ出る状態。
- [167] - 類型B: vmstat reclaim 振動により短い回復と再悪化を繰り返す状態。
- [167] - 類型A: 利用率は安全に見えるのに vmstat reclaim 圧力で stall が上がる状態。
- [167] - 領域: 仮想メモリと reclaim 挙動
- [167] - 迷ったら本番の vmstat 関連メモリノブ変更前に SEE ALSO を辿って証拠を増やしてください。
- [167] - 緩和が10分遅れた場合、どの vmstat 連動ユーザー指標が先に閾値を超えるか？

