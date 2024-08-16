use todo_app_api::db_cli;

fn main() {
    let conn = &mut db_cli::establish_connection();
    let res = db_cli::get_tasks(conn).expect("Error: get_tasks()");
    for task in res {
        println!("{:?}", task);
    }
}
