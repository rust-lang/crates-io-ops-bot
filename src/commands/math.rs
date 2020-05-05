use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[num_args(2)]
#[description = "Multiplies two numbers"]
#[example = "~multiply 2 5"]
pub fn multiply(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("running multiply command");
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    let _ = msg.channel_id.say(&ctx.http, product);

    Ok(())
}
