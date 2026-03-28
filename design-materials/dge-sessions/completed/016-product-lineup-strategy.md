# DGE Session 016: プロダクトラインナップ戦略 — SDK / Provider / UI の優先順位と成長ロードマップ

- **Date**: 2026-03-28
- **Theme**: syslenz エコシステム全体のプロダクトラインナップを定義し、SDK 6言語・Provider 10種・UI/Export 3種の優先順位、工数見積もり、バージョン別ロードマップを策定する
- **Parent Gaps**: G-PLG-1 (プラグインアーキテクチャ), G14 (プロダクトアイデンティティ), G-ECO-1 (エコシステム成長戦略)
- **Characters**: 大和田 (ビジネス戦略家) + 鷲津 (数字の鬼) + 利根川 (ユーザーの現実) + ヤン (怠惰な簡潔主義者) + ラインハルト (ビジョナリー) + リヴァイ (実装の鬼) + 僕 (スコープ削減係)
- **Input**: syslenz core (Rust, TUI/Web/X11, 51+ sources) 実装済み。syslenz4j (Java SDK, Maven Central) リリース済み。provider-jvm (bash, jstat/jcmd) と provider-docker (bash, docker stats) 実装済み。Session 012 で5層レイヤーモデルとプラグインアーキテクチャを策定済み。次のステップとして、全ラインナップの優先順位とロードマップの確定が必要。

---

## 現状の整理

先輩 (ナレーション): syslenz エコシステムの現在地を整理する。

### 現行ラインナップ

```
syslenz (core)         — Rust, TUI/Web/X11, 51+ sources      ✓ shipped
sdk/java (syslenz4j)   — Maven Central published, Watch API   ✓ shipped
providers/jvm          — bash script, jstat/jcmd              ✓ shipped
providers/docker       — bash script, docker stats            ✓ shipped
```

### 計画中ラインナップ

```
SDK (アプリ内組み込み "4x"):
  syslenz4j      — Java 17+ (Maven Central) ✓ done
  syslenz4cs     — .NET / C# (NuGet)
  syslenz4py     — Python (PyPI)
  syslenz4node   — Node.js (npm)
  syslenz4go     — Go
  syslenz4swift  — Swift (macOS native metrics via IOKit)

Providers (外部メトリクス取得):
  provider-jvm       ✓ done (bash)
  provider-docker    ✓ done (bash)
  provider-mysql     — MySQL SHOW STATUS / performance_schema
  provider-postgres  — PostgreSQL pg_stat_*
  provider-redis     — Redis INFO
  provider-nginx     — nginx stub_status
  provider-apache    — Apache server-status
  provider-mongodb   — MongoDB serverStatus
  provider-k8s       — Kubernetes API
  provider-elasticsearch — ES _cluster/stats

UI/Export:
  syslenz-web        — Web UI (currently feature-gated)
  syslenz-otel       — OpenTelemetry bridge
  syslenz-grafana    — Grafana data source plugin
```

**問題: 20個以上のコンポーネントを計画しているが、開発リソースは極めて限られている。何を先に作り、何を後回しにし、何を作らないかを決める必要がある。**

---

## Scene 1: SDK の優先順位 — 開発者人口とユースケースマッチ

先輩: 6つの SDK の優先順位を決める。全部作る前に、どれが最もインパクトがあるかを見極める。

💼 大和田 (テーブルを叩いて): 「全部作る金と人がどこにある？優先順位をつけろ。SDK 6言語なんて夢物語だ。今のチーム規模で6言語のメンテナンスを継続できるのか？リリースしたら終わりじゃない。API 変更のたびに6つ全部アップデートだぞ。」

🦅 鷲津 (ノートPCを開いて): 「数字で話そう。各言語の開発者人口と、syslenz のユースケースとのマッチ度を見る。」

