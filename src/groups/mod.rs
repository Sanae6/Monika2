use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use crate::groups::general::PING_COMMAND;
use crate::groups::owner::QUIT_COMMAND;
pub mod general;
pub mod owner;
use groups::general::MY_HELP;
/*
Sadly this entire file is not the moderation group's command definitions
it's just the name of the file needed to tie the groups together, hence there being several group definitions below this comment
*/
group!({
    name: "general",
    options: {},
    commands: [PING]
});
group!({
    name: "fun",
    options: {},
    commands: []
});
group!({
    name: "owner",
    options: {},
    commands: [QUIT]
});

pub fn add_groups(f: StandardFramework) ->StandardFramework{
    f.group(&GENERAL_GROUP).group(&FUN_GROUP).group(&OWNER_GROUP).help(&MY_HELP)
}