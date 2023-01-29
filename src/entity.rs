
use crate::*;

pub enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub name: &'static str,
    pub position: Point,
}


impl Entity {
    pub fn new(name: &'static str, position: Point) -> Entity {
        Entity { name, position }
    }

    pub fn move_entity(&self, direction: console::Key) -> Entity {
        match direction {
            ArrowRight => Entity {
                name: self.name,
                position: (self.position.0 + 1, self.position.1),
            },
            ArrowLeft => Entity {
                name: self.name,
                position: (self.position.0 - 1, self.position.1),
            },
            ArrowDown => Entity {
                name: self.name,
                position: (self.position.0, self.position.1 + 1),
            },
            ArrowUp => Entity {
                name: self.name,
                position: (self.position.0, self.position.1 - 1),
            },
            _ => *self,
        }
    }
}