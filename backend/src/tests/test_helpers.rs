use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Executor, PgPool};
pub async fn setup_test_db() -> PgPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL_TEST").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/waterfall_manager_test".to_string()
    });

    // Connect to the default 'postgres' database to create the test database
    let admin_url = "postgres://postgres:postgres@localhost:5432/postgres";
    let admin_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(admin_url)
        .await
        .expect("Failed to connect to admin database");

    // Create the test database if it doesn't exist
    admin_pool
        .execute("DROP DATABASE IF EXISTS waterfall_manager_test;")
        .await
        .expect("Failed to drop test database");

    admin_pool
        .execute("CREATE DATABASE waterfall_manager_test;")
        .await
        .expect("Failed to create test database");

    // Connect to the test database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test database pool");

    // Run migrations
    sqlx::migrate!("./db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

pub async fn cleanup_test_db(pool: &PgPool) {
    // Clean up all tables after tests
    let tables = vec!["tasks", "projects", "resources", "users"];
    for table in tables {
        let query = format!("TRUNCATE TABLE {} CASCADE", table);
        sqlx::query(&query)
            .execute(pool)
            .await
            .expect(&format!("Failed to truncate {}", table));
    }
}
