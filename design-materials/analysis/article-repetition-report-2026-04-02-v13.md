# Article Repetition Report
- root: resources/articles
- generated_at: 2026-04-02T20:14:24+09:00

## en
### Raw (All Lines)
- nonempty lines: 88922
- unique lines: 8402
- duplicated lines: 80520
- duplication ratio: 90.55%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57619
- unique lines: 7057
- duplicated lines: 50562
- duplication ratio: 87.75%

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
- [226] - If uncertain, follow SEE ALSO links before changing production network knobs.
- [226] - Branch 4: Short recovery then relapse. -> Check retry and wake-delay feedback loops.
- [226] - Branch 3: Both worsen after mitigation. -> Roll back quickly and preserve transport evidence snapshot.
- [226] - Branch 2: Network-trend improves but symptom does not. -> Inspect parallel CPU or storage bottleneck chain.
- [226] - Branch 1: Symptom improves but network-trend does not. -> Revisit transport causal layer assumption.
- [226] ### Timeline Template (Network Incident)
- [226] ### Postmortem Questions (Network)
- [226] ### Incident Slice 3 (Network)
- [226] ### Incident Slice 2 (Network)
- [226] ### Incident Slice 1 (Network)
- [226] ### Evidence Quality Rubric (Network)
- [226] ### Counterfactual Questions (Network)
- [226] ### Anti-Drift Checklist (Network)
- [226] ### Anchor (Network)
- [226] ## Source Drillbook (Network Family)
- [226] ## Runbook Drill (Network Lens)
- [226] ## Reading Protocol (Network Lens)
- [226] ## Operational Meaning (Network Lens)
- [226] ## MAN Notes (Network Lens)
- [226] ## Field Episode (Network Lens)

## ja
### Raw (All Lines)
- nonempty lines: 88923
- unique lines: 8400
- duplicated lines: 80523
- duplication ratio: 90.55%

### Framework-Excluded (Signal Lines)
- nonempty lines: 57620
- unique lines: 7054
- duplicated lines: 50566
- duplication ratio: 87.76%

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
- [226] - 迷ったら本番ネットワークノブ変更前に SEE ALSO を辿って証拠を増やしてください。
- [226] - 分岐4: 一時回復後に再悪化。 -> retryとwake遅延のフィードバックループ確認。
- [226] - 分岐3: 対策後に両方悪化。 -> 即ロールバックし転送証拠スナップショットを保全。
- [226] - 分岐2: ネットワークトレンドは改善したが症状は不変。 -> CPUまたはストレージの並行連鎖を探索。
- [226] - 分岐1: 症状は改善したがネットワークトレンドは不変。 -> 転送因果レイヤー仮説を再検証。
- [226] ### 証拠品質ルーブリック（ネットワーク）
- [226] ### 時系列テンプレート（ネットワーク障害）
- [226] ### 反事実クエスチョン（ネットワーク）
- [226] ### ポストモーテム設問（ネットワーク）
- [226] ### ドリフト防止チェック（ネットワーク）
- [226] ### インシデント断面3（ネットワーク）
- [226] ### インシデント断面2（ネットワーク）
- [226] ### インシデント断面1（ネットワーク）
- [226] ### アンカー（ネットワーク）
- [226] ## 避けるべき失敗パターン（ネットワーク系）
- [226] ## 運用上の意味（ネットワーク視点）
- [226] ## 読み取りプロトコル（ネットワーク視点）
- [226] ## 深掘り付録: 反事実分析とレビュー設問（ネットワーク系）
- [226] ## 失敗分岐（ネットワーク系）
- [226] ## 判断ヒューリスティクス（ネットワーク系）

