use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rect};

const COLOUR_BORDER: Color = [0.15, 0.15, 0.15, 1.0];
const COLOUR_CLEAR: Color = [0.0, 0.0, 0.0, 0.0];
const COLOUR_APPLE: Color = [0.9, 0.0, 0.2, 1.0];
const COLOUR_GAMEOVER: Color = [0.72, 0.09, 0.09, 0.5];
const COLOUR_PAUSE: Color = [0.16, 0.27, 0.34, 0.5];
const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 3.0;

const SCREEN_MAX_HEIGHT: f64 = 765.0;
const SCREEN_MAX_WIDTH: f64 = 750.0;

pub struct Game {
    snake: Snake,
    level: i32,

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
    /// Initialises a new game.
    pub fn new(width: i32, height: i32) -> Game {
        let snake = Snake::new(2, 2);
        let (apple_x, apple_y) = Self::randomise_apple_location(&snake, width, height);

        Game {
            snake,
            level: 0,

            apple_exists: true,
            apple_x,
            apple_y,

            width,
            height,

            game_over: false,
            paused: false,
            waiting_time: 0.0,
        }
    }

    /// Responsible for detecting key presses and taking their prescribed actions if applicable.
    pub fn key_pressed(&mut self, key: Key) {
        // If the game is over, do nothing:
        if self.game_over {
            return;
        }

        // If the user presses the Space key, the game will pause/unpause:
        if key == Key::Space {
            self.pause_resume();
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

    /// Responsible for drawing the blocks in the game.
    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.snake.draw(con, g);

        if self.apple_exists {
            draw_block(COLOUR_APPLE, self.apple_x, self.apple_y, con, g);
        }

        // TOP, RIGHT, BOTTOM, LEFT borders:
        draw_rect(COLOUR_BORDER, 0, 0, self.width, 1, con, g);
        draw_rect(COLOUR_BORDER, self.width - 1, 0, 1, self.height, con, g);
        draw_rect(COLOUR_BORDER, 0, self.height - 1, self.width, 1, con, g);
        draw_rect(COLOUR_BORDER, 0, 0, 1, self.height, con, g);

        /// Macro to add text to screen. Assumes parameters have the type (in order):
        /// - text: `&str`
        /// - colour: `Color` or `[f32; 4]`
        /// - size: `FontSize` or `u32`
        /// - x: `f32` or `f64`
        /// - y: `f32` or `f64`
        macro_rules! add_text {
            ($text:expr, $colour:expr, $size:expr, $x:expr, $y:expr) => {
                Text::new_color($colour, $size).draw(
                    $text,
                    glyphs,
                    &con.draw_state,
                    con.transform.trans($x, $y),
                    g
                ).unwrap();
            };
        }

        if self.game_over {
            draw_rect(COLOUR_GAMEOVER, 0, 0, self.width, self.height, con, g);
            add_text!("Oh no! You died!", WHITE, 40, 170.0, 300.0);
            add_text!(format!("You're score was {}", self.level).as_str(), WHITE, 15, 280.0, 340.0);
        }

        if self.paused {
            draw_rect(COLOUR_PAUSE, 0, 0, self.width, self.height, con, g);
            add_text!("Paused", WHITE, 40, 290.0, 300.0);
            add_text!("Press <SPACE> to resume game", WHITE, 15, 240.0, 340.0);
            add_text!("ABC", WHITE, 15, SCREEN_MAX_WIDTH, SCREEN_MAX_HEIGHT);
        } else {
            draw_rect(COLOUR_CLEAR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// Responsible for updating all the blocks in the game.
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

    /// Checks to see if the snake has eaten an apple.
    fn has_eaten(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        if self.apple_exists && head_x == self.apple_x && head_y == self.apple_y {
            self.apple_exists = false;
            self.snake.grow_tail();
            self.up_level();
        }
    }

    /// Checks to see if the snake is alive.
    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        // Check if snake has touched itself:
        if self.snake.is_overlapping(next_x, next_y) {
            return false;
        }

        // Check if snake is out-of-bounds (over the border):
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// Randomise location of apple.
    fn randomise_apple_location(snake: &Snake, width: i32, height: i32) -> (i32, i32) {
        let mut rng = thread_rng();

        // Set random thread's range dimensions:
        let mut new_x = rng.gen_range(1, width - 1);
        let mut new_y = rng.gen_range(1, height - 1);

        // Prevent apple from spawning on the snake:
        while snake.is_overlapping(new_x, new_y) {
            new_x = rng.gen_range(1, width - 1);
            new_y = rng.gen_range(1, height - 1);
        }

        (new_x, new_y)
    }

    /// Adds an apple to the screen.
    fn add_apple(&mut self) {
        let (new_x, new_y) = Self::randomise_apple_location(
            &self.snake,
            self.width,
            self.height);

        // Set the coordinates of the new apple:
        self.apple_x = new_x;
        self.apple_y = new_y;
        self.apple_exists = true;
    }

    /// Updates the direction and position of the snake.
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

    fn up_level(&mut self) {
        self.level += 1;
        println!("* Level: {}, Length: {}", self.level, self.snake.get_length());
    }

    /// Toggles between pausing and resuming the game depending on current game state.
    fn pause_resume(&mut self) {
        self.paused = !self.paused;
    }

    /// Restarts the game with set properties.
    fn restart(&mut self) {
        let (apple_x, apple_y) = Self::randomise_apple_location(
            &self.snake,
            self.width,
            self.height);

        self.snake = Snake::new(2, 2);
        self.level = 1;

        self.apple_exists = true;
        self.apple_x = apple_x;
        self.apple_y = apple_y;

        self.game_over = false;
        self.paused = false;
        self.waiting_time = 0.0;
    }
}