# sourceguide: vmstat

[English version](../en/sourceguide.vmstat.md)

---

## What is this source? / このソースとは？

`/proc/vmstat` はカーネルの仮想メモリイベントログです。特定のVMイベントが発生するたびにカーネルが加算する累積カウンタのフラットなリストです。現在の状態を示す `/proc/meminfo` と違い、`/proc/vmstat` はブート以来「どれだけの活動が起きたか」を示します。

カウンタはページ回収、ページフォールト、スワップI/O、ライトバック、コンパクション等をカバーします。現代のカーネルでは200以上のフィールドが存在します。健全なシステムでは大半がゼロ——だからこそゼロでないフィールドが重要です。

```
  /proc/vmstat: 累積イベントカウンタ（リセットされない）
  ┌─────────────────────────────────────────────────────┐
  │  nr_dirty          = 1842     （現在のスナップショット）│
  │  pgmajfault        = 38210    （ブート以来の累計）    │
  │  pswpin            = 0        （スワップインされたページ）│
  │  pswpout           = 0        （スワップアウトされたページ）│
  │  nr_writeback      = 0        （現在フラッシュ中）     │
  └─────────────────────────────────────────────────────┘
  
  レートを得るには：2回サンプリングして差分を取り、間隔で割る
  （vmstat 1 コマンドが自動でやってくれる）
```

おなじみの `vmstat 1` コマンドはここから読んで「毎秒デルタ」を表示します。`/proc/vmstat` がその生の累積ソースです。

---

## What questions does it answer? / 何がわかる？

- カーネルはページをスワップイン/アウトしているか？（`pswpin`、`pswpout`）
- メジャーページフォールトはどれくらい発生しているか？（`pgmajfault`）— 1回ごとにプロセスがディスク待ちでスタールする
- ダーティデータがフラッシュより速く蓄積しているか？（`nr_dirty`、`nr_writeback`）
- カーネルは積極的にメモリを回収しているか？（`pgsteal_kswapd`、`pgscand`）
- メモリ断片化によるコンパクション失敗が起きているか？（`compactfail`）

---

## Key fields to watch / 重点フィールド

| フィールド | 種別 | 意味 |
|---|---|---|
| `nr_dirty` | スナップショット | 現在のダーティページ数。高くて横ばい（フラッシュ停止）vs 高くて減少中（フラッシュ中）を区別する。 |
| `nr_writeback` | スナップショット | 現在ディスクに書き込み中のページ数。nr_dirtyが高いのにゼロ＝フラッシュがブロックされている。 |
| `pgmajfault` | カウンタ | ブート以来のメジャーページフォールト累計。レートのスパイク＝スワップまたはmmapへの圧力。 |
| `pswpin` / `pswpout` | カウンタ | ブート以来のスワップイン/アウトページ数。非ゼロのレートは匿名ページが追い出されていることを意味する。 |
| `pgsteal_kswapd` | カウンタ | kswapd（バックグラウンドスレッド）が回収したページ数。高レートは継続的なメモリ圧力を示す。 |
| `pgscand` | カウンタ | ダイレクト回収時にスキャンされたページ数。ダイレクト回収はフォールトを起こしたプロセスをブロックする——kswapd より痛い。 |

---

## How to read it directly / 直接読む方法

```sh
# 単発スナップショット
cat /proc/vmstat | grep -E 'nr_dirty|nr_writeback|pgmajfault|pswpin|pswpout'

# 2秒ごとのデルタ（定番ツール）
vmstat 2

# 特定カウンタの変化をウォッチ
watch -n 2 'grep -E "pgmajfault|pswpin|pswpout|nr_dirty" /proc/vmstat'
```

手動でレートを計算する場合：

```sh
# pgmajfault を10秒間隔で2回サンプリング
v1=$(grep pgmajfault /proc/vmstat | awk '{print $2}')
sleep 10
v2=$(grep pgmajfault /proc/vmstat | awk '{print $2}')
echo "pgmajfault レート: $(( (v2 - v1) / 10 )) /秒"
```

---

## A real episode / 実際のエピソード

JavaサービスがCPUもヒープも正常なのに、数分おきに200〜500msのレイテンシスパイクを出していた。`/proc/vmstat` を見ると、スパイク窓の間 `pgmajfault` が毎秒80〜120増加していた。システムが2GB空きと表示しているのに `pswpin` も非ゼロだった。

問題の核心：JVMのオフヒープメモリ（Luceneインデックスのmmapファイル）が、ページキャッシュによってRAMをほぼ全て占有された結果スワップに追い出されていた。`MemFree` はゼロではなかったが、`MemAvailable` は全く別の話を語っていた。JVMがそのmmapセグメントにアクセスするたびにスワップからフォールトバックが起き、1回あたり5〜15msの遅延を加えていた。

解決は `vm.dirty_ratio` でページキャッシュを制限し、アプリ側に `madvise(MADV_WILLNEED)` 呼び出しを追加してmmapリージョンをRAMに常駐するようカーネルにヒントを与えることだった。

---

## See also / 関連項目

- `sourceguide.meminfo` — 現在のメモリ状態（vmstatと組み合わせると全体像が見える）
- `vmstat.nr_dirty` — ダーティページ、フラッシュ閾値、フラッシュストームの詳細解説
- `sourceguide.pressure` — PSIメトリクス：プロセスが実際にメモリやI/Oで停止したか？
- `sourceguide.diskstats` — ディスクスループットとキュー深度（ライトバック活動との相関）
