use serenity::client::Client;
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};

mod commands;

use commands::{heroku::*, math::*, myid::*, ping::*};

mod authorizations;

mod config;

use crate::config::Config;

#[group]
#[commands(ping, multiply, myid, get_apps)]
struct General;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn run() {
    let config = Config::default();

    let mut client = Client::new(&config.discord_token, Handler).expect("Err creating client");

    // Insert default config into data
    // that is passed to each of the commands
    {
        let mut data = client.data.write();
        data.insert::<Config>(config);
    }

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
