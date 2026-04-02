# sourceguide: stat

[English version](../en/sourceguide.stat.md)

---

## What is this source? / このソースとは？

`/proc/stat` はカーネルのCPU時間会計ファイルです。ブート以来、各CPUが各モードで費やした時間の累積ティックカウントを記録します。割り込み、コンテキストスイッチ、プロセス生成のシステム全体カウンタも含みます。

```
$ cat /proc/stat
cpu  428291 0 183742 12847392 14832 0 3291 0 0 0
cpu0 107432 0 46038  3212832  3621  0 824  0 0 0
cpu1 106847 0 46209  3212432  3609  0 820  0 0 0
...
intr 48293847 ...
ctxt 29483920
btime 1711234567
processes 48291
procs_running 2
procs_blocked 0
```

最初の `cpu` 行は全CPUの合計です。各数値はUSER_HZティック（Linuxでは通常1/100秒）単位です。パーセンテージを得るには2回スナップショットを取り、差分を計算する必要があります。

```
  CPUタイムフィールド（順番に）：
  user      - ユーザーモードで費やした時間
  nice      - 低優先度（nice）のユーザーモード時間
  system    - カーネルモードで費やした時間
  idle      - アイドル時間
  iowait    - I/O完了待ち時間
  irq       - ハードウェア割り込み処理時間
  softirq   - ソフトウェア割り込み処理時間
  steal     - ハイパーバイザーが他のVMのために「奪った」時間  ← VM環境で重要
  guest     - 仮想CPUの実行時間
  guest_nice - 低優先度仮想CPUの実行時間
```

---

## What questions does it answer? / 何がわかる？

- CPU時間は実際にどこへ行っているか——アプリコード、カーネルコード、I/O待ち？
- 仮想化ホストでハイパーバイザーにCPU時間を奪われていないか？（`steal`）
- 今この瞬間、実行中またはI/Oブロック中のプロセスは何個か？（`procs_running`、`procs_blocked`）
- ユーザーモードと比べてカーネルモードの時間が多すぎないか？（`system` の比率）
- 新しいプロセスはどれくらいの速度で生成されているか？（`processes` カウンタのレート）

---

## Key fields to watch / 重点フィールド

| フィールド | 意味 | 警戒のタイミング |
|---|---|---|
| `user` | アプリケーションコードの実行時間 | 計算負荷の高いワークロードでは支配的であることが期待される |
| `system` | カーネル時間（syscall、スケジューリング） | 継続的に20%超はsyscall多用またはロック競合コードのサイン |
| `iowait` | CPUがI/O完了を待ってアイドルしていた時間 | 10%超はI/Oがボトルネック。diskstatsと相関させる。 |
| `steal` | ハイパーバイザーが他のVMに使った時間 | **VM環境でsteal%が継続するのはスロットルされている証拠。** 5〜10%のstealでもレイテンシジッターが起きる。 |
| `idle` | 本当のアイドル時間 | 低idle + 低steal + 低iowait ＝ CPU飽和 |
| `procs_blocked` | 現在D状態のプロセス数 | 非ゼロは割り込み不可待ち（多くはI/OまたはカーネルロックL）でスレッドが詰まっていることを意味する |

---

## How to read it directly / 直接読む方法

```sh
cat /proc/stat

# /proc/statからパーセンテージを計算する伝統的ツール
mpstat 1 5

# またはvmstatで
vmstat 1
# 列: us sy id wa st
#     user system idle iowait steal
```

2スナップショットから手動でCPU%を計算：

```sh
# 5秒間隔で2回読む
s1=$(grep '^cpu ' /proc/stat)
sleep 5
s2=$(grep '^cpu ' /proc/stat)
# 各フィールドを差し引き、非idle合計を全体合計で割ってbusy%を得る
```

クラウドVMでのsteal確認：

```sh
# vmstatでstealを確認
vmstat 1 | awk '{print "steal:", $17}'

# topでも確認できる：CPU行の%st
top -bn1 | grep '%Cpu'
```

---

## A real episode / 実際のエピソード

クラウドVM上で動くWebサービスが、CPU プロファイルが同じリクエストでもp99レイテンシが40〜80msばらつくという問題を抱えていた。アプリチームは徹底的にプロファイリングした——ホットパスなし、GCポーズなし、特殊なsyscallなし。

`/proc/stat` を確認すると、`steal` が終日12〜18%を示し、数分おきに30%超にスパイクしていた。このVMは「ノイジーネイバー」ホスト上にあった——同じ物理マシンの別テナントがI/O集中型のバッチジョブを実行しており、ハイパーバイザーが定期的にこのVMのCPUを横取りしていた。

解決策はアプリチューニングではなかった。VMを専有ホストインスタンス（CPU非共有）に移したところ、stealが0%に落ち、p99レイテンシは20ms未満に正常化した。シグナルは最初から `/proc/stat` にあった——誰のダッシュボードにも表示されていなかっただけで。

---

## See also / 関連項目

- `sourceguide.loadavg` — `/proc/stat` が説明する助けとなる派生ロードシグナル
- `sourceguide.pressure` — PSIはstatよりクリーンにCPUスケジューリング遅延とI/O停止を分離する
- `sourceguide.schedstat` — ランキュー待ち時間を含むCPU別スケジューラ統計
- `sourceguide.processes` — D状態数を含む現在のプロセス・スレッド状態
