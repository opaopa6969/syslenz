# DGE Session 015: 競合 Gap 分析 — syslenz v1.2.0 のポジショニングと差別化戦略

- **Date**: 2026-03-28
- **Theme**: syslenz v1.2.0 を主要競合（htop/btop, glances, netdata, node_exporter, Datadog, Prometheus+Grafana, Wireshark）と比較し、埋めるべき Gap と伸ばすべき強みを明確化する
- **Parent Gaps**: G14 (プロダクトアイデンティティ), G-PLG-1 (5層レイヤーモデル), 新規 Gap
- **Characters**: 大和田 (ビジネスリアリスト) + 鷲津 (数字の鬼) + 利根川 (ユーザー真実) + ラインハルト (ビジョナリー) + 今泉 (初心者の代弁者) + ヤン (怠惰な簡潔主義者) + ハウス (診断の天才) + 僕
- **Input**: syslenz v1.2.0 — 51+ Linux sources, 600 typed fields, 24 macOS/24 Windows sources, TUI (9 views, AA graphs, auto-sparkline) + Web UI (Chart.js) + X11 widget, i18n 584/600 EN/JA 3-level descriptions, 教育コンテンツ (6 category guides, 4-level help, diagnostic flowcharts), auto-diagnostics 15+ patterns, alert system, time-travel diff, remote (SSH/Docker/TCP), plugin system + SDK (syslenz4j on Maven Central), multi-host F1-F9, ViewData unified UI layer, zero config single binary, MIT license

---

## 現状の整理

先輩 (ナレーション): syslenz v1.2.0 の現在地と、主要競合の全体像を並べる。

**syslenz v1.2.0 のスペックシート:**

| カテゴリ | 現状 |
|---------|------|
| メトリクス | 51+ Linux sources, 600 typed fields, 24 macOS, 24 Windows |
| UI | TUI (9 views, AA graphs, sparkline) + Web UI (Chart.js) + X11 widget |
| i18n | 584/600 EN/JA 3-level descriptions |
| 教育 | 6 category guides, 4-level help, diagnostic flowcharts |
| 診断 | 15+ auto-diagnostic patterns |
| アラート | [[alert]] config rules |
| 時系列 | time-travel diff (60 snapshots ring buffer, [ ] keys) |
| リモート | SSH, Docker, TCP (--serve/--connect) |
| プラグイン | plugin system + SDK (syslenz4j on Maven Central) |
| マルチホスト | F1-F9 tab switching |
| デプロイ | zero config, single binary, MIT license |

**競合マップ:**

| 競合 | カテゴリ | メトリクス数 | UI | 教育 | 価格 |
|------|---------|------------|-----|------|------|
| htop/btop | TUI プロセスモニタ | ~50 | TUI (美麗) | なし | 無料 |
| glances | Python システムモニタ | ~200 | TUI + Web + API | なし | 無料 |
| netdata | リアルタイム監視基盤 | 2000+ | Web ダッシュボード | なし | Freemium |
| node_exporter | Prometheus exporter | ~500 | なし (Grafana 必要) | なし | 無料 |
| Datadog | Enterprise SaaS | 500+ integrations | Web ダッシュボード | ドキュメント | $23/host/月 |
| Prometheus+Grafana | OSS 監視スタック | ∞ (exporter 依存) | Grafana | なし | 無料 |
| Wireshark | パケット解析 | N/A (プロトコル) | GUI | 部分的 | 無料 |

問題の本質: **syslenz は「教えるモニタリングツール」という唯一無二のポジションにいるが、競合がカバーする機能領域でいくつかの Gap がある。すべてを埋める必要はないが、意図的な選択が必要。**

---

## Scene 1: メトリクス数の Gap — 2000 vs 600、数字は嘘をつく

先輩: netdata の 2000 メトリクスと syslenz の 600 フィールド。この数字の差をどう考えるか。

💼 大和田: 「おい、現実を見ろ。netdata は無料で 2000 メトリクスだぞ？ syslenz は 600。3 倍以上の差だ。GitHub のスター数も比較にならん。README に '600 metrics' と書いてある横で netdata が '2000+ metrics out of the box' と謳ってたら、ユーザーはどっちを選ぶ？ 数字は正義だ。勝てるのか？」

