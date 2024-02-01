use serenity::{client::Client, framework::StandardFramework, prelude::Mutex};
use songbird::{driver::DecodeMode, Config, SerenityInit};
use dotenv::dotenv;
use std::{env, sync::Arc};
use serenity::prelude::*;

mod commands;
mod handler;

use commands::receiver::Receiver;
use handler::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::var("USRP_SEND").expect("Expected a port to send USRP audio to in the environment.");
    env::var("USRP_RECEIVE").expect("Expected a port to receive USRP audio from in the environment.");
    let token = env::var("BOT_TOKEN").expect("Expected a Bot token in the environment.");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(env::var("BOT_PREFIX").unwrap_or_else(|_| String::from("!")).as_str()))
        .group(&commands::GENERAL_GROUP);
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_VOICE_STATES;
    let songbird_config = Config::default().decode_mode(DecodeMode::Decode);
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird_from_config(songbird_config)
        .await
        .expect("Error creating client.");
    {
        let mut data = client.data.write().await;
        data.insert::<commands::USRPContext>(Arc::new(Mutex::new(Receiver::new())));
    }
    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}
