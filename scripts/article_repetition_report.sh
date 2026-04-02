#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-resources/articles}"
TOPN="${2:-40}"

framework_exclude_regex='^(## (NAME|WHY NOW|EVIDENCE ORDER|SEE ALSO|Incident Forensics|Failure Archetype Matrix|Counterfactual Branches|Man-Page Crosswalk|Unix Internals Lens|Metric Snapshot|Action Loop)|## (なぜ今読むか|証拠の順序|障害フォレンジクス|失敗類型マトリクス|反事実分岐|manページ・クロスウォーク|Unix内部システム視点|メトリクス概要|アクションループ)|### (Evidence Capture|Decision Record|Drill Steps|Debrief Questions|エビデンス取得|証拠取得|判断記録|ドリル手順|振り返り設問)|[0-9]+\..*|- (Field in focus|注目field|Disproof attempt|反証試行):.*|Use related links in the article overlay to continue the same evidence chain\.|記事オーバーレイの関連記事リンクから、同じ証拠連鎖を辿ってください。)$'

normalize_stream() {
  awk 'NF>0' "$@" | sed 's/[[:space:]]\+$//'
}

print_stats() {
  local label="$1"
  local total="$2"
  local unique="$3"
  local dup ratio
  dup=$((total - unique))
  ratio=$(awk -v d="$dup" -v t="$total" 'BEGIN{ if (t==0) print "0.00"; else printf "%.2f", (d*100.0)/t }')
  echo "### ${label}"
  echo "- nonempty lines: ${total}"
  echo "- unique lines: ${unique}"
  echo "- duplicated lines: ${dup}"
  echo "- duplication ratio: ${ratio}%"
  echo
}

report_lang() {
  local lang="$1"
  local files
  files=$(rg --files "${ROOT}/${lang}")
  local total_raw unique_raw total_eff unique_eff

  echo "## ${lang}"
  total_raw=$(normalize_stream ${files} | wc -l)
  unique_raw=$(normalize_stream ${files} | sort -u | wc -l)
  print_stats "Raw (All Lines)" "$total_raw" "$unique_raw"

  total_eff=$(normalize_stream ${files} | rg -v "${framework_exclude_regex}" | wc -l)
  unique_eff=$(normalize_stream ${files} | rg -v "${framework_exclude_regex}" | sort -u | wc -l)
  print_stats "Framework-Excluded (Signal Lines)" "$total_eff" "$unique_eff"

  echo "#### Exclusion regex"
  echo "- \`${framework_exclude_regex}\`"
  echo

  echo "### Top ${TOPN} repeated lines (Raw)"
  normalize_stream ${files} \
    | sort | uniq -c | sort -nr \
    | awk -v n="$TOPN" 'NR<=n {print "- ["$1"] "substr($0, index($0,$2))}'
  echo

  echo "### Top ${TOPN} repeated lines (Framework-Excluded)"
  normalize_stream ${files} \
    | rg -v "${framework_exclude_regex}" \
    | sort | uniq -c | sort -nr \
    | awk -v n="$TOPN" 'NR<=n {print "- ["$1"] "substr($0, index($0,$2))}'
  echo
}

echo "# Article Repetition Report"
echo "- root: ${ROOT}"
echo "- generated_at: $(date -Iseconds)"
echo
report_lang en
report_lang ja
