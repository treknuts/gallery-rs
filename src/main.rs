mod handlers;
use axum::{routing::get, routing::post, Router};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let pool = MySqlPool::connect(&db_url).await?;

    let app = Router::new()
        .route("/artists", get(handlers::get_artists))
        .route("/artists/{id}", get(handlers::get_artist_with_paintings))
        .route("/artists", post(handlers::create_artist))
        .route("/paintings", get(handlers::get_paintings))
        .route("/paintings/{id}", get(handlers::get_painting_with_artist))
        .route("/paintings", post(handlers::create_painting))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
