<!-- ⚠ DGE AUTO-GENERATED — review before acting -->
---
status: draft
source: DGE-020
---

# P-A1: Dashboard 診断サマリーバッジ

## 変更ファイル
- `src/ui/view_data.rs` — DashboardData に `diag_count: usize`, `diag_severity: Option<ViewColor>` 追加
- `src/ui/render.rs` — draw_dashboard() の loadavg セクション右端にバッジ描画

## 表示仕様
- EN: `"⚠ 3 issues (X:diagnostics)"` / `"✓ healthy"`
- JA: `"⚠ 3件の問題 (X:診断)"` / `"✓ 正常"`
- 色: Critical→赤, Warning→黄, Info→シアン, 0件→緑

## 実装
- `build_dashboard_data()` 内で `diagnostics::analyze()` を1回呼ぶ
- `diag_count` と `worst_severity` を DashboardData に格納
- render 側は DashboardData を読むだけ

## テスト
- `build_dashboard_data().diag_count` が `analyze().len()` と一致すること
