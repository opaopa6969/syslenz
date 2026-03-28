# DGE Session 004: G4 + G11 深掘り — バイナリ配布と OSS インフラ整備

- **Date**: 2026-03-28
- **Theme**: 配布チャネル / CI・CD / ライセンス / リリースワークフロー
- **Parent Gaps**: G4 (バイナリ配布がない, Ops gap, High), G11 (OSS インフラ未整備, Ops gap, High)
- **Characters**: リヴァイ (実装執行者) + 大和田 (ビジネスリアリスト) + 僕 (スコープ削減) + ソウル (法務フィクサー)

---

## Scene 1: 配布チャネル — 誰がどうやって入れるのか

先輩 (ナレーション): 現在 syslenz のインストール方法は `cargo install --path .` のみ。Rust ツールチェインがない環境では使えない。Linux のシステム監視ツールなのに、監視対象のサーバーに Rust コンパイラを入れろというのは本末転倒。

⚔️ リヴァイ: 「動くバイナリがなければ意味がない。`cargo install` はユーザーに Rust のインストールを強制する。本番サーバーに `rustup` を入れる SRE がどこにいる。GitHub Releases にスタティックリンクしたバイナリを置け。それだけでいい。余計なことはするな。」

→ **Gap 発見: バイナリ配布が存在しない。GitHub Releases にビルド済みバイナリがない。Rust ツールチェインなしでは一切インストールできない。**

🦈 大和田: 「ターゲットは何だ。syslenz は Linux の /proc を読むツールだろう。macOS や Windows のバイナリを今作る必要があるのか？ リソースの無駄遣いだ。」

⚔️ リヴァイ: 「Linux だけでいい。だが 2 つのアーキテクチャは必要だ。`x86_64-unknown-linux-gnu` と `aarch64-unknown-linux-gnu`。AWS Graviton、Raspberry Pi、ARM サーバー。aarch64 を無視するな。あと musl。Alpine や distroless コンテナで動かすなら `x86_64-unknown-linux-musl` と `aarch64-unknown-linux-musl` も要る。」

