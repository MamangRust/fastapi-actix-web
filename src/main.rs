use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

#[actix_web::post("/items")]
async fn create_item(data: web::Json<Item>, state: web::Data<AppState>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    items.push(data.into_inner());
    HttpResponse::Created().json("Item created")
}

#[actix_web::get("/items")]
async fn get_items(state: web::Data<AppState>) -> impl Responder {
    let items = state.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

#[actix_web::get("/items/{id}")]
async fn get_item(state: web::Data<AppState>, item_id: web::Path<i32>) -> impl Responder {
    let items = state.items.lock().unwrap();
    match items.iter().find(|&item| item.id == *item_id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json("Item not found"),
    }
}

#[actix_web::put("/items/{id}")]
async fn update_item(
    state: web::Data<AppState>,
    item_id: web::Path<i32>,
    item: web::Json<Item>,
) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(existing_item) = items.iter_mut().find(|item| item.id == *item_id) {
        existing_item.name = item.name.clone();
        HttpResponse::Ok().json("Item updated")
    } else {
        HttpResponse::NotFound().json("Item not found")
    }
}

#[actix_web::delete("/items/{id}")]
async fn delete_item(state: web::Data<AppState>, item_id: web::Path<i32>) -> impl Responder {
    let mut items = state.items.lock().unwrap();
    if let Some(pos) = items.iter().position(|item| item.id == *item_id) {
        items.remove(pos);
        HttpResponse::Ok().json("Item deleted")
    } else {
        HttpResponse::NotFound().json("Item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(create_item)
            .service(get_items)
            .service(get_item)
            .service(update_item)
            .service(delete_item)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
