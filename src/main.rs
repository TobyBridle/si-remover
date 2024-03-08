use dotenv_codegen::dotenv;
use serenity::{
    all::{CacheHttp, EventHandler, GatewayIntents},
    async_trait, Client,
};
use serenity::all::ChunkGuildFilter::Query;
use url::{Url, ParseError, UrlQuery};
use url::form_urlencoded::Serializer;

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
                let chars: Vec<char> = new_message.content.chars().collect();
                let prev_char = chars[position - 1];
                if prev_char == '?' || prev_char == '&' {
                    println!("Checking {}", new_message.content);
                    // Find space after the tracking
                    let mut first_index: usize = 0;
                    let mut last_index = chars.len() - 1;
                    if let Some(_last_index) = chars[position..=last_index]
                        .into_iter()
                        .position(|c| *c == ' ')
                    {
                        last_index = _last_index + position;
                    }
                    if let Some(_first_index) =
                        chars[0..position].into_iter().collect::<String>().rfind(' ')
                    {
                        first_index = _first_index + 1;
                    }
                    if let Err(e) = new_message.delete(&ctx.http).await {
                        println!("Could not delete message within guild {:?}. Check permissions.", new_message.guild_id);
                    }
                    // if let Err(e) = new_message.channel_id.say(&ctx.http, "https://imgur.com/tmlPeSe").await {
                    //     eprintln!("{e:?}")
                    // }
                    let uri = chars[first_index..=last_index].to_vec();
                    if let Ok(mut new_uri) = Url::parse(uri.iter().collect::<String>().as_str()) {
                        // If we have found ?si/&si and the Uri is valid, then it MUST be within the query
                        let pairs: Vec<_> = new_uri.query_pairs().filter_map(|(key, value)| if key == "si" { None } else { Some((key, value)) }).collect();
                        let query = Serializer::new(String::new()).extend_pairs(pairs).finish();
                        new_uri.set_query(Some(query.as_str()));
                        if let Err(e) = new_message.channel_id.say(&ctx.http, new_uri).await {
                            eprintln!("{e:?}")
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
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
