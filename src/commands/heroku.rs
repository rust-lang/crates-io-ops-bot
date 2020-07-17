use crate::HerokuClientKey;
use heroku_rs::endpoints::{apps, builds, config_vars, dynos, formations, releases};
use heroku_rs::framework::apiclient::HerokuApiClient;

use serde::Deserialize;

use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::collections::HashSet;

use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;

use crate::utilities::*;

use reqwest::blocking::Client as reqwest_client;
use reqwest::header::{self, HeaderMap, HeaderValue};

use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct HerokuApp {
    id: String,
    name: String,
    released_at: String,
    web_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubResponse {
    sha: String,
}

#[derive(Debug)]
struct GitHubClient {
    client: reqwest_client,
    headers: HeaderMap,
}

impl GitHubClient {
    pub fn new(auth_token: String) -> Self {
        let github_client = reqwest_client::new();

        let mut headers = HeaderMap::new();
        let accept = HeaderValue::from_str("application/vnd.github.v3+json");
        headers.insert(header::ACCEPT, accept.unwrap());

        let auth = HeaderValue::from_str(&format!("token {}", auth_token));
        headers.insert(header::AUTHORIZATION, auth.unwrap());

        // Required for the GitHub API
        // https://developer.github.com/v3/#user-agent-required
        let useragent = HeaderValue::from_str("rust-lang/crates-io-ops-bot");
        headers.insert(header::USER_AGENT, useragent.unwrap());

        GitHubClient {
            client: github_client,
            headers,
        }
    }
}

// Get app by name or id
#[command]
#[num_args(1)]
#[description = "Gets information about an individual Heroku app"]
#[example = "~get_app app_name_or_id"]
pub fn get_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let app = heroku_client(ctx).request(&apps::AppDetails {
        app_id: app_name.clone(),
    })?;

    msg.reply(&ctx, app_info_response(app))?;

    let formations = heroku_client(ctx).request(&formations::FormationList { app_id: app_name })?;

    msg.reply(&ctx, app_formations_response(formations))?;

    Ok(())
}

// App config variables that can be updated through Discord
const AUTHORIZED_CONFIG_VARS: &[&str] = &["FOO"];

// Get app by name or id
#[command]
#[num_args(3)]
#[description = "Updates an environmental variable (only if it is included in the AUTHORIZED_CONFIG_VARS constant"]
#[example = "~update_app_config app_name_or_id ENV_VAR value"]
#[example = "~update_app_config my_app FOO bar"]
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

        let _response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: config_var.clone(),
        })?;

        msg.reply(ctx, format!("Config Var has been updated {:?}", config_var))?;
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
#[description = "Block an IP address"]
#[example = "~block_ip app_name_or_id ip_address_to_block"]
#[example = "~block_ip my_app 123.4.5.67"]
pub fn block_ip(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ip_addr = args
        .single::<String>()
        .expect("You must include an IP address to block");

    let current_config_vars =
        heroku_client(ctx).request(&config_vars::AppConfigVarDetails { app_id: &app_name })?;

    // If the BLOCKED_IPS environmental variable does not
    // currently exist, create it
    if !blocked_ips_exist(&current_config_vars) {
        let _response = heroku_client(&ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: empty_config_var(),
        })?;

        msg.reply(
            &ctx,
            format!(
                "The {} environmental variable has been created for {}",
                BLOCKED_IPS_ENV_VAR, app_name
            ),
        )?;
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

        let _response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
            app_id: &app_name,
            params: updated_config_var,
        })?;

        msg.reply(ctx, format!("IP address {} has been blocked", ip_addr))?;
    };

    Ok(())
}

#[command]
#[num_args(2)]
#[description = "Unblock an IP address"]
#[example = "~unblock_ip app_name_or_id ip_address_to_unblock"]
#[example = "~unblock_ip my_app 123.4.5.6"]
pub fn unblock_ip(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let ip_addr = args
        .single::<String>()
        .expect("You must include an IP address to unblock");

    let current_config_vars =
        heroku_client(ctx).request(&config_vars::AppConfigVarDetails { app_id: &app_name })?;

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
            let _response = heroku_client(ctx).request(&config_vars::AppConfigVarDelete {
                app_id: &app_name,
                params: null_blocked_ips_config_var(),
            })?;

            msg.reply(
                ctx,
                format!(
                    "IP address {} has been unblocked, there are now no unblocked IP addresses",
                    ip_addr,
                ),
            )?;
        } else {
            let _response = heroku_client(ctx).request(&config_vars::AppConfigVarUpdate {
                app_id: &app_name,
                params: blocked_ips_config_var(blocked_ips_set),
            })?;

            msg.reply(ctx, format!("IP address {} has been unblocked", ip_addr))?;
        };
    }

    Ok(())
}

