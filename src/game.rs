use piston_window::*;
use rand::Rng;

use crate::config::Config;
use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Block, Direction, Snake};

pub struct Game<'c> {
  snake: Snake,

  food: Option<Block>,

  width: i32,
  height: i32,

  game_over: bool,
  waiting_time: f64,
  can_restart: bool,
  turbo: bool,
  score: u32,
  config: &'c Config,
}

impl<'c> Game<'c> {
  pub fn new(config: &'c Config) -> Game {
    Game {
      snake: Snake::new(2, 2),
      food: Some(Block { x: 6, y: 4 }),
      width: config.game.width,
      height: config.game.height,
      game_over: false,
      waiting_time: 0.0,
      can_restart: false,
      turbo: false,
      score: 0,
      config,
    }
  }

  pub fn key_pressed(&mut self, key: Key) {
    if self.game_over {
      if self.can_restart {
        self.restart();
      }
      return;
    }

    let dir = match key {
      Key::Up | Key::W => Some(Direction::Up),
      Key::Down | Key::S => Some(Direction::Down),
      Key::Left | Key::A => Some(Direction::Left),
      Key::Right | Key::D => Some(Direction::Right),
      _ => None,
    };
    if matches!(key, Key::Space) {
      self.turbo = true;
    }

    if let Some(dir) = dir {
      if dir == self.snake.head_direction().opposite() {
        return;
      }
    }

    self.update_snake(dir);
  }

  pub fn key_released(&mut self, key: Key) {
    if matches!(key, Key::Space) {
      self.turbo = false;
    }
  }

  pub fn draw(&self, con: &Context, g: &mut G2d) {
    self.snake.draw(con, g);
    // Walls
    const TOP: i32 = 0;
    const LEFT: i32 = 0;
    let width: i32 = self.width;
    let height: i32 = self.height;
    draw_rectangle(self.config.colors.border, TOP, LEFT, width, 1, con, g);
    draw_rectangle(self.config.colors.border, TOP, height - 1, width, 1, con, g);
    draw_rectangle(self.config.colors.border, TOP, LEFT, 1, height, con, g);
    draw_rectangle(
      self.config.colors.border,
      width - 1,
      LEFT,
      1,
      height,
      con,
      g,
    );

    if let Some(ref food) = self.food {
      draw_block(self.config.colors.food, food.x, food.y, con, g);
    };
    if self.game_over {
      draw_rectangle(
        self.config.colors.gameover,
        0,
        0,
        self.width,
        self.height,
        con,
        g,
      );
    }
  }

  pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += if self.turbo {
      delta_time * self.config.game.turbo_multiplier
    } else {
      delta_time
    };
    if self.game_over {
      if self.waiting_time > self.config.game.restart_time {
        // self.restart();
        self.can_restart = true;
      }
      return;
    }

    if self.food.is_none() {
      self.add_food();
    }

    if self.waiting_time > self.config.game.moving_period {
      self.update_snake(None);
    }
  }
  pub fn get_score(&self) -> u32 {
    self.score
  }
  fn check_eating(&mut self) {
    let head = self.snake.head_position();
    if let Some(ref food) = self.food {
      if food.x == head.x && food.y == head.y {
        self.food = None;
        self.score += 1;
        self.snake.restore_tail();
      }
    }
    self.waiting_time = 0.0;
  }

  fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food = Some(Block { x: 6, y: 4 });
    self.game_over = false;
    self.can_restart = false;
    self.turbo = false;
    self.score = 0;
  }

  fn check_snake_alive(&self, dir: Option<Direction>) -> bool {
    let next = self.snake.next_head(dir);
    if self.snake.overlap_tail(&next) {
      return false;
    }

    next.x > 0 && next.y > 0 && next.x < self.width - 1 && next.y < self.height - 1
  }

  fn add_food(&mut self) {
    if self.food.is_some() {
      return;
    }
    let mut rng = rand::thread_rng();

    let food_pos = loop {
      let block = Block {
        x: rng.gen_range(1..self.width - 1),
        y: rng.gen_range(1..self.height - 1),
      };
      if !self.snake.overlap_tail(&block) {
        break block;
      }
    };
    self.food = Some(food_pos);
  }

  fn update_snake(&mut self, dir: Option<Direction>) {
    self.waiting_time = 0.0;
    if self.check_snake_alive(dir) {
      self.snake.move_forward(dir);
      self.check_eating();
    } else {
      self.game_over = true;
    }
  }
}
