use piston_window::types::Color;
use serde_derive::Deserialize;
use std::sync::{Arc, RwLock};

// Allow accessing config from multiple files.
thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}
impl Config {
  pub fn current() -> Arc<Config> {
    CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
  }
  pub fn make_current(self) {
    CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
  }
}

// fn main() {
//   Config { debug_mode: true }.make_current();
//   if Config::current().debug_mode {
//     // do something
//   }
// }

#[derive(Deserialize, Default)]
pub struct Config {
  pub version: u8,
  pub game: GameConfig,
  pub colors: ColorConfig,
  pub scores: ScoresConfig,
}

#[derive(Deserialize, Default)]
pub struct GameConfig {
  pub width: i32,
  pub height: i32,
  pub block_size: f64,
  pub moving_period: f64,
  pub restart_time: f64,
  pub turbo_multiplier: f64,
}

#[derive(Deserialize, Default)]
pub struct ColorConfig {
  pub board: Color,
  pub border: Color,
  pub food: Color,
  pub gameover: Color,
  pub snake: Color,
}

#[derive(Deserialize, Default)]
pub struct ScoresConfig {
  pub last: u32,
  pub top: Vec<u32>,
}

impl Config {
  pub fn from(file_name: &str) -> Config {
    let config = std::fs::read_to_string(file_name).expect("Unable to read configuration file.");
    let config: Config = toml::from_str(&config).expect("Unable to parse configuration file.");
    config
  }
}
