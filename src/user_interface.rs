use owo_colors::OwoColorize;
use std::io::{stdout, Write};
use termion::terminal_size;

pub async fn reset_screen() {
    let (_x, y) = terminal_size().unwrap();
    print!(
        "{}{}> {}\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, y),
        "Enter a message or command".dimmed(),
        termion::cursor::Right(2)
    );
    stdout().lock().flush().unwrap();
}

pub async fn home_screen() {
    let (_x, y) = terminal_size().unwrap();
    print!(
        "{}{}{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n\n{}> {}\r{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        "Commands:",
        "To join a channel's chat, enter :join <channel>",
        "To change your login credentials, enter :credentials <username> <OAuth token>",
        "(Your OAuth token is saved locally, however at this time it is not encryptid,",
        "please never share your OAuth token with anyone.)",
        "Enter :help to bring up this help documentation.",
        "If you have any suggestions or would like to report any bugs, please visit the",
        "project's GitHub repository at https://github.com/brandontdev/tuitch.",
        termion::cursor::Goto(1, y),
        "Enter a message or command".dimmed(),
        termion::cursor::Right(2)
    );
    stdout().lock().flush().unwrap();
}
