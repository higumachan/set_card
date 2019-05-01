use set_card::*;
use set_card::model::card::Card3;
use set_card::view::views::{CardView, BoardView};
use set_card::model::board::{StandardBoard, Board};

fn main() {
    println!("Hello world");

    let mut board = StandardBoard::new();
    board.put_card(0, Card3::new(0, 0, 0));
    board.put_card(1, Card3::new(0, 1, 1));
    board.put_card(2, Card3::new(1, 2, 1));
    board.draw();
}