🦅 鷲津: 「数字を正確に見よう。netdata の 2000 は "メトリクス" — 個々の数値。syslenz の 600 は "typed fields" で、各フィールドに 3 レベル description が付いている。600 × 3 = 1800 コンテンツ。**コンテンツ密度** では負けていない。netdata は 2000 の数値を "見せる"。syslenz は 600 の数値を "教える"。比較軸が違う。」

💼 大和田: 「コンテンツ密度？ そんなものユーザーは比較しない。README で最初に見るのは数字だ。」

📋 利根川: 「大和田、黙れ。ユーザーが本当に欲しいのは 2000 メトリクスじゃない。ユーザーが本当に欲しいのは "なぜサーバーが重いか" の答えだ。2000 メトリクスが全部グラフで並んでいても、どれを見ればいいかわからなければノイズでしかない。」

👤 今泉: 「あの... そもそも netdata のユーザーと syslenz のユーザーは同じ人なんですか？ netdata を使う人は SRE やインフラエンジニアで、ダッシュボードを自分で読める人ですよね。syslenz のターゲットは "Linux を学びたい人" や "たまにサーバーを見る開発者" では？」

☕ ヤン: 「今泉、いい質問だ。同じ市場で戦う必要はない。ただし、数で負けている**カテゴリ**は確認すべきだ。syslenz が取れていないメトリクスカテゴリを洗い出そう。」

**syslenz が未カバーのメトリクスカテゴリ:**

| カテゴリ | 競合の実装 | syslenz の状態 | 重要度 |
|---------|-----------|--------------|--------|
| GPU (nvidia-smi, AMD ROCm) | netdata, Datadog | 未実装 | 高 (ML/AI 時代) |
| systemd サービス状態 | netdata, node_exporter | 未実装 | 高 (全 Linux) |
| container runtime (cgroup v2 詳細) | netdata, Datadog, cAdvisor | 部分的 (Docker remote のみ) | 中 |
| application-level (JVM, Node.js) | Datadog, netdata (plugins) | plugin SDK あり、公式 provider なし | 中 |
| ZFS/Btrfs 詳細 | netdata | 未実装 | 低 |
| SMART (ディスク健康) | netdata, node_exporter | 未実装 (Session 012 で Layer 5 計画) | 中 |

🏥 ハウス: 「GPU が "高" なのは同意する。2026 年に NVIDIA GPU のメトリクスを取れないのは、聴診器を持たない医者みたいなものだ。AI/ML ワークロードが増えている今、GPU temperature、utilization、memory は必須だ。nvidia-smi のパースは難しくない。systemd も同様。全 Linux サーバーで動いてるのに状態を見れないのはおかしい。」

🦁 ラインハルト: 「GPU と systemd。この 2 つは v1.3.0 で取る。他は plugin エコシステムに任せる。数の勝負はしない。600 の全フィールドに教育コンテンツが付いている — これが我々の戦い方だ。」

→ **Gap G-COMP-1 発見: GPU メトリクス (nvidia-smi) の組み込みサポート。temperature, utilization, memory, power を最低限カバー。**
→ **Gap G-COMP-2 発見: systemd サービス状態の取得。active/inactive/failed のリスト、起動時間、リソース使用量。**

---

## Scene 2: ダッシュボード/可視化の Gap — 固定 vs カスタマイズ

先輩: netdata と Grafana のカスタムダッシュボードの自由度に対し、syslenz の固定レイアウトについて議論する。

💼 大和田: 「Grafana のダッシュボードを見たことあるか？ ドラッグ＆ドロップで好きなメトリクスを好きな場所に配置できる。パネルのサイズも変えられる。アラートも埋め込める。テンプレート変数でホストを切り替えられる。netdata も同様だ。それに対して syslenz は？ 9 つの固定ビューだ。ユーザーが配置を変えたくても変えられない。」

