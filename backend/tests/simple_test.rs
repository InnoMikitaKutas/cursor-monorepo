#[cfg(test)]
mod tests {
    use cursor_backend::models::{Claims, CreateUserRequest, CreateAddressRequest, CreateGeoRequest};
    use uuid::Uuid;

    #[test]
    fn test_claims_creation() {
        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            email: "test@example.com".to_string(),
            exp: 1234567890,
            iat: 1234567890,
        };

        assert_eq!(claims.email, "test@example.com");
        assert!(claims.sub.len() > 0);
    }

    #[test]
    fn test_create_user_request_validation() {
        let user_request = CreateUserRequest {
            name: "John Doe".to_string(),
            username: "johndoe".to_string(),
            email: "john@example.com".to_string(),
            phone: Some("+1234567890".to_string()),
            website: Some("https://johndoe.com".to_string()),
            address: Some(CreateAddressRequest {
                street: "123 Main St".to_string(),
                suite: Some("Apt 4B".to_string()),
                city: "New York".to_string(),
                zipcode: "10001".to_string(),
                geo: Some(CreateGeoRequest {
                    lat: 40.7128,
                    lng: -74.0060,
                }),
            }),
            company: None,
        };

        assert_eq!(user_request.name, "John Doe");
        assert_eq!(user_request.email, "john@example.com");
        assert!(user_request.address.is_some());
        
        if let Some(address) = &user_request.address {
            assert_eq!(address.city, "New York");
            assert!(address.geo.is_some());
            
            if let Some(geo) = &address.geo {
                assert_eq!(geo.lat, 40.7128);
                assert_eq!(geo.lng, -74.0060);
            }
        }
    }

    #[test]
    fn test_uuid_generation() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.to_string().len(), 36); // UUID string length
    }
} 