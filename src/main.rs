#![deny(warnings)]

use serenity::model::prelude::*;
use serenity::prelude::*;
use std::error::Error;
use std::str::FromStr;

use heroku_command::*;

mod heroku_command;

struct Handler {
    app_name: String,
    authorized_users: Vec<UserId>,
}

impl Handler {
    fn is_dm_or_mention_in_ops_channel(&self, context: &Context, msg: &Message) -> bool {
        if msg.is_own(context) {
            return false;
        }

        let current_user = context.cache.read().user.id;
        msg.is_private()
            || msg.mentions_user_id(current_user)
                && msg.channel_id.name(context).unwrap_or_default() == "crates-io-operations"
    }

    fn user_can_execute_command(&self, user: &User, _: &Command) -> bool {
        self.authorized_users.contains(&user.id)
    }

    fn handle_command(&self, context: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        let command = msg.content.parse()?;

        if !self.user_can_execute_command(&msg.author, &command) {
            return Ok(());
        }

        println!(
            "Received command from {}: `{}`",
            msg.author.tag(),
            msg.content
        );

        match command {
            Command::Ping => {
                msg.channel_id.say(&context, "pong")?;
            }

            Command::Restart => {
                msg.channel_id.say(
                    &context,
                    "Running command (there will be no output until the command completes)",
                )?;

                let command = HerokuCommand::run(&self.app_name, "restart", &[])?;
                // FIXME: Use tokio-process once async/await is stable
                for line in command.output_lines() {
                    msg.channel_id.say(&context, line?)?;
                }

                if command.status().success() {
                    msg.channel_id.say(&context, "Command complete")?;
                } else {
                    msg.channel_id.say(&context, "Command failed")?;
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if !self.is_dm_or_mention_in_ops_channel(&context, &msg) {
            return;
        }

        if let Err(e) = self.handle_command(&context, &msg) {
            let _ = msg
                .channel_id
                .say(&context, format_args!("Error running command: {}", e));
        }
    }
}

enum Command {
    Ping,
    Restart,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let args = &*s.split(" ").collect::<Vec<_>>();
        match args {
            ["ping"] => Ok(Command::Ping),
            _ if args.starts_with(&["ping"]) => Err("ping does not take arguments".into()),
            ["restart"] => Ok(Command::Restart),
            _ if args.starts_with(&["restart"]) => Err("restart does not take arguments".into()),
            _ => Err(format!("unrecognized command: {}", s).into()),
        }
    }
}

fn main() {
    let authorized_users = dotenv::var("AUTHORIZED_USERS")
        .unwrap_or_default()
        .split(",")
        .map(|uid| uid.parse().map(UserId))
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid user id");
    let app_name = dotenv::var("APP_NAME").expect("APP_NAME must be set");
    let handler = Handler {
        authorized_users,
        app_name,
    };

    let token = dotenv::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let mut client = Client::new(&token, handler).expect("Could not construct client");
    client.start().expect("Could not start client");
}
