use crate::snake::{Direction, Snake};
use piston_window::*;
use rand::{thread_rng, Rng};

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    food_exist: bool,
    food_x: u32,
    food_y: u32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    snake: Snake,
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
            snake: Snake::new(3, 3),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _ => Some(Direction::Up),
        };

        // 만약 내가 이후에 dir을 쓴다고 한다면 reference로 받아야 함
        if &self.snake.head_direction().opposite() == dir.as_ref().unwrap() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        if self.food_exist {
            // draw food
        }

        if self.game_over {
            // draw game over            
        }

        // draw game board

        // draw snake
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exist {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&self) -> bool {
        let (head_x, head_y) = self.snake.head_position();
        head_x as u32 == self.food_x && head_y as u32 == self.food_y
    }

    fn check_if_snake_alive(&self) -> bool {
        // next move에서 블럭이랑 겹치는지 확인
        let (next_x, next_y) = self.snake.next_head();

        // 뱀에 닿으면 false
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // 벽에 닿으면 false
        next_x < 0 || next_y < 0 || next_x >= self.width || next_y >= self.height
    }

    fn add_food(&mut self) {
        // 뱀에 안겹칠 때 까지 랜덤 생성
        let mut rng = thread_rng();
        let mut rand_x = rng.gen_range(1, self.width - 1);
        let mut rand_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(rand_x, rand_y) {
            rand_x = rng.gen_range(1, self.width - 1);
            rand_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = rand_x as u32;
        self.food_y = rand_y as u32;
        self.food_exist = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive() {
            // 앞으로 움직이고, 음식을 먹었는지 확인
            self.snake.move_forward(dir);
            if self.check_eating() {
                // 음식을 먹었다면 꼬리를 복구
                self.snake.restore_tail();
                self.food_exist = false;
            }
        }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.food_exist = true;
        self.food_x = 10;
        self.food_y = 10;
        self.game_over = false;
        self.waiting_time = 0.0;
        self.snake = Snake::new(3, 3);
    }
}
