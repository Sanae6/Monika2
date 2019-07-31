//add commands like usual

use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::command, Command};

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult{
    msg.reply(ctx,"Pong")?;
    Ok(())
}