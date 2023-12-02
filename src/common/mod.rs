use wasm_bindgen::prelude::wasm_bindgen;
use crate::level_gen;
extern crate web_sys;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BlockType {
    Empty = 0,
    Wall = 1,
    PacDot = 2,
    PowerPellet = 3,
    Gate = 4,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Difficulty {
    Normal = 0,
    Expert = 1,
}
#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    Red = 1,
    Pink = 2,
    Cyan = 3,
    Orange = 4,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Entity {
    pub cord: Cord,
    pub prev_cord: Cord,
    pub start_cord: Cord,
    pub dir: Dir,
    pub prev_dir: Dir,
    pub tick: u64,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Cord(pub u32, pub u32);

 impl std::ops::Add<(i32, i32)> for Cord {
    type Output = Self;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self((self.0 as i32 + rhs.0) as u32, (self.1 as i32 + rhs.1) as u32)
    }
}

impl From<(u32, u32)> for Cord {
    fn from(value: (u32, u32)) -> Self {
        Cord(value.0, value.1)
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
    None = 99,
}

impl Dir {
    pub fn from_keycode(key_code: &str) -> Self {
        return match key_code {
            "ArrowRight" | "KeyD" => Dir::Right,
            "ArrowLeft" | "KeyA" => Dir::Left,
            "ArrowUp" | "KeyW" => Dir::Up,
            "ArrowDown" | "KeyS" => Dir::Down,
            _ => Dir::None,
        }
    }
    pub fn to_tup(&self) -> (i32, i32) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
            Dir::Left => (0, -1),
            Dir::None => (0,0),
        }
    }
    pub fn get_opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
            Dir::None => Dir::None,
        }
    }

    pub fn get_perpendicular_dirs(&self) -> [Dir; 2] {
        if let Dir::Down | Dir::Up = self {
            return [Dir::Right, Dir::Left];
        }
        [Dir::Down, Dir::Up]
    }
}

#[derive(Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Level {
    width: u32,
    height: u32,
    dot_count: u64,
    field: Vec<BlockType>,
}

const MIN_WIDTH: u32 = 13;
const MIN_HEIGHT: u32 = 11;

impl Level {
    pub fn new(mut width: u32, mut height: u32) -> Self {
        width = std::cmp::max(if width % 2 == 0 { width - 1} else {width}, MIN_WIDTH);
        height = std::cmp::max(if height % 2 == 0 { height - 1} else {height}, MIN_HEIGHT);

        let field = level_gen::generate(width, height);
        Self {
            width,
            height,
            dot_count: Self::count_dots(&field),
            field,
        }
    }

    fn count_dots(level: &Vec<BlockType>) -> u64 {
        level.iter().filter(|t| **t == BlockType::PowerPellet || **t == BlockType::PacDot).count() as u64
    }

    pub fn to_idx(&self, cord: Cord) -> usize {
        (cord.0 * self.width + cord.1) as usize
    }

    pub fn block(&self, cord: Cord) -> BlockType {
        let idx = self.to_idx(cord);
        self.field[idx]
    }

    pub fn set_block(&mut self, cord: Cord, block: BlockType){
        let idx = self.to_idx(cord);
        self.field[idx] = block;
    }

    pub fn reduce_dot_count(&mut self) {
        self.dot_count -= 1;
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn field(&self) -> *const BlockType {
        self.field.as_ptr()
    }
    pub fn dot_count(&self) -> u64 {
        self.dot_count
    }

}

