# 割り当てストール (allocstall) — vmstat

[English version](../en/vmstat.allocstall.md)

---

## これは何？

カーネルがメモリ割り当てのための空きページを見つけられないとき、**ダイレクトリクレーム**に入ります。ページキャッシュからページを同期的に解放するか匿名ページをスワップアウトしてから割り当てを満たします。この間、割り当てを要求したプロセスは*ブロック*されます。

`allocstall_*`はこれがメモリゾーンごとに何回発生したかをカウントします：

| メトリクス | ゾーン |
|-----------|--------|
| `allocstall_dma32` | DMA32ゾーン（物理4GB未満） |
| `allocstall_normal` | Normalゾーン（通常のRAM） |
| `allocstall_movable` | Movableゾーン（ホットプラグメモリ） |
| `allocstall_device` | Deviceゾーン（GPU/デバイスメモリ） |

各`allocstall`は**同期ストール**であり、アプリケーションはカーネルがメモリを解放する間フリーズします。`pressure/memory_some_avg10`が高い場合と組み合わせると、アプリケーションがストールされていることが確認できます。

---

## 関連項目

- `vmstat.compact` — compact_stall（ヒュージページ専用）
- `pressure.memory_some_avg10` — カーネル計測のメモリストールシグナル
- `meminfo.MemAvailable` — 利用可能メモリのヘッドルーム
- `sourceguide.vmstat` — vmstatソース全体の概要
