use serde::Deserialize;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
pub struct GameState {
	pub sessionId: String,
	pub playerCount: i32,
	pub phase: Phase,
	pub round: i32,
	pub discardPile: Vec<CardValue>,
	pub players: Vec<Player>,
	pub cardStack: CardStack
}

pub type Card = Option<CardValue>;
pub type CardValue = i8;

#[allow(dead_code, non_camel_case_types)]
#[derive(Deserialize)]
pub enum CardColor {
	darkblue,
	lightblue,
	green,
	yellow,
	red,
	black
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CardStack {
	pub cards: Vec<Card>
}

pub type Deck = Vec<[Card; 3]>;

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
pub struct Player {
	pub id: i32,
	pub socketId: String,
	pub name: String,
	pub knownCardPositions: Vec<[bool; 3]>,
	pub playersTurn: bool,
	pub cardCache: Card,
	pub tookDispiledCard: bool,
	pub roundPoints: i32,
	pub totalPoints: i32,
	pub closedRound: bool,
	pub deck: Deck,
	pub place: Option<i32>
}

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
	#[serde(rename(deserialize = "new round"))]
	NewRound,

	#[serde(rename(deserialize = "reveal two cards"))]
	RevealTwoCards,

	#[serde(rename(deserialize = "pick up card"))]
	PickUpCard,

	#[serde(rename(deserialize = "place card"))]
	PlaceCard,

	#[serde(rename(deserialize = "reveal card"))]
	RevealCard,

	#[serde(rename(deserialize = "revealed last card"))]
	RevealedLastCard,

	#[serde(rename(deserialize = "game ended"))]
	GameEnded
}