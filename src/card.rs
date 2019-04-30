#[derive(Default, Copy, Clone)]
pub struct Card3 {
    pub shape: u32,
    pub color: u32,
    pub number: u32,
}

impl Card3 {
    pub fn new(shape: u32, color: u32, number: u32) -> Card3
    {
        return Card3{shape: shape, color: color, number: number}
    }
}