use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{ Block, Borders };
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::MouseTerminal;

pub fn run_terminal() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;    
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Chat")
                .borders(Borders::ALL);
            f.render_widget(block, size);
    })?;
    Ok(())
}
