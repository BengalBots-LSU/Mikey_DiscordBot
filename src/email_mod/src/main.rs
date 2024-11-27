use email_mod::*;
use entities::person::PersonInput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = open_database("dummy_database.db", false, true).await?;
    create_dummy_database(&db).await?;
    let emails = get_emails(&db).await?;
    for (discord_id, email) in emails {
        println!("Discord ID: {}, Email: {}", discord_id, email);
    }

    for x in (0..10) {
        let person_to_set = PersonInput::random_person();
        add_email_to_database(&db, person_to_set).await?;
    }
    Ok(())
}
