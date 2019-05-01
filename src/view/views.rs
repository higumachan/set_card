pub trait View {
}

pub trait CardView {
    fn draw(&self);
}

pub trait BoardView  {
    fn draw(&self);
}

pub trait GameView {
    fn draw(&self);
}
