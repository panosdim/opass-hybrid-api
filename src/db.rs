use sqlx::{sqlite::SqliteRow, Pool, Row, Sqlite};

use crate::model::{
    ApiErrors, CategoriesResponse, DirectionsResponse, ExitsRequest, FrontalStation, TollsCost,
    TollsRequest,
};

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

pub async fn get_exits(request: ExitsRequest, db: &Pool<Sqlite>) -> Result<Vec<String>, ApiErrors> {
    if let Some(enter) = request.enter {
        // Check if Enter is valid
        let row = sqlx::query("SELECT * FROM stations WHERE name = ?")
            .bind(enter)
            .fetch_one(db)
            .await;

        let result = row.map_err(|_| {
            ApiErrors::ValidationError(
                "Enter not found. Check enter name provided in body payload.".to_string(),
            )
        })?;

        let enter_order: i64 = result.get("order");

        let sql_query: String = match request.direction {
            1 => "SELECT s.name FROM exits e, stations s WHERE e.name_id == s.id AND e.direction_id = ? AND s.`order` > ?".to_string(),
            2 => "SELECT s.name FROM exits e, stations s WHERE e.name_id == s.id AND e.direction_id = ? AND s.`order` < ?".to_string(),
            _ => unreachable!(),
        };

        let row = sqlx::query(&sql_query)
            .bind(request.direction)
            .bind(enter_order)
            .map(|row: SqliteRow| row.get("name"))
            .fetch_all(db)
            .await;

        let result: Vec<String> = row.map_err(|e| ApiErrors::SqlError(e.to_string()))?;
        return Ok(result);
    } else {
        let row = sqlx::query(
            "SELECT s.name FROM exits e, stations s WHERE e.name_id == s.id AND e.direction_id = ?",
        )
        .bind(request.direction)
        .map(|row: SqliteRow| row.get("name"))
        .fetch_all(db)
        .await;

        let result: Vec<String> = row.map_err(|e| ApiErrors::SqlError(e.to_string()))?;

        Ok(result)
    }
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

pub async fn calculate_tolls(
    request: TollsRequest,
    db: &Pool<Sqlite>,
) -> Result<Vec<TollsCost>, ApiErrors> {
    // Check if Enter is valid
    let row = sqlx::query("SELECT * FROM stations WHERE name = ?")
        .bind(&request.enter)
        .fetch_one(db)
        .await;

    let result = row.map_err(|_| {
        ApiErrors::ValidationError(
            "Enter not found. Check enter name provided in body payload.".to_string(),
        )
    })?;

    let enter_id: i64 = result.get("id");
    let enter_order: i64 = result.get("order");

    // Check if Exit is valid
    let row = sqlx::query("SELECT * FROM stations WHERE name = ?")
        .bind(&request.exit)
        .fetch_one(db)
        .await;

    let result = row.map_err(|_| {
        ApiErrors::ValidationError(
            "Exit not found. Check exit name provided in body payload.".to_string(),
        )
    })?;

    let exit_id: i64 = result.get("id");
    let exit_order: i64 = result.get("order");

    // Check if Direction is valid
    let row = sqlx::query("SELECT id FROM directions WHERE id = ?")
        .bind(&request.direction)
        .fetch_one(db)
        .await;

    row.map_err(|_| {
        ApiErrors::ValidationError(
            "Direction not found. Check direction value provided in body payload.".to_string(),
        )
    })?;

    // Check if Category is valid
    let row = sqlx::query("SELECT id FROM categories WHERE id = ?")
        .bind(&request.category)
        .fetch_one(db)
        .await;

    row.map_err(|_| {
        ApiErrors::ValidationError(
            "Category not found. Check category value provided in body payload.".to_string(),
        )
    })?;

    // Check if Enter is valid for specific direction
    let row = sqlx::query("SELECT id FROM enters WHERE name_id = ? AND direction_id = ?")
        .bind(&enter_id)
        .bind(&request.direction)
        .fetch_one(db)
        .await;

    row.map_err(|_| {
        ApiErrors::ValidationError("Enter is not valid for specific direction.".to_string())
    })?;

    // Check if Exit is valid for specific direction
    let row = sqlx::query("SELECT id FROM exits WHERE name_id = ? AND direction_id = ?")
        .bind(&exit_id)
        .bind(&request.direction)
        .fetch_one(db)
        .await;

    row.map_err(|_| {
        ApiErrors::ValidationError("Exit is not valid for specific direction.".to_string())
    })?;

    // Check if exit is after enter
    if request.direction == 1 {
        if enter_order > exit_order {
            return Err(ApiErrors::ValidationError(
                "Enter is before exit.".to_string(),
            ));
        }
    } else {
        if enter_order < exit_order {
            return Err(ApiErrors::ValidationError(
                "Enter is before exit.".to_string(),
            ));
        }
    }

    let mut tolls_cost: Vec<TollsCost> = vec![];

    // Add cost for enter
    let row = sqlx::query(
        "SELECT * FROM prices WHERE station_id = ? AND direction_id = ? AND enter_or_exit = ?",
    )
    .bind(&enter_id)
    .bind(&request.direction)
    .bind(0)
    .fetch_one(db)
    .await;

    let result =
        row.map_err(|_| ApiErrors::ValidationError("Price not found. Check DB.".to_string()))?;

    let price: f32 = match request.category {
        1 => result.get("cat_1"),
        2 => result.get("cat_2"),
        3 => result.get("cat_3"),
        4 => result.get("cat_4"),
        _ => unreachable!(),
    };

    if price != 0.0 {
        tolls_cost.push(TollsCost {
            station: request.enter,
            cost: price,
        })
    }

    // Calculate frontal stations between enter and exit
    let frontal_stations = sqlx::query_as::<_, FrontalStation>("SELECT * FROM frontal_stations")
        .fetch_all(db)
        .await
        .unwrap();

    match request.direction {
        1 => {
            for fs in frontal_stations {
                if enter_order <= fs.between_station_1 && exit_order >= fs.between_station_2 {
                    match request.category {
                        1 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_1,
                        }),
                        2 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_2,
                        }),
                        3 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_3,
                        }),
                        4 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_4,
                        }),
                        _ => unreachable!(),
                    };
                }
            }
        }
        2 => {
            for fs in frontal_stations {
                if enter_order >= fs.between_station_1 && exit_order <= fs.between_station_2 {
                    match request.category {
                        1 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_1,
                        }),
                        2 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_2,
                        }),
                        3 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_3,
                        }),
                        4 => tolls_cost.push(TollsCost {
                            station: fs.name,
                            cost: fs.cat_4,
                        }),
                        _ => unreachable!(),
                    };
                }
            }
        }
        _ => unreachable!(),
    }

    // Add cost for exit
    let row = sqlx::query(
        "SELECT * FROM prices WHERE station_id = ? AND direction_id = ? AND enter_or_exit = ?",
    )
    .bind(&exit_id)
    .bind(&request.direction)
    .bind(1)
    .fetch_one(db)
    .await;

    let result =
        row.map_err(|_| ApiErrors::ValidationError("Price not found. Check DB.".to_string()))?;

    let price: f32 = match request.category {
        1 => result.get("cat_1"),
        2 => result.get("cat_2"),
        3 => result.get("cat_3"),
        4 => result.get("cat_4"),
        _ => unreachable!(),
    };

    if price != 0.0 {
        tolls_cost.push(TollsCost {
            station: request.exit,
            cost: price,
        })
    }

    Ok(tolls_cost)
}
