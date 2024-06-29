use std::io::{self, Empty};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};
use tetris::TBlockColor;

mod tetris;
mod tui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
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
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const NUM_ROWS: u16 = 20;
        const NUM_COLS: u16 = 10;

        let mut grid = [[TBlockColor::Empty; NUM_COLS as usize]; NUM_ROWS as usize];

        let cell_size = std::cmp::min(
            area.width / NUM_COLS * 2 as u16,
            area.height / NUM_ROWS as u16,
        );

        let total_width = cell_size * NUM_COLS * 2 as u16;
        let total_height = cell_size * NUM_ROWS as u16;

        let start_x = area.x + (area.width - total_width) / 2;
        let start_y = area.y + (area.height - total_height) / 2;

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
                let color = match grid[y][x] {
                    TBlockColor::Empty => Color::Reset,
                    TBlockColor::Red => Color::Red,
                    TBlockColor::Green => Color::Green,
                    TBlockColor::Blue => Color::Blue,
                    TBlockColor::Yellow => Color::Yellow,
                    TBlockColor::Magenta => Color::Magenta,
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
