use crate::{
    add_email_to_database, create_dummy_database, entities::person::PersonInput, get_emails,
    open_database,
};
use email_address::EmailAddress;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// A command that responds with pong!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn print_emails(ctx: Context<'_>) -> Result<(), Error> {
    // Open the database connection asynchronously
    let conn = open_database("dummy_database.db", false, true).await?;

    // Fetch emails from the database
    match get_emails(&conn).await {
        Ok(emails) => {
            if emails.is_empty() {
                ctx.say("No emails found in the database.").await?;
            } else {
                // Format the emails into a readable string
                let thing_to_say = emails
                    .into_iter()
                    .map(|(discord_id, email)| format!("{} | {}", discord_id, email.as_str()))
                    .collect::<Vec<_>>()
                    .join("\n");

                ctx.say(thing_to_say).await?;
            }
        }
        Err(e) => {
            // Handle error fetching emails
            ctx.say(format!("Failed to fetch emails: {}", e)).await?;
            return Err(e.into());
        }
    }

    Ok(())
}


//TODO: Change this to run a from String String
#[poise::command(slash_command)]
pub async fn add_email(ctx: Context<'_>, discord_id: String, email: String) -> Result<(), Error> {
    let db = open_database("dummy_database.db", false, true).await?;

    match email.parse::<EmailAddress>() {
        Ok(parsed_email) => {
            // Create a new PersonMapping
            let person = crate::entities::person::PersonInput {
                discord_id: discord_id.clone(),
                email: parsed_email.to_string(),
            };
            add_email_to_database(&db, person).await?;
            ctx.say(format!("Added {} with email {}", discord_id, parsed_email))
                .await?;
        }
        Err(_) => {
            // Respond with error if email is invalid
            ctx.say("Invalid email format. Please provide a valid email.")
                .await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn clean_dummy_data_base(ctx: Context<'_>) -> Result<(), Error> {
    let db = open_database("dummy_database.db", false, true).await?;
    create_dummy_database(&db).await?;
    let emails = get_emails(&db).await?;
    for (discord_id, email) in emails {
        println!("Discord ID: {}, Email: {}", discord_id, email);
    }

    for _ in 0..10 {
        let person_to_set = PersonInput::random_person();
        add_email_to_database(&db, person_to_set).await?;
    }
    ctx.say("done").await?;
    Ok(())
}

/// A command that responds with pong!
#[poise::command(slash_command)]
async fn ping_bet_again(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}
