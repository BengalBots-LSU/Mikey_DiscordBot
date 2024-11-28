use poise::serenity_prelude as serenity;

mod commands;
use commands::{
    links::links,
    rules::rules,
};

use shuttle_runtime::{SecretStore, Secrets};

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(
        #[Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {

    let token = secrets
        .get("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                links(),
                rules(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("?".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready: &serenity::model::prelude::Ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    Ok(client.into())
}