```
言語          開発者人口 (2025)    syslenz ユースケースマッチ    理由
Java          約1,200万            ★★★★★                     エンタープライズ、サーバーサイド。JVM 監視と直結
Python        約1,500万            ★★★★☆                     データサイエンス、ML。GPUサーバー監視需要
JavaScript    約2,000万            ★★★☆☆                     Web開発中心。サーバー監視は Node バックエンド限定
Go            約300万              ★★★★★                     インフラエンジニアのメイン言語。監視ツール親和性高
C#            約700万              ★★★☆☆                     Windows 中心。Linux サーバーは少数
Swift         約400万              ★★☆☆☆                     iOS/macOS。サーバー監視はニッチ
```

🦅 鷲津: 「TAM (Total Addressable Market) だけで見れば JavaScript が最大。だが syslenz のユースケース — サーバー/インフラ監視 — にマッチするかが重要だ。JavaScript 開発者の大半はフロントエンドで、サーバーの /proc なんて見ない。Python は2番目にすべきだ。ML/データ系のエンジニアがGPUサーバーの監視に最も困ってる。Jupyter notebook から syslenz4py でメトリクスを取れたら、それだけで差別化になる。」

🎰 利根川 (腕組みして): 「開発者が本当に使うのはどれだ？npm install で入らないなら誰も使わない。pip install で入らないなら誰も使わない。各言語のパッケージマネージャにちゃんと載るかが生命線だ。Maven Central は syslenz4j でクリアした。PyPI も問題ない。npm も問題ない。Go は go get で終わり。問題は NuGet と Swift Package Manager — ユーザー数に対してメンテコストが見合わない。」

☕ ヤン (紅茶を飲みながら): 「SDK 6個？2個でいい。Java と Python で世界の8割カバーする。いや、正確に言おう。Java はサーバーサイドの王者。Python はデータ/ML/スクリプトの王者。この2つで、syslenz を使いたいエンジニアの8割以上がカバーできる。残りは "需要が来てから" でいい。紅茶ください。」

🦁 ラインハルト: 「全言語対応は必須だ。syslenz がどこでも使えることが価値だ。"Java と Python しかサポートしてません" では、Go でインフラツールを書いている人を取りこぼす。Go のインフラエンジニアは少数でも、彼らは最も影響力がある。OSS コントリビューターの中核だ。」

⚔️ リヴァイ: 「ラインハルトの言うことは正しいが、順番の話だ。全部やる。だが一度にやるな。SDK の共通インターフェースを先に設計しろ。全 SDK で同じ API 体験を提供する。`SyslenzClient.connect()`, `.watch()`, `.snapshot()` — この3つのメソッドが全言語で同じ名前、同じセマンティクスで動く。共通インターフェースを doc として定義してから、言語ごとに実装する。そうすれば後発の SDK も迷わない。」

僕: 「...全部作るの...無理...」

☕ ヤン: 「無理じゃない。順番だ。v1.3 で Python。v1.4 で Node。v3.0 で Go と C#。Swift は v4.0 以降。これでいい。全部は作る。だが今日全部作る必要はない。」

🦅 鷲津: 「SDK の共通インターフェース設計は先行投資として正しい。ドキュメント1枚書くだけだ。コードゼロで全 SDK の API 仕様が決まる。」

→ **SDK 優先順位確定:**
```
1. syslenz4j      — ✓ done (Java 17+, Maven Central)
2. syslenz4py     — Python 3.9+ (PyPI) — v1.3
3. syslenz4node   — Node.js 18+ (npm) — v1.4
4. syslenz4go     — Go 1.21+ — v3.0
5. syslenz4cs     — .NET 8+ (NuGet) — v3.0
6. syslenz4swift  — Swift 5.9+ (SPM) — v4.0+
```

→ **Gap G-SDK-1 発見: 全 SDK 共通インターフェース仕様書 (SDK Common Interface Spec) の策定が必要。SyslenzClient / Watch API / Snapshot API の言語非依存な仕様を先に定義し、各 SDK 実装のガイドラインとする。**

---

## Scene 2: Provider の優先順位 — 80/20 ルールとテンプレート

先輩: 10個の Provider の優先順位を決める。どれが最もユーザーの課題を解決するか。

