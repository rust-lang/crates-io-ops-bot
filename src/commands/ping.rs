use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::authorizations::users::*;

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let author_name = &msg.author.name;
    println!("Message {:?}", author_name);
    println!("running pong command");

    if is_authorized(author_name.to_string()) {
        msg.reply(ctx, "Pong!")?;
    } else {
        msg.reply(
            ctx,
            format!("{} is not authorized to run this command", msg.author.name),
        )?;
    }

    Ok(())
}
