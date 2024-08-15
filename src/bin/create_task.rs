use todo_app_api::db::{create_task, establish_connection};

fn main() {
    let conn = &mut establish_connection();
    let title = String::from("task title");
    let description = Some(String::from("task description"));
    let _now = chrono::Local::now();
    let _naive_date = _now.date_naive();
    let _time = chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap();
    let due_date = Some(chrono::NaiveDateTime::new(_naive_date, _time));
    let _ = create_task(conn, title, description, due_date);
}
