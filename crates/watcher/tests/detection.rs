#![forbid(unsafe_code)]
use ametista_watcher::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
fn game(path: &str, ids: &[&str]) -> Game {
    let mut game = Game::manual(path.into()).unwrap_or_else(|e| panic!("fixture: {e}"));
    game.enabled = true;
    game.consent = true;
    game.action_ids = ids.iter().map(|s| s.to_string()).collect();
    game
}
fn event(game: &Game, pid: u32, generation: &str, started: bool) -> Event {
    Event {
        started,
        key: ProcessKey {
            pid,
            generation: generation.into(),
        },
        path: Some(game.exe.clone()),
    }
}
#[test]
fn matching_requires_full_path_not_basename_and_deduplicates_events() -> TestResult {
    let game = game(r"C:\Games\Game.exe", &["reg.game_mode"]);
    let mut detector = Detector::new(Library {
        games: vec![game.clone()],
        ..Library::default()
    })?;
    let mut wrong = event(&game, 10, "one", true);
    wrong.path = Some(r"C:\Other\Game.exe".into());
    assert!(!detector.observe(wrong)?);
    let mut missing = event(&game, 10, "one", true);
    missing.path = None;
    assert!(!detector.observe(missing)?);
    assert!(detector.observe(event(&game, 10, "one", true))?);
    assert!(!detector.observe(event(&game, 10, "one", true))?);
    assert!(detector.observe(event(&game, 10, "one", false))?);
    assert!(detector.desired().is_empty());
    Ok(())
}
#[test]
fn overlapping_games_and_instances_keep_shared_actions_until_last_exit() -> TestResult {
    let a = game(r"C:\Games\A.exe", &["reg.game_mode"]);
    let b = game(r"C:\Games\B.exe", &["reg.game_mode", "svc.fax"]);
    let mut d = Detector::new(Library {
        games: vec![a.clone(), b.clone()],
        ..Library::default()
    })?;
    d.observe(event(&a, 10, "a", true))?;
    d.observe(event(&b, 20, "b", true))?;
    d.observe(event(&b, 21, "c", true))?;
    d.observe(event(&a, 10, "a", false))?;
    d.observe(event(&b, 20, "b", false))?;
    assert_eq!(d.desired(), vec!["reg.game_mode", "svc.fax"]);
    d.observe(event(&b, 21, "c", false))?;
    assert!(d.desired().is_empty());
    Ok(())
}
#[test]
fn reused_pid_late_stop_and_late_start_cannot_revive_old_session() -> TestResult {
    let game = game(r"C:\Games\Game.exe", &["reg.game_mode"]);
    let mut d = Detector::new(Library {
        games: vec![game.clone()],
        ..Library::default()
    })?;
    d.observe(event(&game, 10, "old", true))?;
    d.observe(event(&game, 10, "new", true))?;
    d.observe(event(&game, 10, "old", false))?;
    d.observe(event(&game, 10, "old", true))?;
    assert!(!d.desired().is_empty());
    let mut ended = event(&game, 10, "new", false);
    ended.path = None;
    d.observe(ended)?;
    assert!(d.desired().is_empty());
    d.observe(event(&game, 10, "new", true))?;
    assert!(d.desired().is_empty());
    Ok(())
}
#[test]
fn reused_pid_for_unrelated_program_drops_stale_game_without_name_match() -> TestResult {
    let g = game(r"C:\Games\Game.exe", &["reg.game_mode"]);
    let mut d = Detector::new(Library {
        games: vec![g.clone()],
        ..Library::default()
    })?;
    d.observe(event(&g, 10, "old", true))?;
    assert!(d.observe(Event {
        started: true,
        key: ProcessKey {
            pid: 10,
            generation: "new".into()
        },
        path: Some(r"C:\Other\Other.exe".into())
    })?);
    assert!(d.desired().is_empty());
    Ok(())
}
#[test]
fn consent_and_closed_ids_are_required_and_paths_reject_commands() -> TestResult {
    for path in [
        r"\\server\game.exe",
        r"C:\Games\..\Windows\cmd.exe",
        r"C:\a.exe:stream.exe",
        r"C:\a.exe --run",
        r"cmd.exe",
    ] {
        assert!(normalize_exe(path).is_err());
    }
    let mut g = game(r"C:\Games\Game.exe", &["reg.game_mode"]);
    g.consent = false;
    assert!(
        Detector::new(Library {
            games: vec![g.clone()],
            ..Library::default()
        })
        .is_err()
    );
    g.consent = true;
    g.action_ids = vec!["svc.xblauthmanager".into()];
    assert!(
        Detector::new(Library {
            games: vec![g],
            ..Library::default()
        })
        .is_err()
    );
    Ok(())
}
#[test]
fn stop_without_generation_fails_closed_and_disabled_profiles_do_nothing() -> TestResult {
    let game = game(r"C:\Games\Game.exe", &["reg.game_mode"]);
    let mut d = Detector::new(Library {
        games: vec![game.clone()],
        ..Library::default()
    })?;
    d.observe(event(&game, 11, "one", true))?;
    assert!(d.observe(event(&game, 11, "", false)).is_err());
    let mut disabled = game.clone();
    disabled.enabled = false;
    let mut d = Detector::new(Library {
        games: vec![disabled],
        ..Library::default()
    })?;
    assert!(!d.observe(event(&game, 11, "one", true))?);
    Ok(())
}
