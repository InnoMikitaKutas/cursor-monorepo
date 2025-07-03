use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

use crate::models::Claims;

pub async fn auth_middleware(
    State(_pool): State<crate::database::DbPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let token = match auth_header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();

    match decode::<Claims>(token, &key, &validation) {
        Ok(token_data) => {
            // Add user info to request extensions
            request.extensions_mut().insert(token_data.claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
} 