use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::answers;

use crate::app::modules::answer::model::{Answer, NewAnswer};

pub async fn get_all(db: &Db) -> Result<Vec<Answer>, diesel::result::Error> {
    let result = db
        .run(move |conn| answers::table.load::<(i32, String, String)>(conn))
        .await?;

    let answers: Vec<Answer> = result.into_iter().map(|answer| answer.into()).collect();

    Ok(answers)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Answer, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            answers::table
                .filter(answers::id.eq(id))
                .first::<(i32, String, String)>(conn)
        })
        .await?;

    Ok(result.into())
}

pub async fn add(db: &Db, answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            diesel::insert_into(answers::table)
                .values(&answer)
                .get_result::<(i32, String, String)>(conn)
        })
        .await?;

    Ok(result.into())
}

pub async fn update(db: &Db, id: i32, answer: NewAnswer) -> Result<Answer, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            diesel::update(answers::table)
                .filter(answers::id.eq(id))
                .set(&answer)
                .get_result::<(i32, String, String)>(conn)
        })
        .await?;

    Ok(result.into())
}
