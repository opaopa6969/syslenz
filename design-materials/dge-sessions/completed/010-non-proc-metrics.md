# DGE Session 010: /proc 外メトリクスの導入 — df, 温度, systemd, dmesg, FD

- **Date**: 2026-03-28
- **Theme**: syslenz を「/proc パーサー」から「Linux システム情報ツール」に進化させるために必要な非 /proc メトリクスの特定と実装計画
- **Parent Gaps**: G1 (ターゲットユーザー差別化), G2 (ダッシュボード), 新規 Gap
- **Characters**: ハウス (診断の天才) + 千石 (品質の番人) + ヤン (怠惰な簡潔主義者) + リヴァイ (実装の鬼) + 今泉 (初心者の代弁者) + 僕
- **Input**: 43 /proc パーサー実装済み。diagnostics.rs で meminfo, loadavg, swap, pressure, processes, net/tcp を診断。Dashboard に loadavg, meminfo, stat, uptime, net/dev を表示。しかし df (ディスク使用率)、CPU 温度、systemd サービス状態、dmesg、システム FD 使用率など、運用上不可欠なメトリクスが欠落。

---

## 現状の整理

先輩 (ナレーション): syslenz は現在 43 の /proc ソースをパースし、構造化データとして表示・診断・エクスポートする。しかし Linux システム監視において重要な情報源は /proc だけではない。

**現在カバーしていない重要メトリクス:**

| カテゴリ | メトリクス | ソース | 重要度 |
|---------|-----------|--------|--------|
| ストレージ | ディスク使用率 (df) | `statfs()` syscall | Critical |
| ハードウェア | CPU/GPU 温度 | `/sys/class/thermal/thermal_zone*/temp` | Critical |
| カーネル | システム FD 使用率 | `/proc/sys/fs/file-nr` | Critical |
| サービス | systemd ユニット状態 | D-Bus / `systemctl` | Critical |
| ストレージ | ディスク種別 (SSD/HDD) | `/sys/block/*/queue/rotational` | Important |
| CPU | 周波数スケーリング | `/sys/devices/system/cpu/cpu*/cpufreq/` | Important |
| 電源 | バッテリー状態 | `/sys/class/power_supply/` | Important |
| ネットワーク | NIC リンク速度 | `/sys/class/net/*/speed` | Important |
| カーネル | dmesg / カーネルログ | `klogctl()` / `/dev/kmsg` | Operational |
| ユーザー | ログインユーザー | utmp / `who` | Operational |
| ネットワーク | DNS 設定 | `/etc/resolv.conf` | Operational |
| コンテナ | cgroup v2 統計 | `memory.current`, `cpu.stat` | Operational |

問題の本質: syslenz は「Wireshark for /proc」と名乗っているが、/proc だけでは監視ツールとして致命的な穴がある。ディスクが 100% になっても、CPU が 95℃ でスロットリングしていても、syslenz は何も表示しない。

---

## Scene 1: スコープの議論 — /proc 専用ツール vs システム情報ツール

先輩: syslenz のプロダクトアイデンティティについて議論する。/proc に限定すべきか、Linux システム全体に広げるべきか。

👤 今泉: 「すみません、そもそもの質問なんですけど... syslenz って "Wireshark for /proc" ですよね？ /sys とか systemd とか dmesg とか、それ /proc じゃないですよね。範囲を広げていいんですか？ それって別のツールじゃないですか？」

☕ ヤン: 「いい質問。"Wireshark for /proc" は開発開始時のコンセプトだった。でも Wireshark だって最初は "Ethereal" で Ethernet だけだったのが、今は Bluetooth も USB も Wi-Fi もキャプチャする。名前を変えて進化した。紅茶ください。」

🏥 ハウス: 「患者に "心臓は心臓科、肝臓は肝臓科に行け" と言う病院は潰れる。システムの健康状態を診断するのに、情報源が /proc かどうかなんて患者には関係ない。/proc しか見ない診断ツールは、聴診器だけで CT も血液検査もしない医者と同じだ。」

👤 今泉: 「でも、コードベースの一貫性は？ src/proc/ に全部入ってますよね。/sys のパーサーを proc ディレクトリに入れるのは...」

⚔️ リヴァイ: 「ディレクトリ名を変えればいい。今は `src/proc/` に 43 ファイル。`src/sys/` を追加して /sys 系を入れる。あるいは `src/sources/` にリネームする。ディレクトリ名に命を懸けるな。」

