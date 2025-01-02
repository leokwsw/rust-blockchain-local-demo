use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

mod blockchain;
use blockchain::Blockchain;

struct AppState {
    blockchain: Mutex<Blockchain>,
}

#[derive(Deserialize)]
struct NewBlockData {
    data: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(Blockchain::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(get_blocks))
            .route("/add", web::post().to(add_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_blocks(state: web::Data<AppState>) -> impl Responder {
    let blockchain = state.blockchain.lock().unwrap();
    web::Json(blockchain.blocks.clone())
}

async fn add_block(
    state: web::Data<AppState>,
    block_data: web::Json<NewBlockData>,
) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    blockchain.add_block(block_data.data.clone());
    "Block added successfully"
}
