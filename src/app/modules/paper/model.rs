use serde::{Deserialize, Serialize};

use crate::app::modules::answer::model::Answer;

use crate::app::providers::interfaces::form::Form;
use crate::app::providers::interfaces::user::User;

use crate::database::schema::papers;

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Paper {
    pub id: i32,
    pub user_id: i32,
    pub form_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "papers"]
pub struct NewPaper {
    pub user_id: i32,
    pub form_id: i32,
}

impl From<Paper> for NewPaper {
    fn from(paper: Paper) -> Self {
        NewPaper {
            user_id: paper.user_id,
            form_id: paper.form_id,
        }
    }
}

impl From<PaperComplete> for NewPaper {
    fn from(paper: PaperComplete) -> Self {
        NewPaper {
            user_id: paper.user.id,
            form_id: paper.form.id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperWithAnswers {
    pub id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub answers: Vec<Answer>,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct NewPaperWithAnswers {
//     pub form_id: i32,
//     pub user_id: i32,
//     pub answers: Vec<Answer>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaperComplete {
    pub id: i32,
    pub user: User,
    pub form: Form,
    pub answers: Vec<Answer>,
}
