use std::collections::HashMap;
use std::sync::OnceLock;

use rand::{Rng, RngExt};
use serde::Deserialize;

use crate::domain::news::*;

// ─── Raw deserialization structures ───

#[derive(Debug, Deserialize)]
struct RawNewsTemplate {
    id: String,
    category: String,
    headlines: Vec<RawHeadline>,
    body: String,
    body_key: String,
    sources: Vec<RawSource>,
    #[serde(default)]
    translations: HashMap<String, RawCategoryTranslations>,
}

#[derive(Debug, Deserialize)]
struct RawHeadline {
    key: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct RawSource {
    key: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct RawCategoryTranslations {
    #[serde(default)]
    headlines: Vec<RawHeadline>,
    #[serde(default)]
    body: Option<String>,
}

// ─── Compiled template ───

pub struct NewsTemplate {
    pub category: NewsCategory,
    headlines: Vec<HeadlineVariant>,
    body_template: String,
    body_key: String,
    sources: Vec<SourceVariant>,
    translations: HashMap<String, CategoryTranslations>,
}

struct HeadlineVariant {
    key: String,
    text: String,
}

struct SourceVariant {
    key: String,
    text: String,
}

struct CategoryTranslations {
    headlines: Vec<HeadlineVariant>,
    body: Option<String>,
}

// ─── Global store ───

static NEWS_TEMPLATE_STORE: OnceLock<NewsTemplateStore> = OnceLock::new();

pub struct NewsTemplateStore {
    templates: HashMap<NewsCategory, NewsTemplate>,
}

impl NewsTemplateStore {
    fn load() -> Self {
        let mut templates = HashMap::new();

        // ── season_preview ──────────────────────────────────────
        let raw: RawNewsTemplate = serde_json::from_str(include_str!(
            "../../../../../data/news/season_preview/template.json"
        ))
        .expect("Failed to parse season_preview news template");
        let template = compile_template(raw);
        templates.insert(template.category.clone(), template);

        NewsTemplateStore { templates }
    }

    pub fn global() -> &'static NewsTemplateStore {
        NEWS_TEMPLATE_STORE.get_or_init(Self::load)
    }

    pub fn get(&self, category: &NewsCategory) -> Option<&NewsTemplate> {
        self.templates.get(category)
    }
}

fn compile_template(raw: RawNewsTemplate) -> NewsTemplate {
    let category = match raw.category.as_str() {
        "SeasonPreview" => NewsCategory::SeasonPreview,
        "LeagueRoundup" => NewsCategory::LeagueRoundup,
        "StandingsUpdate" => NewsCategory::StandingsUpdate,
        "TransferRumour" => NewsCategory::TransferRumour,
        "MatchReport" => NewsCategory::MatchReport,
        "Editorial" => NewsCategory::Editorial,
        "InjuryNews" => NewsCategory::InjuryNews,
        "ManagerialChange" => NewsCategory::ManagerialChange,
        other => panic!("Unknown news category in template: {other}"),
    };

    let translations = raw
        .translations
        .into_iter()
        .map(|(lang, t)| {
            let ct = CategoryTranslations {
                headlines: t
                    .headlines
                    .into_iter()
                    .map(|h| HeadlineVariant {
                        key: h.key,
                        text: h.text,
                    })
                    .collect(),
                body: t.body,
            };
            (lang, ct)
        })
        .collect();

    NewsTemplate {
        category,
        headlines: raw
            .headlines
            .into_iter()
            .map(|h| HeadlineVariant {
                key: h.key,
                text: h.text,
            })
            .collect(),
        body_template: raw.body,
        body_key: raw.body_key,
        sources: raw
            .sources
            .into_iter()
            .map(|s| SourceVariant {
                key: s.key,
                text: s.text,
            })
            .collect(),
        translations,
    }
}

// ─── Article builder ───

impl NewsTemplate {
    /// Build a `NewsArticle` from the template, picking a random
    /// headline/source and interpolating `{placeholder}` values.
    pub fn build_article(
        &self,
        id: String,
        date: String,
        params: &[(&str, &str)],
        lang: &str,
    ) -> NewsArticle {
        let mut rng = rand::rng();
        let lang_t = self.translations.get(lang);

        // Headline — try translation first
        let (headline_key, headline_text) =
            if let Some(lt) = lang_t.and_then(|t| select_random(&t.headlines, &mut rng)) {
                (lt.key.clone(), interpolate(&lt.text, params))
            } else {
                let h = select_random(&self.headlines, &mut rng)
                    .expect("Template must have at least one headline");
                (h.key.clone(), interpolate(&h.text, params))
            };

        // Body
        let body = if let Some(lt) = lang_t.and_then(|t| t.body.as_ref()) {
            interpolate(lt, params)
        } else {
            interpolate(&self.body_template, params)
        };

        // Source
        let src = select_random(&self.sources, &mut rng)
            .expect("Template must have at least one source");

        let i18n_params: HashMap<String, String> = params
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        NewsArticle::new(
            id,
            headline_text,
            body,
            src.text.clone(),
            date,
            self.category.clone(),
        )
        .with_i18n(&headline_key, &self.body_key, &src.key, i18n_params)
    }
}

// ─── Helpers ───

fn select_random<'a, T>(items: &'a [T], rng: &mut impl Rng) -> Option<&'a T> {
    if items.is_empty() {
        return None;
    }
    Some(&items[rng.random_range(0..items.len())])
}

fn interpolate(text: &str, params: &[(&str, &str)]) -> String {
    let mut result = text.to_string();
    for (key, value) in params {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}
