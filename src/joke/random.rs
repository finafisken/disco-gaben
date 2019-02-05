extern crate reqwest;

use reqwest::header::{ACCEPT};

fn fetch_joke() -> Result<(String), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let mut response = client.get("https://icanhazdadjoke.com")
        .header(ACCEPT, "text/plain")
        .send()?;
    let joke = response.text()?;
    Ok(joke)
}

command!(cmd(_ctx, message) {
    match fetch_joke() {
        Ok(joke) => {
            message.channel_id.say(joke)?;
        }
        Err(e) => {
            println!("[joke::random] Error {:?}", e);
            message.channel_id.say(String::from("Error, unforeseen consequences."))?;
        }
    }
});