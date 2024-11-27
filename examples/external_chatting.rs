use ::serenity::all::CreateMessage;
use mikey::*;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Create the client and shared context.
    let (client, senerity_context) = create_client(vec![ping(), cnn_test()]).await;
    let (message_buffer_send, message_buffer_rec) = mpsc::channel::<SendDataToDiscord>(50);

    // Use tokio::join! to run multiple async tasks concurrently.
    let _ = tokio::join!(
        async { start_discord_client(client.unwrap()).await },
        async { send_message_to_channel(senerity_context.clone(), message_buffer_rec).await },
        // async { scream_from_outside(message_buffer_send).await }
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
