use mongodb::{bson::doc, options::ClientOptions, Client};
use futures::stream::{StreamExt, TryStreamExt};
use bson;
use chrono::Utc;
use crate::User;
use crate::Match;

pub async fn test_connection() -> mongodb::error::Result<()> {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}

pub async fn test_signup_user() -> mongodb::error::Result<()> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let users = client.database("main").collection("users");

    let test_user = User {
        id: None,
        username: "test_user".to_owned(),
        password: "user_pw".to_owned(),
        wins: 0,
        losses: 0,
    };

    let serialized_user = bson::to_bson(&test_user)?;
    let document = serialized_user.as_document().unwrap();

    let insert_result = users.insert_one(document.to_owned(), None).await?;
    Ok(())
}

pub async fn signup_user(username: String, password: String) -> mongodb::error::Result<bool> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let users = client.database("main").collection("users");

    let insert_user = User {
        id: None,
        username: username.to_owned(),
        password: password.to_owned(),
        wins: 0,
        losses: 0,
    };
    //If found then error
    if let Some(_) = users.find_one(doc! {
            "username": username.to_owned(),
        },
        None,
    ).await? {
        return Ok(false);
    }

    let serialized_user = bson::to_bson(&insert_user)?;
    let document = serialized_user.as_document().unwrap();

    users.insert_one(document.to_owned(), None).await?;
    Ok(true)
}

pub async fn signin_user(username: String, password: String) -> mongodb::error::Result<bool> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let users = client.database("main").collection::<User>("users");

    if let Some(user) = users.find_one(doc! {
            "username": username.to_owned(),
        },
        None,
    ).await? {
        if user.password == password {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
    return Ok(false);
}

pub async fn get_user(username: String) -> mongodb::error::Result<Option<User>> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let users = client.database("main").collection("users");

    let user = users.find_one(doc! {
            "username": username.to_owned(),
        },
        None,
    ).await?;

    Ok(user)
}

pub async fn insert_match(winner: String, loser: String, game_type: i8) -> mongodb::error::Result<()> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let match_history = client.database("main").collection("match_history");

    let insert_match = Match {
        id: None,
        winner: winner,
        loser: loser,
        game_type: game_type,
        time: Utc::now(),
    };

    let serialized_match = bson::to_bson(&insert_match)?;
    let document = serialized_match.as_document().unwrap();

    match_history.insert_one(document.to_owned(), None).await?;
    Ok(())
}

pub async fn get_matches(username: String) -> mongodb::error::Result<Option<Vec<Match>>> {
    let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    let match_history = client.database("main").collection::<Match>("match_history");

    let matches = match_history.find(doc! {
        "$or": [
            {"winner": username.to_owned()},
            {"loser": username.to_owned()}
        ],
    }, None).await?;
    let v :Vec<_> = matches.try_collect().await?;
    Ok(Some(v))
}
