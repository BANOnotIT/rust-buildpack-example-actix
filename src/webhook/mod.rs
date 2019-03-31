//#[macro_use] extern crate serde_derive;
use actix_web::{error, Error, HttpResponse, web};
use base64;
use telegram_typings::{Message, Update};

use crate::BotState;

mod ban;
mod callback_commands;
mod report;
mod utils;

pub fn handle(state: web::Data<BotState>, data: web::Json<Update>) -> Result<HttpResponse, Error> {
//    let data = &data;
    let message = &data.message;
    let query = &data.callback_query;

    let unhandled = Ok(HttpResponse::Ok().finish());

    if let Some(message) = message {
        report::handle_report(&state, message)
    } else if let Some(data) = query {
        if data.data.is_none() {
            return unhandled;
        }
        let callback = data;
        let data = data.data.unwrap();
        let command = base64::decode(&data).map_err(|e| error::ErrorInternalServerError(e))?;

        let command =
            serde_cbor::from_slice(&command).map_err(|e| error::ErrorInternalServerError(e))?;

        match command {
            callback_commands::Command::BanAndRemove(user, messages) => {
                ban::handle_ban(&state, &user, &messages, &callback.message)
            }
            _ => unhandled,
        }
    } else {
        unhandled
    }
}

fn is_message_a_spam_report(msg: &Box<Message>) -> bool {
    msg.text.is_some() && msg.text.unwrap() == "/spam".to_owned() && msg.reply_to_message.is_some()
}
