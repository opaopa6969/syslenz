<!-- ⚠ DGE AUTO-GENERATED — review before acting -->
# DGE Session 020: ペルソナ別 UI/UX — 個人開発者〜超エンタープライズ

- **Date**: 2026-03-31
- **Theme**: 複数ユーザーペルソナに対する UI/UX の Gap。基盤は v1.4.0 で整った。
- **Characters**: 今泉 + 利根川 + 千石 + ヤン + 大和田 + リヴァイ
- **Iterations**: 4 (Spec 飽和で終了)

## ペルソナ定義

| ペルソナ | 台数 | 求めるもの | 現在の Fit |
|---------|------|-----------|-----------|
| A: 個人開発者 | 1-3 | 一目でわかる、学びたい | 80% |
| B: スタートアップ SRE | 10-50 | マルチホスト、Slack 通知 | 30% |
| C: 中規模インフラ | 100-500 | 既存スタック統合、Fleet | 10% |
| D: 超エンタープライズ | 1000+ | RBAC、監査、SSO | 0% (対象外) |

## Design Decision DD-020-1
Enterprise (ペルソナ D) は syslenz の UI ではなく Prometheus/OTEL/SDK 経由で
既存 Enterprise ツールに統合する。syslenz に RBAC/SSO を入れない。

## Phase 1 (v1.5.0) — ペルソナ A 完成

| ID | Gap | Spec Status |
|----|-----|-------------|
| P-A1 | Dashboard 診断バッジ | ✅ Ready |
| P-A2 | Welcome 2段キーバインド | ✅ Ready |
| P-A3 | CLI --query モード | ✅ Ready |
| P-A4 | /api/field-help エンドポイント | ✅ Ready |
| P-A5 | History JSONL (書き込みのみ) | ✅ Ready |

## Phase 2 (v1.6.0) — ペルソナ B 対応

| ID | Gap | Spec Status |
|----|-----|-------------|
| G20-8 | Webhook 通知 (Slack/generic) | ✅ Ready |
| G20-7 | Web UI ファーストクラス化 | ✅ Ready |
| G20-9 | アラート履歴タイムライン | ✅ Ready |
| G20-10 | URL 状態共有 | ✅ Ready |
| G20-11 | Runbook URL 連携 | ✅ Ready |

## Phase 3 (v2.0.0) — ペルソナ C 対応

| ID | Gap |
|----|-----|
| G20-12 | Fleet View |
| G20-13 | Web API v1 バージョニング |
| G20-14 | Web UI 認証 |
| G20-15 | 設定 GUI |
