use actix_web::{Error, error::ErrorInternalServerError, HttpResponse, web::Data};
use telegram_typings::{InlineKeyboardMarkup, Message};

use crate::BotState;

use super::callback_commands::{Command, MessageId};
use super::utils::create_btn;

pub fn handle_report(
    state: &Data<BotState>,
    report_message: &Box<Message>,
) -> Result<HttpResponse, Error> {
    let spam_message = report_message.reply_to_message;
    let spam_message = spam_message.unwrap();

    let mut ban_btn = create_btn("Ban");
    let ban_command = Command::BanAndRemove(
        spam_message.from.unwrap().id,
        vec![
            MessageId(report_message.chat.id, spam_message.message_id),
            MessageId(report_message.chat.id, report_message.message_id),
        ],
    );
    let ban_command = serde_cbor::to_vec(&ban_command).map_err(|e| ErrorInternalServerError(e))?;
    ban_btn.callback_data = Some(base64::encode(&ban_command));

    let mut pardon_btn = create_btn("Pardon");
    let pardon_command = Command::Remove(vec![MessageId(
        report_message.chat.id,
        report_message.message_id,
    )]);
    let pardon_command =
        serde_cbor::to_vec(&pardon_command).map_err(|e| ErrorInternalServerError(e))?;
    pardon_btn.callback_data = Some(base64::encode(&pardon_command));

    let btns = vec![ban_btn, pardon_btn];
    let keyboard = InlineKeyboardMarkup {
        inline_keyboard: vec![btns],
    };

    state
        .bot
        .send_message(state.channel_id, "Report...", None, None, Some(keyboard))
        .map(|message| HttpResponse::Ok().finish())
        .map_err(|a| ErrorInternalServerError(a))
}