☕ ヤン: 「待ってくれ。そこを同じ土俵で比較するのは罠だ。Grafana のカスタマイズ性は Grafana の本質であり、Grafana はそれだけで何年もかけて開発されている。syslenz がカスタムダッシュボードエディタを自前で作るのは投資対効果が低い。そもそも TUI でドラッグ＆ドロップは... まあ、やれなくはないが。」

📋 利根川: 「ユーザーの声を代弁する。"ダッシュボードをカスタマイズしたい" ユーザーは、すでに Grafana を使っている。syslenz に来るユーザーは "設定ゼロで動くこと" に価値を感じている。カスタマイズ性は syslenz の強みではなく、**zero config が強み** だ。」

👤 今泉: 「でも... Web UI だけでも、よく使うメトリクスを上に持ってきたり、使わないパネルを隠したりしたいです。全部カスタマイズできなくていいですけど、"お気に入り" くらいは欲しいです。」

🦅 鷲津: 「数字で整理しよう。」

| 機能 | Grafana | netdata | syslenz |
|------|---------|---------|---------|
| パネル配置変更 | ✅ 自由 | ✅ 自由 | ❌ 固定 |
| パネルサイズ変更 | ✅ | ✅ | ❌ |
| カスタムクエリ | ✅ (PromQL) | ✅ (NRQL 的) | ❌ |
| お気に入り/ブックマーク | ✅ | ✅ | ❌ |
| テーマ | ✅ | ✅ | 部分的 (TUI カラー) |
| ダッシュボード共有 | ✅ JSON export | ✅ snapshot | ❌ |

🦁 ラインハルト: 「Grafana と同じことをやる必要はない。だが "お気に入りビュー" と "パネル表示/非表示の設定" くらいは入れたい。config ファイルに `[views]` セクションを追加し、表示するソースの順番とフィルタを指定できるようにする。ゼロコンフィグのデフォルトは維持したまま、**設定すればカスタマイズ可能** にする。」

☕ ヤン: 「いいバランスだ。"zero config by default, configurable by choice"。TOML の `[views]` セクションで十分。JSON ダッシュボードエディタは作らない。」

🏥 ハウス: 「もう一つ。Grafana には "探索" がある。任意のメトリクスを ad-hoc にクエリできる。syslenz にもそれが欲しい。Web UI で "フィールド検索" を入れて、任意のフィールドを検索してグラフ表示できるようにしろ。これはカスタムダッシュボードより遥かにシンプルで、診断の現場では遥かに役に立つ。」

→ **Gap G-COMP-3 発見: ビューカスタマイズ — config の `[views]` セクションで表示ソースの順序・フィルタ・お気に入りを設定可能にする。zero config デフォルトは維持。**
→ **Gap G-COMP-4 発見: フィールド検索 & ad-hoc グラフ — Web UI でフィールド名を検索し、任意のフィールドのスパークラインを表示する機能。**

---

## Scene 3: データ保存/長期分析の Gap — 1 分 vs 永遠

先輩: syslenz の 60 スナップショットリングバッファ (約 1 分) と、Prometheus/netdata の長期保存機能の差について議論する。

🦅 鷲津: 「ここが一番大きな Gap だ。数字を見てくれ。」

| ツール | 保存期間 | ストレージ | クエリ言語 |
|--------|---------|-----------|-----------|
| syslenz | ~60 秒 (ring buffer) | メモリのみ | なし ([ ] で diff) |
| Prometheus | デフォルト 15 日、拡張で年単位 | TSDB (ディスク) | PromQL |
| netdata | dbengine で数ヶ月～年 | 圧縮ディスク | NIDL |
| Datadog | 15 ヶ月 (SaaS) | クラウド | ダッシュボードクエリ |
| Grafana + Mimir | 無制限 | オブジェクトストレージ | PromQL |

🦅 鷲津: 「syslenz の 60 秒は "今何が起きているか" しか見れない。"昨日の 3 時に何が起きたか" は見れない。これは明確な機能 Gap だ。」

📋 利根川: 「障害の 80% は事後分析だ。問題が起きた瞬間にモニタリングツールを開いている人間は少ない。通知が来て、調査するときには問題は 30 分前に起きている。60 秒のバッファでは間に合わない。」

