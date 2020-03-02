use heroku_rs::client::{Executor, Heroku};

use serde_json::Value;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;

#[derive(Debug)]
struct HerokuApp {
    id: String,
    name: String,
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let response = heroku_client().get().apps().execute::<Value>();
    let mut processed_app_list: Vec<HerokuApp> = Vec::new();

    match response {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                let array = json.as_array().unwrap();
                processed_app_list.append(&mut app_list(array))
            }
        }

        Err(e) => println!("Err {}", e),
    }

    msg.reply(ctx, app_response(processed_app_list))?;

    Ok(())
}

fn heroku_client() -> heroku_rs::client::Heroku {
    let heroku_api_key = Config::default().heroku_api_key;
    Heroku::new(heroku_api_key).unwrap()
}

fn app_list(app_array: &std::vec::Vec<serde_json::value::Value>) -> Vec<HerokuApp> {
    let mut app_list: Vec<HerokuApp> = Vec::new();

    for item in app_array.iter() {
        let app: HerokuApp = HerokuApp {
            id: item["id"].to_string(),
            name: item["name"].to_string(),
        };
        app_list.push(app);
    }

    app_list
}

fn app_response(processed_app_list: Vec<HerokuApp>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = format!("App ID: {}\n App Name: {}\n\n", app.id, app.name);
        list.push_str(&app_info);
    }

    list
}
