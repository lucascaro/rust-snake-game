use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use crate::config::Config;

pub struct Draw {
  block_size: f64,
}

impl Draw {
  pub fn new() -> Draw {
    let config = Config::current();
    Draw {
      block_size: config.game.block_size,
    }
  }

  pub fn to_coord(&self, game_coord: i32) -> f64 {
    (game_coord as f64) * self.block_size
  }

  pub fn to_coord_u32(&self, game_coord: i32) -> u32 {
    self.to_coord(game_coord) as u32
  }

  pub fn block(&self, color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = self.to_coord(x);
    let gui_y = self.to_coord(y);

    rectangle(
      color,
      [
        gui_x + 1.0,
        gui_y + 1.0,
        self.block_size - 2.0,
        self.block_size - 2.0,
      ],
      con.transform,
      g,
    )
  }

  pub fn rectangle(
    &self,
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
  ) {
    let gui_x = self.to_coord(x);
    let gui_y = self.to_coord(y);

    rectangle(
      color,
      [
        gui_x,
        gui_y,
        self.block_size * (width as f64),
        self.block_size * (height as f64),
      ],
      con.transform,
      g,
    )
  }
}
