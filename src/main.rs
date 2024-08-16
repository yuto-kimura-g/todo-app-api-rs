use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use todo_app_api::{db, handler};

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
        .expect("Error: parsing API_PORT");
    let database_url: &str = &std::env::var("DATABASE_URL").expect("Error: missing DATABASE_URL");
    let db_pool = db::establish_connection_pool(database_url);
    println!("\nLaunching server at http://{addr}:{port}\n");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(handler::index)
            .service(handler::create_task)
            .service(handler::get_tasks)
            .service(handler::update_task)
            .service(handler::delete_task)
    })
    .bind((addr, port))?
    .run()
    .await
}
