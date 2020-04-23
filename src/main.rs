// This code is heavily based on https://github.com/serenity-rs/serenity/blob/current/examples/01_basic_ping_bot/src/main.rs
// Also inspired by https://github.com/rust-lang/crates-io-ops-bot/pull/7/files

extern crate crates_io_ops_bot;
extern crate dotenv;
use crates_io_ops_bot::config::Config;

fn main() {
    let config = Config::new(
        dotenv::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set"),
        dotenv::var("AUTHORIZED_USERS").expect("AUTHORIZED_USERS must be set"),
        dotenv::var("HEROKU_API_KEY").expect("HEROKU_API_KEY must be set"),
        dotenv::var("BUILD_CHECK_INTERVAL").expect("BUILD_CHECK_INTERVAL must be set"),
    );

    crates_io_ops_bot::run(config)
}
