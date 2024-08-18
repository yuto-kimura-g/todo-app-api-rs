use crate::schema;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::tasks)]
// rename_all指定すると，JsonとSerialize/Deserializeする時にキーを変換してくれる．便利．
// rustは snake_case だけど， typescriptは camelCase
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i32, // TODO: use ULID: Universally Unique Lexicographically Sortable Identifier
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub is_done: bool,
    // pub created_at: String,
    // pub updated_at: String,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = schema::tasks)]
#[serde(rename_all = "camelCase")]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub is_done: bool,
}
