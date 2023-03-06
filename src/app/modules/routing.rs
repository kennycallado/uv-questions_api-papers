use crate::app::modules::answer::controller as answer_controller;
use crate::app::modules::paper::controller as paper_controller;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/answer", answer_controller::routes())
            .mount("/api/v1/paper", paper_controller::routes())
    })
}
