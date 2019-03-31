use reqwest::{Client, Result};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use telegram_typings::{InlineKeyboardMarkup, Message};

#[derive(Deserialize)]
struct BotResponse<T> {
    ok: bool,
    description: Option<String>,
    result: Option<T>,
}

#[derive(Clone)]
pub struct Bot {
    client: Client,
    token: String,
}

pub enum ParseMode {
    Markdown,
    HTML,
}

impl Bot {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    fn get_url(&self, method: &str) -> String {
        format!("https://api.telegram.org/bot{}/{}", self.token, method)
    }
    fn do_request<A: DeserializeOwned, F: Serialize + ?Sized>(
        &self,
        method: &str,
        fields: &F,
    ) -> Result<BotResponse<A>> {
        self.client
            .post(&self.get_url(method))
            .form(&fields)
            .send()
            .and_then(|mut a| a.json())
    }

    pub fn kick_chat_member(&self, chat_id: i64, user_id: i64) -> Result<bool> {
        let fields = [("chat_id", chat_id), ("user_id", user_id)];

        self.do_request("kickChatMember", &fields)
            .and_then(|a| Ok(a.result == Some(true)))
    }
    pub fn unban_chat_member(&self, chat_id: i64, user_id: i64) -> Result<bool> {
        let fields = [("chat_id", chat_id), ("user_id", user_id)];

        self.do_request("unbanChatMember", &fields)
            .and_then(|a| Ok(a.result == Some(true)))
    }

    pub fn delete_message(&self, chat_id: i64, message_id: i64) -> Result<bool> {
        let fields = [("chat_id", chat_id), ("message_id", message_id)];

        self.do_request("deleteMessage", &fields)
            .and_then(|a| Ok(a.result == Some(true)))
    }
    pub fn edit_message_text(&self, chat_id: i64, message_id: i64, text: &str) -> Result<bool> {
        #[derive(Serialize)]
        struct Fields<'a> {
            chat_id: i64,
            message_id: i64,
            text: &'a str,
        }

        let fields = Fields {
            chat_id,
            message_id,
            text,
        };

        self.do_request("editMessage", &fields)
            .and_then(|a| Ok(a.result == Some(true)))
    }

    pub fn edit_message_reply_markup(
        &self,
        chat_id: i64,
        message_id: i64,
        markup: InlineKeyboardMarkup,
    ) -> Result<Message> {
        #[derive(Serialize)]
        struct Fields {
            chat_id: i64,
            message_id: i64,
            reply_markup: InlineKeyboardMarkup,
        }

        let fields = Fields {
            chat_id,
            message_id,
            reply_markup: markup,
        };

        self.do_request("sendMessage", &fields).map(|a| {
            a.result
                .expect("Expected message structure in response result")
        })
    }

    pub fn send_message(
        &self,
        chat_id: i64,
        text: &str,
        parse_mode: Option<ParseMode>,
        reply_to_message_id: Option<i64>,
        reply_markup: Option<InlineKeyboardMarkup>,
    ) -> Result<Message> {
        #[derive(Serialize)]
        struct Fields<'a> {
            chat_id: i64,
            text: &'a str,
            parse_mode: Option<&'static str>,
            reply_to_message_id: Option<i64>,
            reply_markup: Option<InlineKeyboardMarkup>,
        }

        let fields = Fields {
            chat_id,
            text,
            reply_to_message_id,
            reply_markup,
            parse_mode: match parse_mode {
                Some(mode) => match mode {
                    ParseMode::HTML => Some("HTML"),
                    ParseMode::Markdown => Some("Markdown"),
                },
                None => None,
            },
        };

        self.do_request("sendMessage", &fields).map(|a| {
            a.result
                .expect("Expected message structure in response result")
        })
    }
}
