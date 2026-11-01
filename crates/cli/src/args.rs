use crate::Failure;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Analyze,
    List,
    Undo(Option<String>),
    Remove(String),
    Reinstall(String),
    Reconcile(String),
    Help,
    Version,
}
#[derive(Debug, Clone)]
pub struct Options {
    pub action: Action,
    pub profile: String,
    pub apply: bool,
    pub json: bool,
    pub language: String,
    pub no_undo: bool,
    pub gaming_risk: bool,
    pub ack_risk: BTreeSet<String>,
    pub ack_unverified: BTreeSet<String>,
}
impl Options {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        if args.len() > 1024 {
            return Err(Failure::usage());
        }
        let mut options = Self {
            action: Action::Analyze,
            profile: "safe".into(),
            apply: false,
            json: false,
            language: "pt-BR".into(),
            no_undo: false,
            gaming_risk: false,
            ack_risk: BTreeSet::new(),
            ack_unverified: BTreeSet::new(),
        };
        let (mut dry, mut confirm, mut explicit_profile, mut format_set) =
            (false, false, false, false);
        let mut seen = BTreeSet::new();
        let mut index = 0;
        while index < args.len() {
            let arg = args[index].as_str();
            if !["--ack-risk", "--ack-unverified"].contains(&arg) && !seen.insert(arg) {
                return Err(Failure::usage());
            }
            match arg {
                "--profile" => {
                    options.profile = value(args, &mut index)?;
                    explicit_profile = true;
                }
                "--lang" => {
                    options.language = value(args, &mut index)?;
                    if !["pt-BR", "en"].contains(&options.language.as_str()) {
                        return Err(Failure::usage());
                    }
                }
                "--apply" => options.apply = true,
                "--dry-run" => dry = true,
                "--confirm" => confirm = true,
                "--json" | "--text" => {
                    if format_set {
                        return Err(Failure::usage());
                    }
                    format_set = true;
                    options.json = arg == "--json";
                }
                "--ack-no-undo" => options.no_undo = true,
                "--ack-gaming-risk" => options.gaming_risk = true,
                "--ack-risk" => {
                    options.ack_risk.insert(value(args, &mut index)?);
                }
                "--ack-unverified" => {
                    options.ack_unverified.insert(value(args, &mut index)?);
                }
                "--list" => set_action(&mut options.action, Action::List)?,
                "--help" | "-h" => set_action(&mut options.action, Action::Help)?,
                "--version" => set_action(&mut options.action, Action::Version)?,
                "--undo" => {
                    let id = if args.get(index + 1).is_some_and(|s| !s.starts_with('-')) {
                        index += 1;
                        let id = args[index].clone();
                        if id == "all" { None } else { Some(id) }
                    } else {
                        None
                    };
                    set_action(&mut options.action, Action::Undo(id))?;
                }
                "appx" | "apps" => {
                    let verb = value(args, &mut index)?;
                    let id = value(args, &mut index)?;
                    let action = match verb.as_str() {
                        "remove" => Action::Remove(id),
                        "reinstall" => Action::Reinstall(id),
                        "reconcile" => Action::Reconcile(id),
                        _ => return Err(Failure::usage()),
                    };
                    set_action(&mut options.action, action)?;
                }
                _ => return Err(Failure::usage()),
            }
            index += 1;
        }
        if dry && options.apply {
            return Err(Failure::usage());
        }
        if options.apply && !confirm {
            return Err(Failure::new(4, "confirmation"));
        }
        if confirm && !options.apply {
            return Err(Failure::usage());
        }
        if explicit_profile && !matches!(options.action, Action::Analyze | Action::List) {
            return Err(Failure::usage());
        }
        if options.apply
            && matches!(
                options.action,
                Action::List | Action::Help | Action::Version
            )
        {
            return Err(Failure::usage());
        }
        if (options.no_undo || options.gaming_risk) && !matches!(options.action, Action::Remove(_))
        {
            return Err(Failure::usage());
        }
        if options.apply && matches!(options.action, Action::Remove(_)) && !options.no_undo {
            return Err(Failure::new(4, "noUndoRequired"));
        }
        Ok(options)
    }
}
fn value(args: &[String], index: &mut usize) -> Result<String, Failure> {
    *index += 1;
    args.get(*index)
        .filter(|s| !s.starts_with('-') && !s.is_empty())
        .cloned()
        .ok_or_else(Failure::usage)
}
fn set_action(action: &mut Action, next: Action) -> Result<(), Failure> {
    if !matches!(action, Action::Analyze) {
        return Err(Failure::usage());
    }
    *action = next;
    Ok(())
}
