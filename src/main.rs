extern crate piston_window;
extern crate rand;

mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use self::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (w, h) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake", [500u32, 500u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(10,20);

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
