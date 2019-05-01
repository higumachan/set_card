use crate::model::board;
use crate::model::card;
use rand::{thread_rng, Rng};


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

        return Game { board, bill };
    }

    pub fn try_get_cards(&mut self, a: usize, b: usize, c: usize) -> bool {
        if !self.board.try_get_cards(a, b, c) {
            return false;
        }
        self.board.put_card(a, self.bill.pop().unwrap());
        self.board.put_card(b, self.bill.pop().unwrap());
        self.board.put_card(c, self.bill.pop().unwrap());

        return true;
    }
}
