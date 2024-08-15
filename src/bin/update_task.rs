use todo_app_api::db::{
    establish_connection, update_task_description, update_task_due_date, update_task_title,
};

fn main() {
    let conn = &mut establish_connection();
    let task_id = 2;
    let new_title = String::from("updated task title");
    let new_description = None;
    let new_due_date = Some(
        chrono::NaiveDateTime::parse_from_str("1945-08-15 09:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    let _ = update_task_title(conn, task_id, new_title);
    let _ = update_task_description(conn, task_id, new_description);
    let _ = update_task_due_date(conn, task_id, new_due_date);
}
