use ametista_cli::{args::*, *};
use ametista_core::*;
use serde_json::{Value, json};
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn parse(args: &[&str]) -> std::result::Result<Options, Failure> {
    Options::parse(&args.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}
struct Fake {
    catalog: Catalog,
    writes: usize,
    reads: usize,
    opens: usize,
    denied: bool,
    incomplete: bool,
}
impl Fake {
    fn new() -> TestResultWith<Self> {
        Ok(Self {
            catalog: ametista_catalog::load_embedded()?,
            writes: 0,
            reads: 0,
            opens: 0,
            denied: false,
            incomplete: false,
        })
    }
}
type TestResultWith<T> = std::result::Result<T, Box<dyn std::error::Error>>;
impl Backend for Fake {
    fn catalog(&self) -> &Catalog {
        &self.catalog
    }
    fn machine(&self) -> Machine {
        Machine {
            build: 26200,
            edition: Edition::Pro,
            disk: DiskKind::Unknown,
        }
    }
    fn inspect(&mut self) -> std::result::Result<Vec<Observation>, Failure> {
        self.reads += 1;
        let mut rows = self
            .catalog
            .items()
            .map(|i| {
                observe(
                    i,
                    Err(if self.denied {
                        ExecError::AccessDenied
                    } else {
                        ExecError::NotFound
                    }),
                    ReadScope::Machine,
                )
            })
            .collect::<Vec<_>>();
        if self.incomplete {
            rows.pop();
        }
        Ok(rows)
    }
    fn apply(&mut self, request: &Request) -> std::result::Result<Value, Failure> {
        self.writes += 1;
        Ok(
            json!({"report":{"items":request.item_ids.iter().map(|id|json!({"itemId":id,"outcome":"applied"})).collect::<Vec<_>>()}}),
        )
    }
    fn undo_preview(&mut self) -> std::result::Result<JournalReadView, Failure> {
        self.reads += 1;
        Ok(JournalReadView {
            entries: vec![],
            needs_recovery: false,
            torn_tail: false,
        })
    }
    fn undo(&mut self, _: &[String]) -> std::result::Result<Report, Failure> {
        self.writes += 1;
        Ok(Report {
            dry_run: false,
            items: vec![],
        })
    }
    fn remove(&mut self, _: &AppRemovalConsent) -> std::result::Result<Value, Failure> {
        self.writes += 1;
        Ok(Value::Null)
    }
    fn reinstall(&mut self, _: &str, launch: bool) -> std::result::Result<Value, Failure> {
        self.reads += 1;
        if launch {
            self.opens += 1;
        }
        Ok(Value::Null)
    }
    fn reconcile(&mut self, _: &str) -> std::result::Result<Value, Failure> {
        self.writes += 1;
        Ok(Value::Null)
    }
}
fn options(args: &[&str]) -> TestResultWith<Options> {
    parse(args).map_err(|e| format!("{}:{}", e.code, e.key).into())
}
fn run(options: &Options, fake: &mut Fake) -> TestResultWith<Response> {
    execute(
        options,
        fake,
        &Locale::load(&options.language).map_err(|e| e.key)?,
    )
    .map_err(|e| format!("{}:{}", e.code, e.key).into())
}
#[test]
fn default_and_every_explicit_preview_never_mutate() -> TestResult {
    for args in [
        vec![],
        vec!["--dry-run"],
        vec!["--undo"],
        vec!["appx", "remove", "appx.microsoft.copilot"],
        vec!["appx", "reinstall", "appx.microsoft.copilot"],
        vec!["appx", "reconcile", "appx.microsoft.copilot"],
    ] {
        let mut fake = Fake::new()?;
        let response = run(&options(&args)?, &mut fake)?;
        assert!(response.dry_run);
        assert_eq!((fake.writes, fake.opens), (0, 0));
        assert!(fake.reads > 0);
    }
    Ok(())
}
#[test]
fn parser_requires_explicit_apply_and_confirmation() -> TestResult {
    for args in [
        vec!["--apply"],
        vec!["--confirm"],
        vec!["--apply", "--confirm", "--dry-run"],
        vec!["--list", "--undo"],
        vec!["--json", "--text"],
        vec!["--lang", "fr"],
        vec!["--command", "calc.exe"],
        vec![
            "appx",
            "remove",
            "appx.microsoft.copilot",
            "--apply",
            "--confirm",
        ],
    ] {
        assert!(parse(&args).is_err(), "{args:?}");
    }
    assert!(!options(&[])?.apply);
    assert!(options(&["--apply", "--confirm"])?.apply);
    Ok(())
}
#[test]
fn actual_apply_uses_only_selected_catalog_ids() -> TestResult {
    let mut fake = Fake::new()?;
    let expected = Preset::Safe.select(&fake.catalog, fake.machine());
    let response = run(
        &options(&["--profile", "safe", "--apply", "--confirm"])?,
        &mut fake,
    )?;
    assert_eq!(fake.writes, 1);
    assert!(!response.dry_run);
    let ids = response.result["report"]["items"]
        .as_array()
        .ok_or("items")?;
    assert_eq!(ids.len(), expected.len());
    assert!(
        ids.iter()
            .all(|i| expected.iter().any(|id| i["itemId"] == *id))
    );
    Ok(())
}
#[test]
fn unknown_app_and_untrusted_profile_never_mutate() -> TestResult {
    let mut fake = Fake::new()?;
    let opt = options(&[
        "appx",
        "remove",
        "unknown",
        "--apply",
        "--confirm",
        "--ack-no-undo",
    ])?;
    assert!(execute(&opt, &mut fake, &Locale::load("en").map_err(|e| e.key)?).is_err());
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("bad.json");
    std::fs::write(
        &path,
        r#"{"version":1,"name":"injected","itemIds":["unknown"],"command":"calc.exe"}"#,
    )?;
    let opt = options(&[
        "--profile",
        path.to_str().ok_or("path")?,
        "--apply",
        "--confirm",
    ])?;
    assert!(execute(&opt, &mut fake, &Locale::load("en").map_err(|e| e.key)?).is_err());
    assert_eq!(fake.writes, 0);
    Ok(())
}
#[test]
fn high_risk_app_requires_reinforced_consent() -> TestResult {
    let mut fake = Fake::new()?;
    let id = fake
        .catalog
        .items()
        .find(|i| i.action_type == ActionType::Appx && i.risk == Risk::High)
        .ok_or("risk app")?
        .id
        .clone();
    let opt = options(&[
        "appx",
        "remove",
        &id,
        "--apply",
        "--confirm",
        "--ack-no-undo",
        "--ack-unverified",
        &id,
    ])?;
    assert_eq!(
        execute(&opt, &mut fake, &Locale::load("en").map_err(|e| e.key)?)
            .err()
            .ok_or("expected refusal")?
            .code,
        4
    );
    assert_eq!(fake.writes, 0);
    let opt = options(&[
        "appx",
        "remove",
        &id,
        "--apply",
        "--confirm",
        "--ack-no-undo",
        "--ack-unverified",
        &id,
        "--ack-gaming-risk",
    ])?;
    run(&opt, &mut fake)?;
    assert_eq!(fake.writes, 1);
    Ok(())
}
#[test]
fn every_row_and_partial_read_failure_are_reported() -> TestResult {
    let mut fake = Fake::new()?;
    fake.denied = true;
    let reply = run(&options(&["--json"])?, &mut fake)?;
    assert_eq!(reply.items.len(), 86);
    assert_eq!(reply.exit_code, 3);
    assert!(reply.items.iter().all(|i| i.observation.exists.is_none()));
    fake.incomplete = true;
    assert!(
        execute(
            &options(&[])?,
            &mut fake,
            &Locale::load("en").map_err(|e| e.key)?
        )
        .is_err()
    );
    assert_eq!(fake.writes, 0);
    Ok(())
}
#[test]
fn translations_have_same_keys_and_readable_output_is_localized() -> TestResult {
    let pt: std::collections::BTreeMap<String, String> =
        serde_json::from_str(include_str!("../../../locales/cli.pt-BR.json"))?;
    let en: std::collections::BTreeMap<String, String> =
        serde_json::from_str(include_str!("../../../locales/cli.en.json"))?;
    assert_eq!(pt.keys().collect::<Vec<_>>(), en.keys().collect::<Vec<_>>());
    for language in ["pt-BR", "en"] {
        let mut fake = Fake::new()?;
        let reply = run(&options(&["--lang", language])?, &mut fake)?;
        let locale = Locale::load(language).map_err(|e| e.key)?;
        assert!(text_report(&reply, &locale).contains(&locale.text("missing")));
        assert_eq!(reply.language, language);
    }
    Ok(())
}
#[test]
fn binary_json_is_single_document_with_documented_exit_codes() -> TestResult {
    for (args, expected) in [
        (vec!["--list", "--json"], 0),
        (vec!["--apply", "--json"], 4),
        (vec!["--bad", "--json", "--lang", "en"], 2),
        (vec!["--help", "--json"], 0),
    ] {
        let out = std::process::Command::new(env!("CARGO_BIN_EXE_ametista-cli"))
            .args(args)
            .output()?;
        assert_eq!(out.status.code(), Some(expected));
        let result: Value = serde_json::from_slice(&out.stdout)?;
        assert_eq!(result["exitCode"], expected);
        assert!(out.stderr.is_empty());
    }
    Ok(())
}

#[test]
fn appx_undo_and_unknown_acknowledgements_are_rejected() -> TestResult {
    let mut fake = Fake::new()?;
    for args in [
        vec!["--undo", "appx.microsoft.copilot", "--apply", "--confirm"],
        vec!["--apply", "--confirm", "--ack-risk", "unknown"],
    ] {
        assert!(
            execute(
                &options(&args)?,
                &mut fake,
                &Locale::load("pt-BR").map_err(|e| e.key)?
            )
            .is_err()
        );
    }
    assert_eq!(fake.writes, 0);
    Ok(())
}
