mod card;
mod board;

#[cfg(test)]
mod tests {
    use crate::board::*;
    use crate::card::*;


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
}
