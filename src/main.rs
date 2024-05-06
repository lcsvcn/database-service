
mod config;
mod model {
    pub mod user;
    mod schema;
}

use diesel::{Connection, PgConnection};
use redis::Client;
use serde_json::Value;
use crate::config::Config;
use crate::model::user::User;

fn main() {
    // Initialize configuration
    let config = Config::from_env();

    // Initialize database connection
    let mut connection = PgConnection::establish(&config.database_url)
    .expect("Error connecting to database");

    // Initialize Redis connection
    let redis_client = Client::open(config.redis_url.clone()).expect("Error connecting to Redis");
    let mut redis_connection = redis_client.get_connection().expect("Error connecting to Redis");
    
    // Initialize Redis pubsub
    let mut pubsub = redis_connection.as_pubsub();
    pubsub.subscribe("events").expect("Error subscribing to Redis channel");

    // Loop to continuously listen for new events
    loop {
        let message = pubsub.get_message().expect("Error receiving message from Redis");
        let payload: String = message.get_payload().unwrap();
        let event_payload: serde_json::Value =
            serde_json::from_str(&payload).expect("Error parsing event payload");

        // Check event type and handle accordingly
        if let Some(event_type) = event_payload.get("event_type").and_then(|t| t.as_str()) {
            match event_type {
                "create_user" => {
                    if let (Some(id), Some(username), Some(email)) = (
                        event_payload.get("id").and_then(Value::as_i64), 
                        event_payload.get("username").and_then(Value::as_str),
                        event_payload.get("email").and_then(Value::as_str),
                    ) {
                        // Clone strings to ensure ownership
                        let username = username.to_string();
                        let email = email.to_string();
            
                        // Create User instance
                        let user_data = User {
                            id: id as i32,
                            username,
                            email,
                            // Other user fields...
                        };
                        
                        // Register the user
                        User::register(&mut connection, &user_data);
                    } else {
                        println!("Missing or invalid user data in event payload");
                    }
                }
                // Add other event types handling here if needed
                _ => {
                    println!("Unhandled event type: {}", event_type);
                }
            }    
        }
    }
}
