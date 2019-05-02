use crate::model::board;
use crate::model::card;
use rand::{thread_rng, Rng};
use std::error;
use std::fmt;


#[derive(Debug)]
pub enum TryGetCardsError {
    BillEmpty,
    CanNotGetCard,
}

impl fmt::Display for TryGetCardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TryGetCardsError::BillEmpty => write!(f, "bill is empty"),
            TryGetCardsError::CanNotGetCard => write!(f, "can not get card"),
        }
    }
}

impl error::Error for TryGetCardsError {
    fn description(&self) -> &str {
        match *self {
            TryGetCardsError::BillEmpty => "bill is empty",
            TryGetCardsError::CanNotGetCard => "can not get card",
        }
    }
}

pub struct Game<'a, Card: card::Card + Clone, Board: board::Board<Card>> {
    pub board: &'a mut Board,
    pub bill: Vec<Card>,
}


impl<'a, Card: card::Card + Clone, Board: board::Board<Card>> Game<'a, Card, Board> {
    pub fn new(board: &mut Board) -> Game<Card, Board> {
        let mut rng = thread_rng();

        let mut bill = Card::all();
        rng.shuffle(&mut bill);

        for i in 0..board.num_cards() {
            board.put_card(i, bill.pop().unwrap())
        }

        Game { board, bill }
    }

    pub fn try_get_cards(&mut self, a: usize, b: usize, c: usize) -> Result<(), TryGetCardsError> {
        if !self.board.try_get_cards(a, b, c) {
            return Err(TryGetCardsError::CanNotGetCard);
        }

        if self.bill.len() < 3 {
            return Err(TryGetCardsError::BillEmpty);
        }

        self.board.put_card(a, self.bill.pop().unwrap());
        self.board.put_card(b, self.bill.pop().unwrap());
        self.board.put_card(c, self.bill.pop().unwrap());

        Ok(())
    }
}
