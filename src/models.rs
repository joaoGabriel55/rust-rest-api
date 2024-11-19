use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub content: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::todos)]
pub struct UpdateTodo {
    pub title: String,
    pub content: String,
}
