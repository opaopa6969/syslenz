# nr_free_cma

## NAME
`vmstat.nr_free_cma` - `vmstat` の指標シグナル（nr_free_cma）

## なぜ今読むか
vmstat `nr_free_cma` の reclaim/scan 挙動がノイジーで、短時間に防御可能な判断が必要な時に読みます。
`nr_free_cma` 周辺で reclaim圧力と副作用の切り分けが曖昧なら、調整前に証拠順序をここで固定します。

## 証拠の順序
1. 先にユーザー症状と時刻窓を固定する。
2. この記事の主シグナルのトレンドを確認する。
3. 隣接シグナル1つと跨層シグナル1つで照合する。
4. 可逆な緩和策を当て、回復トレンドを検証する。

## SEE ALSO
記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。

## メトリクス概要
- ID: `vmstat.nr_free_cma`
- Source: `vmstat` (`nr_free_cma`)
- Field: `nr_free_cma`
- 領域: 仮想メモリと reclaim 挙動
- シグナル分類: 容量余力とアロケータ圧力

## 運用上の意味（VMStat視点）
この field は、vmstat `nr_free_cma` の reclaim 主信号と副作用ノイズを、scan/reclaimトレンド分岐時に分離するのに最も有効です。

## フィールドエピソード（VMStat視点）
単一チャートでは余力があるように見えても、関連 field では不安定域へ向けた浸食が進んでいました。

`nr_free_cma` の実務価値は、証拠の時間順を整理することで出ます。何が先に動き、何が後追いし、対策後に何が変わったかを追ってください。

## 読み取りプロトコル（VMStat視点）
1. 現在の方向（上昇/下降/横ばい）と短期勾配を確認する。
2. `vmstat` の隣接 field と比較して単独判断を避ける。
3. 別 source から queue/stall/pressure 指標を1つ照合する。
4. 指標変化をユーザー影響（遅延・エラー・スループット）に結びつけてから対応する。

## 判断ヒューリスティクス（VMStat系）
この field と queue/stall 系が複数スナップショット連動するなら、一時ノイズではなく対応対象です。

## 避けるべき失敗パターン（VMStat系）
- 単一の vmstat reclaim/scan スナップショットで根因断定する。
- vmstat 時系列で緩和後回復カーブを追わない。
- vmstat 相関を因果と誤認する。

## アクションループ
1. `nr_free_cma` に対する反証可能な仮説を置く。
2. 可逆な緩和策を適用する。
3. 2〜3 の相関 field を複数リフレッシュで検証する。
4. ポストモーテム再利用のため証拠メモを残す。

## Unix内部システム視点

この field は **仮想メモリ状態遷移の発露** です。

- カーネル経路: reclaim、compaction、page fault、swap 制御。
- 主な契機: メモリ圧力やワークロード位相変化。
- 併読: scheduler 遅延と storage writeback 副作用。

## ケースブック（VMStat系）

### インシデント断面1（VMStat）
ケースB: クロスソース照合で初期診断が反転した。

### インシデント断面2（VMStat）
ケースC: 侵襲的変更より可逆対策で学習速度が上がった。

### インシデント断面3（VMStat）
ケースA: 絶対値ではなくトレンドが最初の異常を示した。

## 失敗分岐（VMStat系）
- 分岐1: 症状は改善したが vmstat トレンドは不変。 -> vmstat 因果レイヤー仮説を再検証。
- 分岐2: vmstat トレンドは改善したが症状は不変。 -> CPUまたはIOの並行連鎖を探索。
- 分岐3: 対策後に両方悪化。 -> 即ロールバックし vmstat 証拠スナップショットを保全。
- 分岐4: 一時回復後に再悪化。 -> vmstat reclaim と retry のフィードバックループ確認。

## ランブック演習（VMStat視点）
1. 15分の障害窓を選び、T0/T1/T2 の出来事を刻む。
2. 主field・隣接field・跨層fieldの3点連鎖を作る。
3. 反証可能な仮説1つと、ロールバック容易な対策1つを書く。
4. 成功条件を「トレンド回復 + ユーザー症状回復」で定義する。

## MANメモ（VMStat視点）
- この章は vmstat reclaim 分析向けman流（定義 -> scan/reclaim文脈 -> 失敗分岐 -> 証拠順）で構成しています。
- 前提時刻と vmstat reclaim 位相を明示してください。位相情報なしでは再利用性が下がります。
- 迷ったら本番で `nr_free_cma` に関わる vmstat 関連メモリノブ変更前に SEE ALSO を辿って証拠を増やしてください。

## 深掘り付録: 反事実分析とレビュー設問（VMStat系）

