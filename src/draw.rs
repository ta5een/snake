use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

// `&con.transform`
// ^ should be `con.transform`?
pub fn draw_block(colour: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        colour,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
    );
}

// `&con.transform`
// ^ should be `con.transform`?
pub fn draw_rect(
    colour: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        colour,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (width as f64)],
        con.transform,
        g
    );
}