🎋 千石: 「待ってほしい。リネームは影響範囲が大きい。`mod.rs` の構造、`Snapshot::capture()` の呼び出し、`ProcEntry` の型名... 全部変わる。既存の 43 パーサーは動いている。動いているものに触るな。」

☕ ヤン: 「千石が正しい。リネームは今やることじゃない。新しいソースは新しいディレクトリに入れる。既存の `src/proc/` はそのまま。`ProcEntry` は... まあ、名前は微妙だけど、中身は `source + fields` だから `/sys` の情報を入れても動く。型名は後で直す。」

🏥 ハウス: 「つまり "Wireshark for /proc" から "Wireshark for Linux" に進化する。ただし /proc の 43 パーサーは最大の資産だからそのまま維持。新しいソースを追加する形で拡張。ブランドメッセージだけ更新する。」

→ **Gap G14 発見: プロダクトアイデンティティの再定義。タグラインを "Wireshark for /proc" から "Wireshark for Linux" 相当に更新。README、ヘルプテキスト、i18n メッセージの修正が必要。**

→ **Gap G15 発見: ソースディレクトリ構成。新メトリクス用のコードを `src/proc/` に入れるか、新ディレクトリ (`src/sys/`, `src/system/`) を作るか。**

---

## Scene 2: 絶対必要なメトリクス — df と温度

先輩: 最も優先度の高い 2 つのメトリクス、ディスク使用率と CPU 温度について詳細に議論する。

### df (ディスク使用率)

🎋 千石: 「はっきり言う。**df がない監視ツールはレストランにメニューがないのと同じだ。** 何が提供されるのかわからない。ディスク full は障害原因の第 1 位だ。ログが書けない、データベースが止まる、デプロイが失敗する — 全部ディスク full が原因。syslenz がこれを見せないのは致命的な欠陥だ。」

👤 今泉: 「あの、/proc/mounts にマウント情報はありますよね？ 既に mounts パーサーがありますけど...」

🎋 千石: 「mounts はマウントポイントの一覧を出すだけだ。"/ は ext4 でマウントされている" という情報。使用率は出ない。"/ が 92% 使用" は別のソースから取る必要がある。/proc には**ファイルシステムの使用率情報は存在しない。**」

🏥 ハウス: 「補足する。ディスク使用率の取得方法は 2 つある。」

```
方法 1: statfs() / statvfs() syscall
  - C: statfs("/", &buf) で f_blocks, f_bfree, f_bavail, f_bsize が取れる
  - Rust: nix::sys::statvfs::statvfs("/") または libc::statvfs を直接呼ぶ
  - 利点: ファイルを読まない。syscall 1 回で完結。高速。
  - 欠点: マウントポイントごとに呼ぶ必要がある

方法 2: /proc/mounts のマウントポイント一覧 + 各マウントポイントに statvfs()
  - 既存の mounts パーサーからマウントポイントを取得
  - 仮想 FS (proc, sys, tmpfs, devtmpfs 等) を除外
  - 残った実 FS に statvfs() を呼ぶ
  - これが df コマンドの実装と同じ
```

☕ ヤン: 「方法 2 が自然だね。mounts パーサーは既にある。除外リストは `proc, sysfs, devtmpfs, tmpfs, cgroup, cgroup2, debugfs, tracefs, securityfs, pstore, bpf, hugetlbfs, mqueue, fusectl` あたり。実質 `/`, `/home`, `/boot`, `/var` とかだけ残る。」

🏥 ハウス: 「出力フィールドは `df -h` と同じにしろ:」

```
filesystem: String     — デバイス名 (/dev/sda1)
mount_point: String    — マウントポイント (/)
fs_type: String        — ファイルシステム種別 (ext4)
total: Bytes(u64)      — 総容量
used: Bytes(u64)       — 使用量
available: Bytes(u64)  — 空き容量
use_percent: Float(f64) — 使用率 (%)
```

⚔️ リヴァイ: 「実装は Rust の `nix` クレートを使う。Cargo.toml に `nix` は... まだ入ってないか？ `libc` は入ってるなら直接 `libc::statvfs` を呼べばいい。外部クレートを増やすな。」

```rust
// 実装イメージ
use std::ffi::CString;
use std::mem::MaybeUninit;

fn statvfs_for(path: &str) -> anyhow::Result<(u64, u64, u64)> {
    let c_path = CString::new(path)?;
    let mut buf = MaybeUninit::<libc::statvfs>::uninit();
    let ret = unsafe { libc::statvfs(c_path.as_ptr(), buf.as_mut_ptr()) };
    if ret != 0 {
        anyhow::bail!("statvfs failed for {}", path);
    }
    let stat = unsafe { buf.assume_init() };
    let block_size = stat.f_frsize as u64;
    let total = stat.f_blocks * block_size;
    let available = stat.f_bavail * block_size;  // non-root available
    let free = stat.f_bfree * block_size;
    let used = total - free;
    Ok((total, used, available))
}
```

