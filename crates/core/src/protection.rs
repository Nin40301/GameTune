//! Positive target allowlist. Adding a catalog row alone cannot grant a new capability.
use crate::{Error, Item, Result, Revert, Risk, Target};

const APPROVED_TARGETS: &str = include_str!("approved_targets.json");

/// The appendix lists these network services but the absolute protection rule wins.
/// Keep their descriptions for transparency, never grant execution or preselection.
pub fn is_protected(item: &Item) -> bool {
    if matches!(&item.target,Target::Registry{name,..} if name=="DisableAIDataAnalysis") {
        return true;
    }
    matches!(&item.target, Target::Service { name, .. }
        if ["icssvc", "WMPNetworkSvc", "XboxNetApiSvc"].contains(&name.as_str()))
}

pub fn validate(item: &Item) -> Result<()> {
    let allowed: Vec<Target> = serde_json::from_str(APPROVED_TARGETS)?;
    let invalid = |why: &str| Error::InvalidCatalog(format!("{}: {why}", item.id));
    if !allowed.contains(&item.target) && !valid_dynamic_startup(item) && !valid_test_target(item) {
        return Err(invalid("target is outside the safety allowlist"));
    }
    if item.action_type != item.target.action_type()
        || item.level > 3
        || item.min_build < 19045
        || item.id.is_empty()
        || item.supported_editions.is_empty()
        || item.max_build.is_some_and(|max| max < item.min_build)
    {
        return Err(invalid("invalid type, level, build, id or editions"));
    }
    if item.verified && item.sources.is_empty() {
        return Err(invalid("verified item needs evidence"));
    }
    if matches!(item.target, Target::Appx { .. }) && item.revert != Revert::Manual {
        return Err(invalid("Appx cannot promise exact automatic restoration"));
    }
    if matches!(item.target, Target::Startup { .. }) && item.level != 0 {
        return Err(invalid("startup is always manual"));
    }
    let high_risk = match &item.target {
        Target::Service { name, .. } => [
            "XblAuthManager",
            "XblGameSave",
            "XboxNetApiSvc",
            "XboxGipSvc",
        ]
        .contains(&name.as_str()),
        Target::Appx { name } => {
            ["Microsoft.GamingServices", "Microsoft.XboxIdentityProvider"].contains(&name.as_str())
        }
        Target::Registry { name, .. } => name == "LetAppsRunInBackground",
        _ => false,
    };
    if (high_risk && item.risk != Risk::High)
        || (item.risk == Risk::High
            && (item.risk_note.pt_br.trim().is_empty() || item.risk_note.en.trim().is_empty()))
    {
        return Err(invalid("required risk warning is missing or downgraded"));
    }
    if matches!(&item.target, Target::Service { name, .. } if name == "SysMain")
        && !item.requires_ssd
    {
        return Err(invalid("SysMain requires SSD gating"));
    }
    if matches!(&item.target, Target::Registry { name, .. } if name == "HwSchMode")
        && !item.requires_reboot
    {
        return Err(invalid("HAGS requires a restart warning"));
    }
    Ok(())
}

pub fn permitted_startup_name(name: &str) -> bool {
    [
        "Steam",
        "Discord",
        "Spotify",
        "EpicGamesLauncher",
        "com.squirrel.Teams.Teams",
        "MicrosoftTeams",
        "MSTeams",
        "Skype",
    ]
    .contains(&name)
}
fn valid_dynamic_startup(item: &Item) -> bool {
    let Target::Startup {
        hive,
        entry: Some(entry),
    } = &item.target
    else {
        return false;
    };
    let prefix = match hive {
        crate::Hive::HKCU => "startup.hkcu.",
        crate::Hive::HKLM => "startup.hklm.",
    };
    permitted_startup_name(&entry.name)
        && entry.fingerprint.len() == 64
        && entry.fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
        && item.id == format!("{prefix}{}", entry.fingerprint)
        && item.level == 0
        && item.revert == Revert::Auto
}

#[cfg(not(feature = "test-targets"))]
fn valid_test_target(_: &Item) -> bool {
    false
}

/// Compiled out of production. Only fixed dedicated fixture namespaces are accepted.
#[cfg(feature = "test-targets")]
pub fn valid_test_target(item: &Item) -> bool {
    fn safe_suffix(s: &str) -> bool {
        !s.is_empty() && s.len() < 80 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
    }
    if !item.id.starts_with("test.") {
        return false;
    }
    match &item.target {
        Target::Registry {
            hive: crate::Hive::HKCU,
            path,
            name,
            ..
        } => {
            path.strip_prefix("Software\\AmetistaTest\\")
                .is_some_and(|s| s.split('\\').all(safe_suffix))
                && safe_suffix(name)
        }
        Target::Service { name, .. } => name.strip_prefix("AmetistaTest_").is_some_and(safe_suffix),
        Target::ScheduledTask { path, name } => {
            path == "\\AmetistaTest\\"
                && name.strip_prefix("AmetistaTest_").is_some_and(safe_suffix)
        }
        Target::Startup {
            hive: crate::Hive::HKCU,
            entry: Some(e),
        } => {
            e.name
                .strip_prefix("AmetistaTest_")
                .is_some_and(safe_suffix)
                && e.fingerprint.len() == 64
                && e.fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
        }
        Target::Power { guid } => {
            guid.starts_with("47544e54-")
                && guid.len() == 36
                && guid.bytes().enumerate().all(|(i, b)| {
                    if [8, 13, 18, 23].contains(&i) {
                        b == b'-'
                    } else {
                        b.is_ascii_hexdigit()
                    }
                })
        }
        _ => false,
    }
}
