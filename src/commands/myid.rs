use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub fn myid(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("user id {:?}", &msg.author.id.to_string());

    msg.reply(ctx, format!("Here is your user id {}", &msg.author.id.to_string()));

    Ok(())
}
 