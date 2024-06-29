use crate::NUM_ROWS;

#[derive(Debug)]
pub enum TBlockType {
    IBlock,
    JBlock,
    LBlock,
    ZBlock,
    TBlock,
    SBlock,
}
#[derive(Copy, Clone, Debug)]
pub enum TBlockColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Empty,
}

#[derive(Debug)]
pub enum TBlockOrientation {
    HORIZONTAL,
    VERTICAL,
}

#[derive(Debug)]
pub struct TetrisBlock {
    x: usize,
    y: usize,
    block: TBlockType,
    orientation: TBlockOrientation,
}

impl TetrisBlock {
    pub fn new(x: usize, y: usize, block: TBlockType) -> Self {
        Self {
            x,
            y,
            block,
            orientation: TBlockOrientation::HORIZONTAL,
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        return (self.x, self.y);
    }

    pub fn move_down(&mut self) -> Result<bool, bool> {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;

            return Ok(false);
        }
        return Ok(true);
    }
}
