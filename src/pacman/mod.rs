

use crate::{common::{Entity, Dir, Cord, BlockType, Level}, log};

pub struct Pacman {
    lives: u8,
    pub data: Entity
}

impl Pacman {
    pub fn new(start_cord: Cord, lives: u8) -> Self {
        Pacman { 
            lives, 
            data: Entity {
                start_cord,
                cord: start_cord,
                prev_cord: start_cord,
                dir: Dir::None,
                prev_dir: Dir::None,
                tick: 0,
            }
         }
    }

    pub fn set_dir_outside(&mut self, dir: Dir) {
        self.data.dir = dir;
    }

    fn mv(&mut self, level: &Level){
        let dir_valid = |cord: Cord, dir: &Dir|
            level.block(cord + dir.to_tup()) != BlockType::Wall && level.block(cord + dir.to_tup()) != BlockType::Gate;


        let adjusted_cord = if dir_valid(self.data.cord, &self.data.dir) {self.data.cord} else {self.data.prev_cord};
        //log!("")
        self.data.prev_cord = adjusted_cord;
        self.data.cord = adjusted_cord + self.data.dir.to_tup();
        self.data.prev_dir = self.data.dir;
    }

    fn set_dir(&mut self, level: &Level) {
        let dir_valid = |cord: Cord, dir: &Dir|
            level.block(cord + dir.to_tup()) != BlockType::Wall && level.block(cord + dir.to_tup()) != BlockType::Gate;

        if !dir_valid(self.data.cord, &self.data.dir) && !dir_valid(self.data.cord, &self.data.prev_dir){
            self.data.dir = Dir::None;
        } else if !dir_valid(self.data.cord, &self.data.dir) {
            self.data.dir = self.data.prev_dir;
        }
    }

    pub fn tick(&mut self, level: &Level) {
        self.data.tick += 1;
        self.set_dir(level);
        self.mv(level);
    }
    pub fn revive(&mut self) -> bool {
        if self.lives == 1 { self.lives -= 1; return false;}
		if self.lives < 1 { return false;}
        self.data.cord = self.data.start_cord;
        self.data.dir = Dir::None; self.data.prev_dir = Dir::None;
        self.lives -= 1;
        true
    }

    pub fn parse_for_fe(&self) -> [u32; 6]{
        [self.data.cord.0, self.data.cord.1,self.data.prev_cord.0, self.data.prev_cord.1,self.data.prev_dir as u32, self.lives as u32]
    }
    pub fn lives(&self) -> u8 {
        self.lives
    }
}