💼 大和田: 「Provider は SDK より単純だ。bash スクリプトで JSON を吐くだけだからな。問題は "どれを先に作るか" じゃない。"どれが最もユーザーの課題解決に直結するか" だ。MySQL と PostgreSQL が先。世界のデータベースの80%をカバーする。DB が遅いとき、アプリ開発者が最初に見たいのはクエリのスループットとスロークエリ数だ。」

⚔️ リヴァイ: 「Provider は bash でいい。SDK は型が必要だからちゃんと作れ。だが Provider は違う。SQL 叩いて JSON 吐くだけだ。MySQL の Provider なんて 10 分で書ける。」

```bash
# provider-mysql のイメージ (実質これだけ)
#!/bin/bash
mysql -u "$SYSLENZ_MYSQL_USER" -p"$SYSLENZ_MYSQL_PASS" -h "${SYSLENZ_MYSQL_HOST:-localhost}" \
  -e "SHOW GLOBAL STATUS WHERE Variable_name IN ('Queries','Slow_queries','Threads_connected','Threads_running','Uptime')" \
  --batch --skip-column-names | \
jq -R -s '
  split("\n") | map(select(. != "") | split("\t")) |
  {
    source: "mysql",
    description: "MySQL server status via SHOW GLOBAL STATUS",
    fields: map({name: .[0], value: {Float: (.[1] | tonumber)}, description: .[0]})
  }
'
```

⚔️ リヴァイ: 「PostgreSQL も同じ。`pg_stat_activity`, `pg_stat_database` を SELECT して JSON にする。Redis は `INFO` コマンド一発。nginx は `stub_status` の curl。全部 10-30 行の bash だ。」

☕ ヤン: 「10個の Provider を全部見て、工数と価値のマトリクスを作ろう。」

```
Provider             ユーザー価値    工数    対象ユーザー数    優先度
provider-mysql       ★★★★★        S       巨大              1
provider-postgres    ★★★★★        S       巨大              2
provider-redis       ★★★★☆        S       大                3
provider-nginx       ★★★★☆        S       大                4
provider-apache      ★★★☆☆        S       中 (減少傾向)     7
provider-mongodb     ★★★☆☆        M       中                6
provider-elasticsearch ★★★☆☆      M       中                8
provider-k8s         ★★★★★        XL      大                5 (だが工数が...)
```

🎰 利根川: 「K8s は別次元だ。bash スクリプトじゃ無理。API サーバーへの認証、ServiceAccount、RBAC、kubeconfig の処理... kubectl を叩くだけでも認証の設定が必要だ。K8s Provider は "bash で10分" の世界じゃない。L か XL の工数だ。」

⚔️ リヴァイ: 「だから K8s は後だ。MySQL, PostgreSQL, Redis, nginx は全部 S サイズ。1日で4つ書ける。Apache は nginx とほぼ同じ。MongoDB と Elasticsearch は API がやや複雑で M サイズ。K8s だけが XL だ。」

💼 大和田: 「もう一つ。Provider のテンプレートを作るべきだ。scaffold コマンドがあれば、コミュニティが自分で Provider を書ける。我々が全部作る必要はない。」

☕ ヤン: 「テンプレートは 1 ファイルでいい。`provider-template.sh` に記入箇所をコメントで書いておく。scaffold コマンドなんか要らない。cp してコメントを埋めるだけだ。5分で Provider が書ける。これでいい。」

```bash
#!/bin/bash
# syslenz Provider Template
# 1. Copy this file: cp provider-template.sh provider-yourname
# 2. chmod +x provider-yourname
# 3. Edit the sections below
# 4. Place in ~/.config/syslenz/plugins/

SOURCE_NAME="yourname"
DESCRIPTION="Description of what this provider monitors"

# --- Collect metrics here ---
# Example: result=$(some_command)

# --- Output JSON ---
cat <<EOF
{
  "source": "$SOURCE_NAME",
  "description": "$DESCRIPTION",
  "fields": [
    {"name": "metric_name", "value": {"Float": 0.0}, "description": "What this metric means"}
  ]
}
EOF
```

