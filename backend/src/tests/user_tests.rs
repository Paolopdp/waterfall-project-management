#[cfg(test)]
mod tests {
    use crate::models::user::{UserCreate, UserRole, UserUpdate};
    use crate::services::user_service::UserService;
    use dotenv::dotenv;
    use serial_test::serial;
    use sqlx::PgPool;
    use std::env;

    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        sqlx::PgPool::connect(&database_url).await.unwrap()
    }

    #[actix_rt::test]
    #[serial]
    async fn test_user_crud_operations() {
        let pool = setup_test_db().await;

        // Test Create
        let new_user = UserCreate {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Test User".to_string(),
            role: UserRole::Developer,
        };

        let created_user = UserService::create(new_user, &pool).await.unwrap();
        assert_eq!(created_user.email, "test@example.com");
        assert_eq!(created_user.full_name, "Test User");

        // Test Read
        let found_user = UserService::get_by_id(created_user.id, &pool).await.unwrap();
        assert_eq!(found_user.id, created_user.id);

        // Test Update
        let update = UserUpdate {
            email: Some("updated@example.com".to_string()),
            password: None,
            full_name: Some("Updated User".to_string()),
            role: Some(UserRole::ProjectManager),
        };

        let updated_user = UserService::update(created_user.id, update, &pool)
            .await
            .unwrap();
        assert_eq!(updated_user.email, "updated@example.com");
        assert_eq!(updated_user.full_name, "Updated User");

        // Test Delete
        let delete_result = UserService::delete(created_user.id, &pool).await;
        assert!(delete_result.is_ok());

        // Verify deletion
        let find_result = UserService::get_by_id(created_user.id, &pool).await;
        assert!(find_result.is_err());
    }

    #[actix_rt::test]
    #[serial]
    async fn test_user_validation() {
        let pool = setup_test_db().await;

        // Test invalid email
        let invalid_user = UserCreate {
            email: "invalid_email".to_string(),
            password: "password123".to_string(),
            full_name: "Test User".to_string(),
            role: UserRole::Developer,
        };

        let result = UserService::create(invalid_user, &pool).await;
        assert!(result.is_err());

        // Test password too short
        let invalid_user = UserCreate {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            full_name: "Test User".to_string(),
            role: UserRole::Developer,
        };

        let result = UserService::create(invalid_user, &pool).await;
        assert!(result.is_err());
    }
}