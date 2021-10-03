use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

#[tokio::main]
pub async fn main() {
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming 
    // messages, otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(prvmsg) =>
                    println!("{} [{}]: {}", prvmsg.server_timestamp.format("%H:%M"), prvmsg.sender.name, prvmsg.message_text),
                //TODO: ServerMessage::ClearChat
                //TODO: ServerMessage::ClearMsg
                //TODO: ServerMessage::GlobalUserState
                //TODO: ServerMessage::HostTarget
                //TODO: ServerMessage::Join
                //TODO: ServerMessage::Notice
                //TODO: ServerMessage::Part
                //TODO: ServerMessage::Reconnect
                //TODO: ServerMessage::UserNotice
                _ =>
                    println!("nothing"),
            }
        }
    });

    //TODO: Insure proper error handling here.
    // join a channel
    client.join("brandontdev".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    join_handle.await.unwrap();
}