⚔️ リヴァイ: 「テンプレートは十分。それとは別に、既存の provider-jvm と provider-docker の中身をリファレンス実装としてドキュメント化しろ。"こう書けばいい" の実例が最強のドキュメントだ。」

→ **Provider 優先順位確定:**
```
Tier 1 (v1.3): provider-mysql, provider-postgres     — S サイズ、DB の 80% をカバー
Tier 2 (v1.4): provider-redis, provider-nginx         — S サイズ、キャッシュ + Web サーバー
Tier 3 (v2.0): provider-mongodb, provider-apache      — M/S サイズ
Tier 4 (v3.0): provider-k8s, provider-elasticsearch   — XL/M サイズ
```

→ **Gap G-PRV-1 発見: Provider テンプレート (provider-template.sh) の作成と、既存 Provider のリファレンスドキュメント化が必要。コミュニティ貢献を促進するための "5分で Provider を書ける" 体験の提供。**

---

## Scene 3: UI/Export の分離戦略 — Web UI vs Grafana vs OTEL

先輩: UI と Export の戦略を議論する。自前の Web UI を育てるか、既存エコシステムに乗るか。

☕ ヤン: 「核心的な問いをする。Grafana plugin 作ったら、syslenz の Web UI いらなくない？ Grafana のダッシュボードの方がどう考えても機能が上だ。カスタムパネル、アラート、共有、RBAC... 全部 Grafana にある。自前の Web UI で同じものを作るのは車輪の再発明だ。」

🦁 ラインハルト: 「違う。syslenz の Web UI は "教育" のためにある。Grafana はメトリクスを見せるだけだ。syslenz の Web UI は、各メトリクスの意味を説明し、4段階ヘルプを表示し、診断結果を出す。この "学習体験" は Grafana では再現できない。Grafana の data source plugin は数値を渡すだけだ。syslenz の教育コンテンツは渡せない。」

🎰 利根川: 「つまり2つの UI が必要だということか。syslenz-web は "教育 + オンボーディング" 用。Grafana plugin は "本番運用の監視" 用。ターゲットユーザーが違う。」

💼 大和田: 「正しい。だが優先順位が逆だ。Grafana plugin を先に作れ。なぜなら、Grafana ユーザーは既に "サーバー監視" の習慣がある人だ。syslenz の Grafana plugin があれば、既存の Grafana ダッシュボードに syslenz のメトリクスを追加できる。採用のハードルが最も低い。」

☕ ヤン: 「OTEL bridge はどうだ。syslenz → OpenTelemetry Collector → Prometheus → Grafana。この既存パイプラインに乗れば、Grafana plugin すら要らない。OTEL Exporter を実装するだけだ。」

⚔️ リヴァイ: 「OTEL bridge は既に feature flag で実装途中だ。これを完成させるのが最短パスだ。OTEL Exporter → Prometheus → Grafana のパイプラインは、新しいコードをほぼ書かずに Grafana 連携が実現する。Grafana の data source plugin をゼロから書くより、OTEL bridge を完成させる方が10倍速い。」

🦅 鷲津: 「コスト比較をしよう。」

```
コンポーネント          工数      得られるもの
syslenz-otel          M        OTEL → Prometheus → Grafana パイプライン全体
                               (Prometheus, Datadog, New Relic も同時に対応)
syslenz-grafana       L        Grafana 直接連携 (カスタムパネル可能)
                               だが OTEL 経由でも同じことができる
syslenz-web 分離      M        独立した Web UI パッケージ
                               教育機能は Web UI でしか提供できない
```

⚔️ リヴァイ: 「結論。OTEL bridge を先に完成させろ。Grafana plugin は OTEL bridge で Prometheus 連携できた後で評価する。"OTEL で十分" なら Grafana plugin は要らない。"OTEL では教育メタデータが渡せない" なら Grafana plugin を作る。Web UI は feature flag のまま育てる。分離は v2.0 以降。」

🦁 ラインハルト: 「Web UI の分離は必要だ。syslenz core のバイナリサイズを小さく保ちたい。Web UI の JavaScript/CSS を core に含めるべきではない。だが分離のタイミングは v2.0 でいい。今は feature flag で十分だ。」

