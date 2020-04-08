use crate::HerokuClientKey;
use heroku_rs::endpoints::{apps, config_vars, dynos, formations, releases};
use heroku_rs::framework::apiclient::HerokuApiClient;

use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

use crate::utilities::*;

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

    let app_resp = heroku_client(ctx).request(&apps::AppDetails {
        app_id: app_name.clone(),
    });

    msg.reply(
        &ctx,
        match app_resp {
            Ok(app) => app_info_response(app),
            Err(e) => format!("An error occurred when fetching your Heroku app:\n{}", e),
        },
    )?;

    let app_formations_resp =
        heroku_client(ctx).request(&formations::FormationList { app_id: app_name });

    msg.reply(
        ctx,
        match app_formations_resp {
            Ok(formations) => app_formations_response(formations),
            Err(e) => format!(
                "An error occured when fetching your Heroku app formation info:\n{}",
                e
            ),
        },
    )?;

    Ok(())
}

// App config variables that can be updated through Discord
const AUTHORIZED_CONFIG_VARS: &[&str] = &["BLOCKED_IPS"];

// Get app by name or id
#[command]
#[num_args(3)]
pub fn update_app_config(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let config_var_key = args
        .single::<String>()
        .expect("You must include a config variable key");

    let config_var_key_reference: &str = &config_var_key;

    let config_var_value = args
        .single::<String>()
        .expect("You must include a config variable value");

    if AUTHORIZED_CONFIG_VARS.contains(&config_var_key_reference) {
        let mut config_var = HashMap::new();
        config_var.insert(config_var_key, config_var_value);

        let response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: config_var.clone(),
        });

        msg.reply(
            ctx,
            match response {
                Ok(_response) => format!("Config Var has been updated {:?}", config_var),
                Err(e) => format!(
                    "An error occured when trying to update your config var:\n{}",
                    e
                ),
            },
        )?;
    } else {
        msg.reply(
            &ctx,
            format!(
                "Config var {} is not authorized to be updated from Discord",
                &config_var_key
            ),
        )?;
    }

    Ok(())
}

#[command]
#[num_args(2)]
pub fn block_ip(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ip_addr = args
        .single::<String>()
        .expect("You must include an IP address to block");

    let blocked_ips_value = block_ips_value(heroku_app_config_vars(&ctx, &app_name));

    let mut blocked_ips_set = parse_config_value_set(blocked_ips_value);

    if blocked_ips_set.contains(&ip_addr) {
        msg.reply(
            &ctx,
            format!("That IP address is already blocked for {}", app_name),
        )?;
    } else {
        blocked_ips_set.insert(ip_addr.clone());

        let updated_config_var = blocked_ips_config_var(blocked_ips_set);
        
        let response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: updated_config_var,
        });

        msg.reply(
            ctx,
            match response {
                Ok(_response) => format!("IP address {} has been blocked", ip_addr.clone()),
                Err(e) => format!(
                    "An error occurred when trying to block the IP address: {}\n{}",
                    ip_addr,
                    e
                ),
            },
        )?;
    };

    Ok(())
}

#[command]
#[num_args(4)]
pub fn scale_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let formation_name = args
        .single::<String>()
        .expect("You must include a formation name");

    let quantity = args.single::<i32>().expect("You must include a quantity");

    let size = args.single::<String>().expect("You must include a size");

    let response = heroku_client(ctx).request(&formations::FormationUpdate {
        app_id: app_name.clone(),
        formation_id: formation_name,
        params: formations::FormationUpdateParams {
            quantity: Some(quantity),
            size: Some(size),
        },
    });

    msg.reply(
        ctx,
        match response {
            Ok(formation) => formation_updated_response(app_name, formation),
            Err(e) => format!(
                "An error occured when trying to scale your app formation:\n{}",
                e
            ),
        },
    )?;

    Ok(())
}

