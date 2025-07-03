use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    database::DbPool,
    models::AuthUser,
    schema::auth_users,
};

pub async fn create_user(
    pool: &DbPool,
    user: &AuthUser,
) -> Result<AuthUser, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    diesel::insert_into(auth_users::table)
        .values(user)
        .get_result(&mut conn)
        .await
}

pub async fn get_user_by_email(
    pool: &DbPool,
    email: &str,
) -> Result<AuthUser, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    auth_users::table
        .filter(auth_users::email.eq(email))
        .first(&mut conn)
        .await
}

pub async fn get_user_by_id(
    pool: &DbPool,
    user_id: Uuid,
) -> Result<AuthUser, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    auth_users::table
        .filter(auth_users::id.eq(user_id))
        .first(&mut conn)
        .await
} 