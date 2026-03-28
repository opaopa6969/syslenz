# DGE Session 014: クロスプラットフォーム MetricEnum — 文字列ペアからの脱却と型安全なメトリクス体系

- **Date**: 2026-03-28
- **Theme**: 文字列ベースのメトリクス識別 (`("meminfo", "MemTotal")`) を型安全な enum 体系に移行し、Linux/macOS/Windows のメトリクス対応表を構築する
- **Parent Gaps**: G-TYPE-1 (メトリクス識別の型安全性), G-XPLAT-1 (クロスプラットフォームメトリクスマッピング), G-DISC-1 (メトリクスの発見可能性)
- **Characters**: 千石 (品質の番人) + ヤン (怠惰な簡潔主義者) + リヴァイ (実装の鬼) + 今泉 (初心者の代弁者) + ラインハルト (ビジョナリー) + 僕 (スコープ削減係)
- **Input**: Linux 50+ ソース (`/proc/*`, `/sys/*`, `ip`, `ss`, `dns`, `conntrack`)、macOS 14+ ソース (`platform_macos.rs`: `sysctl`, `vm_stat`, `df`, `thermal`, `battery` 等 23 エントリ)、Windows 13+ ソース (`platform_windows.rs`: `Win32_*`, `PowerShell`, `perf_*` 等 23 エントリ、将来 24+ 予定)。現状のメトリクス識別は `AlertRule { source: String, field: String }` — 全て文字列ペア。`education.rs` の `Category` enum は 6 バリアントで「ソースの分類」のみ、個別メトリクスの型安全性なし。

---

## 現状の整理

先輩 (ナレーション): メトリクス識別の現在地を整理する。

### 現行アーキテクチャの問題

```rust
// 現状: alert.rs — 全てが文字列
pub struct AlertRule {
    pub source: String,    // "meminfo" — typo しても実行時まで分からない
    pub field: String,     // "MemTotal" — 大文字小文字も自由
    pub condition: String, // "> 90.0"
    pub severity: String,  // これも文字列...
    pub message: String,
}

// 現状: proc/mod.rs — データ構造も文字列
pub struct Field {
    pub name: String,        // "MemTotal" — 同じフィールドが3プラットフォームで別名
    pub value: FieldValue,
    pub unit: Option<String>, // "kB" — これも文字列
    pub description: String,  // パーサーごとにハードコード
}
```

### プラットフォーム間の対応関係 (現状は暗黙的)

同じ概念が3つのプラットフォームで異なるソース名・フィールド名を持つ:

| 概念 | Linux ソース/フィールド | macOS 実装 | Windows 実装 |
|------|----------------------|-----------|-------------|
| 総メモリ | `meminfo/MemTotal` | `hw.memsize` (sysctl) | `Win32_ComputerSystem.TotalPhysicalMemory` |
| 利用可能メモリ | `meminfo/MemAvailable` | `vm_stat` (free+inactive pages) | `Win32_OperatingSystem.FreePhysicalMemory` |
| CPU使用率 | `stat/cpu_usage_pct` | `top -l 1` CPU% | `\Processor\% Processor Time` |
| ロード平均 | `loadavg/load_1min` | `vm.loadavg` (sysctl) | `\System\Processor Queue Length` (近似) |
| ディスク使用率 | `df/root_use_pct` | `df /` | `Get-PSDrive C:` |
| CPU温度 | `thermal/max_temp` | `smc`/`xcpm` | `MSAcpi_ThermalZoneTemperature` |
| TCP接続 | `net/tcp` | `netstat`/`lsof` | `Get-NetTCPConnection` |
| プロセス数 | `processes/process_count` | `ps aux` | `Get-Process` |
| FD使用率 | `file-nr/fd_usage_pct` | `kern.num_files` (sysctl) | `HandleCount` |
| 稼働時間 | `uptime/uptime` | `kern.boottime` (sysctl) | `GetTickCount64` |
| スワップ使用量 | `meminfo/SwapTotal-SwapFree` | `vm_stat` swapused | `PageFile` |
| バッテリー | N/A (ラップトップ) | `pmset -g batt` | `Win32_Battery` |
| サービス | N/A (systemd 将来) | `launchctl list` | `Get-Service` |

### プラットフォーム固有メトリクス (マッピング不能)

- **Linux のみ**: `/proc/pressure` (PSI), `buddyinfo`, `slabinfo`, `conntrack`, `cgroups`, `pagetypeinfo`, `zoneinfo`, `schedstat`, `softirqs`, `iomem`, `ioports`
- **macOS のみ**: `system_profiler`, `kextstat` (kernel_extensions), `diskutil`, `power_management`, `software_update`
- **Windows のみ**: `eventlog`, `hotfix`, `scheduled_tasks`, `firewall`, `handles`, `perf_cpu`, `perf_memory`, `perf_disk`, `volumes`, `dns_cache`

---

## Scene 1: MetricKind の設計 — カテゴリの粒度と階層

先輩: メトリクスのカテゴリ分類について議論する。

🎋 千石 (ホワイトボードの前で): 「まず `education.rs` の `Category` を見ろ。Memory, CpuLoad, Network, Storage, Process, Hardware の6つだ。これは "ソースの分類" だった。今回は "メトリクスの分類" だ。同じ6つでいいのか？」

🪐 ラインハルト: 「足りない。System (uptime, version, boottime), Security (firewall, crypto), Power (battery, power_management) は独立したカテゴリだろう。syslenz を使う人が "バッテリーは Hardware だっけ？ Storage だっけ？" と迷わない分類が必要だ。」

👤 今泉: 「あの... 今 Category は 6 つで、related_sources() で各カテゴリに紐付くソースが定義されてますよね。MetricKind を別に作ると、Category と MetricKind の関係はどうなるんですか？ 同じもの？ 別のもの？」

