use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;

use crate::app::providers::guards::admin::AdminClaims;
use crate::config::database::Db;

use crate::app::modules::paper::model::{NewPaper, Paper, PaperWithAnswers};

use crate::app::modules::paper::services::repository as paper_repository;
use crate::app::modules::paper_answers::services::repository as paper_answers_repository;

use super::model::PaperWithNewAnswers;

pub fn routes() -> Vec<Route> {
    routes![
        get_index_admin,
        get_index_none,
        get_show_admin,
        get_show_none,
        create_paper_admin,
        create_paper_none,
        update_paper_admin,
        update_paper_none,
    ]
}

#[get("/", rank = 1)]
pub async fn get_index_admin(db: Db, _admin: AdminClaims) -> Result<Json<Vec<Paper>>, Status> {
    let papers = paper_repository::get_all(&db).await;

    match papers {
        Ok(papers) => Ok(Json(papers)),
        Err(e) => {
            println!("Error: Paper, Method: GET, Action: get_index_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/", rank = 5)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 1)]
pub async fn get_show_admin(db: Db, id: i32) -> Result<Json<PaperWithAnswers>, Status> {
    let paper = paper_repository::get_by_id(&db, id).await;

    if let Err(e) = paper {
        println!("Error: Paper, Method: GET, Action: get_show_admin");
        println!("Error: {:?}", e);
        return Err(Status::NotFound);
    }
    let paper = paper.unwrap();

    let answers = paper_answers_repository::get_answers_by_paper_id(&db, paper.id.clone()).await;
    if let Err(e) = answers {
        println!("Error: Paper, Method: GET, Action: get_show_admin");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let answers = answers.unwrap();

    let paper_answers = PaperWithAnswers {
        id: paper.id,
        user_id: paper.user_id,
        form_id: paper.form_id,
        answers,
    };

    Ok(Json(paper_answers))
}

#[get("/<_id>", rank = 5)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_paper>", rank = 1)]
pub async fn create_paper_admin(
    db: Db,
    _admin: AdminClaims,
    new_paper: Json<NewPaper>,
) -> Result<Json<Paper>, Status> {
    let new_paper = new_paper.into_inner();

    let paper = paper_repository::create(&db, new_paper).await;

    match paper {
        Ok(paper) => Ok(Json(paper)),
        Err(e) => {
            println!("Error: Paper, Method: POST, Action: create_paper_admin");
            println!("Error: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/", data = "<_new_paper>", rank = 5)]
pub async fn create_paper_none(_new_paper: Json<NewPaper>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<paper>", rank = 1)]
pub async fn update_paper_admin(
    db: Db,
    _admin: AdminClaims,
    id: i32,
    paper: Json<PaperWithNewAnswers>,
) -> Result<Json<PaperWithAnswers>, Status> {
    let paper = paper.into_inner();

    let new_answers = paper.answers;
    let new_paper = NewPaper {
        user_id: paper.user_id,
        form_id: paper.form_id,
    };

    // Update paper
    let paper = paper_repository::update(&db, id, new_paper).await;
    if let Err(e) = paper {
        println!("Error: Paper, Method: PUT, Action: update_paper_admin");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let paper = paper.unwrap();

    // Update answers
    let answers = paper_answers_repository::update_answers_by_paper_id(&db, id, new_answers).await;
    if let Err(e) = answers {
        println!("Error: Paper, Method: PUT, Action: update_paper_admin");
        println!("Error: {:?}", e);
        return Err(Status::InternalServerError);
    }
    let answers = answers.unwrap();

    // Return paper with answers
    let response = PaperWithAnswers {
        id: paper.id,
        user_id: paper.user_id,
        form_id: paper.form_id,
        answers,
    };

    Ok(Json(response))
}

#[put("/<_id>", data = "<_paper>", rank = 5)]
pub async fn update_paper_none(_id: i32, _paper: Json<PaperWithNewAnswers>) -> Status {
    Status::Unauthorized
}
