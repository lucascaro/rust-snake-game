use piston_window::types::Color;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub version: u8,
  pub game: GameConfig,
  pub colors: ColorConfig,
  pub scores: ScoresConfig,
}

#[derive(Deserialize)]
pub struct GameConfig {
  pub width: i32,
  pub height: i32,
  pub block_size: f64,
  pub moving_period: f64,
  pub restart_time: f64,
  pub turbo_multiplier: f64,
}

#[derive(Deserialize)]
pub struct ColorConfig {
  pub board: Color,
  pub border: Color,
  pub food: Color,
  pub gameover: Color,
  pub snake: Color,
}

#[derive(Deserialize)]
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
