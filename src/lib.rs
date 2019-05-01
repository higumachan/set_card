pub mod model;
pub mod view;

#[cfg(test)]
mod tests {
    use crate::model::board::*;
    use crate::model::card::*;
    use crate::model::game::*;


    #[test]
    fn try_get_cards() {
        let mut board = StandardBoard::new();
        board.put_card(0, Card3::new(0, 0, 0));
        board.put_card(1, Card3::new(1, 0, 0));
        board.put_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);

        board.put_card(0, Card3::new(0, 0, 0));
        board.put_card(1, Card3::new(1, 1, 0));
        board.put_card(2, Card3::new(2, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), false);

        board.put_card(0, Card3::new(0, 1, 0));
        board.put_card(1, Card3::new(1, 1, 0));
        board.put_card(2, Card3::new(2, 1, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);
    }

    #[test]
    fn try_get_cards4() {
        let mut board = StandardBoard::new();
        board.put_card(0, Card4::new(0, 0, 0, 0));
        board.put_card(1, Card4::new(1, 0, 0, 0));
        board.put_card(2, Card4::new(2, 0, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);

        board.put_card(0, Card4::new(0, 0, 0, 0));
        board.put_card(1, Card4::new(1, 1, 0, 0));
        board.put_card(2, Card4::new(2, 0, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), false);

        board.put_card(0, Card4::new(0, 1, 0, 0));
        board.put_card(1, Card4::new(1, 1, 0, 0));
        board.put_card(2, Card4::new(2, 1, 0, 0));

        assert_eq!(board.try_get_cards(0, 1, 2), true);
    }

    #[test]
    fn is_no_set() {
        let mut board = MiniBoard::new();
        for i in 0..board.num_cards() {
            board.put_card(i, Card3::new(i as u32 % 2, (i + 1) as u32 % 2, (i as u32 / 2) + 3));
        }
        assert_eq!(board.is_no_set(), true);


        board.put_card(0, Card3::new(0, 1, 0));
        board.put_card(1, Card3::new(1, 1, 0));
        board.put_card(2, Card3::new(2, 1, 0));

        assert_eq!(board.is_no_set(), false);
    }

    #[test]
    fn game_initialize() {
        let mut board : MiniBoard<Card3> = MiniBoard::new();
        let mut game = Game::new(&mut board);
    }
}
