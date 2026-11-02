//! Authenticated helper stays alive only for one active game session. EOF/timeout
//! always attempts WAL rollback; a helper/OS crash leaves recovery to next launch.
use crate::*;
use ametista_core::Outcome;
use ametista_windows_runtime::WindowsSession;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::windows::named_pipe::{NamedPipeClient, NamedPipeServer},
};
async fn reply(stream: &mut NamedPipeServer, reply: Reply) -> Result<()> {
    let bytes = serde_json::to_vec(&reply)?;
    tokio::time::timeout(Duration::from_secs(10), async {
        stream.write_u32_le(bytes.len() as u32).await?;
        stream.write_all(&bytes).await?;
        stream.flush().await
    })
    .await
    .map_err(|_| Error::Timeout)??;
    Ok(())
}
fn good(value: serde_json::Value) -> Reply {
    Reply {
        ok: true,
        result: value,
        error: None,
    }
}
pub async fn serve(
    mut session: WindowsSession,
    mut stream: NamedPipeServer,
    token: String,
    allowed: Vec<String>,
) -> Result<()> {
    if let Err(ametista_core::ExecError::Failed(message)) = session.context.ensure_elevated()
        && message == "legacy_data"
    {
        reply(
            &mut stream,
            Reply {
                ok: false,
                result: serde_json::Value::Null,
                error: Some("legacy_data".into()),
            },
        )
        .await?;
        return Ok(());
    }
    let store = session
        .context
        .protected_game_store()
        .map_err(|_| Error::Operation)?;
    let _ = store.log(ametista_windows_runtime::logging::Event::GameSession, true);
    let mut journal = store.game_journal().map_err(|_| Error::Operation)?;
    let recovered = session
        .reconcile_games(&mut journal, &[])
        .map_err(|_| Error::Operation)?;
    if recovered.items.iter().any(|i| i.outcome == Outcome::Failed) {
        return Err(Error::Operation);
    }
    reply(&mut stream, good(serde_json::json!({"ready":true}))).await?;
    let mut gate = GameGate::new(token, allowed)?;
    let mut checkpoint = false;
    let result: Result<()> = async {
        loop {
            let bytes =
                tokio::time::timeout(Duration::from_secs(10), pipe::read_frame(&mut stream))
                    .await
                    .map_err(|_| Error::Timeout)??;
            match gate.accept(&bytes)? {
                GameStep::Set { ids } => {
                    if !ids.is_empty() && !checkpoint {
                        session
                            .context
                            .create_restore_point()
                            .map_err(|_| Error::Operation)?;
                        checkpoint = true;
                    }
                    let report = session
                        .reconcile_games(&mut journal, &ids)
                        .map_err(|_| Error::Operation)?;
                    if report.items.iter().any(|i| i.outcome == Outcome::Failed) {
                        return Err(Error::Operation);
                    }
                    reply(&mut stream, good(serde_json::to_value(report)?)).await?;
                }
                GameStep::Ping {} => reply(&mut stream, good(serde_json::Value::Null)).await?,
                GameStep::Finish {} => break,
            }
        }
        Ok(())
    }
    .await;
    // Independent of the protocol result: unauthorized/replayed/malformed frames
    // close this session AND restore, instead of leaving a privileged session open.
    let rollback = session
        .reconcile_games(&mut journal, &[])
        .map_err(|_| Error::Operation)?;
    if rollback.items.iter().any(|i| i.outcome == Outcome::Failed) {
        return Err(Error::Operation);
    }
    if result.is_ok() {
        reply(&mut stream, good(serde_json::json!({"restored":true}))).await?;
    }
    result
}
pub struct Client {
    stream: NamedPipeClient,
    token: String,
    sequence: u64,
    _helper: native::Helper,
}
impl Client {
    pub fn helper_pid(&self) -> u32 {
        self._helper.pid
    }
    pub async fn open(ids: Vec<String>) -> Result<Self> {
        let token = nonce()?;
        let helper = tokio::task::spawn_blocking({
            let token = token.clone();
            move || native::launch_helper(&token)
        })
        .await
        .map_err(|_| Error::Operation)??;
        let until = tokio::time::Instant::now() + Duration::from_secs(30);
        let mut stream = loop {
            match native::client(&token) {
                Ok(c) => break c,
                Err(e) => {
                    if tokio::time::Instant::now() >= until {
                        return Err(e);
                    }
                    tokio::time::sleep(Duration::from_millis(40)).await;
                }
            }
        };
        if native::server_pid(&stream)? != helper.pid {
            return Err(Error::Identity);
        }
        pipe::write_frame(
            &mut stream,
            &serde_json::to_vec(&Envelope {
                version: 1,
                catalog_hash: ametista_catalog::embedded_hash(),
                nonce: token.clone(),
                sequence: 1,
                command: Command::TemporarySession { ids, confirm: true },
            })?,
        )
        .await?;
        let mut client = Self {
            stream,
            token,
            sequence: 2,
            _helper: helper,
        };
        client.receive().await?;
        Ok(client)
    }
    async fn receive(&mut self) -> Result<Reply> {
        let received = tokio::time::timeout(Duration::from_secs(180), async {
            let length = self.stream.read_u32_le().await? as usize;
            if length == 0 || length > 4 * 1024 * 1024 {
                return Err(Error::InvalidPayload);
            }
            let mut bytes = vec![0; length];
            self.stream.read_exact(&mut bytes).await?;
            Ok(serde_json::from_slice::<Reply>(&bytes)?)
        })
        .await
        .map_err(|_| Error::Timeout)??;
        if !received.ok {
            return Err(Error::Operation);
        }
        Ok(received)
    }
    pub async fn step(&mut self, step: GameStep) -> Result<Reply> {
        let frame = GameFrame {
            nonce: self.token.clone(),
            sequence: self.sequence,
            step,
        };
        self.sequence += 1;
        pipe::write_frame(&mut self.stream, &serde_json::to_vec(&frame)?).await?;
        self.receive().await
    }
}