🎋 千石: 「`f_bavail` と `f_bfree` の違いに注意。`f_bfree` はスーパーユーザー用の予約ブロックを含む。一般ユーザーが使えるのは `f_bavail`。df は `f_bavail` を "Available" として表示する。ここを間違えると df コマンドと値がズレて信用を失う。」

### CPU/GPU 温度

🏥 ハウス: 「**温度を見ずに診断するのは、体温を測らずに風邪を診断するのと同じだ。** CPU が 95℃ でサーマルスロットリングしていたら、load average が上がる。メモリ帯域が下がる。全てのパフォーマンスメトリクスが劣化する。根本原因は温度なのに、温度を見ないから "CPU が遅い" "メモリが遅い" と誤診する。」

👤 今泉: 「温度ってどこから取るんですか？ /proc にはないですよね。」

🏥 ハウス: 「/sys/class/thermal/ だ。」

```
/sys/class/thermal/thermal_zone0/temp    → "49000" (49.0℃、ミリ度)
/sys/class/thermal/thermal_zone0/type    → "x86_pkg_temp" (CPU パッケージ温度)
/sys/class/thermal/thermal_zone1/type    → "acpitz" (ACPI thermal zone)
```

☕ ヤン: 「読み方は /proc と全く同じ。`std::fs::read_to_string` してパース。ミリ度を 1000 で割って℃にする。thermal_zone の数はマシンによって違うからディレクトリを列挙する。」

⚔️ リヴァイ: 「/sys の読み方は /proc と同じだ。テキストファイルを読んでパースする。作れ。」

```rust
// 実装イメージ
fn parse_thermal() -> anyhow::Result<ProcEntry> {
    let mut fields = Vec::new();
    for entry in std::fs::read_dir("/sys/class/thermal/")? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with("thermal_zone") { continue; }
        let base = entry.path();
        let temp_raw: String = std::fs::read_to_string(base.join("temp"))?
            .trim().to_string();
        let temp_c: f64 = temp_raw.parse::<f64>()? / 1000.0;
        let zone_type = std::fs::read_to_string(base.join("type"))
            .unwrap_or_default().trim().to_string();
        fields.push(Field {
            name: format!("{}_{}", name, zone_type),
            value: FieldValue::Float(temp_c),
            unit: Some("C".into()),
            description: format!("Temperature of {} ({})", name, zone_type),
        });
    }
    Ok(ProcEntry { source: "thermal".into(), fields })
}
```

🏥 ハウス: 「hwmon もある。`/sys/class/hwmon/hwmon*/temp*_input` はより詳細なセンサー値を出す。ファン回転数、電圧も取れる。だが MVP は thermal_zone だけでいい。hwmon は次のフェーズだ。」

🎋 千石: 「一点。thermal_zone が存在しない環境がある。VM、コンテナ、古いハードウェア。`/sys/class/thermal/` が空でもパニックしないこと。空なら "thermal data unavailable" と INFO レベルで出すだけ。エラーにするな。」

→ **Gap G16 発見: df パーサーの設計と実装。statfs() syscall を使い、mounts パーサーと連携して実 FS の使用率を取得。**

→ **Gap G17 発見: 温度パーサーの設計と実装。/sys/class/thermal/ からセンサー値を取得。VM/コンテナでの不在に対応。**

---

## Scene 3: 運用に効くメトリクス — systemd, dmesg, FD

先輩: Critical と Operational カテゴリの残りのメトリクスについて議論する。

### システム FD 使用率

🏥 ハウス: 「FD 枯渇は silent killer だ。ログに "Too many open files" が出て初めて気づく。その時には既にサービスが死んでいる。`/proc/sys/fs/file-nr` は 3 つの数字を返す:」

```
/proc/sys/fs/file-nr: "9248  0  9223372036854775807"
  field 1: 割り当て済み FD 数 (9248)
  field 2: 未使用 FD 数 (カーネル 2.6+ では常に 0)
  field 3: 最大 FD 数 (file-max)
```

☕ ヤン: 「これは /proc にあるから、本来 43 パーサーに入ってるべきだったやつだね。忘れてただけ。src/proc/file_nr.rs を追加すればいい。1 行パースするだけだから 20 行で書ける。」

