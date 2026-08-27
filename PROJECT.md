# syslenz — Multi-Project Structure

## ディレクトリ構成

```
syslenz/
├── Cargo.toml              ← Rust 本体 (core)
├── src/                    ← Rust ソースコード
├── tests/                  ← Rust + Playwright テスト
│
├── sdk/                    ← アプリ組み込み用 SDK (4x = "for")
│   ├── java/               ← syslenz4j (Maven, Java 17+)
│   ├── dotnet/             ← syslenz4cs (NuGet, .NET 8+) [planned]
│   ├── python/             ← syslenz4py (PyPI) [planned]
│   └── node/               ← syslenz4node (npm) [planned]
│
├── providers/              ← 外部メトリクス取得プラグイン
│   ├── jvm/                ← JVM メトリクス (jstat/jcmd)
│   ├── docker-stats.sh     ← Docker コンテナメトリクス
│   ├── mysql/              ← MySQL メトリクス
│   ├── postgres/           ← PostgreSQL メトリクス
│   └── nginx/              ← nginx メトリクス
│
├── docs/                   ← ユーザードキュメント
│   ├── en/                 ← English (13 docs)
│   ├── ja/                 ← Japanese (13 docs)
│   ├── features/           ← 機能仕様 (completed/in-progress/planned)
│   └── assets/             ← GIF, スクリーンショット, 動画
│
├── design-materials/       ← 設計材料 (DGE セッション, specs)
│   ├── dge-sessions/       ← DGE 会話劇 (14 sessions)
│   └── specs/              ← usecase, spec, architecture, backlog
│
├── DGE/                    ← DGE メソッドツールキット
│
├── Dockerfile              ← Docker イメージ
├── docker-compose.yml      ← Docker Compose
└── run-web.sh              ← Web UI 起動スクリプト
```

## コンセプト

### Core (本体)
syslenz 本体。Rust で書かれた TUI/Web/X11 システム監視・教育ツール。

### SDK (4x = "for X")
アプリケーションの **内側** からメトリクスを送るライブラリ。
TCP サーバーとして動作し、`syslenz --connect` で接続する。

| SDK | 言語 | パッケージ | Status |
|-----|------|-----------|--------|
| syslenz4j | Java 17+ | Maven Central `org.unlaxer.infra:syslenz4j` | Published |
| syslenz4cs | .NET 8+ | NuGet | Planned |
| syslenz4py | Python 3.8+ | PyPI | Planned |
| syslenz4node | Node.js 18+ | npm | Planned |

### Providers
アプリケーションの **外側** からメトリクスを取得するプラグイン。
stdout に ProcEntry JSON を出力する実行ファイル。
`~/.config/syslenz/plugins/` に配置して使う。

| Provider | 対象 | 方式 |
|----------|------|------|
| jvm | JVM | jstat + jcmd |
| docker | Docker | docker stats |
| mysql | MySQL | SHOW GLOBAL STATUS |
| postgres | PostgreSQL | pg_stat_database / pg_stat_activity |
| nginx | nginx | stub_status |

## ビルド・テスト

```bash
# Rust (core)
cargo build
cargo test
cargo build --features web    # Web UI 付き

# Java SDK
cd sdk/java && mvn package

# Playwright テスト
node tests/web-ui.spec.mjs

# 全プラットフォームリリース
git tag v1.x.0 && git push origin --tags  # → GitHub Actions
```

## バージョニング

- Core と SDK は独立バージョン
- Core: Cargo.toml `version`
- Java SDK: pom.xml `${revision}`
- タグ: core は `v1.x.0`、SDK は `sdk-java-v1.x.0`
