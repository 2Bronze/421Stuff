use mongodb::{bson::doc, options::ClientOptions, Client};
use mongodb::bson::Bson;
mod user;
mod r#match;
mod api;
use user::User;
use r#match::Match;


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    api::test_connection().await?;
    api::test_signup_user().await?;
    if !api::signup_user("USer1".to_string(), "Password".to_string()).await? {
        println!("User already exists!");
    }

    if api::signin_user("USer1".to_string(), "Paword".to_string()).await? {
        println!("Signed in!");
    } else {
        println!("Failed to sign in!");
    }

    // api::insert_match("User1".to_string(), "User2".to_string(), 0).await?;


    let user = api::get_user("USer1".to_string()).await?;
    println!("User: {:#?}", user.unwrap());

    let matches = api::get_matches("User1".to_string()).await?;
    println!("Matches: {:#?}", matches.unwrap());
    Ok(())
}
