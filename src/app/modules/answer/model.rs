use serde::{Deserialize, Serialize};

use crate::database::schema::answers;

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub id: i32,
    pub answer: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "answers"]
pub struct NewAnswer {
    pub answer: String,
}

impl From<Answer> for NewAnswer {
    fn from(answer: Answer) -> Self {
        NewAnswer {
            answer: answer.answer,
        }
    }
}
