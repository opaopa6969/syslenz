# DGE Session 009: 教育コンテンツと診断パターン — フィールド i18n・解説・診断 Tips

- **Date**: 2026-03-28
- **Theme**: ヘルプパネルを「見るだけの辞書」から「教える診断ツール」へ進化させる
- **Parent Gaps**: G2 (オンボーディング), G6 (アラート連携), 新規教育系 Gap
- **Characters**: ハウス (診断の天才、皮肉屋) + 千石 (品質の番人) + 今泉 (初心者の代弁者) + ヤン (怠惰な簡潔主義者) + 僕
- **Input**: 43 パーサーのフィールド description が英語ハードコード。ヘルプパネルは 5 行固定。`source_description()` は en/ja 対応済みだがフィールドレベルは未対応。

---

## 現状の整理

先輩 (ナレーション): 現在の実装を確認する。

- `src/i18n.rs`: `source_description(locale, source)` で 43 ソースの 1 行説明を en/ja で提供。UI ラベルも `t(locale, key)` で en/ja 対応済み
- `src/proc/*.rs`: 各パーサーの `Field { description: "hardcoded english".into() }` — フィールド説明は **英語のみ**
- `src/ui/render.rs`: `draw_help_panel()` が source 説明 + 選択中フィールドの `description` を表示。5 行固定パネル
- `App.show_help: bool` でトグル。表示内容は「ソース名 + 1 行説明 + フィールド名: 英語説明」のみ
- 教育的解説: **なし**。診断パターン: **なし**。ローリング表示: **なし**

問題の本質:
1. **i18n 漏れ**: `?` パネルでフィールド説明が常に英語。`--lang ja` でも "Total usable RAM" と表示される
2. **説明が浅い**: "Total usable RAM" は辞書的定義。「だから何？」に答えていない
3. **診断知識ゼロ**: syslenz は数値を見せるだけ。「この数値がおかしいとき何が起きてるか」を教えない
4. **ヘルプパネルが静的**: 5 行に 2 行分の情報しか表示していない。残り 3 行が空白

---

## Scene 1: フィールド説明の i18n 問題

先輩: 43 パーサーに散らばるフィールド description の日本語化について議論する。

👤 今泉: 「すみません、根本的な質問なんですけど... 43 パーサーに description がハードコードされてますよね。meminfo だけで 30 フィールド、processes のテーブルもある。全部で何フィールドあるんですか？」

☕ ヤン: 「ざっくり数えると... meminfo が 30、cpuinfo が 20、vmstat が 100 以上、stat が 15、net 系が各 5-10... 合計 300-400 フィールドくらいかな。紅茶ください。」

👤 今泉: 「400 フィールドを全部日本語化するんですか...？ そもそも description はどこに持つべきなんですか？パーサー内？i18n.rs？別ファイル？」

→ **Gap 発見: フィールド説明の i18n 戦略が未定義。400 フィールド × 2 言語のデータをどこに格納するか。**

🏥 ハウス (杖をついて登場): 「くだらない。400 フィールドの日本語訳を全部書くって？ 患者 400 人のカルテを一人で書くようなもんだ。しかも半分は "Unknown memory field" みたいなゴミ説明だろ。全部訳す前に、訳す価値があるフィールドがどれかを診断しろ。」

→ **Gap 発見: 全フィールドの翻訳は ROI が低い。頻出・重要フィールドの優先順位付けが必要。**

☕ ヤン: 「3 つの選択肢を整理しよう。」

**案 A: パーサー内ハードコードを維持し、i18n レイヤーで上書き**
```rust
// i18n.rs に field_description(locale, source, field_name) を追加
// ja にマッチしたら日本語を返す、なければパーサーの英語フォールバック
pub fn field_description(locale: Locale, source: &str, field: &str) -> Option<&'static str> {
    match locale {
        Locale::Ja => field_desc_ja(source, field),
        Locale::En => None, // パーサーのデフォルトを使う
    }
}
```

**案 B: 外部 TOML/JSON ファイルで全管理**
```toml
# i18n/ja/meminfo.toml
[fields]
MemTotal = "システムが利用可能な物理メモリの総量"
MemFree = "カーネルにもユーザー空間にも使われていない空きメモリ"
```

**案 C: パーサーから description を除去し、完全に i18n レイヤーに移行**

🎋 千石: 「案 C は危険。パーサーのテストで description を検証できなくなる。そしてフィールド名と description の対応が物理的に離れると、パーサーにフィールドを追加したときに description を忘れる。人間は忘れる。必ず忘れる。」

🏥 ハウス: 「案 B は過剰設計だ。TOML ファイルを 43 個作って、起動時にパースして、ファイルが壊れてたらどうする？エラーハンドリングだけで 100 行。得られるのは "設定ファイルの方が管理しやすい" という幻想だけだ。設定ファイルは人を幸せにしない。」

☕ ヤン: 「案 A が一番怠惰で一番正しい。パーサーの英語 description はそのまま。i18n.rs に `field_description()` を足して、日本語がある場合だけ上書き。ないフィールドは英語のまま。段階的に翻訳を増やせる。ファイルは増えない。ビルドも壊れない。」

👤 今泉: 「でも、i18n.rs が巨大になりません？ 400 フィールドの日本語を全部 match 文に書いたら...」

