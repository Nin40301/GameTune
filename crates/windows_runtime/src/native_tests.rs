//! VM-only HKCU integration, exclusively below Software\GameTuneTest.
#![cfg(windows)]
use crate::{WindowsContext, registry};
use gametune_core::{ExecError, Hive, RegistryValue};
use std::collections::BTreeMap;
use winreg::{RegKey, enums::*};
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn require_vm() -> TestResult {
    if !cfg!(feature = "vm-tests")
        || std::env::var("GAMETUNE_DISPOSABLE_VM").as_deref() != Ok("I_HAVE_A_SNAPSHOT")
    {
        return Err("dedicated HKCU tests require vm-tests and the disposable VM opt-in".into());
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    values: BTreeMap<String, (u32, Vec<u8>)>,
    children: BTreeMap<String, Tree>,
}
fn tree(key: &RegKey) -> std::io::Result<Tree> {
    let values = key
        .enum_values()
        .map(|e| e.map(|(n, v)| (n, (v.vtype as u32, v.bytes))))
        .collect::<std::io::Result<BTreeMap<_, _>>>()?;
    let children = key
        .enum_keys()
        .map(|e| {
            let name = e?;
            Ok((name.clone(), tree(&key.open_subkey(&name)?)?))
        })
        .collect::<std::io::Result<BTreeMap<_, _>>>()?;
    Ok(Tree { values, children })
}
fn unique() -> String {
    format!(
        "R{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    )
}

#[test]
#[ignore = "writes dedicated HKCU targets; disposable VM opt-in only"]
fn dedicated_hkcu_raw_types_and_missing_keys_roundtrip() -> TestResult {
    require_vm()?;
    let _guard = TEST_LOCK.lock().map_err(|_| "native test lock poisoned")?;
    let context = WindowsContext::current_test_user()?;
    let name = unique();
    let root_path = format!("Software\\GameTuneTest\\{name}");
    assert!(root_path.starts_with("Software\\GameTuneTest\\R"));
    let users = RegKey::predef(HKEY_USERS);
    let absolute = format!("{}\\{root_path}", context.interactive_sid());
    let parent_existed = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\GameTuneTest")
        .is_ok();
    let (root, _) = users.create_subkey(&absolute)?;
    let result = (|| -> TestResult {
        let values = [
            RegistryValue {
                kind: 0,
                bytes: vec![0, 1, 255],
            },
            RegistryValue {
                kind: 1,
                bytes: vec![65, 0, 0, 0, 0, 0],
            },
            RegistryValue {
                kind: 2,
                bytes: "%TEMP%"
                    .encode_utf16()
                    .chain([0])
                    .flat_map(u16::to_le_bytes)
                    .collect(),
            },
            RegistryValue {
                kind: 3,
                bytes: vec![0, 255, 128],
            },
            RegistryValue {
                kind: 4,
                bytes: 42_u32.to_le_bytes().to_vec(),
            },
            RegistryValue {
                kind: 7,
                bytes: "a\0b\0\0"
                    .encode_utf16()
                    .flat_map(u16::to_le_bytes)
                    .collect(),
            },
            RegistryValue {
                kind: 11,
                bytes: u64::MAX.to_le_bytes().to_vec(),
            },
        ];
        for (index, value) in values.iter().enumerate() {
            let value_name = format!("Original{index}");
            registry::write(&context, Hive::HKCU, &root_path, &value_name, value)?;
            let before = tree(&root)?;
            let (original, missing) =
                registry::capture(&context, Hive::HKCU, &root_path, &value_name)?;
            assert_eq!(original, Some(value.clone()));
            assert!(missing.is_empty());
            assert_eq!(tree(&root)?, before);
            registry::write(
                &context,
                Hive::HKCU,
                &root_path,
                &value_name,
                &RegistryValue {
                    kind: 4,
                    bytes: 0_u32.to_le_bytes().to_vec(),
                },
            )?;
            assert_ne!(tree(&root)?, before);
            registry::restore(
                &context,
                Hive::HKCU,
                &root_path,
                &value_name,
                &original,
                &missing,
            )?;
            assert_eq!(tree(&root)?, before);
        }
        let before = tree(&root)?;
        let absent = format!("{root_path}\\A\\B");
        let (original, missing) = registry::capture(&context, Hive::HKCU, &absent, "OnlyTest")?;
        assert!(original.is_none());
        assert_eq!(missing.len(), 2);
        assert_eq!(tree(&root)?, before);
        registry::write(
            &context,
            Hive::HKCU,
            &absent,
            "OnlyTest",
            &RegistryValue {
                kind: 4,
                bytes: 1_u32.to_le_bytes().to_vec(),
            },
        )?;
        registry::restore(
            &context,
            Hive::HKCU,
            &absent,
            "OnlyTest",
            &original,
            &missing,
        )?;
        assert_eq!(tree(&root)?, before);
        Ok(())
    })();
    drop(root);
    users.delete_subkey_all(&absolute)?;
    if !parent_existed {
        let _ = RegKey::predef(HKEY_CURRENT_USER).delete_subkey("Software\\GameTuneTest");
    }
    result
}

#[test]
#[ignore = "writes dedicated HKCU targets; disposable VM opt-in only"]
fn dedicated_hkcu_recovery_keeps_foreign_values_in_created_keys() -> TestResult {
    require_vm()?;
    let _guard = TEST_LOCK.lock().map_err(|_| "native test lock poisoned")?;
    let context = WindowsContext::current_test_user()?;
    let name = unique();
    let path = format!("Software\\GameTuneTest\\{name}");
    let (original, missing) = registry::capture(&context, Hive::HKCU, &path, "OnlyTest")?;
    let value = RegistryValue {
        kind: 3,
        bytes: vec![1],
    };
    registry::write(&context, Hive::HKCU, &path, "OnlyTest", &value)?;
    registry::write(&context, Hive::HKCU, &path, "ForeignTestValue", &value)?;
    assert_eq!(
        registry::restore(&context, Hive::HKCU, &path, "OnlyTest", &original, &missing),
        Err(ExecError::Conflict)
    );
    assert_eq!(
        registry::capture(&context, Hive::HKCU, &path, "ForeignTestValue")?.0,
        Some(value)
    );
    let users = RegKey::predef(HKEY_USERS);
    let absolute = format!("{}\\{path}", context.interactive_sid());
    assert!(path.starts_with("Software\\GameTuneTest\\R"));
    users.delete_subkey_all(&absolute)?;
    for parent in missing.iter().filter(|p| p.as_str() != path).rev() {
        if parent == "Software\\GameTuneTest" {
            let _ = users.delete_subkey(format!("{}\\{parent}", context.interactive_sid()));
        }
    }
    Ok(())
}