⚔️ リヴァイ: 「20 行。やれ。」

🎋 千石: 「付け加える。`/proc/sys/fs/file-nr` だけでなく、`/proc/sys/fs/inode-nr` もある。inode 枯渇も同じくらい危険だ。小さなファイルを大量に作る環境 (メールサーバー、キャッシュサーバー) で起きる。」

### systemd サービス状態

🏥 ハウス: 「systemd が failed ユニットを抱えてるのに気づかないのは、患者が熱を出してるのに "まだ歩けるから大丈夫" と言うようなもんだ。サーバー監視で "全サービス正常" を確認できないツールは監視ツールじゃない。」

👤 今泉: 「systemd ってどうやって状態を取るんですか？ /proc にも /sys にもないですよね？」

☕ ヤン: 「3 つの方法がある:」

```
方法 A: D-Bus API (sd-bus)
  - systemd の正式な API。libsystemd に依存
  - Rust: zbus クレートで非同期アクセス
  - 利点: 最も正確。リアルタイム
  - 欠点: 依存が重い。zbus は async 必須

方法 B: systemctl コマンド実行
  - Command::new("systemctl").args(["list-units", "--failed", "--no-pager", "--plain"])
  - 出力をパース
  - 利点: 依存なし。シンプル
  - 欠点: コマンド起動のオーバーヘッド。systemctl がない環境 (コンテナ) で失敗

方法 C: /run/systemd/system/ のファイル直接読み取り
  - 非公式。バージョンで構造が変わる可能性
  - 利点: 依存なし
  - 欠点: 不安定。非推奨
```

⚔️ リヴァイ: 「方法 B だ。`systemctl --failed` の出力をパースする。外部コマンド実行は好きじゃないが、D-Bus のためにクレートを追加するのはもっと嫌いだ。systemctl がない環境ではスキップ。」

🎋 千石: 「同意。ただし `systemctl list-units --failed --no-legend --plain --no-pager` を使え。`--no-legend` でヘッダーとフッターが消える。パースが楽になる。出力が空なら "All units OK"。」

🏥 ハウス: 「もう一つ。`--state=failed` だけじゃなく、`--type=service --state=running` で稼働中サービス数も取れると有用だ。"47 services running, 2 failed" は一目で状態がわかる。」

### dmesg (カーネルログ)

🏥 ハウス: 「dmesg は金鉱だ。OOM kill、ディスクエラー、NIC リセット、セグフォルト — 全部ここに出る。だが...」

⚔️ リヴァイ: 「dmesg は汚い。パースするな。最新 N 行だけ見せろ。」

🏥 ハウス: 「半分同意だ。全文パースは無意味。だが "見せるだけ" も不十分。キーワードフィルタをかけろ:」

```
重要キーワード:
  "Out of memory"  → OOM Killer 発動
  "oom_reaper"     → OOM リーパーがメモリ回収中
  "error"          → 汎用エラー
  "I/O error"      → ディスク障害
  "link is not ready" / "link down"  → NIC 障害
  "segfault"       → プロセスクラッシュ
  "kernel BUG"     → カーネルバグ
  "Call Trace"     → カーネルスタックトレース
  "Hardware Error" → MCE (Machine Check Exception)
```

☕ ヤン: 「実装は `Command::new("dmesg").args(["--time-format", "iso", "-l", "err,warn,crit"])` で error 以上のメッセージだけ取得。`--since` オプションで最新 1 時間に制限。出力は Table で見せる。」

⚔️ リヴァイ: 「dmesg は root 権限が要る場合がある。`/dev/kmsg` は通常 root のみ。`dmesg` コマンドは `kernel.dmesg_restrict=1` だとユーザーは読めない。権限なしなら "dmesg: permission denied" と表示してスキップ。エラーにするな。」

🎋 千石: 「`/dev/kmsg` を直接読む方法もある。テキストストリームとして読める。だが systemctl と同じでコマンド実行がシンプル。MVP はコマンド実行でいい。」

→ **Gap G18 発見: /proc/sys/fs/file-nr パーサー。既存の /proc パーサーに追加。inode-nr も含む。**

→ **Gap G19 発見: systemd サービス状態パーサー。systemctl コマンド実行でユニット一覧を取得。**

→ **Gap G20 発見: dmesg 表示機能。最新のエラー/警告レベルメッセージを取得し、キーワードハイライト。**

---

## Scene 4: 診断エンジンへの統合

先輩: 新メトリクスを既存の diagnostics.rs にどう統合するか。ハウスの診断パターン設計。