☕ ヤン: 「全部は書かない。ハウスが言った通り、重要なフィールドだけ。meminfo の主要 15 個、loadavg の 6 個、net/tcp の 2 個、processes の 2 個、pressure の主要 6 個。50 フィールドもあれば実用上 80% カバーできる。残りは英語フォールバック。」

→ **Spec 提案: 案 A を採用。`i18n.rs` に `field_description(locale, source, field_name) -> Option<&'static str>` を追加。MVP は主要 50 フィールドの日本語訳。**

---

## Scene 2: 教育コンテンツの構造設計

先輩: ヘルプパネルに何を表示するか。現在は 1 行の概要だけ。教育ツールとしてどこまで踏み込むか。

🏥 ハウス: 「いいか、よく聞け。syslenz はモニタリングツールじゃない。モニタリングなら Prometheus でいい。syslenz が存在する理由は "ユーザーにシステムの中身を理解させる" ことだ。患者に病名を告げるだけの医者は二流だ。患者が自分の体を理解できるように説明するのが診断だ。」

🎋 千石: 「同意する。ただし中途半端な説明は有害。"MemAvailable はカーネルが計算した利用可能メモリ" — これは正しいが不十分。"MemFree + 回収可能なキャッシュ + 回収可能な Slab" と書いて初めて理解できる。不正確な簡略化は嘘と同じ。」

🏥 ハウス: 「全員嘘をつく (Everybody lies)。メトリクスも嘘をつく。MemFree が 100MB しかなくても MemAvailable が 8GB なら問題ない。MemFree だけ見て "メモリ不足だ!" と騒ぐ新人を何人見てきたか。嘘を見抜く方法を教えるのが教育だ。」

→ **Gap 発見: 現在の 1 行説明では「メトリクスの嘘」を見抜けない。値の意味、他フィールドとの関係、よくある誤解を教える必要がある。**

👤 今泉: 「じゃあ、ヘルプパネルに何を入れるんですか？ 全部入れたら読みきれないし...」

☕ ヤン: 「3 層構造にしよう。」

```
Layer 1: 概要 (Summary)     — 1 行。今の source_description と同じ粒度
Layer 2: 解説 (Detail)      — 2-3 行。フィールドの意味、計算方法、他フィールドとの関係
Layer 3: 診断 Tips (Diag)   — 1-2 行。異常値のパターンと原因候補
```

🏥 ハウス: 「例を出す。meminfo の MemAvailable。」

```
[Layer 1] 新しいプロセスに割り当て可能なメモリ量
[Layer 2] MemFree + Active(file) + Inactive(file) + SReclaimable のうち回収可能な分。
          カーネル 3.14+ で追加。MemFree より正確な "本当の空き"。
[Layer 3] MemAvailable < MemTotal の 10% → OOM Killer 発動の危険。
          スワップ使用量も合わせて確認。dmesg に OOM のログがないか。
```

🎋 千石: 「Layer 2 の "Active(file) + Inactive(file) + SReclaimable のうち回収可能な分" — これはカーネルバージョンで計算方法が違う。3.14 と 5.x では挙動が異なる。注釈が要る。」

🏥 ハウス: 「完璧を求めてリリースしないよりマシだ。"カーネル 3.14+ で追加" で十分。詳細はカーネルドキュメントを読め。syslenz は医学辞典じゃない。臨床の手引きだ。」

→ **Gap 発見: 教育コンテンツの粒度基準が未定義。どこまで詳しく書くか、カーネルバージョン差異をどう扱うか。**

👤 今泉: 「コンテンツはどこに保存するんですか？ i18n.rs にベタ書き？」

😰 僕: 「...あの、i18n.rs が 3000 行になったら...」

☕ ヤン: 「最初は i18n.rs にベタ書きでいい。外部ファイルは人が増えてから考える。1 ソースあたり概要 1 行 + 解説 3 行 + 診断 2 行 = 6 行。主要 10 ソースで 60 行。日英で 120 行。i18n.rs が 120 行増えるだけ。死なない。」

😰 僕: 「...でも将来 43 ソース × フィールドレベル解説まで入れたら...」

☕ ヤン: 「そのときに分割すればいい。`i18n/` ディレクトリ作って `i18n/education.rs` に切り出す。今やることじゃない。YAGNI。紅茶ください。」

→ **Spec 提案: MVP は i18n.rs 内に 3 層コンテンツをベタ書き。肥大化したら `src/i18n/education.rs` に分割。**

---

## Scene 3: 診断パターンの設計

先輩: ハウスの専門領域。具体的な診断パターンを列挙し、データ構造を決める。

🏥 ハウス (ホワイトボードの前に立つ): 「いいか、診断パターンとはこういうものだ。症状を見て、鑑別診断のリストを作り、一つずつ潰す。メトリクスも同じだ。異常値を見たら、原因候補を挙げ、確認方法を提示する。」

🏥 ハウス: 「構造は 4 段階だ: **条件 → 症状 → 原因候補 → 確認方法**。」

### net/tcp の診断パターン

🏥 ハウス: 「TCP 接続は嘘の宝庫だ。」

