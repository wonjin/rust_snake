use piston_window::*;
use snake::Direction;

pub struct Game {
    food_exist: bool,
    food_x: u32,
    food_y: u32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            food_exist: true,
            food_x: 10,
            food_y: 10,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        self.update_snake();
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {}

    pub fn update(&mut self, delta_time: f64) {}

    fn check_eating() {}

    fn check_if_snake_alive() {}

    fn add_food() {}

    fn update_snake(&mut self) {}

    fn restart(&mut self) {}
}