僕: 「...つまり今やることは OTEL bridge の完成だけ...?」

☕ ヤン: 「そう。1つだけ完成させる。残りは後。」

→ **UI/Export 優先順位確定:**
```
1. syslenz-otel       — v1.3 で完成。OTEL → Prometheus → Grafana パイプライン
2. syslenz-web 分離   — v2.0 で feature flag から独立パッケージへ
3. syslenz-grafana    — v2.0 で評価。OTEL bridge で不十分な場合のみ実装
```

→ **Gap G-UI-1 発見: OTEL bridge (feature flag 実装中) の完成。Prometheus remote write もしくは OTLP exporter の安定化が必要。**

---

## Scene 4: エコシステムの成長戦略 — コミュニティ駆動への転換

先輩: 全部自分たちで作る戦略は持続不可能だ。コミュニティに書いてもらう仕組みを議論する。

💼 大和田: 「ビジネスの現実を言う。SDK 6言語 + Provider 10種 + UI 3種 = 19コンポーネント。仮にフルタイム1人で作るとして、SDK 1つに2-4週間、Provider 1つに1-3日。全部で半年以上かかる。しかもリリースして終わりじゃない。メンテナンスが永続的にかかる。」

🎰 利根川: 「答えはコミュニティだ。Provider は bash スクリプトだから、参入障壁が低い。"5分で Provider を書ける" テンプレートを出して、contribution guide を整備すれば、MySQL の DBA が provider-mysql を書いてくれるかもしれない。Redis のエキスパートが provider-redis を書いてくれるかもしれない。各ドメインの専門家が一番いい Provider を書ける。」

☕ ヤン: 「コミュニティ貢献を促進する仕組みは3つ。」

```
1. Provider テンプレート (provider-template.sh)
   → cp して5分で書ける。Scene 2 で定義済み

2. Contribution Guide (CONTRIBUTING.md の Provider セクション)
   → Provider の submit 方法、テスト方法、命名規則

3. awesome-syslenz リスト (GitHub リポジトリ)
   → コミュニティ製 Provider のカタログ
   → 公式レビューを通過したものに "verified" バッジ
```

⚔️ リヴァイ: 「Provider の acceptance criteria を明確にしろ。こうだ。」

```
Provider Acceptance Criteria:
  1. provider-template.sh の形式に準拠している
  2. JSON 出力が ProcEntry スキーマに適合する
  3. exit 0 / exit 1 のエラーハンドリングがある
  4. 接続情報は環境変数で渡す (ハードコードしない)
  5. README に必要な権限と前提条件が記載されている
  6. CI でスキーマバリデーションが通る
```

💼 大和田: 「SDK の方はどうだ。SDK をコミュニティに書いてもらうのは現実的か？」

🦁 ラインハルト: 「SDK は難しい。共通インターフェース仕様に準拠しないと互換性が壊れる。公式 SDK は我々が書く。だが、1.0 を出したら後方互換を保証すべきだ。API が安定していれば、コミュニティが安心して上位ライブラリを書ける。」

🦅 鷲津: 「API の安定性保証は信頼の基盤だ。SemVer を厳密に守る。1.x の間はパブリック API を壊さない。これを README と CHANGELOG で明示する。」

☕ ヤン: 「SDK の 1.0 リリース基準も決めておこう。」

```
SDK 1.0 リリース基準:
  1. 共通インターフェース仕様の全メソッドが実装されている
  2. 単体テストカバレッジ 80% 以上
  3. パッケージマネージャに公開されている (Maven Central / PyPI / npm 等)
  4. README に Quick Start / API Reference / Examples がある
  5. CI で全テストが通る
  6. 最低 2 週間の alpha/beta 期間
```

→ **Gap G-ECO-1 発見: コミュニティ貢献の仕組み整備。Contribution Guide、Provider テンプレート、awesome-syslenz リスト、SDK 1.0 リリース基準の策定が必要。**

→ **Gap G-ECO-2 発見: SDK の API 安定性保証ポリシー。SemVer 準拠と後方互換性の明文化。**

---

