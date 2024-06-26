use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::messages::CreateRoomMsg;
use jarust_plugins::audio_bridge::messages::DestroyRoomMsg;
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("jarust=trace".parse()?))
        .init();
    let timeout = std::time::Duration::from_secs(10);

    let config = JaConfig::builder().url("ws://localhost:8188/ws").build();
    let mut connection = jarust::connect(config, TransportType::Ws).await?;
    let session = connection.create(10).await?;
    let (handle, ..) = session.attach_audio_bridge().await?;

    let _ = handle
        .create_room_with_config(
            CreateRoomMsg {
                room: Some(4321),
                description: Some("A nice description".to_string()),
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    let destroy_room_rsp = handle
        .destroy_room(
            4321,
            DestroyRoomMsg {
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    tracing::info!(
        "Detroyed Room {}, permanent: {}",
        destroy_room_rsp.room,
        destroy_room_rsp.permanent
    );

    Ok(())
}
