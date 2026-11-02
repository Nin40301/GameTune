use crate::{
    WindowsContext, registry, service, startup,
    transport::{self, Script},
};
use ametista_core::*;
use serde_json::json;
use std::{cell::RefCell, collections::BTreeMap};

pub struct WindowsExecutor {
    kind: ActionType,
    context: WindowsContext,
    captured: RefCell<BTreeMap<String, Snapshot>>,
}
impl WindowsExecutor {
    pub fn new(kind: ActionType, context: WindowsContext) -> Self {
        Self {
            kind,
            context,
            captured: RefCell::new(BTreeMap::new()),
        }
    }
    fn authorize(&self, item: &Item) -> ExecResult<()> {
        if item.action_type != self.kind {
            return Err(ExecError::TypeMismatch);
        }
        if protection::is_protected(item) {
            return Err(ExecError::Protected);
        }
        self.authorize_read(item)
    }
    pub(crate) fn authorize_read(&self, item: &Item) -> ExecResult<()> {
        if item.action_type != self.kind {
            return Err(ExecError::TypeMismatch);
        }
        #[cfg(feature = "vm-tests")]
        if protection::valid_test_target(item) {
            return Ok(());
        }
        protection::validate(item).map_err(|_| ExecError::Protected)?;
        if matches!(item.target, Target::Startup { entry: Some(_), .. }) {
            startup::bound(&self.context, item)?;
            return Ok(());
        }
        let catalog = ametista_catalog::load_embedded().map_err(crate::failed)?;
        let approved = catalog.get(&item.id).map_err(|_| ExecError::Protected)?;
        if item.target != approved.target || item.action_type != approved.action_type {
            return Err(ExecError::Protected);
        }
        Ok(())
    }
    pub(crate) fn read(&self, item: &Item) -> ExecResult<Snapshot> {
        self.read_controlled(item, None)
    }
    pub(crate) fn read_controlled(
        &self,
        item: &Item,
        cancel: Option<&std::sync::atomic::AtomicBool>,
    ) -> ExecResult<Snapshot> {
        match &item.target {
            Target::Registry {
                hive, path, name, ..
            } => {
                let (value, created_keys) = registry::capture(&self.context, *hive, path, name)?;
                Ok(Snapshot::Registry {
                    value,
                    created_keys,
                })
            }
            Target::Startup { .. } => {
                let (hive, name) = startup::bound(&self.context, item)?;
                let (approved, created_keys) =
                    registry::capture(&self.context, hive, registry::APPROVED, &name)?;
                Ok(Snapshot::Startup {
                    approved,
                    created_keys,
                })
            }
            Target::Service { name, .. } => {
                service::run(&json!({"operation":"capture","name":name}))
            }
            Target::ScheduledTask { path, name } => transport::run_controlled(
                Script::Task,
                &json!({"operation":"capture","path":path,"name":name}),
                cancel,
            ),
            Target::Power { .. } => {
                transport::run_controlled(Script::Power, &json!({"operation":"capture"}), cancel)
            }
            Target::Appx { name } => transport::run_controlled(
                Script::Appx,
                &json!({"operation":"capture","name":name}),
                cancel,
            ),
        }
    }
    fn desired(item: &Item, original: &Snapshot) -> ExecResult<Snapshot> {
        Ok(match (&item.target, original) {
            (Target::Registry { value, .. }, Snapshot::Registry { created_keys, .. }) => {
                Snapshot::Registry {
                    value: Some(RegistryValue {
                        kind: 4,
                        bytes: value.to_le_bytes().to_vec(),
                    }),
                    created_keys: created_keys.clone(),
                }
            }
            (Target::Startup { .. }, Snapshot::Startup { created_keys, .. }) => Snapshot::Startup {
                approved: Some(RegistryValue {
                    kind: 3,
                    bytes: vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                }),
                created_keys: created_keys.clone(),
            },
            (
                Target::Service { mode, .. },
                Snapshot::Service {
                    delayed_auto_start, ..
                },
            ) => Snapshot::Service {
                start_mode: if *mode == ServiceMode::Manual { 3 } else { 4 },
                delayed_auto_start: *delayed_auto_start,
                running: false,
            },
            (Target::ScheduledTask { .. }, Snapshot::ScheduledTask { definition, .. }) => {
                Snapshot::ScheduledTask {
                    enabled: false,
                    definition: definition.clone(),
                }
            }
            (Target::Power { guid }, Snapshot::Power { existing_guids, .. }) => {
                let mut guids = existing_guids.clone();
                if !guids.contains(guid) {
                    guids.push(guid.clone());
                    guids.sort();
                }
                Snapshot::Power {
                    active_guid: guid.clone(),
                    existing_guids: guids,
                }
            }
            _ => return Err(ExecError::TypeMismatch),
        })
    }
    fn changed_value_equal(a: &Snapshot, b: &Snapshot) -> bool {
        match (a, b) {
            (
                Snapshot::ScheduledTask { enabled: a, .. },
                Snapshot::ScheduledTask { enabled: b, .. },
            ) => a == b,
            (Snapshot::Registry { value: a, .. }, Snapshot::Registry { value: b, .. }) => a == b,
            (Snapshot::Startup { approved: a, .. }, Snapshot::Startup { approved: b, .. }) => {
                a == b
            }
            _ => a == b,
        }
    }
    fn write_action(&self, item: &Item, original: &Snapshot, restore: bool) -> ExecResult<()> {
        let operation = if restore { "restore" } else { "apply" };
        match (&item.target, original) {
            (
                Target::Registry {
                    hive,
                    path,
                    name,
                    value,
                },
                Snapshot::Registry {
                    value: prior,
                    created_keys,
                },
            ) => {
                if restore {
                    registry::restore(&self.context, *hive, path, name, prior, created_keys)
                } else {
                    registry::write(
                        &self.context,
                        *hive,
                        path,
                        name,
                        &RegistryValue {
                            kind: 4,
                            bytes: value.to_le_bytes().to_vec(),
                        },
                    )
                }
            }
            (
                Target::Startup { .. },
                Snapshot::Startup {
                    approved,
                    created_keys,
                },
            ) => {
                let (hive, name) = startup::bound(&self.context, item)?;
                if restore {
                    registry::restore(
                        &self.context,
                        hive,
                        registry::APPROVED,
                        &name,
                        approved,
                        created_keys,
                    )
                } else {
                    registry::write(
                        &self.context,
                        hive,
                        registry::APPROVED,
                        &name,
                        &RegistryValue {
                            kind: 3,
                            bytes: vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        },
                    )
                }
            }
            (Target::Service { name, mode }, _) => {
                let expected = if restore { None } else { Some(original) };
                service::write(
                    &json!({"operation":operation,"name":name,"mode":mode,"original":original,"expected":expected}),
                )
            }
            (Target::ScheduledTask { path, name }, _) => Self::script_write(
                Script::Task,
                &json!({"operation":operation,"path":path,"name":name,"original":original,"expected":if restore{None}else{Some(original)}}),
            ),
            (Target::Power { guid }, _) => {
                let mut args = json!({"operation":operation,"guid":guid,"original":original,"expected":if restore{None}else{Some(original)}});
                #[cfg(feature = "vm-tests")]
                if protection::valid_test_target(item)
                    && let Snapshot::Power { active_guid, .. } = original
                {
                    args["testSource"] = json!(active_guid);
                }
                // Mutable only under the test feature; production cannot choose another source.
                let _ = &mut args;
                Self::script_write(Script::Power, &args)
            }
            (Target::Appx { name }, _) if !restore => Self::script_write(
                Script::Appx,
                &json!({"operation":"apply","name":name,"expected":original}),
            ),
            _ => Err(ExecError::ManualReinstallRequired),
        }
    }
    fn script_write(script: Script, args: &serde_json::Value) -> ExecResult<()> {
        let success: bool = transport::run(script, args)?;
        if success {
            Ok(())
        } else {
            Err(ExecError::Failed(
                "Windows operation was not confirmed".into(),
            ))
        }
    }
}
impl Executor for WindowsExecutor {
    fn apply_temporary(&mut self, item: &Item) -> ExecResult<()> {
        if self.kind != ActionType::Service {
            return self.apply(item);
        }
        self.authorize(item)?;
        let catalog = ametista_catalog::load_embedded().map_err(crate::failed)?;
        if !ametista_catalog::game_actions(&catalog)
            .map_err(crate::failed)?
            .iter()
            .any(|a| a.id == item.id && a.pause_service)
        {
            return Err(ExecError::Protected);
        }
        self.ensure_write_ready()?;
        let Target::Service { name, .. } = &item.target else {
            return Err(ExecError::TypeMismatch);
        };
        let original = self
            .captured
            .borrow()
            .get(&item.target.key())
            .cloned()
            .ok_or(ExecError::Conflict)?;
        service::write(&json!({"operation":"pause","name":name,"expected":original}))?;
        let mut desired = original;
        if let Snapshot::Service { running, .. } = &mut desired {
            *running = false;
        }
        if self.read(item)? != desired {
            return Err(ExecError::Conflict);
        }
        Ok(())
    }
    fn action_type(&self) -> ActionType {
        self.kind
    }
    fn execution_scope(&self) -> Option<&str> {
        Some(self.context.scope())
    }
    fn ensure_write_ready(&self) -> ExecResult<()> {
        self.context.ensure_elevated()
    }
    fn capture(&self, item: &Item) -> ExecResult<Snapshot> {
        self.authorize(item)?;
        let snapshot = self.read(item)?;
        self.captured
            .borrow_mut()
            .insert(item.target.key(), snapshot.clone());
        Ok(snapshot)
    }
    fn apply(&mut self, item: &Item) -> ExecResult<()> {
        self.authorize(item)?;
        self.ensure_write_ready()?;
        let original = self
            .captured
            .borrow()
            .get(&item.target.key())
            .cloned()
            .ok_or(ExecError::Conflict)?;
        if self.read(item)? != original {
            return Err(ExecError::Conflict);
        }
        self.write_action(item, &original, false)?;
        if self.kind == ActionType::Appx {
            return match self.read(item) {
                Err(ExecError::NotFound) => Ok(()),
                Err(e) => Err(e),
                Ok(_) => Err(ExecError::Failed("app removal was incomplete".into())),
            };
        }
        let desired = Self::desired(item, &original)?;
        if !Self::changed_value_equal(&self.read(item)?, &desired) {
            return Err(ExecError::Conflict);
        }
        Ok(())
    }
    fn restore(&mut self, item: &Item, original: &Snapshot) -> ExecResult<()> {
        self.authorize(item)?;
        self.ensure_write_ready()?;
        if self.kind == ActionType::Appx {
            return Err(ExecError::ManualReinstallRequired);
        }
        if original.action_type() != self.kind {
            return Err(ExecError::TypeMismatch);
        }
        let current = self.read(item)?;
        let desired = Self::desired(item, original)?;
        if !Self::changed_value_equal(&current, original)
            && !Self::changed_value_equal(&current, &desired)
        {
            let partial = match (&current, original, &desired) {
                (
                    Snapshot::Service {
                        start_mode: a,
                        delayed_auto_start: ad,
                        running: false,
                    },
                    Snapshot::Service {
                        start_mode: b,
                        delayed_auto_start: bd,
                        ..
                    },
                    _,
                ) => a == b && ad == bd,
                (
                    Snapshot::Power {
                        active_guid: a,
                        existing_guids: ag,
                    },
                    Snapshot::Power { active_guid: b, .. },
                    Snapshot::Power {
                        existing_guids: dg, ..
                    },
                ) => a == b && ag == dg,
                _ => false,
            };
            if !partial {
                return Err(ExecError::Conflict);
            }
        }
        self.write_action(item, original, true)?;
        if self.read(item)? != *original {
            return Err(ExecError::Conflict);
        }
        Ok(())
    }
}

