# memory_full_avg60

## NAME
`pressure.memory_full_avg60` - `pressure` の指標シグナル（memory_full_avg60）

## なぜ今読むか
pressure の stalled-time 挙動が `memory_full_avg60` 周辺でノイジーで、短時間に防御可能な判断が必要な時に読みます。
`memory_full_avg60` 周辺で stalled-time 主信号と副作用の切り分けが曖昧なら、調整前に証拠順序をここで固定します。

## 証拠の順序
1. 先にユーザー症状と時刻窓を固定する。
2. この記事の主シグナルのトレンドを確認する。
3. 隣接シグナル1つと跨層シグナル1つで照合する。
4. 可逆な緩和策を当て、回復トレンドを検証する。

## SEE ALSO
記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。

## メトリクス概要
- ID: `pressure.memory_full_avg60`
- Source: `pressure`
- Field: `memory_full_avg60`
- 領域: stall 蓄積とバックプレッシャー
- シグナル分類: stall 時間蓄積

## 運用上の意味（Pressure視点）
この field は、pressure の stalled-time 主信号と副作用ノイズを、some/fullトレンド分岐時に分離するのに最も有効です（`memory_full_avg60`）。

## フィールドエピソード（Pressure視点）
平均 pressure は許容でも、累積 stall 時間が tail latency 劣化を説明しました。

`memory_full_avg60` の実務価値は、証拠の時間順を整理することで出ます。何が先に動き、何が後追いし、対策後に何が変わったかを追ってください。

## 読み取りプロトコル（Pressure視点）
1. 現在の方向（上昇/下降/横ばい）と短期勾配を確認する。
2. `pressure` の隣接 field と比較して単独判断を避ける。
3. 別 source から queue/stall/pressure 指標を1つ照合する。
4. 指標変化をユーザー影響（遅延・エラー・スループット）に結びつけてから対応する。

## 判断ヒューリスティクス（Pressure系）
この field だけ上がり圧力系が平坦なら、容量障害よりワークロード形状変化を先に疑います。

## 避けるべき失敗パターン（Pressure系）
- 単一の pressure some/full スナップショットで根因断定する。
- pressure 時系列で緩和後回復カーブを追わない。
- pressure 相関を因果と誤認する。

## アクションループ
1. `memory_full_avg60` に対する反証可能な仮説を置く。
2. 可逆な緩和策を適用する。
3. 2〜3 の相関 field を複数リフレッシュで検証する。
4. ポストモーテム再利用のため証拠メモを残す。

## Unix内部システム視点

この field は **カーネル内 stall 時間蓄積の発露（`memory_full_avg60`）** です。

- カーネル経路（`memory_full_avg60`）: runnable 待ち、reclaim stall、I/O待ち連鎖。
- 主な契機（`memory_full_avg60`）: CPU%に出にくい資源競合。
- 併読（`memory_full_avg60`）: runqueue、reclaim、queue-depth。

## ケースブック（Pressure系）

### インシデント断面1（Pressure）
ケースA（利用率の罠）: CPU利用率は中程度でもPSI totalが継続増加。実行ではなく待機がSLO予算を消費。

### インシデント断面2（Pressure）
ケースB（短周期ストール）: 粗い可視化で見えない短stallがユーザー苦情時刻と一致。

### インシデント断面3（Pressure）
ケースC（跨層誤認）: ネットワーク起因に見えたが、先行シグナルはメモリ競合。

## 失敗分岐（Pressure系）
- 分岐1: 症状は改善したが pressure トレンドは不変。 -> pressure 因果レイヤー仮説を再検証。
- 分岐2: pressure トレンドは改善したが症状は不変。 -> CPUまたはIOの並行連鎖を探索。
- 分岐3: 対策後に両方悪化。 -> 即ロールバックし pressure 証拠スナップショットを保全。
- 分岐4: 一時回復後に再悪化。 -> pressure stall と retry のフィードバックループ確認。

## ランブック演習（Pressure視点）
1. 15分の障害窓を選び、T0/T1/T2 の出来事を刻む。
2. 主field・隣接field・跨層fieldの3点連鎖を作る。
3. 反証可能な仮説1つと、ロールバック容易な対策1つを書く。
4. 成功条件を「トレンド回復 + ユーザー症状回復」で定義する。

## MANメモ（Pressure視点）
- この章は pressure 分析向けman流（定義 -> stalled-time文脈 -> 失敗分岐 -> 証拠順）で構成しています。
- 前提時刻と pressure 位相を明示してください。位相情報なしでは再利用性が下がります。
- 迷ったら本番の pressure 関連ノブ変更前に SEE ALSO を辿って証拠を増やしてください。

