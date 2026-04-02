# カーネルスラブメモリ — meminfo

[English version](../en/meminfo.Slab.md)

---

## これは何？

カーネルは**スラブアロケーター**を使って、頻繁に使用されるカーネルオブジェクト（dentry、inode、ネットワークソケットバッファー等）を効率的に割り当て・キャッシュします。

| メトリクス | 内容 |
|-----------|------|
| `Slab` | スラブメモリ合計（SReclaimable＋SUnreclaim） |
| `SReclaimable` | 回収可能スラブ（dentryキャッシュ等） |
| `SUnreclaim` | 回収不可スラブ（ソケットバッファー等） |
| `KReclaimable` | 全回収可能カーネルメモリ |

`SReclaimable`が大きいのは正常です。`SUnreclaim`が増加し続ける場合は解放できないカーネルオブジェクトが蓄積しています（ネットワーク接続、大量のオープンファイル等）。

```sh
sudo slabtop -o | head -20
```

---

## 関連項目

- `meminfo.KReclaimable` — 全回収可能カーネルメモリ
- `sourceguide.meminfo` — meminfoソース全体の概要
