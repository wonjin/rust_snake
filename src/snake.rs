use std::collections::LinkedList;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x, y: y + 2 });
        body.push_back(Block { x, y: y + 1 });
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn next_head(&self) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let (next_x, next_y) = match self.head_direction() {
            Direction::Down => (head_x, head_y + 1),
            Direction::Up => (head_x, head_y - 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        };
        (next_x, next_y)
    }

    pub fn head_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().expect("No head!");
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, direction: Option<Direction>) {
        match direction {
            Some(dir) => self.direction = dir,
            None => ()
        }

        let head = self.body.front().expect("no snake body");
        let new_block = match self.direction {
            Direction::Down => Block {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Up => Block {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Block {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Block {
                x: head.x + 1,
                y: head.y,
            },
        };
        self.body.push_front(new_block);
        // remove block
        self.tail = self.body.pop_back();
    }

    pub fn overlap_tail(&self, next_x: i32, next_y: i32) -> bool {
        for block in &self.body {
            if block.x == next_x && block.y == next_y {
                return true;
            }
        }
        return false;
    }

    pub fn restore_tail(&mut self) {
        let lost_tail = self.tail.clone().expect("No Tail");
        self.body.push_back(lost_tail);
    }
}