```
Pattern: SYN_SENT 多数
  条件: state="SYN_SENT" の行が 10 以上
  症状: 接続が完了しない。アプリケーションがタイムアウトする
  原因候補:
    - 接続先ホストがダウンしている
    - ファイアウォールが SYN パケットを DROP している
    - ネットワーク経路障害
  確認方法: ping, traceroute, iptables -L, ss -tn state syn-sent

Pattern: TIME_WAIT 大量蓄積
  条件: state="TIME_WAIT" が 1000 以上
  症状: エフェメラルポート枯渇。新規接続が EADDRNOTAVAIL で失敗
  原因候補:
    - 短命な HTTP 接続が大量に発生 (マイクロサービス間通信)
    - connection pooling が未設定 / 設定ミス
  確認方法: ss -s, sysctl net.ipv4.tcp_tw_reuse, アプリの connection pool 設定

Pattern: CLOSE_WAIT リーク
  条件: state="CLOSE_WAIT" が増加し続ける
  症状: アプリケーションがソケットを close していない。FD リーク
  原因候補:
    - アプリのバグ (close 忘れ)
    - 例外処理でソケットクリーンアップが漏れている
  確認方法: lsof -i -p <pid>, /proc/<pid>/fd の数を監視
```

### meminfo の診断パターン

🏥 ハウス: 「メモリは最も誤診が多い。MemFree が少ないだけで "メモリ不足" と騒ぐ新人が後を絶たない。」

```
Pattern: MemAvailable 低下
  条件: MemAvailable < MemTotal * 0.10
  症状: OOM Killer が発動する危険。新プロセスの fork が失敗する可能性
  原因候補:
    - メモリリーク (特定プロセスの RSS が単調増加)
    - 正当な負荷増加 (ワーカー数増、データ量増)
    - hugepages の過剰予約
  確認方法: /proc/<top-rss-pid>/status, dmesg | grep -i oom, smem

Pattern: Dirty pages 蓄積
  条件: Dirty > 100MB が継続
  症状: I/O スパイク。sync / fsync が遅い。データベースのコミット遅延
  原因候補:
    - ストレージが遅い (HDD、ネットワークFS)
    - dirty_ratio / dirty_background_ratio の設定ミス
    - I/O スケジューラのボトルネック
  確認方法: iostat -x, /proc/sys/vm/dirty_*, iotop

Pattern: Slab 肥大化
  条件: Slab > MemTotal * 0.20
  症状: カーネル内部のメモリ消費が異常。ユーザー空間に回せるメモリが減少
  原因候補:
    - dentry / inode キャッシュの肥大化 (ファイル数が極めて多い環境)
    - カーネルモジュールのメモリリーク
  確認方法: slabtop, /proc/slabinfo を監視、echo 2 > /proc/sys/vm/drop_caches (応急)
```

### processes の診断パターン

```
Pattern: Zombie プロセス多数
  条件: state に "Z" のプロセスが 10 以上
  症状: プロセステーブルの汚染。PID 枯渇のリスク (極端な場合)
  原因候補:
    - 親プロセスが wait() / waitpid() していない
    - シグナルハンドラの実装ミス
  確認方法: ps aux | grep Z, 親プロセスの特定と修正

Pattern: FD リーク
  条件: /proc/<pid>/fd の数が ulimit -n の 80% 以上
  症状: open() が EMFILE で失敗。ログファイルが開けない。ソケットが作れない
  原因候補:
    - ファイル / ソケットの close 忘れ
    - ログローテーション後に古い fd を保持
  確認方法: ls /proc/<pid>/fd | wc -l, lsof -p <pid>, ulimit -n
```

### pressure (PSI) の診断パターン

```
Pattern: CPU pressure — some avg10 高い
  条件: cpu_some_avg10 > 25.0
  症状: 一部のタスクが CPU 待ちで停滞。レスポンスタイムの悪化
  原因候補:
    - CPU バウンドな処理の集中
    - コア数不足
    - cgroup の CPU 制限が厳しすぎる
  確認方法: mpstat -P ALL, perf top, cgroup の cpu.max

Pattern: I/O pressure — some avg10 高い
  条件: io_some_avg10 > 25.0
  症状: I/O 待ちでタスクが停滞。ファイル読み書きが遅い
  原因候補:
    - ストレージ性能の限界
    - swap thrashing (メモリ不足 → スワップ I/O)
    - 非効率なファイルアクセスパターン
  確認方法: iostat -x, iotop, /proc/meminfo の SwapFree 確認

Pattern: Memory pressure — full avg10 > 0
  条件: memory_full_avg10 > 0
  症状: 全タスクがメモリ回収待ちで停滞。システム全体が遅い
  原因候補:
    - 深刻なメモリ不足
    - OOM Killer 直前の状態
  確認方法: dmesg, /proc/meminfo, free -h
```

### loadavg の診断パターン

```
Pattern: 高負荷
  条件: load_1min > CPU コア数 * 2
  症状: CPU saturation。新しいタスクがキューで待たされる
  原因候補:
    - CPU バウンドプロセスが多すぎる
    - I/O wait が load に含まれている (D state プロセス)
    - fork bomb
  確認方法: nproc (CPU数確認), ps aux --sort=-%cpu, iostat (D state 確認)
```

### diskstats / slabinfo / net/dev

