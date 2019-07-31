use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use crate::groups::general::PING_COMMAND;
pub mod general;
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

pub fn add_groups(f: StandardFramework) ->StandardFramework{
    f.group(&GENERAL_GROUP).group(&FUN_GROUP)
}