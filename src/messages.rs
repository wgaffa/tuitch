use twitch_irc::message::ClearChatAction;
use twitch_irc::message::HostTargetAction;
use twitch_irc::message::ServerMessage;
use twitch_irc::message::UserNoticeEvent;

// TODO: use owo-colors crate for color and style formatting.

pub fn format_message(message: ServerMessage) -> String {
    match message {
        // Format and print user chat messages:
        ServerMessage::Privmsg(prvmsg) => format!(
            "{} [{}]: {}",
            prvmsg.server_timestamp.format("%H:%M"),
            prvmsg.sender.name,
            prvmsg.message_text
        ),

        // User time-outs, bans, and a cleared chat history messages:
        ServerMessage::ClearChat(clearchat) => match clearchat.action {
            ClearChatAction::UserBanned { user_login, .. } => {
                format!("{name} has been banned.", name = user_login)
            }
            ClearChatAction::UserTimedOut {
                user_login,
                timeout_length,
                ..
            } => format!(
                "{name} has been timed-out for {seconds} seconds.",
                name = user_login,
                seconds = timeout_length.as_secs()
            ),
            ClearChatAction::ChatCleared => format!("Chat has been cleared."),
        },

        // Channel-hosting messages:
        ServerMessage::HostTarget(hosttargetmessage) => match hosttargetmessage.action {
            HostTargetAction::HostModeOn {
                hosted_channel_login,
                viewer_count,
            } => {
                let viewer_count = viewer_count.unwrap_or(0);
                format!(
                    "Hosted {} with {:#?} users",
                    hosted_channel_login, viewer_count
                )
            }
            HostTargetAction::HostModeOff { .. } => format!("No longer hosting."),
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
                    format!(
                        "{name} has subscribed for {months} months with {plan}!",
                        name = usernotice.sender.name,
                        months = cumulative_months,
                        plan = sub_plan,
                    )
                } else {
                    format!(
                        "{name} has just subscribed with {plan}!",
                        name = usernotice.sender.name,
                        plan = sub_plan,
                    )
                }
            }

            UserNoticeEvent::Raid {
                viewer_count,
                profile_image_url,
            } => {
                format!(
                    "{name} raided with {viewers} viewers!",
                    name = usernotice.sender.name,
                    viewers = viewer_count,
                )
            }

            UserNoticeEvent::SubGift {
                is_sender_anonymous,
                cumulative_months,
                recipient,
                sub_plan,
                sub_plan_name,
                num_gifted_months,
            } => {
                if is_sender_anonymous {
                    format!(
                        "An anonymous user gifted {} a {} for {:?}!",
                        recipient.name, sub_plan, num_gifted_months,
                    )
                } else {
                    format!(
                        "{} gifted {} a {} for {:?}!",
                        usernotice.sender.name, recipient.name, sub_plan, num_gifted_months,
                    )
                }
            }

            UserNoticeEvent::SubMysteryGift {
                mass_gift_count,
                sender_total_gifts,
                sub_plan,
            } => {
                format!(
                    "{} is gifting {} subs! They've gifted a total of {}!",
                    usernotice.sender.name, mass_gift_count, sender_total_gifts,
                )
            }

            UserNoticeEvent::AnonSubMysteryGift {
                mass_gift_count,
                sub_plan,
            } => {
                format!("An anonymous user is gifting {} subs!", mass_gift_count)
            }

            UserNoticeEvent::GiftPaidUpgrade {
                gifter_login,
                gifter_name,
                promotion,
            } => {
                format!(
                    "{} continued their gifted sub from {}!",
                    usernotice.sender.name, gifter_name
                )
            }

            UserNoticeEvent::AnonGiftPaidUpgrade { promotion } => {
                format!(
                    "{} continued their gifted sub from an anonymous user!",
                    usernotice.sender.name
                )
            }

            UserNoticeEvent::Ritual { ritual_name } => {
                format!("{} is new to chat! Say hi!", usernotice.sender.name)
            }

            UserNoticeEvent::BitsBadgeTier { threshold } => {
                format!(
                    "{} just earned the {} bits badge!",
                    usernotice.sender.name, threshold
                )
            }

            _ => {
                format!("Unknown Message")
            }
        },

        // Simple server messages related to user and moderator actions and
        // server-side messages:

        //TODO: Look into and proper message removal.
        ServerMessage::ClearMsg(_) => format!("Message deleted."),
        ServerMessage::GlobalUserState(_) => format!("Login successful!"),
        ServerMessage::Part(_) => format!("Departed chat."),
        ServerMessage::Notice(notice) => format!("{}", notice.message_text),
        ServerMessage::Join(join) => format!("Joined {}'s chat!", join.channel_login),

        // Any other events that do not need to be verbose
        _ => {
            format!("Unknown Message")
        }
    }
}

pub fn print_message(message: String) {
    print!("ESC7");
    print!("{}", message);
}