🎋 千石: 「良い質問だ。Category はUI上の "学習パス" のための分類。MetricKind は **メトリクス自体の属性**。重なるが同じものではない。例えば pressure ソースは Category::Memory にも Category::CpuLoad にも属している。だが pressure の memory_some_avg10 は MetricKind::Memory で、cpu_some_avg10 は MetricKind::Cpu だ。**ソースの分類** と **フィールドの分類** は粒度が違う。」

⚔️ リヴァイ: 「階層は要らん。Memory > Swap > SwapCached みたいな階層を入れると、enum のマッチが地獄になる。フラットな enum + `fn is_memory_related(&self) -> bool` で十分だ。」

☕ ヤン (椅子に深く座って): 「10個以下にしてくれ。カテゴリが多すぎると誰も覚えない。」

🎋 千石: 「8つだ。Memory, Cpu, Network, Storage, Process, System, Power, Security。Hardware は Cpu と Power に分割する。thermal は Cpu に入れる — CPU温度を見る文脈で使うからだ。バッテリーと電源管理は Power。firewall と crypto は Security。これでカバーできないものは System に入れる。」

⚔️ リヴァイ: 「異論なし。フラットな8バリアント。各バリアントに display name と icon を持たせる。education.rs の Category もこの8つに統一するか、MetricKind を参照する形に変える。」

僕: 「education.rs の Category は今回変更しない。MetricKind を定義して、将来 Category と統合可能にするだけだ。」

**合意: MetricKind は 8 バリアントのフラット enum。階層なし。**

```
MetricKind = Memory | Cpu | Network | Storage | Process | System | Power | Security
```

---

## Scene 2: MetricField enum の設計 — プラットフォーム別 vs 統一型

先輩: メトリクスの個別フィールドを enum でどう表現するかを議論する。

⚔️ リヴァイ: 「選択肢は3つある。」

```
// 案A: プラットフォーム別 enum
enum LinuxMetric { MeminfoMemTotal, MeminfoMemFree, StatCpuUsagePct, ... } // 500+
enum MacMetric { HwMemsize, VmStatFreePages, TopCpuPercent, ... }         // 100+
enum WindowsMetric { TotalPhysicalMemory, FreePhysicalMemory, ... }       // 150+

// 案B: 統一 enum + platform flag
enum MetricField { MemTotal, MemFree, CpuUsagePct, ... } // 600+ (全プラットフォーム合算)
impl MetricField { fn platforms(&self) -> &[Platform] }

// 案C: ソース + フィールド の2層 enum
enum MetricSource { Meminfo, Stat, Loadavg, ... }
enum MeminfoField { MemTotal, MemFree, MemAvailable, ... }
enum StatField { CpuUsagePct, ContextSwitches, ... }
```

☕ ヤン: 「案C は enum が 50 個になる。正気か？ 案B がいい。1つの enum で全部。プラットフォームごとの差は `#[cfg]` で variant を出し分ける。」

🎋 千石: 「案B で 600+ variant。コンパイル時間が死ぬぞ。」

⚔️ リヴァイ: 「600 variant の enum は match の exhaustiveness check で遅くなるが、致命的ではない。問題は **description, kind, unit を全 variant に持たせるかどうか** だ。」

👤 今泉: 「えっと、今 `i18n.rs` に `field_description()` があって 3 層の説明 (normal, detailed, extra_detailed) を返してますよね。これが enum に入るんですか？」

⚔️ リヴァイ: 「入れない。enum の variant は **識別子** だ。description は i18n の仕事。enum には kind と unit だけ持たせる。」

```rust
// 決定: プラットフォーム別 enum (案A をベースに整理)
// 各プラットフォームの enum は独立。CommonMetric で橋渡しする。

pub enum LinuxMetric {
    // meminfo
    MeminfoMemTotal,
    MeminfoMemFree,
    MeminfoMemAvailable,
    MeminfoBuffers,
    MeminfoCached,
    MeminfoSwapTotal,
    MeminfoSwapFree,
    // stat
    StatCpuUsagePct,
    StatContextSwitches,
    StatProcessesCreated,
    // loadavg
    LoadavgLoad1min,
    LoadavgLoad5min,
    LoadavgLoad15min,
    // ... 500+ more
}

pub enum MacMetric {
    HwMemsize,
    VmStatFreePages,
    VmStatActivePages,
    VmStatInactivePages,
    // ... 100+ more
}

pub enum WindowsMetric {
    TotalPhysicalMemory,
    FreePhysicalMemory,
    ProcessorPercentTime,
    // ... 150+ more
}
```

🎋 千石: 「案A にする理由を明確にしろ。」

⚔️ リヴァイ: 「理由は3つ。1) プラットフォームごとにコンパイルされるのでバイナリサイズが膨らまない — `#[cfg(target_os = "linux")]` で他プラットフォームの variant は消える。2) 各プラットフォームの variant 名がそのプラットフォームのネイティブ概念を反映する — Windows 開発者に `MeminfoMemTotal` を見せても意味がない。3) CommonMetric で必要な部分だけ橋渡しする — 全 variant をマッピングする必要がない。」

☕ ヤン: 「案A のデメリットは、trait を実装するコードが3回必要になること。」

⚔️ リヴァイ: 「自動生成で解決する。Scene 5 で議論する。」

👤 今泉: 「description はどこに行くんですか？ 今は各パーサーに `describe_meminfo_field()` みたいな関数がハードコードされてますよね。」

⚔️ リヴァイ: 「`i18n.rs` の `field_description()` に一本化する。enum の variant から source+field 文字列を逆引きして、既存の field_description() を呼ぶ。将来的には enum variant から直接 i18n キーを引くようにする。」