## Scene 5: ロードマップ — v1.3 から v3.0 へ

先輩: 具体的なバージョン別ロードマップを策定する。各バージョンのテーマと内容を決める。

💼 大和田: 「ロードマップを出せ。投資家に見せるやつじゃない。自分たちが "次に何をやるか" を迷わないための地図だ。各バージョンにテーマを付けろ。テーマがないバージョンは散漫になる。」

☕ ヤン: 「3バージョン先まで。それ以上は予測不能だから決めても無駄。」

🦅 鷲津: 「各バージョンの期間目安も設定する。デッドラインがないスケジュールは幻想だ。」

⚔️ リヴァイ: 「全バージョンの中身を整理する。」

### v1.3: "Data Layer" — データソースの拡張 (目標: 4-6 週間)

テーマ: syslenz が見える世界を広げる。DB 監視とデータ系エンジニアの取り込み。

```
SDK:
  [NEW] syslenz4py         — Python SDK (PyPI)
                             Watch API, Snapshot API, Jupyter integration
                             工数: L (3-4 週間)

Provider:
  [NEW] provider-mysql     — MySQL SHOW STATUS / performance_schema
                             工数: S (1-2 日)
  [NEW] provider-postgres  — PostgreSQL pg_stat_activity / pg_stat_database
                             工数: S (1-2 日)

UI/Export:
  [FIN] syslenz-otel       — OTEL bridge 完成 (feature flag → stable)
                             工数: M (1-2 週間)

Infra:
  [NEW] provider-template.sh  — コミュニティ向けテンプレート
                                工数: S (半日)
  [NEW] SDK Common Interface Spec (ドキュメント)
                                工数: S (1-2 日)
```

### v1.4: "Web Scale" — Web インフラの監視 (目標: 4-6 週間)

テーマ: Web アプリケーションスタック全体の可視化。フロントエンド開発者の取り込み。

```
SDK:
  [NEW] syslenz4node       — Node.js SDK (npm)
                             Watch API, Snapshot API, Express middleware
                             工数: L (3-4 週間)

Provider:
  [NEW] provider-redis     — Redis INFO コマンド
                             工数: S (1 日)
  [NEW] provider-nginx     — nginx stub_status
                             工数: S (1 日)

Infra:
  [NEW] Contribution Guide — Provider の submit 手順
                             工数: S (1 日)
  [NEW] awesome-syslenz    — コミュニティ Provider カタログ
                             工数: S (半日)
```

### v2.0: "Platform" — プラットフォーム化 (目標: 8-12 週間)

テーマ: syslenz をツールからプラットフォームへ。長期保存、カスタムダッシュボード、Grafana 連携。

```
UI/Export:
  [NEW] syslenz-web 分離   — feature flag → 独立パッケージ
                             教育 UI + カスタムダッシュボード
                             工数: L (3-4 週間)
  [EVL] syslenz-grafana    — Grafana data source plugin (OTEL で不十分な場合)
                             工数: L (3-4 週間) ※要否判断あり

Provider:
  [NEW] provider-mongodb   — MongoDB serverStatus / db.stats()
                             工数: M (2-3 日)
  [NEW] provider-apache    — Apache server-status (mod_status)
                             工数: S (1 日)

Core:
  [NEW] 長期メトリクス保存  — SQLite / ファイルベースの時系列データ
                             工数: XL (4-6 週間)
  [NEW] カスタムダッシュボード — ユーザー定義のメトリクスレイアウト
                                工数: L (3-4 週間)
```

### v3.0: "Enterprise" — エンタープライズ対応 (目標: 12-16 週間)

テーマ: 大規模環境、K8s、ML 異常検知。エンタープライズ顧客が求める機能群。

```
SDK:
  [NEW] syslenz4go         — Go SDK
                             工数: M (2-3 週間)
  [NEW] syslenz4cs         — .NET SDK (NuGet)
                             工数: L (3-4 週間)

Provider:
  [NEW] provider-k8s       — Kubernetes API (Pod/Node/Deployment metrics)
                             認証: kubeconfig / ServiceAccount
                             工数: XL (2-3 週間)
  [NEW] provider-elasticsearch — ES _cluster/stats, _nodes/stats
                                 工数: M (2-3 日)

Core:
  [NEW] ML 異常検知        — メトリクスのベースライン学習 + 異常アラート
                             工数: XL (6-8 週間)
```

