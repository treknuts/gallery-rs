use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Artist {
    id: Option<i32>,
    name: String,
    age: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Painting {
    id: Option<i32>,
    title: String,
    gallery_id: i32,
    artist_id: i32,
}

pub async fn create_artists(
    State(pool): State<MySqlPool>,
    Json(new_artist): Json<Artist>,
) -> Result<(StatusCode, Json<Artist>), (StatusCode, String)> {
    let res = sqlx::query("INSERT INTO artists (name, age) VALUES (?, ?)")
        .bind(&new_artist.name)
        .bind(&new_artist.age)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(new_artist))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
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
