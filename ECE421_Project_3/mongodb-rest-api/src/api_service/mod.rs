// External imports
use bson::{doc, Document, oid::ObjectId};
use mongodb::results::{DeleteResult, UpdateResult, InsertOneResult};
use mongodb::{error::Error, Collection};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use chrono::serde::ts_seconds;
// External constructors
extern crate serde;
extern crate serde_json;

// Estructure data for DB
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
    pub wins: i32,
    pub losses: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Match {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub winner: String,
    pub loser: String,
    pub game_type: i32, // 0 = Connect4; 1 = TooT OttO
    #[serde(with = "ts_seconds")]
    pub time: chrono::DateTime<Utc>, //nessary field
}

// Reference colection clone
#[derive(Clone)]
pub struct ApiService {
    users: Collection,
    match_history: Collection,
}

fn user_to_document(user: &User) -> Document {
    let User {
        username,
        password,
        ..
    } = user;
    doc! {
        "username": username,
        "password": password,
        "wins": 0,
        "losses": 0,
    }
    // let serialized_user = bson::to_bson(&user).unwrap();
    // serialized_user.as_document().unwrap().to_owned()
}

// Transform data to mongo db document
fn match_to_document(match_data: &Match) -> Document {
    let Match {
        winner,
        loser,
        game_type,
        ..
    } = match_data;
    doc! {
        "winner": winner,
        "loser": loser,
        "game_type": game_type,
        "time": Utc::now()
    }
}

// Functions with quieries to Mongo
impl ApiService {
    pub fn new(users: Collection, match_history: Collection) -> ApiService {
        ApiService { users, match_history }
    }

    // Insert data to Mongo DB
    pub fn create_user(&self, _user: &User) -> Result<InsertOneResult, Error> {
        self.users.insert_one(user_to_document(_user), None)
    }

    pub fn signup(&self, _user: &User) -> Result<InsertOneResult, Error> {
        let document = self.users.find_one(doc! {
            "username": _user.username.to_owned(),
        }, None).ok().expect("Failed to execute find!");

        self.users.insert_one(user_to_document(_user), None)
    }

    pub fn signin(&self, _user: &User) -> Result<bool, Error> {
        let document = self.users.find_one(doc! {
            "username": _user.username.to_owned(),
        }, None).ok().expect("Failed to execute find!");

        if document.unwrap().get("password").unwrap().as_str().unwrap() == _user.password {
            return Ok(true);
        }
        Ok(false)
    }

    pub fn create_match(&self, _match: &Match) -> Result<InsertOneResult, Error> {
        self.match_history.insert_one(match_to_document(_match), None)
    }

    pub fn get_user(&self, username: &String) -> Result<Option<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let document = self.users.find_one(doc! {
            "username": username,
        }, None).ok().expect("Failed to execute find!");
        Ok(document)
    }

    pub fn get_matches(&self, username: &String) -> Result<Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
        let cursor = self.match_history.find(doc! {
            "$or": [
                {"winner": username.to_owned()},
                {"loser": username.to_owned()}
            ],
        }, None).ok().expect("Failed to execute find!");
        let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }

    // Update an existing document
    // pub fn update(&self, _data:&Data, _param: &String) -> Result<UpdateResult, Error> {
    //     let object_param = bson::oid::ObjectId::with_string(_param).unwrap();
    //     self.collection.update_one(doc! { "_id": object_param }, data_to_document(_data), None)
    // }
    //
    // // Delete some document
    // pub fn delete(&self, _title: &String) -> Result<DeleteResult, Error> {
    //     self.collection.delete_one(doc! { "title": _title }, None)
    // }
    //
    // // Get all documents
    // pub fn get_json(&self) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    //     let cursor = self.collection.find(None, None).ok().expect("Failed to execute find.");
    //     let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    //     Ok(docs)
    // }
    //
    // // Get documents with quiery
    // pub fn get_by(&self, param: &String) -> std::result::Result<std::vec::Vec<bson::ordered::OrderedDocument>, mongodb::error::Error> {
    //     let cursor = self.collection.find(doc! { "author": { "$regex": param } }, None).ok().expect("Failed to execute find.");
    //     let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    //     let _serialized = serde_json::to_string(&docs).unwrap();
    //     Ok(docs)
    // }
}
