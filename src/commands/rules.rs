use serenity::all::{CreateAttachment, CreateEmbed, CreateMessage};

use crate::{Data, Error, Context};

/// A command that responds with BengalBot's Discord server rules.
#[poise::command(
    slash_command,
    prefix_command,
)]
pub async fn rules(ctx: Context<'_>) -> Result<(), Error> {

    let embed = CreateEmbed::new()
        .color(0xeb10ef)
        .title("BengalBots Server Links")
        .image("attachment://DesktopWallpaper.png")
        .fields(vec![
            ("Rule 1", "Don't be Kyle", false),
            ("Rule 2", "Don't turn on drones indoors lol", false),
            ("Rule 3", "Ignore Rule 1", false),
            ("Rule 4", "Don't listen to Adrian", false),
        ]);

    let builder = CreateMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./media/DesktopWallpaper.png").await.unwrap());

    ctx.channel_id().send_message(&ctx.http(), builder).await?;
    
    Ok(())
}