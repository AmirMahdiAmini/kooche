use mongodb::{sync::Client, options::ClientOptions};

use crate::get_env;

pub fn mongodb_connection()->Result<Client,Box<dyn std::error::Error>>{
    let options = ClientOptions::parse(get_env("MONGODB_URL")).expect("wrong url");
    let client = mongodb::sync::Client::with_options(options).expect("couldn't connect to the database");
    Ok(client)
}
pub fn redisdb_connection()->Result<redis::Client,Box<dyn std::error::Error>>{
    let client = redis::Client::open(get_env("REDISDB_URL")).expect("couldn't connect to the redis db");
    Ok(client)
}