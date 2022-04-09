use serde::{Deserialize, Serialize};
use mongodb::bson::{Bson, oid::ObjectId};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub winner: String,
    pub loser: String,
    pub game_type: i8, // 0 = Connect4; 1 = TooT OttO
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub time: chrono::DateTime<Utc>,
}
