/*
This example demonstrates how to create a bot with slash commands using Poise.

    Namely, it creates the '/age' and '/ping' commands, 
    which display the account creation date of a user and respond with 'Pong!' respectively.
*/

use poise::serenity_prelude as serenity;

use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// A command that responds with pong!
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Load the token from the environment using the dotenvy crate
    let _ = dotenvy::dotenv();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = serenity::GatewayIntents::non_privileged();

    // Create a new instance of the Client, logging in as a bot.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            // Register the commands
            commands: vec![age(), ping()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Start the bot by passing the token, intents, and framework to the ClientBuilder struct
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}