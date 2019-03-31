use serde::{Deserialize, Serialize};

type UserId = i64;

#[derive(Serialize, Deserialize)]
pub struct MessageId(pub i64, pub i64);

#[derive(Serialize, Deserialize)]
//#[serde(tag = "action")]
pub enum Command {
    BanAndRemove(UserId, Vec<MessageId>),
    Remove(Vec<MessageId>),
    Mute(UserId),
    Unban(UserId),
}
