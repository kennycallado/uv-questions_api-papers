use std::fmt;

use diesel::{
    expression::AsExpression, helper_types::AsExprOf, pg::Pg, row::Row, sql_types::Text,
    types::FromSqlRow,
};
use serde::{Deserialize, Serialize};

use crate::database::schema::answers;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TypeEnum {
    Int,
    String,
}

impl fmt::Display for TypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TypeEnum::Int => "int",
                TypeEnum::String => "string",
            }
        )
    }
}

impl FromSqlRow<Text, Pg> for TypeEnum {
    fn build_from_row<R: Row<Pg>>(
        row: &mut R,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match String::build_from_row(row)?.as_ref() {
            "int" => Ok(TypeEnum::Int),
            "string" => Ok(TypeEnum::String),
            v => Err(format!("Unknown value {} for TypeEnum found", v).into()),
        }
    }
}

impl AsExpression<Text> for TypeEnum {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

impl<'a> AsExpression<Text> for &'a TypeEnum {
    type Expression = AsExprOf<String, Text>;
    fn as_expression(self) -> Self::Expression {
        <String as AsExpression<Text>>::as_expression(self.to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub id: i32,
    pub a_type: TypeEnum,
    pub answer: String,
}

impl From<(i32, String, String)> for Answer {
    fn from(answer: (i32, String, String)) -> Self {
        Answer {
            id: answer.0,
            a_type: match answer.1.as_str() {
                "int" => TypeEnum::Int,
                "string" => TypeEnum::String,
                _ => TypeEnum::String,
            },
            answer: answer.2,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "answers"]
pub struct NewAnswer {
    pub a_type: TypeEnum,
    pub answer: String,
}

impl From<Answer> for NewAnswer {
    fn from(answer: Answer) -> Self {
        NewAnswer {
            answer: answer.answer,
            a_type: answer.a_type,
        }
    }
}
