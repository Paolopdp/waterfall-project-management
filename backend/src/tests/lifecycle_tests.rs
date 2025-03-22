#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::ServiceError,
        models::{
            auth::AuthResponse,
            lifecycle::{LifecyclePhase, PhaseTransition},
            project::ProjectCreate,
            user::{UserCreate, UserRole},
        },
        routes,
        services::{
            auth_service::AuthService, lifecycle_service::LifecycleService,
            project_service::ProjectService,
        },
        tests::test_helpers::{cleanup_test_db, setup_test_db},
    };
    use actix_web::{test, web, App};
    use bigdecimal::{BigDecimal, FromPrimitive};
    use chrono::{Duration, Utc};
    use serial_test::serial;
    use sqlx::PgPool;
    use uuid::Uuid;

    // Add this helper function inside the tests module
    async fn create_test_user_and_project(pool: &PgPool) -> (AuthResponse, Uuid) {
        // Create a project manager user
        let pm_user = UserCreate {
            email: "pm@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Project Manager".to_string(),
            role: UserRole::ProjectManager,
        };
        let auth_response = AuthService::register(pm_user, pool).await.unwrap();

        // Create a test project
        let project = ProjectCreate {
            name: "Test Project".to_string(),
            description: Some("Test Description".to_string()),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            budget: BigDecimal::from_f64(10000.0).unwrap(),
            client_id: None,
        };
        let created_project = ProjectService::create(project, pool).await.unwrap();

        (auth_response, created_project.id)
    }

    // Integration tests for controller layer
    #[actix_rt::test]
    #[serial]
    async fn test_phase_transition_authorization() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        // Create a developer user
        let developer = UserCreate {
            email: "dev@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Developer".to_string(),
            role: UserRole::Developer,
        };
        let dev_response = AuthService::register(developer, &pool).await.unwrap();

        // Create test project
        let (_, project_id) = create_test_user_and_project(&pool).await;

        // Try to transition phase as developer
        let transition = PhaseTransition {
            project_id,
            phase: LifecyclePhase::Requirements,
            description: "Unauthorized transition".to_string(),
            attachments: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/lifecycle/transition")
            .insert_header(("Authorization", format!("Bearer {}", dev_response.token)))
            .set_json(&transition)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 403); // Forbidden

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    #[serial]
    async fn test_complete_project_lifecycle() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        // Create PM user and project
        let (auth_response, project_id) = create_test_user_and_project(&pool).await;

        let phases = vec![
            (LifecyclePhase::Requirements, "Requirements gathered"),
            (LifecyclePhase::Design, "Design completed"),
            // ... other phases
        ];

        for (phase, description) in phases {
            let transition = PhaseTransition {
                project_id,
                phase,
                description: description.to_string(),
                attachments: None,
            };

            let req = test::TestRequest::post()
                .uri("/api/lifecycle/transition")
                .insert_header(("Authorization", format!("Bearer {}", auth_response.token)))
                .set_json(&transition)
                .to_request();

            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        cleanup_test_db(&pool).await;
    }
}