### 反事実クエスチョン（VMStat）
- トラフィック一定でも、vmstat の reclaim/scan 遷移はこのfieldを同方向に動かすか？
- このfieldが横ばいでも、どの非 vmstat 指標が症状を説明できるか？
- 緩和が10分遅れた場合、`nr_free_cma` 連動のどの vmstat ユーザー指標が先に閾値を超えるか？
- 1レイヤーしか計測できないなら、どの vmstat 隣接レイヤーが説明力を最も残すか？

### 時系列テンプレート（VMStat障害）
- T-10m: vmstat reclaim/scan 位相注記付き基線スナップショット
- T-5m: 最初の vmstat reclaim/scan 異常候補
- T0: vmstat 側文脈付きでユーザー症状確定
- T+3m: vmstat reclaim 前提付き初期仮説を明文化
- T+6m: scheduler または writeback を含むクロスソース検証
- T+10m: vmstat 経路でロールバック境界付き緩和策を適用
- T+15m: vmstat reclaim 安定化観点でトレンド反応確認
- T+30m: vmstat と pressure 確認込みで回復確度を判定

### 証拠品質ルーブリック（VMStat）
- 強い: 時系列順・跨層・vmstat reclaim整合トレンド
- 中程度: 相関はあるが vmstat reclaim/scan 境界が曖昧
- 弱い: vmstat reclaim/scan 文脈なしの一点値

### ポストモーテム設問（VMStat）
1. チーム判断を最も変えた証拠は何か？
2. 一見有力だったが二次要因だった指標は何か？
3. 暗黙の前提で次回は明示すべきものは何か？
4. 低ノイズでより早く発火できるアラートは何か？

### ドリフト防止チェック（VMStat）
- 環境、負荷位相、`nr_free_cma` を含む vmstat reclaimモードごとに基線メモを1つ維持する。
- リリース、カーネル、アロケータ挙動変更後は vmstat 関連閾値を再検証する。
- まね調整を避け、vmstat+pressure 文脈付きの before/after 証拠を要求する。
- この記事をランブックで隣接する vmstat または memory-path 記事2件以上へ接続する。

## 障害フォレンジクス

### 証拠取得
- 30分の時系列を取り、ユーザー影響アラート前の最初の reclaim/scan 変曲点を刻む。
- このfieldを scheduler系1つと writeback または swap 系1つで組み、跨層整合を検証する。

### 判断記録
- 主張: vmstat.nr_free_cma は意味のある状態遷移を示した。
- 反証試行: 代替原因を1つ立て、なぜ棄却したかを記録する。
- 行動メモ: nr_free_cma は単独判定ではなく、証拠連鎖の一部として扱った。

## manページ・クロスウォーク
- Process視点: Process: vmstat 変曲窓で runnable と blocked の膨張差を確認する。
- Syscall視点: Syscall: vmstat 遷移と整合する page-fault/メモリ接触呼び出しを特定する。
- Scheduler視点: Scheduler: vmstat スパイク時に reclaim待ちが wake遅延を増幅していないか検証する。
- Interrupt/IO視点: Storage/IO: vmstat 変動周辺で writeback/page-in 副作用を確認する。
- Fieldアンカー: nr_free_cma
- Sourceアンカー: vmstat

## ソース別ドリルブック（VMStat系）

### ドリル手順
1. 主field、隣接field、ユーザー症状の3画面を同期して20分リプレイする。
2. 閾値未達でもトレンド方向が変わった地点を1つ刻む。
3. その変化が reclaim圧力、割当位相変化、scheduler遅延のどれを示すか説明する。
4. ロールバック安全な対策を1つ書き、停止条件を先に定義する。
5. 初期仮説を棄却すべき証拠を先に明文化する。

### 振り返り設問
- どの vmstat 側手順が最も確信度を上げたか？
- reclaim圧力と副作用の不確実性を減らせなかった手順はどれか？
- 次回このドリルを速くする `nr_free_cma` 周辺の vmstat 計測改善は何か？

### アンカー（VMStat）
練習対象field: nr_free_cma

## 失敗類型マトリクス
- 類型A: 利用率は安全に見えるのに vmstat reclaim 圧力で stall が上がる状態。
- 類型B: vmstat reclaim 振動により短い回復と再悪化を繰り返す状態。
- 類型C: vmstat scan/reclaim 位相変化が遅れてユーザー影響へ出る状態。
- 注目field: nr_free_cma

## 反事実分岐
1. トラフィック一定でも、このfieldは同方向にドリフトするか？
2. reclaimが安定しても遅延が悪いなら、主因はどの非メモリ経路か？
3. メモリ側のどの観測が、現在の対策を即時に無効化するか？
