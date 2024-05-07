mod config;
mod model {
    pub mod user;
    mod schema;
}
mod service {
    pub mod database;
}


use config::Config;
use redis::Client;

use crate::service::database::DatabaseService;

fn main() {
    // Initialize configuration
    let config = Config::from_env();

    // Initialize database service
    let mut database_service = DatabaseService::new(&config);

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
        
        // Handle event using the database service
        database_service.handle_event(&payload);
    }
}