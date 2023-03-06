use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::answers;
use crate::database::schema::paper_answers;

use crate::app::modules::answer::model::Answer;

pub async fn get_answers_by_paper_id(db: &Db, paper_id: i32) -> Result<Vec<Answer>, diesel::result::Error> {
    let answers = db
        .run(move |conn| {
            paper_answers::table
                .inner_join(answers::table)
                .filter(paper_answers::paper_id.eq(paper_id.clone()))
                .select(answers::all_columns)
                .load::<Answer>(conn)
        })
        .await?;

    Ok(answers)
}