```
Pattern: I/O await 高い (diskstats)
  条件: await > 50ms が継続
  症状: ディスク I/O が遅い。アプリケーションの応答遅延
  原因候補: ディスク性能限界、RAID 再構築中、I/O スケジューラ不適合
  確認方法: iostat -x, smartctl, dmesg

Pattern: dentry/inode キャッシュ肥大 (slabinfo)
  条件: dentry の active_objs が 100 万以上
  症状: Slab メモリ消費増加、MemAvailable 低下
  原因候補: 大量のファイルを扱うアプリ (find, locate, バックアップ)
  確認方法: slabtop, echo 2 > /proc/sys/vm/drop_caches

Pattern: errors/drops 増加 (net/dev)
  条件: rx_errors or tx_errors or rx_dropped が増加
  症状: パケットロス。TCP 再送。通信遅延
  原因候補: NIC の性能限界、ドライバの問題、ring buffer が小さい
  確認方法: ethtool -S <dev>, ethtool -g <dev>, dmesg | grep <dev>
```

🏥 ハウス: 「これだけあれば初期診断には十分だ。カンファレンスで症例報告できるレベルにするにはもっと要るが、それは臨床研修が進んでからでいい。」

👤 今泉: 「あの... これ、Session 005 のアラートシステム (G6) と連携できません？ 条件の部分、アラートの閾値と同じ構造じゃないですか。」

🏥 ハウス: 「いい質問だ。今泉にしては珍しい。診断パターンの "条件" はアラートの "threshold" と同じデータで表現できる。つまり:」

```
アラートが発火 → ヘルプパネルに該当する診断パターンを自動表示
```

🏥 ハウス: 「患者が検査室に来た時点で、異常値に対応する鑑別診断リストが自動的に画面に出る。これが本当のインテリジェント診断だ。G6 のアラートは "何がおかしい" を教える。診断パターンは "なぜおかしい" を教える。別々に作るな。統合しろ。」

→ **Gap 発見: 診断パターンとアラートシステム (G6) は同じ条件データを共有できる。統合設計が必要。**

🎋 千石: 「データ構造の整合性を確認させてくれ。Session 005 の `AlertRule` は:」

```rust
pub struct AlertRule {
    pub source: String,
    pub field: String,
    pub op: CompareOp,
    pub threshold: f64,
    pub severity: Severity,
    pub message: String,
}
```

🎋 千石: 「これに診断情報を足すと:」

```rust
pub struct DiagnosticPattern {
    // アラート条件 (AlertRule と共有可能)
    pub source: String,
    pub field: String,
    pub op: CompareOp,
    pub threshold: f64,
    // 診断情報
    pub symptom: &'static str,        // 症状の説明
    pub causes: &'static [&'static str],  // 原因候補リスト
    pub checks: &'static [&'static str],  // 確認方法リスト
}
```

🎋 千石: 「AlertRule と DiagnosticPattern は条件部分が共通。AlertRule に `diagnostic: Option<DiagnosticPatternId>` を持たせれば、アラート発火時に対応する診断パターンを参照できる。」

→ **Spec 提案: `DiagnosticPattern` 構造体を定義。`AlertRule` と条件部分を共有。アラート発火時に診断パターンをヘルプパネルに自動表示。**

---

## Scene 4: ローリング表示の UX 設計

先輩: ヘルプパネル (5 行、`?` でトグル) に 3 層コンテンツをどう表示するか。

☕ ヤン: 「現状のヘルプパネルは 5 行で 2 行しか使ってない。3 層コンテンツを入れると、Layer 2 だけで 3 行。Layer 3 も 2 行。全部同時に表示するには足りない。」

👤 今泉: 「ローリング表示ってどういう意味ですか？ 自動で切り替わるんですか？」

☕ ヤン: 「3 秒ごとに 概要 → 解説 → 診断 Tips と自動的に切り替わる。ニュースティッカーみたいなもの。」

🏥 ハウス: 「自動ローリング？ 患者の検査結果が 3 秒で消えたら読めないだろ。ルーキーがメトリクスの解説を読んでる途中で画面が切り替わったら、"何が書いてあったっけ" で振り出しに戻る。教育ツールとして最悪の UX だ。」

☕ ヤン: 「...確かに。自動ローリングはうざいか。手動切替だけでいい？ Tab キーで Layer 1 → 2 → 3 をサイクルとか。」

🏥 ハウス: 「手動サイクルも面倒だ。ユーザーが Tab を 3 回押す間に、知りたかった情報を忘れる。ベストは **全部同時に見せる** ことだ。パネルの高さを可変にしろ。」

🎋 千石: 「全部同時だと、Layer 1 (1行) + Layer 2 (3行) + Layer 3 (2行) = 6 行。ボーダー入れて 8 行。メインコンテンツ領域が圧迫される。ターミナル 35 行の環境だと 23% をヘルプに取られる。」

☕ ヤン: 「折衷案。デフォルトは Layer 1 だけ (コンパクト、3 行)。Tab で展開して全 Layer 表示 (8 行)。もう一度 Tab で閉じる。3 段階: 非表示 → コンパクト → 展開。」

👤 今泉: 「あ、それいいですね。`?` で表示トグルして、Tab で展開/縮小。直感的です。」

