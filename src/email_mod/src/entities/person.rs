use anyhow::{Context, Result};
use distributions::Alphanumeric;
use email_address::{EmailAddress, Options};
use rand::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "people")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, // Auto-incrementing integer primary key
    #[sea_orm(unique)]
    pub discord_id: String, // Unique column but not a primary key
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations for this entity") // Use this when there are no relations
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Input struct for creating a person with validation
pub struct PersonInput {
    pub discord_id: String,
    pub email: String,
}

impl PersonInput {
    /// Validates and converts `PersonInput` into a `Model`
    pub fn validate(self) -> Result<Model> {
        let options = Options::default();
        let email = EmailAddress::parse_with_options(&self.email, options)
            .with_context(|| format!("Invalid email address: {}", self.email))?;
        Ok(Model {
            id: 0, // Default ID, will be auto-generated
            discord_id: self.discord_id,
            email: email.as_str().to_string(),
        })
    }
    pub fn random_person() -> Self {
        let discord_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let email = format!(
            "{}@example.com",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .map(char::from)
                .collect::<String>()
        );

        PersonInput { discord_id, email }
    }
}
