use std::env;
use std::time::SystemTime;
use dotenv::dotenv;

use bonsaidb::{
    core::schema::{Collection, SerializedCollection},
    local::{
        config::{Builder, StorageConfiguration},
        Database,
    },
};

use serenity::{
    async_trait,
    prelude::*,
    model::channel::Message,
    framework::standard::{
        macros::{command, group},
        {StandardFramework, CommandResult},
    },
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "tickets")]
struct Ticket {
    pub timestamp: SystemTime,
    pub user: i32,
    pub message: String,
    pub status: String,
}

#[group]
#[commands(ping)]

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), bonsaidb::core::Error> {
    dotenv().ok();

    let db = Database::open::<Ticket>(StorageConfiguration::new("ticket.storage"))?;

    let ticket = Ticket {
        timestamp: SystemTime::now(),
        user: 123,
        message: String::from("Test ticket"),
        status: String::from("New"),
    }
    .push_into(&db)?;
    

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
    //Login with a bot token from the environment
    let token = env::var("DISCORDBOTTOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    
    // start listening fro events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }

    Ok(())

}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;


    Ok(())
}
