use crate::WindowsContext;
use ametista_core::{ExecResult, Hive, RegistryValue};

#[cfg(windows)]
mod native {
    use super::*;
    use ametista_core::ExecError;
    use winreg::{RegKey, RegValue, enums::*};

    fn location(context: &WindowsContext, hive: Hive, path: &str) -> (RegKey, String) {
        match hive {
            Hive::HKCU => (
                RegKey::predef(HKEY_USERS),
                format!("{}\\{path}", context.interactive_sid()),
            ),
            Hive::HKLM => (RegKey::predef(HKEY_LOCAL_MACHINE), path.into()),
        }
    }
    pub fn system_root() -> ExecResult<String> {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(
                "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
                KEY_READ | KEY_WOW64_64KEY,
            )
            .and_then(|k| k.get_value("SystemRoot"))
            .map_err(crate::io_error)
    }
    pub fn capture(
        context: &WindowsContext,
        hive: Hive,
        path: &str,
        name: &str,
    ) -> ExecResult<(Option<RegistryValue>, Vec<String>)> {
        let (base, absolute) = location(context, hive, path);
        let mut missing = Vec::new();
        let mut prefix = String::new();
        for part in path.split('\\') {
            if !prefix.is_empty() {
                prefix.push('\\');
            }
            prefix.push_str(part);
            let (_, p) = location(context, hive, &prefix);
            match base.open_subkey_with_flags(&p, KEY_READ | KEY_WOW64_64KEY) {
                Ok(_) => (),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => missing.push(prefix.clone()),
                Err(e) => return Err(crate::io_error(e)),
            }
        }
        let key = match base.open_subkey_with_flags(absolute, KEY_READ | KEY_WOW64_64KEY) {
            Ok(key) => key,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok((None, missing)),
            Err(e) => return Err(crate::io_error(e)),
        };
        let value = match key.get_raw_value(name) {
            Ok(v) => Some(RegistryValue {
                kind: v.vtype as u32,
                bytes: v.bytes,
            }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(crate::io_error(e)),
        };
        Ok((value, missing))
    }
    fn kind(value: u32) -> ExecResult<RegType> {
        Ok(match value {
            0 => REG_NONE,
            1 => REG_SZ,
            2 => REG_EXPAND_SZ,
            3 => REG_BINARY,
            4 => REG_DWORD,
            5 => REG_DWORD_BIG_ENDIAN,
            6 => REG_LINK,
            7 => REG_MULTI_SZ,
            8 => REG_RESOURCE_LIST,
            9 => REG_FULL_RESOURCE_DESCRIPTOR,
            10 => REG_RESOURCE_REQUIREMENTS_LIST,
            11 => REG_QWORD,
            _ => return Err(ExecError::Failed("unsupported registry value type".into())),
        })
    }
    pub fn write(
        context: &WindowsContext,
        hive: Hive,
        path: &str,
        name: &str,
        value: &RegistryValue,
    ) -> ExecResult<()> {
        let (base, path) = location(context, hive, path);
        let (key, _) = base
            .create_subkey_with_flags(path, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY)
            .map_err(crate::io_error)?;
        key.set_raw_value(
            name,
            &RegValue {
                vtype: kind(value.kind)?,
                bytes: value.bytes.clone(),
            },
        )
        .map_err(crate::io_error)?;
        Ok(())
    }
    pub fn restore(
        context: &WindowsContext,
        hive: Hive,
        path: &str,
        name: &str,
        value: &Option<RegistryValue>,
        created: &[String],
    ) -> ExecResult<()> {
        if let Some(value) = value {
            write(context, hive, path, name, value)?;
        } else {
            let (base, absolute) = location(context, hive, path);
            match base.open_subkey_with_flags(absolute, KEY_READ | KEY_WRITE | KEY_WOW64_64KEY) {
                Ok(key) => {
                    match key.delete_value(name) {
                        Ok(()) => (),
                        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                        Err(e) => return Err(crate::io_error(e)),
                    };
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(e) => return Err(crate::io_error(e)),
            }
        }
        for candidate in created.iter().rev() {
            if !(candidate == path || path.starts_with(&format!("{candidate}\\"))) {
                return Err(ExecError::Protected);
            }
            let (base, absolute) = location(context, hive, candidate);
            let key = match base.open_subkey_with_flags(&absolute, KEY_READ | KEY_WOW64_64KEY) {
                Ok(k) => k,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => return Err(crate::io_error(e)),
            };
            // RegDeleteKey deletes values too. Explicitly require BOTH no values and no children.
            if key.enum_values().next().is_some() || key.enum_keys().next().is_some() {
                return Err(ExecError::Conflict);
            }
            drop(key);
            base.delete_subkey_with_flags(&absolute, KEY_WOW64_64KEY)
                .map_err(crate::io_error)?;
        }
        Ok(())
    }
    pub fn run_entries(
        context: &WindowsContext,
        hive: Hive,
    ) -> ExecResult<Vec<(String, RegistryValue)>> {
        let (base, path) = location(context, hive, super::RUN);
        let key = match base.open_subkey_with_flags(path, KEY_READ | KEY_WOW64_64KEY) {
            Ok(k) => k,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(crate::io_error(e)),
        };
        key.enum_values()
            .map(|entry| {
                entry
                    .map(|(n, v)| {
                        (
                            n,
                            RegistryValue {
                                kind: v.vtype as u32,
                                bytes: v.bytes,
                            },
                        )
                    })
                    .map_err(crate::io_error)
            })
            .collect()
    }
}

pub(crate) const RUN: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
pub(crate) const APPROVED: &str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";
#[cfg(windows)]
pub(crate) use native::*;

#[cfg(not(windows))]
pub(crate) fn capture(
    _: &WindowsContext,
    _: Hive,
    _: &str,
    _: &str,
) -> ExecResult<(Option<RegistryValue>, Vec<String>)> {
    Err(ametista_core::ExecError::Unavailable)
}
#[cfg(not(windows))]
pub(crate) fn write(
    _: &WindowsContext,
    _: Hive,
    _: &str,
    _: &str,
    _: &RegistryValue,
) -> ExecResult<()> {
    Err(ametista_core::ExecError::Unavailable)
}
#[cfg(not(windows))]
pub(crate) fn restore(
    _: &WindowsContext,
    _: Hive,
    _: &str,
    _: &str,
    _: &Option<RegistryValue>,
    _: &[String],
) -> ExecResult<()> {
    Err(ametista_core::ExecError::Unavailable)
}
#[cfg(not(windows))]
pub(crate) fn run_entries(_: &WindowsContext, _: Hive) -> ExecResult<Vec<(String, RegistryValue)>> {
    Err(ametista_core::ExecError::Unavailable)
}
