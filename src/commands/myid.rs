use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub fn myid(ctx: &mut Context, msg: &Message) -> CommandResult {
    println!("user id {:?}", &msg.author.id.to_string());

    msg.reply(ctx, format!("Here is your user id {}", &msg.author.id.to_string()))?;

    Ok(())
}
 