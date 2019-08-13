use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult,macros::{command}};
use serenity::model::id::UserId;
use std::collections::HashSet;
use serenity::client::bridge::gateway::ShardManager;
use serenity::gateway::Shard;
use ShardManagerContainer;
use serenity::model::user::OnlineStatus::Offline;
use Replie;

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.replie(&ctx,"Seeya!");
        ctx.set_presence(None,Offline);
        manager.lock().shutdown_all();
    } else {
        let _ = msg.reply(&ctx, "There was a problem getting the shard manager");

        return Ok(());
    }

    let _ = msg.reply(&ctx, "Shutting down!");

    Ok(())
}