use crate::common::{Dir,Cord, BlockType};
use rand::{Rng, thread_rng};
use std::cmp;

trait BlockPositions {
    fn block_positions(&self) -> Vec<(i32, i32)>;
}

struct Rectangle {
    width: u32,
    height: u32,
}

//the origin is included in all sizes
struct Star {
    height: u32,
    offset: u32,
    left: u32,
    right: u32,
}

struct Field {
    width: u32,
    height: u32,
    field: Vec<BlockType>,
}

impl BlockPositions for Rectangle {
    fn block_positions(&self) -> Vec<(i32, i32)> {
        let mut positions: Vec<(i32, i32)> = Vec::with_capacity((self.width * self.height) as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                positions.push((y as i32, x as i32));
            }
        }
        positions
    }
}

impl BlockPositions for Star {
    fn block_positions(&self) -> Vec<(i32, i32)> {
        let mut positions = Vec::with_capacity((self.height + self.left + self.right) as usize);

        if self.height < 1 || self.right < 1 || self.left < 1 {
            panic!("Dimensions invalid! (block_positions(...))")
        }

        for y in 0..self.height {
            positions.push((y as i32, 0));
        }
        for x in (self.left as i32 - 1) * -1..(self.right as i32) {
            positions.push((self.offset as i32, x));
        }

        positions
    }
}

const MAX_WALL_LENGTH: u32 = 7;

impl Field {
    pub fn new(width: u32, height: u32) -> Self {
        if width % 2 == 0 || height % 2 == 0 || width < 9 || height < 7 {
            panic!("Invalid width or height! (new(...))")
        }
        Self {
            width,
            height,
            field: Vec::with_capacity((width * height) as usize),
        }
    }

    fn to_idx(&self, cord: Cord) -> usize {
        (self.width * cord.0 + cord.1) as usize
    }

    fn insert_shape(&mut self, org: Cord, shape: &Vec<(i32, i32)>) {
        shape.iter().for_each(|off| {
            let new_cord = org + *off;
            let i = self.to_idx(new_cord);
            self.field[i] = BlockType::Wall;
        });
    }

    fn insert_spawn(&mut self) {
        let spawn_area = vec![
            (-1, -2), (-1, -1), (-1, 1), (-1, 2),
            (0, -2), (0, 2),
            (1, -2), (1, -1), (1, 0), (1, 1), (1, 2),
        ];
        let (my, mx): (u32, u32) = ((self.height / 2), (self.width / 2));
        let gate_idx = self.to_idx(Cord(my-1, mx));
        // delete exising blocks in spawn area
        for y in my - 2..=my + 2 {
            for x in mx - 3..=mx + 3 {
                let idx = self.to_idx(Cord(y, x));
                self.field[idx] = BlockType::Empty;
            }
        }
        self.insert_shape(Cord::from((my, mx)), &spawn_area);
        self.field[gate_idx] = BlockType::Gate;
    }
    fn init(&mut self) {
        self.field = vec![BlockType::PacDot; (self.width * self.height) as usize];
        let w = self.width as usize;
        let h = self.height as usize;
        self.field.iter_mut()
            .enumerate()
            .for_each(|(i, val)| {
                let first_or_last_row = i < w || i > ((h - 1) * w);
                let first_or_last_col = i % w == 0 || i % w == (w - 1);
                if first_or_last_col || first_or_last_row {
                    *val = BlockType::Wall;
                }
            });
        self.insert_spawn();
    }

    fn get_blocks(&self, mut cord: Cord, dir: &Dir) -> Vec<Cord> {
        let off = dir.to_tup();
        let mut cords: Vec<Cord> = Vec::new();
        while cord.0 < self.height && cord.1 < self.width {
            cords.push(cord);
            if cord.0 == 0 || cord.1 == 0 { break; }
            cord = cord + off;
        }
        cords
    }

    fn get_distance_to_wall(&self, cord: Cord, dir: &Dir) -> u32 {
        let blocks = self.get_blocks(cord, dir);
        blocks.iter().skip(1).position(|block| self.field[self.to_idx(*block)] == BlockType::Wall).unwrap_or(blocks.len()) as u32
    }

    //max length in one direction -> pos is included
    fn get_max_dir(&self, cord: Cord, dir: &Dir, for_star: bool) -> u32 {
        let valid_distance = |block: &Cord| {
            let perpendicular = dir.get_perpendicular_dirs();
            let distances = (self.get_distance_to_wall(*block, &perpendicular[0]), self.get_distance_to_wall(*block, &perpendicular[1]));
            (distances.0 != 0 && distances.1 != 0) && (!for_star || distances.0 != 2 && distances.1 != 2)
        };
        let blocks = self.get_blocks(cord, dir);
        let n = blocks.iter()
            .position(|block| !valid_distance(block) || self.field[self.to_idx(*block)] == BlockType::Wall)
            .unwrap_or(0) as u32;
        if n == 0 { 0 } else { n - 1 }
    }

    fn get_rand_size(&self, cord: Cord, direction: &Dir, for_star: bool) -> u32 {
        const PREFFERED_LEN: u32 = 3;
        let mut rng = thread_rng();
        let max = cmp::min(self.get_max_dir(cord, &direction, for_star), MAX_WALL_LENGTH);
        if max == 0 { return 0; }
        let mut rand: u32;
        if max > PREFFERED_LEN { rand = rng.gen_range(PREFFERED_LEN..=max); } else { rand = max; };
        let off = (direction.to_tup().0 * (rand - 1) as i32, direction.to_tup().1 * (rand - 1) as i32);

        if self.get_distance_to_wall(cord + off, direction) == 2 {
            rand += 1;
        }
        rand
    }
    fn insert_random_rect(&mut self, cord: Cord) -> u32 {
        let mut height = self.get_rand_size(cord, &Dir::Down, false);
        return loop {
            let (mut max_width, mut off_y) = (u32::MAX, 0);
            for y in 0..height {
                let curr_max_width = self.get_max_dir(cord + (y as i32, 0), &Dir::Right, false);
                if curr_max_width < max_width {
                    max_width = curr_max_width;
                    off_y = y;
                }
            }
            if max_width == 0 {
                height -= 1;
                continue;
            }
            let width = self.get_rand_size(cord + (off_y as i32, 0), &Dir::Right, false);
            self.insert_shape(cord, &Rectangle { width, height }.block_positions());
            break width;
        };
    }

