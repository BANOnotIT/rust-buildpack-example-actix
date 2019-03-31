use actix_web::{Error, error::ErrorInternalServerError, HttpResponse, web::Data};
use telegram_typings::Message;

use crate::BotState;

use super::callback_commands::{Command, MessageId};
use super::utils::create_btn;

pub fn handle_ban(
    state: &Data<BotState>,
    user: &i64,
    messages: &Vec<MessageId>,
    log_message: &Option<Box<Message>>,
) -> Result<HttpResponse, Error> {
    if log_message.is_none() {
        state
            .bot
            .send_message(
                state.channel_id,
                &format!("Ban user #{}", user),
                None,
                None,
                None,
            )
            .map_err(|a| ErrorInternalServerError(a))?;
    }

    //    remove all spam related messages
    messages.iter().for_each(|msg| {
        state.bot.delete_message(msg.0, msg.1);
    });

    // kick user from all chats
    state.chat_ids.iter().for_each(|&id| {
        state.bot.kick_chat_member(id, *user);
    });

    match log_message {
        None => Ok(HttpResponse::Ok().finish()),
        Some(msg) => {
            let id = msg.message_id;
            //            state.bot.edit_message_reply_markup();
            state
                .bot
                .send_message(state.channel_id, "Report...", None, None, None)
                .map(|message| HttpResponse::Ok().finish())
                .map_err(|a| ErrorInternalServerError(a))
        }
    }
}
