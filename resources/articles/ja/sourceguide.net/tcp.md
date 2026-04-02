# TCP接続詳細

[English version](../en/sourceguide.net/tcp.md)

---

## これは何？

/proc/net/tcpは接続ごとのTCPソケット詳細を提供します：ローカル/リモートアドレス:ポート、状態、送受信キューサイズ、ソケットメモリ。すべてのTCP接続の生だが完全なビューです。

---

## 関連項目

- `ss.tcp_established`
- `net/sockstat.TCP_inuse`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