🏥 ハウス (ホワイトボードの前、杖で図を描きながら): 「今の diagnostics.rs は 6 つのチェック関数がある: `check_memory`, `check_load`, `check_swap`, `check_pressure`, `check_processes`, `check_network`。ここに新しいチェッカーを追加する。だが重要なのは単体チェックじゃない。**クロス診断** だ。」

### 単体診断パターン

🏥 ハウス: 「まず基本。新メトリクスの閾値診断:」

```
check_disk_usage():
  条件: use_percent > 90.0
  重症度: CRITICAL
  Title (ja): "ディスク危機: {mount_point} が {use_percent}% 使用"
  Detail: "残り {available}。ログ書き込み失敗、デプロイ失敗のリスク。"
  Suggestion: "du -sh /* | sort -rh | head で大きなディレクトリを特定。
               journalctl --vacuum-size=500M でジャーナルを縮小。"

  条件: use_percent > 80.0
  重症度: WARNING
  Title (ja): "ディスク残量注意: {mount_point} が {use_percent}% 使用"
  Detail: "残り {available}。増加傾向ならアクションが必要。"
  Suggestion: "df -ih で inode 使用率も確認。find / -xdev -type f -size +100M"

check_temperature():
  条件: temp > 90.0
  重症度: CRITICAL
  Title (ja): "CPU 過熱: {zone_type} が {temp}℃"
  Detail: "サーマルスロットリング作動中。CPU クロックが強制低下。全パフォーマンスに影響。"
  Suggestion: "ファン動作確認。ヒートシンクの埃。サーバールームの空調。
               負荷を分散するか、一時的にプロセスを停止。"

  条件: temp > 75.0
  重症度: WARNING
  Title (ja): "CPU 高温: {zone_type} が {temp}℃"
  Detail: "スロットリング閾値に接近中。負荷またはファン異常の可能性。"
  Suggestion: "top で CPU 消費の高いプロセスを確認。ファンの動作音を確認。"

check_fd_usage():
  条件: allocated / max > 0.80
  重症度: WARNING
  Title (ja): "システム FD 残量注意: {allocated}/{max} ({percent}%)"
  Detail: "新規ファイル/ソケットのオープンが失敗し始める。サービス障害の前兆。"
  Suggestion: "ls /proc/*/fd | wc -l でプロセスごとの FD 数を確認。
               sysctl fs.file-max で上限引き上げを検討。"

  条件: allocated / max > 0.95
  重症度: CRITICAL
  Title (ja): "システム FD 枯渇寸前: {allocated}/{max} ({percent}%)"
  Detail: "EMFILE エラーが発生する。ログ書き込み不可、新規接続不可。"
  Suggestion: "即座に FD リークしているプロセスを特定: lsof | awk '{print $2}' | sort | uniq -c | sort -rn | head"

check_systemd():
  条件: failed_units > 0
  重症度: WARNING
  Title (ja): "systemd: {failed_count} 個のユニットが failed"
  Detail: "failed: {unit_names}"
  Suggestion: "journalctl -u {unit} で失敗原因を確認。systemctl restart {unit} で再起動を試行。"

check_dmesg():
  条件: "Out of memory" in recent messages
  重症度: WARNING
  Title (ja): "OOM Killer 発動検出 (dmesg)"
  Detail: "カーネルがメモリ不足でプロセスを強制終了した。"
  Suggestion: "dmesg | grep -i oom で詳細確認。killed されたプロセスと原因を調査。"

  条件: "I/O error" in recent messages
  重症度: WARNING
  Title (ja): "ディスク I/O エラー検出 (dmesg)"
  Detail: "ストレージデバイスでエラー。ディスク障害の前兆の可能性。"
  Suggestion: "smartctl -a /dev/sdX でディスク健全性を確認。バックアップを確認。"
```

### クロス診断パターン

🏥 ハウス: 「ここからが本番だ。**単一メトリクスの閾値チェックは素人でもできる。** プロの診断は複数のメトリクスを組み合わせて原因を絞り込む。」