💼 大和田: 「だが長期保存を自前で実装するのは巨大な投資だ。TSDB を Rust で書くのか？ それとも SQLite を使うのか？ どっちにしても数ヶ月の工数だ。」

☕ ヤン: 「自前で TSDB を書く必要はない。2 つのアプローチがある。」

**案 A: 軽量ファイル保存 (自前)**
```
~/.config/syslenz/history/
  2026-03-28T14:00:00.json.zst  (zstd 圧縮スナップショット)
  2026-03-28T14:01:00.json.zst
  ...
```
- 1 分ごとにスナップショットを zstd 圧縮で保存
- デフォルト 24 時間、config で変更可能
- `syslenz --history "2026-03-28 14:00"` で過去を閲覧
- 工数: 中 (2-3 週間)

**案 B: 外部 TSDB 連携 (export)**
```toml
[export]
prometheus = true   # /metrics エンドポイント公開
influxdb = "http://localhost:8086"
```
- Prometheus 形式の `/metrics` エンドポイント
- InfluxDB line protocol での push export
- 長期保存は外部ツールに任せる
- 工数: 小 (1-2 週間)

🏥 ハウス: 「両方やれ。案 A は "zero config で動く軽量履歴"、案 B は "本格的に使いたい人向けの連携"。案 A を先にやれ。Prometheus 連携は後でいい。なぜなら、syslenz のターゲットユーザーは Prometheus をセットアップできる人じゃないからだ。」

🦁 ラインハルト: 「同意する。案 A の "軽量ファイル履歴" は syslenz の哲学に合う。zero config で、single binary で、勝手に過去 24 時間を保存する。`syslenz --history` で time-travel できる。これは netdata の dbengine の syslenz 版だ。」

☕ ヤン: 「案 A を v1.3.0、案 B を v1.4.0 のロードマップにしよう。紅茶ください。」

→ **Gap G-COMP-5 発見: 軽量ファイル履歴 — zstd 圧縮スナップショットを ~/.config/syslenz/history/ に保存。デフォルト 24 時間。`--history` フラグで過去閲覧。**
→ **Gap G-COMP-6 発見: Prometheus /metrics エンドポイント — OpenMetrics 互換のメトリクス公開。Prometheus scrape 対応。**
→ **Gap G-COMP-7 発見: InfluxDB / 外部 TSDB export — push 型のメトリクスエクスポート。**

---

## Scene 4: 教育/診断の優位性 — 伸ばすべき強み

先輩: syslenz のユニークな強みである教育コンテンツと診断機能について議論する。ここは差を広げるべき領域。

🦁 ラインハルト: 「ここが我々の**聖域**だ。全競合を調査した結果を見ろ。」

| ツール | フィールド説明 | 教育コンテンツ | 診断支援 | 学習パス |
|--------|-------------|-------------|---------|---------|
| htop/btop | なし | なし | なし | なし |
| glances | 最小限 | なし | なし | なし |
| netdata | tooltip 程度 | なし | anomaly detection (ML) | なし |
| node_exporter | `HELP` メタデータ | なし | なし | なし |
| Datadog | ドキュメント | Knowledge Base (外部) | ML anomaly detection | Learning Center (別サービス) |
| Prometheus | `HELP` + `TYPE` | なし | なし | なし |
| **syslenz** | **3-level descriptions (584/600)** | **6 category guides** | **15+ auto patterns + flowcharts** | **4-level help** |

🦁 ラインハルト: 「この表を見ろ。教育コンテンツと組み込み診断支援を両方持っているのは syslenz だけだ。これは "あったらいいな" の機能ではない。**これが syslenz のアイデンティティだ**。"netdata は見せる、syslenz は教える" — これを 10 倍にしろ。」

📋 利根川: 「ユーザーの真実を言う。モニタリングツールを使う人の 90% は、表示されたメトリクスの意味を完全には理解していない。loadavg が 4.0 だと高いのか低いのか、CPU コア数によるということを知らない人が多い。syslenz の教育コンテンツは、この 90% の人間に刺さる。」

