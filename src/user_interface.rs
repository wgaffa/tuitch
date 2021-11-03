use owo_colors::OwoColorize;
use std::io::{stdout, Write};
use termion::terminal_size;

pub async fn reset_screen() {
    let (x, y) = terminal_size().unwrap();
    print!(
        "{}{}> {}\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, y),
        "Enter a message or command".dimmed(),
        termion::cursor::Right(2)
    );
    stdout().lock().flush();
}