```
パターン 1: 温度 + load average
  条件: temp > 80℃ AND load/cpu_count > 1.5
  診断: "CPU 過熱が原因で性能低下している可能性が高い。
         スロットリングにより実効クロックが下がり、
         同じ処理に時間がかかるため load が上がる。"
  Suggestion: "cpufreq の current_frequency を確認。
               max_frequency より著しく低ければスロットリング確定。"

パターン 2: df + I/O pressure
  条件: disk_usage > 90% AND io_some_avg10 > 25.0
  診断: "ディスクがほぼ満杯かつ I/O 圧力が高い。
         断片化によるシーク増加、または空き領域不足による
         アロケーション遅延が発生している可能性。"
  Suggestion: "不要ファイルを削除して空き容量を確保。
               /var/log, /tmp, core dumps を確認。"

パターン 3: FD 使用率 + CLOSE_WAIT
  条件: fd_usage > 70% AND close_wait > 20
  診断: "FD リークが進行中。CLOSE_WAIT ソケットが
         FD を消費し続けている。特定のアプリケーションが
         ソケットを close() していない。"
  Suggestion: "ss -tnp state close-wait で PID を特定。
               そのプロセスの FD 数を /proc/{pid}/fd で確認。"

パターン 4: OOM (dmesg) + swap 枯渇 + memory pressure
  条件: dmesg に "Out of memory" AND SwapFree == 0 AND memory_full_avg10 > 0
  診断: "OOM Killer が発動済み。スワップも枯渇。
         メモリ回収が追いつかず全タスクが停滞。
         根本的なメモリ不足。"
  Suggestion: "即座に不要プロセスを kill。
               長期的には RAM 増設またはメモリリーク修正。"

パターン 5: systemd failed + dmesg segfault
  条件: failed_units > 0 AND dmesg に "segfault"
  診断: "サービスがセグフォルトで crashed している可能性。
         systemd が restart を繰り返しているかもしれない。"
  Suggestion: "coredumpctl list で core dump を確認。
               journalctl -u {unit} --since '1 hour ago' で再起動ループを確認。"
```

🎋 千石: 「クロス診断は強力だが、組み合わせが指数的に増える。全パターンを実装すると保守できない。」

🏥 ハウス: 「だから最初は 5 パターンだけだ。実際の障害対応で "これがあれば 10 分早く原因がわかった" というパターンだけ。」

☕ ヤン: 「実装方法は、`analyze()` の最後に `check_cross_diagnostics(&mut findings, snapshot, locale)` を追加。各単体チェックの後に呼ぶ。findings を見て "WARN が 2 つ以上の特定の組み合わせ" を検出。」

→ **Gap G21 発見: 新メトリクスの単体診断パターン追加 (check_disk_usage, check_temperature, check_fd, check_systemd, check_dmesg)。**

→ **Gap G22 発見: クロス診断パターンの実装。複数メトリクスの組み合わせで原因を絞り込む診断エンジンの進化。**

---

## Scene 5: MVP スコープと実装順序

先輩: 何をどの順番で実装するか。全部やろうとすると終わらない。

☕ ヤン: 「全部やる必要ある？ まず df と温度と FD だけでいい。systemd と dmesg は次のフェーズだ。理由は:」

```
Phase 1 (今すぐ):
  1. df (ディスク使用率)     — 障害原因 #1。実装は statfs() だけ
  2. thermal (温度)          — /sys を読むだけ。パーサーのテンプレートになる
  3. file-nr (FD 使用率)     — /proc を読むだけ。1 ファイル 20 行

Phase 2 (次のフェーズ):
  4. systemd                 — コマンド実行が必要。テスト書きにくい
  5. dmesg                   — 権限問題。出力が環境依存
  6. NIC speed               — /sys 読むだけだが優先度低い

Phase 3 (将来):
  7. cpufreq                 — /sys 読むだけ。温度のクロス診断で必要になったら
  8. battery                 — ラップトップ限定
  9. disk type (SSD/HDD)     — diskstats の解釈で必要になったら
  10. resolv.conf             — 静的情報。変化しない
  11. utmp (logged-in users)  — 優先度低い
  12. cgroup v2 stats         — コンテナ限定
```

🏥 ハウス: 「Phase 1 に診断パターンの追加もセットだ。df パーサーだけ作って診断に繋がないなら価値は半分だ。」

⚔️ リヴァイ: 「Phase 1 の実装量を見積もる:」

```
src/sys/mod.rs           — 新モジュール宣言 (thermal, df)  ~20 行
src/sys/thermal.rs       — /sys/class/thermal パーサー      ~60 行
src/sys/df.rs            — statfs ベースの FS 使用率        ~80 行
src/proc/file_nr.rs      — /proc/sys/fs/file-nr パーサー    ~30 行
src/proc/mod.rs          — file_nr の mod 宣言 + capture    ~5 行追加
src/diagnostics.rs       — check_disk_usage()               ~40 行
                         — check_temperature()              ~40 行
                         — check_fd_usage()                 ~30 行
src/ui/render.rs         — Dashboard に df セクション追加    ~40 行
                                                    合計: ~345 行
```