👤 今泉: 「僕がまさにそうです。loadavg の意味、syslenz の 4-level help で初めてわかりました。他のツールだと数字を見て "ふーん" で終わりです。」

🏥 ハウス: 「診断パターンの話をしよう。現在 15 パターンだが、これでは足りない。Datadog の ML anomaly detection と比較される。ML を入れる必要はないが、ルールベースのパターンを増やす必要がある。」

**現在の 15 パターン vs 追加すべきパターン:**

| 現状 | 追加候補 |
|------|---------|
| OOM 危険度 | swap thrashing 検出 (si/so oscillation) |
| CPU 飽和 | IRQ storm 検出 (特定 IRQ の急増) |
| ディスク I/O 遅延 | iowait + disk queue 相関分析 |
| ネットワークエラー | TCP retransmission rate → 品質劣化 |
| メモリリーク疑い | RSS 単調増加パターン (30 分トレンド — 案 A 必要) |
| ... (10 パターン) | fork bomb 検出 (processes 急増) |
| | connection leak (CLOSE_WAIT 蓄積) |
| | zombie process 蓄積 |
| | CPU steal 高 → ノイジーネイバー |
| | thermal throttling (temperature + CPU freq 相関) |

🏥 ハウス: 「15 → 25 パターンに増やせば、"syslenz は 25 の障害パターンを自動検出します" と言える。Datadog の ML に勝てなくても、"設定ゼロで動くルールベース診断" というカテゴリでは勝てる。」

🦁 ラインハルト: 「教育コンテンツも拡充する。」

**教育コンテンツの拡充ロードマップ:**

| 機能 | 現状 | 目標 |
|------|------|------|
| フィールド description | 584/600 (3-level) | 600/600 完全カバー |
| category guides | 6 カテゴリ | 10 カテゴリ (GPU, systemd, container, network-deep 追加) |
| diagnostic flowcharts | あり (テキスト) | インタラクティブ化 (TUI 内で選択肢を選ぶと次へ) |
| 学習パス | なし | "Linux 初心者コース" — 10 ステップの guided tour |
| "なぜ" の説明 | 部分的 | 全パターンに "なぜこれが問題か" "何をすべきか" を追加 |
| コミュニティ共有 | なし | diagnostic pattern の TOML 定義 + 共有リポジトリ |

☕ ヤン: 「学習パスは面白い。`syslenz --learn` で起動すると、ステップバイステップで "まず CPU を見ましょう。loadavg はこれです..." と教えてくれる。これは他のどのツールにもない。」

→ **Gap G-COMP-8 発見: 診断パターン拡充 — 15 → 25 パターン。swap thrashing, IRQ storm, connection leak, zombie 蓄積, thermal throttling 等。**
→ **Gap G-COMP-9 発見: インタラクティブ学習パス — `syslenz --learn` で Linux 初心者向け guided tour。10 ステップ。**
→ **Gap G-COMP-10 発見: 診断パターンの TOML 定義 — コミュニティが独自の診断パターンを共有できる仕組み。**

---

## Scene 5: エコシステム/統合の Gap — 500 vs 2

先輩: Datadog の 500+ インテグレーションに対する syslenz のエコシステムの現状について議論する。

💼 大和田: 「Datadog は 500 以上のインテグレーションを持っている。AWS CloudWatch、Kubernetes、MySQL、Redis、Nginx、Jenkins... 何でも繋がる。syslenz は？ SSH と Docker と TCP。3 つだ。500 vs 3。これは恥ずかしい数字だ。」

☕ ヤン: 「大和田さん、その比較は不公平だ。Datadog は 3000 人のエンジニアが 10 年以上開発している SaaS プロダクトだ。syslenz は... 紅茶が冷めた。」

🦅 鷲津: 「比較するなら条件を揃えよう。Datadog の 500 のうち、**syslenz のユーザーが必要とするもの**はいくつだ？」

