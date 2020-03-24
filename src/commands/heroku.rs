use heroku_rs::endpoints::{apps, dynos};
use heroku_rs::framework::apiclient::HerokuApiClient;
use crate::HerokuClientKey;


use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
    released_at: String,
    web_url: String,
}

// Get app by name or id
#[command]
#[num_args(1)]
pub fn get_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ctx_clone = ctx.clone();
    let data = ctx_clone.data.read();

    let heroku_client = data
        .get::<HerokuClientKey>()
        .expect("Expected Heroku client");

    let response = heroku_client
        .request(&apps::AppDetails { app_id: app_name });

    msg.reply(
        ctx,
        match response {
            Ok(app) => app_response(app),
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when fetching your Heroku app".into()
            }
        },
    )?;

    Ok(())
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let ctx_clone = ctx.clone();
    let data = ctx_clone.data.read();

    let heroku_client = data
        .get::<HerokuClientKey>()
        .expect("Expected Heroku client");

    let response = heroku_client.request(&apps::AppList {});

    msg.reply(
        ctx,
        match response {
            Ok(apps) => apps_response(apps),
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when fetching your Heroku apps".into()
            }
        },
    )?;

    Ok(())
}

#[command]
#[num_args(1)]
pub fn restart_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ctx_clone = ctx.clone();
    let data = ctx_clone.data.read();

    let heroku_client = data
        .get::<HerokuClientKey>()
        .expect("Expected Heroku client");

    let response = heroku_client.request(&dynos::DynoAllRestart {
        app_id: app_name.clone(),
    });

    msg.reply(
        ctx,
        match response {
            Ok(_response) => format!("All dynos in {} have been restarted.", app_name),
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when trying to restart your Heroku app".into()
            }
        },
    )?;

    Ok(())
}

fn app_response(app: heroku_rs::endpoints::apps::App) -> String {
    format!(
        "\nApp ID: {}\nApp Name: {}\nReleased At: {}\nWeb URL: {}\n\n",
        app.id,
        app.name,
        app.released_at.unwrap(),
        app.web_url
    )
}

fn apps_response(processed_app_list: Vec<heroku_rs::endpoints::apps::App>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = app_response(app);
        list.push_str(&app_info);
    }

    list
}
