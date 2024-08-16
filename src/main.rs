use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use todo_app_api::{db, handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let api_addr: &str = &std::env::var("API_ADDRESS").unwrap_or("localhost".to_string());
    let api_port: u16 = std::env::var("API_PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Error: parsing API_PORT");
    let database_url: String = std::env::var("DATABASE_URL").expect("Error: missing DATABASE_URL");
    let db_pool = db::establish_connection_pool(&database_url);
    let client_url: String =
        std::env::var("CLIENT_URL").unwrap_or("http://localhost:3000".to_string());
    println!("\nLaunching server at http://{api_addr}:{api_port}\n");
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allow_any_origin()
            .allowed_origin(&client_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(handler::index)
            .service(handler::create_task)
            .service(handler::get_tasks)
            .service(handler::update_task)
            .service(handler::delete_task)
    })
    .bind((api_addr, api_port))?
    .run()
    .await
}
