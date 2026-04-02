# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:16:26+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 8502
- duplicated lines: 80420
- duplication ratio: 90.44%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 7157
- duplicated lines: 50462
- duplication ratio: 87.58%

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
- [211] The narrative quality matters more than single-point precision: strong incidents are solved by ordered evidence, explicit assumptions, and controlled experiments.
- [211] If your team can replay this article as a short diagnostic script, the article is operationally useful.
- [211] - Which syscall family likely carried the user-visible penalty?
- [211] - Which mitigation was reversible and what rollback trigger was defined?
- [211] - Which counter was tempting but eventually demoted to a side effect?
- [211] - Which counter moved first in time order?
- [211] - User-facing latency regressed only in burst windows.
- [211] - This field moved first, and neighboring fields confirmed direction.
- [211] - The winning move was not a large tuning change, but narrowing uncertainty quickly.
- [211] - The dashboard looked green because averages stayed normal.
- [211] - Syscall lifecycle: where did request time shift (entry, sleep, wakeup, return)?
- [211] - Scheduler: did fairness protect throughput while harming tail latency?
- [211] - Process model: did runnable tasks increase, or did blocked tasks accumulate?
- [211] - Interrupt path: did wakeup delivery or softirq backlog alter tail behavior?
- [211] ### What Senior Reviewers Usually Ask
- [211] ### Review Outcome
- [211] ### Practical Mentor Notes
- [211] ### Episode: The Mismatch Between Dashboard Confidence and User Pain
- [211] ### Drill C: Evidence Compression
- [211] ### Drill B: Reversible Mitigation Design

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 8500
- duplicated lines: 80423
- duplication ratio: 90.44%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 7154
- duplicated lines: 50466
- duplication ratio: 87.58%

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
- [211] 単点精度より叙述品質が重要です。強い障害対応は、順序化された証拠、明示前提、可逆実験で解かれます。
- [211] この article を短い診断スクリプトとして再生できるなら、運用価値が高いです。
- [211] - 有力に見えたが副作用へ降格したカウンタは何か？
- [211] - 時系列で最初に動いたカウンタは何か？
- [211] - 平均値は正常で、ダッシュボードは緑でした。
- [211] - 可逆な対策と、ロールバックトリガは定義されたか？
- [211] - 勝ち筋は大きな調整ではなく、不確実性を素早く狭めることでした。
- [211] - ユーザー影響を運んだ syscall family は何か？
- [211] - しかしユーザー遅延はバースト窓だけで悪化しました。
- [211] - この field が先に動き、隣接 field が方向を裏付けました。
- [211] - syscall lifecycle: request時間は entry/sleep/wakeup/return のどこへ移ったか。
- [211] - scheduler: fairness は throughput を守りつつ tail を悪化させていないか。
- [211] - process model: runnable が増えたのか、blocked が滞留したのか。
- [211] - interrupt path: wakeup配送や softirq backlog が tail を押し上げたか。
- [211] ### 跨層翻訳
- [211] ### 演習C: 証拠圧縮
- [211] ### 演習B: 可逆対策の設計
- [211] ### 演習A: 先行シグナル特定
- [211] ### レビュー判定
- [211] ### メンターノート