| カテゴリ | Datadog | syslenz ユーザーに必要？ | 優先度 |
|---------|---------|----------------------|--------|
| OS メトリクス | ✅ 組み込み | ✅ 実装済み | — |
| Docker/Container | ✅ | ⚠️ remote のみ、ローカル不完全 | 高 |
| Kubernetes | ✅ | ⚠️ なし (ターゲット外？) | 低 |
| クラウド (AWS/GCP) | ✅ | なし | 低 |
| データベース | ✅ | なし (plugin で対応可能) | 中 |
| Web サーバー | ✅ | なし (plugin で対応可能) | 中 |
| CI/CD | ✅ | ❌ スコープ外 | — |
| APM | ✅ | ❌ スコープ外 | — |
| ログ | ✅ | ❌ スコープ外 | — |

🦅 鷲津: 「500 のうち syslenz が気にすべきは 3-5 カテゴリだ。OS メトリクスは済んでいる。Docker ローカル、データベース、Web サーバーが足りない。だがこれらは plugin で対応できる。」

🦁 ラインハルト: 「問題は plugin エコシステムがまだ小さいことだ。syslenz4j は Maven Central にあるが、公式 provider は限られている。コミュニティが plugin を書きたくなる仕組みが必要だ。」

💼 大和田: 「plugin エコシステムを育てるには 3 つ必要だ。」

**Plugin エコシステム成長の 3 条件:**

1. **優秀な公式サンプル** — 最初の 5-10 plugin は自分たちで書く。これが "こう書けばいいのか" のお手本になる
2. **開発体験 (DX)** — plugin を書くのが楽しくて簡単であること。scaffold コマンド、テストフレームワーク、ドキュメント
3. **発見可能性** — 書いた plugin を他の人が見つけられること。レジストリ or awesome-list

☕ ヤン: 「v1.3.0 では公式 provider を 5 つ追加することを目標にしよう。Docker (ローカル), nginx, PostgreSQL, Redis, systemd。これが ecosystem の種になる。」

🏥 ハウス: 「各 provider に教育コンテンツを付けるのを忘れるな。"PostgreSQL の connections が 100 に達したら何が起きるか" — これが syslenz の provider の価値だ。ただメトリクスを取るだけなら node_exporter で十分だ。」

→ **Gap G-COMP-11 発見: 公式 provider 拡充 — Docker (ローカル), nginx, PostgreSQL, Redis, systemd の 5 provider を公式で提供。**
→ **Gap G-COMP-12 発見: Plugin scaffold コマンド — `syslenz plugin new <name>` でテンプレートを生成。テストフレームワーク付き。**
→ **Gap G-COMP-13 発見: Plugin レジストリ — 公開 plugin のリスト。GitHub awesome-list から始め、将来的にはインストールコマンド (`syslenz plugin install`) に進化。**

---

## Scene 6: 実現ロードマップと優先順位 — 全部はやらない

先輩: 発見された Gap を整理し、優先順位を付ける。"全部やる" は戦略ではない。

☕ ヤン: 「13 個の Gap が出た。全部やったら 2 年かかる。紅茶を 3 杯飲む間に優先順位を決めよう。」

🦅 鷲津: 「基準は 3 つ。**ユーザーインパクト** (何人のユーザーに刺さるか)、**差別化効果** (syslenz だけの価値になるか)、**実装コスト** (工数)。」

📋 利根川: 「もう一つ加える。**ストーリー性** — README に書いたとき "おっ" と思わせるか。」

🦁 ラインハルト: 「いいだろう。整理しよう。」

**優先度マトリクス:**

