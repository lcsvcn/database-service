use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde_json::Value;

use crate::model::user::User;
use crate::config::Config;


pub struct DatabaseService {
    pub connection: PgConnection,
}

impl DatabaseService {
    pub fn new(config: &Config) -> Self {
        // Initialize database connection
        let mut connection = PgConnection::establish(&config.database_url)
            .expect("Error connecting to database");

        // Run migrations
        Self::run_migration(&mut connection);

        DatabaseService {
            connection,
        }
    }

    fn run_migration(connection: &mut PgConnection) {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn handle_event(&mut self, message: &str) {
        let event_payload: serde_json::Value =
            serde_json::from_str(message).expect("Error parsing event payload");

        if let Some(event_type) = event_payload.get("event_type").and_then(|t| t.as_str()) {
            match event_type {
                "create_user" => self.handle_create_user(&event_payload),
                _ => println!("Unhandled event type: {}", event_type),
            }
        }
    }

    fn handle_create_user(&mut self, event_payload: &serde_json::Value) {
        if let (Some(id), Some(username), Some(email)) = (
            event_payload.get("id").and_then(Value::as_i64),
            event_payload.get("username").and_then(Value::as_str),
            event_payload.get("email").and_then(Value::as_str),
        ) {
            let username = username.to_string();
            let email = email.to_string();

            let user_data = User {
                id: id as i32,
                username,
                email,
            };

            User::register(&mut self.connection, &user_data);
            
        } else {
            println!("Missing or invalid user data in event payload");
        }
    }
}
