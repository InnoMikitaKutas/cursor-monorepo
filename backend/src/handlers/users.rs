use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{CreateUserRequest, ErrorResponse, UpdateUserRequest, User},
    services::user_service,
};

pub async fn get_users(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<ErrorResponse>)> {
    match user_service::get_all_users(&pool).await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: "Failed to fetch users".to_string(),
            }),
        )),
    }
}

pub async fn get_user(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    match user_service::get_user_by_id(&pool, id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: "User not found".to_string(),
            }),
        )),
    }
}

pub async fn create_user(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: format!("Validation failed: {:?}", errors),
            }),
        ));
    }

    match user_service::create_user(&pool, &payload).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "create_error".to_string(),
                message: "Failed to create user".to_string(),
            }),
        )),
    }
}

pub async fn update_user(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    // Validate input
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: format!("Validation failed: {:?}", errors),
            }),
        ));
    }

    match user_service::update_user(&pool, id, &payload).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: "User not found or update failed".to_string(),
            }),
        )),
    }
}

pub async fn delete_user(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match user_service::delete_user(&pool, id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: "User not found".to_string(),
            }),
        )),
    }
} 