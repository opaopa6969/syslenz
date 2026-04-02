# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:11:14+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 8345
- duplicated lines: 80577
- duplication ratio: 90.62%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 7000
- duplicated lines: 50619
- duplication ratio: 87.85%

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
- [231] - Link this article to at least two neighboring articles in your runbook.
- [226] - T+6m: cross-source validation including socket or scheduler
- [226] - T+3m: first hypothesis written with segment assumption
- [226] - T+30m: recovery confidence decision with retransmit/latency confirmation
- [226] - T+15m: trend reaction checked for transport stabilization
- [226] - Root-cause declaration from one snapshot of transport counters.
- [226] - Keep one baseline note per environment, workload phase, and segment scope.
- [226] - Ignoring post-mitigation recovery shape in transport timeline.
- [226] - If uncertain, follow SEE ALSO links before changing production network knobs.
- [226] - If traffic had stayed constant, would transport-path transitions still move this field the same way?
- [226] - If this field had remained flat, which non-network signal could still explain the symptom?
- [226] - If only one layer could be instrumented, which network-adjacent layer would preserve most explanatory power?
- [226] - If mitigation was delayed by 10 minutes, which network-sensitive user metric would have crossed first?
- [226] - Confusing cross-layer transport correlation with causation.
- [226] - Branch 4: Short recovery then relapse. -> Check retry and wake-delay feedback loops.
- [226] - Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve transport evidence snapshot.
- [226] - Branch 2: Network-trend improves but symptom does not. -> Inspect parallel CPU or storage bottleneck chain.
- [226] - Branch 1: Symptom improves but network-trend does not. -> Revisit transport causal layer assumption.
- [226] - Avoid cargo-cult tuning; require before and after evidence with transport-path context.
- [226] ### Timeline Template (Network Incident)

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 8347
- duplicated lines: 80576
- duplication ratio: 90.61%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 7001
- duplicated lines: 50619
- duplication ratio: 87.85%

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
- [231] - この記事をランブックで隣接2記事以上に接続する。
- [226] - 迷ったら本番ネットワークノブ変更前に SEE ALSO を辿って証拠を増やしてください。
- [226] - 転送相関を因果と誤認する。
- [226] - 転送時系列で緩和後回復カーブを追わない。
- [226] - 緩和が10分遅れた場合、どのネットワーク感受性ユーザー指標が先に閾値を超えるか？
- [226] - 環境、負荷位相、セグメント範囲ごとに基線メモを1つ維持する。
- [226] - 分岐4: 一時回復後に再悪化。 -> retryとwake遅延のフィードバックループ確認。
- [226] - 分岐3: 対策後に両方悪化。 -> 即ロールバックし転送証拠スナップショットを保全。
- [226] - 分岐2: ネットワークトレンドは改善したが症状は不変。 -> CPUまたはストレージの並行連鎖を探索。
- [226] - 分岐1: 症状は改善したがネットワークトレンドは不変。 -> 転送因果レイヤー仮説を再検証。
- [226] - トラフィック一定でも、転送経路遷移はこのfieldを同方向に動かすか？
- [226] - まね調整を避け、転送経路文脈付きの before/after 証拠を要求する。
- [226] - このfieldが横ばいでも、どの非ネットワーク指標が症状を説明できるか？
- [226] - T+6m: socketまたはschedulerを含むクロスソース検証
- [226] - T+3m: セグメント前提付き初期仮説を明文化
- [226] - T+30m: 再送/遅延確認込みで回復確度を判定
- [226] - T+15m: 転送安定化観点でトレンド反応確認
- [226] - 1レイヤーしか計測できないなら、どのネットワーク隣接レイヤーが説明力を最も残すか？
- [226] ### 証拠品質ルーブリック（ネットワーク）
- [226] ### 時系列テンプレート（ネットワーク障害）

