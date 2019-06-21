extern crate serenity;

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
    utils::MessageBuilder,
};

fn main() {
    println!("Hello, world!");

    let mut client =
        Client::new(&env::var("DISCORD_KEY").expect("No Token found"), Handler).expect("Whoopse");

    if let Err(why) = client.start() {
        println!("Error: {:?}", why);
    }
}

enum Commands {

}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _: Context, msg: Message) {
        if msg.content.starts_with("bot") {
            println!(
                "Bot heard in {:?} with the message:\n {:?}",
                msg.channel_id, msg.content
            );

            

            let response = MessageBuilder::new().push("Pong!");
            msg.channel_id.say(&response);
        }
    }

    /* fn message_delete(_ctx: Context, _channel_id: ChannelId, _deleted_message_id: MessageId) {

    } */
}
