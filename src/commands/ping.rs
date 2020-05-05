use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "Returns a pong message"]
#[usage = "Play ping pong"]
#[example = "~ping"]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
