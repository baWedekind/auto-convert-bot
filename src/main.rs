use futures::StreamExt;
use telegram_bot::*;

use auto_convert_bot;
use std::ops::AddAssign;

extern crate diesel;
extern crate dotenv;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut bot = auto_convert_bot::AutoConvertBot::bot_setup();

    // Fetch new updates via long poll method
    let mut stream = bot.api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            // this loop exists just to enable the break out of match
            loop { match message.kind {
                MessageKind::Text { ref data, .. } => match data.as_str() {
                    "/start" => {
                        if let Err(_) = bot.register_chat(&message.chat.id()) {
                            // This should never happen if all tests run, but lets catch it
                            bot.api.send(message.text_reply("An Error occurred while \
                             initialising your settings. Please contact my creator.")).await?;
                        }
                    },
                    _ => {
                        // Any noncommand message has been sent in the chat,
                        // check for each measurement
                        if !bot.chat_map.contains_key(&message.chat.id()) {
                            match bot
                                .fetch_chat_settings(message.chat.id().to_string().parse().unwrap())
                            {
                                Ok(_) => (), // bot now has chat_settings
                                Err(_) => {
                                    if let Err(_) = bot.register_chat(&message.chat.id()) {
                                        bot.api.send(message.text_reply(
                                            // This should never happen if all tests run, but lets
                                            // catch it
                                            "Sorry, but your settings are broken, please \
                                            use /restore_defaults to restore functionality.\n\
                                            We apologize for the inconvenience")).await?;
                                        break;
                                    }
                                }
                            }
                        }
                        let measurement_list =
                            bot.detect_measurements_and_convert(&message.chat.id(), data);

                        // Print received text message to stdout.
                        println!("<{}>: {}", &message.from.first_name, data);
                        // Answer message with "Hi".
                        // TODO: replace with Display of Units into the send
                        let mut response_string = String::from("Your units:\n");
                        for unit in measurement_list {
                            response_string.add_assign(format!("{}\n", unit).as_str());
                        }
                        bot.api.send(message.text_reply(response_string)).await?;
                    }
                },
                MessageKind::NewChatMembers { ref data } => {
                    // if the bot is one of the members, bot is new and must be setup for the chat
                    for user in data {
                        if user.username == Some("@auto_convert_bot".to_owned()) {
                            if let Err(_) = bot.register_chat(&message.chat.id()) {
                                // This should never happen if all tests run, but lets catch it
                                bot.api.send(message.text_reply("An Error occurred \
                                while initialising your settings. Please contact my creator."))
                                    .await?;
                            }
                        }
                    }
                }
                _ => {}
            } break;}
        }
    }
    Ok(())
}
