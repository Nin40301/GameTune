#![cfg(windows)]
use ametista_ipc::*;
type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;
#[tokio::test]
async fn real_pipe_authenticates_sid_pid_and_exclusive_instance() -> TestResult {
    let token = nonce()?;
    let sid = native::current_sid()?;
    let mut server = native::server(&token, &sid)?;
    assert!(native::server(&token, &sid).is_err());
    let mut client = native::client(&token)?;
    server.connect().await?;
    pipe::write_frame(&mut client, b"identity probe").await?;
    assert_eq!(pipe::read_frame(&mut server).await?, b"identity probe");
    assert_eq!(native::client_sid(&server)?, sid);
    assert_eq!(native::client_pid(&server)?, std::process::id());
    assert_eq!(native::server_pid(&client)?, std::process::id());
    // Test only IPC. Never launch helper, request UAC, or call execute().
    Ok(())
}
#[tokio::test]
async fn real_pipe_acl_denies_a_different_user() -> TestResult {
    let token = nonce()?;
    let other = "S-1-5-21-1111111111-2222222222-3333333333-9999";
    assert_ne!(native::current_sid()?, other);
    let _server = native::server(&token, other)?;
    let result = native::client(&token);
    assert!(matches!(result,Err(Error::Io(e)) if e.kind()==std::io::ErrorKind::PermissionDenied));
    Ok(())
}
#[tokio::test]
async fn real_pipe_replay_and_malformed_frame_never_reach_execution() -> TestResult {
    let token = nonce()?;
    let sid = native::current_sid()?;
    let catalog = ametista_catalog::load_embedded()?;
    let mut server = native::server(&token, &sid)?;
    let mut client = native::client(&token)?;
    server.connect().await?;
    let data = serde_json::to_vec(&Envelope {
        version: 1,
        catalog_hash: ametista_catalog::embedded_hash(),
        sequence: 1,
        nonce: token.clone(),
        command: Command::History,
    })?;
    let mut gate = Gate::new(sid.clone(), std::process::id(), token)?;
    for index in 0..2 {
        pipe::write_frame(&mut client, &data).await?;
        let bytes = pipe::read_frame(&mut server).await?;
        let result = gate.accept(
            &bytes,
            &native::client_sid(&server)?,
            native::client_pid(&server)?,
            &catalog,
        );
        if index == 0 {
            assert!(result.is_ok())
        } else {
            assert!(matches!(result, Err(Error::Replay)))
        }
    }
    use tokio::io::AsyncWriteExt;
    client.write_u32_le((MAX_FRAME + 1) as u32).await?;
    assert!(matches!(
        pipe::read_frame(&mut server).await,
        Err(Error::InvalidPayload)
    ));
    Ok(())
}
