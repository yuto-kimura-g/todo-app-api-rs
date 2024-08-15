use actix_web::{App, HttpServer};
use dotenv::dotenv;
use todo_app_api::handler::{create_task, delete_task, get_tasks, index, update_task};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let addr: &str = &std::env::var("API_ADDRESS").unwrap_or_else(|err| {
        eprintln!("Error: loading API_ADDRESS: {}", err);
        "127.0.0.1".to_string()
    });
    let port: u16 = std::env::var("API_PORT")
        .unwrap_or_else(|err| {
            eprintln!("Error: loading API_PORT: {}", err);
            "8080".to_string()
        })
        .parse()
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_task)
            .service(get_tasks)
            .service(update_task)
            .service(delete_task)
    })
    .bind((addr, port))?
    .run()
    .await
}