// Get app by name or id
#[command]
#[num_args(1)]
pub fn get_app_releases(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let response = heroku_client(ctx).request(&releases::ReleaseList { app_id: app_name });

    msg.reply(
        ctx,
        match response {
            Ok(releases) => releases_response(releases),
            Err(e) => format!("An error occured when fetching your app's releases:\n{}", e),
        },
    )?;

    Ok(())
}

#[command]
#[num_args(2)]
pub fn rollback_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let version_to_rollback_to = args
        .single::<String>()
        .expect("You must include the version to roll back to");

    let response = heroku_client(ctx).request(&releases::ReleaseRollback {
        app_id: app_name.clone(),
        params: releases::ReleaseRollbackParams {
            release: version_to_rollback_to.clone(),
        },
    });

    msg.reply(
        ctx,
        match response {
            Ok(_response) => format!(
                "App {} was successfully rolled back to the code at {}",
                app_name, version_to_rollback_to
            ),
            Err(e) => format!("An error occured when trying to roll back your app:\n{}", e),
        },
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
            Err(e) => format!("An error occured when fetching your Heroku apps:\n{}", e),
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
            Err(e) => format!(
                "An error occured when trying to restart your Heroku app:\n{}",
                e
            ),
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
        "\nName: {}\nCommand: {}\nQuantity: {}\nSize: {}\n\n",
        formation.r#type, formation.command, formation.quantity, formation.size,
    )
}

fn app_formations_response(
    formations_list: Vec<heroku_rs::endpoints::formations::Formation>,
) -> String {
    let mut list = String::from("\nFormations for this app:\n");

    for formation in formations_list {
        let formation_info = app_formation_response(formation);
        list.push_str(&formation_info);
    }

    list
}

fn formation_updated_response(
    app_name: String,
    formation: heroku_rs::endpoints::formations::Formation,
) -> String {
    let mut response = format!(
        "App {}'s formation {} has been updated",
        app_name, formation.r#type
    );

    response.push_str(&app_formation_response(formation));
    response
}

fn apps_response(processed_app_list: Vec<heroku_rs::endpoints::apps::App>) -> String {
    let mut list = String::from("Here are your Heroku apps\n");

    for app in processed_app_list {
        let app_info = app_info_response(app);
        list.push_str(&app_info);
    }

    list
}

fn releases_response(
    processed_release_list: Vec<heroku_rs::endpoints::releases::Release>,
) -> String {
    let mut list = String::from("Here are your app releases\n");

    for release in processed_release_list {
        let release_info = release_info_response(release);
        list.push_str(&release_info);
    }

    list
}

fn release_info_response(release: heroku_rs::endpoints::releases::Release) -> String {
    format!(
        "ID: {}\nVersion: {}\nStatus: {}\n\n",
        release.id, release.version, release.status,
    )
}

fn heroku_client(ctx: &Context) -> std::sync::Arc<heroku_rs::framework::HttpApiClient> {
    ctx.data
        .read()
        .get::<HerokuClientKey>()
        .expect("Expected Heroku Client Key")
        .clone()
}

fn heroku_app_config_vars(ctx: &Context, app_name: &str) -> HashMap<String, Option<String>> {
    let config_var_list = heroku_client(ctx).request(&config_vars::AppConfigVarDetails { app_id: &app_name }).unwrap();
    config_var_list
}

fn block_ips_value(config_vars: HashMap<String, Option<String>>) -> String {
    config_vars.get(&"BLOCKED_IPS".to_string()).unwrap().as_ref().unwrap().to_string()
}

fn blocked_ips_config_var(blocked_ips_set: HashSet<String>) -> HashMap<String,String> {
    let blocked_ips_set_string = parse_config_value_string(blocked_ips_set);
    let blocked_ips_config_var = config_var(blocked_ips_set_string);
    blocked_ips_config_var
}

fn config_var(updated_blocked_ips_value: String) -> HashMap<String, String> {
    let mut config_var = HashMap::new();
    config_var.insert("BLOCKED_IPS".to_string(), updated_blocked_ips_value);
    config_var
}