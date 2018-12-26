extern crate piston_window;
extern crate rand;

mod game;
mod snake;
mod draw;

use piston_window::types::Color;
use piston_window::*;

use self::game::Game;
use self::draw::to_coord;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (w, h) = (10,10);
    let (gui_w_u32, gui_h_u32) = (to_coord(w) as u32, to_coord(h) as u32);
    let mut window: PistonWindow = WindowSettings::new("Snake", [gui_w_u32, gui_h_u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(w, h);

    // Todo: Some 에 대해 좀 더 잘 알아 볼것!
    // event와 key가 변수로 선언된것은 알겠음
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Todo: Clouser에 대해서 알아봐야 함
        // 람다 같은 것이라고 생각함
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
