# DGE Session 019: Datadog を超えるプロダクトになるために

- **Date**: 2026-03-30
- **Theme**: syslenz が Datadog を「超える」とはどういう意味か。Datadog が絶対にやらない/できないことで圧倒的な価値を作る戦略を策定する
- **Parent Gaps**: DGE 015 (競合 Gap 分析), DGE 017 (教育ファーストクラス), DGE 018 (教育機能強化)
- **Characters**: 大和田 (ビジネスリアリスト) + 鷲津 (数字の鬼) + ラインハルト (ビジョナリー) + 利根川 (ユーザーの現実) + ハウス (診断の天才) + ヤン (怠惰な簡潔主義) + Red Team (競合視点) + 僕 (スコープ縮小)

---

## 核心: 5つの競争軸

Datadog と同じ土俵で戦わない。Datadog が構造的にやれない/やらない5軸で圧倒する。

### 軸1: 教育 (Education-first)
- Datadog: ドキュメントは外部サイト。ツール内では学べない。
- syslenz: ツール自体が教科書。使うだけで Linux が分かるようになる。
- 勝利条件: syslenz を1ヶ月使った初心者が、Datadog ユーザーよりトラブルシュートが速い。
- 構造的壕: SaaS は Enterprise 顧客から「邪魔」と言われる教育 UI を入れられない。

### 軸2: Time to Value (起動→価値の速さ)
- Datadog: サインアップ→Agent→API→ダッシュボード = 7分
- syslenz: curl | sh → syslenz = 21秒
- 勝利条件: 問題特定まで21秒。20倍の差。

### 軸3: Data Sovereignty (データ主権)
- Datadog: メトリクスがアメリカのクラウドに。GDPR/HIPAA 監査負荷。
- syslenz: データはローカル。外部送信ゼロ。エアギャップ対応。
- 勝利条件: 金融・医療・政府で「外部送信禁止」の環境に選ばれる。

### 軸4: Embeddability (組み込み可能性)
- Datadog: モニタリング "ツール"。API はあるが組み込み不可。
- syslenz: SDK 3言語。/health に組み込み、自動回復、CI 記録、IoT。
- 勝利条件: SDK がユーザーの monitoring stack のコアになる。

### 軸5: コスト
- Datadog: $23/host/月 (年間 $276/host)
- syslenz: $0 永久。MIT ライセンス。
- 勝利条件: 「Datadog やめて syslenz にした」事例。

---

## アクションロードマップ

### 即効性 (M1-M3)
- **M1**: ワンライナーインストーラー (install.syslenz.dev)
- **M2**: "21 Seconds to First Insight" campaign (GIF + README)
- **M3**: systemd service インストーラー (--install-service)

### 中期 (M4-M6)
- **M4**: --challenge モード (教育ゲーミフィケーション)
- **M5**: SDK Cookbook (5ユースケース実装例)
- **M6**: セキュリティホワイトペーパー (外部送信ゼロの証明)

### 長期 (M7-M8)
- **M7**: "Educational Monitoring" カテゴリの確立
- **M8**: 認定プログラム "syslenz Certified Linux Diagnostics"

---

## Gap 一覧

| # | Gap | カテゴリ | 優先度 | 軸 |
|---|-----|---------|--------|-----|
| G19-1 | 5つの競争軸の明文化と marketing 利用 | 戦略 | P0 | 全体 |
| G19-2 | アクティブ教育 (異常時自動解説、行動分析) | 教育 | P1 | 教育 |
| G19-3 | ゲーミフィケーション (スキルツリー、--challenge) | 教育 | P2 | 教育 |
| G19-4 | 教育は retention、acquisition は「30秒で動く」 | 戦略 | P0 | TtV |
| G19-5 | ワンライナーインストーラー (install.syslenz.dev) | 配布 | P0 | TtV |
| G19-6 | "21 Seconds to First Insight" campaign | マーケ | P0 | TtV |
| G19-7 | systemd service インストーラー (--install-service) | 機能 | P1 | TtV |
| G19-8 | セキュリティホワイトペーパー (外部送信ゼロの証明) | ドキュメント | P1 | 主権 |
| G19-9 | --air-gap フラグの明文化 | 機能 | P2 | 主権 |
| G19-10 | SDK Cookbook (5ユースケース実装例) | ドキュメント | P1 | 組込 |
| G19-11 | バイナリサイズ最適化 (IoT 対応) | 配布 | P2 | 組込 |
| G19-12 | "Educational Monitoring" カテゴリの確立 | 戦略 | P1 | 全体 |
| G19-13 | 認定プログラム構想 | 戦略 | P3 | 全体 |

---

## 結論

```
Datadog は「システムを監視する」。
syslenz は「システムを理解させる」。

超えるとは、同じ土俵で戦うことではない。
Datadog が絶対にやらない/できない5つの軸で、
圧倒的な価値を作ること。

この5つで勝てば、
「Datadog の代替」ではなく「Datadog にできないこと」として選ばれる。
```
