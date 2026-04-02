# ネットワークパス

[English version](../en/concept.network-path.md)

---

ネットワークパスを理解すると、ネットワーク問題の切り分けに役立ちます。

アプリケーション → ソケットバッファー → TCP層 → IP層 → NICドライバー → NICハードウェア → 物理リンク

問題箇所の確認：ソケットバッファー満杯 → `TcpExt_TCPBacklogDrop`、TCP輻輳 → `TcpExt_TCPTimeouts`、IPルーティング → `Ip_OutNoRoutes`

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
