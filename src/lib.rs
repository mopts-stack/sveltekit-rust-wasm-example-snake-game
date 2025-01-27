use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hi there {}", name).as_str());
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// SNAKE GAME IMPLEMENTATION

#[wasm_bindgen(module = "/web/src/lib/utils.ts")]
extern "C" {
    // from javascript defined in utils
    fn random(max: usize) -> usize;
}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Self {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize,
}

fn spawn_reward(snake_body: &[SnakeCell], size: usize) -> Option<usize> {
    loop {
        let reward_cell = random(size);

        if !snake_body.contains(&SnakeCell(reward_cell)) {
            return Some(reward_cell);
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_idx, 3);

        let reward_cell = spawn_reward(&snake.body, size);

        Self {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell,
            status: None,
            points: 0,
        }
    }

    pub fn reset(&mut self) {
        self.next_cell = None;
        self.status = None;
        self.snake = Snake::new(random(self.size), 3);
        self.reward_cell = spawn_reward(&self.snake.body, self.size);
        self.points = 0;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing..."),
            None => String::from("Idle"),
        }
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        // in fact if the next step cell is equal to the previous body part's cell it means
        // we are tending to move backward which is not possible
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);

        self.snake.direction = direction;
    }

    // cannot return a reference, because of the borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     self.snake.body
    // }

    // *const is raw pointer
    // borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn calculate_snake_next_cell(&mut self) {
        if let Some(GameStatus::Played) = self.status {
            let temp = self.snake.body.clone();

            match self.next_cell {
                Some(cell) => {
                    self.snake.body[0] = cell;
                    self.next_cell = None;
                }
                None => {
                    self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
                }
            }

            let length = self.snake.body.len();
            for i in 1..length {
                self.snake.body[i] = SnakeCell(temp[i - 1].0);
            }

            // check if the head is colliding with the body
            if self.snake.body[1..length].contains(&self.snake.body[0]) {
                self.status = Some(GameStatus::Lost);
            }

            // check if the head colliding with the reward
            if self.reward_cell == Some(self.snake_head_idx()) {
                // make sure snake doesn't grow to the size of the grid
                if length < self.size {
                    self.reward_cell = spawn_reward(&self.snake.body, self.size);
                    self.points += 1;
                } else {
                    // send it off screen
                    self.reward_cell = None;
                    self.status = Some(GameStatus::Won);
                }

                self.snake.body.push(SnakeCell(self.snake.body[1].0));
            }
        }
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();

        let row = snake_idx / self.width;

        // Modulo and division operations are expensive
        // match self.snake.direction {
        //     Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
        //     Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        //     Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
        //     Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
        // }

        match direction {
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell(self.size - self.width + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + self.width - 1)
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
        }
    }
}
