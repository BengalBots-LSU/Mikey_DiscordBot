use serenity::all::{CreateAttachment, CreateEmbed, CreateMessage, EmbedMessageBuilding, MessageBuilder};

use crate::{Data, Error, Context};

/// A command that responds with BengalBot's Links
#[poise::command(
    slash_command,
    prefix_command,
)]
pub async fn links(ctx: Context<'_>) -> Result<(), Error> {

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
    let builder = CreateMessage::new()
        .embed(embed)
        .add_file(CreateAttachment::path("./media/bengalbots.png").await.unwrap());

    ctx.channel_id().send_message(&ctx.http(), builder).await?;
    
    Ok(())
}