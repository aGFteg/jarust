use jarust::jaconfig::JaConfig;
use jarust::jaconfig::TransportType;
use jarust_plugins::audio_bridge::jahandle_ext::AudioBridge;
use jarust_plugins::audio_bridge::messages::CreateRoomMsg;
use jarust_plugins::audio_bridge::messages::JoinRoomMsg;
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
    let (handle, mut event_receiver) = session.attach_audio_bridge().await?;

    let create_room_rsp = handle
        .create_room_with_config(
            CreateRoomMsg {
                secret: Some("superdupersecret".to_string()),
                ..Default::default()
            },
            timeout,
        )
        .await?;

    tracing::info!(
        "Created Room {}, permanent: {}",
        create_room_rsp.room,
        create_room_rsp.permanent
    );

    let _ = handle
        .join_room(
            create_room_rsp.room,
            JoinRoomMsg {
                secret: Some("superdupersecret".to_string()),
                generate_offer: Some(true),
                ..Default::default()
            },
            None,
            timeout,
        )
        .await?;

    if let Some((event, protocol)) = event_receiver.recv().await {
        tracing::info!(
            "Joined Room {}, {:#?}, {:#?}",
            create_room_rsp.room,
            event,
            protocol
        );
    }

    Ok(())
}
