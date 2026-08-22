---
name: multi-host-monitoring
description: 複数ホストを syslenz で監視する手順
volta:
  version: 1
  namespace: syslenz
  locality: service
  applies_when: 複数ホストの監視を設定するとき
  requires: syslenz MCP バックエンドが ready
  min_role: MEMBER
  export: true
---

# 複数ホストを syslenz で監視する

## 概要

syslenz は `--serve` モード（TCP サーバ、ポート 9100）と `--connect` モード（リモートホストからメトリクスを受信）で複数ホスト監視をサポートする。各ホストで syslenz を起動し、中央ホストで集約する。

## 手順

1. **リモートホストの設定**: 各監視対象ホストで `syslenz --serve 0.0.0.0:9100` を起動する。
   - Docker の場合は `--pid=host` が必要。
2. **中央ホストの設定**: `syslenz --connect <remote_host>:9100` でリモートホストからメトリクスを受信。
3. **MCP 経由の監視**: 中央ホストの `syslenz__snapshot` で全ホストの状態を取得。
4. **syslenz4j の活用**: JVM メトリクスは syslenz4j（Java エージェント）で収集し、syslenz の TCP プロトコルで送信。

## 注意

- MCP エンドポイント（`/mcp`）は Web サーバ（`--web`）モードでのみ提供。TCP サーバ（`--serve`）モードには MCP はない。
- TCP プロトコル（SNAPSHOT/METRICS/QUIT）は syslenz4j が依存する。MCP 化の影響を受けない。
