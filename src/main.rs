use actix_web::{
    delete, get,
    http::{header::ContentType, StatusCode},
    post, put, web, App, HttpResponse, HttpServer, Responder, Result,
};

mod models;
use models::Task;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .insert_header(ContentType::plaintext())
        .body("Hello, Rust API World!")
}

#[post("/tasks")]
async fn create_task(task: web::Json<Task>) -> impl Responder {
    println!("create_task: {:?}", task);
    HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .body("Task created")
}

#[get("/tasks")]
async fn get_tasks() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("get_tasks(): Not implemented")
}

#[get("/tasks/{id}")]
async fn get_task(id: web::Path<String>) -> impl Responder {
    let task = Task {
        id: id.to_string(),
        title: "Task 1".to_string(),
        description: Some("Task 1 description".to_string()),
        due_date: Some("2021-12-31".to_string()),
        is_done: false,
    };
    // Ok(web::Json(task))
    HttpResponse::Ok().status(StatusCode::OK).json(task)
}

#[put("/tasks/{id}")]
async fn update_task() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("update_task(): Not implemented")
}

#[delete("/tasks/{id}")]
async fn delete_task() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("delete_task(): Not implemented")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1";
    let port = 8080;
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_task)
            .service(get_tasks)
            .service(get_task)
            .service(update_task)
            .service(delete_task)
    })
    .bind((addr, port))?
    .run()
    .await
}
