use ::serenity::all::CreateMessage;
use mikey::{
    create_client, send_message_to_channel, start_discord_client, SendDataToDiscord,
    SendToDiscordMpsc,
};
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create the client and shared context.
    let (client, senerity_context) = create_client().await;

    let (message_buffer_send, message_buffer_rec) = mpsc::channel::<SendDataToDiscord>(50);

    // Use tokio::join! to run multiple async tasks concurrently.
    let (_discord_result, _message_result, _scream_result) = tokio::join!(
        async { start_discord_client(client.unwrap()).await },
        async { send_message_to_channel(senerity_context.clone(), message_buffer_rec).await },
        async { scream_from_outside(message_buffer_send).await }
    );
}

async fn scream_from_outside(send: SendToDiscordMpsc) {
    loop {
        let send_message = SendDataToDiscord {
            message: CreateMessage::new().content("AAAAAAAAAAAAAAAAAAA"),
            channel: ChannelId::new(1102864318233595934),
        };
        send.send(send_message).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
