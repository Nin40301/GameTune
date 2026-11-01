#![forbid(unsafe_code)]
use gametune_watcher::discovery::*;
#[test]
fn parses_launcher_metadata_without_executing_commands() {
    let epic=json_candidates("epic",br#"{"DisplayName":"Test","InstallLocation":"C:\\Games\\Test","LaunchExecutable":"Binaries/Test.exe","LaunchCommand":"ignore me"}"#);
    assert!(epic.is_ok_and(|c| c[0].exe.as_deref() == Some(r"C:\Games\Test\Binaries\Test.exe")));
    let riot = json_candidates(
        "riot",
        br#"{"associated_client":{"C:/Riot/Games/Game.exe":"C:/Riot/Client.exe"}}"#,
    );
    assert!(riot.is_ok_and(|c| c.len() == 1));
    let minecraft=json_candidates("minecraft",br#"{"profiles":{"x":{"name":"Test","javaDir":"C:\\Java\\javaw.exe"}},"authenticationDatabase":{"token":"must not escape"}}"#);
    assert!(minecraft.is_ok_and(|c| c[0].exe.is_some()));
    let blizzard = json_candidates(
        "battlenet",
        br#"{"Games":{"wow":{"InstallPath":"C:\\Games\\WoW"}}}"#,
    );
    assert!(blizzard.is_ok_and(|c| c[0].exe.is_none()));
}
#[test]
fn steam_has_no_guessed_game_executable_and_traversal_is_rejected() {
    let c = steam_manifest(
        r#""AppState" {"name" "Test Game" "installdir" "Test"}"#,
        std::path::Path::new("C:/Steam/steamapps"),
    );
    assert!(c.is_ok_and(|c| c.exe.is_none() && c.name == "Test Game"));
    assert!(
        steam_manifest(
            r#""name" "Test" "installdir" "..\\Windows""#,
            std::path::Path::new("C:/Steam/steamapps")
        )
        .is_err()
    );
    assert!(json_candidates("epic",br#"{"DisplayName":"Test","InstallLocation":"C:\\Game","LaunchExecutable":"../evil.exe"}"#).is_ok_and(|c|c[0].exe.is_none()));
    assert!(vdf_pairs("\"unterminated").is_err());
}
