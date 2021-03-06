use crate::snake::{Direction, Snake};
use crate::draw::*;
use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BOARD_COLOR: Color = [0.0, 0.8, 0.0, 1.0];
const SNAKE_COLOR: Color = [0.0, 0.0, 0.8, 1.0];
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

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
            food_x: 5,
            food_y: 5,
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
            _ => None,
        };

        // 만약 내가 이후에 dir을 쓴다고 한다면 reference로 받아야 함
        if &self.snake.head_direction().opposite() == dir.as_ref().unwrap() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x as i32, self.food_y as i32, c, g);
        }

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, c, g);
        }

        draw_rectangle(BOARD_COLOR, 0, 0, 1, self.height, c, g);
        draw_rectangle(BOARD_COLOR, self.width - 1, 0, 1, self.height, c, g);
        draw_rectangle(BOARD_COLOR, 0, 0, self.width, 1, c, g);
        draw_rectangle(BOARD_COLOR, 0, self.height - 1, self.width, 1, c, g);

        // draw snake
        let snake_body = self.snake.get_body();
        for block in snake_body {
            draw_block(SNAKE_COLOR, block.x, block.y, c, g);
        }
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
        next_x >= 0 && next_y >= 0 && next_x < self.width && next_y < self.height
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
            // TODO 게임 완료 분기 넣어야 함
        }
        else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    // TODO new 메서드랑 동일하게 하는 장치가 필요함
    fn restart(&mut self) {
        self.food_exist = true;
        self.food_x = 5;
        self.food_y = 5;
        self.game_over = false;
        self.waiting_time = 0.0;
        self.snake = Snake::new(3, 3);
    }
}