| 優先度 | Gap | 理由 |
|--------|-----|------|
| 🔴 v1.3.0 必須 | G-COMP-5 (軽量ファイル履歴) | 60 秒→24 時間は全ユーザーに刺さる。zero config で動く |
| 🔴 v1.3.0 必須 | G-COMP-8 (診断パターン 25) | 差別化の核。工数小 |
| 🔴 v1.3.0 必須 | G-COMP-1 (GPU メトリクス) | AI/ML 時代に必須。ストーリー性高 |
| 🔴 v1.3.0 必須 | G-COMP-2 (systemd) | 全 Linux ユーザーに必要 |
| 🟡 v1.3.0 推奨 | G-COMP-3 (ビューカスタマイズ) | config ベースなら工数小 |
| 🟡 v1.3.0 推奨 | G-COMP-4 (フィールド検索) | 診断に必要。Web UI のみなら工数中 |
| 🟡 v1.4.0 | G-COMP-6 (Prometheus /metrics) | エコシステム接続。工数小 |
| 🟡 v1.4.0 | G-COMP-11 (公式 provider 5 つ) | エコシステムの種 |
| 🟡 v1.4.0 | G-COMP-9 (学習パス) | 差別化大だが工数も大 |
| 🟢 v1.5.0+ | G-COMP-7 (InfluxDB export) | Prometheus で十分な人が多い |
| 🟢 v1.5.0+ | G-COMP-10 (診断 TOML 定義) | コミュニティが育ってから |
| 🟢 v1.5.0+ | G-COMP-12 (plugin scaffold) | provider が増えてから |
| 🟢 v1.5.0+ | G-COMP-13 (plugin registry) | まだ早い |

---

## Gap Summary

| # | Gap 名 | 競合 | 現状 | 目標 | 優先度 | 工数 |
|---|--------|------|------|------|--------|------|
| G-COMP-1 | GPU メトリクス | netdata, Datadog | 未実装 | nvidia-smi: temp, util, mem, power | v1.3.0 必須 | 1 週間 |
| G-COMP-2 | systemd サービス状態 | netdata, node_exporter | 未実装 | active/inactive/failed リスト + 教育 | v1.3.0 必須 | 1 週間 |
| G-COMP-3 | ビューカスタマイズ | Grafana, netdata | 9 固定ビュー | config `[views]` でソース順序/フィルタ | v1.3.0 推奨 | 3 日 |
| G-COMP-4 | フィールド検索 & ad-hoc グラフ | Grafana Explore | なし | Web UI でフィールド検索 + スパークライン | v1.3.0 推奨 | 1 週間 |
| G-COMP-5 | 軽量ファイル履歴 | Prometheus, netdata dbengine | 60 秒 ring buffer | 24 時間 zstd 圧縮ファイル保存 + --history | v1.3.0 必須 | 2-3 週間 |
| G-COMP-6 | Prometheus /metrics | node_exporter, netdata | なし | OpenMetrics 互換 /metrics エンドポイント | v1.4.0 | 1 週間 |
| G-COMP-7 | InfluxDB export | Datadog, glances | なし | InfluxDB line protocol push | v1.5.0+ | 3 日 |
| G-COMP-8 | 診断パターン拡充 | Datadog ML | 15 パターン | 25 パターン (swap thrash, IRQ storm 等) | v1.3.0 必須 | 2 週間 |
| G-COMP-9 | インタラクティブ学習パス | なし (syslenz unique) | なし | `--learn` 10 ステップ guided tour | v1.4.0 | 3 週間 |
| G-COMP-10 | 診断パターン TOML 定義 | なし | ハードコード | TOML 定義 + コミュニティ共有 | v1.5.0+ | 2 週間 |
| G-COMP-11 | 公式 provider 5 つ | Datadog 500+ | SSH, Docker, TCP | + Docker local, nginx, PG, Redis, systemd | v1.4.0 | 4 週間 |
| G-COMP-12 | Plugin scaffold | Datadog SDK | syslenz4j のみ | `syslenz plugin new` コマンド | v1.5.0+ | 1 週間 |
| G-COMP-13 | Plugin レジストリ | Datadog integrations | なし | awesome-list → install コマンド | v1.5.0+ | 2 週間 |

---

## Next Actions (バックログアイテム)

### v1.3.0 マイルストーン (8-10 週間)

1. **[G-COMP-5] 軽量ファイル履歴の実装**
   - `~/.config/syslenz/history/` に zstd 圧縮スナップショット保存
   - デフォルト 24 時間、`[history]` セクションで設定可能
   - `syslenz --history "2026-03-28 14:00"` で過去閲覧
   - time-travel diff の既存 [ ] キーを履歴にも対応

