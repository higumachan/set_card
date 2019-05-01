use crate::model::board;
use crate::model::card;
use rand::{thread_rng, Rng};


pub struct Game<'a, Card: card::Card + Clone, Board: board::Board<Card>> {
    board: &'a mut Board,
    bill: Vec<Card>,
}


impl<'a, Card: card::Card + Clone, Board: board::Board<Card>> Game<'a, Card, Board> {
    pub fn new(board: &mut Board) -> Game<Card, Board> {
        let mut rng = thread_rng();

        let mut bill = Card::all();
        rng.shuffle(&mut bill);

        return Game { board, bill };
    }
}
