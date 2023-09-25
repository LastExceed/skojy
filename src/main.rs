#![feature(array_methods)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(return_position_impl_trait_in_trait)]

mod structs;
mod logic;
mod best_columns_by_key;

use std::thread::sleep;
use std::time::Duration;
use rust_socketio::{ClientBuilder, Event, Payload};
use serde_json::json;
use crate::logic::on_game_update;
use crate::structs::GameState;
use crate::structs::Phase::NewRound;

const ROOM: &str = "78";

fn main() {
	println!("go");

	let mut previous_phase = NewRound;

	let _client = ClientBuilder::new("https://skyjo-backend.voltvector.org/")//https://39cc-91-137-119-248.ngrok-free.app/
		.on(Event::Error, |payload, _socket| println!("on error: {:?}", payload))
		.on(Event::Close, |payload, _socket| println!("on close: {:?}", payload))
		.on(Event::Connect, |_payload, raw_client| {
			raw_client
				.emit("join-session", json!(ROOM))
				.expect("emit join-session failed");
		})
		.on("message", |payload, _raw_client| {
			let Payload::String(message) = payload
				else { panic!("message wasnt a string") };
			println!("msg: {}", message);
		})
		.on("clients-in-session", |payload, _raw_client| {
			let Payload::String(s) = payload
				else { panic!("clients-in-session wasnt a string") };
			let _count = s.parse::<i32>().unwrap();

			// if count == 2i32 {
			// 	raw_client
			// 		.emit("new-game", json!({"sessionId":ROOM}))
			// 		.expect("emit new-game failed");
			// }
		})
		.on("game-update", move |payload, raw_client| {
			let Payload::String(j) = payload
				else { panic!("gamestate wasnt a string") };
			let game_state: GameState = serde_json::from_str(&j).expect("failed to parse game state");

			if previous_phase != game_state.phase {
				previous_phase = game_state.phase;
				on_game_update(game_state, raw_client);
			}
		})
		.connect()
		.expect("connect failed");

	sleep(Duration::from_secs(99999));
	println!("fin");
}

//565-dont place 6

//11 6 11   6  //didnt take 6