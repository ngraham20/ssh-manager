use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
             .borders(Borders::NONE);
        f.render_widget(block, chunks[1]);
        let chunks2 = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50)
                ].as_ref()
            )
            .split(tui::layout::Rect {
                x: chunks[1].x,
                y: chunks[1].y,
                width: chunks[1].width,
                height: chunks[1].height
            });
            let block = Block::default()
            .title("Block 3")
            .borders(Borders::ALL);
       f.render_widget(block, chunks2[0]);
       let block = Block::default()
            .title("Block 4")
            .borders(Borders::ALL);
       f.render_widget(block, chunks2[1]);
    })
}