use telegram_typings::InlineKeyboardButton;

pub fn create_btn(text: &str) -> Box<InlineKeyboardButton> {
    Box::new(InlineKeyboardButton {
        text: text.to_string(),
        callback_data: None,
        switch_inline_query: None,
        url: None,
        switch_inline_query_current_chat: None,
        callback_game: None,
        pay: None,
    })
}
