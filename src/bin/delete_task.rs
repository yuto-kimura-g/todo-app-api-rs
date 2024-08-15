use todo_app_api::db::{delete_task, establish_connection};

fn main() {
    let conn = &mut establish_connection();
    let task_id = 1;
    let _ = delete_task(conn, task_id);
}