**合意: プラットフォーム別 enum (案A)。description は i18n に委任。enum には kind() と unit() のメソッドのみ。**

---

## Scene 3: CommonMetric マッピングの精度問題 — "同じ" とは何か

先輩: クロスプラットフォームのメトリクス対応が本当に正確かを検証する。

👤 今泉 (手を挙げて): 「根本的な疑問があるんですけど... Linux の MemAvailable と Windows の FreePhysicalMemory って **本当に同じもの** なんですか？」

⚔️ リヴァイ: 「違う。」

👤 今泉: 「えっ。」

⚔️ リヴァイ: 「Linux の MemAvailable は "新しいプロセスに割り当て可能なメモリ量" だ。MemFree + 回収可能なキャッシュ + reclaimable Slab を含む。Windows の FreePhysicalMemory は Win32_OperatingSystem の値で、standby list を含むかどうかは OS バージョンによる。macOS は vm_stat の free + inactive pages で計算しているが、compressed pages の扱いが Linux と違う。**3つとも "利用可能メモリ" の近似値だが、定義が違う。**」

👤 今泉: 「じゃあ CommonMetric::MemAvailable って嘘じゃないですか？」

🎋 千石: 「嘘ではないが、不正確だ。CommonMetric は "同一の計測" ではなく "同一の概念" だと明示する必要がある。ドキュメントなしで MemAvailable をプラットフォーム間で比較したら、ユーザーを誤解させる。」

🪐 ラインハルト: 「confidence level を付けよう。Exact — 同じ定義。Comparable — 同じ概念、微差あり。Approximate — 近いが注意が必要。」

```rust
pub enum MappingConfidence {
    /// 定義が同一 (例: Uptime — 全プラットフォームでブートからの秒数)
    Exact,
    /// 同じ概念、軽微な定義差 (例: MemTotal — 全プラットフォームで物理メモリ総量)
    Comparable,
    /// 近似。比較には注意が必要 (例: MemAvailable — OS間で計算方法が異なる)
    Approximate,
}
```

👤 今泉: 「Processor Queue Length を Load Average の代わりにするのは Approximate ですよね？」

⚔️ リヴァイ: 「それ以下だ。Processor Queue Length は "現在キューにあるスレッド数" で、Linux の load average は "実行中 + 待機中のプロセスの指数移動平均" だ。単位も時間窓も違う。これは Approximate ではなく **マッピング不能** とすべきだ。」

🎋 千石: 「それならマッピングテーブルに入れるな。`CommonMetric::Load1` は Linux と macOS のみ。Windows はマッピングなし。**無理にマッピングして品質を落とすな。**」

☕ ヤン: 「CommonMetric は 15 個でいい。全部マッピングするな。確実に対応するものだけ。残りはプラットフォーム固有メトリクスとして扱う。」

⚔️ リヴァイ: 「プラットフォーム固有メトリクスの扱いを決めろ。CommonMetric に含まれない LinuxMetric variant は、Java 側でどう扱う？ syslenz4j から使えなくていいのか？」

🪐 ラインハルト: 「使える必要がある。CommonMetric はショートカットだ。プラットフォーム固有のメトリクスにも MetricField 経由でアクセスできなければ syslenz の価値が半減する。syslenz で Linux を学んだ人が、そのまま Windows でも **同じ知識構造** でメトリクスにアクセスできる。CommonMetric で 15 個の共通言語、プラットフォーム enum で全メトリクスへの型安全アクセス。両方必要だ。」

**合意:**
- CommonMetric は 15 個以下。confidence level 付き。
- Approximate 以上のマッピングのみ収録。無理な近似は含めない。
- プラットフォーム固有メトリクスは各プラットフォーム enum で直接アクセス。
- Load1 の Windows マッピングは **除外**。

---

## Scene 4: Metric trait/interface の設計 — Rust と Java の統一インターフェース

先輩: 全プラットフォーム enum が共通で実装する trait と、Java 側の interface を設計する。

⚔️ リヴァイ: 「Rust 側。」

```rust
pub trait Metric: Copy + Eq + std::hash::Hash {
    /// ソース名 (例: "meminfo", "stat")
    fn source(&self) -> &'static str;

    /// フィールド名 (例: "MemTotal", "cpu_usage_pct")
    fn field(&self) -> &'static str;

    /// カテゴリ
    fn kind(&self) -> MetricKind;

    /// 単位 (Bytes, Percent, Count, Seconds, Celsius, None)
    fn unit(&self) -> MetricUnit;

    /// i18n 記述を引くためのキー (source/field 形式)
    fn i18n_key(&self) -> String {
        format!("{}/{}", self.source(), self.field())
    }

    /// CommonMetric への変換 (対応がない場合 None)
    fn common(&self) -> Option<CommonMetric>;

    /// Snapshot からこのメトリクスの値を取得
    fn extract(&self, snapshot: &Snapshot) -> Option<&FieldValue>;
}
```

☕ ヤン: 「extract は trait に入れるのか？ Snapshot への依存が入るぞ。」

⚔️ リヴァイ: 「入れる。これが MetricField の存在意義だ。文字列ペアで `snapshot.entries["meminfo"].fields.iter().find(|f| f.name == "MemTotal")` と書いていたのが `LinuxMetric::MeminfoMemTotal.extract(&snapshot)` で済む。型安全。」

🎋 千石: 「extract の戻り値は `Option<&FieldValue>` か？ そのメトリクスが取得に失敗したときの扱いは？」

⚔️ リヴァイ: 「None を返す。そのソースが Snapshot に含まれていない、またはそのフィールドが存在しない場合。Alert 判定では None を "データなし" として扱い、発火しない。」

