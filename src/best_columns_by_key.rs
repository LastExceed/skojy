use std::slice::Iter;
use itertools::Itertools;
use crate::structs::CardValue;

type Card = Option<CardValue>;
type Column = [Card; 3];
//, I: Iterator<Item=Card>
pub trait BestColumnsByKeyExt: IntoIterator<Item=(usize, Column)> {
	fn best_columns_by_key<O: Ord>(self, f: fn(Iter<Card>) -> O) -> Vec<(usize, Column)>;
}

impl<'c, T: IntoIterator<Item=(usize, Column)>> BestColumnsByKeyExt for T {
	fn best_columns_by_key<O: Ord>(self, f: fn(Iter<Card>) -> O) -> Vec<(usize, Column)> {
		self.into_iter()
			.max_set_by_key(|(_index, column): &(_, Column)| {
				f(column.iter())
			})
	}
}