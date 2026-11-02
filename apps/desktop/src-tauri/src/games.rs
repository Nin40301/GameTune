#![forbid(unsafe_code)]
use ametista_watcher::{
    discovery::{Candidate, Locations},
    storage::*,
    *,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex};
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Status {},
    Discover {},
    AddManual {},
    AddCandidate {
        index: usize,
    },
    Save {
        game: Game,
        language: String,
    },
    Remove {
        id: String,
        confirm: bool,
    },
    Start {
        confirm: bool,
        readonly: bool,
        language: String,
    },
    Stop {},
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub library: Library,
    pub candidates: Vec<Candidate>,
    pub actions: Vec<ametista_core::temporary::GameAction>,
    pub monitor: MonitorStatus,
}
pub struct Games {
    directory: PathBuf,
    candidates: Mutex<Vec<Candidate>>,
    actions: Vec<ametista_core::temporary::GameAction>,
}
impl Games {
    pub fn new(directory: PathBuf) -> Result<Self> {
        let catalog = ametista_catalog::load_embedded().map_err(|_| Error::Invalid)?;
        let actions = ametista_catalog::game_actions(&catalog).map_err(|_| Error::Invalid)?;
        Ok(Self {
            directory,
            candidates: Mutex::new(vec![]),
            actions,
        })
    }
    fn view(&self) -> Result<View> {
        let mut monitor = status(&self.directory);
        let library = match load(&self.directory) {
            Ok(library) => library,
            Err(_) => {
                monitor.error = Some("games_invalid".into());
                Library::default()
            }
        };
        Ok(View {
            library,
            candidates: self.candidates.lock().map_err(|_| Error::Storage)?.clone(),
            actions: self.actions.clone(),
            monitor,
        })
    }
    pub fn action(&self, action: Action) -> std::result::Result<View, String> {
        self.apply(action).map_err(|e| e.to_string())
    }
    fn apply(&self, action: Action) -> Result<View> {
        match action {
            Action::Status {} => {}
            Action::Discover {} => {
                let program = std::env::var_os("ProgramFiles(x86)")
                    .or_else(|| std::env::var_os("ProgramFiles"))
                    .map(PathBuf::from)
                    .ok_or(Error::Storage)?;
                let locations = Locations {
                    steam: vec![program.join("Steam")],
                    program_data: std::env::var_os("ProgramData")
                        .map(PathBuf::from)
                        .ok_or(Error::Storage)?,
                    roaming: std::env::var_os("APPDATA")
                        .map(PathBuf::from)
                        .ok_or(Error::Storage)?,
                };
                *self.candidates.lock().map_err(|_| Error::Storage)? =
                    discovery::discover(&locations);
            }
            Action::AddManual {} => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("EXE", &["exe"])
                    .pick_file()
                {
                    let path = discovery::local_executable(&path)?;
                    if !path.is_file() {
                        return Err(Error::Invalid);
                    }
                    let mut library = load(&self.directory)?;
                    library
                        .games
                        .push(Game::manual(path.to_string_lossy().into())?);
                    save_library(&self.directory, &library)?;
                }
            }
            Action::AddCandidate { index } => {
                let candidates = self.candidates.lock().map_err(|_| Error::Storage)?;
                let candidate = candidates.get(index).ok_or(Error::Invalid)?;
                if let Some(exe) = &candidate.exe {
                    let path = discovery::local_executable(&PathBuf::from(exe))?;
                    if !path.is_file() {
                        return Err(Error::Invalid);
                    }
                    let mut library = load(&self.directory)?;
                    let mut game = Game::manual(path.to_string_lossy().into())?;
                    game.name = candidate.name.clone();
                    library.games.push(game);
                    save_library(&self.directory, &library)?;
                } else {
                    return Err(Error::Invalid);
                }
            }
            Action::Save { game, language } => {
                let mut library = load(&self.directory)?;
                let original = library
                    .games
                    .iter_mut()
                    .find(|g| g.id == game.id)
                    .ok_or(Error::Invalid)?;
                if original.exe != game.exe {
                    return Err(Error::Invalid);
                }
                *original = game;
                library.language = language;
                save_library(&self.directory, &library)?;
            }
            Action::Remove { id, confirm } => {
                if !confirm {
                    return Err(Error::Invalid);
                }
                let mut library = load(&self.directory)?;
                library.games.retain(|g| g.id != id);
                save_library(&self.directory, &library)?;
            }
            Action::Start {
                confirm,
                readonly,
                language,
            } => {
                use std::os::windows::process::CommandExt;
                if !confirm || std::env::var_os("AMETISTA_RESOURCE_MODE").is_some() {
                    return Err(Error::Invalid);
                }
                match load(&self.directory) {
                    Ok(mut library) => {
                        library.language = language;
                        save_library(&self.directory, &library)?;
                    }
                    Err(_) if debt(&self.directory) => {
                        let _lease = lock(&self.directory)?;
                    }
                    Err(e) => return Err(e),
                }
                let exe = std::env::current_exe()
                    .map_err(|_| Error::Storage)?
                    .parent()
                    .ok_or(Error::Storage)?
                    .join("ametista-monitor.exe");
                if !exe.is_file() {
                    return Err(Error::Storage);
                }
                let mut command = std::process::Command::new(exe);
                command.creation_flags(0x08000000);
                if readonly {
                    command.arg("--readonly");
                }
                command.spawn().map_err(|_| Error::Storage)?;
                for _ in 0..40 {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    if status(&self.directory).running {
                        break;
                    }
                }
            }
            Action::Stop {} => {
                let live = status(&self.directory);
                if live.running {
                    save(&self.directory.join("monitor-stop.json"), &live.instance)?;
                }
            }
        }
        self.view()
    }
}
pub fn monitor_running() -> std::result::Result<bool, String> {
    // Tauri's identifier resolves to this same per-user known-folder directory.
    let directory = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .ok_or("games_storage")?
        .join("io.github.nin40301.ametista/games");
    Ok(status(&directory).running)
}
