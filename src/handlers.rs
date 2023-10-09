use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Deserialize, Serialize, FromRow)]
pub struct Artist {
    id: Option<i32>,
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Painting {
    id: Option<i32>,
    title: String,
    artist_id: i32,
    gallery_id: i32,
}

#[derive(Serialize)]
pub struct ArtistWithPaintings {
    name: String,
    paintings: Vec<Painting>,
}

pub async fn create_artist(
    State(pool): State<MySqlPool>,
    Json(new_artist): Json<Artist>,
) -> Result<(StatusCode, Json<Artist>), StatusCode> {
    let res = sqlx::query("INSERT INTO artists (name, age) VALUES (?, ?)")
        .bind(&new_artist.name)
        .bind(&new_artist.age)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(new_artist))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_painting(
    State(pool): State<MySqlPool>,
    Json(new_painting): Json<Painting>,
) -> Result<(StatusCode, Json<Painting>), StatusCode> {
    let res = sqlx::query("INSERT INTO paintings (title, artist_id, gallery_id) values (?, ?, ?)")
        .bind(&new_painting.title)
        .bind(&new_painting.artist_id)
        .bind(&new_painting.gallery_id)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(new_painting))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_artist_with_paintings(
    State(pool): State<MySqlPool>,
    Path(id): Path<i32>,
) -> Result<Json<ArtistWithPaintings>, (StatusCode, String)> {
    let artist_query = sqlx::query_as::<_, Artist>("SELECT * FROM artists where id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await;

    let artist = match artist_query {
        Ok(a) => a,
        Err(e) => return Err((StatusCode::NOT_FOUND, e.to_string())),
    };

    let paintings_query =
        sqlx::query_as::<_, Painting>("SELECT * FROM paintings WHERE artist_id = ?")
            .bind(artist.id)
            .fetch_all(&pool)
            .await;

    let paintings = match paintings_query {
        Ok(p) => p,
        Err(e) => return Err((StatusCode::NOT_FOUND, e.to_string())),
    };

    let res = ArtistWithPaintings {
        name: artist.name,
        paintings: paintings,
    };

    Ok(Json(res))
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
) -> Result<Json<Vec<Painting>>, (StatusCode, String)> {
    let res = sqlx::query_as!(
        Painting,
        r#"
        SELECT * 
        FROM paintings 
        "#,
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(paintings) => Ok(Json(paintings)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
