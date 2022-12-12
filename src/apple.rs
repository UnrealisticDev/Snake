use crate::position::Position;

pub struct Apple {
    pub position: Position,
}

impl Apple {
    pub fn new(position: Position) -> Apple {
        Apple { position }
    }
}
