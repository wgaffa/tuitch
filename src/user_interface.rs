use std::io::{stdout, Write};
use termion::terminal_size;

pub async fn reset_screen() {
    let (x, y) = terminal_size().unwrap();
    print!("{}", termion::clear::All);
    print!("{}> ", termion::cursor::Goto(1, y));
    stdout().lock().flush();
}
