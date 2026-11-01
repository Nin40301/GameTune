use crate::{WindowsContext, registry};
use ametista_core::{
    Catalog, ExecError, ExecResult, Hive, Item, RegistryValue, StartupTarget, Target,
};
use sha2::{Digest, Sha256};

fn eligible_command(name: &str, value: &RegistryValue) -> bool {
    if ![1, 2].contains(&value.kind) || !value.bytes.len().is_multiple_of(2) {
        return false;
    }
    let words: Vec<_> = value
        .bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|b| u16::from_le_bytes(*b))
        .collect();
    let Ok(text) = String::from_utf16(&words) else {
        return false;
    };
    let command = text.trim_end_matches('\0').to_ascii_lowercase();
    if command.contains('%')
        || command.contains('\0')
        || [
            "securityhealth",
            "defender",
            "vanguard",
            "vgc.exe",
            "system32",
            "drivers",
            "anticheat",
            "battleye",
        ]
        .iter()
        .any(|s| command.contains(s))
    {
        return false;
    }
    let text = command.trim_start();
    let path = if let Some(quoted) = text.strip_prefix('"') {
        let Some(end) = quoted.find('"') else {
            return false;
        };
        &quoted[..end]
    } else {
        let Some(end) = text.find(char::is_whitespace) else {
            return expected_executable(name, text);
        };
        &text[..end]
    };
    expected_executable(name, path)
}
fn expected_executable(name: &str, path: &str) -> bool {
    if path.len() < 4
        || !path.as_bytes()[0].is_ascii_alphabetic()
        || !path.starts_with(&format!("{}:\\", &path[..1]))
    {
        return false;
    }
    let executable = path.rsplit('\\').next().unwrap_or("");
    match name {
        "Steam" => executable == "steam.exe",
        "Spotify" => executable == "spotify.exe",
        "EpicGamesLauncher" => executable == "epicgameslauncher.exe",
        "Discord" => {
            executable == "discord.exe"
                || (executable == "update.exe" && path.contains("\\discord\\"))
        }
        "com.squirrel.Teams.Teams" | "MicrosoftTeams" | "MSTeams" => {
            ["teams.exe", "ms-teams.exe"].contains(&executable)
        }
        "Skype" => ["skype.exe", "skypeapp.exe"].contains(&executable),
        _ => false,
    }
}

fn fingerprint(context: &WindowsContext, hive: Hive, name: &str, value: &RegistryValue) -> String {
    let mut hash = Sha256::new();
    hash.update(context.scope());
    hash.update(format!("{hive:?}:{name}:{}:", value.kind));
    hash.update(&value.bytes);
    format!("{:x}", hash.finalize())
}
impl WindowsContext {
    pub fn catalog_with_startup(&self) -> ExecResult<Catalog> {
        let catalog = ametista_catalog::load_embedded().map_err(crate::failed)?;
        let mut items: Vec<_> = catalog.items().cloned().collect();
        for hive in [Hive::HKCU, Hive::HKLM] {
            let descriptor = match hive {
                Hive::HKCU => "startup.hkcu",
                Hive::HKLM => "startup.hklm",
            };
            for (name, value) in registry::run_entries(self, hive)? {
                if !ametista_core::protection::permitted_startup_name(&name) {
                    continue;
                }
                if !eligible_command(&name, &value) {
                    continue;
                }
                let token = fingerprint(self, hive, &name, &value);
                let mut item = catalog.get(descriptor).map_err(crate::failed)?.clone();
                item.id = format!("{descriptor}.{token}");
                item.target = Target::Startup {
                    hive,
                    entry: Some(StartupTarget {
                        name: name.clone(),
                        fingerprint: token,
                    }),
                };
                item.name.pt_br = format!("{} — {name}", item.name.pt_br);
                item.name.en = format!("{} — {name}", item.name.en);
                items.push(item);
            }
        }
        Catalog::try_new(items).map_err(crate::failed)
    }
}
pub(crate) fn bound(context: &WindowsContext, item: &Item) -> ExecResult<(Hive, String)> {
    let Target::Startup {
        hive,
        entry: Some(entry),
    } = &item.target
    else {
        return Err(ExecError::NotFound);
    };
    let (value, _) = registry::capture(context, *hive, registry::RUN, &entry.name)?;
    let value = value.ok_or(ExecError::NotFound)?;
    let mut permitted = eligible_command(&entry.name, &value);
    #[cfg(feature = "vm-tests")]
    if ametista_core::protection::valid_test_target(item) {
        permitted = true;
    }
    let _ = &mut permitted;
    if !permitted {
        return Err(ExecError::Protected);
    }
    if fingerprint(context, *hive, &entry.name, &value) != entry.fingerprint {
        return Err(ExecError::Conflict);
    }
    Ok((*hive, entry.name.clone()))
}

#[cfg(feature = "vm-tests")]
pub(crate) fn test_item(
    context: &WindowsContext,
    mut item: Item,
    name: String,
    value: &RegistryValue,
) -> Item {
    item.target = Target::Startup {
        hive: Hive::HKCU,
        entry: Some(StartupTarget {
            fingerprint: fingerprint(context, Hive::HKCU, &name, value),
            name,
        }),
    };
    item
}
#[cfg(test)]
mod tests {
    use super::*;
    fn reg(s: &str) -> RegistryValue {
        RegistryValue {
            kind: 1,
            bytes: s
                .encode_utf16()
                .chain([0])
                .flat_map(u16::to_le_bytes)
                .collect(),
        }
    }
    #[test]
    fn startup_names_do_not_grant_arbitrary_executable_targets() {
        assert!(eligible_command(
            "Steam",
            &reg(r#""C:\Games\Steam\steam.exe" -silent"#)
        ));
        for command in [
            r#""C:\Windows\System32\SecurityHealthSystray.exe""#,
            r#""C:\Other\arbitrary.exe""#,
            r#"%STEAM_EXECUTABLE% -silent"#,
            r#"\\server\steam.exe"#,
        ] {
            assert!(!eligible_command("Steam", &reg(command)));
        }
        assert!(!eligible_command(
            "SecurityHealth",
            &reg(r#"C:\Games\Steam\steam.exe"#)
        ));
    }
}