impl WindowsContext {
    pub fn executors(&self) -> Executors {
        let mut executors = Executors::default();
        for kind in [
            ActionType::Registry,
            ActionType::Service,
            ActionType::ScheduledTask,
            ActionType::Appx,
            ActionType::Startup,
            ActionType::Power,
        ] {
            executors.insert(WindowsExecutor::new(kind, self.clone()));
        }
        executors
    }
    pub fn open_reinstall(&self, action: &ReinstallAction) -> ExecResult<()> {
        let catalog = ametista_catalog::load_embedded().map_err(crate::failed)?;
        let item = catalog
            .get(action.item_id())
            .map_err(|_| ExecError::Protected)?;
        let Target::Appx { name } = &item.target else {
            return Err(ExecError::Protected);
        };
        if !action
            .family()
            .to_ascii_lowercase()
            .starts_with(&format!("{name}_").to_ascii_lowercase())
        {
            return Err(ExecError::Protected);
        }
        WindowsExecutor::script_write(
            Script::Store,
            &json!({"uri":action.uri(),"expectedSid":self.interactive_sid(),"operation":"launch"}),
        )
    }
}

/// Production orchestration for the future CLI/UI. Destructive APIs accept IDs/consent only.
pub struct WindowsSession {
    pub context: WindowsContext,
    catalog: Catalog,
    executors: Executors,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppliedBatch {
    pub restore_point: crate::RestorePointStatus,
    pub report: Report,
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRemovalPreview {
    pub item_id: String,
    pub all_users: bool,
    pub snapshot: Option<Snapshot>,
    pub error: Option<ExecError>,
}
impl WindowsSession {
    pub fn reconcile_games(&mut self, journal: &mut Journal, ids: &[String]) -> Result<Report> {
        let actions = ametista_catalog::game_actions(&self.catalog)?;
        ametista_core::temporary::reconcile(&mut self.engine(), journal, &actions, ids)
    }
    pub fn validate_request(&mut self, request: &Request) -> Result<()> {
        self.engine().validate_request(request)
    }
    pub fn removed_apps_readonly(&self) -> Result<Vec<RemovedApp>> {
        match RemovedApps::read_entries(
            self.context.state_directory().join("removed-apps.json"),
            self.context.scope(),
            &self.catalog,
        ) {
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
            other => other,
        }
    }
    pub fn history_readonly(&self) -> Result<JournalReadView> {
        match Journal::read_only(
            self.context.state_directory().join("journal.wal"),
            self.context.scope(),
        ) {
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => Ok(JournalReadView {
                entries: vec![],
                needs_recovery: false,
                torn_tail: false,
            }),
            other => other,
        }
    }
    pub fn reinstall_actions(&mut self, id: &str) -> Result<Vec<ReinstallAction>> {
        let item = self.catalog.get(id)?;
        if item.action_type != ActionType::Appx {
            return Err(Error::UnknownItem(id.into()));
        }
        let history = RemovedApps::read_reinstall_actions(
            self.context.state_directory().join("removed-apps.json"),
            self.context.scope(),
            &self.catalog,
            id,
        );
        if let Ok(actions) = history {
            return Ok(actions);
        }
        let preview = self.engine().preview_app_removal(id)?;
        if let Ok(snapshot) = preview {
            return reinstall_from_inventory(&self.catalog, id, &snapshot);
        }
        match history {
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                Err(Error::Io(e))
            }
            _ => match preview {
                Err(ExecError::NotFound) => Ok(vec![]),
                Err(error) => Err(Error::Preflight(error.to_string())),
                Ok(_) => Ok(vec![]),
            },
        }
    }
    pub fn connect() -> ExecResult<Self> {
        let context = WindowsContext::detect()?;
        let catalog = context.catalog_with_startup()?;
        let executors = context.executors();
        Ok(Self {
            context,
            catalog,
            executors,
        })
    }
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }
    /// Inspect every catalog target without creating a store, journal or checkpoint.
    pub fn inspect(&self) -> Vec<Observation> {
        crate::scan(
            &self.context,
            &self.catalog,
            &std::sync::atomic::AtomicBool::new(false),
            |_| {},
        )
        .unwrap_or_else(|error| {
            self.catalog
                .items()
                .map(|item| observe(item, Err(error.clone()), ReadScope::Machine))
                .collect()
        })
    }
    fn engine(&mut self) -> Engine<'_> {
        Engine {
            catalog: &self.catalog,
            machine: self.context.machine(),
            executors: &mut self.executors,
        }
    }
    pub fn simulate(&mut self, request: &Request) -> Result<Report> {
        self.engine().run(request)
    }
    pub fn preview_app_removal(&mut self, id: &str) -> Result<AppRemovalPreview> {
        let all_users = self.context.elevated();
        let result = self.engine().preview_app_removal(id)?;
        Ok(match result {
            Ok(snapshot) => AppRemovalPreview {
                item_id: id.into(),
                all_users,
                snapshot: Some(snapshot),
                error: None,
            },
            Err(error) => AppRemovalPreview {
                item_id: id.into(),
                all_users,
                snapshot: None,
                error: Some(error),
            },
        })
    }
    pub fn apply(&mut self, request: &Request) -> Result<AppliedBatch> {
        self.engine().run(request)?; // authorize all IDs and confirmations before any writes
        self.context
            .ensure_elevated()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let store = self
            .context
            .protected_store()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let mut journal = store.journal()?;
        if journal.needs_recovery() {
            return Err(Error::RecoveryRequired);
        }
        let restore_point = self
            .context
            .create_restore_point()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let report = self.engine().apply(request, &mut journal)?;
        store.export_journal(&journal)?;
        let _ = store.log(crate::logging::Event::Apply, true);
        Ok(AppliedBatch {
            restore_point,
            report,
        })
    }
    pub fn undo(&mut self, ids: &[String]) -> Result<Report> {
        let store = self
            .context
            .protected_store()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let mut journal = store.journal()?;
        let report = self.engine().undo(ids, &mut journal)?;
        store.export_journal(&journal)?;
        let _ = store.log(crate::logging::Event::Undo, true);
        Ok(report)
    }
    pub fn remove_app(
        &mut self,
        consent: &AppRemovalConsent,
    ) -> Result<std::result::Result<(), ExecError>> {
        let item = self.catalog.get(&consent.item_id)?;
        if item.action_type != ActionType::Appx
            || !consent.acknowledge_no_undo
            || (item.risk == Risk::High && !consent.acknowledge_gaming_dependencies)
            || (!item.verified && !consent.acknowledge_unverified)
        {
            return Err(Error::ConfirmationRequired(item.id.clone()));
        }
        let store = self
            .context
            .protected_store()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let mut apps = store.removed_apps()?;
        if apps.pending() {
            return Err(Error::RecoveryRequired);
        }
        self.context
            .create_restore_point()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let result = self.engine().remove_app(consent, &mut apps);
        let _ = store.log(
            crate::logging::Event::RemoveApp,
            matches!(result, Ok(Ok(()))),
        );
        result
    }
    pub fn reconcile_app_removal(&mut self, id: &str) -> Result<()> {
        let store = self
            .context
            .protected_store()
            .map_err(|e| Error::Preflight(e.to_string()))?;
        let mut apps = store.removed_apps()?;
        self.engine().reconcile_app_removal(id, &mut apps)
    }
}
