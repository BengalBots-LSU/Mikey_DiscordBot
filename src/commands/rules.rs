use serenity::all::{CreateAttachment, CreateEmbed, CreateMessage, EmbedMessageBuilding, MessageBuilder};

use crate::{Data, Error, Context};

/// A command that responds with BengalBot's Discord server rules.
#[poise::command(
    slash_command,
    prefix_command,
)]
pub async fn rules(ctx: Context<'_>) -> Result<(), Error> {

    let DiscordTOS = MessageBuilder::new()
        .push("Follow Discord's ")
        .push_named_link("Terms of Service", "https://discord.com/terms").build();

    let LSUCoC = MessageBuilder::new()
        .push("This is a Discord Server for an LSU club, so follow LSU's ")
        .push_named_link("Code of Conduct", "https://www.lsu.edu/saa/students/codeofconduct.php").build();

    let embed = CreateEmbed::new()
        .color(0xeb10ef)
        .title("BengalBots Server Rules")
        .image("attachment://DesktopWallpaper.png")
        .fields(vec![
            ("Rule 1️⃣", "Be Respectful to Others!", false),
            ("Rule 2️⃣", "Don't Spam Ping", false),
            ("Rule 3️⃣", &DiscordTOS, false),
            ("Rule 4️⃣", &LSUCoC, false),
        ]);

    let builder = CreateMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./media/DesktopWallpaper.png").await.unwrap());

    ctx.channel_id().send_message(&ctx.http(), builder).await?;
    
    Ok(())
}