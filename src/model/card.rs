pub trait Card where Self: Sized {
    fn is_correct_set(card1: &Self, card2: &Self, card3: &Self) -> bool;
    fn all() -> Vec<Self>;
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Card3 {
    pub shape: u32,
    pub color: u32,
    pub number: u32,
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Card4 {
    pub shape: u32,
    pub color: u32,
    pub number: u32,
    pub pattern: u32,
}

impl Card for Card3 {
    fn is_correct_set(card1: &Card3, card2: &Card3, card3: &Card3) -> bool {
        (triple_equal(card1.shape, card2.shape, card3.shape) || triple_not_equal(card1.shape, card2.shape, card3.shape)) &&
            triple_equal(card1.color, card2.color, card3.color) || triple_not_equal(card1.color, card2.color, card3.color) &&
            triple_equal(card1.number, card2.number, card3.number) || triple_not_equal(card1.number, card2.number, card3.number)
    }

    fn all() -> Vec<Self> {
        let mut v = Vec::new();
        for color in 0..3 {
            for shape in 0..3 {
                for number in 0..3 {
                    v.push(Self::new(shape, color, number))
                }
            }
        }

        v
    }
}

impl Card3 {
    pub fn new(shape: u32, color: u32, number: u32) -> Card3
    {
        Card3{shape: shape, color: color, number: number}
    }
}

impl Card for Card4 {
    fn is_correct_set(card1: &Card4, card2: &Card4, card3: &Card4) -> bool {
        (triple_equal(card1.shape, card2.shape, card3.shape) || triple_not_equal(card1.shape, card2.shape, card3.shape)) &&
            triple_equal(card1.color, card2.color, card3.color) || triple_not_equal(card1.color, card2.color, card3.color) &&
            triple_equal(card1.number, card2.number, card3.number) || triple_not_equal(card1.number, card2.number, card3.number) &&
            triple_equal(card1.pattern, card2.pattern, card3.pattern) || triple_not_equal(card1.pattern, card2.pattern, card3.pattern)
    }

    fn all() -> Vec<Self> {
        let mut v = Vec::new();
        for color in 0..3 {
            for shape in 0..3 {
                for number in 0..3 {
                    for pattern in 0..3 {
                        v.push(Self::new(shape, color, number, pattern))
                    }
                }
            }
        }

        v
    }
}

impl Card4 {
    pub fn new(shape: u32, color: u32, number: u32, pattern: u32) -> Card4
    {
        Card4{shape, color, number, pattern}
    }
}

fn triple_equal(a: u32, b: u32, c: u32) -> bool {
    a == b && b == c
}

fn triple_not_equal(a: u32, b: u32, c: u32) -> bool {
    a != b && b != c && a != c
}
