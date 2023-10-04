use std::env;
use std::time::SystemTime;
use dotenv::dotenv;
use rusqlite::{Connection,Result};


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

#[derive(Debug)]
struct Ticket {
    id: i32,
    owner: String,
    description: String,
    status: String
}

#[group]
#[commands(ping, list)]

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let conn = Connection::open("tickets.db")?;

    conn.execute(
        "create table if not exists tickets (
            id integer primary key,
            owner text not null,
            description text not null,
            status text not null
        )",
        (),
    )?;

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
    msg.reply(ctx, "Listing Tickets...").await?;

    let conn = Connection::open("Tickets.db");

    let stmt = conn.prepare("SELECT id, owner, description, status FROM tickets")?;
    let ticket_iter = stmt.query_map([], |row| {
        Ok(Ticket {
            id: row.get(0)?,
            owner: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    for ticket in ticket_iter {
        println!("Found ticket {:?}", ticket.unwrap());
    }

    Ok(())
}

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Creating ticket...").await?;

    let conn = Connection::open("Tickets.db");

    let test = Ticket {
        id: 0,
        owner: "Unassigned".to_string(),
        description: "Test ticket".to_string(),
        status: "New".to_string()
    };
    conn.execute(
        "INSERT INTO tickets (owner, description, status) VALUES (?1, ?2, ?3,)",
        (&test.owner, &test.description, &test.status),
    )?;
}

