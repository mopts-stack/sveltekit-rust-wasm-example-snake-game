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
const SIZE: usize = 16;

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
    snake: Snake,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: SIZE,
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

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }
}
