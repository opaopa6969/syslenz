# ネットワークインターフェース統計

[English version](../en/sourceguide.net/dev.md)

---

## これは何？

/proc/net/devはインターフェースごとのパケットとバイトカウンターを提供します：受信/送信パケットとバイト、エラーと廃棄カウンターも含みます。ネットワークスループット監視の主要ソースです。

---

## 関連項目

- `net/snmp.Ip`
- `ss.tcp_established`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
