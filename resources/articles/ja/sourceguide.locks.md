# カーネルファイルロック

[English version](../en/sourceguide.locks.md)

---

## これは何？

/proc/locksはカーネルレベルのファイルロックを示します：POSIXロック（fcntl）、BSDロック（flock）、必須ロック。ロック競合やスタックしたプロセスのデバッグに役立ちます。

---

## 関連項目

- `processes.process_count`
- `sourceguide.vmstat` — vmstatメモリ統計
- `sourceguide.meminfo` — メモリ情報
