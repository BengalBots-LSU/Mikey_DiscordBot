use anyhow::{Context, Result};
use email_address::EmailAddress;
use entities::person::PersonInput;
use sea_orm::sea_query::*;
use sea_orm::*;
use std::fs;
pub mod entities;
pub mod email_commands;

pub async fn open_database(
    database_name: &str,
    is_encrypted: bool,
    create_if_missing: bool,
) -> Result<DatabaseConnection> {
    if create_if_missing {
        if let Some(parent_dir) = std::path::Path::new(database_name).parent() {
            fs::create_dir_all(parent_dir).with_context(|| {
                format!(
                    "Failed to create parent directories for '{}'",
                    database_name
                )
            })?;
        }

        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(database_name)
            .with_context(|| {
                format!("Failed to create or open database file '{}'", database_name)
            })?;
    }

    let conn_string = if is_encrypted {
        format!(
            "sqlite://{}?mode=memory&key=my_secure_password",
            database_name
        )
    } else {
        format!("sqlite://{}", database_name)
    };

    Database::connect(&conn_string)
        .await
        .with_context(|| format!("Failed to connect to database '{}'", database_name))
}

pub async fn create_dummy_database(db: &DatabaseConnection) -> Result<()> {
    let schema = Schema::new(db.get_database_backend());

    let create_table_stmt: TableCreateStatement =
        schema.create_table_from_entity(entities::person::Entity);

    let sql = create_table_stmt.to_string(SqliteQueryBuilder);
    db.execute(Statement::from_string(DbBackend::Sqlite, sql))
        .await
        .with_context(|| "Failed to execute SQL for creating the 'people' table")?;

    Ok(())
}

pub async fn get_emails(db: &DatabaseConnection) -> Result<Vec<(String, EmailAddress)>> {
    let people = entities::person::Entity::find()
        .all(db)
        .await
        .with_context(|| "Failed to query all rows from the 'people' table")?;

    let results = people
        .into_iter()
        .filter_map(|person| {
            person
                .email
                .parse::<EmailAddress>()
                .ok()
                .map(|email| (person.discord_id, email))
        })
        .collect::<Vec<_>>();

    Ok(results)
}

pub async fn add_email_to_database(db: &DatabaseConnection, person: PersonInput) -> Result<()> {
    let new_person = entities::person::ActiveModel {
        id: NotSet, // Let the database auto-generate the ID
        discord_id: Set(person.discord_id.clone()),
        email: Set(person.email.clone()),
    };

    new_person.insert(db).await.with_context(|| {
        format!(
            "Failed to insert person with discord_id '{}'",
            person.discord_id
        )
    })?;

    println!(
        "Added email: {} for discord_id: {}",
        person.email, person.discord_id
    );

    Ok(())
}
