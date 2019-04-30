use crate::card;


pub trait Board {
    fn num_cards(&self) -> usize;
    fn cards(&self, index: usize) -> &card::Card3;
    fn cards_mut(&mut self, index: usize) -> &mut card::Card3;
    fn try_get_cards(&self, a: usize, b: usize, c: usize) -> bool {
        let card1 = self.cards(a);
        let card2 = self.cards(b);
        let card3 = self.cards(c);

        return card::Card::is_correct_set(card1, card2, card3);
    }
    fn put_card(&mut self, index: usize, card: card::Card3) {
        self.cards_mut(index).clone_from(&card);
    }
    fn is_no_set(&self) -> bool {
        for i in 0..self.num_cards() {
            for j in i + 1..self.num_cards() {
                for k in j + 1..self.num_cards() {
                    if self.try_get_cards(i, j, k) {
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
    cards: [card::Card3; 12],
}

impl Board for StandardBoard {
    fn cards(&self, index: usize) -> &card::Card3 {
        return &self.cards[index];
    }

    fn cards_mut(&mut self, index: usize) -> &mut card::Card3 {
        return &mut self.cards[index];
    }
    fn num_cards(&self) -> usize {
        return self.cards.len();
    }
}

impl StandardBoard {
    pub fn new() -> StandardBoard {
        return StandardBoard { cards: Default::default() };
    }
}

pub struct MiniBoard {
    cards: [card::Card3; 3],
}

impl Board for MiniBoard {
    fn cards(&self, index: usize) -> &card::Card3 {
        return &self.cards[index];
    }

    fn cards_mut(&mut self, index: usize) -> &mut card::Card3 {
        return &mut self.cards[index];
    }
    fn num_cards(&self) -> usize {
        return self.cards.len();
    }
}

impl MiniBoard {
    pub fn new() -> MiniBoard {
        return MiniBoard { cards: Default::default() };
    }
}