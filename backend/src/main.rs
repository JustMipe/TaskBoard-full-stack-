use tokio::net::TcpListener;
use axum::{
    routing::post,
    Router,
    extract::State,
    Json,
    serve,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::net::SocketAddr;
use dotenvy::dotenv;
use std::env;
use argon2::{self, Config};

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    message: String,
}

async fn register_user(
    State(db): State<SqlitePool>,
    Json(payload): Json<RegisterPayload>,
) -> Json<RegisterResponse> {
    // Hashování hesla
    let salt = b"randomsalt"; // může být později náhodný
    let config = Config::default();
    let password_hash = argon2::hash_encoded(payload.password.as_bytes(), salt, &config)
        .expect("Failed to hash password");

    // Uložení do DB
    let _ = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        payload.username,
        password_hash
    )
    .execute(&db)
    .await
    .expect("Failed to insert user");

    Json(RegisterResponse {
        message: "New User is registered successfully!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let db_pool = SqlitePool::connect(&db_url).await.expect("DB connection failed");

    let app = Router::new()
        .route("/register", post(register_user))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server is running at http://{}", addr);
    serve(listener, app).await.unwrap();
}
