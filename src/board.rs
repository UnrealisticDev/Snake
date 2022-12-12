pub struct Board {
    pub width: u8,
    pub height: u8,
}

impl Board {
    pub fn new(width: u8, height: u8) -> Board {
        Board { width, height }
    }
}
