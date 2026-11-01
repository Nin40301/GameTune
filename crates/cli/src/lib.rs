#![forbid(unsafe_code)]
pub mod args;
mod output;
use args::{Action, Options};
use gametune_core::*;
use gametune_windows_runtime::WindowsSession;
pub use output::*;
use serde::Serialize;
use serde_json::{Value, json};
use std::result::Result;

#[derive(Debug, Clone)]
pub struct Failure {
    pub code: u8,
    pub key: &'static str,
}
impl Failure {
    pub fn new(code: u8, key: &'static str) -> Self {
        Self { code, key }
    }
    pub fn usage() -> Self {
        Self::new(2, "arguments")
    }
}
impl From<gametune_core::Error> for Failure {
    fn from(error: gametune_core::Error) -> Self {
        match error {
            Error::ConfirmationRequired(_) => Self::new(4, "confirmation"),
            Error::Incompatible(_) => Self::new(4, "incompatible"),
            Error::UnknownItem(_)
            | Error::InvalidProfile(_)
            | Error::InvalidCatalog(_)
            | Error::Json(_) => Self::new(2, "profile"),
            Error::JournalLocked(_)
            | Error::CorruptJournal(_)
            | Error::JournalPoisoned
            | Error::RecoveryRequired => Self::new(6, "recovery"),
            Error::Io(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                Self::new(5, "administrator")
            }
            Error::Io(_) => Self::new(7, "io"),
            _ => Self::new(7, "preflight"),
        }
    }
}
impl From<ExecError> for Failure {
    fn from(error: ExecError) -> Self {
        match error {
            ExecError::AccessDenied => Self::new(5, "administrator"),
            ExecError::Protected | ExecError::ManualReinstallRequired => Self::new(4, "blocked"),
            ExecError::Conflict => Self::new(6, "recovery"),
            ExecError::NotFound => Self::new(7, "notFound"),
            ExecError::Unavailable => Self::new(7, "windows"),
            _ => Self::new(7, "preflight"),
        }
    }
}

