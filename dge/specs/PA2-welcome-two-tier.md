<!-- ⚠ DGE AUTO-GENERATED — review before acting -->
---
status: draft
source: DGE-020
---

# P-A2: Welcome 2段キーバインド

## 変更ファイル
- `src/ui/view_data.rs` — WelcomeData に `advanced_keybindings` 追加、`build_welcome_data()` を分割
- `src/ui/render.rs` — `draw_welcome()` で help_level に応じて表示切り替え

## 表示仕様
- help_level Off/Normal → 基本6キーのみ:
  `D Dashboard / O Classic / j/k Navigate / Enter Drill / BS Back / q Quit`
- help_level Detailed/Extra → 基本 + 上級:
  `d Diff / g Graph / / Search / X Diagnostics / C Category / ? Help / L Lang / e Export / c Copy`

## テスト
- help_level Off → basic_keybindings のみ返す
- help_level Extra → basic + advanced 両方返す
