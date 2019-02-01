use uuid::Uuid;
use std::{collections::HashMap, default::Default};
use rusoto_dynamodb::{DynamoDb, AttributeValue, PutItemInput};
use crate::util::DbClient;

command!(cmd(ctx, message, args) {
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