# I/Oパス：システムコールからデバイスまで

[English version](../en/concept.io-path-syscall-to-device.md)

---

I/Oパスを理解するとレイテンシがどこに潜んでいるかの診断に役立ちます。

アプリケーション → システムコール → VFS → ファイルシステム → ページキャッシュ → ブロック層 → リクエストキュー → デバイスドライバー → ハードウェア

レイテンシ箇所：`stat.cpu_system`高/`iowait`低 → VFS/FS問題。`vmstat.nr_dirty`高 → フラッシュストーム。`diskstats.io_queue_depth_distribution`大 → ブロックキュー問題。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
