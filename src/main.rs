//! This is a discord bot made with `serenity.rs`
//! This was designed to serve the same functions as the
//! Discord.py Carberretta Bot.
//! This is a learning project, first discord bot in Rust
//! so bare with the most likely stupid and terrible code


mod commands; // Load the commands module
mod utils; // Load the utils module
mod global_data; // Load global data

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager, 
            framework::{
                        StandardFramework,
                        standard::macros::group,
            },
    client::ClientBuilder,
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};


use commands::{
    math::*,
    meta::*,
    owner::*,
};

use serde::Deserialize;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
         info!("Resumed");
    }
}

#[group]
#[commands(multiply, ping, quit, about, botinfo)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");


    let token = env::var("TOKEN")
        .expect("Expected a token in the environment");

    let version = env::var("VERSION")
        .expect("Expected a version in the environment");

    let prefix = env::var("PREFIX")
        .expect("Expected a prefix in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    
    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
                   .owners(owners)
                   .prefix(&prefix))
        .group(&GENERAL_GROUP);

    let mut client = ClientBuilder::new(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}