#[macro_use] extern crate serenity;
extern crate kankyo;
extern crate typemap;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate uuid;

use uuid::Uuid;
use serenity::{client::Client, prelude::EventHandler, framework::standard::StandardFramework};
use typemap::Key;
use std::{env, collections::HashMap, default::Default};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, AttributeValue, PutItemInput, UpdateItemInput, ScanInput};

struct Handler;

impl EventHandler for Handler { }

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
            .cmd("list", event_list)
            .cmd("join", event_join)
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
            let _ = message.channel_id.say(format!("Failed adding event\n{}", error));
        }
    }
});

command!(event_list(ctx, message) {
    let mut data = ctx.data.lock();
    let db_client = data.get_mut::<DbClient>().unwrap();

    let db_scan = ScanInput {
        table_name: String::from("disco-gaben-events"),
        ..Default::default()
    };

    match db_client.scan(db_scan).sync() {
        Ok(result) => {
            match result.items {
                Some(items) => {
                    for event in items.iter() {
                        let id = event.get(&String::from("id")).unwrap().s.clone().unwrap();
                        let date = event.get(&String::from("date")).unwrap().s.clone().unwrap();
                        let title = event.get(&String::from("title")).unwrap().s.clone().unwrap();
                        let link = event.get(&String::from("link")).unwrap().s.clone().unwrap();
                        let participants = event.get(&String::from("participants")).unwrap().ss.clone().unwrap();

                        let msg = format!(":date: **{}** [{}]\n`{}`\n{}\n```{:?}```", title, id, date, link, participants);
                        let _ = message.channel_id.say(&msg);
                    }
                }
                None => {
                    let _ = message.channel_id.say(String::from("No events found"));
                }
            }
        }
        Err(error) => {
            let _ = message.channel_id.say(format!("Failed adding you to the event\n{}", error));
            println!("Failed adding user to the event\n{}", error);
        }
    }
});

command!(event_join(ctx, message, args) {
    let mut data = ctx.data.lock();
    let db_client = data.get_mut::<DbClient>().unwrap();

    let mut key = HashMap::new();
    let mut exp_val = HashMap::new();

    key.insert(String::from("id"), AttributeValue {
        s: Some(args.single::<String>().unwrap()),
        ..Default::default()
    });

    exp_val.insert(String::from(":user"), AttributeValue {
        ss: Some(vec![message.author.name.clone()]),
        ..Default::default()
    });

    let db_update = UpdateItemInput {
        key,
        expression_attribute_values: Some(exp_val),
        update_expression: Some(String::from("ADD participants :user")),
        table_name: String::from("disco-gaben-events"),
        ..Default::default()
    };

    match db_client.update_item(db_update).sync() {
        Ok(_) => {
            let _ = message.channel_id.say("Added you to event");
        }
        Err(error) => {
            let _ = message.channel_id.say(format!("Failed adding you to the event\n{}", error));
            println!("Failed adding user to the event\n{}", error);
        }
    }
});