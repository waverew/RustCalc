pub enum Directions {
    Up,
    Down,
    Left,
    Right
}

pub enum Commands {
    Start,
    Quit,
    Turn
}

pub struct Position {
    x: u16,
    y: u16
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    
}