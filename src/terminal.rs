use anyhow::Result;
use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub fn run_terminal() -> Result<()> {
    let mut stdin = io::stdin().keys();
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Chat").borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        match stdin.next().unwrap().unwrap() {
            Key::Char('q') => break,
            _ => {}
        }
    }

    drop(terminal);
    Ok(())
}