🏥 ハウス: 「もう一つ。フィールドを選択していないとき — つまりサイドバーにフォーカスがあるとき — はフィールドレベルの解説がない。その場合は source レベルの診断概要を表示しろ。"meminfo 全体としてこういうことに気をつけろ" という情報だ。」

→ **Gap 発見: ヘルプパネルの表示モードが未設計。コンパクト/展開の切替、フィールド未選択時の fallback が必要。**

☕ ヤン: 「App state の変更:」

```rust
pub enum HelpMode {
    Hidden,        // ヘルプパネル非表示 (現在の show_help: false)
    Compact,       // Layer 1 のみ (3行パネル)
    Expanded,      // Layer 1 + 2 + 3 (8行パネル)
}
```

☕ ヤン: 「`?` で Hidden → Compact → Hidden。Compact のとき Tab で Compact → Expanded → Compact。シンプル。」

🏥 ハウス: 「診断パターンに該当する値がある場合は、Compact モードでも Layer 3 の 1 行目を黄色で表示しろ。ユーザーに "何かおかしいぞ" と気づかせる。患者が熱を出してるのにカルテの 3 ページ目に書いてあったら誰も気づかない。」

→ **Spec 提案: 診断パターン該当時、Compact モードでも警告行を表示。色は Yellow (Warning) / Red (Critical)。**

🎋 千石: 「表示の優先順位を明確にしておく:」

```
Compact モード:
  Line 1: [source名] — 概要 (Layer 1)
  Line 2: [フィールド名] : フィールド説明 (i18n済み)
  Line 3: ⚠ 診断Tips の 1 行目 (該当する場合のみ、黄色/赤色)

Expanded モード:
  Line 1: [source名] — 概要 (Layer 1)
  Line 2-3: フィールド解説 (Layer 2)
  Line 4-5: 📋 原因候補 (Layer 3)
  Line 6: 🔍 確認方法
  Line 7: (空行 or 追加情報)

フィールド未選択時:
  source レベルの概要 + source レベルの主要診断パターン一覧
```

→ **Spec 提案: Compact は 3 行 (Length(3))、Expanded は 8 行 (Length(8))。`draw_help_panel()` が `HelpMode` を見て描画を分岐。**

---

## Scene 5: 実装の MVP と優先順位

先輩: 全部やると膨大。何を最初に作り、何を後回しにするか。

☕ ヤン: 「MVP を定義しよう。最小限で "教育ツール感" が出るのは:」

```
MVP (Phase 1):
  - field_description() の i18n 対応 (主要 5 ソースの重要フィールド: ~50 フィールド)
  - 3 層コンテンツ: 主要 5 ソース (meminfo, loadavg, net/tcp, processes, pressure)
  - HelpMode: Hidden / Compact / Expanded の 3 段階
  - 診断パターン: 5 個 (各ソースから 1 つずつ)

Phase 2:
  - 全 43 ソースの source レベル 3 層コンテンツ
  - 診断パターン: 15 個追加 (合計 20 個)
  - アラートシステム (G6) との統合
  - フィールドレベル翻訳: 次の 10 ソース

Phase 3:
  - 外部ファイル化 (i18n.rs が肥大化した場合)
  - コミュニティ貢献ガイド (診断パターンの追加方法)
  - フィールドレベル翻訳: 残り全ソース
```

🏥 ハウス: 「Phase 1 の 5 ソースの選定は正しい。meminfo は "メモリの誤診を防ぐ"。loadavg は "CPU saturation の基本"。net/tcp は "ネットワーク障害の初動"。processes は "プロセスの異常検知"。pressure は "PSI という最新の指標を教える"。この 5 つをマスターすれば、Linux の障害の 70% は初動対応できる。」

😰 僕: 「...あの、i18n.rs に全部書いたら... 何行くらいに...」

☕ ヤン: 「計算しよう。」

```
source_description: 既存 43 × 2 言語 = ~86 行 (既存)
field_description: 50 フィールド × ja のみ = ~50 行
education_content: 5 ソース × 3 層 × 2 言語 × 3 行平均 = ~90 行
diagnostic_patterns: 5 パターン × 2 言語 × 5 行 = ~50 行

合計追加: ~190 行
i18n.rs 現在: 267 行
i18n.rs MVP 後: ~460 行
```

☕ ヤン: 「460 行。全然許容範囲。Go の標準ライブラリには 1 ファイル 5000 行のやつもある。460 行で騒ぐな。紅茶ください。」

😰 僕: 「...Phase 2 まで行くと...」

☕ ヤン: 「Phase 2 で 800 行くらい。Phase 3 で 1200 行。1200 行になったらファイル分割する。それまでは 1 ファイルでいい。分割のタイミングは "IDE のスクロールが辛い" と感じたとき。定量的な基準は要らない。」

🎋 千石: 「ファイル分割のプランだけ決めておこう。今実行する必要はないが、設計段階で方向性を示しておく:」

```
Phase 3 のファイル構造 (将来):
  src/i18n.rs              — UI ラベル、source_description (既存)
  src/i18n/mod.rs          — re-export
  src/i18n/labels.rs       — UI ラベル (既存の t() を移動)
  src/i18n/sources.rs      — source_description (既存を移動)
  src/i18n/fields.rs       — field_description (新規)
  src/i18n/education.rs    — 3 層コンテンツ (新規)
  src/i18n/diagnostics.rs  — 診断パターン (新規)
```

