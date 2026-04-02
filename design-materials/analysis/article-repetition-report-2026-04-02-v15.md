# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:18:21+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 8602
- duplicated lines: 80320
- duplication ratio: 90.33%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 7257
- duplicated lines: 50362
- duplication ratio: 87.41%

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
- [211] ### Drill A: First-Mover Detection
- [211] ### Cross-Layer Translation
- [211] ### Combining With Unix Internals
- [211] ## Systems Narrative
- [211] ## Incident Lab
- [210] - Verify trend recovery after action.
- [210] - State one rollback condition.
- [210] - State one reversible action.
- [210] - Identify one user-facing symptom and timestamp.
- [210] - Identify one first-moving signal.
- [210] - Identify one cross-layer confirmation signal.
- [210] ## Quick Checklist
- [202] - Archetype B: anomaly amplification driven by wake-delay and queue coupling.
- [179] This field is a manifestation of **extended networking edge conditions**.
- [179] - Typical trigger: hidden retry loops and pathological connection patterns.
- [179] - Kernel path: drop/retry/error pathways not obvious in throughput charts.
- [179] - Cross-check: protocol counters plus application timeout/error rates.
- [172] Case C: Reversible mitigation provided faster learning than invasive change.
- [172] Case B: Cross-source correlation reversed the initial diagnosis.
- [172] Case A: First anomaly came from this field trend, not absolute value.

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 8590
- duplicated lines: 80333
- duplication ratio: 90.34%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 7244
- duplicated lines: 50376
- duplication ratio: 87.43%

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
- [211] ### 跨層翻訳
- [211] ### 演習A: 先行シグナル特定
- [211] ### シニアレビューでよく問われる点
- [211] ### エピソード: ダッシュボードの安心感とユーザー痛みがずれた日
- [211] ### Unix内部との接続
- [211] ## システム叙述
- [211] ## インシデント演習
- [210] - 跨層の確認シグナルを1つ選ぶ。
- [210] - 最初に動いたシグナルを1つ特定する。
- [210] - 実施後に回復トレンドを確認する。
- [210] - 可逆なアクションを1つ定義する。
- [210] - ロールバック条件を1つ定義する。
- [210] - ユーザー症状と時刻を1つ固定する。
- [210] ## クイックチェック
- [202] - 類型B: wake遅延とqueue結合で異常が増幅する状態。
- [179] この field は **拡張ネットワーク異常経路の発露** です。
- [179] - 併読: protocol counters と app timeout/error。
- [179] - 主な契機: 隠れた再試行ループ、病的接続パターン。
- [179] - カーネル経路: throughput だけでは見えない drop/retry/error 系。
- [172] ケースC: 侵襲的変更より可逆対策で学習速度が上がった。

