extern crate serenity;

use std::env;

use serenity::{
	model::{channel::Message, gateway::Ready, id::ChannelId},
	prelude::*,
	utils::MessageBuilder,
};

const CAPTIVE_ROOM: u64 = 442835683086827520;
const CAPTCHA_BOT: u64 = 512333785338216465;

fn main() {
	println!("Hello, world!");

	let mut client =
		Client::new(&env::var("DISCORD_KEY").expect("No Token found"), Handler).expect("Whoopse");

	if let Err(why) = client.start() {
		println!("Error: {:?}", why);
	}
}

enum Commands {}


struct Handler;

impl EventHandler for Handler {
	fn message(&self, _: Context, msg: Message) {
		// Delete all non-bot spam in captive_room
		if msg.channel_id.0 == CAPTIVE_ROOM
			&& msg.author.id.0 != CAPTCHA_BOT
			&& msg.content != "!verify"
		{
			msg.delete();
		}
	}

	/* fn message_delete(_ctx: Context, _channel_id: ChannelId, _deleted_message_id: MessageId) {

	} */
}
