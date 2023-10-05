use axum::{extract::State, routing::get, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub pool: MySqlPool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gallery {
    id: i32,
    title: String,
    city: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Artist {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Painting {
    id: i32,
    title: String,
    gallery_id: i32,
    artist_id: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL")?).await?;

    let server_config = ServerConfig {
        host: String::from("127.0.0.1"),
        port: 3000,
        pool: pool,
    };

    let app = Router::new()
        .with_state(server_config)
        .route("/", get(index))
        .route("/artists", get(get_artists))
        .route("/paintings", get(paintings));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn index() -> &'static str {
    "Hello, Axum!"
}

async fn artists() -> &'static str {
    "Hello, artists"
}

async fn paintings() -> &'static str {
    "Hello, paintings"
}

async fn get_artists(State(server_config): State<ServerConfig>) -> anyhow::Result<()> {
    let artists: Vec<Artist> = sqlx::query!(
        r#"
    SELECT *
    FROM artists
    ORDER BY id
    "#
    )
    .fetch_all(&server_config.pool)
    .await?
    .iter()
    .map(|artist| Artist {
        id: artist.id.clone(),
        name: artist.name.clone().unwrap().clone(),
        age: artist.age.unwrap().clone(),
    })
    .collect();

    println!("{}", artists.len());

    Ok(())
}

async fn get_paintings(State(server_config): State<ServerConfig>) -> anyhow::Result<()> {
    let results = sqlx::query!(
        r#"
    SELECT p.title, a.name, g.title as gallery_name
    FROM paintings as p, artists as a, galleries as g
    WHERE p.artist_id = a.id AND p.gallery_id = g.id
    ORDER BY a.name
    "#
    )
    .fetch_all(&server_config.pool)
    .await?;

    for row in results {
        println!(
            r#"
        {}
            by {}
            displayed in {}
        "#,
            &row.title.unwrap(),
            &row.name.unwrap(),
            &row.gallery_name.unwrap()
        );
    }

    Ok(())
}
