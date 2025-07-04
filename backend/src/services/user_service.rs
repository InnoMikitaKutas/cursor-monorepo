use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    database::DbPool,
    models::{Address, Company, CreateUserRequest, Geo, UpdateUserRequest, User},
    schema::{addresses, companies, users},
};

pub async fn get_all_users(pool: &DbPool) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    // Get all users with their addresses and companies
    let users_data: Vec<(
        (Uuid, String, String, String, Option<String>, Option<String>, chrono::DateTime<Utc>, chrono::DateTime<Utc>),
        Option<(Uuid, Uuid, String, Option<String>, String, String, Option<bigdecimal::BigDecimal>, Option<bigdecimal::BigDecimal>)>,
        Option<(Uuid, Uuid, String, Option<String>, Option<String>)>,
    )> = users::table
        .left_join(addresses::table.on(addresses::user_id.eq(users::id)))
        .left_join(companies::table.on(companies::user_id.eq(users::id)))
        .load(&mut conn)
        .await?;

    let mut users_map: std::collections::HashMap<Uuid, User> = std::collections::HashMap::new();

    for (user_data, address_data, company_data) in users_data {
        let user_id = user_data.0;
        
        let address = address_data.map(|addr| Address {
            id: addr.0,
            user_id: addr.1,
            street: addr.2,
            suite: addr.3,
            city: addr.4,
            zipcode: addr.5,
            geo: match (addr.6, addr.7) {
                (Some(lat), Some(lng)) => Some(Geo {
                    lat: lat.to_string().parse().unwrap_or(0.0),
                    lng: lng.to_string().parse().unwrap_or(0.0),
                }),
                _ => None,
            },
        });

        let company = company_data.map(|comp| Company {
            id: comp.0,
            user_id: comp.1,
            name: comp.2,
            catch_phrase: comp.3,
            bs: comp.4,
        });

        users_map.insert(user_id, User {
            id: user_data.0,
            name: user_data.1,
            username: user_data.2,
            email: user_data.3,
            phone: user_data.4,
            website: user_data.5,
            address,
            company,
            created_at: user_data.6,
            updated_at: user_data.7,
        });
    }

    Ok(users_map.into_values().collect())
}

pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    // Get user
    let user_data: (Uuid, String, String, String, Option<String>, Option<String>, chrono::DateTime<Utc>, chrono::DateTime<Utc>) = 
        users::table
            .filter(users::id.eq(user_id))
            .first(&mut conn)
            .await?;

    // Get address
    let address_data: Option<(Uuid, Uuid, String, Option<String>, String, String, Option<bigdecimal::BigDecimal>, Option<bigdecimal::BigDecimal>)> = 
        addresses::table
            .filter(addresses::user_id.eq(user_id))
            .first(&mut conn)
            .await
            .optional()?;

    // Get company
    let company_data: Option<(Uuid, Uuid, String, Option<String>, Option<String>)> = 
        companies::table
            .filter(companies::user_id.eq(user_id))
            .first(&mut conn)
            .await
            .optional()?;

    let address = address_data.map(|addr| Address {
        id: addr.0,
        user_id: addr.1,
        street: addr.2,
        suite: addr.3,
        city: addr.4,
        zipcode: addr.5,
        geo: match (addr.6, addr.7) {
            (Some(lat), Some(lng)) => Some(Geo {
                lat: lat.to_string().parse().unwrap_or(0.0),
                lng: lng.to_string().parse().unwrap_or(0.0),
            }),
            _ => None,
        },
    });

    let company = company_data.map(|comp| Company {
        id: comp.0,
        user_id: comp.1,
        name: comp.2,
        catch_phrase: comp.3,
        bs: comp.4,
    });

    Ok(User {
        id: user_data.0,
        name: user_data.1,
        username: user_data.2,
        email: user_data.3,
        phone: user_data.4,
        website: user_data.5,
        address,
        company,
        created_at: user_data.6,
        updated_at: user_data.7,
    })
}

pub async fn create_user(
    pool: &DbPool,
    user_data: &CreateUserRequest,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    let user_id = Uuid::new_v4();
    let now = Utc::now();

    // Insert user
    diesel::insert_into(users::table)
        .values((
            users::id.eq(user_id),
            users::name.eq(&user_data.name),
            users::username.eq(&user_data.username),
            users::email.eq(&user_data.email),
            users::phone.eq(&user_data.phone),
            users::website.eq(&user_data.website),
            users::created_at.eq(now),
            users::updated_at.eq(now),
        ))
        .execute(&mut conn)
        .await?;

    // Insert address if provided
    if let Some(address_data) = &user_data.address {
        diesel::insert_into(addresses::table)
            .values((
                addresses::id.eq(Uuid::new_v4()),
                addresses::user_id.eq(user_id),
                addresses::street.eq(&address_data.street),
                addresses::suite.eq(&address_data.suite),
                addresses::city.eq(&address_data.city),
                addresses::zipcode.eq(&address_data.zipcode),
                addresses::lat.eq(address_data.geo.as_ref().map(|g| BigDecimal::from_f64(g.lat).unwrap_or_default())),
                addresses::lng.eq(address_data.geo.as_ref().map(|g| BigDecimal::from_f64(g.lng).unwrap_or_default())),
            ))
            .execute(&mut conn)
            .await?;
    }

    // Insert company if provided
    if let Some(company_data) = &user_data.company {
        diesel::insert_into(companies::table)
            .values((
                companies::id.eq(Uuid::new_v4()),
                companies::user_id.eq(user_id),
                companies::name.eq(&company_data.name),
                companies::catch_phrase.eq(&company_data.catch_phrase),
                companies::bs.eq(&company_data.bs),
            ))
            .execute(&mut conn)
            .await?;
    }

    get_user_by_id(pool, user_id).await
}

pub async fn update_user(
    pool: &DbPool,
    user_id: Uuid,
    user_data: &UpdateUserRequest,
) -> Result<User, diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    let now = Utc::now();

    // Update user
    diesel::update(users::table.filter(users::id.eq(user_id)))
        .set((
            users::name.eq(user_data.name.as_ref().unwrap_or(&String::new())),
            users::username.eq(user_data.username.as_ref().unwrap_or(&String::new())),
            users::email.eq(user_data.email.as_ref().unwrap_or(&String::new())),
            users::phone.eq(&user_data.phone),
            users::website.eq(&user_data.website),
            users::updated_at.eq(now),
        ))
        .execute(&mut conn)
        .await?;

    get_user_by_id(pool, user_id).await
}

pub async fn delete_user(pool: &DbPool, user_id: Uuid) -> Result<(), diesel::result::Error> {
    let mut conn = pool.get().await.map_err(|_| diesel::result::Error::BrokenTransactionManager)?;
    
    diesel::delete(users::table.filter(users::id.eq(user_id)))
        .execute(&mut conn)
        .await?;

    Ok(())
} 