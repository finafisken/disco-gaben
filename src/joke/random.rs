extern crate reqwest;

use reqwest::header::{ACCEPT};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

fn fetch_joke() -> Result<(HashMap<String, String>), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let mut response = client.get("https://icanhazdadjoke.com")
        .header(ACCEPT, "application/json")
        .send()?;
    let json = response.json()?;        
    Ok(json)
}

command!(cmd(_ctx, message) {
    match fetch_joke() {
        Ok(response) => {
            match response.get("joke").ok_or(Error::new(ErrorKind::Other, "Joke not found!")) {
                Ok(joke) => {
                    message.channel_id.say(joke)?;
                }
                Err(_e) => {
                    message.channel_id.say(String::from("Error, Gaben is sleeping on the job"))?;
                }
            }
        }
        Err(_e) => {
            message.channel_id.say(String::from("Error, unforeseen consequences."))?;
        }
    }
});