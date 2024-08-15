use chrono::NaiveDateTime;
use diesel::{
    prelude::{AsChangeset, Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use super::schema::tasks;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32, // TODO: use ULID: Universally Unique Lexicographically Sortable Identifier
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    // pub due_date: Option<DateTime<Utc>>,
    // pub due_date: Option<sql_types::Nullable<sql_types::Datetime>>,
    pub is_done: bool,
    // pub created_at: String,
    // pub updated_at: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    // pub due_date: Option<sql_types::Nullable<sql_types::Datetime>>,
    pub is_done: bool,
}

// impl NewTask {
//     pub fn new(
//         title: String,
//         description: Option<String>,
//         due_date: Option<DateTime<Utc>>,
//         is_done: bool,
//     ) -> Self {
//         NewTask {
//             title,
//             description,
//             due_date: due_date.map(|dt| dt.naive_utc()),
//             is_done,
//         }
//     }
// }
