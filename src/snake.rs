use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

const SNAKE_COLOUR: Color = [0.00, 0.80, 0.10, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    /// Returns the opposite direction to prevent snake from going back onto itself.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake {
    /// Initialises a snake of 3 blocks (with no tail) moving to the right.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    /// Draws the snake.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOUR, block.x, block.y, con, g);
        }
    }

    /// Determines the position of the head.
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
            // `unwrap()` is used to let us get rid of the Option<T> enum without explicitly writing
            // some error handling

        (head_block.x, head_block.y)
    }

    /// Moves the snake forward.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => ()
        }

        // Get the front position of the snake
        let (last_x, last_y): (i32, i32) = self.head_position();

        // Match the snake direction
        let new_block = match self.direction {
            // The reason why the y-values seem inverted is because we are looking at the y-axis as
            // a range of positive values. For example, in a screen with dimensions 600x400, the
            // top-right corner would be origin (0, 0), and the bottom-left corner would be the max
            // y and min x-values, i.e. (0, 600). As a result, this shows that the snake going
            // upwards is really DECREMENTING in its y-value (say from 200 to 100).
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y }
        };

        // Push a new block to the front of the snake (depending on which direction it is moving
        // in), and remove the last block.
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    /// Gets the direction the snake is moving in.
    pub fn head_direction(&self) -> Direction {
        self.direction
            // We were required to derive the `Copy` & `Clone` traits for Direction to allow `self`
            // to move out of the borrowed content, (i.e. the direction of the snake in this case).
    }

    /// Gets the next head coordinates according to the current direction the snake is moving in.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }
}