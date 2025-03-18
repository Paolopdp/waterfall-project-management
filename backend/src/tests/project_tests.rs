#[cfg(test)]
use crate::models::project::{ProjectCreate, ProjectStatus, ProjectUpdate};
use crate::services::project_service::ProjectService;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{Duration, Utc};
use serial_test::serial;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Executor, PgPool,
};
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/waterfall_manager".to_string()
    });

    // Connect to the default 'postgres' database to create the test database
    let admin_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let admin_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(admin_url)
        .await
        .expect("Failed to connect to admin database");

    // Create the test database if it doesn’t exist
    admin_pool
        .execute("CREATE DATABASE waterfall_manager_test;")
        .await
        .unwrap_or_else(|e| {
            if e.to_string().contains("already exists") {
                PgQueryResult::default()
            } else {
                panic!("Failed to create test database: {}", e);
            }
        });

    // Connect to the test database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test database pool");
    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Clear the table
    sqlx::query!("TRUNCATE projects CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate test database");

    pool
}

#[actix_rt::test]
#[serial]
async fn test_create_project() {
    let pool = setup_test_db().await;

    let new_project = ProjectCreate {
        name: "Test Project".to_string(),
        description: Some("Test Description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(30),
        budget: BigDecimal::from_f64(10000.0).unwrap(),
        client_id: None,
    };

    let result = ProjectService::create(new_project, &pool).await;
    assert!(result.is_ok());

    let project = result.unwrap();
    assert_eq!(project.name, "Test Project");
    assert_eq!(project.description, Some("Test Description".to_string()));
    assert_eq!(project.status, ProjectStatus::Planning);
    assert_eq!(project.budget, BigDecimal::from_f64(10000.0).unwrap(),);
}

#[actix_rt::test]
#[serial]
async fn test_get_project() {
    let pool = setup_test_db().await;

    // First create a project
    let new_project = ProjectCreate {
        name: "Get Test Project".to_string(),
        description: Some("Test Description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(30),
        budget: BigDecimal::from_f64(10000.0).unwrap(),
        client_id: None,
    };

    let created = ProjectService::create(new_project, &pool).await.unwrap();

    // Now try to get it
    let result = ProjectService::get_by_id(created.id, &pool).await;
    assert!(result.is_ok());

    let project = result.unwrap();
    assert_eq!(project.id, created.id);
    assert_eq!(project.name, "Get Test Project");
}

#[actix_rt::test]
#[serial]
async fn test_get_all_projects() {
    let pool = setup_test_db().await;

    // Create several projects
    for i in 1..4 {
        let new_project = ProjectCreate {
            name: format!("Project {}", i),
            description: Some(format!("Description {}", i)),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(30),
            budget: BigDecimal::from_f64(10000.0).unwrap() * i,
            client_id: None,
        };

        ProjectService::create(new_project, &pool).await.unwrap();
    }

    // Now get all projects
    let result = ProjectService::get_all(&pool).await;
    assert!(result.is_ok());

    let projects = result.unwrap();
    assert_eq!(projects.len(), 3);

    // Projects should be ordered by created_at DESC
    assert_eq!(projects[0].name, "Project 3");
    assert_eq!(projects[1].name, "Project 2");
    assert_eq!(projects[2].name, "Project 1");
}

#[actix_rt::test]
#[serial]
async fn test_update_project() {
    let pool = setup_test_db().await;

    // First create a project
    let new_project = ProjectCreate {
        name: "Update Test Project".to_string(),
        description: Some("Original Description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(30),
        budget: BigDecimal::from_f64(10000.0).unwrap(),
        client_id: None,
    };

    let created = ProjectService::create(new_project, &pool).await.unwrap();

    // Now update it
    let update = ProjectUpdate {
        name: Some("Updated Project".to_string()),
        description: Some("Updated Description".to_string()),
        start_date: None,
        end_date: None,
        status: Some(ProjectStatus::Development),
        budget: Some(BigDecimal::from_f64(20000.0).unwrap()),
        client_id: None,
    };

    let result = ProjectService::update(created.id, update, &pool).await;
    assert!(result.is_ok());

    let project = result.unwrap();
    assert_eq!(project.name, "Updated Project");
    assert_eq!(project.description, Some("Updated Description".to_string()));
    assert_eq!(project.status, ProjectStatus::Development);
    assert_eq!(project.budget, BigDecimal::from_f64(20000.0).unwrap());
    // Dates should remain unchanged
    assert_eq!(project.start_date, created.start_date);
    assert_eq!(project.end_date, created.end_date);
}

#[actix_rt::test]
#[serial]
async fn test_delete_project() {
    let pool = setup_test_db().await;

    // First create a project
    let new_project = ProjectCreate {
        name: "Delete Test Project".to_string(),
        description: Some("Test Description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(30),
        budget: BigDecimal::from_f64(10000.0).unwrap(),
        client_id: None,
    };

    let created = ProjectService::create(new_project, &pool).await.unwrap();

    // Now delete it
    let result = ProjectService::delete(created.id, &pool).await;
    assert!(result.is_ok());

    // Try to get it - should fail
    let get_result = ProjectService::get_by_id(created.id, &pool).await;
    assert!(get_result.is_err());
}

#[actix_rt::test]
#[serial]
async fn test_non_existent_project() {
    let pool = setup_test_db().await;

    let random_id = Uuid::new_v4();
    let result = ProjectService::get_by_id(random_id, &pool).await;

    assert!(result.is_err());
    match result {
        Err(e) => {
            let error_string = e.to_string();
            assert!(error_string.contains("not found"));
        }
        _ => panic!("Expected error but got success"),
    }
}
