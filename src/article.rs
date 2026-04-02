use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::i18n::Locale;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ArticleKind {
    Metric,
    Group,
    Concept,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ArticleLink {
    Metric {
        label_en: &'static str,
        label_ja: &'static str,
        source: &'static str,
        field: &'static str,
    },
    Article {
        label_en: &'static str,
        label_ja: &'static str,
        id: &'static str,
    },
}

impl ArticleLink {
    pub fn label(&self, locale: Locale) -> &'static str {
        match (self, locale) {
            (ArticleLink::Metric { label_en, .. }, Locale::En) => label_en,
            (ArticleLink::Metric { label_ja, .. }, Locale::Ja) => label_ja,
            (ArticleLink::Article { label_en, .. }, Locale::En) => label_en,
            (ArticleLink::Article { label_ja, .. }, Locale::Ja) => label_ja,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EducationArticle {
    pub id: &'static str,
    pub kind: ArticleKind,
    pub title_en: &'static str,
    pub title_ja: &'static str,
    pub body_en: &'static str,
    pub body_ja: &'static str,
    pub links: &'static [ArticleLink],
}

impl EducationArticle {
    pub fn title(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => self.title_en,
            Locale::Ja => self.title_ja,
        }
    }

    pub fn body(&self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => self.body_en,
            Locale::Ja => self.body_ja,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ResourceIndex {
    articles: Vec<ResourceIndexArticle>,
}

#[derive(Debug, Deserialize)]
struct ResourceIndexArticle {
    id: String,
    kind: String,
    #[serde(default)]
    links: Vec<ResourceArticleLinkDef>,
}

#[derive(Debug, Deserialize)]
struct LocaleBundle {
    articles: std::collections::BTreeMap<String, LocaleArticleText>,
}

#[derive(Debug, Deserialize)]
struct LocaleArticleText {
    title: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct LegacyResourceCatalog {
    articles: Vec<LegacyResourceArticle>,
}

#[derive(Debug, Deserialize)]
struct LegacyResourceArticle {
    id: String,
    kind: String,
    title_en: String,
    title_ja: String,
    body_en: String,
    body_ja: String,
    #[serde(default)]
    links: Vec<ResourceArticleLinkDef>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ResourceArticleLinkDef {
    Metric {
        label_en: String,
        label_ja: String,
        source: String,
        field: String,
    },
    Article {
        label_en: String,
        label_ja: String,
        id: String,
    },
}

static RESOURCE_ARTICLES: OnceLock<Vec<EducationArticle>> = OnceLock::new();

fn leak_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn parse_kind(kind: &str) -> ArticleKind {
    match kind.to_ascii_lowercase().as_str() {
        "metric" => ArticleKind::Metric,
        "group" => ArticleKind::Group,
        "concept" => ArticleKind::Concept,
        _ => ArticleKind::Concept,
    }
}

fn article_markdown_path(base_dir: &Path, locale: &str, id: &str) -> PathBuf {
    base_dir.join(locale).join(format!("{id}.md"))
}

fn map_resource_links(links: Vec<ResourceArticleLinkDef>) -> &'static [ArticleLink] {
    let links_vec: Vec<ArticleLink> = links
        .into_iter()
        .map(|l| match l {
            ResourceArticleLinkDef::Metric {
                label_en,
                label_ja,
                source,
                field,
            } => ArticleLink::Metric {
                label_en: leak_str(label_en),
                label_ja: leak_str(label_ja),
                source: leak_str(source),
                field: leak_str(field),
            },
            ResourceArticleLinkDef::Article {
                label_en,
                label_ja,
                id,
            } => ArticleLink::Article {
                label_en: leak_str(label_en),
                label_ja: leak_str(label_ja),
                id: leak_str(id),
            },
        })
        .collect();
    Box::leak(links_vec.into_boxed_slice())
}

fn build_resource_articles_from_markdown_fs(base_dir: &Path) -> Option<Vec<EducationArticle>> {
    let index_path = base_dir.join("index.json");
    let raw_index = fs::read_to_string(&index_path).ok()?;
    let index: ResourceIndex = serde_json::from_str(&raw_index).ok()?;

    let mut result = Vec::new();
    for a in index.articles {
        let en_path = article_markdown_path(base_dir, "en", &a.id);
        let ja_path = article_markdown_path(base_dir, "ja", &a.id);
        let body_en = fs::read_to_string(&en_path).ok()?;
        let body_ja = fs::read_to_string(&ja_path).ok()?;

        result.push(EducationArticle {
            id: leak_str(a.id),
            kind: parse_kind(&a.kind),
            title_en: leak_str(
                body_en
                    .lines()
                    .next()
                    .unwrap_or("Untitled")
                    .trim_start_matches("# ")
                    .to_string(),
            ),
            title_ja: leak_str(
                body_ja
                    .lines()
                    .next()
                    .unwrap_or("無題")
                    .trim_start_matches("# ")
                    .to_string(),
            ),
            body_en: leak_str(body_en),
            body_ja: leak_str(body_ja),
            links: map_resource_links(a.links),
        });
    }
    Some(result)
}

fn build_resource_articles_from_split() -> Option<Vec<EducationArticle>> {
    let candidates = [
        Path::new("resources/articles"),
        Path::new("resources/articles-json-backup"),
    ];
    let mut loaded: Option<(String, String, String)> = None;
    for base in candidates {
        let index_path = base.join("index.json");
        let en_path = base.join("en.json");
        let ja_path = base.join("ja.json");
        if index_path.exists() && en_path.exists() && ja_path.exists() {
            let raw_index = fs::read_to_string(index_path).ok()?;
            let raw_en = fs::read_to_string(en_path).ok()?;
            let raw_ja = fs::read_to_string(ja_path).ok()?;
            loaded = Some((raw_index, raw_en, raw_ja));
            break;
        }
    }
    let (raw_index, raw_en, raw_ja) = loaded?;

    let index: ResourceIndex = serde_json::from_str(&raw_index).ok()?;
    let en_bundle: LocaleBundle = serde_json::from_str(&raw_en).ok()?;
    let ja_bundle: LocaleBundle = serde_json::from_str(&raw_ja).ok()?;

    let mut result = Vec::new();
    for a in index.articles {
        let en = match en_bundle.articles.get(&a.id) {
            Some(v) => v,
            None => continue,
        };
        let ja = match ja_bundle.articles.get(&a.id) {
            Some(v) => v,
            None => continue,
        };
        result.push(EducationArticle {
            id: leak_str(a.id),
            kind: parse_kind(&a.kind),
            title_en: leak_str(en.title.clone()),
            title_ja: leak_str(ja.title.clone()),
            body_en: leak_str(en.body.clone()),
            body_ja: leak_str(ja.body.clone()),
            links: map_resource_links(a.links),
        });
    }
    Some(result)
}

fn build_resource_articles_from_legacy() -> Option<Vec<EducationArticle>> {
    let candidates = [
        Path::new("resources/articles/catalog.json"),
        Path::new("resources/articles-json-backup/catalog.json"),
        Path::new("resources/articles-json-backup/catalog.json.latest"),
    ];
    let raw = candidates.iter().find_map(|p| fs::read_to_string(p).ok())?;
    let catalog: LegacyResourceCatalog = serde_json::from_str(&raw).ok()?;
    Some(
        catalog
            .articles
            .into_iter()
            .map(|a| EducationArticle {
                id: leak_str(a.id),
                kind: parse_kind(&a.kind),
                title_en: leak_str(a.title_en),
                title_ja: leak_str(a.title_ja),
                body_en: leak_str(a.body_en),
                body_ja: leak_str(a.body_ja),
                links: map_resource_links(a.links),
            })
            .collect(),
    )
}

fn build_resource_articles() -> Vec<EducationArticle> {
    if let Some(v) = build_resource_articles_from_markdown_fs(Path::new("resources/articles")) {
        return v;
    }
    if let Some(v) = build_resource_articles_from_split() {
        return v;
    }
    if let Some(v) = build_resource_articles_from_legacy() {
        return v;
    }
    eprintln!(
        "article resources not loaded: split(index/en/ja) and legacy(catalog) both unavailable"
    );
    Vec::new()
}

fn resource_articles() -> &'static [EducationArticle] {
    RESOURCE_ARTICLES
        .get_or_init(build_resource_articles)
        .as_slice()
}

const ARTICLE_FALLBACK_READING_METRICS: EducationArticle = EducationArticle {
    id: "concept.reading-metrics",
    kind: ArticleKind::Concept,
    title_en: "Reading Metrics in syslenz",
    title_ja: "syslenzでメトリクスを読む基本",
    body_en: "What is this?\nThis is the fallback article used when a metric-specific article is not available yet.\n\nWhy it matters\nA metric value is meaningful only with context: trend, related fields, and workload state.\n\nHow to read\n1. Start from the current value.\n2. Compare with related fields.\n3. Check trend in Diff/Graph.\n4. Confirm user impact in Diagnostics.\n\nCommon mistakes\n- Reading one field in isolation\n- Ignoring units\n- Ignoring time trend\n\nDiagnostic flow\n- Unexpected value -> Diff\n- Persistent trend -> Graph\n- User-visible impact -> Diagnostics\n",
    body_ja: "これは何か\nメトリクス専用記事が未作成のときに表示される基本記事です。\n\nなぜ重要か\nメトリクス値は、トレンドと関連項目があって初めて意味を持ちます。\n\nどう読むか\n1. 現在値を見る\n2. 関連フィールドと比較する\n3. Diff/Graph で時間変化を見る\n4. 診断で影響を確認する\n\nよくある誤解\n- 単一フィールドだけで判断する\n- 単位を無視する\n- 時間変化を見ない\n\n診断フロー\n- 異常値 -> Diff\n- 継続傾向 -> Graph\n- 体感影響あり -> Diagnostics\n",
    links: &[
        ArticleLink::Article {
            label_en: "Cross-metric reading",
            label_ja: "クロスメトリクスの読み方",
            id: "concept.cross-metric-reading",
        },
        ArticleLink::Article {
            label_en: "Pressure Stall Information",
            label_ja: "PSI プレッシャー",
            id: "concept.pressure-stall",
        },
        ArticleLink::Metric {
            label_en: "MemAvailable",
            label_ja: "MemAvailable",
            source: "meminfo",
            field: "MemAvailable",
        },
    ],
};

fn all_articles_iter() -> impl Iterator<Item = &'static EducationArticle> {
    resource_articles()
        .iter()
        .chain(std::iter::once(&ARTICLE_FALLBACK_READING_METRICS))
        .chain(crate::article_concepts::ARTICLES_CONCEPTS.iter())
        .chain(crate::article_metrics::ARTICLES_METRICS.iter())
        .chain(crate::article_metrics_generated::ARTICLES_METRICS_GENERATED.iter())
        .chain(crate::article_sources::ARTICLES_SOURCES.iter())
        .chain(crate::article_groups::ARTICLES_GROUPS.iter())
}

fn unique_articles() -> Vec<&'static EducationArticle> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for a in all_articles_iter() {
        if seen.insert(a.id) {
            out.push(a);
        }
    }
    out
}

pub fn fallback_article() -> &'static EducationArticle {
    &ARTICLE_FALLBACK_READING_METRICS
}

