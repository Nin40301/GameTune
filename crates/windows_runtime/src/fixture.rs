//! Dedicated Windows fixtures. Compiled only with vm-tests; never used by a release.
use crate::{
    WindowsContext, registry, startup,
    transport::{self, Script},
};
use ametista_core::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub struct VmFixture {
    pub context: WindowsContext,
    pub catalog: Catalog,
    pub ids: Vec<String>,
    nonce: String,
    directory: PathBuf,
}
impl VmFixture {
    pub fn create() -> ExecResult<Self> {
        if std::env::var("AMETISTA_DISPOSABLE_VM").as_deref() != Ok("I_HAVE_A_SNAPSHOT") {
            return Err(ExecError::Protected);
        }
        let context = WindowsContext::detect()?;
        context.ensure_elevated()?;
        let nonce = format!(
            "V{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(crate::failed)?
                .as_nanos()
        );
        let directory: String = transport::run(
            Script::Fixture,
            &json!({"operation":"setup","nonce":nonce,"sid":context.interactive_sid()}),
        )?;
        let base = ametista_catalog::load_embedded().map_err(crate::failed)?;
        let name = format!("AmetistaTest_{nonce}");
        let digest = format!("{:x}", Sha256::digest(nonce.as_bytes()));
        let guid = format!(
            "47544e54-{}-{}-{}-{}",
            &digest[0..4],
            &digest[4..8],
            &digest[8..12],
            &digest[12..24]
        );
        let mut items = Vec::new();
        for (kind, template, target) in [
            (
                "registry",
                "reg.cortana",
                Target::Registry {
                    hive: Hive::HKCU,
                    path: format!("Software\\AmetistaTest\\{nonce}\\Settings"),
                    name: "OnlyTest".into(),
                    value: 1,
                },
            ),
            (
                "service",
                "svc.diagtrack",
                Target::Service {
                    name: name.clone(),
                    mode: ServiceMode::Disabled,
                },
            ),
            (
                "task",
                "task.consolidator",
                Target::ScheduledTask {
                    path: "\\AmetistaTest\\".into(),
                    name: name.clone(),
                },
            ),
            ("power", "power.high_performance", Target::Power { guid }),
        ] {
            let mut item = base.get(template).map_err(crate::failed)?.clone();
            item.id = format!("test.{kind}.{nonce}");
            item.target = target;
            item.min_build = 19045;
            item.max_build = None;
            item.supported_editions = vec![context.machine().edition];
            item.verified = false;
            item.automatic = false;
            items.push(item);
        }
        let mut startup_item = base.get("startup.hkcu").map_err(crate::failed)?.clone();
        startup_item.id = format!("test.startup.{nonce}");
        let (value, _) = registry::capture(&context, Hive::HKCU, registry::RUN, &name)?;
        let startup_item = startup::test_item(
            &context,
            startup_item,
            name,
            &value.ok_or(ExecError::NotFound)?,
        );
        items.push(startup_item);
        let ids = items.iter().map(|i| i.id.clone()).collect();
        let catalog = Catalog::try_new(items).map_err(crate::failed)?;
        Ok(Self {
            context,
            catalog,
            ids,
            nonce,
            directory: PathBuf::from(directory),
        })
    }
    pub fn directory(&self) -> &std::path::Path {
        &self.directory
    }
    pub fn request(&self) -> Request {
        Request {
            item_ids: self.ids.clone(),
            unverified_confirmations: self.ids.iter().cloned().collect(),
            ..Request::default()
        }
    }
    pub fn snapshot(&self) -> ExecResult<Value> {
        transport::run(
            Script::Fixture,
            &json!({"operation":"snapshot","nonce":self.nonce,"sid":self.context.interactive_sid()}),
        )
    }
    pub fn cleanup(&self) -> ExecResult<()> {
        let success: bool = transport::run(
            Script::Fixture,
            &json!({"operation":"cleanup","nonce":self.nonce,"sid":self.context.interactive_sid()}),
        )?;
        if success {
            Ok(())
        } else {
            Err(ExecError::Failed("fixture cleanup incomplete".into()))
        }
    }
    pub fn test_protected_store(&self) -> ExecResult<Value> {
        transport::run(
            Script::Storage,
            &json!({"sid":self.context.interactive_sid(),"testRoot":true}),
        )
    }
    pub fn cleanup_test_store(&self) -> ExecResult<bool> {
        transport::run(
            Script::Storage,
            &json!({"sid":self.context.interactive_sid(),"testRoot":true,"cleanupTest":true}),
        )
    }
}