🦁 ラインハルト: 「v3.0 の ML 異常検知は大きい。だがこれがあれば syslenz は "ただの監視ツール" から "予測する監視プラットフォーム" になる。既存ツールとの決定的な差別化だ。」

💼 大和田: 「v1.3 と v1.4 は現実的だ。v2.0 はスコープが大きすぎないか？長期保存とカスタムダッシュボードを同時にやるのは危ない。」

僕: 「...v2.0 は分割した方が...」

☕ ヤン: 「v2.0 を v2.0 と v2.1 に分けてもいい。だがバージョン番号の詳細は今決めなくていい。"v2.0 のテーマはプラットフォーム化" という方向性が決まればいい。スコープは着手時に再評価する。」

🦅 鷲津: 「各バージョンの KPI も定めておくべきだ。」

```
v1.3 KPI:
  - syslenz4py の PyPI ダウンロード数 (リリース後1ヶ月で 100+)
  - provider-mysql/postgres の利用レポート (GitHub Issues でのフィードバック)
  - OTEL bridge 経由での Prometheus メトリクス取得の動作確認

v1.4 KPI:
  - syslenz4node の npm weekly downloads (リリース後1ヶ月で 200+)
  - コミュニティ製 Provider の投稿数 (1つ以上)

v2.0 KPI:
  - Grafana 連携のデモが公開できる状態
  - 24 時間以上のメトリクス保存が動作する
```

→ **ロードマップ確定。v1.3 "Data Layer" → v1.4 "Web Scale" → v2.0 "Platform" → v3.0 "Enterprise"。**

---

## 最終合意

### 優先順位付きラインナップ表

| # | コンポーネント | 種別 | バージョン | 工数 | 状態 |
|---|---------------|------|-----------|------|------|
| 1 | syslenz4j | SDK | - | - | ✓ done |
| 2 | provider-jvm | Provider | - | - | ✓ done |
| 3 | provider-docker | Provider | - | - | ✓ done |
| 4 | syslenz4py | SDK | v1.3 | L | planned |
| 5 | provider-mysql | Provider | v1.3 | S | planned |
| 6 | provider-postgres | Provider | v1.3 | S | planned |
| 7 | syslenz-otel | Export | v1.3 | M | in progress |
| 8 | syslenz4node | SDK | v1.4 | L | planned |
| 9 | provider-redis | Provider | v1.4 | S | planned |
| 10 | provider-nginx | Provider | v1.4 | S | planned |
| 11 | syslenz-web | UI | v2.0 | L | feature-gated |
| 12 | syslenz-grafana | UI | v2.0 | L | evaluate |
| 13 | provider-mongodb | Provider | v2.0 | M | planned |
| 14 | provider-apache | Provider | v2.0 | S | planned |
| 15 | syslenz4go | SDK | v3.0 | M | planned |
| 16 | syslenz4cs | SDK | v3.0 | L | planned |
| 17 | provider-k8s | Provider | v3.0 | XL | planned |
| 18 | provider-elasticsearch | Provider | v3.0 | M | planned |
| 19 | syslenz4swift | SDK | v4.0+ | M | backlog |

### 工数見積もり凡例

