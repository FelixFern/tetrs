use rand::{seq::SliceRandom, thread_rng};
use ratatui::style::Color;

use crate::{NUM_COLS, NUM_ROWS};

#[derive(Debug, Clone, Copy)]
pub enum TBlockType {
    IBlock,
    JBlock,
    LBlock,
    ZBlock,
    TBlock,
    SBlock,
    OBlock,
}

impl TBlockType {
    pub fn get(&self) -> [(i8, i8); 4] {
        match self {
            &Self::IBlock => [(0, 0), (1, 0), (2, 0), (3, 0)],
            &Self::LBlock => [(0, 0), (0, 1), (0, 2), (1, 2)],
            &Self::JBlock => [(1, 0), (1, 1), (1, 2), (0, 2)],
            &Self::ZBlock => [(0, 0), (1, 0), (1, 1), (2, 1)],
            &Self::SBlock => [(0, 1), (1, 1), (1, 0), (2, 0)],
            &Self::TBlock => [(0, 1), (1, 1), (1, 0), (2, 1)],
            &Self::OBlock => [(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
    fn values() -> &'static [TBlockType] {
        static BLOCK_TYPES: &[TBlockType] = &[
            TBlockType::IBlock,
            TBlockType::JBlock,
            TBlockType::LBlock,
            TBlockType::ZBlock,
            TBlockType::TBlock,
            TBlockType::SBlock,
            TBlockType::OBlock,
        ];
        BLOCK_TYPES
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        *TBlockType::values()
            .choose(&mut rng)
            .expect("Enum values cannot be empty")
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TBlockColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Orange,
    Empty,
}

#[derive(Debug)]
pub enum ColidingDirection {
    Left,
    Right,
    Down,
}

#[derive(Debug)]
pub struct TetrisBlock {
    color: TBlockColor,
    pos: [(u8, u8); 4],
}

impl TetrisBlock {
    pub fn new(x: u8, y: u8, block: TBlockType) -> Self {
        let color = match block {
            TBlockType::IBlock => TBlockColor::Blue,
            TBlockType::JBlock => TBlockColor::Green,
            TBlockType::LBlock => TBlockColor::Orange,
            TBlockType::ZBlock => TBlockColor::Red,
            TBlockType::SBlock => TBlockColor::Magenta,
            TBlockType::TBlock => TBlockColor::Cyan,
            TBlockType::OBlock => TBlockColor::Yellow,
        };

        let pos = block.get().map(|f| {
            if x as i8 + f.0 >= 0 && x as i8 + f.0 < 10 && y as i8 + f.1 >= 0 && y as i8 + f.1 < 20
            {
                (x + f.0 as u8, y + f.1 as u8)
            } else {
                (0, 0)
            }
        });

        Self { color, pos }
    }

    pub fn get_pos(&self) -> ([(u8, u8); 4], TBlockColor) {
        return (self.pos, self.color);
    }

    pub fn move_down(&mut self, grid: [[TBlockColor; NUM_COLS]; NUM_ROWS]) -> bool {
        let mut new_pos = self.pos.clone();
        for i in 0..new_pos.len() {
            if self.is_colliding(
                grid,
                (new_pos[i].0 as i8, new_pos[i].1 as i8 + 1),
                ColidingDirection::Down,
            ) {
                return false;
            } else {
                new_pos[i].1 += 1
            }
        }
        self.pos = new_pos;
        return true;
    }

    pub fn move_left(&mut self, grid: [[TBlockColor; NUM_COLS]; NUM_ROWS]) {
        let mut new_pos = self.pos.clone();
        let mut moveable = true;

        for i in 0..new_pos.len() {
            if self.is_colliding(
                grid,
                (new_pos[i].0 as i8 - 1, new_pos[i].1 as i8),
                ColidingDirection::Left,
            ) {
                moveable = false;
            } else {
                new_pos[i].0 -= 1
            }
        }

        if moveable {
            self.pos = new_pos;
        }
    }

    pub fn move_right(&mut self, grid: [[TBlockColor; NUM_COLS]; NUM_ROWS]) {
        let mut new_pos = self.pos.clone();
        let mut moveable = true;

        for i in 0..new_pos.len() {
            if self.is_colliding(
                grid,
                (new_pos[i].0 as i8 + 1, new_pos[i].1 as i8),
                ColidingDirection::Right,
            ) {
                moveable = false;
            } else {
                new_pos[i].0 += 1
            }
        }

        if moveable {
            self.pos = new_pos;
        }
    }

    fn is_colliding(
        &self,
        grid: [[TBlockColor; NUM_COLS]; NUM_ROWS],
        new_pos: (i8, i8),
        direction: ColidingDirection,
    ) -> bool {
        match direction {
            ColidingDirection::Down => {
                if new_pos.1 > (NUM_ROWS - 1) as i8 {
                    return true;
                } else if grid[new_pos.1 as usize][new_pos.0 as usize] != TBlockColor::Empty {
                    return true;
                }
                return false;
            }
            ColidingDirection::Left => {
                if new_pos.0 < 0 {
                    return true;
                } else if grid[new_pos.1 as usize][new_pos.0 as usize] != TBlockColor::Empty {
                    return true;
                }
                return false;
            }
            ColidingDirection::Right => {
                if new_pos.0 > (NUM_COLS - 1) as i8 {
                    return true;
                } else if grid[new_pos.1 as usize][new_pos.0 as usize] != TBlockColor::Empty {
                    return true;
                }
                return false;
            }
        }
    }
}
