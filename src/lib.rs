use ::serenity::all::CreateMessage;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use serenity::prelude::Context as SerenityContext;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
pub type SendToDiscordMpsc = tokio::sync::mpsc::Sender<SendDataToDiscord>;

pub struct SendDataToDiscord {
    pub message: CreateMessage,
    pub channel: ChannelId,
}
pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// A command that responds with pong!
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

pub async fn get_rss_feed() -> Vec<String> {
    let content = reqwest::get("http://rss.cnn.com/rss/cnn_topstories.rss")
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = rss::Channel::read_from(&content[..]).unwrap();
    channel
        .items
        .into_iter()
        .map(|x| x.title().unwrap().to_string())
        .collect()
}

#[poise::command(slash_command)]
async fn cnn_test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(get_rss_feed().await.last_chunk::<3>().unwrap().join(" "))
        .await?;
    Ok(())
}

pub async fn start_discord_client(mut client: serenity::Client) {
    client.start().await.unwrap();
}

pub async fn create_client() -> (
    Result<serenity::Client, serenity::Error>,
    Arc<serenity::prelude::Mutex<Option<serenity::prelude::Context>>>,
) {
    let _ = dotenvy::dotenv();
    let shared_ctx: Arc<Mutex<Option<SerenityContext>>> = Arc::new(Mutex::new(None));

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), cnn_test()],
            ..Default::default()
        })
        .setup({
            let shared_ctx = Arc::clone(&shared_ctx);
            move |ctx, _ready, framework| {
                Box::pin(async move {
                    // Store `serenity::Context` in `shared_ctx`
                    {
                        let mut ctx_lock = shared_ctx.lock().await;
                        *ctx_lock = Some(ctx.clone());
                    }
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                    Ok(Data {})
                })
            }
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    (client, shared_ctx)
}

pub async fn send_message_to_channel(
    global_ctx: Arc<Mutex<Option<SerenityContext>>>,
    mut rec: tokio::sync::mpsc::Receiver<SendDataToDiscord>,
) {
    loop {
        let dirty = rec.recv().await.unwrap();

        let channel_id = dirty.channel;

        let builder = dirty.message;
        let ctx = global_ctx.lock().await;

        if let Some(ctx) = ctx.clone() {
            let message = channel_id.send_message(ctx.clone(), builder).await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
            };
        } else {
            println!("Waiting...")
        }
    }
}