👤 今泉: 「Java 側は？ syslenz4j で Watch API を使うとき、メトリクスをどう指定するんですか？」

⚔️ リヴァイ: 「Java 側。」

```java
public interface Metric {
    String source();
    String field();
    MetricKind kind();
    MetricUnit unit();
    String i18nKey();
    Optional<CommonMetric> common();
}

// プラットフォーム別 enum が実装
public enum LinuxMetric implements Metric {
    MEMINFO_MEM_TOTAL("meminfo", "MemTotal", MetricKind.MEMORY, MetricUnit.BYTES),
    MEMINFO_MEM_FREE("meminfo", "MemFree", MetricKind.MEMORY, MetricUnit.BYTES),
    MEMINFO_MEM_AVAILABLE("meminfo", "MemAvailable", MetricKind.MEMORY, MetricUnit.BYTES),
    // ...
    ;
    // constructor, fields, Metric implementation
}

// Watch API での使用
syslenz.watch(LinuxMetric.MEMINFO_MEM_AVAILABLE, value -> {
    if (value.asBytes() < threshold) { alert(); }
});

// CommonMetric での使用 (プラットフォーム非依存)
syslenz.watch(CommonMetric.MEM_AVAILABLE, value -> {
    // Linux でも macOS でも Windows でも動く
});
```

🪐 ラインハルト: 「ここが重要だ。Alert config で MetricField を参照可能にする。」

```yaml
# 現状: 文字列ベース (typo しても分からない)
alerts:
  - source: "meminfo"
    field: "MemAvailable"
    condition: "< 500000000"

# 将来: MetricField 参照 (バリデーション可能)
alerts:
  - metric: "linux:meminfo/MemAvailable"  # または common:MemAvailable
    condition: "< 500MB"
    severity: critical
```

⚔️ リヴァイ: 「config ファイルでは文字列が避けられないが、パース時に MetricField に変換してバリデーションする。未知のメトリクス名は起動時にエラー。」

**合意:**
- Rust: `trait Metric` に `source()`, `field()`, `kind()`, `unit()`, `common()`, `extract()` を定義
- Java: `interface Metric` に同等のメソッド
- Alert config はパース時にメトリクス名をバリデーション
- Watch API は `Metric` trait object / interface で受け取る

---

## Scene 5: 自動生成パイプライン — ソースオブトゥルースと生成物

先輩: enum と interface を手書きせず自動生成する仕組みを設計する。

☕ ヤン: 「手で 600 variant を書くのは論外だ。ソースオブトゥルースはどこにする？」

⚔️ リヴァイ: 「候補は3つ。1) パーサーの実装コード (各 .rs ファイルの describe_xxx_field 関数)。2) i18n.rs の field_description エントリ。3) 別途定義する TOML/YAML ファイル。」

🎋 千石: 「1 と 2 は既にずれている。describe_meminfo_field は meminfo.rs にハードコードされていて、i18n.rs の field_description はサブセットしかない。パーサーの実装が真実だが、コードからの抽出は脆い。」

⚔️ リヴァイ: 「3 にしろ。TOML で全メトリクスを定義する。パーサーもi18nもこのファイルから生成する。」

```toml
# metrics/linux.toml — ソースオブトゥルース
[[metrics]]
source = "meminfo"
field = "MemTotal"
kind = "Memory"
unit = "Bytes"
description.en = "Total usable RAM"
description.ja = "利用可能なRAMの合計"
description_detail.en = "Total amount of physical RAM, minus reserved bits and kernel binary code"
description_detail.ja = "物理RAMの合計量。予約領域とカーネルバイナリコードを除く"

[[metrics]]
source = "meminfo"
field = "MemAvailable"
kind = "Memory"
unit = "Bytes"
description.en = "Available memory for new processes without swapping"
description.ja = "スワップなしで新規プロセスに利用可能なメモリ量"
# ...
```

```toml
# metrics/common.toml — クロスプラットフォームマッピング
[[common]]
name = "MemTotal"
kind = "Memory"
unit = "Bytes"
confidence = "Comparable"
linux = "meminfo/MemTotal"
macos = "meminfo/MemTotal"       # platform_macos.rs では hw.memsize をこのフィールドに格納
windows = "meminfo/MemTotal"     # platform_windows.rs では Win32 を同名フィールドに格納
note.en = "All platforms report total physical RAM. Minor differences in excluded reserved memory."
note.ja = "全プラットフォームで物理RAM合計を報告。予約メモリの除外範囲に軽微な差異あり。"
```

👤 今泉: 「生成は build.rs ですか？ それとも手動スクリプト？」

☕ ヤン: 「build.rs は避けろ。ビルドのたびに TOML をパースして enum を生成すると CI が遅くなる。`cargo xtask generate-metrics` のような手動コマンドにして、生成結果を git にコミットしろ。」

⚔️ リヴァイ: 「同意する。理由は3つ。1) 生成結果がレビュー可能。2) TOML パーサーが build dependency に入らない。3) IDE が生成コードを即座に認識する (build.rs の OUT_DIR はIDEが追えないことがある)。」

🎋 千石: 「生成物のリスト。」

```
入力:
  metrics/linux.toml    — Linux 全メトリクス定義
  metrics/macos.toml    — macOS 全メトリクス定義
  metrics/windows.toml  — Windows 全メトリクス定義
  metrics/common.toml   — CommonMetric マッピング

生成物 (Rust):
  src/metric/kind.rs       — MetricKind enum
  src/metric/unit.rs       — MetricUnit enum
  src/metric/linux.rs      — LinuxMetric enum + Metric impl
  src/metric/macos.rs      — MacMetric enum + Metric impl
  src/metric/windows.rs    — WindowsMetric enum + Metric impl
  src/metric/common.rs     — CommonMetric enum + マッピングテーブル
  src/metric/mod.rs        — re-exports + trait 定義

生成物 (Java — syslenz4j 用):
  MetricKind.java
  MetricUnit.java
  LinuxMetric.java
  MacMetric.java
  WindowsMetric.java
  CommonMetric.java
  Metric.java (interface)

生成物 (ドキュメント):
  docs/metrics-reference.md — 全メトリクス一覧 + CommonMetric マッピング表
```

