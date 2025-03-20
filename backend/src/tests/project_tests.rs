#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

    use crate::models::project::{ProjectCreate, ProjectStatus, ProjectUpdate};
    use crate::services::project_service::ProjectService;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use chrono::{Duration, Utc};
    use serial_test::serial;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        pool
    }

    #[actix_rt::test]
    #[serial]
    async fn test_crud_operations() {
        let pool = setup_test_db().await;

        // Test Create
        let new_project = ProjectCreate {
            name: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            budget: BigDecimal::from_f64(10000.0).unwrap(),
            client_id: None,
        };

        let created_project = ProjectService::create(new_project, &pool).await.unwrap();
        assert_eq!(created_project.name, "Test Project");

        // Test Read
        let found_project = ProjectService::get_by_id(created_project.id, &pool)
            .await
            .unwrap();
        assert_eq!(found_project.id, created_project.id);

        // Test Update
        let update = ProjectUpdate {
            name: Some("Updated Project".to_string()),
            description: None,
            start_date: None,
            end_date: None,
            status: Some(ProjectStatus::Development),
            budget: None,
            client_id: None,
        };

        let updated_project = ProjectService::update(created_project.id, update, &pool)
            .await
            .unwrap();
        assert_eq!(updated_project.name, "Updated Project");
        assert_eq!(updated_project.status, ProjectStatus::Development);

        // Test Delete
        let delete_result = ProjectService::delete(created_project.id, &pool).await;
        assert!(delete_result.is_ok());

        // Verify deletion
        let find_result = ProjectService::get_by_id(created_project.id, &pool).await;
        assert!(find_result.is_err());
    }

    #[actix_rt::test]
    #[serial]
    async fn test_validation() {
        let pool = setup_test_db().await;

        // Test invalid dates (end before start)
        let invalid_project = ProjectCreate {
            name: "Invalid Project".to_string(),
            description: None,
            start_date: Utc::now() + Duration::days(30),
            end_date: Utc::now(),
            budget: BigDecimal::from_f64(10000.0).unwrap(),
            client_id: None,
        };

        let result = ProjectService::create(invalid_project, &pool).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("end date must be after start date"));
        }

        // Test invalid budget
        let invalid_project = ProjectCreate {
            name: "Invalid Project".to_string(),
            description: None,
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            budget: BigDecimal::from_f64(-1000.0).unwrap(),
            client_id: None,
        };

        let result = ProjectService::create(invalid_project, &pool).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("budget must be non-negative"));
        }

        // Test invalid name (empty)
        let invalid_project = ProjectCreate {
            name: "".to_string(),
            description: None,
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            budget: BigDecimal::from_f64(1000.0).unwrap(),
            client_id: None,
        };

        let result = ProjectService::create(invalid_project, &pool).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("length"));
        }
    }
}
