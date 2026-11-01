//! Positive target allowlist. Adding a catalog row alone cannot grant a new capability.
use crate::{Error, Item, Result, Revert, Risk, Target};

const APPROVED_TARGETS: &str = include_str!("approved_targets.json");

/// The appendix lists these network services but the absolute protection rule wins.
/// Keep their descriptions for transparency, never grant execution or preselection.
pub fn is_protected(item: &Item) -> bool {
    matches!(&item.target, Target::Service { name, .. }
        if ["icssvc", "WMPNetworkSvc", "XboxNetApiSvc"].contains(&name.as_str()))
}

pub fn validate(item: &Item) -> Result<()> {
    let allowed: Vec<Target> = serde_json::from_str(APPROVED_TARGETS)?;
    let invalid = |why: &str| Error::InvalidCatalog(format!("{}: {why}", item.id));
    if !allowed.contains(&item.target) {
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
