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
const WIDTH: usize = 16;

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
}

impl Snake {
    fn new(spawn_index: usize) -> Self {
        Self {
            body: vec![SnakeCell(spawn_index)],
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: WIDTH,
            size: WIDTH * WIDTH,
            snake: Snake::new(10),
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new() -> Self {
        Self::default()
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

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        self.snake.body[0].0 = (snake_idx + 1) % (self.size);
    }
}