⚔️ リヴァイ: 「バージョニング。メトリクスの追加は後方互換。削除は deprecated flag を経由して次メジャーバージョンで削除。variant のリネームは破壊的変更 — 避ける。」

☕ ヤン: 「TOML ファイルに `since = "0.6.0"` と `deprecated = "0.8.0"` を付けろ。生成スクリプトが `#[deprecated]` attribute を自動付与する。」

**合意:**
- ソースオブトゥルース: TOML ファイル (linux.toml, macos.toml, windows.toml, common.toml)
- 生成方法: `cargo xtask generate-metrics` (手動実行、生成結果を git コミット)
- 生成物: Rust enum, Java enum/interface, ドキュメント
- バージョニング: `since`/`deprecated` フィールドで後方互換性管理

---

## Gap Summary

### 解決された問題

| ID | Gap | 解決策 |
|----|-----|--------|
| G-TYPE-1 | メトリクス識別が文字列ペアで型安全性なし | MetricField enum (プラットフォーム別) + Metric trait |
| G-XPLAT-1 | プラットフォーム間のメトリクス対応が暗黙的 | CommonMetric enum + confidence level |
| G-DISC-1 | メトリクスの発見可能性がない | enum variant による IDE 補完 + docs 生成 |

### 新規 Gap (この Session で発見)

| ID | Gap | 優先度 | 備考 |
|----|-----|--------|------|
| G-TYPE-2 | TOML 定義ファイルの初期作成 (Linux 500+ エントリ) | P1 | 既存パーサーからフィールド名を抽出するスクリプトが必要 |
| G-TYPE-3 | 既存コードの AlertRule.source/field を MetricField に移行 | P2 | 段階的移行。まず新コードで MetricField を使い、旧コードは互換レイヤーで対応 |
| G-TYPE-4 | education.rs の Category と MetricKind の統合 | P3 | Category を MetricKind のエイリアスにするか、Category が MetricKind を参照する形に |
| G-TYPE-5 | i18n.rs の field_description を TOML 駆動に移行 | P2 | 現在はソース+フィールド文字列でマッチ。TOML から生成した enum ベースに変更 |
| G-TYPE-6 | cargo xtask 基盤の構築 | P1 | xtask パターンの導入。generate-metrics サブコマンドの実装 |
| G-XPLAT-2 | macOS/Windows の description が不足 | P2 | platform_macos.rs, platform_windows.rs の describe 関数は Linux より貧弱 |
| G-XPLAT-3 | CommonMetric マッピングの検証テスト | P1 | 各プラットフォームで CommonMetric の値が妥当な範囲にあるかの integration test |

---

## 設計成果物

### 1. MetricKind — 全バリアント定義

```rust
/// メトリクスのカテゴリ分類。
/// education.rs の Category (6種) を拡張し、全プラットフォームのメトリクスを分類する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricKind {
    /// メモリ: RAM使用量、キャッシュ、バッファ、スワップ、メモリ圧力
    /// Linux: meminfo, vmstat, swaps, buddyinfo, zoneinfo, slabinfo, pagetypeinfo, pressure(memory)
    /// macOS: hw.memsize, vm_stat
    /// Windows: Win32_OperatingSystem, perf_memory, PageFile
    Memory,

    /// CPU: 使用率、ロード、スケジューリング、割り込み、温度
    /// Linux: stat, loadavg, cpuinfo, schedstat, softirqs, interrupts, pressure(cpu), thermal
    /// macOS: top, vm.loadavg, smc/xcpm
    /// Windows: perf_cpu, \Processor, MSAcpi_ThermalZoneTemperature
    Cpu,

    /// ネットワーク: インターフェース、接続、ルーティング、DNS、ファイアウォール
    /// Linux: net/dev, net/tcp, net/udp, net/unix, net/arp, net/route, net/sockstat,
    ///        net/snmp, net/netstat, net/wireless, ip/route, ip/neighbor, ss, dns, conntrack
    /// macOS: netstat, lsof, net/config, dns
    /// Windows: Get-NetTCPConnection, udp_endpoints, dns_cache, firewall
    Network,

    /// ストレージ: ディスクI/O、ファイルシステム、マウント、パーティション、ボリューム
    /// Linux: diskstats, df, mounts, partitions, locks, pressure(io)
    /// macOS: df, diskutil, diskstats
    /// Windows: perf_disk, df, volumes
    Storage,

    /// プロセス: プロセス一覧、FD使用量、ハンドル
    /// Linux: processes, file-nr
    /// macOS: ps, open_files, kern.num_files
    /// Windows: Get-Process, handles
    Process,

    /// システム: 稼働時間、カーネル情報、ブート設定、cgroup、スケジュールタスク
    /// Linux: uptime, version, cmdline, cgroups, consoles, devices, filesystems,
    ///        iomem, ioports, misc, dma, timer_list, modules
    /// macOS: kern.boottime, system_profile, software_update
    /// Windows: uptime, version, hotfix, scheduled_tasks, eventlog
    System,

    /// 電源: バッテリー、電源管理
    /// Linux: (ラップトップ環境のみ、将来対応)
    /// macOS: pmset, power_management
    /// Windows: Win32_Battery
    Power,

    /// セキュリティ: 暗号、ファイアウォール、カーネルモジュール
    /// Linux: crypto, modules
    /// macOS: kextstat (kernel_extensions)
    /// Windows: firewall
    Security,
}

impl MetricKind {
    pub fn all() -> &'static [MetricKind] {
        &[
            MetricKind::Memory, MetricKind::Cpu, MetricKind::Network,
            MetricKind::Storage, MetricKind::Process, MetricKind::System,
            MetricKind::Power, MetricKind::Security,
        ]
    }

    pub fn icon(&self) -> &'static str {
        match self {
            MetricKind::Memory   => "MEM",
            MetricKind::Cpu      => "CPU",
            MetricKind::Network  => "NET",
            MetricKind::Storage  => "DSK",
            MetricKind::Process  => "PRC",
            MetricKind::System   => "SYS",
            MetricKind::Power    => "PWR",
            MetricKind::Security => "SEC",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            MetricKind::Memory   => "Memory",
            MetricKind::Cpu      => "CPU / Load",
            MetricKind::Network  => "Network",
            MetricKind::Storage  => "Storage",
            MetricKind::Process  => "Process",
            MetricKind::System   => "System",
            MetricKind::Power    => "Power",
            MetricKind::Security => "Security",
        }
    }

    pub fn name_ja(&self) -> &'static str {
        match self {
            MetricKind::Memory   => "メモリ",
            MetricKind::Cpu      => "CPU / 負荷",
            MetricKind::Network  => "ネットワーク",
            MetricKind::Storage  => "ストレージ",
            MetricKind::Process  => "プロセス",
            MetricKind::System   => "システム",
            MetricKind::Power    => "電源",
            MetricKind::Security => "セキュリティ",
        }
    }
}
```

