use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rect};

const APPLE_COLOUR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOUR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOUR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.5;

pub struct Game {
    snake: Snake,

    apple_exists: bool,
    apple_x: i32,
    apple_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    paused: bool,
    waiting_time: f64
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),

            apple_exists: true,
            apple_x: 6,
            apple_y: 4,

            width,
            height,

            game_over: false,
            paused: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        // If the game is over, do nothing:
        if self.game_over {
            return;
        }

        if key.code() == 0x20 {
            self.pause_unpause_game();
        }

        // Map the keys to a direction, other keys will be ignored:
        let dir = match key {
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::A => Some(Direction::Left),
            Key::D => Some(Direction::Right),
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        // If the direction is the opposite as the snake's moving direction, do nothing:
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.apple_exists {
            draw_block(APPLE_COLOUR, self.apple_x, self.apple_y, con, g);
        }

        // TOP, RIGHT, BOTTOM, LEFT Borders:
        draw_rect(BORDER_COLOUR, 0, 0, self.width, 1, con, g);
        draw_rect(BORDER_COLOUR, self.width - 1, 0, 1, self.height, con, g);
        draw_rect(BORDER_COLOUR, 0, self.height - 1, self.width, 1, con, g);
        draw_rect(BORDER_COLOUR, 0, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rect(GAMEOVER_COLOUR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // Restart the game when the waiting time is over after game over, otherwise do nothing:
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }

            return;
        }

        // Add an apple if there isn't one currently:
        if !self.apple_exists {
            self.add_apple();
        }

        // Update the snake once it's over the moving period (i.e. the frame-rate):
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn has_eaten(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();

        if self.apple_exists && head_x == self.apple_x && head_y == self.apple_y {
            self.apple_exists = false;
            self.snake.grow_tail();
        }
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // Check if snake has touched itself:
        if self.snake.is_overlapping(next_x, next_y) {
            return false;
        }

        // Check if snake is out-of-bounds (over the border):
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_apple(&mut self) {
        let mut rng = thread_rng();

        // Set random thread's range dimensions:
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);

        // Prevent apple from spawning on the snake:
        while self.snake.is_overlapping(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        // Set the coordinates of the new apple:
        self.apple_x = new_x;
        self.apple_y = new_y;
        self.apple_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            if !self.paused {
                self.snake.move_forward(dir);
                self.has_eaten();
            }
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    fn pause_unpause_game(&mut self) {
        self.paused = !self.paused;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);

        self.apple_exists = true;
        self.apple_x = 6;
        self.apple_y = 4;

        self.game_over = false;
        self.paused = false;
        self.waiting_time = 0.0;
    }
}