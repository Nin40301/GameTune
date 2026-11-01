use crate::{Error, Protocol, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    checksum: String,
    protocol: Protocol,
}
pub struct Store {
    path: PathBuf,
    _lock: File,
}
impl Store {
    pub fn open(directory: &Path) -> Result<Self> {
        fs::create_dir_all(directory).map_err(|_| Error::Storage)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join("benchmark.lock"))
            .map_err(|_| Error::Storage)?;
        lock.try_lock().map_err(|_| Error::Storage)?;
        Ok(Self {
            path: directory.join("benchmark.json"),
            _lock: lock,
        })
    }
    pub fn load(&self) -> Result<Protocol> {
        let file = match File::open(&self.path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Protocol::new()),
            Err(_) => return Err(Error::Storage),
        };
        let mut bytes = Vec::new();
        file.take(4 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::Storage)?;
        if bytes.len() > 4 * 1024 * 1024 {
            return Err(Error::Storage);
        }
        let envelope: Envelope = serde_json::from_slice(&bytes).map_err(|_| Error::Storage)?;
        let encoded = serde_json::to_vec(&envelope.protocol).map_err(|_| Error::Storage)?;
        if envelope.protocol.version != 1
            || format!("{:x}", Sha256::digest(encoded)) != envelope.checksum
        {
            return Err(Error::Storage);
        }
        Ok(envelope.protocol)
    }
    pub fn save(&self, protocol: &Protocol) -> Result<()> {
        let bytes = serde_json::to_vec(protocol).map_err(|_| Error::Storage)?;
        let envelope = Envelope {
            checksum: format!("{:x}", Sha256::digest(bytes)),
            protocol: protocol.clone(),
        };
        let mut temp = tempfile::NamedTempFile::new_in(self.path.parent().ok_or(Error::Storage)?)
            .map_err(|_| Error::Storage)?;
        serde_json::to_writer(&mut temp, &envelope).map_err(|_| Error::Storage)?;
        temp.flush().map_err(|_| Error::Storage)?;
        temp.as_file().sync_all().map_err(|_| Error::Storage)?;
        temp.persist(&self.path).map_err(|_| Error::Storage)?;
        Ok(())
    }
}