→ **Gap 発見: クロスコンパイルターゲットが未定義。最低限 4 ターゲットが必要:**
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`

😰 僕: 「...あの、4 ターゲットは多くないですか... 最初は `x86_64-unknown-linux-gnu` だけじゃダメですか...？」

⚔️ リヴァイ: 「GitHub Actions の `cross` を使えば 4 ターゲット同時ビルドに追加コストはない。matrix strategy で回すだけだ。手抜きするな。」

🦈 大和田: 「Homebrew はどうする。`brew install syslenz` で入るなら macOS ユーザーも取れる。... いや待て、/proc は Linux だけだ。macOS に /proc はない。Homebrew の Linux tap (Linuxbrew) に絞るか、macOS 対応 (sysctl) ができるまで見送るか。」

→ **Gap 発見: Homebrew formula の戦略が未定義。macOS サポート未実装の現状では Linuxbrew only か見送りの判断が必要。**

⚔️ リヴァイ: 「Homebrew は後だ。まず GitHub Releases にバイナリを置く。`curl -L | tar xz` でインストールできる状態が最低ライン。Homebrew formula は macOS 対応 (platform_macos.rs の完成) 後でいい。」

→ **Spec 含意: v0.1.0 では GitHub Releases + curl インストールスクリプト。Homebrew は macOS 対応後に別 Session で扱う。**

😰 僕: 「...`cargo install syslenz` で crates.io からインストールできるのも... いいんじゃないですか... バイナリより簡単ですし...」

⚔️ リヴァイ: 「crates.io publish は CI でやれ。タグを打ったら自動で `cargo publish` する。手動は事故の元だ。ただし crates.io はビルド済みバイナリではない。`cargo install` を受け入れられるユーザー向けの追加チャネルだ。GitHub Releases の代替にはならない。」

→ **Spec 含意: crates.io publish も CI パイプラインに組み込む。ただし優先度は GitHub Releases の下。**

---

## Scene 2: CI/CD パイプライン — 何を自動化するのか

先輩: GitHub Actions のワークフローが 1 つもない。PR を出しても何もチェックされない。main にマージしても何も起きない。タグを打っても何も起きない。

⚔️ リヴァイ: 「CI がないプロジェクトは信用できない。最低限、PR ごとに以下を回せ:」

```
1. cargo fmt --check     — フォーマット
2. cargo clippy -- -D warnings  — lint
3. cargo build            — コンパイル確認
4. cargo test             — テスト実行
```

→ **Gap 発見: CI が存在しない。PR に対するビルド・テスト・lint チェックがゼロ。**

🦈 大和田: 「feature flag が 3 つあるだろう。`otel`, `web`, `x11widget`。default ビルドだけ通っても意味がない。全 feature combination でビルドが通るか確認しないと、気づかないうちに壊れる。」

⚔️ リヴァイ: 「全組み合わせは 8 通り (2^3)。それは多すぎる。`--all-features` と `--no-default-features` の 2 パターンで十分だ。個別 feature の破損は clippy が拾う。」

→ **Spec 含意: CI matrix は `default features` + `--all-features` + `--no-default-features` の 3 パターン。**

😰 僕: 「...テストが remote.rs に 1 つしかないんですが... CI 回しても意味あるんですか...？」

⚔️ リヴァイ: 「テストが少ないのは別の問題だ。CI はまず "コンパイルが通る" ことを保証する。テストは後から増やせるが、CI がなければ増やす動機も生まれない。先に箱を作れ。」

→ **Gap 発見: テスト不足は G11 の一部だが、CI 整備が先。CI があればテスト追加の Pull Request を出しやすくなる。**

🦈 大和田: 「CI のワークフローファイル、具体的にどう分ける？ 1 ファイルに全部入れるのか？」

⚔️ リヴァイ: 「2 ファイルに分ける。混ぜるな。」

```
.github/workflows/ci.yml       — PR + push to main: fmt, clippy, build, test
.github/workflows/release.yml  — tag push (v*): cross build → GitHub Release → crates.io publish
```

→ **Spec 含意: ワークフローは ci.yml と release.yml の 2 ファイル構成。**

⚔️ リヴァイ: 「ci.yml の具体的なジョブ構成はこうだ:」

```yaml
# ci.yml の骨格
name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        features: ["", "--all-features", "--no-default-features"]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check
      - run: cargo clippy ${{ matrix.features }} -- -D warnings
      - run: cargo build ${{ matrix.features }}
      - run: cargo test ${{ matrix.features }}
```

→ **Spec 含意: `dtolnay/rust-toolchain` + `Swatinem/rust-cache` で高速化。matrix で 3 feature パターンを並列実行。**

---

## Scene 3: ライセンスと法務 — README に MIT と書いてあるが...

先輩: README の末尾に「MIT」と書いてある。しかし LICENSE ファイルが存在しない。法的にはライセンスが付与されていない状態。

👻 ソウル: 「...面白いね。README に "MIT" と書くだけでは、法的にはライセンスは成立しない。MIT License には著作権者の名前と年が必要だ。`LICENSE` ファイルに正式なテキストを入れないと、厳密にはフォークもできない。」

→ **Gap 発見: LICENSE ファイルが存在しない。README の "MIT" 記載だけでは法的に不十分。**

⚔️ リヴァイ: 「LICENSE ファイルを作れ。中身はこうだ:」

```
MIT License

