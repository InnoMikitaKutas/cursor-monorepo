use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;
use uuid::Uuid;
use validator::Validate;

use crate::{
    database::DbPool,
    models::{
        AuthResponse, AuthUser, AuthUserResponse, Claims, ErrorResponse, LoginRequest,
        NewAuthUser, RegisterRequest,
    },
    services::auth_service,
};

pub async fn register(
    State(pool): State<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
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

    // Check if user already exists
    match auth_service::get_user_by_email(&pool, &payload.email).await {
        Ok(_) => {
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    error: "user_exists".to_string(),
                    message: "User with this email already exists".to_string(),
                }),
            ));
        }
        Err(_) => {} // User doesn't exist, continue
    }

    // Hash password
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "hash_error".to_string(),
                    message: "Failed to hash password".to_string(),
                }),
            ));
        }
    };

    // Create user
    let user = NewAuthUser {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email.clone(),
        password_hash,
    };

    match auth_service::create_user(&pool, &user).await {
        Ok(created_user) => {
            let token = generate_jwt(&created_user)?;
            Ok(Json(AuthResponse {
                token,
                user: AuthUserResponse {
                    id: created_user.id,
                    name: created_user.name,
                    email: created_user.email,
                    created_at: created_user.created_at,
                },
            }))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "create_error".to_string(),
                message: "Failed to create user".to_string(),
            }),
        )),
    }
}

pub async fn login(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
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

    // Get user by email
    let user = match auth_service::get_user_by_email(&pool, &payload.email).await {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "invalid_credentials".to_string(),
                    message: "Invalid email or password".to_string(),
                }),
            ));
        }
    };

    // Verify password
    match verify(&payload.password, &user.password_hash) {
        Ok(true) => {
            let token = generate_jwt(&user)?;
            Ok(Json(AuthResponse {
                token,
                user: AuthUserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    created_at: user.created_at,
                },
            }))
        }
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "invalid_credentials".to_string(),
                message: "Invalid email or password".to_string(),
            }),
        )),
    }
}

fn generate_jwt(user: &AuthUser) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = Utc::now();
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp,
        iat: now.timestamp() as usize,
    };

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(token) => Ok(token),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "token_error".to_string(),
                message: "Failed to generate token".to_string(),
            }),
        )),
    }
} 