## 深掘り付録: 反事実分析とレビュー設問（Pressure系）

### 反事実クエスチョン（Pressure）
- トラフィック一定でも、pressure some/full 遷移はこのfieldを同方向に動かすか？
- このfieldが横ばいでも、どの非 pressure 指標が症状を説明できるか？
- 緩和が10分遅れた場合、どの pressure 連動ユーザー指標が先に閾値を超えるか？
- 1レイヤーしか計測できないなら、どの pressure 隣接レイヤーが説明力を最も残すか？

### 時系列テンプレート（Pressure障害）
- T-10m: pressure some/full 注記付き基線スナップショット
- T-5m: 最初の pressure stalled-time 異常候補
- T0: pressure 側文脈付きでユーザー症状確定
- T+3m: pressure stalled-time 前提付き初期仮説を明文化
- T+6m: scheduler または容量系 source を含むクロスソース検証
- T+10m: pressure 経路でロールバック境界付き緩和策を適用
- T+15m: pressure stall 安定化観点でトレンド反応確認
- T+30m: pressure と症状確認込みで回復確度を判定

### 証拠品質ルーブリック（Pressure）
- 強い: 時系列順・跨層・pressure stalled-time 整合トレンド
- 中程度: 相関はあるが pressure some/full 境界が曖昧
- 弱い: pressure some/full 文脈なしの一点値

### ポストモーテム設問（Pressure）
1. チーム判断を最も変えた証拠は何か？
2. 一見有力だったが二次要因だった指標は何か？
3. 暗黙の前提で次回は明示すべきものは何か？
4. 低ノイズでより早く発火できるアラートは何か？

### ドリフト防止チェック（Pressure）
- 環境、負荷位相、pressure プロファイルごとに基線メモを1つ維持する。
- リリース、カーネル、アロケータ挙動変更後は pressure 関連閾値を再検証する。
- まね調整を避け、pressure+症状 文脈付きの before/after 証拠を要求する。
- この記事をランブックで隣接する pressure または memory-path 記事2件以上へ接続する。

## 障害フォレンジクス

### 証拠取得
- 30分の時系列を取り、ユーザー影響アラート前の最初の stalled-time 変曲点を刻む。
- このfieldを scheduler系1つと source固有の容量系1つで組み、跨層整合を検証する。

### 判断記録
- 主張: pressure.memory_full_avg60 は意味のある状態遷移を示した。
- 反証試行: 代替原因を1つ立て、なぜ棄却したかを記録する。
- 行動メモ: memory_full_avg60 は単独判定ではなく、証拠連鎖の一部として扱った。

## manページ・クロスウォーク
- Process視点: Process: pressure 変曲窓で runnable と blocked の膨張差を確認する。
- Syscall視点: Syscall: pressure stalled-time 遷移と整合する blocking 呼び出しを特定する。
- Scheduler視点: Scheduler: wait-to-run 膨張が pressure stalled-time を増幅していないか検証する。
- Interrupt/IO視点: Storage/IO: pressure 変動周辺で writeback/page-in 副作用を確認する。
- Fieldアンカー: memory_full_avg60
- Sourceアンカー: pressure

## ソース別ドリルブック（Pressure系）

### ドリル手順
1. 主field、隣接field、ユーザー症状の3画面を同期して20分リプレイする。
2. 閾値未達でもトレンド方向が変わった地点を1つ刻む。
3. その変化が reclaim圧力、割当位相変化、scheduler遅延のどれを示すか説明する。
4. ロールバック安全な対策を1つ書き、停止条件を先に定義する。
5. 初期仮説を棄却すべき証拠を先に明文化する。

### 振り返り設問
- どの pressure 側手順が最も確信度を上げたか？
- stalled-time と副作用の不確実性を減らせなかった手順はどれか？
- 次回このドリルを速くする pressure 計測改善は何か？

### アンカー（Pressure）
練習対象field: memory_full_avg60

## 失敗類型マトリクス
- 類型A: 利用率は安全でも pressure stalled-time が上がる静かな状態。
- 類型B: pressure some/full 振動で短い回復と再悪化を繰り返す状態。
- 類型C: pressure 位相変化が遅れてユーザー影響へ出る状態。
- 注目field: memory_full_avg60

## 反事実分岐
1. トラフィック一定でも、このfieldは同方向にドリフトするか？
2. reclaimが安定しても遅延が悪いなら、主因はどの非メモリ経路か？
3. メモリ側のどの観測が、現在の対策を即時に無効化するか？
