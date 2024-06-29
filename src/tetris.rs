pub enum TBlockType {
    IBlock,
    JBlock,
    LBlock,
    ZBlock,
    TBlock,
    SBlock,
}
#[derive(Copy, Clone)]
pub enum TBlockColor {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Empty,
}

pub enum TBlockOrientation {
    HORIZONTAL,
    VERTICAL,
}

pub struct TetrisBlock {
    x: u8,
    y: u8,
    block: TBlockType,
    orientation: TBlockOrientation,
}

impl TetrisBlock {
    fn create(&mut self, x: u8, y: u8, block: TBlockType) {
        self.x = x;
        self.y = y;
        self.block = block;
        self.orientation = TBlockOrientation::VERTICAL
    }

    fn move_down(&mut self) {
        self.y += 1
    }
}
