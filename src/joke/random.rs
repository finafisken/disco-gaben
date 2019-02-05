extern crate reqwest;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};

fn fetch_joke() -> Result<(HashMap<String, String>), Box<std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get("https://icanhazdadjoke.com")?
        .json()?;
    Ok(resp)
}

command!(cmd(_ctx, message) {
    match fetch_joke() {
        Ok(response) => {
            match response.get("joke").ok_or(Error::new(ErrorKind::Other, "Joke not found!")) {
                Ok(joke) => {
                    message.channel_id.say(joke)?;
                }
                Err(_e) => {
                    message.channel_id.say(String::from(":troll:"))?;        
                }
            }
        }
        Err(_e) => {
            message.channel_id.say(String::from(":troll:"))?;
        }
    }
});