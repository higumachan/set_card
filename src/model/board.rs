use crate::model::card;


pub trait Board<Card: card::Card + Clone> {
    fn num_cards(&self) -> usize;
    fn card(&self, index: usize) -> &Card;
    fn cards_mut(&mut self, index: usize) -> &mut Card;
    fn try_get_cards(&self, a: usize, b: usize, c: usize) -> bool {
        let card1 = self.card(a);
        let card2 = self.card(b);
        let card3 = self.card(c);

        card::Card::is_correct_set(card1, card2, card3)
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
        true
    }
}

pub struct StandardBoard<Card: card::Card> {
    cards: [Card; 12],
}

impl<Card: card::Card + Clone> Board<Card> for StandardBoard<Card> {
    fn card(&self, index: usize) -> &Card {
        &self.cards[index]
    }

    fn cards_mut(&mut self, index: usize) -> &mut Card {
        &mut self.cards[index]
    }
    fn num_cards(&self) -> usize {
        self.cards.len()
    }
}

impl<Card: card::Card + Default> StandardBoard<Card> {
    pub fn new() -> StandardBoard<Card> {
        StandardBoard { cards: Default::default() }
    }
}

pub struct MiniBoard<Card: card::Card> {
    cards: [Card; 3],
}


impl<Card: card::Card + Clone> Board<Card> for MiniBoard<Card> {
    fn card(&self, index: usize) -> &Card {
        &self.cards[index]
    }

    fn cards_mut(&mut self, index: usize) -> &mut Card {
        &mut self.cards[index]
    }
    fn num_cards(&self) -> usize {
        self.cards.len()
    }
}

impl<Card: card::Card + Default> MiniBoard<Card> {
    pub fn new() -> MiniBoard<Card> {
        MiniBoard { cards: Default::default() }
    }
}
