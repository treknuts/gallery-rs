mod handlers;
use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub pool: MySqlPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let pool = MySqlPool::connect(&db_url).await?;

    let server_config = ServerConfig {
        host: String::from("127.0.0.1"),
        port: 3000,
        pool: pool,
    };

    let app = Router::new()
        .route("/artists", get(handlers::get_artists))
        .route("/paintings", get(handlers::get_paintings))
        .with_state(server_config.pool);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
