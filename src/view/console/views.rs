use colored::*;
use crate::view::views;
use crate::model;
use crate::model::board::Board;

impl views::CardView for model::card::Card3 {
    fn draw(&self){
        let s = match self.shape {
            0 => "*",
            1 => "%",
            2 => "&",
            _ => "_",
        }.repeat((self.number + 1) as usize);

        let s = match self.color {
            0 => s.red(),
            1 => s.blue(),
            2 => s.green(),
            _ => s.white(),
        };

        print!("{:^4}", s);
    }
}

impl<Card: model::card::Card + views::CardView + Clone> views::BoardView for model::board::StandardBoard<Card> {
    fn draw(&self) {
        for i in 0..self.num_cards() {
            self.card(i).draw();
        }
    }
}