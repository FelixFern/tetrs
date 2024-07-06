use rand::{seq::SliceRandom, thread_rng};

use crate::{NUM_COLS, NUM_ROWS};

#[derive(Debug, Clone, Copy)]
pub enum TBlockType {
    IBlock,
    JBlock,
    LBlock,
    ZBlock,
    TBlock,
    SBlock,
}

impl TBlockType {
    pub fn get(&self) -> [(i8, i8); 4] {
        match self {
            &Self::IBlock => [(0, 0), (1, 0), (2, 0), (3, 0)],
            &Self::LBlock => [(0, 0), (0, 1), (0, 2), (1, 2)],
            &Self::JBlock => [(1, 0), (1, 1), (1, 2), (2, 1)],
            &Self::ZBlock => [(0, 0), (1, 0), (1, 1), (2, 1)],
            &Self::SBlock => [(0, 1), (1, 1), (1, 0), (2, 0)],
            &Self::TBlock => [(0, 0), (1, 0), (1, 1), (2, 0)],
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
    Empty,
}

#[derive(Debug)]
pub enum HorizontalMovement {
    Left,
    Right,
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
            TBlockType::LBlock => TBlockColor::Yellow,
            TBlockType::ZBlock => TBlockColor::Red,
            TBlockType::SBlock => TBlockColor::Magenta,
            TBlockType::TBlock => TBlockColor::Cyan,
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
            if self.is_colliding(grid, (new_pos[i].0 as usize, new_pos[i].1 as usize + 1)) {
                return false;
            } else {
                new_pos[i].1 += 1
            }
        }
        self.pos = new_pos;
        return true;
    }

    pub fn move_horizontal(&self, movement: HorizontalMovement) {
        fn move_left(grid: [[TBlockColor; NUM_COLS]; NUM_ROWS]) {}

        fn move_right(grid: [[TBlockColor; NUM_COLS]; NUM_ROWS]) {}
        // match movement {
        //     HorizontalMovement::Left =>
        // }
    }

    fn is_colliding(
        &self,
        grid: [[TBlockColor; NUM_COLS]; NUM_ROWS],
        new_pos: (usize, usize),
    ) -> bool {
        if new_pos.1 > NUM_ROWS - 1 {
            return true;
        } else if grid[new_pos.1][new_pos.0] != TBlockColor::Empty {
            return true;
        }
        return false;
    }
}