    fn insert_random_star(&mut self, cord: Cord) -> u32 {
        let height = self.get_rand_size(cord, &Dir::Down, false);

        let mut distances_per_block: Vec<u32> = Vec::new();
        for diff_y in 0..height {
            let max_right = self.get_max_dir(cord + (diff_y as i32, 0), &Dir::Right, true);
            let max_left = self.get_max_dir(cord + (diff_y as i32, 0), &Dir::Left, true);
            if (max_right != 1 || max_left != 1) && (max_right != 0 && max_left != 0) {
                distances_per_block.extend(vec![diff_y; (max_left + max_right) as usize - 1]);
            }
        }

        let mut star = Star { height, offset: 0, left: 1, right: 1 };
        if distances_per_block.len() != 0 {
            let mut rng = thread_rng();
            let idx = rng.gen_range(0..distances_per_block.len());
            let offset = distances_per_block[idx];

            let right = self.get_rand_size(cord + (offset as i32, 0), &Dir::Right, true);
            let left = self.get_rand_size(cord + (offset as i32, 0), &Dir::Left, true);

            star = Star { height, offset, right, left };
        }
        self.insert_shape(cord, &star.block_positions());

        1
    }

    fn neighbours(&self, cord: Cord) -> u32 {
        let mut count = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                if y == 0 && x == 0 { continue; }
                if self.field[self.to_idx(cord + (y, x))] == BlockType::Wall { count += 1; }
            }
        }
        count
    }
    fn post_processing(&mut self) {
        for y in 2..self.height - 2 {
            for x in 2..self.width - 2 {
                if self.field[self.to_idx(Cord(y, x))] == BlockType::Wall { continue; }

                let cords: [Cord; 4] = [Cord(y, x + 1), Cord(y + 1, x + 1), Cord(y, x), Cord(y + 1, x)];
                if cords.iter().fold(false, |a, b| self.field[self.to_idx(*b)] == BlockType::Wall || a) {
                    continue;
                }

                let valid_cords = cords.iter().find(|cord| {
                    let dirs: Vec<&Dir> = [Dir::Down, Dir::Right, Dir::Left, Dir::Up]
                        .iter()
                        .filter(|dir| self.field[self.to_idx(**cord + dir.to_tup())] == BlockType::Wall)
                        .collect();
                    let mut visited: Vec<Cord> = Vec::new();
                    let count: u32 = dirs.iter()
                        .map(|dir|{
                            let perp_dirs = dir.get_perpendicular_dirs();
                            let new_cord = **cord + dir.to_tup();

                            let cord1 = new_cord + perp_dirs[0].to_tup();
                            let cord2 = new_cord + perp_dirs[1].to_tup();

                            let c = 1 + (!visited.contains(&cord1) && self.field[self.to_idx(cord1)] == BlockType::Wall) as u32
                                + (!visited.contains(&cord2) && self.field[self.to_idx(cord2)] == BlockType::Wall) as u32;
                            visited.push(cord1); visited.push(cord2);
                            c
                        }).sum();
                    self.neighbours(**cord) as i32 - count as i32 <= 0
                });

                if let Some(cord) = valid_cords {
                    let idx = self.to_idx(*cord);
                    self.field[idx] = BlockType::Wall;
                }
            }
        }
    }

    fn insert_pallets(&mut self) {
        [Cord(1,1), Cord(1,self.width-2), Cord(self.height-2, 1), Cord(self.height-2, self.width-2)].iter()
            .for_each(|cord|{ 
                let idx = self.to_idx(*cord);
                self.field[idx] = BlockType::PowerPellet
            });
    }
    fn create_passages(&mut self) {
        let mut rng = thread_rng();
        for y in 2..self.width - 2 {
            let mut x = 2;
            while x < self.height - 2 {
                let cord = Cord(y, x);
                if self.field[self.to_idx(cord)] == BlockType::Wall || self.neighbours(cord) > 0 {
                    x += 1;
                    continue;
                }

                let max_bottom = self.get_max_dir(cord, &Dir::Down, true);
                let max_right = self.get_max_dir(cord, &Dir::Right, true);
                let star_valid = max_right > 0 && max_bottom > 0;

                let max_bottom = self.get_max_dir(cord, &Dir::Down, false);
                let max_right = self.get_max_dir(cord, &Dir::Right, false);
                let rect_valid = max_bottom > 0 && max_right > 0;

                let rand = rng.gen::<u8>();
                let is_first_element = y == 2 && x == 2;

                if (is_first_element || rand < (u8::MAX / 3) || !star_valid) && rect_valid {
                    x += self.insert_random_rect(cord);
                } else if star_valid {
                    x += self.insert_random_star(cord);
                }
                x += 1;
            }
        }
        self.insert_spawn();
        self.post_processing();
        self.insert_pallets();
    }

}

pub fn generate(width: u32, height: u32) -> Vec<BlockType> {
    let mut pf = Field::new(width, height);
    pf.init();
    pf.create_passages();
    pf.field.clone()
}
