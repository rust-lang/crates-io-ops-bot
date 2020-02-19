use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::authorizations::users::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("running pong command");
    
    if is_authorized(msg.author.id.to_string()) {
        msg.reply(ctx, "Pong!")?;
    } else {
        msg.reply(
            ctx,
            format!("{}'s Discord ID is not authorized to run this command", msg.author.name)
        )?;
    }

    Ok(())
}
