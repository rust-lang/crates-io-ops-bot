use heroku_rs::client::{Executor, Heroku};

use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let config = ctx.data.read().get::<Config>().expect("Expected config").clone();

    let response = heroku_client(&config.heroku_api_key)
        .get()
        .apps()
        .execute::<Vec<HerokuApp>>();

    msg.reply(ctx, match response {
        Ok((_, _, Some(apps))) => app_response(apps),
        Ok((_, _, None)) => "You have no Heroku apps".into(),
        Err(err) => {
            println!("Err {}", err);
            "An error occured while fetching your Heroku apps".into()
        }
    })?;

    Ok(())
}

fn heroku_client(api_key: &str) -> heroku_rs::client::Heroku {
    Heroku::new(api_key).unwrap()
}

fn app_response(processed_app_list: Vec<HerokuApp>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = format!("App ID: {}\n App Name: {}\n\n", app.id, app.name);
        list.push_str(&app_info);
    }

    list
}
