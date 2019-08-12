extern crate ron;
extern crate serde;
extern crate serenity;

use std::collections::HashSet;
use std::fs::{File, metadata};
use std::io::copy;
use std::iter::FromIterator;
use std::process::exit;

use serde::Deserialize;
use serenity::client::{Client, Context};
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use serenity::prelude::EventHandler;
use serenity::model::gateway::{Ready, Activity};
use serenity::model::prelude::Mentionable;

mod groups;
#[derive(Debug, Deserialize)]
struct Config{
    token: String,

}
struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context,data: Ready){
        ctx.set_activity(Activity::listening("for poggers"))
    }
}
fn load_config() -> Result<Config,String>{
    if metadata("moniconfig.ron").is_err(){
        if metadata("defaconfig.ron").is_err(){
            return Err("This isn't the outcome that I wanted, there's no configs to use!".to_string());
        }else{
            copy(&mut File::open("defaconfig.ron").unwrap(),&mut File::create("moniconfig.ron").unwrap()).unwrap();
        }
    }
    let a:Config = ron::de::from_reader(&mut File::open("moniconfig.ron").unwrap()).unwrap();
    Ok(a)
}

fn main(){
    let yayornay = load_config();
    if yayornay.is_err(){
        println!("{}",yayornay.err().unwrap());
        exit(1);
    }else {
        let yes = yayornay.unwrap();
        let mut client = Client::new(yes.token,
                                     Handler).unwrap();
        client.with_framework(groups::add_groups(StandardFramework::new().configure(|c| c.
            owners(HashSet::from_iter(vec![UserId(201745963394531328u64), UserId(310496481435975693u64)]))
            .prefix("$1")
            .on_mention(Some(UserId(490587459269165077))))));
        client.start().unwrap();
    }
}