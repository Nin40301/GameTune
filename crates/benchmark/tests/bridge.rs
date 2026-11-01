#![forbid(unsafe_code)]
use gametune_benchmark::service::{Action, FileAction};
#[test]
fn bridge_rejects_free_paths_commands_and_unknown_fields() {
    for text in [
        r#"{"kind":"status","script":"whoami"}"#,
        r#"{"kind":"execute","ids":[]}"#,
        r#"{"kind":"start","conditions":{"appsClosed":true,"sameWorkload":true,"cooled":true,"samePower":true,"command":"x"}}"#,
    ] {
        assert!(serde_json::from_str::<Action>(text).is_err());
    }
    for text in [
        r#"{"kind":"import_capture","path":"C:\\secret.csv"}"#,
        r#"{"kind":"export","format":"exe","anonymize":true,"english":false}"#,
        r#"{"kind":"export","format":"json","anonymize":true,"english":false,"command":"x"}"#,
    ] {
        assert!(serde_json::from_str::<FileAction>(text).is_err());
    }
}
