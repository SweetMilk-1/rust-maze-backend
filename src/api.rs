use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::map::{Map, Point};

// API Version
const API_VERSION: &str = "v1";

// In-memory storage for maps
pub type MapStore = Arc<RwLock<HashMap<Uuid, Map>>>;

// API Response types
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Request/Response DTOs
#[derive(Deserialize)]
pub struct CreateMapRequest {
    pub map_string: String,
}

#[derive(Serialize)]
pub struct MapResponse {
    pub id: Uuid,
    pub map_string: String,
}

#[derive(Deserialize)]
pub struct SolveRequest {
    pub start: PointDto,
    pub end: PointDto,
}

#[derive(Serialize)]
pub struct SolveResponse {
    pub solution_map: String,
    pub path_found: bool,
}

#[derive(Deserialize, Serialize)]
pub struct PointDto {
    pub x: usize,
    pub y: usize,
}

impl From<PointDto> for Point {
    fn from(dto: PointDto) -> Self {
        Point { x: dto.x, y: dto.y }
    }
}

// API Routes
pub fn create_api_router(map_store: MapStore) -> Router {
    Router::new()
        .route(&format!("/{}/maps", API_VERSION), post(create_map))
        .route(&format!("/{}/maps", API_VERSION), get(list_maps))
        .route(&format!("/{}/maps/:id", API_VERSION), get(get_map))
        .route(&format!("/{}/maps/:id", API_VERSION), delete(delete_map))
        .route(&format!("/{}/maps/:id/solve", API_VERSION), post(solve_map))
        .with_state(map_store)
}

// Handler functions
async fn create_map(
    State(map_store): State<MapStore>,
    Json(payload): Json<CreateMapRequest>,
) -> Result<Json<ApiResponse<MapResponse>>, (StatusCode, Json<ErrorResponse>)> {
    // Parse map from string
    let map = Map::from_str(&payload.map_string).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid map format".to_string(),
            }),
        )
    })?;

    let id = Uuid::new_v4();

    // Store the map
    map_store.write().unwrap().insert(id, map);

    Ok(Json(ApiResponse {
        data: MapResponse {
            id,
            map_string: payload.map_string,
        },
    }))
}

async fn list_maps(
    State(map_store): State<MapStore>,
) -> Result<Json<ApiResponse<Vec<MapResponse>>>, (StatusCode, Json<ErrorResponse>)> {
    let maps = map_store.read().unwrap();
    let response: Vec<MapResponse> = maps
        .iter()
        .map(|(id, map)| MapResponse {
            id: *id,
            map_string: map.to_string(),
        })
        .collect();

    Ok(Json(ApiResponse { data: response }))
}

async fn get_map(
    Path(id): Path<Uuid>,
    State(map_store): State<MapStore>,
) -> Result<Json<ApiResponse<MapResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let maps = map_store.read().unwrap();
    let map = maps.get(&id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Map not found".to_string(),
            }),
        )
    })?;

    Ok(Json(ApiResponse {
        data: MapResponse {
            id,
            map_string: map.to_string(),
        },
    }))
}

async fn delete_map(
    Path(id): Path<Uuid>,
    State(map_store): State<MapStore>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut maps = map_store.write().unwrap();

    if maps.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Map not found".to_string(),
            }),
        ))
    }
}

async fn solve_map(
    Path(id): Path<Uuid>,
    State(map_store): State<MapStore>,
    Json(payload): Json<SolveRequest>,
) -> Result<Json<ApiResponse<SolveResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mut maps = map_store.write().unwrap();
    let map = maps.get_mut(&id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Map not found".to_string(),
            }),
        )
    })?;

    // Create a clone to work with
    let mut map_clone = map.clone();

    let start: Point = payload.start.into();
    let end: Point = payload.end.into();

    // Check if start and end are not walls
    if !map_clone.validate_coordinates(start) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Start position  is invalid".to_string(),
            }),
        ));
    }

    if !map_clone.validate_coordinates(end)  {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "End position is invalid".to_string(),
            }),
        ));
    }

    // Find and mark the path
    let path_found = map_clone.find_and_mark_path(start, end);

    Ok(Json(ApiResponse {
        data: SolveResponse {
            solution_map: map_clone.to_string(),
            path_found,
        },
    }))
}
