use crate::models::{NewTask, Task};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{result::Error as DieselError, MysqlConnection};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let conn_manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let n_pool: u32 = 16; // デフォルトは無制限
    r2d2::Pool::builder()
        .max_size(n_pool)
        .build(conn_manager)
        .expect("Error: failed to create connection pool")
}

pub async fn create_task(db_pool: &DbPool, new_task: NewTask) -> Result<Task, DieselError> {
    use crate::schema::tasks;
    // poolしているconnから一つ借りる
    let mut db_conn = db_pool
        .get()
        .expect("Error: failed to get db connection from pool");
    // PostgreSQLなら，returning句をサポートしているため get_result() を使えるが，
    // MySQLはそんなもの無いので，execしてからfirst()する．
    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(&mut db_conn)?;
    // .get_result(&mut db_conn)
    tasks::table.order(tasks::id.desc()).first(&mut db_conn)
}

pub async fn get_tasks(db_pool: &DbPool) -> Result<Vec<Task>, DieselError> {
    use crate::schema::tasks::dsl::*;
    let mut db_conn = db_pool
        .get()
        .expect("Error: failed to get db connection from pool");
    // tasks.select(tasks::all_columns()).load::<Task>(&mut db_conn)
    tasks.load::<Task>(&mut db_conn)
}

pub async fn update_task(
    db_pool: &DbPool,
    task_id: i32,
    new_task: NewTask,
) -> Result<Task, DieselError> {
    use crate::schema::tasks::dsl::*;
    let mut db_conn = db_pool
        .get()
        .expect("Error: failed to get db connection from pool");
    // createと同様，MySQLにはreturning句が無いので，execしてからfind().first()する．
    // NOTE: （注意）
    //      .set(new_task)とすると，nullへの更新が出来ない
    //      例えば，description=nullとして，debug_query()で確認してみると
    //      Query { sql: "
    //           UPDATE `tasks` SET `title` = ?, `due_date` = ?, `is_done` = ?
    //           WHERE (`tasks`.`id` = ?)
    //      ", binds: ["hoge", "piyo", "foo"] }
    //      というqueryが発行されている．そのため，.set(())で全てのカラムを明示的に指定する必要がある．
    // let new_task_clone = new_task.clone();
    // let query = diesel::update(tasks.filter(id.eq(task_id))).set((
    //     title.eq(new_task_clone.title),
    //     description.eq(new_task_clone.description),
    //     due_date.eq(new_task_clone.due_date),
    //     is_done.eq(new_task_clone.is_done),
    // ));
    // println!("[update query] {:?}", diesel::debug_query(&query));
    diesel::update(tasks.filter(id.eq(task_id)))
        // .set(new_task) // <- nullへの更新が出来ない
        .set((
            title.eq(new_task.title),
            description.eq(new_task.description),
            due_date.eq(new_task.due_date),
            is_done.eq(new_task.is_done),
        ))
        .execute(&mut db_conn)?;
    tasks.find(task_id).first(&mut db_conn)
}

pub async fn delete_task(db_pool: &DbPool, task_id: i32) -> Result<usize, DieselError> {
    use crate::schema::tasks::dsl::*;
    let mut db_conn = db_pool
        .get()
        .expect("Error: failed to get db connection from pool");
    diesel::delete(tasks.filter(id.eq(task_id))).execute(&mut db_conn)
}
