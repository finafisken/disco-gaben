
use std::default::Default;
use rusoto_dynamodb::{DynamoDb, ScanInput};
use crate::util::DbClient;

command!(cmd(ctx, message) {
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
                        let fmt_participants: String = participants.iter().map(|p| format!("{}{}{}", "+ ", p, "\n")).collect();

                        let msg = format!(":date: **{}** (<{}>) ```diff\nid {}\n\nWhen?\n- {}\n\nWho?\n{}```", title, link, id, date, fmt_participants);
                        message.channel_id.say(&msg)?;
                    }
                }
                None => {
                    message.channel_id.say(String::from("No events found"))?;
                }
            }
        }
        Err(error) => {
            message.channel_id.say(format!("Failed adding you to the event\n{}", error))?;
            println!("Failed adding user to the event\n{}", error);
        }
    }
});