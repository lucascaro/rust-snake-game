use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.21, 0.15, 0.65, 1.0];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn opposite(&self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub x: i32,
  pub y: i32,
}

impl Block {
  pub fn from(dir: &Direction) -> Block {
    match dir {
      Direction::Up => Block { x: 0, y: -1 },
      Direction::Down => Block { x: 0, y: 1 },
      Direction::Left => Block { x: -1, y: 0 },
      Direction::Right => Block { x: 1, y: 0 },
    }
  }
}

impl std::ops::Add<&Block> for Block {
  type Output = Block;

  fn add(self, p: &Block) -> Block {
    Block {
      x: self.x + p.x,
      y: self.y + p.y,
    }
  }
}
pub struct Snake {
  direction: Direction,
  body: LinkedList<Block>,
  tail: Option<Block>,
}

impl Snake {
  pub fn new(x: i32, y: i32) -> Snake {
    let mut body: LinkedList<Block> = LinkedList::new();
    body.push_back(Block { x: x + 2, y });
    body.push_back(Block { x: x + 1, y });
    body.push_back(Block { x, y });
    Snake {
      direction: Direction::Right,
      body,
      tail: None,
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    for block in &self.body {
      draw_block(SNAKE_COLOR, block.x, block.y, con, g)
    }
  }

  pub fn head_position(&self) -> Block {
    let head_block = self.body.front().unwrap();
    Block {
      x: head_block.x,
      y: head_block.y,
    }
  }

  pub fn change_dir(&mut self, dir: Direction) {
    self.direction = dir;
  }

  pub fn move_forward(&mut self, dir: Option<Direction>) {
    if let Some(dir) = dir {
      self.change_dir(dir);
    }

    let last_position = self.head_position();
    let move_direction = Block::from(&self.direction);
    let new_block = last_position + &move_direction;

    self.body.push_front(new_block);
    let removed_block = self.body.pop_back().unwrap();
    self.tail = Some(removed_block);
  }

  pub fn head_direction(&self) -> Direction {
    self.direction
  }

  pub fn next_head(&self, dir: Option<Direction>) -> Block {
    let head_position = self.head_position();
    let moving_dir = match dir {
      Some(d) => d,
      None => self.direction,
    };

    let move_direction = Block::from(&moving_dir);
    head_position + &move_direction
  }

  pub fn restore_tail(&mut self) {
    let blk = self.tail.clone().unwrap();
    self.body.push_back(blk);
  }

  pub fn overlap_tail(&self, next: &Block) -> bool {
    let mut ch = 0;
    for block in &self.body {
      if block == next {
        return true;
      }
      ch += 1;
      if ch == self.body.len() - 1 {
        // Ignore collission with last block.
        break;
      }
    }
    false
  }
}
