use std::{collections::HashMap, default::Default};
use rusoto_dynamodb::{DynamoDb, AttributeValue, DeleteItemInput};
use crate::util::DbClient;

command!(cmd(ctx, message, args) {
    let mut data = ctx.data.lock();
    let db_client = data.get_mut::<DbClient>().unwrap();

    let mut key = HashMap::new();

    key.insert(String::from("id"), AttributeValue {
        s: Some(args.single::<String>()?),
        ..Default::default()
    });

    let db_delete = DeleteItemInput {
        key,
        table_name: String::from("disco-gaben-events"),
        ..Default::default()
    };

    match db_client.delete_item(db_delete).sync() {
        Ok(_) => {
            message.channel_id.say("Removed event")?;
        }
        Err(error) => {
            message.channel_id.say(format!("Failed to remove the event\n{}", error))?;
        }
    }
});