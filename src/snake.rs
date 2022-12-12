use std::fmt::Display;

use crate::{apple::Apple, board::Board, position::Position};

pub struct Snake<'a> {
    pub body: Vec<Position>,
    pub len: u8,
    pub direction: Direction,
    pub stage: &'a Board,
    pub stomach: Vec<Position>,
}

impl Snake<'_> {
    pub fn new(stage: &Board) -> Snake {
        Snake {
            body: vec![
                Position::new(5, 3),
                Position::new(5, 4),
                Position::new(5, 5),
            ],
            len: 3,
            direction: Direction::Up,
            stage,
            stomach: vec![],
        }
    }

    pub fn head(&self) -> &Position {
        self.body.last().unwrap()
    }

    pub fn process_input(&mut self, input: char) {
        match input {
            'w' => {
                if matches!(self.direction, Direction::Left | Direction::Right) {
                    self.direction = Direction::Up
                }
            }
            'a' => {
                if matches!(self.direction, Direction::Up | Direction::Down) {
                    self.direction = Direction::Left
                }
            }
            's' => {
                if matches!(self.direction, Direction::Left | Direction::Right) {
                    self.direction = Direction::Down
                }
            }
            'd' => {
                if matches!(self.direction, Direction::Up | Direction::Down) {
                    self.direction = Direction::Right
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        let head = self.head();
        let head = *head
            + match self.direction {
                Direction::Up => Position::new(0, 1),
                Direction::Down => Position::new(0, -1),
                Direction::Left => Position::new(-1, 0),
                Direction::Right => Position::new(1, 0),
            };
        self.body.push(head);
        self.grow_if_necessary();
    }

    pub fn consume_apple(&mut self, apple: Apple) {
        self.stomach.push(apple.position)
    }

    fn grow_if_necessary(&mut self) {
        if let Some(pos) = self.stomach.get(0) {
            let after_tail = self.body[self.body.len() - self.len as usize - 1];
            if &after_tail == pos {
                self.len += 1;
                self.stomach.remove(0);
            }
        }
    }
}

impl Display for Snake<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .rev()
                .take(self.len as usize)
                .map(|pos| format!("{}", pos))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