/// Test seam only. The production binary constructs LiveBackend, never a mock mode.
pub trait Backend {
    fn catalog(&self) -> &Catalog;
    fn machine(&self) -> Machine;
    fn inspect(&mut self) -> Result<Vec<Observation>, Failure>;
    fn apply(&mut self, request: &Request) -> Result<Value, Failure>;
    fn undo_preview(&mut self) -> Result<JournalReadView, Failure>;
    fn undo(&mut self, ids: &[String]) -> Result<Report, Failure>;
    fn remove(&mut self, consent: &AppRemovalConsent) -> Result<Value, Failure>;
    fn reinstall(&mut self, id: &str, launch: bool) -> Result<Value, Failure>;
    fn reconcile(&mut self, id: &str) -> Result<Value, Failure>;
}
pub struct LiveBackend {
    session: WindowsSession,
}
impl LiveBackend {
    pub fn connect() -> Result<Self, Failure> {
        Ok(Self {
            session: WindowsSession::connect().map_err(Failure::from)?,
        })
    }
}
impl Backend for LiveBackend {
    fn catalog(&self) -> &Catalog {
        self.session.catalog()
    }
    fn machine(&self) -> Machine {
        self.session.context.machine()
    }
    fn inspect(&mut self) -> Result<Vec<Observation>, Failure> {
        Ok(self.session.inspect())
    }
    fn apply(&mut self, request: &Request) -> Result<Value, Failure> {
        self.session.context.ensure_elevated()?;
        serde_json::to_value(self.session.apply(request)?).map_err(|_| Failure::new(7, "io"))
    }
    fn undo_preview(&mut self) -> Result<JournalReadView, Failure> {
        Ok(self.session.history_readonly()?)
    }
    fn undo(&mut self, ids: &[String]) -> Result<Report, Failure> {
        self.session.context.ensure_elevated()?;
        Ok(self.session.undo(ids)?)
    }
    fn remove(&mut self, consent: &AppRemovalConsent) -> Result<Value, Failure> {
        self.session.context.ensure_elevated()?;
        self.session.remove_app(consent)??;
        Ok(json!({"itemId":consent.item_id,"removed":true,"undoRestores":false}))
    }
    fn reinstall(&mut self, id: &str, launch: bool) -> Result<Value, Failure> {
        let actions = self.session.reinstall_actions(id)?;
        if actions.is_empty() {
            return Err(Failure::new(7, "notFound"));
        }
        if launch {
            for action in &actions {
                self.session.context.open_reinstall(action)?;
            }
        }
        Ok(json!({"actions":actions,"storeOpened":launch,"installedAutomatically":false}))
    }
    fn reconcile(&mut self, id: &str) -> Result<Value, Failure> {
        self.session.context.ensure_elevated()?;
        self.session.reconcile_app_removal(id)?;
        Ok(json!({"itemId":id,"reconciled":true}))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisItem {
    pub item_id: String,
    pub name: String,
    pub selected: bool,
    pub compatible: bool,
    pub blocked: bool,
    pub risk: Risk,
    pub risk_note: String,
    pub verified: bool,
    pub revert: Revert,
    pub requires_reboot: bool,
    pub observation: Observation,
    pub message: String,
}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub schema_version: u8,
    pub action: String,
    pub dry_run: bool,
    pub language: String,
    pub exit_code: u8,
    pub message: String,
    pub items: Vec<AnalysisItem>,
    pub result: Value,
}
fn response(options: &Options, action: &str, key: &str, code: u8, locale: &Locale) -> Response {
    Response {
        schema_version: 1,
        action: action.into(),
        dry_run: !options.apply,
        language: options.language.clone(),
        exit_code: code,
        message: locale.text(key),
        items: vec![],
        result: Value::Null,
    }
}
pub fn execute(
    options: &Options,
    backend: &mut dyn Backend,
    locale: &Locale,
) -> Result<Response, Failure> {
    match &options.action {
        Action::Analyze => analyze(options, backend, locale),
        Action::Undo(id) => {
            if let Some(id) = id
                && backend.catalog().get(id)?.action_type == ActionType::Appx
            {
                return Err(Failure::new(4, "appxUndo"));
            }
            let mut reply = response(
                options,
                "undo",
                if options.apply {
                    "completed"
                } else {
                    "undoPreview"
                },
                0,
                locale,
            );
            if options.apply {
                let ids = id.iter().cloned().collect::<Vec<_>>();
                let report = backend.undo(&ids)?;
                reply.exit_code = report_code(&report);
                reply.result = json!(report);
            } else {
                let view = backend.undo_preview()?;
                reply.result = json!({"entries":view.entries.into_iter().filter(|e|id.as_ref().is_none_or(|id|id==&e.item_id)).collect::<Vec<_>>(),"needsRecovery":view.needs_recovery,"tornTail":view.torn_tail});
            }
            Ok(reply)
        }
        Action::Remove(id) | Action::Reconcile(id) => {
            let item = backend.catalog().get(id)?;
            if item.action_type != ActionType::Appx {
                return Err(Failure::usage());
            }
            if options.apply {
                if !backend.machine().supports(item) {
                    return Err(Failure::new(4, "incompatible"));
                }
                validate_acknowledgements(options, backend.catalog(), std::slice::from_ref(id))?;
                if !item.verified && !options.ack_unverified.contains(id) {
                    return Err(Failure::new(4, "unverified"));
                }
                if item.risk == Risk::High
                    && !options.gaming_risk
                    && matches!(options.action, Action::Remove(_))
                {
                    return Err(Failure::new(4, "gamingRisk"));
                }
                let result = if matches!(options.action, Action::Reconcile(_)) {
                    backend.reconcile(id)?
                } else {
                    backend.remove(&AppRemovalConsent {
                        item_id: id.clone(),
                        acknowledge_no_undo: options.no_undo,
                        acknowledge_gaming_dependencies: options.gaming_risk,
                        acknowledge_unverified: options.ack_unverified.contains(id),
                    })?
                };
                let mut reply = response(options, "appx", "completed", 0, locale);
                reply.result = result;
                Ok(reply)
            } else {
                let mut reply = build_analysis(options, backend, locale, std::slice::from_ref(id))?;
                reply.action = "appx_preview".into();
                reply.message = locale.text("appxWarning");
                Ok(reply)
            }
        }
        Action::Reinstall(id) => {
            let item = backend.catalog().get(id)?;
            if item.action_type != ActionType::Appx {
                return Err(Failure::usage());
            }
            let result = backend.reinstall(id, options.apply)?;
            let mut reply = response(
                options,
                "reinstall",
                if options.apply {
                    "storeOpened"
                } else {
                    "storePreview"
                },
                0,
                locale,
            );
            reply.result = result;
            Ok(reply)
        }
        _ => Err(Failure::usage()),
    }
}
fn profile_ids(profile: &str, catalog: &Catalog, machine: Machine) -> Result<Vec<String>, Failure> {
    let preset = match profile {
        "safe" | "seguro" => Some(Preset::Safe),
        "balanced" | "equilibrado" => Some(Preset::Balanced),
        "aggressive" | "agressivo" => Some(Preset::Aggressive),
        _ => None,
    };
    if let Some(preset) = preset {
        return Ok(preset.select(catalog, machine));
    }
    use std::io::Read;
    let mut json = String::new();
    std::fs::File::open(profile)
        .map_err(|_| Failure::new(2, "profile"))?
        .take(65537)
        .read_to_string(&mut json)
        .map_err(|_| Failure::new(2, "profile"))?;
    gametune_catalog::validate_profile_json(&json)?;
    Ok(Profile::import(&json, catalog)?.item_ids().to_vec())
}
fn validate_acknowledgements(
    options: &Options,
    catalog: &Catalog,
    ids: &[String],
) -> Result<(), Failure> {
    for id in options.ack_risk.iter().chain(&options.ack_unverified) {
        catalog.get(id)?;
        if !ids.contains(id) {
            return Err(Failure::usage());
        }
    }
    Ok(())
}
fn analyze(
    options: &Options,
    backend: &mut dyn Backend,
    locale: &Locale,
) -> Result<Response, Failure> {
    let ids = profile_ids(&options.profile, backend.catalog(), backend.machine())?;
    validate_acknowledgements(options, backend.catalog(), &ids)?;
    if options.apply {
        for id in &ids {
            let item = backend.catalog().get(id)?;
            if protection::is_protected(item) || item.revert == Revert::Manual {
                return Err(Failure::new(4, "blocked"));
            }
            if !backend.machine().supports(item) {
                return Err(Failure::new(4, "incompatible"));
            }
            if item.risk == Risk::High && !options.ack_risk.contains(id) {
                return Err(Failure::new(4, "risk"));
            }
            if !item.verified && !options.ack_unverified.contains(id) {
                return Err(Failure::new(4, "unverified"));
            }
        }
    }
    let mut reply = build_analysis(options, backend, locale, &ids)?;
    if options.apply {
        let selected: Vec<_> = reply
            .items
            .iter()
            .filter(|i| i.selected && i.observation.already_applied != Some(true))
            .map(|i| i.item_id.clone())
            .collect();
        if !selected.is_empty() {
            reply.result = backend.apply(&Request {
                item_ids: selected,
                high_risk_confirmations: options.ack_risk.clone(),
                unverified_confirmations: options.ack_unverified.clone(),
            })?;
            if reply.result["report"]["items"]
                .as_array()
                .is_some_and(|items| {
                    items
                        .iter()
                        .any(|i| i["outcome"] == "failed" || i["outcome"] == "skipped")
                })
            {
                reply.exit_code = 3;
            }
        }
        reply.action = "apply".into();
        reply.message = locale.text("completed");
    }
    Ok(reply)
}
fn build_analysis(
    options: &Options,
    backend: &mut dyn Backend,
    locale: &Locale,
    ids: &[String],
) -> Result<Response, Failure> {
    let observations = backend.inspect()?;
    let machine = backend.machine();
    let mut reply = response(options, "analyze", "analysis", 0, locale);
    for observation in observations {
        let item = backend.catalog().get(&observation.item_id)?;
        let key = if let Some(error) = &observation.error {
            exec_key(error)
        } else {
            match observation.presence {
                Presence::Exists => "exists",
                Presence::Missing => "missing",
                Presence::AlreadyApplied => "already",
                Presence::Unknown => "unknown",
                Presence::Discovery => "discovery",
            }
        };
        if observation.error.is_some() {
            reply.exit_code = 3;
        }
        let localized = |text: &Localized| {
            if options.language == "en" {
                text.en.clone()
            } else {
                text.pt_br.clone()
            }
        };
        reply.items.push(AnalysisItem {
            item_id: item.id.clone(),
            name: localized(&item.name),
            selected: ids.contains(&item.id),
            compatible: machine.supports(item),
            blocked: protection::is_protected(item),
            risk: item.risk,
            risk_note: localized(&item.risk_note),
            verified: item.verified,
            revert: item.revert,
            requires_reboot: item.requires_reboot,
            message: locale.text(key),
            observation,
        });
    }
    // Refuse incomplete backends instead of dropping a selected item silently.
    if backend
        .catalog()
        .items()
        .any(|i| !reply.items.iter().any(|r| r.item_id == i.id))
    {
        return Err(Failure::new(7, "incomplete"));
    }
    Ok(reply)
}
fn report_code(report: &Report) -> u8 {
    if report
        .items
        .iter()
        .any(|i| matches!(i.outcome, Outcome::Failed | Outcome::Skipped))
    {
        3
    } else {
        0
    }
}
pub fn exec_key(error: &ExecError) -> &'static str {
    match error {
        ExecError::AccessDenied => "readDenied",
        ExecError::NotFound => "missing",
        ExecError::Protected => "blocked",
        ExecError::Unavailable => "unavailable",
        ExecError::Conflict => "conflict",
        ExecError::Timeout => "timeout",
        _ => "unknown",
    }
}

pub fn static_response(options: &Options, locale: &Locale) -> Result<Option<Response>, Failure> {
    let mut reply = match options.action {
        Action::List => response(options, "list", "catalog", 0, locale),
        Action::Help => response(options, "help", "help", 0, locale),
        Action::Version => response(options, "version", "version", 0, locale),
        _ => return Ok(None),
    };
    reply.result = match options.action {
        Action::List => {
            json!({"items":gametune_catalog::load_embedded()?.items().collect::<Vec<_>>()})
        }
        Action::Help => json!({"usage":locale.text("help")}),
        _ => json!({"version":env!("CARGO_PKG_VERSION")}),
    };
    Ok(Some(reply))
}
