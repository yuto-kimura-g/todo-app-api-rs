use crate::models::{NewTask, Task};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Error: loading DATABASE_URL");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error: connecting to {}", database_url))
}

pub fn create_task(
    conn: &mut MysqlConnection,
    new_title: String,
    new_description: Option<String>,
    new_due_date: Option<NaiveDateTime>,
) -> QueryResult<usize> {
    use crate::schema::tasks;
    let new_task = NewTask {
        title: new_title,
        description: new_description,
        due_date: new_due_date,
        is_done: false,
    };
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
}

pub fn get_tasks(conn: &mut MysqlConnection) -> QueryResult<Vec<Task>> {
    use crate::schema::tasks::dsl::*;
    tasks.select(Task::as_select()).load(conn)
}

pub fn update_task(
    conn: &mut MysqlConnection,
    task_id: i32,
    // new_task: NewTask,
    new_title: String,
    new_description: Option<String>,
    new_due_date: Option<NaiveDateTime>,
    new_is_done: bool,
) -> QueryResult<usize> {
    use crate::schema::tasks::dsl::*;
    let new_task = NewTask {
        title: new_title,
        description: new_description,
        due_date: new_due_date,
        is_done: new_is_done,
    };
    diesel::update(tasks.filter(id.eq(task_id)))
        .set(new_task)
        .execute(conn)
}

pub fn delete_task(conn: &mut MysqlConnection, task_id: i32) -> QueryResult<usize> {
    use crate::schema::tasks::dsl::*;
    diesel::delete(tasks.filter(id.eq(task_id))).execute(conn)
}
