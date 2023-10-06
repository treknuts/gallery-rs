use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPool, FromRow};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub pool: MySqlPool,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
struct Gallery {
    id: i32,
    title: String,
    city: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
struct Artist {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
struct Painting {
    id: i32,
    title: String,
    gallery_id: i32,
    artist_id: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = ServerConfig {
        host: String::from("127.0.0.1"),
        port: 3000,
        pool: pool,
    };

    let app = Router::new()
        .route("/artists", get(get_artists))
        .route("/paintings", get(get_paintings))
        .with_state(server_config);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn create_database_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").to_string()).await?;

    Ok(pool)
}

async fn get_artists(
    State(server_config): State<ServerConfig>,
) -> Result<Json<Vec<Artist>>, (StatusCode, String)> {
    let artists = sqlx::query_as!(Artist, "SELECT * FROM artists ORDER BY id")
        .fetch_all(&server_config.pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Houston...we have a problem"),
            )
        })?;

    Ok(Json(artists))
}

async fn get_paintings(
    State(server_config): State<ServerConfig>,
) -> Result<Json<Vec<Painting>>, (StatusCode, String)> {
    let paintings = sqlx::query_as!(Painting, "SELECT * FROM paintings")
        .fetch_all(&server_config.pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, format!("Oopsie Daisy")))?;

    Ok(Json(paintings))
}
