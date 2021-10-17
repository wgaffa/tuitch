use twitch_irc::message::ClearChatAction;
use twitch_irc::message::HostTargetAction;
use twitch_irc::message::ServerMessage;
use twitch_irc::message::UserNoticeEvent;

// TODO: use owo-colors crate for color and style formatting.

pub fn handle_message(message: ServerMessage) {
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
                println!("{name} has been banned.", name = user_login)
            }
            ClearChatAction::UserTimedOut {
                user_login,
                timeout_length,
                ..
            } => println!(
                "{name} has been timed-out for {seconds} seconds.",
                name = user_login,
                seconds = timeout_length.as_secs()
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
                        "{name} has subscribed for {months} months with {plan}!",
                        name = usernotice.sender.name,
                        months = cumulative_months,
                        plan = sub_plan,
                    )
                } else {
                    println!(
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
                println!(
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
                    println!(
                        "An anonymous user gifted {} a {} for {:?}!",
                        recipient.name, sub_plan, num_gifted_months,
                    )
                } else {
                    println!(
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
                println!(
                    "{} is gifting {} subs! They've gifted a total of {}!",
                    usernotice.sender.name, mass_gift_count, sender_total_gifts,
                )
            }

            UserNoticeEvent::AnonSubMysteryGift {
                mass_gift_count,
                sub_plan,
            } => {
                println!("An anonymous user is gifting {} subs!", mass_gift_count)
            }

            UserNoticeEvent::GiftPaidUpgrade {
                gifter_login,
                gifter_name,
                promotion,
            } => {
                println!(
                    "{} continued their gifted sub from {}!",
                    usernotice.sender.name, gifter_name
                )
            }

            UserNoticeEvent::AnonGiftPaidUpgrade { promotion } => {
                println!(
                    "{} continued their gifted sub from an anonymous user!",
                    usernotice.sender.name
                )
            }

            UserNoticeEvent::Ritual { ritual_name } => {
                println!("{} is new to chat! Say hi!", usernotice.sender.name)
            }

            UserNoticeEvent::BitsBadgeTier { threshold } => {
                println!(
                    "{} just earned the {} bits badge!",
                    usernotice.sender.name, threshold
                )
            }

            _ => {}
        },

        // Simple server messages related to user and moderator actions and
        // server-side messages:

        //TODO: Look into connection messages.
        //TODO: Look into and proper message removal.
        ServerMessage::ClearMsg(_) => println!("Message deleted."),
        ServerMessage::GlobalUserState(_) => println!("Login successful!"),
        ServerMessage::Part(_) => println!("Departed chat."),
        ServerMessage::Notice(notice) => println!("{}", notice.message_text),
        ServerMessage::Join(join) => println!("Joined {}'s chat!", join.channel_login),

        // Any other events that do not need to be verbose
        _ => {}
    }
}
