#![forbid(unsafe_code)]
use ametista_ipc::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
#[test]
fn game_frames_reject_replay_foreign_nonce_free_paths_and_unapproved_ids() -> TestResult {
    let nonce = "a".repeat(64);
    let mut gate = GameGate::new(nonce.clone(), vec!["reg.game_mode".into()])?;
    let bytes = serde_json::to_vec(&GameFrame {
        nonce: nonce.clone(),
        sequence: 2,
        step: GameStep::Set {
            ids: vec!["reg.game_mode".into()],
        },
    })?;
    assert!(gate.accept(&bytes).is_ok());
    assert!(gate.accept(&bytes).is_err());
    let bad = serde_json::to_vec(&GameFrame {
        nonce: "b".repeat(64),
        sequence: 3,
        step: GameStep::Ping {},
    })?;
    assert!(gate.accept(&bad).is_err());
    let bad = serde_json::to_vec(&GameFrame {
        nonce,
        sequence: 3,
        step: GameStep::Set {
            ids: vec!["svc.xblauthmanager".into()],
        },
    })?;
    assert!(gate.accept(&bad).is_err());
    assert!(serde_json::from_str::<GameStep>(r#"{"kind":"ping","command":"x"}"#).is_err());
    assert!(serde_json::from_str::<GameStep>(r#"{"kind":"set","ids":[],"path":"C:\\x"}"#).is_err());
    Ok(())
}
#[test]
fn initial_session_authorization_only_accepts_safe_temporary_subset() -> TestResult {
    let catalog = ametista_catalog::load_embedded()?;
    for id in [
        "reg.cortana",
        "svc.xblauthmanager",
        "appx.microsoft.copilot",
    ] {
        assert!(
            Command::TemporarySession {
                ids: vec![id.into()],
                confirm: true
            }
            .validate_shape(&catalog)
            .is_err()
        );
    }
    assert!(
        Command::TemporarySession {
            ids: vec![],
            confirm: true
        }
        .validate_shape(&catalog)
        .is_ok()
    );
    assert!(
        Command::TemporarySession {
            ids: vec!["reg.game_mode".into()],
            confirm: false
        }
        .validate_shape(&catalog)
        .is_err()
    );
    Ok(())
}
