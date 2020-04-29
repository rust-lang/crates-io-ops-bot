use crate::HerokuClientKey;
use heroku_rs::endpoints::{apps, builds, config_vars, dynos, formations, releases};
use heroku_rs::framework::apiclient::HerokuApiClient;

use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

use std::{thread, time};

use crate::config::Config;

use crate::utilities::*;

use github_rs::client::{Executor, Github};

use serde_json::Value;

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
const AUTHORIZED_CONFIG_VARS: &[&str] = &["FOO"];

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

const BLOCKED_IPS_ENV_VAR: &str = "BLOCKED_IPS";

#[command]
#[num_args(2)]
pub fn block_ip(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ip_addr = args
        .single::<String>()
        .expect("You must include an IP address to block");

    let current_config_vars = heroku_app_config_vars(&ctx, &app_name);

    // If the BLOCKED_IPS environmental variable does not
    // currently exist, create it
    if !blocked_ips_exist(&current_config_vars) {
        let response = heroku_client(&ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: empty_config_var(),
        });

        let response_err = response.is_err();

        msg.reply(
            &ctx,
            match response {
                Ok(_response) => format!("The {} environmental variable has been created for {}", BLOCKED_IPS_ENV_VAR, app_name),
                Err(e) => format!(
                    "The {} environmental variable does not current exist for {}.\n There was an error when trying to create it: {}",
                    BLOCKED_IPS_ENV_VAR, app_name, e
                ),
            },
        )?;

        if response_err {
            return Ok(());
        }
    }

    let mut blocked_ips_set = current_blocked_ip_addresses(current_config_vars);

    if blocked_ips_set.contains(&ip_addr) {
        msg.reply(
            &ctx,
            format!("{} is already blocked for {}", &ip_addr, app_name),
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
                    ip_addr, e
                ),
            },
        )?;
    };

    Ok(())
}

#[command]
#[num_args(2)]
pub fn unblock_ip(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ip_addr = args
        .single::<String>()
        .expect("You must include an IP address to unblock");

    let current_config_vars = heroku_app_config_vars(&ctx, &app_name);

    if !blocked_ips_exist(&current_config_vars) {
        msg.reply(
            &ctx,
            format!("No IP addresses are currently blocked for {}", &app_name),
        )?;

        return Ok(());
    }

    let mut blocked_ips_set = current_blocked_ip_addresses(current_config_vars);

    if !blocked_ips_set.contains(&ip_addr) {
        msg.reply(
            &ctx,
            format!("{} is not currently blocked for {}", &ip_addr, app_name),
        )?;
    } else {
        blocked_ips_set.remove(&ip_addr);

        // Removes config variable from the Heroku application
        // if there are no more blocked ip addresses
        if blocked_ips_set.is_empty() {
            let response = heroku_client(ctx).request(&config_vars::AppConfigVarDelete {
                app_id: &app_name,
                params: null_blocked_ips_config_var(),
            });

            msg.reply(
                ctx,
                match response {
                    Ok(_response) => format!(
                        "IP address {} has been unblocked, there are now no unblocked IP addresses",
                        ip_addr.clone()
                    ),
                    Err(e) => format!(
                        "An error occurred when trying to unblock the IP address: {}\n{}",
                        ip_addr, e
                    ),
                },
            )?;
        } else {
            let response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
                app_id: &app_name,
                params: blocked_ips_config_var(blocked_ips_set),
            });

            msg.reply(
                ctx,
                match response {
                    Ok(_response) => format!("IP address {} has been unblocked", ip_addr.clone()),
                    Err(e) => format!(
                        "An error occurred when trying to unblock the IP address: {}\n{}",
                        ip_addr, e
                    ),
                },
            )?;
        };
    }

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

