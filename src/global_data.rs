use std::{
    sync::Arc,
    collections::HashSet,
    time::Instant,
};

use tokio::sync::{
    RwLock,
    Mutex,
};

use serenity::{
    prelude::TypeMapKey,
    client::bridge::{
        gateway::ShardManager,
        voice::ClientVoiceManager,
    },

    model::id::GuildId,
};

use sqlx::PgPool; //PostgreSQL Pool Structure
use darkredis::ConnectionPool as RedisPool;

// Defining the structures to be used for "global" data
// this data is not really global, it's just shared with Context.data
pub struct ShardManagerContainer; // Shard manager to use for the latency
pub struct DatabasePool; // A pool of connections to the database.
pub struct CachePool; // The connection to the redis cache database.
pub struct Tokens; // For the configuration found on "config.toml"
pub struct AnnoyedChannels; // This is a HashSet of all the channels the bot is allowed to be annoyed on.
pub struct Uptime; // This is for the startup time of the bot.

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

impl TypeMapKey for CachePool {
    type Value = RedisPool;
}

impl TypeMapKey for Tokens {
    type Value = Arc<ConfigurationData>;
}

impl TypeMapKey for AnnoyedChannels {
    type Value = Arc<RwLock<HashSet<u64>>>;
}

impl TypeMapKey for Uptime {
    type Value = Arc<Instant>;
}

