#[macro_use] extern crate serenity;
extern crate kankyo;
extern crate chrono;
extern crate typemap;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serenity::model::user::User;
use chrono::prelude::*;
use typemap::Key;
use std::env;
use std::collections::HashMap;

struct Handler;

#[derive(Debug)]
struct Event {
    id: u64,
    date: DateTime<Utc>,
    title: String,
    desc: String,
    link: String,
    participants: Vec<User>,
}

impl EventHandler for Handler {}

struct EventList;

impl Key for EventList {
    type Value = HashMap<u64, Event>;
}

fn main() {
    kankyo::load().expect("Failed to load .env file");
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    {
        let mut data = client.data.lock();
        data.insert::<EventList>(HashMap::default());
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group("Events", |g| g
            .prefix("event")
            .cmd("add", add)
        ));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(add(ctx, message, args) {
    let mut data = ctx.data.lock();
    let events = data.get_mut::<EventList>().unwrap();
    let id = events.len() as u64 + 1;
    let event = Event {
                id,
                date: Utc.datetime_from_str(&args.single::<String>().unwrap(), "%Y-%m-%dT%H:%M").unwrap(),
                title: args.single_quoted::<String>().unwrap(),
                desc: args.single_quoted::<String>().unwrap(),
                link: args.single::<String>().unwrap(),
                participants: vec![message.author.clone()]
            };
    println!("{:?}", event);
    events.insert(id, event);
});