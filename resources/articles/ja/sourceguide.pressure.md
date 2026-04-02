# sourceguide: pressure

[English version](../en/sourceguide.pressure.md)

---

## What is this source? / このソースとは？

`/proc/pressure/` は `cpu`、`memory`、`io` の3ファイルを含むディレクトリで、それぞれ **PSI（Pressure Stall Information）** メトリクスを公開します。Linux 4.20以降で利用可能です。

PSIはロードアベレージが答えられない問いに答えます：**リソースが利用できなかったせいで、プロセスが前進できなかった時間はどれくらいか？** キューのスレッド数を数えるのではなく、リソースの停滞によって失われた実時間をスライディングウィンドウの割合（%）で計測します。

```
  /proc/pressure/memory:
  some avg10=0.34 avg60=0.12 avg300=0.05 total=183429
  full avg10=0.00 avg60=0.00 avg300=0.00 total=0

       │     │         │         │          │
       │     │         │         │          └─ ブート以来の累計マイクロ秒
       │     │         │         └─ 5分スライディングウィンドウ（%）
       │     │         └─ 1分スライディングウィンドウ（%）
       │     └─ 10秒スライディングウィンドウ（%）
       └─ "some" 行：少なくとも1タスクが停止していた
          "full" 行：全ての実行可能タスクが同時に停止していた
```

**some vs full の違い：**
- `some`：少なくとも1つのプロセスがこのリソースを待っていた。システムは前進しているが、全員が進めているわけではない。
- `full`：全ての実行可能プロセスが同時にこのリソースでブロックされた。どこでも前進ゼロ。深刻な状態。

---

## What questions does it answer? / 何がわかる？

- システムはCPU・メモリ・I/Oで実際に停止しているか、単に「忙しい」だけか？（`some` 行）
- あるリソースで「一切の作業が進まない」状態に達したか？（`full` 行）
- 直近10秒と直近5分を比べて圧力は改善しているか悪化しているか？（avg10 vs avg300）
- ロードアベレージが高いのはCPU競合かI/Oブロックか？（`cpu` と `io` のPSIを比較）

---

## Key fields to watch / 重点フィールド

| ファイル | メトリクス | アラートのサイン |
|---|---|---|
| `cpu` | `some avg10` | CPUスケジューリング遅延。10%超が継続＝CPUオーバーサブスクリプション。 |
| `memory` | `some avg10` | メモリ待ちのプロセス。1〜2%の継続でも警戒サイン。 |
| `memory` | `full avg10` | 全タスクがメモリでブロック。非ゼロはどんな値でも深刻。 |
| `io` | `some avg10` | I/O待ち。書き込み負荷の高いワークロードで高くなる。diskstatsと相関させる。 |
| `io` | `full avg10` | 完全なI/O停滞。非ゼロはストレージ層がボトルネックであることを意味する。 |

**PSIはアラートにおいてロードアベレージより優れています。** ロードアベレージはCPU圧力とI/O圧力を1つの数値に混在させます。PSIはリソースごとに分離し、キュー長ではなく実際の停止時間を計測します。

---

## How to read it directly / 直接読む方法

```sh
cat /proc/pressure/cpu
cat /proc/pressure/memory
cat /proc/pressure/io
```

3つをまとめてウォッチ：

```sh
watch -n 2 'echo "=== CPU ===" && cat /proc/pressure/cpu && echo "=== Memory ===" && cat /proc/pressure/memory && echo "=== I/O ===" && cat /proc/pressure/io'
```

PSI閾値はファイルディスクリプタ通知の仕組みを使ってカーネルイベントをトリガーできます——cgroupレベルのアラートに便利：

```sh
# memory pressure some avg10が500ms間5%を超えたらアラート
echo "some 5000 500000" > /proc/pressure/memory
```

---

## A real episode / 実際のエピソード

Postgresデータベースホストのロードアベレージが4.2で、CPU数も4——教科書通りの「完全利用」に見えた。オンコールチームはCPU容量の追加を検討していた。プロビジョニングの前に誰かがPSIを確認した：

```
cpu:    some avg10=1.2  avg60=0.8   （CPUスケジューリング遅延は無視できる水準）
memory: some avg10=18.4 avg60=14.1  （メモリ停止が深刻！）
io:     some avg10=22.1 avg60=19.8  （I/Oも停止している）
```

ロードアベレージ4.2のほぼ全てがI/OとメモリのスタールであってCPU飽和ではなかった。ロードアベレージを押し上げていたD状態スレッドは、メモリ回収とディスク書き込みでブロックしていた。CPU追加は何の効果もなかったはずだ。

実際の解決策：Postgresの `shared_buffers` 設定が小さすぎて、常にページ追い出しとディスクからの再読み込みが発生していた。256 MBから4 GBに増やすと `memory some` は18%から0.3%に、`io some` は22%から4%に下がった。

---

## See also / 関連項目

- `sourceguide.meminfo` — メモリ圧力が発生している理由を説明するメモリ状態カウンタ
- `sourceguide.vmstat` — メモリPSIを引き起こすページ回収とスワップ活動
- `sourceguide.loadavg` — PSIが改善を目指した古いシグナル
- `sourceguide.diskstats` — io PSIと相関するデバイス別I/Oメトリクス
