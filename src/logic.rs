use std::thread;
use std::time::Duration;
use itertools::Itertools;
use rust_socketio::RawClient;
use serde_json::json;
use crate::{best_columns_by_key::BestColumnsByKeyExt, structs::Deck};
use crate::structs::{CardValue, GameState};
use crate::structs::Phase::*;

const LOW_LIMIT: CardValue = 2;

pub fn on_game_update(game_state: GameState, socket: RawClient) {
	let me = &game_state.players[0];

	println!("my turn: {}", me.playersTurn);
	if !me.playersTurn {
		return;
	}

	println!("{:?}", game_state.phase);
	#[deny(non_snake_case)]
	match game_state.phase {
		NewRound => {},
		RevealTwoCards => {
			thread::sleep(Duration::from_millis(100));
			socket.emit("click-card", json!([0, 0])).unwrap();
			thread::sleep(Duration::from_millis(100));
			socket.emit("click-card", json!([1, 0])).unwrap();
		}
		PickUpCard => {
			//stdin().read_line(&mut input).unwrap();
			if let Some(_destination) = find_placement_spot(&me.deck, *game_state.discardPile.last().expect("discard pile  was empty")) {
				//todo: carry over `destination` to `PlaceCard` phase to prevent computing a 2nd time
				println!("pick up - from discard");
				socket.emit("click-discard-pile", json!("take discard pile card")).unwrap();
			} else {
				println!("pick up - from new");
				socket.emit("draw-from-card-stack", json!("draw card")).unwrap();
			}
		}
		PlaceCard => {
			//stdin().read_line(&mut input).unwrap();
			if let Some(destination) = find_placement_spot(&me.deck, me.cardCache.unwrap()) {
				println!("place card - at {:?}", destination);
				socket.emit("click-card", json!(destination)).unwrap();
			} else {
				println!("place card - discard");
				socket.emit("click-discard-pile", json!("take discard pile card")).unwrap(); //pb pls
			}
		}
		RevealCard => {
			//stdin().read_line(&mut input).unwrap();
			let (col_index, best_column) = me.deck
				.iter()
				.cloned()
				.enumerate()
				.best_columns_by_key(|column| column//most concealed
					.filter(|card| card.is_none())
					.count()
				)
				.best_columns_by_key(|column| column//most revealed distinct
					.unique()//no problem with counting concealed as well, since every eligible column has them
					.count()
				)
				.best_columns_by_key(|column| column//highest revealed
					.max()//TIL `Option<T>` is comparable (None is smallest)
					.cloned()
				)
				.pop()//if its still a tie then it doesnt matter
				.unwrap();//the "best by" filters always leave at least 1 column behind

			let row_index = best_column.iter().position(Option::is_none).unwrap();
			let index = [col_index, row_index];

			println!("reveal card - {:?}", index);
			socket.emit("click-card", json!(index)).unwrap();
		}
		RevealedLastCard => {}
		GameEnded => {}
	}
	println!("end");
}

fn find_placement_spot(my_deck: &Deck, buffer: CardValue) -> Option<[usize; 2]> {
	my_deck
		.iter()
		.map(|column| {
			if column.windows(2).all(|window| window[0] == window[1]) && column[0].is_some() {
				return None;
			}

			let (matches, others): (Vec<_>, Vec<_>) =
				column
					.iter()
					.enumerate()
					.partition(|(_row_index, card_value)| **card_value == Some(buffer));

			let match_count = matches.len() as i32;

			if match_count == 1 {
				if others[0].1 == others[1].1 {
					if others[0].1.is_some() {
						//5ss 5
						//5bb 5
						//todo: reconsider in endgame
						return None;
					}
				} else {
					let all_known_and_low = column
						.iter()
						.all(|card|
							card
								.map(|v| v <= LOW_LIMIT)
								.unwrap_or(false)
						);
					if all_known_and_low { //implies that the matched card, and thus the buffer, is low as well
						return None;
					}
					//5_x 5
					//5__ 5
					//todo: reconsider in endgame
				}
			}

			if match_count == 0 {
				if buffer > LOW_LIMIT {
					return None;
				}

				if column.iter().all(Option::is_none) {
					return Some((13, 0));
				}

				let low_count = column
					.iter()
					.filter(|card| {
						card.map(|v| v <= LOW_LIMIT)
							.unwrap_or(false)
					})
					.count() as i32;

				//todo: 2_2 1
				return Some(find_best_spot_in_column(low_count % 3, column.iter().enumerate()));
			}

			Some(find_best_spot_in_column(match_count, others))
		})
		.enumerate()
		.filter_map(|(col_index, verdict)| verdict.map(|v| (col_index, v)))
		.max_by_key(|(_col_index, (score, _row_index))| *score)
		.map(|(col_index, (_score, row_index))| [col_index, row_index])
}

fn find_best_spot_in_column(
	multiplier: i32,
	candidates: impl IntoIterator<Item=(usize, &Option<CardValue>)>
) -> (i32, usize) {
	let (row_index, replaced_card) = candidates
		.into_iter()
		.max_by_key(|(_row_index, card)| card.unwrap_or(LOW_LIMIT))
		.unwrap();

	let score = multiplier * 100 + replaced_card.unwrap_or(LOW_LIMIT) as i32;
	(score, row_index)
}