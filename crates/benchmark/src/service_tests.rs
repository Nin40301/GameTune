use super::service::*;
use super::*;
// Construct no sampler thread: these are deterministic bridge/state tests.
#[test]
fn empty_actions_reject_payloads_and_conditions_are_not_inferred() {
    assert!(serde_json::from_str::<Action>(r#"{"kind":"cancel","command":"x"}"#).is_err());
    assert!(
        serde_json::from_str::<FileAction>(r#"{"kind":"choose_presentmon","path":"x"}"#).is_err()
    );
    assert!(
        !Conditions {
            apps_closed: true,
            same_workload: true,
            cooled: false,
            same_power: true
        }
        .accepted()
    );
}
