# tuitch
Tuitch is a Twitch chat TUI that implements Twitch chat's basic functionality into your terminal. You can join Twitch chat channels anonymously or with your own Twitch account using your Twitch OAuth token. The token is saved locally on your machine in a `Config.toml` file.

This project is ongoing and in early development stages. I do have plans for other future functionality, and will update the `README` and documentation accordingly.

## How does Tuich work?
Tuitch uses the [`twitch_irc`](https://docs.rs/twitch-irc/3.0.1/twitch_irc/) crate to communicate with the Twitch servers and [`termion`](https://docs.rs/termion/1.5.6/termion/) for a light and simple UI. See the `Cargo.toml` file for the full list of dependancies. 

## How to install
Right now, I haven't built any deployment or installation for the project, so you'll need to clone the repository yourself. This project is in early development and I only have so much free time on my hands.

## How to use Tuitch
Tuitch comes with very basic commands and functionality. A list of commands is shown on the home page when the appliction starts, they include `:join <channel>` to join a Twitch channel's chatroom and `:credentials` to update your config file's user credentials (username and the OAuth token).

## Planned features and contributions
If you would like to contribute to this project then I am open to pull requests and bug fixes. This project started as a learning opportunity and has grown into most of the functionality I set out to attempt. 

### Planned features include:
* Proper login handling to include changing users without restarting the application.
* Viewer lists for current channels.
* Color-scheme customization.
* Emote and Twitch Badge support.
* Tabs for multiple chats simultaneously.
