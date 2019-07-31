extern crate serenity;
extern crate ron;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::collections::HashSet;
use std::iter::FromIterator;
use serenity::model::id::UserId;
mod groups;

struct Handler;
impl EventHandler for Handler{

}

fn main(){
    let mut client = Client::new("",
                Handler).unwrap();
    client.with_framework(groups::add_groups(StandardFramework::new().configure(|c|c.
        owners(HashSet::from_iter(vec![UserId(201745963394531328u64),UserId(310496481435975693u64)]))
        .prefix(""))));
    client.start().unwrap();

}