# インフラのSLO思考

[English version](../en/concept.slo-thinking.md)

---

**SLO（サービスレベル目標）**は「速くあるべき」という曖昧な目標を測定可能な目標に変えます。

一般的なインフラSLO：P99リクエストレイテンシ < 100ms、可用性 > 99.9%、エラー率 < 0.1%

Linuxメトリクスへの変換：CPU > 80%でアラートする代わりに、何%のCPUがSLO違反を引き起こすかを問いかける。

**エラーバジェット思考：** 99.9%可用性 = 月あたり43.8分のダウンタイムエラーバジェット。`vmstat.oom_kill`イベントはエラーバジェットを消費します。

---

## 関連項目

- `sourceguide.vmstat` — vmstat概要
- `sourceguide.meminfo` — メモリ情報概要
- `sourceguide.pressure` — PSIプレッシャーストール情報
