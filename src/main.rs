#[macro_use] extern crate serenity;
extern crate kankyo;
extern crate typemap;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate uuid;

mod events;
mod util;
mod joke;

use serenity::{client::Client, prelude::EventHandler, framework::standard::StandardFramework};
use std::env;
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use self::util::DbClient;

struct Handler;

impl EventHandler for Handler { }

fn main() {
    // load .env file
    kankyo::load().expect("Failed to load .env file");

    let db_client = DynamoDbClient::new(Region::EuCentral1);

    // Login with a bot token from the environment
    let mut discord_client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = discord_client.data.lock();
        data.insert::<DbClient>(db_client);
    }

    discord_client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group("Events", |g| g
            .prefix("event")
            .command("add", |c| c
                .allowed_roles(vec!["Glorious leader", "pay2win"])
                .cmd(events::add::cmd)
            )
            .cmd("list", events::list::cmd)
            .cmd("join", events::join::cmd)
        ).group("Jokes", |g| g
            .prefix("joke")
            .command("random", |c| c
                .cmd(joke::random::cmd))
        )

        );

    // start listening for events by starting a single shard
    if let Err(why) = discord_client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}