extern crate piston_window;
extern crate rand;
extern crate serde;
extern crate serde_derive;
extern crate toml;

mod config;
mod draw;
mod game;
mod score;
mod snake;
mod text_renderer;

use piston_window::*;

use crate::config::Config;
use crate::draw::Draw;
use crate::game::Game;

fn main() {
    Config::from("Game.toml").make_current();
    let config = Config::current();
    let draw = Draw::new();
    let width = config.game.width;
    let height = config.game.height;

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [draw.to_coord_u32(width), draw.to_coord_u32(height)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut text_renderer = text_renderer::Renderer::new(&mut window);
    let mut game = Game::new();
    let header_props = text_renderer::TextProps {
        position: (50.0, 18.0),
        color: [0.0, 1.0, 1.0, 1.0],
        size: 12,
    };
    let footer_props = text_renderer::TextProps {
        position: (
            width as f64 * config.game.block_size - 180.0,
            height as f64 * config.game.block_size - 6.0,
        ),
        color: [0.41, 0.25, 0.85, 1.0],
        size: 8,
    };
    let score_props = text_renderer::TextProps {
        position: (10.0, height as f64 * config.game.block_size - 6.0),
        color: [0.41, 0.25, 0.85, 1.0],
        size: 8,
    };
    let mut strings = vec![];
    text_renderer.add_text(&mut strings, "Snake v0.0.1", &header_props);
    text_renderer.add_text(&mut strings, "By Lucas Caro", &footer_props);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            game.key_released(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(config.colors.board, g);
            game.draw(&c, g);
            text_renderer.draw(&strings, &c, g, device);
            let score = format!("Score: {}", game.get_score());
            text_renderer.draw_text(&score, &score_props, &c, g);
            text_renderer.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