### 2. MetricUnit

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricUnit {
    Bytes,
    Percent,
    Count,
    Seconds,
    Celsius,
    BytesPerSec,
    CountPerSec,
    None,
}
```

### 3. MappingConfidence

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MappingConfidence {
    /// 定義が同一。プラットフォーム間の値を直接比較可能。
    Exact,
    /// 同じ概念。軽微な定義差あり。比較は概ね妥当。
    Comparable,
    /// 近似。OS間で計算方法が異なる。注意付きでのみ比較可能。
    Approximate,
}
```

### 4. Metric trait (Rust)

```rust
pub trait Metric: Copy + Clone + Eq + std::hash::Hash + std::fmt::Debug + 'static {
    fn source(&self) -> &'static str;
    fn field(&self) -> &'static str;
    fn kind(&self) -> MetricKind;
    fn unit(&self) -> MetricUnit;

    fn i18n_key(&self) -> String {
        format!("{}/{}", self.source(), self.field())
    }

    fn common(&self) -> Option<CommonMetric>;

    fn extract<'a>(&self, snapshot: &'a Snapshot) -> Option<&'a FieldValue> {
        snapshot.entries.get(self.source())
            .and_then(|entry| entry.fields.iter().find(|f| f.name == self.field()))
            .map(|f| &f.value)
    }
}
```

### 5. Metric interface (Java — syslenz4j)

```java
public interface Metric {
    String source();
    String field();
    MetricKind kind();
    MetricUnit unit();

    default String i18nKey() {
        return source() + "/" + field();
    }

    Optional<CommonMetric> common();
}
```

### 6. CommonMetric — 全エントリマッピングテーブル

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonMetric {
    MemTotal,
    MemAvailable,
    MemUsed,
    SwapUsed,
    CpuUsage,
    Load1,
    Load5,
    Load15,
    DiskUsePct,
    Temperature,
    TcpConnections,
    ProcessCount,
    FdUsage,
    Uptime,
    Battery,
}
```

**マッピングテーブル (全 15 エントリ):**

| CommonMetric | Confidence | Linux (source/field) | macOS (source/field) | Windows (source/field) | 備考 |
|-------------|-----------|---------------------|---------------------|----------------------|------|
| `MemTotal` | Comparable | `meminfo/MemTotal` | `meminfo/MemTotal` | `meminfo/MemTotal` | 全プラットフォームで物理RAM合計。予約領域の除外範囲に軽微差 |
| `MemAvailable` | Approximate | `meminfo/MemAvailable` | `meminfo/MemAvailable` | `meminfo/MemFree` | Linux: Free+回収可能Cache+Slab。macOS: free+inactive pages。Windows: FreePhysicalMemory (standby扱い差異) |
| `MemUsed` | Comparable | `meminfo/MemUsed` (計算値) | `meminfo/MemUsed` | `meminfo/MemUsed` | Total - Available/Free。計算方法差がそのまま伝播 |
| `SwapUsed` | Comparable | `meminfo/SwapUsed` (計算値) | `meminfo/SwapUsed` | `meminfo/SwapUsed` | 全プラットフォームで swap/pagefile の使用量 |
| `CpuUsage` | Comparable | `stat/cpu_usage_pct` | `top_summary/cpu_usage_pct` | `perf_cpu/ProcessorTimePct` | 計算間隔が異なる。Linux: /proc/stat差分。macOS: top snapshot。Windows: perf counter |
| `Load1` | Comparable | `loadavg/load_1min` | `loadavg/load_1min` | **N/A** | Windows には真のload average概念がない。Processor Queue Length は別物のためマッピング除外 |
| `Load5` | Comparable | `loadavg/load_5min` | `loadavg/load_5min` | **N/A** | 同上 |
| `Load15` | Comparable | `loadavg/load_15min` | `loadavg/load_15min` | **N/A** | 同上 |
| `DiskUsePct` | Comparable | `df/root_use_pct` | `df/root_use_pct` | `df/root_use_pct` | ルートファイルシステム (Linux: /, macOS: /, Windows: C:) の使用率 |
| `Temperature` | Approximate | `thermal/max_temp` | `thermal/cpu_temp` | `thermal/temperature` | センサーの種類・位置が異なる。Linux: thermal_zone, macOS: SMC/xcpm, Windows: WMI ACPI |
| `TcpConnections` | Comparable | `net/tcp` (行数) | `net/connections/tcp_count` | `tcp_connections/tcp_count` | 計測方法差 (Linux: /proc/net/tcp, macOS: netstat, Windows: PowerShell) だが同じ概念 |
| `ProcessCount` | Exact | `processes/process_count` | `processes/process_count` | `processes/process_count` | 全プラットフォームで実行中プロセスの総数 |
| `FdUsage` | Approximate | `file-nr/fd_usage_pct` | `file-nr/fd_usage_pct` | `handles/handle_count` | Linux/macOS: ファイルディスクリプタ。Windows: ハンドル (FDよりスコープが広い) |
| `Uptime` | Exact | `uptime/uptime` | `uptime/uptime` | `uptime/uptime` | 全プラットフォームでブートからの経過秒数 |
| `Battery` | Comparable | **N/A** | `battery/battery_percent` | `battery/battery_percent` | Linux は将来対応予定 (upower/sysfs)。デスクトップ環境では常に N/A |

```rust
impl CommonMetric {
    pub fn kind(&self) -> MetricKind {
        match self {
            CommonMetric::MemTotal | CommonMetric::MemAvailable |
            CommonMetric::MemUsed | CommonMetric::SwapUsed => MetricKind::Memory,
            CommonMetric::CpuUsage | CommonMetric::Load1 |
            CommonMetric::Load5 | CommonMetric::Load15 |
            CommonMetric::Temperature => MetricKind::Cpu,
            CommonMetric::DiskUsePct => MetricKind::Storage,
            CommonMetric::TcpConnections => MetricKind::Network,
            CommonMetric::ProcessCount | CommonMetric::FdUsage => MetricKind::Process,
            CommonMetric::Uptime => MetricKind::System,
            CommonMetric::Battery => MetricKind::Power,
        }
    }

