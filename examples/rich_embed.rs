use std::env;

/*
    This example demonstrates how to create an embed message from the bot using Serenity's message builder.
    The command to run this example is `!hello`. {I didn't feel like reformatting it as a slash command - Adrian}
*/

use serenity::all::{EmbedMessageBuilding, MessageBuilder};
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!links" {
            // The create message builder allows you to easily create embeds and messages using a
            // builder syntax.
            // This example will create a message that says "Hello, World!", with an embed that has
            // a title, description, an image, three fields, and a footer.
            let tigerlink = MessageBuilder::new()
                    .push_named_link("**TigerLink**", "https://tigerlink.lsu.edu/BengalBots/club_signup").build();
            
            let instagram = MessageBuilder::new()
                    .push_named_link("**Instagram**", "https://www.instagram.com/bengal.bots/").build();

            let redbubble = MessageBuilder::new()
                    .push_named_link("**Redbubble**", "https://www.redbubble.com/people/BengalBots/shop?").build();

            let github = MessageBuilder::new()
                    .push_named_link("**GitHub**", "https://github.com/BengalBots-LSU").build();

            let linkedin = MessageBuilder::new()
                    .push_named_link("**LinkedIn**", "https://www.linkedin.com/company/bengalbots-lsu/").build();

            let fusion360 = MessageBuilder::new()
                    .push_named_link("**Fusion360**", "https://mylsu1602.autodesk360.com/").build();

            let embed = CreateEmbed::new()
                .color(0x461d7c)
                .title("BengalBots Links")
                .image("attachment://bengalbots.png")
                .fields(vec![
                    ("", tigerlink, true),
                    ("", instagram, true),
                    ("", redbubble, true),
                ])
                .fields(vec![
                    ("", "", true),
                    ("", "", true),
                    ("", "", true),
                ])
                .fields(vec![
                    ("", github, true),
                    ("", linkedin, true),
                    ("", fusion360, true),
                ]);
                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
            let builder = CreateMessage::new()
                .embed(embed)
                .add_file(CreateAttachment::path("./media/bengalbots.png").await.unwrap());
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}