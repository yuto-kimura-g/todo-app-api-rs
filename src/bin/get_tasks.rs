use todo_app_api::db::{establish_connection, get_tasks};

fn main() {
    let conn = &mut establish_connection();
    let res = get_tasks(conn).expect("Error: get_tasks()");
    for task in res {
        println!("{:?}", task);
    }
}
