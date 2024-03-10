use dotenv_codegen::dotenv;
use serenity::{
    all::{EventHandler, GatewayIntents},
    async_trait, Client,
};
use url::form_urlencoded::Serializer;
use url::Url;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(
        &self,
        ctx: serenity::prelude::Context,
        new_message: serenity::model::prelude::Message,
    ) {
        if !new_message.author.bot {
            if let Some(position) = new_message.content.find("si=") {
                let mut chars: Vec<char> = new_message.content.chars().collect();
                let prev_char = chars[position - 1];
                if prev_char == '?' || prev_char == '&' {
                    println!("Attempting to cleanse {}", new_message.content);
                    // Find space after the tracking
                    let mut first_index: usize = 0;
                    let mut last_index = chars.len() - 1;
                    if let Some(_last_index) = chars[position..=last_index].into_iter().position(|c| *c == ' ') {
                        last_index = _last_index + position;
                    }
                    if let Some(_first_index) = chars[0..position].into_iter().collect::<String>().rfind(' ') {
                        first_index = _first_index + 1;
                    }
                    let uri = chars[first_index..=last_index].to_vec();
                    if let Ok(mut new_uri) = Url::parse(uri.iter().collect::<String>().as_str()) {
                        // If we have found ?si/&si and the Uri is valid, then it MUST be within the query
                        let pairs: Vec<_> = new_uri.query_pairs().filter_map(|(key, value)| if key == "si" { None } else { Some((key, value)) }).collect();
                        let query = Serializer::new(String::new()).extend_pairs(pairs).finish();
                        new_uri.set_query(Some(query.as_str()));
                        chars.splice(first_index..=last_index, new_uri.to_string().chars());
                        if let Err(e) = new_message.channel_id.say(&ctx.http, chars.iter().collect::<String>()).await {
                            eprintln!("{e:?}")
                        } else {
                            if let Err(e) = new_message.delete(&ctx.http).await {
                                println!("Could not delete message within guild {:?}. Check permissions.", new_message.guild_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = dotenv!("DISCORD_TOKEN");
    println!("Using Token: {}", token);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}