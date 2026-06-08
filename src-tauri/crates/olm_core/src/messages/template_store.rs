use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::domain::message::{
    ActionOption, ActionType, InboxMessage, MessageAction, MessageCategory, MessageContext,
    MessagePriority,
};

// ─── Template data structures ───

#[derive(Debug, Deserialize)]
pub struct MessageTemplateSender {
    pub name: String,
    #[serde(default)]
    pub name_key: Option<String>,
    pub role: String,
    #[serde(default)]
    pub role_key: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageTemplateAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub label: String,
    #[serde(default)]
    pub label_key: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub description_key: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub options: Vec<MessageTemplateOption>,
}

#[derive(Debug, Deserialize)]
pub struct MessageTemplateOption {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub label_key: Option<String>,
    #[serde(default)]
    pub description_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MessageTemplate {
    pub id: String,
    #[serde(default)]
    pub subject_key: Option<String>,
    pub subject: String,
    #[serde(default)]
    pub body_key: Option<String>,
    pub body: String,
    pub sender: MessageTemplateSender,
    pub category: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub actions: Vec<MessageTemplateAction>,
}

// ─── Template store ───

static TEMPLATE_STORE: OnceLock<TemplateStore> = OnceLock::new();

pub struct TemplateStore {
    /// templates grouped by function (subdirectory name)
    by_function: HashMap<String, Vec<MessageTemplate>>,
}

impl TemplateStore {
    /// Scan `data/messages/*/*.json` and load all templates.
    pub fn load(messages_dir: &Path) -> Result<Self, String> {
        let mut by_function: HashMap<String, Vec<MessageTemplate>> = HashMap::new();

        if !messages_dir.is_dir() {
            return Ok(Self { by_function });
        }

        let entries = fs::read_dir(messages_dir)
            .map_err(|e| format!("Failed to read messages dir {messages_dir:?}: {e}"))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Dir entry error: {e}"))?;
            let func_dir = entry.path();
            if !func_dir.is_dir() {
                continue;
            }
            let function_name = func_dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let mut templates: Vec<MessageTemplate> = Vec::new();
            let json_entries = fs::read_dir(&func_dir)
                .map_err(|e| format!("Failed to read {func_dir:?}: {e}"))?;

            for json_entry in json_entries {
                let json_entry = json_entry.map_err(|e| format!("Entry error: {e}"))?;
                let path = json_entry.path();
                if path.extension().and_then(|s| s.to_str()) != Some("json") {
                    continue;
                }
                let content = fs::read_to_string(&path)
                    .map_err(|e| format!("Failed to read {path:?}: {e}"))?;
                let template: MessageTemplate = serde_json::from_str(&content)
                    .map_err(|e| format!("Failed to parse {path:?}: {e}"))?;
                templates.push(template);
            }

            if !templates.is_empty() {
                by_function.insert(function_name, templates);
            }
        }

        Ok(Self { by_function })
    }

    /// Get a random template for a given function.
    pub fn get_random(&self, function: &str) -> Option<&MessageTemplate> {
        let templates = self.by_function.get(function)?;
        if templates.is_empty() {
            return None;
        }
        let idx = rand::random_range(0..templates.len());
        Some(&templates[idx])
    }

    /// Build an InboxMessage from a template with the given i18n params.
    pub fn build_message(
        &self,
        function: &str,
        id: &str,
        date: &str,
        params: Vec<(&str, &str)>,
    ) -> Option<InboxMessage> {
        let tpl = self.get_random(function)?;

        let i18n_params: std::collections::HashMap<String, String> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let category = match tpl.category.to_lowercase().as_str() {
            "welcome" => MessageCategory::Welcome,
            "leagueinfo" => MessageCategory::LeagueInfo,
            "matchpreview" => MessageCategory::MatchPreview,
            "matchresult" => MessageCategory::MatchResult,
            "transfer" => MessageCategory::Transfer,
            "boarddirective" => MessageCategory::BoardDirective,
            "playermorale" => MessageCategory::PlayerMorale,
            "injury" => MessageCategory::Injury,
            "training" => MessageCategory::Training,
            "finance" => MessageCategory::Finance,
            "contract" => MessageCategory::Contract,
            "scoutreport" => MessageCategory::ScoutReport,
            "media" => MessageCategory::Media,
            "system" => MessageCategory::System,
            "joboffer" => MessageCategory::JobOffer,
            _ => MessageCategory::System,
        };

        let priority = match tpl.priority.to_lowercase().as_str() {
            "low" => MessagePriority::Low,
            "normal" => MessagePriority::Normal,
            "high" => MessagePriority::High,
            "urgent" => MessagePriority::Urgent,
            _ => MessagePriority::Normal,
        };

        let mut msg = InboxMessage::new(
            id.to_string(),
            interpolate(&tpl.subject, &i18n_params),
            interpolate(&tpl.body, &i18n_params),
            interpolate(&tpl.sender.name, &i18n_params),
            date.to_string(),
        )
        .with_category(category)
        .with_priority(priority)
        .with_sender_role(&interpolate(&tpl.sender.role, &i18n_params));

        if let Some(icon) = &tpl.sender.icon {
            msg = msg.with_sender_icon(icon);
        }
        if let Some(key) = &tpl.subject_key {
            msg.subject_key = Some(key.clone());
        }
        if let Some(key) = &tpl.body_key {
            msg.body_key = Some(key.clone());
        }
        if let Some(key) = &tpl.sender.name_key {
            msg.sender_key = Some(key.clone());
        }
        if let Some(key) = &tpl.sender.role_key {
            msg.sender_role_key = Some(key.clone());
        }
        if !i18n_params.is_empty() {
            msg.i18n_params = i18n_params.clone();
        }

        for (i, a) in tpl.actions.iter().enumerate() {
            let action_type = match a.action_type.to_lowercase().as_str() {
                "navigate" | "navigateto" => ActionType::NavigateTo {
                    route: a.route.clone().unwrap_or_default(),
                },
                "choose" | "chooseoption" => ActionType::ChooseOption {
                    options: a
                        .options
                        .iter()
                        .map(|o| ActionOption {
                            id: o.id.clone(),
                            label: o.label.clone(),
                            description: o.description.clone(),
                            label_key: o.label_key.clone(),
                            description_key: o.description_key.clone(),
                        })
                        .collect(),
                },
                "dismiss" => ActionType::Dismiss,
                _ => ActionType::Acknowledge,
            };
            msg.actions.push(MessageAction {
                id: format!("action_{}", i),
                label: a.label.clone(),
                action_type,
                resolved: false,
                label_key: a.label_key.clone(),
            });
        }

        Some(msg)
    }
}

/// Initialize the global template store. Call once at app startup.
pub fn init_template_store(messages_dir: &Path) -> Result<(), String> {
    let store = TemplateStore::load(messages_dir)?;
    TEMPLATE_STORE
        .set(store)
        .map_err(|_| "TemplateStore already initialized".to_string())
}

/// Get a reference to the global template store.
pub fn template_store() -> &'static TemplateStore {
    TEMPLATE_STORE.get_or_init(|| {
        // If not initialized, return an empty store (graceful degradation)
        TemplateStore {
            by_function: HashMap::new(),
        }
    })
}

/// Replace {key} placeholders in template text with actual values.
fn interpolate(text: &str, params: &std::collections::HashMap<String, String>) -> String {
    let mut result = text.to_string();
    for (key, value) in params {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    result
}