pub fn find_article_by_id(id: &str) -> Option<&'static EducationArticle> {
    all_articles_iter().find(|a| a.id == id)
}

#[derive(Serialize)]
struct ExportIndex {
    articles: Vec<ExportIndexArticle>,
}

#[derive(Serialize)]
struct ExportIndexArticle {
    id: String,
    kind: String,
    links: Vec<ExportLink>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ExportLink {
    Metric {
        label_en: String,
        label_ja: String,
        source: String,
        field: String,
    },
    Article {
        label_en: String,
        label_ja: String,
        id: String,
    },
}

#[derive(Serialize)]
struct ExportLocale {
    articles: BTreeMap<String, ExportLocaleArticle>,
}

#[derive(Serialize)]
struct ExportLocaleArticle {
    title: String,
    body: String,
}

pub fn export_split_resources_json() -> anyhow::Result<(String, String, String)> {
    let mut index_articles = Vec::new();
    let mut en_articles = BTreeMap::new();
    let mut ja_articles = BTreeMap::new();

    for a in unique_articles() {
        let links = a
            .links
            .iter()
            .map(|l| match l {
                ArticleLink::Metric {
                    label_en,
                    label_ja,
                    source,
                    field,
                } => ExportLink::Metric {
                    label_en: (*label_en).to_string(),
                    label_ja: (*label_ja).to_string(),
                    source: (*source).to_string(),
                    field: (*field).to_string(),
                },
                ArticleLink::Article {
                    label_en,
                    label_ja,
                    id,
                } => ExportLink::Article {
                    label_en: (*label_en).to_string(),
                    label_ja: (*label_ja).to_string(),
                    id: (*id).to_string(),
                },
            })
            .collect();

        let kind = match a.kind {
            ArticleKind::Metric => "metric",
            ArticleKind::Group => "group",
            ArticleKind::Concept => "concept",
        }
        .to_string();

        index_articles.push(ExportIndexArticle {
            id: a.id.to_string(),
            kind,
            links,
        });

        en_articles.insert(
            a.id.to_string(),
            ExportLocaleArticle {
                title: a.title_en.to_string(),
                body: a.body_en.to_string(),
            },
        );
        ja_articles.insert(
            a.id.to_string(),
            ExportLocaleArticle {
                title: a.title_ja.to_string(),
                body: a.body_ja.to_string(),
            },
        );
    }

    let index_json = serde_json::to_string_pretty(&ExportIndex {
        articles: index_articles,
    })?;
    let en_json = serde_json::to_string_pretty(&ExportLocale {
        articles: en_articles,
    })?;
    let ja_json = serde_json::to_string_pretty(&ExportLocale {
        articles: ja_articles,
    })?;
    Ok((index_json + "\n", en_json + "\n", ja_json + "\n"))
}

pub fn export_markdown_resources(dir: &Path) -> anyhow::Result<usize> {
    fs::create_dir_all(dir)?;

    let mut index_articles = Vec::new();
    let mut count = 0usize;
    for a in unique_articles() {
        let links = a
            .links
            .iter()
            .map(|l| match l {
                ArticleLink::Metric {
                    label_en,
                    label_ja,
                    source,
                    field,
                } => ExportLink::Metric {
                    label_en: (*label_en).to_string(),
                    label_ja: (*label_ja).to_string(),
                    source: (*source).to_string(),
                    field: (*field).to_string(),
                },
                ArticleLink::Article {
                    label_en,
                    label_ja,
                    id,
                } => ExportLink::Article {
                    label_en: (*label_en).to_string(),
                    label_ja: (*label_ja).to_string(),
                    id: (*id).to_string(),
                },
            })
            .collect();

        let kind = match a.kind {
            ArticleKind::Metric => "metric",
            ArticleKind::Group => "group",
            ArticleKind::Concept => "concept",
        }
        .to_string();
        index_articles.push(ExportIndexArticle {
            id: a.id.to_string(),
            kind,
            links,
        });

        let en_path = article_markdown_path(dir, "en", a.id);
        let ja_path = article_markdown_path(dir, "ja", a.id);
        if let Some(parent) = en_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Some(parent) = ja_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let en_body = format!("# {}\n\n{}", a.title_en, a.body_en);
        let ja_body = format!("# {}\n\n{}", a.title_ja, a.body_ja);
        fs::write(en_path, en_body)?;
        fs::write(ja_path, ja_body)?;
        count += 1;
    }

    let index_json = serde_json::to_string_pretty(&ExportIndex {
        articles: index_articles,
    })?;
    fs::write(dir.join("index.json"), index_json + "\n")?;
    Ok(count)
}

fn resolve_group_id(source: &str, field: &str) -> Option<String> {
    let snake_suffixes = ["_min", "_max", "_count"];
    for suffix in snake_suffixes {
        if let Some(stem) = field.strip_suffix(suffix) {
            return Some(format!("{source}.{stem}_distribution"));
        }
    }
    let camel_suffixes = ["Min", "Max", "Count"];
    for suffix in camel_suffixes {
        if let Some(stem) = field.strip_suffix(suffix) {
            return Some(format!("{source}.{stem}_distribution"));
        }
    }
    None
}

pub fn resolve_article_id(source: &str, field: &str) -> String {
    if let Some(group_id) = resolve_group_id(source, field) {
        if find_article_by_id(&group_id).is_some() {
            return group_id;
        }
    }

    let metric_id = format!("{source}.{field}");
    if find_article_by_id(&metric_id).is_some() {
        return metric_id;
    }

    let source_guide_id = format!("sourceguide.{source}");
    if find_article_by_id(&source_guide_id).is_some() {
        return source_guide_id;
    }

    fallback_article().id.to_string()
}

#[cfg(feature = "web")]
pub fn resolve_article(source: &str, field: &str) -> &'static EducationArticle {
    let id = resolve_article_id(source, field);
    find_article_by_id(&id).unwrap_or_else(fallback_article)
}

