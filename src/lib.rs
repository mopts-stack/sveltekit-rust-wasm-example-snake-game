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

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();

        let (row, col) = self.index_to_cell(snake_idx);

        let (row, col) = match self.snake.direction {
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
            Direction::Right => (row, (col + 1) % self.width),
            Direction::Left => (row, (col - 1) % self.width),
        };

        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head_idx(next_idx);
    }

    fn set_snake_head_idx(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}
