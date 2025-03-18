#[cfg(test)]
use crate::models::project::ProjectCreate;
use crate::models::task::{TaskCreate, TaskStatus, TaskUpdate};
use crate::services::project_service::ProjectService;
use crate::services::task_service::TaskService;
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
    sqlx::query!("TRUNCATE tasks CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate test database");

    pool
}
async fn create_test_project(pool: &PgPool) -> Uuid {
    let project = ProjectCreate {
        name: "Test Project".to_string(),
        description: Some("Test Description".to_string()),
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(30),
        budget: BigDecimal::from_f64(10000.0).unwrap(),
        client_id: None,
    };
    let created_project = ProjectService::create(project, pool).await.unwrap();
    created_project.id
}

#[actix_rt::test]
#[serial]
async fn test_create_task() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;

    let new_task = TaskCreate {
        name: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
        project_id,
        assigned_to: None,
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(7),
        dependencies: vec![],
    };

    let result = TaskService::create(new_task, &pool).await;
    assert!(result.is_ok());

    let task = result.unwrap();
    assert_eq!(task.name, "Test Task");
    assert_eq!(task.description, Some("Test Description".to_string()));
    assert_eq!(task.status, TaskStatus::Pending);
    assert_eq!(task.project_id, project_id);
    assert_eq!(task.progress, BigDecimal::from_i32(0).unwrap());
}

#[actix_rt::test]
#[serial]
async fn test_get_task() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;

    let new_task = TaskCreate {
        name: "Get Test Task".to_string(),
        description: Some("Test Description".to_string()),
        project_id,
        assigned_to: None,
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(7),
        dependencies: vec![],
    };

    let created = TaskService::create(new_task, &pool).await.unwrap();

    let result = TaskService::get_by_id(created.id, &pool).await;
    assert!(result.is_ok());

    let task = result.unwrap();
    assert_eq!(task.id, created.id);
    assert_eq!(task.name, "Get Test Task");
}

#[actix_rt::test]
#[serial]
async fn test_get_all_tasks() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;

    for i in 1..4 {
        let new_task = TaskCreate {
            name: format!("Task {}", i),
            description: Some(format!("Description {}", i)),
            project_id,
            assigned_to: None,
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(7),
            dependencies: vec![],
        };

        TaskService::create(new_task, &pool).await.unwrap();
    }

    let result = TaskService::get_all(&pool).await;
    assert!(result.is_ok());

    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].name, "Task 3");
    assert_eq!(tasks[1].name, "Task 2");
    assert_eq!(tasks[2].name, "Task 1");
}

#[actix_rt::test]
#[serial]
async fn test_update_task() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;

    let new_task = TaskCreate {
        name: "Update Test Task".to_string(),
        description: Some("Original Description".to_string()),
        project_id,
        assigned_to: None,
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(7),
        dependencies: vec![],
    };

    let created = TaskService::create(new_task, &pool).await.unwrap();

    let update = TaskUpdate {
        name: Some("Updated Task".to_string()),
        description: Some("Updated Description".to_string()),
        assigned_to: None,
        status: Some(TaskStatus::InProgress),
        progress: Some(BigDecimal::from_f64(50.0).unwrap()),
        start_date: None,
        end_date: None,
        dependencies: Some(vec![]),
    };

    let result = TaskService::update(created.id, update, &pool).await;
    assert!(result.is_ok());

    let task = result.unwrap();
    assert_eq!(task.name, "Updated Task");
    assert_eq!(task.description, Some("Updated Description".to_string()));
    assert_eq!(task.status, TaskStatus::InProgress);
    assert_eq!(task.progress, BigDecimal::from_f64(50.0).unwrap());
}

#[actix_rt::test]
#[serial]
async fn test_delete_task() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;

    let new_task = TaskCreate {
        name: "Delete Test Task".to_string(),
        description: Some("Test Description".to_string()),
        project_id,
        assigned_to: None,
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(7),
        dependencies: vec![],
    };

    let created = TaskService::create(new_task, &pool).await.unwrap();

    let result = TaskService::delete(created.id, &pool).await;
    assert!(result.is_ok());

    let get_result = TaskService::get_by_id(created.id, &pool).await;
    assert!(get_result.is_err());
}

#[actix_rt::test]
#[serial]
async fn test_get_tasks_by_resource() {
    let pool = setup_test_db().await;
    let project_id = create_test_project(&pool).await;
    let resource_id = Uuid::new_v4();

    // Create tasks assigned to the resource
    for i in 1..4 {
        let new_task = TaskCreate {
            name: format!("Resource Task {}", i),
            description: Some(format!("Description {}", i)),
            project_id,
            assigned_to: Some(resource_id),
            start_date: Utc::now(),
            end_date: Utc::now() + Duration::days(7),
            dependencies: vec![],
        };

        TaskService::create(new_task, &pool).await.unwrap();
    }

    // Create a task not assigned to the resource
    let unassigned_task = TaskCreate {
        name: "Unassigned Task".to_string(),
        description: Some("Description".to_string()),
        project_id,
        assigned_to: None,
        start_date: Utc::now(),
        end_date: Utc::now() + Duration::days(7),
        dependencies: vec![],
    };
    TaskService::create(unassigned_task, &pool).await.unwrap();

    let result = TaskService::get_by_resource(resource_id, &pool).await;
    assert!(result.is_ok());

    let tasks = result.unwrap();
    assert_eq!(tasks.len(), 3);
    for task in tasks {
        assert!(task.assigned_to.contains(&resource_id));
    }
}
