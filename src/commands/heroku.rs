use heroku_rs::client::{Executor, Heroku};

use serde::Deserialize;
use serde_json::Value;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::authorizations::users::*;
use crate::config::Config;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
    released_at: String,
    web_url: String,
}

// Get app by name or id
#[command]
pub fn get_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let config = bot_config(ctx);

    let app_name = args.single::<String>().unwrap();

    let response = heroku_client(&config.heroku_api_key)
        .get()
        .apps()
        .app_name(&app_name)
        .execute::<HerokuApp>();

    if is_authorized(&msg.author.id.to_string(), &*config) {
        msg.reply(
            ctx,
            match response {
                Ok((_, _, Some(app))) => app_response(app),
                Ok((_, _, None)) => "There is no Heroku app by that name".into(),
                Err(err) => {
                    println!("Err {}", err);
                    "An error occured while fetching your Heroku app".into()
                }
            },
        )?;
    } else {
        msg.reply(
            ctx,
            format!(
                "{}'s Discord ID is not authorized to run this command",
                msg.author.name
            ),
        )?;
    }

    Ok(())
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let config = bot_config(ctx);

    let response = heroku_client(&config.heroku_api_key)
        .get()
        .apps()
        .execute::<Vec<HerokuApp>>();

    if is_authorized(&msg.author.id.to_string(), &*config) {
        msg.reply(
            ctx,
            match response {
                Ok((_, _, Some(apps))) => apps_response(apps),
                Ok((_, _, None)) => "You have no Heroku apps".into(),
                Err(err) => {
                    println!("Err {}", err);
                    "An error occured while fetching your Heroku apps".into()
                }
            },
        )?;
    } else {
        msg.reply(
            ctx,
            format!(
                "{}'s Discord ID is not authorized to run this command",
                msg.author.name
            ),
        )?;
    }

    Ok(())
}

#[command]
pub fn restart_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let config = bot_config(ctx);

    let app_name = args.single::<String>().unwrap();

    let response = heroku_client(&config.heroku_api_key)
        .delete_empty()
        .apps()
        .app_name(&app_name)
        .app_dynos()
        .execute::<Value>();

    if is_authorized(&msg.author.id.to_string(), &*config) {
        msg.reply(
            ctx,
            match response {
                Ok((_, _, Some(_object))) => {
                    format!("All dynos in {} have been restarted.", app_name)
                }
                Ok((_, _, None)) => "There is no Heroku app by that name".into(),
                Err(err) => {
                    println!("Err {}", err);
                    "An error occured while fetching your Heroku app".into()
                }
            },
        )?;
    } else {
        msg.reply(
            ctx,
            format!(
                "{}'s Discord ID is not authorized to run this command",
                msg.author.name
            ),
        )?;
    }

    Ok(())
}

fn heroku_client(api_key: &str) -> heroku_rs::client::Heroku {
    Heroku::new(api_key).unwrap()
}

fn app_response(app: HerokuApp) -> String {
    format!(
        "\nApp ID: {}\nApp Name: {}\nReleased At: {}\nWeb URL: {}\n\n",
        app.id, app.name, app.released_at, app.web_url
    )
}

fn apps_response(processed_app_list: Vec<HerokuApp>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = app_response(app);
        list.push_str(&app_info);
    }

    list
}

fn bot_config(ctx: &Context) -> std::sync::Arc<Config> {
    let config = ctx
        .data
        .read()
        .get::<Config>()
        .expect("Expected config")
        .clone();

    config
}