#[command]
#[num_args(4)]
#[description = "Scales a formation of dynos within a Heroku application"]
#[example = "~scale_app your_app_name_or_id name_of_formation number_of_dynos_to_scale_to size_of_dyno"]
#[example = "~scale_app my_app web 3 standard-1X"]
pub fn scale_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let formation_name = args
        .single::<String>()
        .expect("You must include a formation name");

    let quantity = args.single::<i32>().expect("You must include a quantity");

    let size = args.single::<String>().expect("You must include a size");

    let formation = heroku_client(ctx).request(&formations::FormationUpdate {
        app_id: app_name.clone(),
        formation_id: formation_name,
        params: formations::FormationUpdateParams {
            quantity: Some(quantity),
            size: Some(size),
        },
    })?;

    msg.reply(ctx, formation_updated_response(app_name, formation))?;

    Ok(())
}

// Get app by name or id
#[command]
#[num_args(1)]
#[description = "Get a list of releases for a Heroku app"]
#[example = "~get_app_releases app_name_or_id"]
pub fn get_app_releases(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let releases = heroku_client(ctx).request(&releases::ReleaseList { app_id: app_name })?;

    msg.reply(ctx, releases_response(releases))?;

    Ok(())
}

#[command]
#[num_args(2)]
#[description = "Rollback an app to the code associated with a previous release"]
#[example = "~rollback_app app_name version-to-rollback-to"]
#[example = "~rollback_app my_app v5"]
#[example = "~rollback_app my_app 5"]
pub fn rollback_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let version_to_rollback_to = args
        .single::<String>()
        .expect("You must include the version to roll back to");

    let _response = heroku_client(ctx).request(&releases::ReleaseRollback {
        app_id: app_name.clone(),
        params: releases::ReleaseRollbackParams {
            release: version_to_rollback_to.clone(),
        },
    })?;

    msg.reply(
        ctx,
        format!(
            "App {} was successfully rolled back to the code at {}",
            app_name, version_to_rollback_to
        ),
    )?;

    Ok(())
}

#[command]
#[description = "Get all apps associated with your Heroku account"]
#[example = "~get_apps"]
pub fn get_apps(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let apps = heroku_client(ctx).request(&apps::AppList {})?;

    msg.reply(ctx, apps_response(apps))?;

    Ok(())
}

#[command]
#[num_args(1)]
#[description = "Restart all dynos associated with an app"]
#[example = "~restart_app app_name_or_id"]
pub fn restart_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let _response = heroku_client(ctx).request(&dynos::DynoAllRestart {
        app_id: app_name.clone(),
    })?;

    msg.reply(
        ctx,
        format!("All dynos in {} have been restarted.", app_name),
    )?;

    Ok(())
}

