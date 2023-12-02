/*

TODO:   Smooth animation, render with pacman assets
        Case where a 2 Block wide passage is created around the spawn area

*/
mod common;
mod level_gen;
mod ghosts;
mod pacman;


use common::Level;
use pacman::Pacman;
use wasm_bindgen::prelude::wasm_bindgen;
use ghosts::Ghost;
use common::BlockType;
use common::Cord;
use crate::common::{Difficulty, Dir,Color};

#[wasm_bindgen]
pub struct Game {
    level: Level,
    pacman: Pacman,
    ghosts: Vec<Ghost>,
    diff: Difficulty,
    score: u32,
    game_over: bool,
}

const SCORE_PACDOT: u32 = 10;
const SCORE_PALLET: u32 = 50;
const SCORE_GHOST: u32 = 200;

#[wasm_bindgen]
impl Game{
    pub fn new(width: u32, height: u32, diff: Difficulty) -> Self {
        let level = Level::new(width, height);
        let (my, mx) = (level.height()/2, level.width() / 2);
        let g: Ghost = Ghost::new(0,Cord(my,mx),Color::Cyan,diff);
        let ghosts = vec![Ghost {color: Color::Red, ..g }, Ghost {color: Color::Pink, id: 1, ..g },
            Ghost {color: Color::Cyan, id: 2, ..g }, Ghost {color: Color::Orange, id: 3, ..g },
        ];

        let pac_start_pos = Cord (my + 2, mx);
        let pacman = Pacman::new(pac_start_pos, 3);
        Self {
            level,
            ghosts,
            pacman,
            diff,
            score: 0,
            game_over: false,
        }
    }

    pub fn set_dir(&mut self, key_code: &str) {
        let dir = Dir::from_keycode(key_code);
        self.pacman.set_dir_outside(dir);
    }


    fn process_block(&mut self) -> bool {
        let pac_block = self.level.block(self.pacman.data.cord);
        let score_gained: u32 = match pac_block {
            BlockType::PacDot => SCORE_PACDOT,
            BlockType::PowerPellet => {
                self.ghosts.iter_mut().for_each(|ghost| ghost.set_vulnerable());
                SCORE_PALLET
            },
            _ => 0
        };
        if score_gained > 0 {self.level.reduce_dot_count();}
        self.level.set_block(self.pacman.data.cord, BlockType::Empty);
        self.score += score_gained;

        for ghost in self.ghosts.iter_mut() {
            let ate_pacman = ghost.data.prev_cord == self.pacman.data.cord && self.pacman.data.prev_cord == ghost.data.cord;

            if ate_pacman || ghost.data.cord == self.pacman.data.cord {
                if ghost.vulnerable() {
                    ghost.die();
                    self.score += SCORE_GHOST;
                } else {
                    if !self.pacman.revive() {self.game_over = true};
                    break;
                }
            }
        }
        self.game_over
    }

    pub fn tick(&mut self) -> bool {
        if self.game_over {return self.game_over;}
        //move pacman
        self.pacman.tick(&self.level);

        //move/tick ghosts
        for ghost in self.ghosts.iter_mut() {
            ghost.tick(self.pacman.data.cord,&self.level);
        }

        self.process_block();

        self.game_over
    }

    pub fn ghosts(&self) -> js_sys::Uint32Array {
        let v: Vec<[u32; 8]> = self.ghosts.iter()
            .map(|g| g.parse_for_fe())
            .collect();
        let f: Vec<u32> = v.iter()
            .flatten()
            .map(|i| *i)
            .collect();

        js_sys::Uint32Array::from(&f[..])
    }
    pub fn pacman(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.pacman.parse_for_fe()[..])
    }

    pub fn diff(&self) -> Difficulty {
        self.diff
    }

    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn game_over(&self) -> bool {
        self.game_over
    }
    pub fn field(&self) -> *const BlockType {
        self.level.field()
    }
    pub fn width(&self) -> u32 {
        self.level.width()
    }
    pub fn height(&self) -> u32 {
        self.level.height()
    }
    pub fn dot_count(&self) -> u64 {
        self.level.dot_count()
    }
    pub fn lives(&self) -> u8 {
        self.pacman.lives()
    }
    pub fn field_at(&self, y: u32, x: u32) -> BlockType {
        self.level.block(Cord(y,x))
    }
}