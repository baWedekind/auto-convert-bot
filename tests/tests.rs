use telegram_bot::*;
use auto_convert_bot::{ChatSettings, AutoConvertBot};

#[test]
fn test_insert_fetch_equality() {
    let mut bot = AutoConvertBot::bot_setup();
    // fail on panic
    bot.register_chat(&ChatId::new(1)).unwrap();
    let returned_settings = bot.fetch_chat_settings(1).unwrap();
    let default_settings = ChatSettings::default();

    assert_eq!(returned_settings, default_settings);
}

#[test]
fn test_empty_fetch() {
    let mut bot = auto_convert_bot::AutoConvertBot::bot_setup();
    assert_eq!(bot.fetch_chat_settings(0), Err(diesel::result::Error::NotFound))
}