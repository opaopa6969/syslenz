# Metric Grouping Classification (2026-04-02)

## Scope
- Snapshot basis: `/tmp/syslenz_cov_snapshot.json`
- Total fields observed: 592
- Goal: classify each field into:
  - `group化` (existing or candidate)
  - `個別記事`
  - `書きにくい` (汎用記事化しにくい)

## Headline
- 既存個別記事カバー: 30 / 592 (5.07%)
- 分類上「記事化可能」(existing + candidate): 552 / 592 (93.24%)
- 「書きにくい」(table/text中心): 40 / 592 (6.76%)

## Class Counts
- `candidate_individual`: 518
- `existing_individual`: 30
- `candidate_group_suffix`: 4
- `hard_table`: 30
- `hard_text`: 10

## Group化候補（今回の機械判定）
Resolver互換 (`*_min/*_max/*_count`, `*Min/*Max/*Count`) で判定。

- `conntrack.conntrack_count` -> group key: `conntrack.conntrack_distribution`
- `conntrack.conntrack_max` -> group key: `conntrack.conntrack_distribution`
- `net/snmp.Tcp_RtoMin` -> group key: `net/snmp.Tcp_Rto_distribution`
- `net/snmp.Tcp_RtoMax` -> group key: `net/snmp.Tcp_Rto_distribution`

提案:
- 上記2グループ記事を追加すれば、この4フィールドは group 経由で説明可能。

## 個別記事候補（優先度高）
数量が多い source から優先的に個別化。

- `vmstat`: candidate_individual 160
- `net/netstat`: candidate_individual 143
- `net/snmp`: candidate_individual 78
- `meminfo`: candidate_individual 50
- `pressure`: candidate_individual 20

## 書きにくい項目
### hard_table (30)
例:
- `processes.processes`
- `diskstats.devices`
- `net/tcp.connections`
- `net/udp.sockets`
- `mounts.mounts`

方針:
- 個別メトリクス記事より、source全体の読み方記事やテーブル列ガイド化が適切。

### hard_text (10)
例:
- `cmdline.cmdline`
- `version.raw`
- `version.kernel_version`
- `cpuinfo.model`

方針:
- 値そのものはホスト依存が強いため、評価軸（どう見るか）記事に寄せる。

## 200記事で90%に届くか
結論: **届く可能性は高い**。

理由:
- 今回分類の「記事化可能」上限は 552 / 592 = 93.24%
- hard 領域が 40 フィールドに収束しているため
- group戦略を増やしつつ、個別記事を高密度 source (vmstat/netstat/snmp/meminfo) に集中すれば、
  200記事前後で運用上90%超は現実的

## Output Files
- Full classification TSV:
  - `design-materials/analysis/generated/field_classification_2026-04-02.tsv`
- Class counts:
  - `design-materials/analysis/generated/class_counts_2026-04-02.tsv`
- Parsed metric IDs:
  - `design-materials/analysis/generated/metric_ids_2026-04-02.txt`
- Parsed group IDs:
  - `design-materials/analysis/generated/group_ids_2026-04-02.txt`
