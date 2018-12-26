use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    game_coord as f64 * BLOCK_SIZE
}

pub fn draw_block(c: Color, game_x: i32, game_y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(game_x);
    let gui_y = to_coord(game_y);

    rectangle(c, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rectangle(c: Color, game_x: i32, game_y: i32, game_w: i32, game_h: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(game_x);
    let gui_y = to_coord(game_y);

    rectangle(c,
              [gui_x, gui_y, BLOCK_SIZE * game_w as f64, BLOCK_SIZE * game_h as f64],
              con.transform,
              g);
}

