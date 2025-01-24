use actix_web::{web,App,HttpServer, Responder,HttpResponse};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::sync::{Mutex, MutexGuard};
use chrono::{Utc,DateTime};

#[derive(Serialize,Deserialize,Clone)]
struct TodoItem{
    id:Uuid,
    title:String,
    completed: bool,
    created_at: DateTime<Utc>
}

#[derive(Deserialize)]

struct CreateTodoItem{
    title: String,
    completed: bool,
}

#[derive(Deserialize)]

struct UpdateTodoItem{
    title: Option<String>,
    completed: Option<bool>
}

struct AppState{
    todo_list:Mutex<Vec<TodoItem>>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder{
    let todos:MutexGuard<'_, Vec<TodoItem>>  = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

async fn add_todos(
    item: web::Json<CreateTodoItem>,
    data: web::Data<AppState>
)-> impl Responder{
    let mut todos:MutexGuard<'_, Vec<TodoItem>>  = data.todo_list.lock().unwrap();
    let new_todo = TodoItem{
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now()
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

async fn update_todo(
    path: web::Path<Uuid>,
    item: web::Json<UpdateTodoItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *path) {
        if let Some(title) = &item.title {
            todo.title = title.clone();
        }

        if let Some(completed) = &item.completed {
            todo.completed = *completed;
        }

        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}
async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = match data.todo_list.lock() {
        Ok(todos) => todos,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to acquire lock"),
    };

    if let Some(index) = todos.iter().position(|todo| todo.id == *path) {
        let deleted_todo = todos.remove(index);
        HttpResponse::Ok().json(deleted_todo)
    } else {
        HttpResponse::NotFound().body(format!("Todo with ID {} not found", path))
    }
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    let app_state=web::Data::new(AppState{
        todo_list: Mutex::new(Vec::new())
    });

    HttpServer::new(move || {
        let cors= Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);


    App::new().app_data(app_state.clone()).wrap(cors).route("/todos",web::get().to(get_todos)).route("/todos",web::post().to(add_todos)).route("/todos/{id}",web::put().to(update_todo))
    .route("/todos/{id}",web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080") ? .run().await
}