Copyright (c) 2026 syslenz contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
...（標準 MIT テキスト）
```

👻 ソウル: 「"syslenz contributors" でいいのか？ 個人名にするか、組織名にするか。将来コントリビューターが増えた時に揉める原因になる。」

🦈 大和田: 「個人プロジェクトなら作者名でいい。ただし Cargo.toml にも `license = "MIT"` フィールドを追加しろ。crates.io に publish するなら必須だ。今の Cargo.toml には license フィールドがない。」

→ **Gap 発見: Cargo.toml に `license` フィールドがない。crates.io publish 時にエラーになる。**

→ **Spec 含意:**
- `LICENSE` ファイルを作成 (MIT 全文、著作権者名入り)
- `Cargo.toml` に `license = "MIT"` を追加
- `Cargo.toml` に `description`, `repository`, `homepage`, `keywords`, `categories` も追加 (crates.io のメタデータ)

👻 ソウル: 「依存クレートのライセンス互換性も確認しておけ。`cargo-deny` を CI に入れれば、GPL 汚染を自動検出できる。」

⚔️ リヴァイ: 「x11rb が "MIT OR Apache-2.0"、ratatui が MIT。問題ない。だが `cargo-deny` は保険として CI に入れておく。将来の依存追加時にすぐ気づける。」

→ **Spec 含意: ci.yml に `cargo-deny check licenses` ステップを追加。`deny.toml` で許可ライセンスを定義。**

---

## Scene 4: リリースワークフロー MVP — タグからバイナリまで

先輩: リリースの自動化について議論する。現在はバージョンが Cargo.toml に `0.1.0` とあるだけで、タグもない、CHANGELOG もない、リリースノートもない。

🦈 大和田: 「release-please を使うか？ Conventional Commits を強制して、自動で CHANGELOG を生成し、バージョンを bump する。Google 製だ。」

⚔️ リヴァイ: 「release-please は過剰だ。コントリビューターが自分だけの段階で Conventional Commits を強制する意味がない。シンプルにやれ。」

```
1. CHANGELOG.md を手動で更新
2. Cargo.toml の version を手動で bump
3. git tag v0.1.0 && git push --tags
4. GitHub Actions release.yml がトリガーされ自動ビルド + リリース作成
```

😰 僕: 「...手動で CHANGELOG 書くの忘れそう...」

⚔️ リヴァイ: 「忘れるなら仕組みで防げ。release.yml の最初のステップで CHANGELOG.md に該当バージョンのエントリがあるか grep で確認しろ。なければ fail させる。」

→ **Spec 含意: release.yml にバージョンと CHANGELOG の整合性チェックを組み込む。**

⚔️ リヴァイ: 「release.yml の骨格はこうだ:」

```yaml
# release.yml の骨格
name: Release
on:
  push:
    tags: ["v*"]

