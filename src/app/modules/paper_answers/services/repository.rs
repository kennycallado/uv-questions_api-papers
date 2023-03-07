use diesel::prelude::*;

use crate::app::modules::answer::model::NewAnswer;
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

pub async fn update_answers_by_paper_id(db: &Db, paper_id: i32, new_answers: Vec<NewAnswer>)
-> Result<Vec<Answer>, diesel::result::Error>
{
    let answers: Vec<Answer> = db
        .run(move |conn| {
            let old_answers = paper_answers::table
                .inner_join(answers::table)
                .filter(paper_answers::paper_id.eq(paper_id.clone()))
                .select(answers::all_columns)
                .load::<Answer>(conn).unwrap();

            for old_answer in old_answers {
                diesel::delete(answers::table.filter(answers::id.eq(old_answer.id))).execute(conn).unwrap();
            }

            let mut answers = Vec::new();
            for new_answer in new_answers {
                let answer = diesel::insert_into(answers::table)
                    .values(&new_answer)
                    .get_result::<Answer>(conn).unwrap();

                diesel::insert_into(paper_answers::table)
                    .values((
                        paper_answers::paper_id.eq(paper_id.clone()),
                        paper_answers::answer_id.eq(answer.id)
                    )).execute(conn).unwrap();

                answers.push(answer);
            }

            answers
        }).await;

    Ok(answers)
}
