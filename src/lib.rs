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

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[wasm_bindgen]
#[derive(Clone)]
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
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        Self {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);

        // in fact if the next step cell is equal to the previous body part's cell it means
        // we are tending to move backward which is not possible
        if self.snake.body[1].0 == next_cell.0 {
            return;
        }

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

    pub fn calculate_snake_cell(&mut self) {
        let temp = self.snake.body.clone();

        let next_cell = self.generate_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let length = self.snake.body.len();

        for i in 1..length {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
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
