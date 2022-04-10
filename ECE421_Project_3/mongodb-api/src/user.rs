use serde::{Deserialize, Serialize};
use mongodb::bson::{Bson, oid::ObjectId};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
    pub wins: i32,
    pub losses: i32,
}