🏥 ハウス: 「最後に一つ。教育コンテンツの品質管理について。千石の領域だ。」

🎋 千石: 「教育コンテンツは コードのバグと違って CI で検出できない。レビューアーが Linux カーネルの知識を持っていなければレビューできない。最低限の品質チェックとして:」

```
1. 各 Layer のテキストが空でないことのコンパイル時チェック (const assert)
2. 英語コンテンツと日本語コンテンツの対称性チェック (キーの漏れ検出)
3. 診断パターンの条件が AlertRule と同じ CompareOp で表現できることの型チェック
```

→ **Spec 提案: 教育コンテンツの品質は型システムとテストで保証。内容の正確性は人間レビュー。**

---

## Gap Summary

| ID | Gap | Severity | Phase | 関連 Session |
|---|---|---|---|---|
| G-EDU-1 | フィールド description が英語ハードコード、i18n 未対応 | High | 1 | 新規 |
| G-EDU-2 | ヘルプパネルの説明が 1 行で浅い、教育的価値が低い | High | 1 | 新規 |
| G-EDU-3 | 診断パターン (条件→症状→原因→確認方法) が存在しない | High | 1 | 新規 |
| G-EDU-4 | ヘルプパネルの表示モード (Compact/Expanded) が未実装 | Medium | 1 | 新規 |
| G-EDU-5 | 診断パターンとアラートシステム (G6) の統合設計が未定 | Medium | 2 | Session 005 |
| G-EDU-6 | フィールド翻訳の優先順位 (400 中どの 50 を先にやるか) | Medium | 1 | 新規 |
| G-EDU-7 | 教育コンテンツの粒度基準 (どこまで詳しく書くか) | Low | 2 | 新規 |
| G-EDU-8 | i18n.rs の将来的なファイル分割計画 | Low | 3 | 新規 |
| G-EDU-9 | 教育コンテンツの品質保証メカニズム | Low | 2 | 新規 |

---

## Concrete Spec

### S1: ヘルプパネルのコンテンツ構造 (Rust 型)

```rust
// i18n.rs に追加

/// ヘルプパネルの表示モード
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HelpMode {
    Hidden,    // パネル非表示
    Compact,   // 3 行: 概要 + フィールド説明 + 診断警告 (該当時)
    Expanded,  // 8 行: 概要 + 解説 + 診断Tips + 確認方法
}

impl HelpMode {
    pub fn toggle_visibility(self) -> Self {
        match self {
            HelpMode::Hidden => HelpMode::Compact,
            _ => HelpMode::Hidden,
        }
    }
    pub fn toggle_expand(self) -> Self {
        match self {
            HelpMode::Compact => HelpMode::Expanded,
            HelpMode::Expanded => HelpMode::Compact,
            HelpMode::Hidden => HelpMode::Hidden,
        }
    }
    pub fn panel_height(self) -> u16 {
        match self {
            HelpMode::Hidden => 0,
            HelpMode::Compact => 3,
            HelpMode::Expanded => 8,
        }
    }
}

/// 3 層教育コンテンツ
pub struct EducationContent {
    pub summary: &'static str,       // Layer 1: 概要 (1行)
    pub detail: &'static [&'static str],  // Layer 2: 解説 (2-3行)
    pub tips: &'static [&'static str],    // Layer 3: 診断Tips (1-2行)
}

/// 診断パターン
pub struct DiagnosticPattern {
    pub source: &'static str,
    pub field: &'static str,
    pub op: CompareOp,
    pub threshold: f64,
    pub symptom_en: &'static str,
    pub symptom_ja: &'static str,
    pub causes_en: &'static [&'static str],
    pub causes_ja: &'static [&'static str],
    pub checks: &'static [&'static str],   // コマンドは言語非依存
}

pub enum CompareOp { Gt, Lt, Gte, Lte }
```

### S2: App state の変更

```rust
// app.rs の App 構造体に追加
pub struct App {
    // ... 既存フィールド ...
    // show_help: bool を削除し、以下に置換:
    pub help_mode: HelpMode,   // Hidden / Compact / Expanded
}

// キーバインド:
// '?' → help_mode.toggle_visibility()
// Tab → help_mode.toggle_expand() (help_mode != Hidden のとき)
```

### S3: 主要 5 ソースの教育コンテンツ (例)

#### meminfo

```
[EN]
Summary: System memory usage: total, free, available, buffers, cache, swap
Detail:
  - MemAvailable is the true "free" memory (MemFree + reclaimable cache/slab).
  - Don't panic if MemFree is low — Linux uses free RAM for cache. Check MemAvailable instead.
  - Swap usage alone isn't bad. Swap activity (si/so in vmstat) indicates pressure.
Tips:
  - MemAvailable < 10% of MemTotal → OOM risk. Check dmesg for OOM killer messages.
  - Dirty > 100MB sustained → storage bottleneck. Check iostat -x.

[JA]
Summary: メモリ使用状況: 合計、空き、利用可能、バッファ、キャッシュ、スワップ
Detail:
  - MemAvailable が本当の「空き」(MemFree + 回収可能なキャッシュ/Slab)。
  - MemFree が少なくても慌てるな。Linux は空きRAMをキャッシュに使う。MemAvailable を見ろ。
  - スワップ使用量だけでは問題ではない。vmstat の si/so (スワップ I/O) が圧力の指標。
Tips:
  - MemAvailable < MemTotal の 10% → OOM の危険。dmesg で OOM killer を確認。
  - Dirty > 100MB 継続 → ストレージボトルネック。iostat -x を確認。
```