2. **[G-COMP-8] 診断パターン 15 → 25**
   - swap thrashing (si/so oscillation)
   - IRQ storm (特定 IRQ の急増)
   - iowait + disk queue 相関
   - TCP retransmission rate
   - fork bomb (processes 急増)
   - connection leak (CLOSE_WAIT 蓄積)
   - zombie process 蓄積
   - CPU steal (ノイジーネイバー)
   - thermal throttling (temperature + CPU freq)
   - RSS 単調増加 (メモリリーク長期トレンド — 履歴機能依存)

3. **[G-COMP-1] GPU メトリクス (nvidia-smi)**
   - `nvidia-smi --query-gpu` の XML/CSV パース
   - フィールド: temperature, utilization (GPU/mem), memory (used/total), power, clock
   - 3-level description (EN/JA)
   - 教育コンテンツ: GPU カテゴリガイド追加

4. **[G-COMP-2] systemd サービス状態**
   - `systemctl list-units --output=json` パース
   - フィールド: unit, active_state, sub_state, load_state, description
   - failed サービスの自動検出 → 診断パターン追加
   - 3-level description (EN/JA)

5. **[G-COMP-3] ビューカスタマイズ (config)**
   - TOML `[views.default]` セクション追加
   - `sources = ["cpu", "memory", "disk"]` で表示ソース選択
   - `order = "custom"` でカスタム順序
   - 未設定時は現在のデフォルト動作を維持

6. **[G-COMP-4] フィールド検索 (Web UI)**
   - Web UI に検索バー追加
   - フィールド名/description でインクリメンタルサーチ
   - 検索結果からスパークライン表示

### v1.4.0 マイルストーン

7. **[G-COMP-6] Prometheus /metrics エンドポイント**
8. **[G-COMP-11] 公式 provider 5 つ**
9. **[G-COMP-9] インタラクティブ学習パス (`--learn`)**

### v1.5.0+ マイルストーン

10. **[G-COMP-7] InfluxDB export**
11. **[G-COMP-10] 診断パターン TOML 定義**
12. **[G-COMP-12] Plugin scaffold コマンド**
13. **[G-COMP-13] Plugin レジストリ**

---

## やらないことリスト（意図的に追わない機能）

| 機能 | 競合 | やらない理由 |
|------|------|------------|
| カスタムダッシュボードエディタ | Grafana | Grafana の土俵で勝てない。zero config 哲学に反する |
| ML ベース anomaly detection | Datadog, netdata | 巨大な投資。ルールベース 25 パターン + 教育で十分差別化可能 |
| ログ収集/分析 | Datadog, Loki | スコープ外。専用ツール (Loki, journalctl) に任せる |
| APM / 分散トレーシング | Datadog, Jaeger | 完全に別ドメイン。やるべきではない |
| Kubernetes ネイティブ統合 | Datadog, Prometheus | ターゲットユーザーが違う。k8s ユーザーは Prometheus を使う |
| SaaS / クラウドホスティング | Datadog, netdata Cloud | ビジネスモデルが違う。MIT OSS を維持する |
| 独自クエリ言語 | PromQL, NIDL | 投資に見合わない。ad-hoc 検索で十分 |
| モバイルアプリ | Datadog | 投資に見合わない。Web UI のレスポンシブで対応 |

---

## 結論

先輩 (ナレーション): 13 の Gap が見つかった。だがすべてを埋める必要はない。

🦁 ラインハルト: 「戦略は明確だ。**"教えるモニタリング" で圧倒的に勝つ。メトリクスは厳選して増やす。長期保存は zero config で追加する。Grafana にはならない。Datadog にはならない。syslenz にしかない価値を 10 倍にする。**」

☕ ヤン: 「v1.3.0 の目標を一言で言えば: "24 時間見える、25 パターン教える、GPU と systemd がわかる"。悪くない。紅茶もう一杯。」

💼 大和田: 「...まあいい。教育で差別化すると言うなら、それが本当に刺さるか v1.3.0 で証明しろ。数字で見せてくれ。」

📋 利根川: 「ユーザーが `syslenz --learn` を使って "Linux がわかるようになった" と言う日が来たら、それが答えだ。」

→ **v1.3.0 テーマ: "See More, Learn More" — 24h 履歴 + 25 診断パターン + GPU/systemd メトリクス**
