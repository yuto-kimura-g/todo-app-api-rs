#[cfg(test)]
mod tests {
    use todo_app_api::db_cli;

    #[test]
    fn test_cli_create_task() {
        let db_conn = &mut db_cli::establish_connection();
        let title = String::from("task title");
        let description = Some(String::from("task description"));
        let _now = chrono::Local::now();
        let _naive_date = _now.date_naive();
        let _time = chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let due_date = Some(chrono::NaiveDateTime::new(_naive_date, _time));
        let _ = db_cli::create_task(db_conn, title, description, due_date);
    }

    #[test]
    fn test_cli_get_tasks() {
        let db_conn = &mut db_cli::establish_connection();
        let res = db_cli::get_tasks(db_conn).expect("Error: get_tasks()");
        for task in res {
            println!("{:?}", task);
        }
    }

    #[test]
    fn test_cli_update_task() {
        let db_conn = &mut db_cli::establish_connection();
        let task_id = 2;
        let new_title = String::from("updated task title");
        let new_description = None;
        let new_due_date = Some(
            chrono::NaiveDateTime::parse_from_str("1945-08-15 09:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        );
        let new_is_done = true;
        // let _ = update_task_title(db_conn, task_id, new_title);
        // let _ = update_task_description(db_conn, task_id, new_description);
        // let _ = update_task_due_date(db_conn, task_id, new_due_date);
        // let _ = update_task_is_done(db_conn, task_id, new_is_done);
        let _ = db_cli::update_task(
            db_conn,
            task_id,
            new_title,
            new_description,
            new_due_date,
            new_is_done,
        );
    }

    #[test]
    fn test_cli_delete_task() {
        let db_conn = &mut db_cli::establish_connection();
        let task_id = 1;
        let _ = db_cli::delete_task(db_conn, task_id);
    }
}
