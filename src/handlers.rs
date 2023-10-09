use axum::{extract::State, http::StatusCode, response::Json};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

#[derive(Serialize, FromRow)]
pub struct Artist {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Serialize, FromRow)]
pub struct Painting {
    id: i32,
    title: String,
    gallery_id: i32,
    artist_id: i32,
}

pub async fn get_artists(State(pool): State<MySqlPool>) -> Result<Json<Vec<Artist>>, StatusCode> {
    let res = sqlx::query_as!(Artist, "SELECT * FROM artists ORDER BY id")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(artists) => Ok(Json(artists)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_paintings(
    State(pool): State<MySqlPool>,
) -> Result<Json<Vec<Painting>>, StatusCode> {
    let res = sqlx::query_as!(Painting, "SELECT * FROM paintings")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(paintings) => Ok(Json(paintings)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
