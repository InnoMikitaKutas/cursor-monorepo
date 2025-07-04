use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<Address>,
    pub company: Option<Company>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    pub id: Uuid,
    pub user_id: Uuid,
    pub street: String,
    pub suite: Option<String>,
    pub city: String,
    pub zipcode: String,
    pub geo: Option<Geo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geo {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub catch_phrase: Option<String>,
    pub bs: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
#[diesel(table_name = crate::schema::auth_users)]
pub struct AuthUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::auth_users)]
pub struct NewAuthUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

// Request/Response DTOs
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<CreateAddressRequest>,
    pub company: Option<CreateCompanyRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<CreateAddressRequest>,
    pub company: Option<CreateCompanyRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAddressRequest {
    #[validate(length(min = 1, max = 100))]
    pub street: String,
    pub suite: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub city: String,
    #[validate(length(min = 1, max = 20))]
    pub zipcode: String,
    pub geo: Option<CreateGeoRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateGeoRequest {
    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: f64,
    #[validate(range(min = -180.0, max = 180.0))]
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCompanyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub catch_phrase: Option<String>,
    pub bs: Option<String>,
}

// Auth DTOs
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, max = 100))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

// Error responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize,  // expiration time
    pub iat: usize,  // issued at
} 