#[macro_use] extern crate serenity;
extern crate kankyo;
extern crate chrono;
extern crate typemap;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate uuid;

use uuid::Uuid;
use serenity::{client::Client, prelude::EventHandler, framework::standard::StandardFramework, model::user::User};
use chrono::prelude::*;
use typemap::Key;
use std::{env, collections::HashMap, default::Default};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, AttributeValue, PutItemInput};

struct Handler;

impl EventHandler for Handler { }

#[derive(Debug)]
struct Event {
    id: u64,
    date: DateTime<Utc>,
    title: String,
    link: String,
    participants: Vec<User>,
}

struct DbClient;

impl Key for DbClient {
    type Value = DynamoDbClient;
}

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
                .cmd(event_add)
            )
            // .cmd("list", event_list)
            // .cmd("join", event_join)
        ));

    // start listening for events by starting a single shard
    if let Err(why) = discord_client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(event_add(ctx, message, args) {
    let mut data = ctx.data.lock();
    let db_client = data.get_mut::<DbClient>().unwrap();
    let mut event: HashMap<String, AttributeValue> = HashMap::new();

    event.insert(String::from("id"), AttributeValue {
        s: Some(Uuid::new_v4().to_string()),
        ..Default::default()
    });

    event.insert(String::from("date"), AttributeValue {
        s: Some(args.single::<String>().unwrap()),
        ..Default::default()
    });

    event.insert(String::from("title"), AttributeValue {
        s: Some(args.single_quoted::<String>().unwrap()),
        ..Default::default()
    });

    event.insert(String::from("link"), AttributeValue {
        s: Some(args.single::<String>().unwrap()),
        ..Default::default()
    });

    event.insert(String::from("participants"), AttributeValue {
        ss: Some(vec![message.author.name.clone()]),
        ..Default::default()
    });

    let db_input = PutItemInput {
        item: event,
        table_name: String::from("disco-gaben-events"),
        ..Default::default()
    };

    match db_client.put_item(db_input).sync() {
        Ok(_) => {
            let _ = message.channel_id.say("Event has been added");
        }
        Err(error) => {
            let _ = message.channel_id.say(format!("Failed adding event \n {}", error));
        }
    }
});

// command!(event_list(ctx, message) {
//     let mut data = ctx.data.lock();
//     let events = data.get_mut::<EventList>().unwrap();

//     for (_, event) in events.iter() {
//         let user_names: Vec<&String> = event.participants.iter().map(|u| &u.name).collect();
//         let msg = format!(":date: **{}** [#{}]\n`{}`\n{}\n```{:?}```", event.title, event.id, event.date, event.link, user_names);
//         let _ = message.channel_id.say(&msg);
//     }
// });

// command!(event_join(ctx, message, args) {
//     let mut data = ctx.data.lock();
//     let events = data.get_mut::<EventList>().unwrap();

//     let event = events.get_mut(&args.single::<u64>().unwrap()).unwrap();
//     event.participants.push(message.author.clone());
//     let _ = message.reply("Added you to event");
// });