🎋 千石: 「`src/sys/` を新設するなら `mod.rs` の構造を決める。`Snapshot::capture()` で呼ぶ方法も。」

☕ ヤン: 「`ProcEntry` の型は source + fields だからそのまま使える。`Snapshot::capture()` に `src/sys/` のパーサー呼び出しを追加するだけ。」

```rust
// src/proc/mod.rs の capture() に追加
// -- /sys sources --
if let Ok(e) = crate::sys::thermal::parse() { entries.insert("thermal".into(), e); }
if let Ok(e) = crate::sys::df::parse() { entries.insert("df".into(), e); }
```

👤 今泉: 「えっと、`src/proc/mod.rs` の `Snapshot::capture()` で /sys のモジュールを呼ぶのは... 変じゃないですか？ proc ディレクトリの中で sys を呼ぶ...」

☕ ヤン: 「変だけど動く。`Snapshot::capture()` は "全ソースをキャプチャする" メソッドだから、ソースがどこから来るかは呼び出し側の自由。気持ち悪いなら `capture()` を `src/main.rs` に移動して `proc::*` と `sys::*` を並列に呼ぶ。でもそれは Phase 2 のリファクタリングでいい。」

⚔️ リヴァイ: 「動くコードを書け。リファクタリングは後だ。」

### Dashboard レイアウトの拡張

🏥 ハウス: 「Dashboard に df と温度を追加する。現在の Dashboard レイアウト:」

```
現在:
┌─────────────────────────────────────┐
│ Load Average  │  Memory             │
├───────────────┼─────────────────────┤
│ CPU Usage     │  Uptime             │
├───────────────┴─────────────────────┤
│ Network I/O (sparklines)            │
└─────────────────────────────────────┘

提案:
┌─────────────────────────────────────┐
│ Load Average  │  Memory             │
├───────────────┼─────────────────────┤
│ CPU Usage     │  Uptime / Temp      │
├───────────────┴─────────────────────┤
│ Disk Usage (per mount, bar chart)   │
├─────────────────────────────────────┤
│ Network I/O (sparklines)            │
└─────────────────────────────────────┘
```

🎋 千石: 「df のバーチャートは直感的でいい。`[████████░░] 82% /` のような表示。色: 80% 未満は緑、80-90% は黄、90% 以上は赤。」

☕ ヤン: 「温度は Uptime の横に小さく入れる。`Temp: 52℃` だけ。詳細は thermal ソースのフィールド一覧で見ればいい。」

---

## Gap Summary

| Gap ID | タイトル | 重要度 | Phase | 推定工数 |
|--------|---------|--------|-------|---------|
| G14 | プロダクトアイデンティティ再定義 ("Wireshark for Linux") | Medium | 1 | 小 (README, i18n テキスト) |
| G15 | ソースディレクトリ構成 (`src/sys/` 新設) | Medium | 1 | 小 (mod.rs + 構造) |
| G16 | df パーサー (statfs syscall) | **Critical** | 1 | 中 (~80 行) |
| G17 | 温度パーサー (/sys/class/thermal) | **Critical** | 1 | 小 (~60 行) |
| G18 | /proc/sys/fs/file-nr パーサー | **Critical** | 1 | 小 (~30 行) |
| G19 | systemd サービス状態パーサー | Important | 2 | 中 (~80 行) |
| G20 | dmesg 表示機能 | Important | 2 | 中 (~60 行) |
| G21 | 新メトリクス単体診断 (df, 温度, FD) | **Critical** | 1 | 中 (~110 行) |
| G22 | クロス診断パターン (複数メトリクス連携) | Important | 2 | 大 (~150 行) |

---

## Concrete Spec Proposals

### Spec S10-1: `src/sys/` ディレクトリ新設

```
src/sys/
  mod.rs          — thermal, df モジュール宣言
  thermal.rs      — /sys/class/thermal パーサー
  df.rs           — statfs ベースのファイルシステム使用率パーサー
```

`src/sys/mod.rs`:
```rust
#[cfg(target_os = "linux")]
pub mod thermal;
#[cfg(target_os = "linux")]
pub mod df;
```

### Spec S10-2: df パーサー (`src/sys/df.rs`)

**入力**: /proc/mounts (既存パーサー) + `statvfs()` syscall
**出力**: `ProcEntry` with source="df", fields は Table 形式

| フィールド | 型 | 説明 |
|-----------|-----|-----|
| filesystems | Table | `[device, mount, fstype, total, used, available, use%]` per row |

