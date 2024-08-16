use todo_app_api::db_cli;

fn main() {
    let conn = &mut db_cli::establish_connection();
    let task_id = 1;
    let _ = db_cli::delete_task(conn, task_id);
}
