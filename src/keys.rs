use serenity::{
    client::bridge::gateway::ShardManager,
};

use serenity::prelude::*;

use std::{
    sync::Arc,
    collections::HashMap,
};

use reqwest;

use chrono::{DateTime, Utc};

// A container type is created for inserting into the Client's `data`, which
// allows for data to be accessible across all events and framework commands, or
// anywhere else that has a copy of the `data` Arc.

pub struct SerenityShardManager;
impl TypeMapKey for SerenityShardManager {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Uptime;
impl TypeMapKey for Uptime {
    type Value = DateTime<Utc>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

pub struct Reqwest;
impl TypeMapKey for Reqwest {
    type Value = Arc<reqwest::blocking::Client>; 
}