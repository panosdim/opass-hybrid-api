use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct EntersRequest {
    pub direction: u8,
}

#[derive(Deserialize)]
pub struct ExitsRequest {
    pub direction: u8,
}

#[derive(Serialize, FromRow)]
pub struct DirectionsResponse {
    id: i64,
    name: String,
}

#[derive(Serialize, FromRow)]
pub struct CategoriesResponse {
    id: i64,
    name: String,
}
