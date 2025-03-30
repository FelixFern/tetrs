use std::{
    io::{self},
    time::{Duration, Instant},
};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use tetris::{TBlockColor, TetrisBlock};

mod tetris;
mod tui;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 10;

const MOVE_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct App {
    exit: bool,
    grid: [[TBlockColor; NUM_COLS]; NUM_ROWS],
    tetris_block: TetrisBlock,
    last_tick: Instant,
    score: u128,
}

impl Default for App {
    fn default() -> Self {
        let grid = [[TBlockColor::Empty; NUM_COLS]; NUM_ROWS];
        App {
            exit: false,
            grid,
            tetris_block: TetrisBlock::new(1, 0, tetris::TBlockType::random()),
            last_tick: Instant::now(),
            score: 0,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            self.update();
            terminal.draw(|f: &mut Frame| self.render_frame(f))?;
            self.handle_events()?;
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
            KeyCode::Left => self.tetris_block.move_left(self.grid),
            KeyCode::Right => self.tetris_block.move_right(self.grid),
            KeyCode::Down => {
                self.tetris_block.move_down(self.grid);
                ()
            }
            KeyCode::Char('z') => self.tetris_block.rotate_counter_clockwise(self.grid),
            KeyCode::Char('x') => self.tetris_block.rotate_clockwise(self.grid),
            _ => {}
        }
    }

    fn update(&mut self) {
        if self.last_tick.elapsed() >= MOVE_INTERVAL {
            if !self.tetris_block.move_down(self.grid) {
                let (block, color) = self.tetris_block.get_pos();
                block.map(|f| self.grid[f.1 as usize][f.0 as usize] = color);
                self.clear_line();
                self.tetris_block = TetrisBlock::new(4, 0, tetris::TBlockType::random());
            }
            self.last_tick = Instant::now();
        }
    }

    fn clear_line(&mut self) {
        let mut new_grid = [[TBlockColor::Empty; NUM_COLS]; NUM_ROWS];
        let mut new_row_index = NUM_ROWS - 1;
        let mut lines_cleared = 0;

        for y in (0..NUM_ROWS).rev() {
            if self.grid[y].iter().all(|&cell| cell != TBlockColor::Empty) {
                lines_cleared += 1;
            } else {
                // Copy the row to the new grid
                new_grid[new_row_index] = self.grid[y];
                if new_row_index > 0 {
                    new_row_index -= 1;
                }
            }
        }

        self.grid = new_grid;

        self.score += match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let score_area = Rect::new(area.x, area.y, area.width, 3);
        let game_area = Rect::new(area.x, area.y + 3, area.width, area.height - 3);

        let score_widget = Paragraph::new(format!("Score: {}", self.score))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
        score_widget.render(score_area, buf);

        let cell_size = std::cmp::min(
            game_area.width / (NUM_COLS as u16) * 2,
            game_area.height / NUM_ROWS as u16,
        );

        let total_width = cell_size * (NUM_COLS as u16) * 2;
        let total_height = cell_size * NUM_ROWS as u16;

        let start_x = game_area.x + (game_area.width - total_width) / 2;
        let start_y = game_area.y + (game_area.height - total_height) / 2;

        let mut moving_grid = [[TBlockColor::Empty; NUM_COLS]; NUM_ROWS];

        let (block, color) = self.tetris_block.get_pos();

        block.map(|f| moving_grid[f.1 as usize][f.0 as usize] = color);

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
                    TBlockColor::Cyan => Color::Cyan,
                    TBlockColor::Orange => Color::Indexed(208),
                };

                let grid_color: Color = match self.grid[y][x] {
                    TBlockColor::Empty => Color::Reset,
                    TBlockColor::Red => Color::Red,
                    TBlockColor::Green => Color::Green,
                    TBlockColor::Blue => Color::Blue,
                    TBlockColor::Yellow => Color::Yellow,
                    TBlockColor::Magenta => Color::Magenta,
                    TBlockColor::Cyan => Color::Cyan,
                    TBlockColor::Orange => Color::Indexed(208),
                };

                let color = if grid_color == Color::Reset {
                    moving_grid_color
                } else {
                    grid_color
                };

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::DOUBLE)
                    .border_style(Style::default().fg(color))
                    .bg(color);

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