#[command]
#[num_args(2)]
#[description = "Deploy an app from a github reference (branch name, partial sha, or full sha)"]
#[example = "~deploy_app app_name_or_id branch_commit_id_or_sha"]
#[example = "~deploy_app my_app master"]
pub fn deploy_app(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let app_name = args
        .single::<String>()
        .expect("You must include an app name");

    let git_ref = args
        .single::<String>()
        .expect("You must include a git ref to deploy");

    let new_github_client = GitHubClient::new(bot_config(ctx).github_token.to_string());

    let github_request = new_github_client
        .client
        .get(&commit_info_url(ctx, git_ref))
        .headers(new_github_client.headers);
    let github_response = github_request
        .send()
        .and_then(|res| res.error_for_status())?;
    let response_text = github_response.text().unwrap();

    let github_json: GitHubResponse = serde_json::from_str(&response_text).unwrap();
    let git_sha = github_json.sha;

    let build = heroku_client(ctx).request(&builds::BuildCreate {
        app_id: app_name.clone(),
        params: builds::BuildCreateParams {
            buildpacks: None,
            source_blob: builds::SourceBlobParam {
                checksum: None,
                url: source_url(&ctx, &git_sha),
                version: Some(git_sha.to_string()),
            },
        },
    })?;

    msg.reply(&ctx, build_response(&app_name, &build))?;

    let build_pending = AtomicBool::new(true);

    let build = heroku_client(ctx).request(&builds::BuildDetails {
        app_id: app_name.clone(),
        build_id: build.clone().id,
    })?;

    let build_status = Arc::new(Mutex::new(build.clone().status));
    let build_status_value = Arc::clone(&build_status);

    let build_id = Arc::new(Mutex::new(build.clone().id));
    let build_id_value = Arc::clone(&build_id);

    let mut sched = JobScheduler::new();

    let build_check_interval = bot_config(&ctx).build_check_interval.clone();

    // Set up job to periodically check if the build is complete
    sched.add(Job::new(
        job_interval(build_check_interval).parse().unwrap(),
        || {
            let result = heroku_client(ctx).request(&builds::BuildDetails {
                app_id: app_name.clone(),
                build_id: build.clone().id,
            });

            match result {
                Ok(_build) => {},
                Err(e) => {
                    println!("An error occured when trying to get the build status for {}: {}", build_id_value.lock().unwrap(), e);
                }
            }

            let mut value = build_status_value.lock().unwrap();
            *value = build.clone().status;
            std::mem::drop(value);

            if *build_status.lock().unwrap() != "pending" {
                build_pending.store(false, Ordering::Relaxed);
            }
        },
    ));

    // Set up job to periodically display a build status message in Discord
    let build_id = build.clone().id;
    let context = ctx.clone();
    let build_message_display_interval = bot_config(&ctx).build_message_display_interval.clone();

    sched.add(Job::new(
        job_interval(build_message_display_interval)
            .parse()
            .unwrap(),
        move || {

            // Doing manual error handling because the try (?) operator cannot be 
            // used in a closure (As of July 2020)
            let result = msg.channel_id
                .say(&context, format!("Build {} is still pending...", build_id));

            // Printing to the console because an error cannot be propogated from an closure
            // up to the enclosing function (As of July 2020)
            match result {
                Ok(_result) => {},
                Err(e) => {
                    println!("An error occured when trying to post the build pending message for {}: {}", build_id, e);
                }
            }
        },
    ));

    while build_pending.load(Ordering::Relaxed) {
//        let build = heroku_client(ctx).request(&builds::BuildDetails {
//            app_id: app_name.clone(),
//            build_id: build.clone().id,
//        })?;

//        let mut value = build_status_value.lock().unwrap();
//        *value = build.status;
//        std::mem::drop(value);

        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }

    // Release the new build
    let final_build_info_response = heroku_client(ctx).request(&builds::BuildDetails {
        app_id: app_name.clone(),
        build_id: build.clone().id,
    });

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

    msg.reply(
        ctx.clone(),
        format!(
            "App {} commit {} has successfully been released!",
            &app_name, git_sha,
        ),
    )?;

    Ok(())
}

fn app_info_response(app: heroku_rs::endpoints::apps::App) -> String {
    format!(
        "\nApp ID: {}\nApp Name: {}\nReleased At: {}\nWeb URL: {}\n\n",
        app.id,
        app.name,
        app.released_at.unwrap_or_else(|| "never".to_string()),
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
    config_var(blocked_ips_set_string)
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
    parse_config_value_set(blocked_ips_value)
}

fn null_blocked_ips_config_var() -> HashMap<String, Option<String>> {
    let mut config_var = HashMap::new();
    config_var.insert(BLOCKED_IPS_ENV_VAR.to_string(), None);
    config_var
}

fn blocked_ips_exist(config_vars: &HashMap<String, Option<String>>) -> bool {
    config_vars.get(&BLOCKED_IPS_ENV_VAR.to_string()).is_some()
}

fn build_response(app_name: &str, build: &heroku_rs::endpoints::builds::Build) -> String {
    format!(
        "Build in progress for {} (this will take a few minutes)\nBuild ID is {}",
        app_name, build.id,
    )
}

fn commit_info_url(ctx: &Context, git_ref: String) -> String {
    format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        bot_config(ctx).github_org,
        bot_config(ctx).github_repo,
        git_ref
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

fn job_interval(interval: String) -> String {
    let interval_string = format!("1/{} * * * * *", interval);
    interval_string
}
