use crate::{Failure, Response};
use serde_json::Value;
use std::collections::BTreeMap;

pub struct Locale {
    messages: BTreeMap<String, String>,
}
impl Locale {
    pub fn load(language: &str) -> Result<Self, Failure> {
        let source = if language == "en" {
            include_str!("../../../locales/cli.en.json")
        } else {
            include_str!("../../../locales/cli.pt-BR.json")
        };
        Ok(Self {
            messages: serde_json::from_str(source).map_err(|_| Failure::new(7, "io"))?,
        })
    }
    pub fn text(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.into())
    }
}
pub fn error_json(error: &Failure, locale: &Locale, language: &str) -> Value {
    serde_json::json!({"schemaVersion":1,"language":language,"exitCode":error.code,"error":{"code":error.key,"message":locale.text(error.key)}})
}
pub fn text_report(report: &Response, locale: &Locale) -> String {
    let mut out = format!("{}\n", report.message);
    if report.action == "help" {
        return out;
    }
    for item in &report.items {
        out.push_str(&format!(
            "{} {} — {} — {}",
            if item.selected { "[x]" } else { "[ ]" },
            item.item_id,
            item.name,
            item.message
        ));
        if let Some(value) = item.observation.current_dword {
            out.push_str(&format!(" = {value}"));
        }
        out.push_str(&format!(
            " [{}]",
            locale.text(match item.observation.scope {
                ametista_core::ReadScope::CurrentUserOnly => "currentUserOnly",
                ametista_core::ReadScope::AllUsers => "allUsers",
                ametista_core::ReadScope::InteractiveUser => "interactiveUser",
                _ => "machine",
            })
        ));
        if item.blocked {
            out.push_str(&format!(" — {}", locale.text("blocked")));
        }
        out.push('\n');
        if let Some(current) = &item.observation.current {
            out.push_str(&format!(
                "  {}: {}\n",
                locale.text("current"),
                serde_json::to_string(current).unwrap_or_default()
            ));
        }
    }
    if report.action == "list" {
        if let Some(items) = report.result["items"].as_array() {
            for item in items {
                let lang = if report.language == "en" {
                    "en"
                } else {
                    "pt-BR"
                };
                out.push_str(&format!(
                    "{} — {}\n",
                    item["id"].as_str().unwrap_or_default(),
                    item["name"][lang].as_str().unwrap_or_default()
                ));
            }
        }
    } else if !report.result.is_null() {
        out.push_str(&format!(
            "{}\n",
            serde_json::to_string_pretty(&report.result).unwrap_or_default()
        ));
    }
    out
}
