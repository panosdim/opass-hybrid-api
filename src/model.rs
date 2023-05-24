use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct EntersRequest {
    pub direction: u8,
}

#[derive(Deserialize)]
pub struct ExitsRequest {
    pub direction: u8,
    pub enter: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct TollsRequest {
    pub enter: String,
    pub exit: String,
    pub direction: u8,
    pub category: u8,
}

pub enum ApiErrors {
    ValidationError(String),
    SqlError(String),
}

#[derive(Deserialize, FromRow)]
pub struct FrontalStation {
    pub id: i64,
    pub name: String,
    pub cat_1: f32,
    pub cat_2: f32,
    pub cat_3: f32,
    pub cat_4: f32,
    pub between_station_1: i64,
    pub between_station_2: i64,
}

#[derive(Serialize)]
pub struct TollsCost {
    pub station: String,
    pub cost: f32,
}