#### loadavg

```
[EN]
Summary: CPU load averages for 1, 5, and 15 minute intervals
Detail:
  - Load = number of processes in runnable + uninterruptible (D) state.
  - Compare to CPU count (nproc): load > nproc means CPU saturation.
  - D-state processes (I/O wait) inflate load — high load doesn't always mean CPU-bound.
Tips:
  - load_1min > nproc * 2 → severe CPU saturation. Check top/htop for CPU-bound processes.
  - load_15min >> load_1min → load is dropping (recovering). The reverse means escalating.

[JA]
Summary: CPU負荷平均: 1分、5分、15分間隔
Detail:
  - Load = 実行可能 + 割り込み不可 (D state) のプロセス数。
  - CPU コア数 (nproc) と比較: load > nproc なら CPU 飽和。
  - D state プロセス (I/O待ち) が load を膨らませる。高 load = CPU 不足とは限らない。
Tips:
  - load_1min > CPU数 × 2 → 深刻な CPU 飽和。top で CPU 消費プロセスを確認。
  - load_15min >> load_1min → 負荷は下降中 (回復傾向)。逆なら悪化中。
```

#### net/tcp

```
[EN]
Summary: Active TCP connections: local/remote address, state
Detail:
  - TCP state reveals connection health. ESTABLISHED = active, TIME_WAIT = closing, SYN_SENT = connecting.
  - High TIME_WAIT count exhausts ephemeral ports. Check net.ipv4.ip_local_port_range.
  - CLOSE_WAIT means the remote closed but local app hasn't — usually a bug.
Tips:
  - SYN_SENT > 10 → target unreachable or firewalled. Check ping/traceroute.
  - CLOSE_WAIT growing → fd leak in application. Check lsof -i -p <pid>.

[JA]
Summary: TCP接続: ローカル/リモートアドレス、状態
Detail:
  - TCP state が接続の健全性を示す。ESTABLISHED=通信中、TIME_WAIT=切断中、SYN_SENT=接続試行中。
  - TIME_WAIT 大量蓄積はエフェメラルポート枯渇。net.ipv4.ip_local_port_range を確認。
  - CLOSE_WAIT はリモートが閉じたがローカルが close していない — 通常バグ。
Tips:
  - SYN_SENT > 10 → 接続先が応答しない / FW で DROP。ping/traceroute で確認。
  - CLOSE_WAIT 増加中 → アプリの FD リーク。lsof -i -p <pid> で確認。
```

#### processes

```
[EN]
Summary: Running processes: PID, name, state, memory, threads
Detail:
  - State: S=sleeping (normal), R=running, D=uninterruptible (I/O wait), Z=zombie, T=stopped.
  - VmRSS = actual physical memory used by the process (not virtual).
  - High thread count per process isn't bad per se — check if it's growing over time.
Tips:
  - Zombie (Z) > 10 → parent not calling wait(). Find parent: ps -o ppid= -p <zombie_pid>.
  - Single process RSS growing monotonically → memory leak. Graph it in syslenz.

[JA]
Summary: プロセス一覧: PID、名前、状態、メモリ、スレッド数
Detail:
  - State: S=スリープ(正常), R=実行中, D=割込不可(I/O待ち), Z=ゾンビ, T=停止。
  - VmRSS = プロセスが実際に使用している物理メモリ (仮想ではない)。
  - スレッド数が多いこと自体は問題ではない。時系列で増加していないか確認。
Tips:
  - Zombie (Z) > 10 → 親が wait() していない。ps -o ppid= -p <pid> で親を特定。
  - 単一プロセスの RSS が単調増加 → メモリリーク。syslenz の graph で追跡。
```

#### pressure

```
[EN]
Summary: PSI: CPU, memory, and I/O pressure stall information
Detail:
  - PSI (Pressure Stall Information) measures how much tasks are stalled waiting for resources.
  - "some" = at least one task stalled. "full" = ALL tasks stalled (worse).
  - avg10/avg60/avg300 = 10s/60s/300s exponential moving averages (%).
Tips:
  - io_some_avg10 > 25 → I/O bottleneck. Check iostat and diskstats.
  - memory_full_avg10 > 0 → all tasks stalled on memory. OOM imminent. Act now.

[JA]
Summary: PSI: CPU、メモリ、I/Oの圧力ストール情報
Detail:
  - PSI (Pressure Stall Information) = タスクがリソース待ちで停滞している度合い。
  - "some" = 一部のタスクが停滞。"full" = 全タスクが停滞 (より深刻)。
  - avg10/avg60/avg300 = 10秒/60秒/300秒の指数移動平均 (%)。
Tips:
  - io_some_avg10 > 25 → I/O ボトルネック。iostat と diskstats を確認。
  - memory_full_avg10 > 0 → 全タスクがメモリ待ち。OOM 直前。即対応。
```

