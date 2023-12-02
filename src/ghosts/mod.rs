use crate::common::{BlockType, Cord, Difficulty, Entity, Color, Level};
use crate::common::Dir;
use rand::{Rng, thread_rng};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GhostState {
    Chase,
    Scatter,
    Vulnerable(u64),
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Ghost {
    pub id: u32,
    pub color: Color,
    pub diff: Difficulty,
    pub state: GhostState,
    pub data: Entity,
}

const TIME_IN_STATE: u8 = 50;
const VULNERABLE_TIME: u8 = 30;

impl Ghost {
    pub fn new(id: u32, start_cord: Cord, color: Color, diff: Difficulty) -> Self {
            Self {
                id, 
                color, 
                diff, 
                state: GhostState::Scatter, 
                data: Entity { cord: start_cord, prev_cord: start_cord, start_cord, 
                    dir: Dir::None, prev_dir: Dir::None, tick:0 }
            }
    }
    fn calc_state(&mut self) {
        let state_c = self.data.tick % (TIME_IN_STATE as u64 * 3);
        let expected_sate = if state_c < TIME_IN_STATE as u64 {GhostState::Scatter} else {GhostState::Chase};

        if let GhostState::Vulnerable(x) = self.state {
           if self.data.tick - x  >= VULNERABLE_TIME as u64 {self.state = expected_sate;}
        } else {
            self.state = expected_sate;
        }
    }

    fn mv(&mut self) {
        self.data.prev_cord = self.data.cord;
        self.data.cord = self.data.cord + self.data.dir.to_tup();
        self.data.prev_dir = self.data.dir;
    }

    fn set_dir(&mut self, preferred_dirs: &mut Vec<Dir>, level: &Level) {
        if self.data.prev_dir == Dir::None {self.data.dir = Dir::Up; return;}
        [Dir::Down, Dir::Right, Dir::Left, Dir::Up].iter().for_each(|d| {
            if !preferred_dirs.contains(d) {preferred_dirs.push(*d);}
        });

        let i = preferred_dirs.iter().position(|d| *d == self.data.prev_dir.get_opposite());
        if let Some(x) = i {
            if self.data.prev_dir != Dir::None {preferred_dirs.remove(x);}
        }   

        for dir in preferred_dirs.iter() {
            let t = dir.to_tup();
            let valid_move = level.block(self.data.cord + t) != BlockType::Wall && (level.block(self.data.cord + t) != BlockType::Gate || *dir == Dir::Up);
            if valid_move {self.data.dir = *dir; return;}
        }
    }

    fn set_dir_scatter(&mut self, level: &Level) {
        let mut dirs = match self.id {
            x if x%4 == 0 => vec![Dir::Up, Dir::Left],
            x if x%4 == 1 => vec![Dir::Up, Dir::Right],
            x if x%4 == 2 => vec![Dir::Down, Dir::Right],
            x if x%4 == 3 => vec![Dir::Down, Dir::Left],
            _ => vec![Dir::Up],
        };
        if thread_rng().gen::<bool>() {dirs.reverse();}
        self.set_dir(&mut dirs, level);

    }

    fn set_dir_chase(&mut self, pacman: Cord, level: &Level) {
        let (off_y, off_x) = (pacman.0 as i32 - self.data.cord.0 as i32, pacman.0 as i32 - self.data.cord.0 as i32);
        let mut dirs : Vec<Dir> = Vec::new();
        if off_y > 0 { dirs.push(Dir::Down); } else if off_y != 0 {dirs.push(Dir::Up);}
        if off_x > 0 { dirs.push(Dir::Right); } else if off_x != 0 {dirs.push(Dir::Left);}

        if thread_rng().gen::<bool>() {dirs.reverse();}

        //TODO: potentially check if the current dir is not the reverse of the lst
        self.set_dir(&mut dirs, level);
    }

    pub fn tick(&mut self, pacman: Cord, level: &Level) {
        self.data.tick += 1;
        self.calc_state();
        match self.state {
            GhostState::Vulnerable(_) | GhostState::Scatter => self.set_dir_scatter(level),
            _ => self.set_dir_chase(pacman,level),
        }
        self.mv();
    }
    pub fn die(&mut self) {
        self.data.cord = self.data.start_cord;
        self.data.prev_dir = Dir::None;
    }

    // TODO: when vulnerable the ghost is slower
    pub fn set_vulnerable(&mut self) {
       self.state = GhostState::Vulnerable(self.data.tick);
    }
    pub fn vulnerable(&self) -> bool {
        if let GhostState::Vulnerable(_) = self.state {
            return true;
        }
        false
    }
    pub fn parse_for_fe(&self) -> [u32; 8]{
        [self.id,self.data.cord.0, self.data.cord.1,self.data.prev_cord.0, self.data.prev_cord.1,self.color as u32,self.data.prev_dir as u32, self.vulnerable() as u32]
    }
}