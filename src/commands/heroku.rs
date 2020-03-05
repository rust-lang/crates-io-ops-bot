use heroku_rs::client::{Executor, Heroku};

use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::config::Config;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let ctx_clone = ctx.clone();
    let data = ctx_clone.data.read();
    let config = data.get::<Config>().expect("Expected config");

    let response = heroku_client(&config.heroku_api_key)
        .get()
        .apps()
        .execute::<Vec<HerokuApp>>();
    let mut processed_app_list: Vec<HerokuApp> = Vec::new();

    match response {
        Ok((_headers, _status, json)) => {
            if let Some(mut json) = json {
                processed_app_list.append(&mut json);
            }
        }

        Err(e) => println!("Err {}", e),
    }

    msg.reply(ctx, app_response(processed_app_list))?;

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