#[cfg(feature = "web")]
#[derive(Debug, Clone, Serialize)]
pub struct ApiArticle {
    pub found: bool,
    pub id: String,
    pub title: String,
    pub kind: String,
    pub body: String,
    pub links: Vec<ApiArticleLink>,
}

#[cfg(feature = "web")]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ApiArticleLink {
    Metric {
        label: String,
        source: String,
        field: String,
    },
    Article {
        label: String,
        id: String,
    },
}

#[cfg(feature = "web")]
pub fn to_api_article(article: &'static EducationArticle, locale: Locale) -> ApiArticle {
    let links = article
        .links
        .iter()
        .map(|l| match l {
            ArticleLink::Metric { source, field, .. } => ApiArticleLink::Metric {
                label: l.label(locale).to_string(),
                source: (*source).to_string(),
                field: (*field).to_string(),
            },
            ArticleLink::Article { id, .. } => ApiArticleLink::Article {
                label: l.label(locale).to_string(),
                id: (*id).to_string(),
            },
        })
        .collect();

    let kind = match article.kind {
        ArticleKind::Metric => "metric",
        ArticleKind::Group => "group",
        ArticleKind::Concept => "concept",
    };

    ApiArticle {
        found: true,
        id: article.id.to_string(),
        title: article.title(locale).to_string(),
        kind: kind.to_string(),
        body: article.body(locale).to_string(),
        links,
    }
}
