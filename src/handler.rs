use crate::{
    db::{self, DbPool},
    models::NewTask,
};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse, Responder,
};

#[actix_web::get("/")]
async fn index() -> impl Responder {
    println!("[index]");
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .insert_header(ContentType::html())
        .body("<h1>Hello, Rust API World!</h1>")
}

/// 引数の順番は関係ない
/// web::X<T> を参照することだけ分かればよい
#[actix_web::post("/tasks")]
async fn create_task(db_pool: web::Data<DbPool>, new_task: web::Json<NewTask>) -> impl Responder {
    // web::Data<DbPool> -> &DbPool
    let db_pool: &DbPool = db_pool.get_ref();
    // into_inner(): Deserialize Json -> NewTask
    let new_task = new_task.into_inner();
    println!("[create] new_task: {:?}", new_task);
    match db::create_task(db_pool, new_task).await {
        // .json(): Serialize Task -> Json
        Ok(created_task) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .json(created_task),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to create task: {:?}", err))
        }
    }
}

#[actix_web::get("/tasks")]
async fn get_tasks(db_pool: web::Data<DbPool>) -> impl Responder {
    let db_pool = db_pool.get_ref();
    println!("[get]");
    match db::get_tasks(db_pool).await {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to get tasks: {:?}", err))
        }
    }
}

#[actix_web::put("/tasks/{id}")]
async fn update_task(
    db_pool: web::Data<DbPool>,
    task_id: web::Path<i32>,
    new_task: web::Json<NewTask>,
) -> impl Responder {
    let db_pool = db_pool.get_ref();
    // into_inner(): Path<i32> -> i32
    let task_id = task_id.into_inner();
    // into_inner(): Deserialize Json -> NewTask
    let new_task = new_task.into_inner();
    println!("[update] task_id: {task_id}, new_task: {:?}", new_task);
    match db::update_task(db_pool, task_id, new_task).await {
        Ok(updated_task) => HttpResponse::Ok().json(updated_task),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update task: {:?}", err))
        }
    }
}

#[actix_web::delete("/tasks/{id}")]
async fn delete_task(db_pool: web::Data<DbPool>, task_id: web::Path<i32>) -> impl Responder {
    let db_pool = db_pool.get_ref();
    let task_id = task_id.into_inner();
    println!("[delete] task_id: {task_id}");
    match db::delete_task(db_pool, task_id).await {
        Ok(_) => {
            // 空のレスポンスを返す時は，finish()するらしい
            HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to delete task: {:?}", err))
        }
    }
}
