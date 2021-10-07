use std::unreachable;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ClearChatAction;
use twitch_irc::message::HostTargetAction;
use twitch_irc::message::ServerMessage;
use twitch_irc::message::UserNoticeEvent;
use twitch_irc::message::UserNoticeMessage;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

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
                // Format and print user chat messages:
                ServerMessage::Privmsg(prvmsg) => println!(
                    "{} [{}]: {}",
                    prvmsg.server_timestamp.format("%H:%M"),
                    prvmsg.sender.name,
                    prvmsg.message_text
                ),

                // User time-outs, bans, and a cleared chat history messages:
                ServerMessage::ClearChat(clearchat) => match clearchat.action {
                    ClearChatAction::UserBanned { user_login, .. } => {
                        println!("{} has been banned.", user_login)
                    }
                    ClearChatAction::UserTimedOut {
                        user_login,
                        timeout_length,
                        ..
                    } => println!(
                        "{} has been timed-out for {} seconds.",
                        user_login,
                        timeout_length.as_secs()
                    ),
                    ClearChatAction::ChatCleared => println!("Chat has been cleared."),
                },

                // Channel-hosting messages:
                ServerMessage::HostTarget(hosttargetmessage) => match hosttargetmessage.action {
                    HostTargetAction::HostModeOn {
                        hosted_channel_login,
                        viewer_count,
                    } => {
                        let viewer_count = viewer_count.unwrap_or(0);
                        println!(
                            "Hosted {} with {:#?} users",
                            hosted_channel_login, viewer_count
                        )
                    }
                    HostTargetAction::HostModeOff { .. } => println!("No longer hosting."),
                },

                // Event messages, raids, subs:
                ServerMessage::UserNotice(usernotice) => match usernotice.event {
                    UserNoticeEvent::SubOrResub {
                        is_resub,
                        cumulative_months,
                        streak_months,
                        sub_plan,
                        sub_plan_name,
                    } => {
                        if is_resub {
                            println!(
                                "{} resubscribed for {} months with {}!",
                                usernotice.sender.name, 
                                cumulative_months,
                                sub_plan,
                            )
                        } else {

                        }
                    }
                    _ => {}
                },

                // Simple server messages related to user and moderator actions and server-side
                // messages:

                //TODO: Look into connection messages.
                //TODO: Look into formatting, and proper message removal.
                ServerMessage::ClearMsg(_) => println!("Message deleted."),
                ServerMessage::GlobalUserState(_) => println!("Login successful!"),
                ServerMessage::Part(_) => println!("Departed chat."),
                ServerMessage::Notice(notice) => println!("{}", notice.message_text),
                ServerMessage::Join(join) => println!("Joined {}'s chat!", join.channel_login),

                // Any other events that do not need to be verbose
                _ => {}
            }
        }
    });

    //TODO: Insure proper error handling here.
    //TODO: Create user prompt for channel name.
    // join a channel
    client.join("brandontdev".to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting,
    // the background task will exit.
    join_handle.await.unwrap();
}
