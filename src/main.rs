extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const COLOUR_BG: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    // Window properties:
    let mut window: PistonWindow = WindowSettings::new(
        "Snake Game",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true)
        .build()
        .unwrap();

    println!("Starting `{}`", window.get_title());
    println!("* Level: 0, Length: 3");

    // Setting the game bounds to window size:
    let mut game = Game::new(width, height);

    use std::path::Path;
    let font = Path::new("assets/mononoki.ttf");
    let factory = window.factory.clone();
    let settings= TextureSettings::new();

    let mut glyphs = Glyphs::new(font, factory, settings).unwrap();

    // Setting the window events:
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g| {
            clear(COLOUR_BG, g);
            game.draw(&c, g, &mut glyphs);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
