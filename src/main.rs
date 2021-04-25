mod data;
mod handlers;
mod session;
mod state;
mod utils;
use crate::handlers::*;
use crate::session::DbStore;
use crate::state::State;

use sqlx::MySqlPool;
use std::sync::Arc;
use tera::Tera;
use tide::sessions::SessionMiddleware;

#[async_std::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    env_logger::init();

    let tera = Tera::new("templates/*").expect("Failed to compile templates.");
    let tera = Arc::new(tera);

    let uri = std::env::var("DATABASE_URL").expect("Failed to find DATABASE_URL in env.");
    let pool = MySqlPool::connect(&uri).await.expect("Failed to connect to database.");
    let pool = Arc::new(pool);

    let state = State::new(pool.clone(), tera);
    let mut app = tide::with_state(state);

    app.at("/").get(index_get);
    app.at("/create").post(create_post);
    app.at("/login").get(login_get).post(login_post);
    app.at("/log_out").get(logout_get);
    app.at("/register").get(register_get).post(register_post);
    app.at("*").get(shortcut_get);

    app.at("/static")
        .serve_dir("static/")
        .expect("Failed to serve static dir.");

    let secret = std::env::var("TIDE_SECRET").expect("Failed to find TIDE_SECRET in env.");
    app.with(SessionMiddleware::new(DbStore::new(pool), secret.as_bytes()));

    app.listen("127.0.0.1:8000").await.expect("Failed to run the app.")
}
