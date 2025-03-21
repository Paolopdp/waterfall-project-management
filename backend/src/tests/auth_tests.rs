#[cfg(test)]
mod tests {
    use crate::models::auth::LoginCredentials;
    use crate::models::user::{UserCreate, UserRole};
    use crate::services::auth_service::AuthService;
    use crate::tests::test_helpers::{cleanup_test_db, setup_test_db};
    use dotenv::dotenv;
    use serial_test::serial;
    use sqlx::PgPool;
    use std::env;

    #[actix_rt::test]
    #[serial]
    async fn test_registration_and_login() {
        let pool = setup_test_db().await;

        // Test registration
        let user_create = UserCreate {
            email: "test_auth@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Test Auth User".to_string(),
            role: UserRole::Developer,
        };

        let register_result = AuthService::register(user_create, &pool).await;
        assert!(register_result.is_ok());
        let auth_response = register_result.unwrap();
        assert!(!auth_response.token.is_empty());

        // Test login
        let credentials = LoginCredentials {
            email: "test_auth@example.com".to_string(),
            password: "password123".to_string(),
        };

        let login_result = AuthService::login(credentials, &pool).await;
        assert!(login_result.is_ok());
        let login_response = login_result.unwrap();
        assert!(!login_response.token.is_empty());

        // Test invalid password
        let invalid_credentials = LoginCredentials {
            email: "test_auth@example.com".to_string(),
            password: "wrongpassword".to_string(),
        };
        let invalid_login = AuthService::login(invalid_credentials, &pool).await;
        assert!(invalid_login.is_err());

        // Test non-existent user
        let nonexistent_credentials = LoginCredentials {
            email: "nonexistent@example.com".to_string(),
            password: "password123".to_string(),
        };
        let nonexistent_login = AuthService::login(nonexistent_credentials, &pool).await;
        assert!(nonexistent_login.is_err());
    }
}
