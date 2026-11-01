use ametista_ipc::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn encoded(command: Command, token: &str) -> TestResultWith<Vec<u8>> {
    Ok(serde_json::to_vec(&Envelope {
        version: 1,
        catalog_hash: ametista_catalog::embedded_hash(),
        nonce: token.into(),
        sequence: 1,
        command,
    })?)
}
type TestResultWith<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[test]
fn a_different_helper_catalog_cannot_execute_the_reviewed_ids() -> TestResult {
    let token = nonce()?;
    let catalog = ametista_catalog::load_embedded()?;
    let mut value: serde_json::Value = serde_json::from_slice(&encoded(Command::History, &token)?)?;
    value["catalogHash"] = serde_json::json!("0".repeat(64));
    let mut gate = Gate::new("SID".into(), 42, token)?;
    assert!(matches!(
        gate.accept(&serde_json::to_vec(&value)?, "SID", 42, &catalog),
        Err(Error::CatalogMismatch)
    ));
    Ok(())
}
#[test]
fn invalid_id_is_rejected_before_execution() -> TestResult {
    let token = nonce()?;
    let catalog = ametista_catalog::load_embedded()?;
    let mut gate = Gate::new("SID".into(), 42, token.clone())?;
    let bytes = encoded(
        Command::Undo {
            ids: vec!["not.in.catalog".into()],
            confirm: true,
        },
        &token,
    )?;
    assert!(matches!(
        gate.accept(&bytes, "SID", 42, &catalog),
        Err(Error::InvalidItem)
    ));
    Ok(())
}
#[test]
fn another_user_or_process_cannot_use_the_pipe() -> TestResult {
    let token = nonce()?;
    let catalog = ametista_catalog::load_embedded()?;
    let bytes = encoded(Command::History, &token)?;
    for (sid, pid) in [("OTHER", 42), ("SID", 99)] {
        let mut gate = Gate::new("SID".into(), 42, token.clone())?;
        assert!(matches!(
            gate.accept(&bytes, sid, pid, &catalog),
            Err(Error::Identity)
        ));
    }
    Ok(())
}
#[test]
fn replay_and_wrong_nonce_are_rejected() -> TestResult {
    let token = nonce()?;
    let catalog = ametista_catalog::load_embedded()?;
    let bytes = encoded(Command::History, &token)?;
    let mut gate = Gate::new("SID".into(), 42, token.clone())?;
    gate.accept(&bytes, "SID", 42, &catalog)?;
    assert!(matches!(
        gate.accept(&bytes, "SID", 42, &catalog),
        Err(Error::Replay)
    ));
    let mut gate = Gate::new("SID".into(), 42, nonce()?)?;
    assert!(gate.accept(&bytes, "SID", 42, &catalog).is_err());
    Ok(())
}
#[test]
fn malformed_oversized_and_free_form_payloads_are_rejected() -> TestResult {
    let catalog = ametista_catalog::load_embedded()?;
    for bytes in [
        b"{".to_vec(),
        vec![0u8; MAX_FRAME + 1],
        br#"{"version":1,"nonce":"x","sequence":1,"command":{"kind":"shell","command":"calc"}}"#
            .to_vec(),
        br#"{"version":1,"nonce":"x","sequence":1,"command":{"kind":"history"},"path":"C:\\x"}"#
            .to_vec(),
    ] {
        let mut gate = Gate::new("SID".into(), 42, nonce()?)?;
        assert!(gate.accept(&bytes, "SID", 42, &catalog).is_err());
        assert!(matches!(
            gate.accept(b"{}", "SID", 42, &catalog),
            Err(Error::Replay)
        ));
    }
    Ok(())
}
#[test]
fn missing_or_reinforced_app_consent_is_rejected() -> TestResult {
    let catalog = ametista_catalog::load_embedded()?;
    for command in [
        Command::UndoAll { confirm: false },
        Command::RemoveApps {
            confirm: true,
            apps: vec![AppConsent {
                item_id: "appx.microsoft.gamingservices".into(),
                no_undo: true,
                gaming_risk: false,
                unverified: true,
            }],
        },
        Command::RemoveApps {
            confirm: true,
            apps: vec![AppConsent {
                item_id: "appx.microsoft.copilot".into(),
                no_undo: false,
                gaming_risk: false,
                unverified: false,
            }],
        },
    ] {
        assert!(matches!(
            command.validate_shape(&catalog),
            Err(Error::Confirmation)
        ));
    }
    Ok(())
}
