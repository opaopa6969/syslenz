# sourceguide: meminfo

[English version](../en/sourceguide.meminfo.md)

---

## What is this source? / このソースとは？

`/proc/meminfo` はカーネルがリアルタイムで更新するメモリ会計台帳です。各行はカーネルのメモリ管理サブシステム（ページアロケータ、slabアロケータ、スワップ機構、ページキャッシュ）が遅延なく更新する名前付きカウンタです。

```
  物理RAM
  ┌─────────────────────────────────────────────┐
  │  カーネル（slab、ページテーブル等）           │
  │  ページキャッシュ（ファイルバック）← Cached  │
  │    └─ ダーティページ          ← Dirty        │
  │  匿名ページ（ヒープ/スタック） ← AnonPages   │
  │  空き                         ← MemFree      │
  └─────────────────────────────────────────────┘
        │                            │
    スワップアウト               スワップイン
        ▼                            ▲
  スワップ領域（ディスク）     ← SwapFree で残量を確認
```

ここに表示される数値は、カーネルがRAMについて「実際に知っている」ことです。監視エージェントが推測した値ではありません。

---

## What questions does it answer? / 何がわかる？

- 新しいアロケーションが実際に使える空きはどれくらいか？（`MemAvailable`）
- 匿名メモリをスワップにページアウトしているか？（`SwapFree` のトレンド）
- ページキャッシュの大きさは？ダーティデータが溜まっていないか？（`Cached`、`Dirty`）
- 匿名ページ（ヒープ、スタック、mmap）が予期せず増えていないか？（`AnonPages`）
- フリーリストやslabキャッシュにメモリが滞留していないか？（`MemFree`、`SReclaimable`）

---

## Key fields to watch / 重点フィールド

| フィールド | 意味 | なぜ重要か |
|---|---|---|
| `MemAvailable` | 空き＋回収可能メモリの推計 | 「実際に確保できるメモリ」を表す最も実用的な数値。MemFree単独より信頼できる。 |
| `AnonPages` | RAM上の匿名（非ファイル）ページ | 増加はヒープ/mmapの拡大。負荷下で減少するならスワップ圧力のサイン。 |
| `Cached` | ページキャッシュサイズ（ファイルバックページ） | 高くて正常。I/Oが吸収されている証拠。AnonPagesを圧迫していないか監視する。 |
| `Dirty` | 書き込み済みだが未フラッシュのページ | 高止まりは書き込みバックログ。スパイクはフラッシュストームの前兆。 |
| `SwapFree` | 残りスワップ空き容量 | 減少は匿名ページの追い出しを意味する。深刻なメモリ圧力のシグナル。 |
| `SReclaimable` | 回収可能なslabメモリ（dentry、inodeキャッシュ等） | 圧力下で解放できる。MemAvailable計算に含まれる。 |
| `Writeback` | 現在ディスクにフラッシュ中のページ | Dirtyと同時に高い → フラッシュ動作中。Dirtyが高くてゼロ → フラッシュ停止。 |

---

## How to read it directly / 直接読む方法

```sh
cat /proc/meminfo
```

64GBサーバーで軽負荷時の典型的な出力：

```
MemTotal:       65780736 kB
MemFree:         2341024 kB
MemAvailable:   58200448 kB
Buffers:           19840 kB
Cached:         56023552 kB
SwapCached:            0 kB
AnonPages:       4312064 kB
Dirty:              2304 kB
Writeback:             0 kB
SwapTotal:       8388604 kB
SwapFree:        8388604 kB
SReclaimable:    3201024 kB
```

注目フィールドだけをリアルタイムで監視する：

```sh
watch -n 1 'grep -E "MemAvailable|AnonPages|Dirty|Writeback|SwapFree" /proc/meminfo'
```

---

## A real episode / 実際のエピソード

毎晩バッチ処理ジョブが走り、それ自体は正常終了していた。しかし翌朝、ジョブ終了後20〜30分間レスポンスタイムが悪化するという問題があった。

`/proc/meminfo` を確認すると、バッチ実行中に `Cached` がRAMをほぼ全て占有していた。ジョブ終了後、Webサービスの `AnonPages` が再び増加し始めたが、その時点で `MemAvailable` が低く、アロケーションのたびにページキャッシュからの回収が発生していた。さらに `Dirty` がこの回収イベント中にスパイクしていた。バッチの書き込みが完全にフラッシュされていなかったためだ。

解決策はRAM追加ではなかった。`ionice` でバッチジョブのI/Oを制限してページキャッシュの独占を防ぎ、`vm.dirty_background_ratio` を調整してバッチ実行中により積極的にフラッシュさせた。この変更後、バッチ終了時の `MemAvailable` が4GB以上を維持するようになり、朝の遅延スパイクは消えた。

---

## See also / 関連項目

- `sourceguide.vmstat` — ページフォールト、スワップI/O、回収活動の累積カウンタ
- `sourceguide.pressure` — プロセスがメモリで実際にブロックされた時間（PSI）
- `vmstat.nr_dirty` — ダーティページ数と閾値、フラッシュストームの解説
- `sourceguide.swaps` — スワップ領域の一覧と使用量内訳
