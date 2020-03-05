pub mod config {
    use serenity::prelude::TypeMapKey;

    #[derive(Debug)]
    pub struct Config {
        pub discord_token: String,
        pub authorized_users: String,
        pub heroku_api_key: String,
    }

    impl Default for Config {
        fn default() -> Self {
            Config {
                discord_token: discord_token(),
                authorized_users: authorized_users(),
                heroku_api_key: heroku_api_key(),
            }
        }
    }

    impl TypeMapKey for Config {
        type Value = Config;
    }

    fn discord_token() -> String {
        dotenv::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set")
    }

    fn authorized_users() -> String {
        dotenv::var("AUTHORIZED_USERS").expect("AUTHORIZED_USERS must be set")
    }

    fn heroku_api_key() -> String {
        dotenv::var("HEROKU_API_KEY").expect("HEROKU_API_KEY must be set")
    }
}
