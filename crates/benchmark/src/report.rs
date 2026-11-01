use crate::{Error, Protocol, Result, stats};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Json,
    Html,
}
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
pub fn data(protocol: &Protocol, anonymize: bool) -> Result<Value> {
    let mut protocol = protocol.clone();
    if anonymize {
        protocol.applied_uptime_ms = None;
        // Projection: remove ALL process names, boot identifiers, and capture names.
        // No path/username regex can reliably cover arbitrary user-chosen names.
        protocol.baseline_boot = protocol.baseline_boot.map(|_| "[boot]".into());
        for capture in &mut protocol.presentmon {
            capture.application = "[application]".into();
        }
        for run in protocol.before.iter_mut().chain(protocol.after.iter_mut()) {
            for sample in &mut run.samples {
                sample.top.clear();
                sample.boot = "[boot]".into();
            }
        }
    }
    let comparisons = stats::comparisons(&protocol);
    let mut value = serde_json::to_value(&protocol).map_err(|_| Error::Storage)?;
    if anonymize {
        for condition in ["before", "after"] {
            if let Some(runs) = value[condition].as_array_mut() {
                for run in runs {
                    if let Some(samples) = run["samples"].as_array_mut() {
                        for sample in samples {
                            if let Some(sample) = sample.as_object_mut() {
                                sample.remove("uptimeMs");
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(
        json!({"schemaVersion":1,"anonymized":anonymize,"protocol":value,"comparisons":comparisons,
        "method":{"runsPerCondition":5,"warmupDiscarded":1,"secondsPerRun":60,"experimentalUnit":"run median","quantiles":"linear interpolation","noiseRule":"overlapping p5/p95 OR absolute median difference <= max(p95-p5 of each condition, resolution floor)","resolutionFloors":{"available_memory_MiB":16,"cpu_percentage_points":0.5,"processes":1},"causalClaim":false,"fpsSource":"PresentMon CSV import, not live game access"}}),
    )
}
pub fn render(
    protocol: &Protocol,
    format: Format,
    anonymize: bool,
    english: bool,
) -> Result<String> {
    let data = data(protocol, anonymize)?;
    let json = serde_json::to_string_pretty(&data).map_err(|_| Error::Storage)?;
    if matches!(format, Format::Json) {
        return Ok(json);
    }
    let (lang, messages) = if english {
        ("en", include_str!("../locales/en.json"))
    } else {
        ("pt-BR", include_str!("../locales/pt-BR.json"))
    };
    let labels: std::collections::BTreeMap<String, String> =
        serde_json::from_str(messages).map_err(|_| Error::State)?;
    let label = |key: &str| labels.get(key).map(|text| escape(text)).ok_or(Error::State);
    let title = label("title")?;
    let note = label("note")?;
    let mut rows = String::new();
    for c in stats::comparisons(protocol) {
        let metric = label(&format!("metric_{}", c.metric))?;
        let conclusion = label(&c.conclusion)?;
        rows.push_str(&format!("<tr><th scope=\"row\">{metric}</th><td>{:.2} [{:.2}, {:.2}]</td><td>{:.2} [{:.2}, {:.2}]</td><td>{:.2}</td><td>{conclusion}</td></tr>",c.before.median,c.before.p5,c.before.p95,c.after.median,c.after.p5,c.after.p95,c.noise_band));
    }
    let mut headings = String::new();
    for key in [
        "columnMetric",
        "columnBefore",
        "columnAfter",
        "columnNoise",
        "columnConclusion",
    ] {
        headings.push_str(&format!("<th scope=\"col\">{}</th>", label(key)?));
    }
    Ok(format!(
        "<!doctype html><html lang=\"{lang}\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; base-uri 'none'; form-action 'none'; frame-src 'none'\"><title>{title}</title></head><body><main><h1>{title}</h1><p>{note}</p><table><caption>{title}</caption><thead><tr>{headings}</tr></thead><tbody>{rows}</tbody></table><details><summary>JSON</summary><pre>{}</pre></details></main></body></html>",
        escape(&json)
    ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn offline_html_escapes_untrusted_names_and_has_no_active_content() -> Result<()> {
        let mut p = Protocol::new();
        p.applied_ids = vec!["</pre><script>alert(1)</script>".into()];
        let html = render(&p, Format::Html, false, false)?;
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("default-src 'none'"));
        assert!(!html.contains("https://"));
        let english = render(&p, Format::Html, true, true)?;
        assert!(english.contains("Ametista measurement report"));
        Ok(())
    }
}