### S4: ローリング表示メカニズム (最終設計)

自動ローリングは **不採用**。理由: 教育コンテンツの読了を妨げる。

```
キーバインド:
  '?'  → Hidden ↔ Compact トグル
  Tab  → Compact ↔ Expanded トグル (Hidden 時は無効)

レイアウト:
  Hidden:   [main_area] [status_bar]
  Compact:  [main_area] [help_panel: 3行] [status_bar]
  Expanded: [main_area] [help_panel: 8行] [status_bar]

draw() の変更:
  let outer_constraints = match app.help_mode {
      HelpMode::Hidden => vec![Constraint::Min(10), Constraint::Length(1)],
      HelpMode::Compact => vec![Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)],
      HelpMode::Expanded => vec![Constraint::Min(10), Constraint::Length(8), Constraint::Length(1)],
  };
```

### S5: 診断パターンのデータ構造

```rust
pub const DIAGNOSTIC_PATTERNS: &[DiagnosticPattern] = &[
    DiagnosticPattern {
        source: "meminfo",
        field: "MemAvailable",
        op: CompareOp::Lt,
        threshold: 0.10,  // MemTotal の 10% (実行時に計算)
        symptom_en: "OOM Killer may activate. New process forks may fail.",
        symptom_ja: "OOM Killer が発動する危険。新プロセスの fork が失敗する可能性。",
        causes_en: &["Memory leak in a specific process", "Legitimate load increase", "Hugepages over-reservation"],
        causes_ja: &["特定プロセスのメモリリーク", "正当な負荷増加", "hugepages の過剰予約"],
        checks: &["dmesg | grep -i oom", "ps aux --sort=-rss | head", "cat /proc/meminfo | grep Huge"],
    },
    DiagnosticPattern {
        source: "loadavg",
        field: "load_1min",
        op: CompareOp::Gt,
        threshold: 2.0,  // CPU数の倍数 (実行時に nproc * 2)
        symptom_en: "CPU saturation. Tasks queued waiting for CPU time.",
        symptom_ja: "CPU 飽和。タスクが CPU 待ちでキューイング。",
        causes_en: &["Too many CPU-bound processes", "D-state (I/O wait) inflating load", "Fork bomb"],
        causes_ja: &["CPU バウンドプロセス過多", "D state (I/O 待ち) が load を膨張", "Fork bomb"],
        checks: &["nproc", "ps aux --sort=-%cpu | head", "iostat -x 1"],
    },
    // ... 他の 3 パターン (net/tcp SYN_SENT, processes zombie, pressure io)
];
```

### S6: i18n 戦略の推奨

```
推奨: 案 A (パーサー英語ハードコード維持 + i18n レイヤーで上書き)

API:
  field_description(locale, source, field_name) -> Option<&'static str>
    - Locale::En → None (パーサーの英語をそのまま使用)
    - Locale::Ja → Some("日本語") or None (翻訳なければ英語フォールバック)

  education_content(locale, source) -> Option<&EducationContent>
    - 3 層コンテンツを返す。未定義ソースは None

  diagnostic_patterns(source) -> &[DiagnosticPattern]
    - ソースに該当する診断パターンのスライスを返す

描画ロジック (render.rs):
  1. field_description() を呼ぶ → Some ならその文字列、None なら Field.description (英語)
  2. education_content() を呼ぶ → HelpMode に応じて Layer 1/2/3 を描画
  3. diagnostic_patterns() でマッチ確認 → 該当パターンがあれば警告色で表示
```

### S7: ファイル構造の提案

```
Phase 1 (MVP):
  src/i18n.rs                 ← 既存ファイルに追加 (~190 行増)
    + field_description()
    + education_content()
    + DIAGNOSTIC_PATTERNS
    + HelpMode enum

  src/ui/app.rs               ← show_help: bool → help_mode: HelpMode に変更
  src/ui/render.rs            ← draw_help_panel() を HelpMode 対応に拡張

Phase 3 (肥大化後):
  src/i18n/mod.rs             ← re-export
  src/i18n/labels.rs          ← t(), en(), ja()
  src/i18n/sources.rs         ← source_description()
  src/i18n/fields.rs          ← field_description()
  src/i18n/education.rs       ← EducationContent, education_content()
  src/i18n/diagnostics.rs     ← DiagnosticPattern, DIAGNOSTIC_PATTERNS
```

---

## Next Actions

- [ ] Phase 1-A: `HelpMode` enum を追加し、`App.show_help` を `App.help_mode` に置換
- [ ] Phase 1-B: `draw_help_panel()` を Compact/Expanded 対応に拡張
- [ ] Phase 1-C: `field_description()` を i18n.rs に追加 (主要 50 フィールド ja 訳)
- [ ] Phase 1-D: 5 ソースの `EducationContent` を i18n.rs に追加
- [ ] Phase 1-E: 5 個の `DiagnosticPattern` を定義
- [ ] Phase 1-F: キーバインド追加 (`Tab` で Compact ↔ Expanded)
- [ ] Phase 2: G6 アラートとの統合 (Session 005 の `AlertRule` と `DiagnosticPattern` の条件共有)
