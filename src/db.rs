use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};

use crate::model::{CategoriesResponse, DirectionsResponse};

pub async fn get_enters(direction: u8, db: &Pool<Sqlite>) -> Result<Vec<String>, sqlx::Error> {
    let result: Vec<String> = sqlx::query(
        "SELECT s.name FROM enters e, stations s WHERE e.name_id == s.id AND e.direction_id = ?",
    )
    .bind(direction)
    .map(|row: SqliteRow| row.get("name"))
    .fetch_all(db)
    .await?;

    Ok(result)
}

pub async fn get_exits(direction: u8, db: &Pool<Sqlite>) -> Result<Vec<String>, sqlx::Error> {
    let result: Vec<String> = sqlx::query(
        "SELECT s.name FROM exits e, stations s WHERE e.name_id == s.id AND e.direction_id = ?",
    )
    .bind(direction)
    .map(|row: SqliteRow| row.get("name"))
    .fetch_all(db)
    .await?;

    Ok(result)
}

pub async fn get_directions(db: &Pool<Sqlite>) -> Result<Vec<DirectionsResponse>, sqlx::Error> {
    let result = sqlx::query_as::<_, DirectionsResponse>("SELECT * FROM directions")
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn get_categories(db: &Pool<Sqlite>) -> Result<Vec<CategoriesResponse>, sqlx::Error> {
    let result = sqlx::query_as::<_, CategoriesResponse>("SELECT * FROM categories")
        .fetch_all(db)
        .await?;

    Ok(result)
}
