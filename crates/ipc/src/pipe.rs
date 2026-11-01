use crate::*;
use gametune_windows_runtime::WindowsSession;
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
const MAX_REPLY: usize = 4 * 1024 * 1024;
pub async fn read_frame(stream: &mut (impl AsyncRead + Unpin)) -> Result<Vec<u8>> {
    let size = tokio::time::timeout(Duration::from_secs(15), stream.read_u32_le())
        .await
        .map_err(|_| Error::Timeout)?? as usize;
    if size == 0 || size > MAX_FRAME {
        return Err(Error::InvalidPayload);
    }
    let mut bytes = vec![0; size];
    tokio::time::timeout(Duration::from_secs(15), stream.read_exact(&mut bytes))
        .await
        .map_err(|_| Error::Timeout)??;
    Ok(bytes)
}
pub async fn write_frame(stream: &mut (impl AsyncWrite + Unpin), bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() || bytes.len() > MAX_FRAME {
        return Err(Error::InvalidPayload);
    }
    stream.write_u32_le(bytes.len() as u32).await?;
    stream.write_all(bytes).await?;
    stream.flush().await?;
    Ok(())
}
pub async fn request(command: Command) -> Result<Reply> {
    let token = nonce()?;
    // RunAs may display UAC; this function is called only after consuming a review.
    let helper = tokio::task::spawn_blocking({
        let token = token.clone();
        move || native::launch_helper(&token)
    })
    .await
    .map_err(|_| Error::Operation)??;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    let mut client = loop {
        match native::client(&token) {
            Ok(client) => break client,
            Err(error) => {
                if tokio::time::Instant::now() >= deadline {
                    return Err(error);
                }
                tokio::time::sleep(Duration::from_millis(40)).await;
            }
        }
    };
    if native::server_pid(&client)? != helper.pid {
        return Err(Error::Identity);
    }
    let bytes = serde_json::to_vec(&Envelope {
        version: 1,
        catalog_hash: gametune_catalog::embedded_hash(),
        nonce: token,
        sequence: 1,
        command,
    })?;
    write_frame(&mut client, &bytes).await?;
    // Bounded transaction deadline; no retry after a timeout or uncertain write.
    let reply = tokio::time::timeout(Duration::from_secs(1200), async {
        // Header can take the duration of a batch, unlike the helper's inbound header.
        let size = client.read_u32_le().await? as usize;
        if size == 0 || size > MAX_REPLY {
            return Err(Error::InvalidPayload);
        }
        let mut bytes = vec![0; size];
        client.read_exact(&mut bytes).await?;
        Ok::<_, Error>(serde_json::from_slice::<Reply>(&bytes)?)
    })
    .await
    .map_err(|_| Error::Timeout)??;
    let _ = client.write_all(&[0]).await;
    Ok(reply)
}
pub async fn serve(token: String, parent: u32) -> Result<()> {
    let mut session = WindowsSession::connect().map_err(|_| Error::Windows)?;
    session
        .context
        .ensure_elevated()
        .map_err(|_| Error::Identity)?;
    let sid = session.context.interactive_sid().to_string();
    let mut pipe = native::server(&token, &sid)?;
    tokio::time::timeout(Duration::from_secs(30), pipe.connect())
        .await
        .map_err(|_| Error::Timeout)??;
    let bytes = read_frame(&mut pipe).await?;
    let peer_pid = native::client_pid(&pipe)?;
    let peer_sid = native::client_sid(&pipe)?;
    let command = Gate::new(sid, parent, token.clone())?.accept(
        &bytes,
        &peer_sid,
        peer_pid,
        session.catalog(),
    )?;
    if let Command::TemporarySession { ids, .. } = command {
        return crate::game_pipe::serve(session, pipe, token, ids).await;
    }
    let reply = execute(&mut session, &command);
    let bytes = serde_json::to_vec(&reply)?;
    if bytes.len() > MAX_REPLY {
        return Err(Error::InvalidPayload);
    }
    tokio::time::timeout(Duration::from_secs(15), async {
        pipe.write_u32_le(bytes.len() as u32).await?;
        pipe.write_all(&bytes).await?;
        pipe.flush().await
    })
    .await
    .map_err(|_| Error::Timeout)??;
    // Keep server handle alive until acknowledgement; a second command is never executed.
    let mut ack = [0u8; 1];
    let _ = tokio::time::timeout(Duration::from_secs(3), pipe.read_exact(&mut ack)).await;
    Ok(())
}
