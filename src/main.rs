use std::io;
use set_card::*;
use set_card::model::card::{Card3, Card4};
use set_card::view::views::{CardView, BoardView, GameView};
use set_card::model::board::{StandardBoard, Board};
use set_card::model::game::Game;

fn main() {
    let mut board: StandardBoard<Card4> = StandardBoard::new();
    let mut game = Game::new(&mut board);

    loop {
        game.draw();

        println!("please your choice:");
        let mut buf= String::new();
        io::stdin().read_line(&mut buf).expect("Failed to read line");

        let numbers: Vec<&str> = buf.split_ascii_whitespace().collect();

        let numbers: Vec<usize> = numbers.into_iter().map(|x| {
            return x.trim().parse().unwrap();
        }).collect();

        if numbers.len() != 3 {
            continue;
        }

        if game.try_get_cards(numbers[0], numbers[1], numbers[2]) {
            println!("get 3 cards!!!");
        }
    }
}