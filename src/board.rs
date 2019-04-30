use crate::card;


pub trait Board<Card: card::Card + Clone> {
    fn num_cards(&self) -> usize;
    fn cards(&self, index: usize) -> &Card;
    fn cards_mut(&mut self, index: usize) -> &mut Card;
    fn try_get_cards(&self, a: usize, b: usize, c: usize) -> bool {
        let card1 = self.cards(a);
        let card2 = self.cards(b);
        let card3 = self.cards(c);

        return card::Card::is_correct_set(card1, card2, card3);
    }
    fn put_card(&mut self, index: usize, card: Card) {
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

pub struct StandardBoard<Card: card::Card> {
    cards: [Card; 12],
}

impl<Card: card::Card + Clone> Board<Card> for StandardBoard<Card> {
    fn cards(&self, index: usize) -> &Card {
        return &self.cards[index];
    }

    fn cards_mut(&mut self, index: usize) -> &mut Card {
        return &mut self.cards[index];
    }
    fn num_cards(&self) -> usize {
        return self.cards.len();
    }
}

impl<Card: card::Card + Default> StandardBoard<Card> {
    pub fn new() -> StandardBoard<Card> {
        return StandardBoard { cards: Default::default() };
    }
}

pub struct MiniBoard<Card: card::Card> {
    cards: [Card; 3],
}


impl<Card: card::Card + Clone> Board<Card> for MiniBoard<Card> {
    fn cards(&self, index: usize) -> &Card {
        return &self.cards[index];
    }

    fn cards_mut(&mut self, index: usize) -> &mut Card {
        return &mut self.cards[index];
    }
    fn num_cards(&self) -> usize {
        return self.cards.len();
    }
}

impl<Card: card::Card + Default> MiniBoard<Card> {
    pub fn new() -> MiniBoard<Card> {
        return MiniBoard { cards: Default::default() };
    }
}
