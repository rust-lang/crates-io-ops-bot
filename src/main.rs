// This code is heavily based on https://github.com/serenity-rs/serenity/blob/current/examples/01_basic_ping_bot/src/main.rs
// Also inspired by https://github.com/rust-lang/crates-io-ops-bot/pull/7/files

extern crate crates_io_ops_bot;
extern crate dotenv;

fn main() {
    //    let config = crates_io_ops_bot::Config::new().unwrap();

    crates_io_ops_bot::run()
}
