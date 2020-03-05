use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::authorizations::users::*;
use crate::config::config::Config;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("running pong command");

    let ctx_clone = ctx.clone();
    let data = ctx_clone.data.read();
    let config = data.get::<Config>().expect("Expected config");

    if is_authorized(&msg.author.id.to_string(), config) {
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
