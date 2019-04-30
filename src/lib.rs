mod set_card {
    const NUM_CARDS:usize = 12;

    pub struct Board {
        cards: [Card3; NUM_CARDS],
    }

    impl Board {
        pub fn try_get_cards(&self, a: u32, b: u32, c: u32) -> bool {
            return true;
        }

        pub fn set_card(&mut self, index: usize, card: Card3) {
            self.cards[index] = card;
        }

        pub fn new() -> Board {
            return Board {cards: [Card3 {shape: 0, color: 0, number: 0}; NUM_CARDS]};
        }
    }

    #[derive(Copy, Clone)]
    pub struct Card3 {
        shape: u32,
        color: u32,
        number: u32,
    }

    impl Card3 {
        pub fn new(shape: u32, color: u32, number: u32) -> Card3
        {
            return Card3{shape: shape, color: color, number: number}
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::set_card::*;

    #[test]
    fn simple_board() {
        let mut board = Board::new();
        board.set_card(0, Card3::new(0, 0, 0));
        board.set_card(1, Card3::new(1, 0, 0));
        board.set_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);

        board.set_card(0, Card3::new(0, 0, 0));
        board.set_card(1, Card3::new(1, 1, 0));
        board.set_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), false);
    }
}
