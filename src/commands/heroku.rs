use crate::HerokuClientKey;
use heroku_rs::endpoints::{apps, dynos, formations};
use heroku_rs::framework::apiclient::HerokuApiClient;

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

    let app_resp = heroku_client(ctx)
        .request(&apps::AppDetails { app_id: app_name.clone() });

    let app_formations_resp = heroku_client(ctx)
        .request(&formations::FormationList { app_id: app_name });

    msg.reply(
        ctx.clone(),
        match app_resp {
            Ok(app) => app_info_response(app),
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when fetching your Heroku app".into()
            }
        },
    )?;

    msg.reply(
        ctx,
        match app_formations_resp {
            Ok(formations) => app_formations_response(formations),
            Err(e) => {
                println!("Error: {}", e);
                "An error occured when fetching your Heroku app formation info".into()
            }
        }
    )?;

    Ok(())
}

#[command]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let response = heroku_client(ctx).request(&apps::AppList {});

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

    let response = heroku_client(ctx).request(&dynos::DynoAllRestart {
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

fn app_info_response(app: heroku_rs::endpoints::apps::App) -> String {
    format!(
        "\nApp ID: {}\nApp Name: {}\nReleased At: {}\nWeb URL: {}\n\n",
        app.id,
        app.name,
        app.released_at.unwrap_or("never".to_string()),
        app.web_url
    )
}

fn app_formation_response(formation: heroku_rs::endpoints::formations::Formation) -> String {
    format!(
        "\nCommand: {}\nQuantity: {}\nType: {}\n\n", 
        formation.command, 
        formation.quantity, 
        formation.r#type
    ) 
}

fn app_formations_response(formations_list: Vec<heroku_rs::endpoints::formations::Formation>) -> String {
    let mut list = String::from("Here are are the formations for that app:\n");

    for formation in formations_list {
        let formation_info = app_formation_response(formation);
        list.push_str(&formation_info);
    }

    list
}

fn apps_response(processed_app_list: Vec<heroku_rs::endpoints::apps::App>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = app_info_response(app);
        list.push_str(&app_info);
    }

    list
}

fn heroku_client(ctx: &Context) -> std::sync::Arc<heroku_rs::framework::HttpApiClient> {
    ctx.data
        .read()
        .get::<HerokuClientKey>()
        .expect("Expected Heroku Client Key")
        .clone()
}
