use crate::{
    WindowsContext, WindowsExecutor, registry,
    transport::{self, Script},
};
use ametista_core::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgress {
    pub completed: usize,
    pub total: usize,
    pub item_id: String,
}
#[derive(Clone, Deserialize)]
struct AppInventory {
    snapshot: Snapshot,
    removable: bool,
}
fn read_one(
    context: &WindowsContext,
    item: &Item,
    apps: &OnceLock<ExecResult<BTreeMap<String, AppInventory>>>,
    names: &[String],
    cancel: &AtomicBool,
) -> Observation {
    let reader = WindowsExecutor::new(item.action_type, context.clone());
    let scope = match item.target {
        Target::Appx { .. } => {
            if context.elevated() {
                ReadScope::AllUsers
            } else {
                ReadScope::CurrentUserOnly
            }
        }
        Target::Registry {
            hive: Hive::HKCU, ..
        }
        | Target::Startup {
            hive: Hive::HKCU, ..
        } => ReadScope::InteractiveUser,
        _ => ReadScope::Machine,
    };
    if let Target::Startup { hive, entry: None } = &item.target {
        let state = registry::capture(context, *hive, registry::RUN, "");
        return Observation {
            item_id: item.id.clone(),
            presence: Presence::Discovery,
            exists: state.as_ref().ok().map(|(_, keys)| keys.is_empty()),
            key_exists: None,
            already_applied: None,
            current: None,
            current_dword: None,
            error: state.err(),
            scope: ReadScope::Discovery,
            removable: None,
        };
    }
    let result = reader.authorize_read(item).and_then(|()| {
        if let Target::Appx { name } = &item.target {
            apps.get_or_init(|| {
                transport::run_controlled(
                    Script::AppxInventory,
                    &serde_json::json!({"names":names}),
                    Some(cancel),
                )
            })
            .as_ref()
            .map_err(Clone::clone)?
            .get(name)
            .map(|a| a.snapshot.clone())
            .ok_or(ExecError::NotFound)
        } else {
            reader.read_controlled(item, Some(cancel))
        }
    });
    let mut row = observe(item, result, scope);
    if let Target::Appx { name } = &item.target {
        row.removable = apps
            .get()
            .and_then(|r| r.as_ref().ok())
            .and_then(|m| m.get(name))
            .map(|a| a.removable);
    }
    row
}
/// At most four concurrent read-only operations. No persistent cache or writer.
pub fn scan(
    context: &WindowsContext,
    catalog: &Catalog,
    cancel: &AtomicBool,
    progress: impl Fn(ScanProgress) + Sync,
) -> ExecResult<Vec<Observation>> {
    let items: Vec<_> = catalog.items().cloned().collect();
    let names: Vec<_> = items
        .iter()
        .filter_map(|i| {
            if let Target::Appx { name } = &i.target {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();
    let next = AtomicUsize::new(0);
    let completed = AtomicUsize::new(0);
    let rows = Mutex::new(Vec::with_capacity(items.len()));
    let apps = OnceLock::new();
    std::thread::scope(|scope| {
        let mut workers = vec![];
        for _ in 0..4 {
            workers.push(scope.spawn(|| -> ExecResult<()> {
                loop {
                    if cancel.load(Ordering::Acquire) {
                        break;
                    }
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(index) else {
                        break;
                    };
                    let row = read_one(context, item, &apps, &names, cancel);
                    if cancel.load(Ordering::Acquire) {
                        break;
                    }
                    rows.lock().map_err(|_| ExecError::Conflict)?.push(row);
                    let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                    progress(ScanProgress {
                        completed: done,
                        total: items.len(),
                        item_id: item.id.clone(),
                    });
                }
                Ok(())
            }));
        }
        for worker in workers {
            worker.join().map_err(|_| ExecError::Conflict)??;
        }
        Ok::<_, ExecError>(())
    })?;
    if cancel.load(Ordering::Acquire) {
        return Err(ExecError::Conflict);
    }
    let mut rows = rows.into_inner().map_err(|_| ExecError::Conflict)?;
    rows.sort_by(|a, b| a.item_id.cmp(&b.item_id));
    Ok(rows)
}
