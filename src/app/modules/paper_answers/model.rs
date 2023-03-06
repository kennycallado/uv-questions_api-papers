use serde::{Deserialize, Serialize};

use crate::database::schema::paper_answers;

use crate::app::modules::answer::model::Answer;
use crate::app::modules::paper::model::Paper;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Paper)]
#[belongs_to(Answer)]
#[serde(crate = "rocket::serde")]
pub struct PaperAnswer {
    pub id: i32,
    pub paper_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "paper_answers"]
pub struct NewPaperAnswer {
    pub paper_id: i32,
    pub answer_id: i32,
}

impl From<PaperAnswer> for NewPaperAnswer {
    fn from(paper_answer: PaperAnswer) -> Self {
        NewPaperAnswer {
            paper_id: paper_answer.paper_id,
            answer_id: paper_answer.answer_id,
        }
    }
}