    pub fn confidence(&self) -> MappingConfidence {
        match self {
            CommonMetric::ProcessCount | CommonMetric::Uptime => MappingConfidence::Exact,
            CommonMetric::MemTotal | CommonMetric::MemUsed | CommonMetric::SwapUsed |
            CommonMetric::CpuUsage | CommonMetric::Load1 | CommonMetric::Load5 |
            CommonMetric::Load15 | CommonMetric::DiskUsePct |
            CommonMetric::TcpConnections | CommonMetric::Battery => MappingConfidence::Comparable,
            CommonMetric::MemAvailable | CommonMetric::Temperature |
            CommonMetric::FdUsage => MappingConfidence::Approximate,
        }
    }

    /// このプラットフォームで対応する MetricField を返す
    #[cfg(target_os = "linux")]
    pub fn to_platform(&self) -> Option<LinuxMetric> {
        match self {
            CommonMetric::MemTotal => Some(LinuxMetric::MeminfoMemTotal),
            CommonMetric::MemAvailable => Some(LinuxMetric::MeminfoMemAvailable),
            CommonMetric::CpuUsage => Some(LinuxMetric::StatCpuUsagePct),
            CommonMetric::Load1 => Some(LinuxMetric::LoadavgLoad1min),
            CommonMetric::Uptime => Some(LinuxMetric::UptimeUptime),
            CommonMetric::Battery => None, // Linux ラップトップサポートは将来対応
            // ... 全 variant
            _ => None,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_platform(&self) -> Option<MacMetric> {
        match self {
            CommonMetric::MemTotal => Some(MacMetric::MeminfoMemTotal),
            CommonMetric::MemAvailable => Some(MacMetric::MeminfoMemAvailable),
            CommonMetric::Load1 => Some(MacMetric::LoadavgLoad1min),
            CommonMetric::Battery => Some(MacMetric::BatteryPercent),
            // ...
            _ => None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn to_platform(&self) -> Option<WindowsMetric> {
        match self {
            CommonMetric::MemTotal => Some(WindowsMetric::MeminfoMemTotal),
            CommonMetric::MemAvailable => Some(WindowsMetric::MeminfoMemFree),
            CommonMetric::Load1 => None, // Windows にはマッピングなし
            CommonMetric::Battery => Some(WindowsMetric::BatteryPercent),
            // ...
            _ => None,
        }
    }
}
```

### 7. Java 側 CommonMetric

```java
public enum CommonMetric implements Metric {
    MEM_TOTAL("MemTotal", MetricKind.MEMORY, MetricUnit.BYTES, MappingConfidence.COMPARABLE),
    MEM_AVAILABLE("MemAvailable", MetricKind.MEMORY, MetricUnit.BYTES, MappingConfidence.APPROXIMATE),
    MEM_USED("MemUsed", MetricKind.MEMORY, MetricUnit.BYTES, MappingConfidence.COMPARABLE),
    SWAP_USED("SwapUsed", MetricKind.MEMORY, MetricUnit.BYTES, MappingConfidence.COMPARABLE),
    CPU_USAGE("CpuUsage", MetricKind.CPU, MetricUnit.PERCENT, MappingConfidence.COMPARABLE),
    LOAD_1("Load1", MetricKind.CPU, MetricUnit.NONE, MappingConfidence.COMPARABLE),
    LOAD_5("Load5", MetricKind.CPU, MetricUnit.NONE, MappingConfidence.COMPARABLE),
    LOAD_15("Load15", MetricKind.CPU, MetricUnit.NONE, MappingConfidence.COMPARABLE),
    DISK_USE_PCT("DiskUsePct", MetricKind.STORAGE, MetricUnit.PERCENT, MappingConfidence.COMPARABLE),
    TEMPERATURE("Temperature", MetricKind.CPU, MetricUnit.CELSIUS, MappingConfidence.APPROXIMATE),
    TCP_CONNECTIONS("TcpConnections", MetricKind.NETWORK, MetricUnit.COUNT, MappingConfidence.COMPARABLE),
    PROCESS_COUNT("ProcessCount", MetricKind.PROCESS, MetricUnit.COUNT, MappingConfidence.EXACT),
    FD_USAGE("FdUsage", MetricKind.PROCESS, MetricUnit.PERCENT, MappingConfidence.APPROXIMATE),
    UPTIME("Uptime", MetricKind.SYSTEM, MetricUnit.SECONDS, MappingConfidence.EXACT),
    BATTERY("Battery", MetricKind.POWER, MetricUnit.PERCENT, MappingConfidence.COMPARABLE),
    ;

    private final String name;
    private final MetricKind kind;
    private final MetricUnit unit;
    private final MappingConfidence confidence;

    CommonMetric(String name, MetricKind kind, MetricUnit unit, MappingConfidence confidence) {
        this.name = name;
        this.kind = kind;
        this.unit = unit;
        this.confidence = confidence;
    }

    @Override public String source() { return "common"; }
    @Override public String field() { return name; }
    @Override public MetricKind kind() { return kind; }
    @Override public MetricUnit unit() { return unit; }
    @Override public Optional<CommonMetric> common() { return Optional.of(this); }
    public MappingConfidence confidence() { return confidence; }
}
```

### 8. 自動生成スクリプト設計

```
xtask/
  src/
    main.rs           -- cargo xtask エントリポイント
    generate_metrics.rs -- メトリクス生成ロジック

metrics/              -- ソースオブトゥルース (TOML)
  linux.toml          -- Linux 全メトリクス定義 (500+ エントリ)
  macos.toml          -- macOS 全メトリクス定義 (100+ エントリ)
  windows.toml        -- Windows 全メトリクス定義 (150+ エントリ)
  common.toml         -- CommonMetric マッピング (15 エントリ)
```

**TOML スキーマ:**

```toml
# metrics/linux.toml
[meta]
platform = "linux"
version = "0.6.0"

[[metrics]]
source = "meminfo"
field = "MemTotal"
variant = "MeminfoMemTotal"        # Rust enum variant 名
kind = "Memory"
unit = "Bytes"
since = "0.1.0"
# deprecated = "0.9.0"            # 非推奨フラグ (任意)
description.en = "Total usable RAM"
description.ja = "利用可能なRAMの合計"
description_detail.en = "Total amount of physical RAM minus reserved bits and kernel binary code"
description_detail.ja = "物理RAMの合計量。予約領域とカーネルバイナリコードを除く"
description_diagnostic.en = "This is a constant value. If it changes, hardware was hot-added/removed or a cgroup limit changed."
description_diagnostic.ja = "通常は定数。変化した場合、ハードウェアのホットアド/リムーブまたはcgroup制限の変更が原因。"
```

```toml
# metrics/common.toml
[meta]
version = "0.6.0"

[[common]]
name = "MemTotal"
variant = "MemTotal"
kind = "Memory"
unit = "Bytes"
confidence = "Comparable"
linux = "meminfo/MemTotal"
macos = "meminfo/MemTotal"
windows = "meminfo/MemTotal"
since = "0.6.0"
note.en = "All platforms report total physical RAM."
note.ja = "全プラットフォームで物理RAM合計を報告。"

[[common]]
name = "Load1"
variant = "Load1"
kind = "Cpu"
unit = "None"
confidence = "Comparable"
linux = "loadavg/load_1min"
macos = "loadavg/load_1min"
# windows は未定義 = マッピングなし
since = "0.6.0"
note.en = "Windows has no true load average equivalent. Processor Queue Length is intentionally excluded."
note.ja = "Windowsには真のload average概念がない。Processor Queue Lengthは意図的に除外。"
```

**生成コマンド:**

```bash
# Rust enum + trait impl を生成
cargo xtask generate-metrics --lang rust

# Java enum + interface を生成
cargo xtask generate-metrics --lang java --output ../syslenz4j/src/main/java/

# ドキュメントを生成
cargo xtask generate-metrics --lang docs --output docs/

# 全部生成
cargo xtask generate-metrics --all

# バリデーション (TOML の整合性チェック)
cargo xtask generate-metrics --validate
```

**バリデーション項目:**
1. 全 variant 名がユニーク
2. common.toml の linux/macos/windows 参照が各プラットフォーム TOML に存在する
3. kind, unit が有効な enum 値
4. since バージョンが semver 準拠
5. deprecated な variant が common.toml から参照されていない

---

## 実装ロードマップ

| Phase | 内容 | 依存 | 優先度 |
|-------|------|------|--------|
| 0 | cargo xtask 基盤構築 | なし | P1 |
| 1 | TOML 定義ファイル初期作成 (既存パーサーからフィールド抽出スクリプト) | Phase 0 | P1 |
| 2 | Rust enum 生成 + Metric trait 定義 | Phase 1 | P1 |
| 3 | CommonMetric マッピング定義 + 検証テスト | Phase 2 | P1 |
| 4 | AlertRule を MetricField ベースに移行 (互換レイヤー経由) | Phase 2 | P2 |
| 5 | i18n.rs の field_description を TOML 駆動に移行 | Phase 1 | P2 |
| 6 | Java enum/interface 生成 (syslenz4j 連携) | Phase 2 | P2 |
| 7 | education.rs の Category を MetricKind に統合 | Phase 2 | P3 |
| 8 | docs/metrics-reference.md の自動生成 | Phase 1 | P3 |
