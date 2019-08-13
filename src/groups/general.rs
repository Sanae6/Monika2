//add commands like usual
extern crate reqwest;

use serenity::prelude::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, macros::{
    help,
    command
}, Args, HelpOptions, CommandGroup, help_commands};
use serenity::model::id::UserId;
use std::collections::HashSet;

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult{
    msg.reply(ctx,"Pong")?;
    Ok(())
}

#[help]
#[individual_command_tip =
"Hello! I'm in early stages of developement, so I don't have much to offer right now sadly."]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]

fn my_help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners)
}