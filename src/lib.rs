mod set_card {

    fn triple_equal(a: u32, b: u32, c: u32) -> bool {
        return a == b && b == c;
    }

    fn triple_not_equal(a: u32, b: u32, c: u32) -> bool {
        return a != b && b != c && a != c;
    }



    pub trait Board {
        fn num_cards(&self) -> usize;
        fn cards(&self, index: usize) -> &Card3;
        fn cards_mut(&mut self, index: usize) -> &mut Card3;
        fn try_get_cards(&self, a: usize, b: usize, c: usize) -> bool {
            let card1 = self.cards(a);
            let card2 = self.cards(b);
            let card3 = self.cards(c);

            return (triple_equal(card1.shape, card2.shape, card3.shape) || triple_not_equal(card1.shape, card2.shape, card3.shape)) &&
                triple_equal(card1.color, card2.color, card3.color) || triple_not_equal(card1.color, card2.color, card3.color) &&
                triple_equal(card1.number, card2.number, card3.number) || triple_not_equal(card1.number, card2.number, card3.number)
        }
        fn set_card(&mut self, index: usize, card: Card3) {
            self.cards_mut(index).clone_from(&card);
        }
        fn is_no_set(&self) -> bool {
            for i in 0..self.num_cards() {
                for j in i+1..self.num_cards() {
                    for k in j+1..self.num_cards() {
                        if self.try_get_cards(i, j , k) {
                            println!("{} {} {}", i, j, k);
                            return false;
                        }
                    }
                }
            }
            return true;
        }
    }

    pub struct StandardBoard {
        cards: [Card3; 12],
    }

    impl Board for StandardBoard {
        fn cards(&self, index: usize) -> &Card3 {
            return &self.cards[index];
        }

        fn cards_mut(&mut self, index: usize) -> &mut Card3 {
            return &mut self.cards[index];
        }
        fn num_cards(&self) -> usize {
            return self.cards.len();
        }
    }

    impl StandardBoard {
        pub fn new() -> StandardBoard {
            return StandardBoard {cards: Default::default()};
        }
    }

     pub struct MiniBoard {
        cards: [Card3; 3],
    }

    impl Board for MiniBoard {
        fn cards(&self, index: usize) -> &Card3 {
            return &self.cards[index];
        }

        fn cards_mut(&mut self, index: usize) -> &mut Card3 {
            return &mut self.cards[index];
        }
        fn num_cards(&self) -> usize {
            return self.cards.len();
        }
    }

    impl MiniBoard {
        pub fn new() -> MiniBoard {
            return MiniBoard {cards: Default::default()};
        }
    }

    #[derive(Default, Copy, Clone)]
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
    fn try_get_cards() {
        let mut board = StandardBoard::new();
        board.set_card(0, Card3::new(0, 0, 0));
        board.set_card(1, Card3::new(1, 0, 0));
        board.set_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);

        board.set_card(0, Card3::new(0, 0, 0));
        board.set_card(1, Card3::new(1, 1, 0));
        board.set_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), false);

        board.set_card(0, Card3::new(0, 1, 0));
        board.set_card(1, Card3::new(1, 1, 0));
        board.set_card(2, Card3::new(2, 1, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);


    }

    #[test]
    fn is_no_set() {
        let mut board = MiniBoard::new();
        for i in 0..board.num_cards() {
            board.set_card(i, Card3::new(i as u32 % 2, (i + 1) as u32 % 2, (i as u32 / 2) + 3));
        }
        assert_eq!(board.is_no_set(), true);


        board.set_card(0, Card3::new(0, 1, 0));
        board.set_card(1, Card3::new(1, 1, 0));
        board.set_card(2, Card3::new(2, 1, 0));

        assert_eq!(board.is_no_set(), false);
    }
}
