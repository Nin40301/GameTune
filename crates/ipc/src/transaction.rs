use crate::{Command, Error, Reply, Result};
use gametune_core::{AppRemovalConsent, Request};
use gametune_windows_runtime::WindowsSession;
use serde_json::json;

/// Independent elevated authorization. Called before creating any store/checkpoint.
pub fn validate(session: &mut WindowsSession, command: &Command) -> Result<()> {
    command.validate_shape(session.catalog())?;
    match command {
        Command::Apply {
            ids,
            high_risk_ids,
            unverified_ids,
            ..
        } => session
            .validate_request(&Request {
                item_ids: ids.clone(),
                high_risk_confirmations: high_risk_ids.clone(),
                unverified_confirmations: unverified_ids.clone(),
            })
            .map_err(|_| Error::Confirmation)?,
        Command::Undo { ids, .. } => {
            for id in ids {
                let item = session.catalog().get(id).map_err(|_| Error::InvalidItem)?;
                if item.revert != gametune_core::Revert::Auto {
                    return Err(Error::InvalidItem);
                }
            }
        }
        Command::RemoveApps { apps, .. } => {
            for app in apps {
                let item = session
                    .catalog()
                    .get(&app.item_id)
                    .map_err(|_| Error::InvalidItem)?;
                if !session.context.machine().supports(item)
                    || gametune_core::protection::is_protected(item)
                {
                    return Err(Error::InvalidItem);
                }
            }
        }
        Command::Reinstall { item_id, .. }
            if session
                .catalog()
                .get(item_id)
                .map_err(|_| Error::InvalidItem)?
                .action_type
                != gametune_core::ActionType::Appx =>
        {
            return Err(Error::InvalidItem);
        }
        _ => {}
    }
    Ok(())
}
pub fn execute(session: &mut WindowsSession, command: &Command) -> Reply {
    let operation = (|| -> Result<serde_json::Value> {
        validate(session, command)?;
        session
            .context
            .ensure_elevated()
            .map_err(|_| Error::Identity)?;
        let result = match command {
            Command::Apply {
                ids,
                high_risk_ids,
                unverified_ids,
                ..
            } => serde_json::to_value(
                session
                    .apply(&Request {
                        item_ids: ids.clone(),
                        high_risk_confirmations: high_risk_ids.clone(),
                        unverified_confirmations: unverified_ids.clone(),
                    })
                    .map_err(|_| Error::Operation)?,
            )?,
            Command::Undo { ids, .. } => {
                serde_json::to_value(session.undo(ids).map_err(|_| Error::Operation)?)?
            }
            Command::UndoAll { .. } => {
                serde_json::to_value(session.undo(&[]).map_err(|_| Error::Operation)?)?
            }
            Command::RemoveApps { apps, .. } => {
                let mut results = vec![];
                for app in apps {
                    let result = session.remove_app(&AppRemovalConsent {
                        item_id: app.item_id.clone(),
                        acknowledge_no_undo: app.no_undo,
                        acknowledge_gaming_dependencies: app.gaming_risk,
                        acknowledge_unverified: app.unverified,
                    });
                    let (ok, failure) = match result {
                        Ok(Ok(())) => (true, None),
                        Ok(Err(error)) => (false, Some(serde_json::to_value(error)?)),
                        Err(_) => (false, Some(json!({"code":"operation_failed"}))),
                    };
                    results.push(json!({"itemId":app.item_id,"ok":ok,"error":failure}));
                }
                json!({"apps":results,"undoRestores":false})
            }
            Command::History => {
                json!({"journal":session.history_readonly().map_err(|_|Error::Operation)?,"apps":session.removed_apps_readonly().map_err(|_|Error::Operation)?})
            }
            Command::Reinstall { item_id, .. } => {
                let actions = session
                    .reinstall_actions(item_id)
                    .map_err(|_| Error::Operation)?;
                if actions.is_empty() {
                    return Err(Error::InvalidItem);
                }
                for action in &actions {
                    session
                        .context
                        .open_reinstall(action)
                        .map_err(|_| Error::Operation)?;
                }
                json!({"storeOpened":true,"installedAutomatically":false})
            }
        };
        if matches!(command, Command::History) {
            Ok(result)
        } else {
            Ok(
                json!({"operation":result,"journal":session.history_readonly().ok(),"apps":session.removed_apps_readonly().unwrap_or_default()}),
            )
        }
    })();
    match operation {
        Ok(result) => Reply {
            ok: true,
            result,
            error: None,
        },
        Err(error) => Reply {
            ok: false,
            result: serde_json::Value::Null,
            error: Some(error.to_string()),
        },
    }
}
