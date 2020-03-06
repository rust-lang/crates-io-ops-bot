use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::authorizations::users::*;
use crate::config::Config;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let config = ctx
        .data
        .read()
        .get::<Config>()
        .expect("Expected config")
        .clone();

    if is_authorized(&msg.author.id.to_string(), &*config) {
        msg.reply(ctx, "Pong!")?;
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
