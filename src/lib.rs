use serenity::client::Client;
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};
use std::sync::Arc;

mod commands;

use commands::{heroku::*, math::*, myid::*, ping::*};

mod authorizations;

pub mod config;

use crate::config::Config;

use crate::authorizations::users::*;

#[group]
#[commands(ping, multiply, myid, get_app, get_apps, restart_app)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn run(config: Config) {
    let mut client = Client::new(&config.discord_token, Handler).expect("Err creating client");

    // Insert default config into data
    // that is passed to each of the commands
    {
        let mut data = client.data.write();
        data.insert::<Config>(Arc::new(config.clone()));
    }

    client.with_framework(
        StandardFramework::new()
            .before(move |ctx, msg, cmd_name| {
                if !is_authorized(&msg.author.id.to_string(), config.clone()) {
                    println!("User is not authorized to run this command");
                    msg.reply(
                        ctx,
                        format!("User {} is not authorized to run this command", &msg.author),
                    ).ok();

                    return false;
                }
                println!("Running command {}", cmd_name);
                true
            })
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