| サイズ | 工数目安 | 例 |
|--------|---------|-----|
| S | 0.5-2 日 | bash Provider (mysql, redis, nginx) |
| M | 1-3 週間 | 複雑な Provider (mongodb), Go SDK, OTEL bridge |
| L | 3-4 週間 | SDK (Python, Node, C#), Web UI 分離, Grafana plugin |
| XL | 4-8 週間 | K8s provider, 長期保存, ML 異常検知 |

### やらないことリスト (意図的に作らないもの)

| やらないこと | 理由 |
|-------------|------|
| syslenz4ruby — Ruby SDK | Ruby のサーバーサイド利用は減少傾向。Go/Python の方が優先度が高い |
| syslenz4php — PHP SDK | PHP は監視系ツールとの親和性が低い。PHP 開発者は APM (New Relic, Datadog) を使う |
| syslenz4rust — Rust SDK | syslenz core 自体が Rust。Rust から syslenz を使うなら直接 crate として依存すればいい |
| provider-aws — AWS CloudWatch 連携 | AWS 独自。クラウド依存の Provider は優先度が低い。OTEL bridge 経由で対応可能 |
| provider-gcp — GCP Monitoring 連携 | 同上 |
| provider-azure — Azure Monitor 連携 | 同上 |
| Grafana plugin の自前パネル UI | OTEL → Prometheus 経由で十分。カスタムパネルのメンテコストが見合わない |
| モバイルアプリ (iOS/Android) | ニッチすぎる。Web UI のレスポンシブ対応で十分 |
| Windows ネイティブ GUI | TUI + Web UI でカバー。Windows 固有の GUI フレームワークへの投資は非効率 |
| 独自アラート通知 (Slack/PagerDuty) | OTEL → AlertManager / Grafana Alerting に委譲。通知インフラの再発明はしない |

### 次の 3 バージョンの具体的バックログ

#### v1.3 "Data Layer" バックログ

```
必須 (Must):
  [ ] syslenz4py — Python SDK (PyPI, Watch API, Snapshot API)
  [ ] provider-mysql — MySQL SHOW STATUS / performance_schema
  [ ] provider-postgres — PostgreSQL pg_stat_* メトリクス
  [ ] syslenz-otel — OTEL bridge の安定化 (feature flag → default)
  [ ] SDK Common Interface Spec — 全 SDK 共通 API 仕様書

推奨 (Should):
  [ ] provider-template.sh — コミュニティ向け Provider テンプレート
  [ ] syslenz4py Jupyter integration — notebook 内でのメトリクス表示

延期可 (Could):
  [ ] syslenz4py async support — asyncio 対応
```

#### v1.4 "Web Scale" バックログ

```
必須 (Must):
  [ ] syslenz4node — Node.js SDK (npm, Watch API, Snapshot API)
  [ ] provider-redis — Redis INFO メトリクス
  [ ] provider-nginx — nginx stub_status メトリクス

推奨 (Should):
  [ ] Contribution Guide — Provider 貢献ガイド (CONTRIBUTING.md)
  [ ] awesome-syslenz — コミュニティ Provider カタログ (GitHub repo)
  [ ] syslenz4node Express middleware — Express アプリへの組み込み

延期可 (Could):
  [ ] provider-apache — Apache server-status (v2.0 に前倒し可)
```

#### v2.0 "Platform" バックログ

```
必須 (Must):
  [ ] syslenz-web 分離 — 独立パッケージ化 (教育 UI + ダッシュボード)
  [ ] 長期メトリクス保存 — SQLite ベースの時系列データストア
  [ ] カスタムダッシュボード — ユーザー定義のメトリクスレイアウト

推奨 (Should):
  [ ] provider-mongodb — MongoDB serverStatus メトリクス
  [ ] provider-apache — Apache server-status メトリクス
  [ ] syslenz-grafana 評価 — OTEL bridge で不十分か判断

延期可 (Could):
  [ ] メトリクスのアノテーション機能 — デプロイ/インシデントのマーカー
  [ ] ダッシュボードの共有/エクスポート
```

---

## 発見された Gap 一覧

| Gap ID | 内容 | 優先度 | 対応バージョン |
|--------|------|--------|---------------|
| G-SDK-1 | SDK Common Interface Spec の策定 | 高 | v1.3 |
| G-PRV-1 | Provider テンプレートとリファレンスドキュメント | 高 | v1.3 |
| G-UI-1 | OTEL bridge の安定化と完成 | 高 | v1.3 |
| G-ECO-1 | コミュニティ貢献の仕組み整備 | 中 | v1.4 |
| G-ECO-2 | SDK API 安定性保証ポリシー (SemVer) | 中 | v1.3 (ドキュメントのみ) |
