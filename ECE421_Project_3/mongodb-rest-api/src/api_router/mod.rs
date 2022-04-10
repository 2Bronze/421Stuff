use crate::api_service::{Match, User};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

// #[get("/get-all")]
// async fn get_all_json(app_data: web::Data<crate::AppState>) -> impl Responder {
//     let action = app_data.service_manager.api.get_json();
//     let result = web::block(move || action).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[get("/get-user/{username}")]
async fn get_user(app_data: web::Data<crate::AppState>, username: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.get_user(&username);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/signup")]
async fn signup(app_data: web::Data<crate::AppState>, data: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.api.get_user(&data.username);
    let result = web::block(move || action).await;
    match result {
        Ok(option) => {
            match option {
                Some(_) => return HttpResponse::Unauthorized().finish(),
                None => {
                    //Expected Result
                },
            }
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    let action = app_data.service_manager.api.create_user(&data);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => {
                HttpResponse::Ok().json(result.inserted_id)
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/signin")]
async fn signin(app_data: web::Data<crate::AppState>, data: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.api.get_user(&data.username);
    let result = web::block(move || action).await;
    match result {
        Ok(option) => {
            match option {
                Some(document) => {
                    if document.get("password").unwrap().as_str().unwrap() == data.password {
                        return HttpResponse::Ok().finish();
                    } else {
                        return HttpResponse::Unauthorized().finish();
                    }
                },
                None => return HttpResponse::Unauthorized().finish(),
            }
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

#[post("/add-match")]
async fn add_match(app_data: web::Data<crate::AppState>, data: web::Json<Match>) -> impl Responder {
    let action = app_data.service_manager.api.create_match(&data);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-matches/{username}")]
async fn get_matches(app_data: web::Data<crate::AppState>, username: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.get_matches(&username);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-all-matches")]
async fn get_all_matches(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.api.get_all_matches();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/update-user/{username}")]
async fn update_user(app_data: web::Data<crate::AppState>, user: web::Json<User>, username: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.update_user(&user, &username);
    let result = web::block(move || action).await;
    match result {
            Ok(result) => HttpResponse::Ok().json(result.modified_count),
            Err(e) => {
                println!("Error while getting, {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}

#[delete("/delete-user")]
async fn delete_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.api.get_user(&user.username);
    let result = web::block(move || action).await;
    match result {
        Ok(option) => {
            match option {
                Some(document) => {
                    if document.get("password").unwrap().as_str().unwrap() == user.password {
                        //Is correct
                    } else {
                        return HttpResponse::Unauthorized().finish();
                    }
                },
                None => return HttpResponse::Unauthorized().finish(),
            }
        },
        Err(e) => {
            println!("Error while getting, {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    let action = app_data.service_manager.api.delete_user(&user);
    let result = web::block(move || action).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[post("/update-match/{_id}")]
async fn update_match(app_data: web::Data<crate::AppState>, match_data: web::Json<Match>, _id: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.api.update_match(&match_data, &_id);
    let result = web::block(move || action).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/delete-match/{_id}")]
async fn delete_match(app_data: web::Data<crate::AppState>, _id: web::Path<String>) -> impl Responder{
    let action = app_data.service_manager.api.delete_match(&_id);
    let result = web::block(move || action).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
// #[post("/update/{param}")]
// async fn update_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>, param: web::Path<String>) -> impl Responder {
//     let action = app_data.service_manager.api.update(&data, &param);
//     let result = web::block(move || action).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result.modified_count),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }
//
// #[delete("/delete")]
// async fn delete_user(app_data: web::Data<crate::AppState>, data: web::Json<Data>) -> impl Responder {
//     let action = app_data.service_manager.api.delete(&data.title);
//     let result = web::block(move || action).await;
//     match result {
//         Ok(result) => HttpResponse::Ok().json(result.deleted_count),
//         Err(e) => {
//             println!("Error while getting, {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(signup);
    cfg.service(signin);
    cfg.service(get_user);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(add_match);
    cfg.service(get_matches);
    cfg.service(get_all_matches);
    cfg.service(update_match);
    cfg.service(delete_match);
    // cfg.service(get_all_json);
}
