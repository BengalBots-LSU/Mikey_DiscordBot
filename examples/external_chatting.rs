use ::serenity::all::{Cache, CreateMessage};
use poise::{serenity_prelude as serenity, Command};
use serenity::prelude::Context as SerenityContext;
use tokio::sync::{mpsc, Mutex};

use std::{env, future::IntoFuture, sync::Arc};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// A command that responds with pong!
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

async fn get_rss_feed() -> Vec<String> {
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

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();
    let shared_ctx: Arc<Mutex<Option<SerenityContext>>> = Arc::new(Mutex::new(None));
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

    let mut client = client.unwrap();

    tokio::join!(
        async { client.start().await.unwrap() },
        loop_outside(shared_ctx)
    );
}
use serenity::model::id::ChannelId;

async fn loop_outside(global_ctx: Arc<Mutex<Option<SerenityContext>>>) {
    loop {
        let builder = CreateMessage::new().content("AAAAAAAAAAAAAAAAAAA");
        let ctx = global_ctx.lock().await;

        if let Some(ctx) = ctx.clone() {
            let message = ChannelId::new(1102864318233595934)
                .send_message(ctx.clone(), builder)
                .await;
            if let Err(why) = message {
                eprintln!("Error sending message: {why:?}");
            };
        } else {
            println!("Waiting...")
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
