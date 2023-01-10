use std::env;
use std::time::SystemTime;
use dotenv::dotenv;

use bonsaidb::{
    core::{
        document::{CollectionDocument, Emit},
        schema::{
            view::CollectionViewSchema, Collection, ReduceResult, SerializedCollection,
            SerializedView, View, ViewMapResult, ViewMappedValue,
        },
    },
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
#[collection(name = "tickets", views = [ListOfTickets])]
struct Ticket {
    pub id: u32,
    pub timestamp: SystemTime,
    pub user: i32,
    pub message: String,
    pub status: String,
}

#[derive(Debug, Clone, View)]
#[view(collection = Ticket, key = u32, value = usize, name = "list-of-tickets")]
struct ListOfTickets;

impl CollectionViewSchema for ListOfTickets {
    type View = Self;

    fn map(&self, document: CollectionDocument<Ticket>) -> ViewMapResult<Self::View> {
        document
            .header
            .emit_key_and_value(document.contents.id, 1)
    }

    fn reduce(
        &self,
        mappings: &[ViewMappedValue<Self>],
        _rereduce: bool,
    ) -> ReduceResult<Self::View> {
        Ok(mappings.iter().map(|m| m.value).sum())
    }
    
}

#[group]
#[commands(ping, list)]

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<(), bonsaidb::core::Error> {
    dotenv().ok();

    let db = Database::open::<Ticket>(StorageConfiguration::new("ticket.storage"))?;

    let ticket = Ticket {
        id: 1,
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

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    
    let tickets = ListOfTickets::entries(&db).with_key(&1).query()?;
    println!("Number of tickets: {}", tickets.len());
    msg.reply(ctx, "Listing Tickets...").await?;

    Ok(())
}
