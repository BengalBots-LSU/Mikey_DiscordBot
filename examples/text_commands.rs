/*
This example demonstrates how to create a bot with text commands using Serenity.

    Namely, it creates the '!ping' and '!karma_test' commands, 
    which respond with 'Pong!' and 'This is a responce to karma's testing' respectively.
*/

use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            "!karma_test" => {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "This is a responce to karma's testing")
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let _ = dotenvy::dotenv();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}