permissions:
  contents: write

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-unknown-linux-musl
            os: ubuntu-latest
          - target: aarch64-unknown-linux-musl
            os: ubuntu-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Install cross
        run: cargo install cross --locked
      - name: Build
        run: cross build --release --target ${{ matrix.target }}
      - name: Package
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/syslenz dist/
          cd dist && tar czf syslenz-${{ matrix.target }}.tar.gz syslenz
      - uses: actions/upload-artifact@v4
        with:
          name: syslenz-${{ matrix.target }}
          path: dist/syslenz-${{ matrix.target }}.tar.gz

  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/download-artifact@v4
        with:
          path: artifacts
          merge-multiple: true
      - name: Verify CHANGELOG entry
        run: |
          VERSION=${GITHUB_REF_NAME#v}
          grep -q "## \[${VERSION}\]" CHANGELOG.md || \
            (echo "ERROR: No CHANGELOG entry for ${VERSION}" && exit 1)
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: artifacts/*.tar.gz
          generate_release_notes: true
      - uses: dtolnay/rust-toolchain@stable
      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

🦈 大和田: 「SHA256 チェックサムも付けろ。セキュリティ意識のあるユーザーは検証する。」

⚔️ リヴァイ: 「Package ステップに `sha256sum` を追加。`.sha256` ファイルもリリースアセットに含める。」

→ **Spec 含意: 各バイナリに SHA256 チェックサムファイルを添付。**

👻 ソウル: 「CHANGELOG のフォーマットは [Keep a Changelog](https://keepachangelog.com/) に従え。パースしやすい。将来 release-please に移行する時も互換性がある。」

→ **Spec 含意: CHANGELOG.md を Keep a Changelog 形式で作成。初回は `## [0.1.0] - Unreleased` から始める。**

🦈 大和田: 「README のバッジも忘れるな。CI が通ってるか、最新バージョンは何か、ライセンスは何か。バッジがないリポジトリは死んでるように見える。」

→ **Gap 発見: README にバッジがない。以下が必要:**
- CI status badge (`![CI](https://github.com/xxx/syslenz/actions/workflows/ci.yml/badge.svg)`)
- crates.io version (`![Crates.io](https://img.shields.io/crates/v/syslenz)`)
- License (`![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)`)

---

## Gap Summary (Session 004)

| # | Gap | Category | Severity | Observe -> Suggest -> Act |
|---|-----|----------|----------|--------------------------|
| G4-1 | ビルド済みバイナリが存在しない | Ops gap | High | Observe: `cargo install --path .` のみ -> Suggest: GitHub Releases にスタティックバイナリ -> Act: release.yml 作成 |
| G4-2 | クロスコンパイルターゲット未定義 | Ops gap | High | Observe: ターゲット指定なし -> Suggest: 4 ターゲット (x86_64/aarch64 x gnu/musl) -> Act: release.yml matrix 定義 |
| G4-3 | Homebrew formula 未整備 | Ops gap | Low | Observe: brew install 不可 -> Suggest: macOS 対応後に作成 -> Act: 現時点では見送り (platform_macos.rs 完成後) |
| G4-4 | crates.io 未公開 | Ops gap | Medium | Observe: `cargo install syslenz` 不可 -> Suggest: CI で自動 publish -> Act: release.yml に cargo publish 追加 |
| G4-5 | SHA256 チェックサムなし | Ops gap | Medium | Observe: バイナリ検証手段なし -> Suggest: sha256sum をリリースに添付 -> Act: release.yml の Package ステップ |
| G11-1 | LICENSE ファイルなし | Legal gap | Critical | Observe: README に MIT 記載あるがファイルなし -> Suggest: MIT 全文の LICENSE ファイル作成 -> Act: `LICENSE` を作成 |
| G11-2 | Cargo.toml に license フィールドなし | Legal gap | High | Observe: crates.io publish 時にエラー -> Suggest: `license = "MIT"` 追加 -> Act: Cargo.toml 編集 |
| G11-3 | Cargo.toml にメタデータ不足 | Ops gap | Medium | Observe: description, repository, keywords なし -> Suggest: crates.io 用メタデータ追加 -> Act: Cargo.toml 編集 |
| G11-4 | CI ワークフローなし | Ops gap | High | Observe: PR チェックがゼロ -> Suggest: ci.yml (fmt + clippy + build + test) -> Act: `.github/workflows/ci.yml` 作成 |
| G11-5 | CHANGELOG なし | Ops gap | Medium | Observe: 変更履歴が追跡不能 -> Suggest: Keep a Changelog 形式 -> Act: `CHANGELOG.md` 作成 |
| G11-6 | README にバッジなし | Message gap | Medium | Observe: プロジェクトの健全性が不可視 -> Suggest: CI, crates.io, license バッジ -> Act: README 先頭に追加 |
| G11-7 | 依存ライセンス監査なし | Legal gap | Low | Observe: GPL 汚染リスク未検証 -> Suggest: cargo-deny で自動チェック -> Act: ci.yml + deny.toml 作成 |

---

## Spec Proposals (具体的に作成するファイル)

### Spec S4-1: `LICENSE` ファイル作成

- **ファイル**: `/LICENSE`
- **内容**: MIT License 全文 (Copyright (c) 2026 + 著作権者名)
- **優先度**: Critical (法的要件)

### Spec S4-2: `Cargo.toml` メタデータ追加

- **ファイル**: `/Cargo.toml`
- **変更内容**:
```toml
[package]
name = "syslenz"
version = "0.1.0"
edition = "2024"
license = "MIT"
description = "Wireshark for /proc — structured, typed Linux system information viewer"
repository = "https://github.com/xxx/syslenz"
homepage = "https://github.com/xxx/syslenz"
keywords = ["linux", "proc", "tui", "system-monitor", "sysadmin"]
categories = ["command-line-utilities", "os::linux-apis"]
```
- **優先度**: High (crates.io publish の前提条件)

### Spec S4-3: `.github/workflows/ci.yml` 作成

- **ファイル**: `/.github/workflows/ci.yml`
- **トリガー**: push to main, pull_request to main
- **ジョブ**: fmt check, clippy, build, test (3 feature matrix)
- **追加**: `cargo-deny check licenses` ステップ
- **優先度**: High

### Spec S4-4: `.github/workflows/release.yml` 作成

- **ファイル**: `/.github/workflows/release.yml`
- **トリガー**: tag push `v*`
- **ジョブ**:
  1. `build`: cross build 4 ターゲット (matrix), tar.gz + sha256 生成
  2. `publish`: CHANGELOG 整合性チェック -> GitHub Release 作成 -> crates.io publish
- **依存**: `secrets.CARGO_REGISTRY_TOKEN` の設定
- **優先度**: High

### Spec S4-5: `CHANGELOG.md` 作成

- **ファイル**: `/CHANGELOG.md`
- **形式**: [Keep a Changelog](https://keepachangelog.com/ja/1.1.0/)
- **初回内容**:
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

## [0.1.0] - 2026-XX-XX

### Added
- 43 /proc sources with structured, typed parsing
- TUI with sidebar navigation, drill-in, search
- Snapshot diffing with red/green highlighting
- Time-series sparkline graphs
- JSON export/import
- SSH remote capture (experimental)
- OpenTelemetry export (feature: otel)
- Web UI (feature: web)
- X11 floating widget (feature: x11widget)
- Japanese/English i18n support
```
- **優先度**: Medium

### Spec S4-6: `deny.toml` 作成

- **ファイル**: `/deny.toml`
- **内容**: 許可ライセンスリスト (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib)
- **優先度**: Low

### Spec S4-7: README バッジ追加

- **ファイル**: `/README.md`
- **変更箇所**: 先頭にバッジ行を追加
```markdown
[![CI](https://github.com/xxx/syslenz/actions/workflows/ci.yml/badge.svg)](https://github.com/xxx/syslenz/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/syslenz)](https://crates.io/crates/syslenz)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
```
- **優先度**: Medium (CI 完成後)

---

## 実装順序 (推奨)

```
Step 1 (法務 + メタデータ):  S4-1 LICENSE → S4-2 Cargo.toml
Step 2 (CI 基盤):            S4-3 ci.yml → S4-6 deny.toml
Step 3 (リリース):           S4-5 CHANGELOG.md → S4-4 release.yml
Step 4 (仕上げ):             S4-7 README バッジ
```

## Next Actions

- [ ] S4-1 実装 -> LICENSE ファイル作成
- [ ] S4-2 実装 -> Cargo.toml メタデータ追加
- [ ] S4-3 実装 -> ci.yml 作成 + 動作確認
- [ ] S4-4 実装 -> release.yml 作成
- [ ] S4-5 実装 -> CHANGELOG.md 作成
- [ ] S4-6 実装 -> deny.toml 作成
- [ ] S4-7 実装 -> README バッジ追加
- [ ] GitHub Secrets 設定 -> `CARGO_REGISTRY_TOKEN`
- [ ] 初回リリース: `git tag v0.1.0 && git push --tags` で E2E 確認
- [ ] G4-3 (Homebrew) -> platform_macos.rs 完成後に別 Session
