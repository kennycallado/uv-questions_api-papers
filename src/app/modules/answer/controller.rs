use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::admin::AdminClaims;
use crate::config::database::Db;

use crate::app::modules::answer::model::Answer;
use crate::app::modules::answer::services::respository as answer_repository;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_index_admin, get_index_none, get_show_admin, get_show_none]
}

#[get("/", rank = 1)]
pub async fn get_index_admin(db: Db, _claims: AdminClaims) -> Result<Json<Vec<Answer>>, Status> {
    let answers = answer_repository::get_all(&db).await;

    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(e) => {
            println!("Error: Answer, Method: GET, Action: get_index_admin");
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/", rank = 5)]
pub async fn get_index_none() -> Status {
    println!("Module: Answer, Method: GET, Action: get_index_none");
    Status::Unauthorized
}

#[get("/<id>", rank = 1)]
pub async fn get_show_admin(id: i32, db: Db) -> Result<Json<Answer>, Status> {
    let answer = answer_repository::get_by_id(&db, id).await;

    match answer {
        Ok(answer) => Ok(Json(answer)),
        Err(e) => {
            println!("Error: Answer, Method: GET, Action: get_show_admin");
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    println!("Module: Answer, Method: GET, Action: get_show_none");
    Status::Unauthorized
}