除外 FS: `proc, sysfs, devtmpfs, tmpfs, cgroup, cgroup2, debugfs, tracefs, securityfs, pstore, bpf, hugetlbfs, mqueue, fusectl, overlay, nsfs, fuse.portal`

### Spec S10-3: 温度パーサー (`src/sys/thermal.rs`)

**入力**: `/sys/class/thermal/thermal_zone*/temp`, `type`
**出力**: `ProcEntry` with source="thermal"

| フィールド | 型 | 説明 |
|-----------|-----|-----|
| `thermal_zone{N}_{type}` | Float(f64) | 温度 (℃) |

**不在処理**: `/sys/class/thermal/` が存在しない or 空の場合、`Ok(ProcEntry { source: "thermal", fields: vec![] })` を返す。

### Spec S10-4: file-nr パーサー (`src/proc/file_nr.rs`)

**入力**: `/proc/sys/fs/file-nr`
**出力**: `ProcEntry` with source="file-nr"

| フィールド | 型 | 説明 |
|-----------|-----|-----|
| allocated | Integer(i64) | 割り当て済み FD 数 |
| free_fds | Integer(i64) | 未使用 FD 数 (通常 0) |
| max | Integer(i64) | 最大 FD 数 |

### Spec S10-5: 診断パターン追加 (`src/diagnostics.rs`)

`analyze()` に追加:
```rust
check_disk_usage(&mut findings, snapshot, locale);
check_temperature(&mut findings, snapshot, locale);
check_fd_usage(&mut findings, snapshot, locale);
```

**閾値テーブル:**

| チェック | WARNING | CRITICAL |
|---------|---------|----------|
| df use% | > 80% | > 90% |
| 温度 | > 75℃ | > 90℃ |
| FD use% | > 80% | > 95% |
| systemd failed (Phase 2) | > 0 | N/A |
| dmesg OOM (Phase 2) | detected | N/A |

### Spec S10-6: Dashboard レイアウト拡張

Uptime セクションに温度を並記。df セクションを Network の上に追加。

df 表示形式:
```
 Disk Usage
 /      [████████░░] 82%  18.2G / 100G
 /home  [██████░░░░] 61%  156G / 400G
 /boot  [███░░░░░░░] 28%  180M / 640M
```

色分け: < 80% → 通常色, 80-90% → Yellow, > 90% → Red

---

## Implementation Priority Order

```
1. src/proc/file_nr.rs        — 最小工数。既存パターンに完全準拠。ウォームアップ。
2. src/sys/mod.rs              — 新ディレクトリの骨格。
3. src/sys/thermal.rs          — /sys パーサーの第一号。テンプレートになる。
4. src/sys/df.rs               — statfs 実装。mounts パーサー連携。
5. src/proc/mod.rs 修正        — capture() に file-nr, thermal, df を追加。
6. src/diagnostics.rs 拡張     — check_disk_usage, check_temperature, check_fd_usage。
7. src/ui/render.rs 拡張       — Dashboard に df セクションと温度表示を追加。
8. src/i18n.rs 拡張            — 新ソースの source_description (en/ja) 追加。
```

---

## New File Structure Proposal

```
src/
  proc/                        ← 既存。変更なし (file_nr.rs 追加のみ)
    mod.rs                     ← capture() に sys:: 呼び出し追加
    file_nr.rs                 ← NEW: /proc/sys/fs/file-nr パーサー
    ... (既存 43 パーサー)
  sys/                         ← NEW: /sys および非 /proc ソース
    mod.rs                     ← thermal, df モジュール宣言
    thermal.rs                 ← NEW: /sys/class/thermal パーサー
    df.rs                      ← NEW: statfs ベースの FS 使用率
    (Phase 2: systemd.rs)      ← systemctl コマンド実行パーサー
    (Phase 2: dmesg.rs)        ← dmesg コマンド実行パーサー
    (Phase 3: cpufreq.rs)      ← /sys/devices/system/cpu/*/cpufreq
    (Phase 3: battery.rs)      ← /sys/class/power_supply
    (Phase 3: nic_speed.rs)    ← /sys/class/net/*/speed
    (Phase 3: disk_type.rs)    ← /sys/block/*/queue/rotational
  diagnostics.rs               ← check_disk_usage, check_temperature, check_fd_usage 追加
  ui/
    render.rs                  ← Dashboard に df セクション + 温度表示追加
  i18n.rs                      ← 新ソースの説明追加
  ... (既存ファイル)
```