#[command]
#[num_args(2)]
pub fn deploy_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let git_ref = args
        .single::<String>()
        .expect("You must include a git ref to deploy");

    let github_client = Github::new(bot_config(ctx).github_token.to_string());

    if github_client.is_err() {
        msg.reply(
            &ctx,
            format!(
                "An error occured when trying to contact GitHub with your GitHub token:\n{:?}",
                github_client.err()
            ),
        )?;

        return Ok(());
    }

    let github_response = github_client
        .unwrap()
        .get()
        .custom_endpoint(&github_ref_endpoint(bot_config(ctx), git_ref))
        .execute::<Value>();

    if github_response.is_err() {
        msg.reply(
            &ctx,
            format!(
                "An error occured when trying to get commit info for {}/{}:\n{:?}",
                bot_config(ctx).github_org,
                bot_config(ctx).github_repo,
                github_response.as_ref().err()
            ),
        )?;
    }

    let git_sha_value = &github_response.unwrap().2.unwrap()["sha"];
    let git_sha = git_sha_value.as_str().unwrap();

    let build_create_response = heroku_client(ctx).request(&builds::BuildCreate {
        app_id: app_name.clone(),
        params: builds::BuildCreateParams {
            buildpacks: None,
            source_blob: builds::SourceBlobParam {
                checksum: None,
                url: source_url(&ctx, &git_sha.to_string()),
                version: Some(git_sha.to_string()),
            },
        },
    });

    if build_create_response.is_err() {
        msg.reply(
            &ctx,
            format!(
                "An error occured when trying to build {}:\n{:?}",
                &app_name,
                build_create_response.err()
            ),
        )?;

        return Ok(());
    }

    let build = build_create_response.unwrap();

    msg.reply(&ctx, build_response(&app_name, &build))?;

    let mut build_pending = true;

    while build_pending == true {
        let build_info_response = heroku_client(ctx).request(&builds::BuildDetails {
            app_id: app_name.clone(),
            build_id: build.clone().id,
        });

        if build_info_response.is_err() {
            msg.reply(
                &ctx,
                format!(
                    "An error occured when trying to get the status of build {} for {}",
                    &build.id, &app_name,
                ),
            )?;

            return Ok(());
        }

        if build_info_response.unwrap().status == String::from("pending") {
            msg.channel_id
                .say(&ctx, format!("Build {} is still pending...", &build.id))?;

            let duration = time::Duration::from_secs(bot_config(&ctx).build_check_interval);
            thread::sleep(duration);
        } else {
            build_pending = false
        }
    }

    // Release the new build
    let final_build_info_response = heroku_client(ctx).request(&builds::BuildDetails {
        app_id: app_name.clone(),
        build_id: build.clone().id,
    });

    println!("final_build_info_response {:?}", final_build_info_response);

    if final_build_info_response.is_err() {
        msg.reply(
            &ctx,
            format!(
                "Unable to get the final information for build {} for {}, cancelling release",
                &build.id, &app_name
            ),
        )?;

        return Ok(());
    }

    let final_build_info = final_build_info_response.unwrap();

    if final_build_info.status != "succeeded" {
        msg.reply(
            &ctx,
            format!(
                "There was a problem with build {} for {}, cancelling release. Please check the build output.",
                &build.id, &app_name
            ),
        )?;

        return Ok(());
    }

    let slug = final_build_info.slug.unwrap().id;

    let release_response = heroku_client(ctx).request(&releases::ReleaseCreate {
        app_id: app_name.clone(),
        params: releases::ReleaseCreateParams {
            slug: String::from(slug),
            description: Some(git_sha.to_string()),
        },
    });

    msg.reply(
        ctx,
        match release_response {
            Ok(_release) => format!(
                "App {} commit {} has successfully been released!",
                &app_name, git_sha.to_string(),
            ),
            Err(e) => format!(
                "An error occured when trying to release your app {}:\n{}",
                app_name, e
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

fn bot_config(ctx: &Context) -> std::sync::Arc<Config> {
    ctx.data
        .read()
        .get::<Config>()
        .expect("Expected Config")
        .clone()
}

fn heroku_app_config_vars(ctx: &Context, app_name: &str) -> HashMap<String, Option<String>> {
    let config_var_list = heroku_client(ctx)
        .request(&config_vars::AppConfigVarDetails { app_id: &app_name })
        .unwrap();
    config_var_list
}

fn block_ips_value(config_vars: HashMap<String, Option<String>>) -> String {
    config_vars
        .get(&BLOCKED_IPS_ENV_VAR.to_string())
        .unwrap()
        .as_ref()
        .unwrap()
        .to_string()
}

fn blocked_ips_config_var(blocked_ips_set: HashSet<String>) -> HashMap<String, String> {
    let blocked_ips_set_string = parse_config_value_string(blocked_ips_set);
    let blocked_ips_config_var = config_var(blocked_ips_set_string);
    blocked_ips_config_var
}

fn config_var(updated_blocked_ips_value: String) -> HashMap<String, String> {
    let mut config_var = HashMap::new();
    config_var.insert(BLOCKED_IPS_ENV_VAR.to_string(), updated_blocked_ips_value);
    config_var
}

fn empty_config_var() -> HashMap<String, String> {
    let mut config_var = HashMap::new();
    config_var.insert(BLOCKED_IPS_ENV_VAR.to_string(), "".to_string());
    config_var
}

fn current_blocked_ip_addresses(config_vars: HashMap<String, Option<String>>) -> HashSet<String> {
    let blocked_ips_value = block_ips_value(config_vars);

    let blocked_ips_set = parse_config_value_set(blocked_ips_value);
    blocked_ips_set
}

fn null_blocked_ips_config_var() -> HashMap<String, Option<String>> {
    let mut config_var = HashMap::new();
    config_var.insert(BLOCKED_IPS_ENV_VAR.to_string(), None);
    config_var
}

fn blocked_ips_exist(config_vars: &HashMap<String, Option<String>>) -> bool {
    let exists = config_vars.get(&BLOCKED_IPS_ENV_VAR.to_string()).is_some();
    exists
}

fn build_response(app_name: &str, build: &heroku_rs::endpoints::builds::Build) -> String {
    format!(
        "Build in progress for {} (this will take a few minutes)\nBuild ID is {}",
        app_name, build.id,
    )
}

fn source_url(ctx: &Context, git_sha: &str) -> String {
    format!(
        "https://codeload.github.com/{}/{}/tar.gz/{}",
        bot_config(ctx).github_org,
        bot_config(ctx).github_repo,
        git_sha,
    )
}

fn github_ref_endpoint(config: std::sync::Arc<Config>, git_ref: String) -> String {
    format!(
        "repos/{}/{}/commits/{}",
        config.github_org, config.github_repo, git_ref
    )
}
