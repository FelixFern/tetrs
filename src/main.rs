use std::io::{self};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};
use tetris::{TBlockColor, TetrisBlock};

mod tetris;
mod tui;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 10;
const FRAMES_PER_MOVE: usize = 30;

#[derive(Debug)]
pub struct App {
    exit: bool,
    grid: [[TBlockColor; NUM_COLS]; NUM_ROWS],
    tetrising: bool,
    tetris_block: TetrisBlock,
    frame_count: usize,
    curr_x: usize,
}

impl Default for App {
    fn default() -> Self {
        let grid = [[TBlockColor::Empty; NUM_COLS]; NUM_ROWS];
        App {
            exit: false,
            grid,
            tetrising: false,
            tetris_block: TetrisBlock::new(0, 0, tetris::TBlockType::IBlock),
            frame_count: 0,
            curr_x: 0,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            self.update();
            terminal.draw(|f: &mut Frame| self.render_frame(f))?;
            // self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;

        if self.frame_count >= FRAMES_PER_MOVE {
            if self.tetris_block.move_down().unwrap() {
                self.curr_x += 1;

                let (x, y) = self.tetris_block.get_pos();
                self.grid[y][x] = TBlockColor::Green;
                self.tetris_block = TetrisBlock::new(self.curr_x, 0, tetris::TBlockType::IBlock)
            }
            self.frame_count = 0;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let cell_size = std::cmp::min(
            area.width / (NUM_COLS as u16) * 2,
            area.height / NUM_ROWS as u16,
        );

        let total_width = cell_size * (NUM_COLS as u16) * 2;
        let total_height = cell_size * NUM_ROWS as u16;

        let start_x = area.x + (area.width - total_width) / 2;
        let start_y = area.y + (area.height - total_height) / 2;

        let mut moving_grid = [[TBlockColor::Empty; NUM_COLS]; NUM_ROWS];

        let (tetris_x, tetris_y) = self.tetris_block.get_pos();
        moving_grid[tetris_y][tetris_x] = TBlockColor::Blue;

        let row_constraint = (0..NUM_ROWS)
            .map(|_| Constraint::Length(cell_size))
            .collect::<Vec<_>>();

        let col_constraint = (0..NUM_COLS)
            .map(|_| Constraint::Length(cell_size * 2))
            .collect::<Vec<_>>();

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraint.clone())
            .split(Rect::new(start_x, start_y, total_width, total_height));

        for (y, row) in rows.iter().enumerate() {
            // Split each row into 8 columns
            let columns = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraint.clone())
                .split(*row);

            for (x, column) in columns.iter().enumerate() {
                let moving_grid_color: Color = match moving_grid[y][x] {
                    TBlockColor::Empty => Color::Reset,
                    TBlockColor::Red => Color::Red,
                    TBlockColor::Green => Color::Green,
                    TBlockColor::Blue => Color::Blue,
                    TBlockColor::Yellow => Color::Yellow,
                    TBlockColor::Magenta => Color::Magenta,
                };

                let grid_color: Color = match self.grid[y][x] {
                    TBlockColor::Empty => Color::Reset,
                    TBlockColor::Red => Color::Red,
                    TBlockColor::Green => Color::Green,
                    TBlockColor::Blue => Color::Blue,
                    TBlockColor::Yellow => Color::Yellow,
                    TBlockColor::Magenta => Color::Magenta,
                };

                let color = if grid_color == Color::Reset {
                    moving_grid_color
                } else {
                    grid_color
                };

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color));

                block.render(*column, buf);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);

    tui::restore()?;
    app_result
}
