# ドライバー・カーネル・OS境界

[English version](../en/concept.driver-kernel-os-boundary.md)

---

問題がどの層で発生しているかを特定するために、異なる層で何が起こるかを理解することが役立ちます。

アプリケーション → システムコール → VFS → ファイルシステム → ブロック層 → デバイスドライバー → ハードウェア

`stat.cpu_iowait`が高くディスクキューが低い → ファイルシステム問題。ディスクキューが高くiowaitが低い → I/Oスケジューラーまたはドライバーの問題。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
