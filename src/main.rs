#[macro_use] extern crate serenity;
extern crate kankyo;
extern crate chrono;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serenity::model::user::User;
use serenity::model::channel::Message;
use chrono::prelude::*;
use std::env;

struct Handler;

#[derive(Debug)]
struct Event {
    id: u64,
    date: DateTime<Utc>,
    // date: String,
    title: String,
    desc: String,
    link: String,
    participants: Vec<User>,
}

impl EventHandler for Handler {}

fn main() {
    kankyo::load().expect("Failed to load .env file");
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .cmd("add", add));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(add(_context, message, args) {
    let event = Event {
                id: 1,
                date: Utc.datetime_from_str(&args.single::<String>().unwrap(), "%Y-%m-%dT%H:%M").unwrap(),
                title: args.single_quoted::<String>().unwrap(),
                desc: args.single_quoted::<String>().unwrap(),
                link: args.single::<String>().unwrap(),
                participants: vec![message.author.clone()]
            };
    println!("{:?}", event);
});

//2014-11-28T12:00:00

// fn parse_event(text: &String) -> (String, String, String) {
//     text.split("#").collect();
//     let title = "lul".to_string();
//     let desc = "bar".to_string();
//     let link = "foo".to_string();
//     (title, desc, link)
// }

// !add 2019-10-10 :: Something will happen :: This is a description :: www.somelink.com