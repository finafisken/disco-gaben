use std::{collections::HashMap, default::Default};
use rusoto_dynamodb::{DynamoDb, AttributeValue, UpdateItemInput};
use crate::util::DbClient;

command!(cmd(ctx, message, args) {
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