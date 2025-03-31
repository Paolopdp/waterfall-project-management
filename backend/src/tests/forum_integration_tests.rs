mod tests {
    use crate::models::auth::LoginCredentials;
    use crate::models::forum::{
        CreateReplyRequest, CreateTagRequest, CreateThreadRequest, ForumTag, Thread,
    };
    use crate::models::user::{UserCreate, UserRole};
    use crate::routes;
    use crate::tests::test_helpers::setup_test_db;
    use actix_web::{test, web, App};
    use serial_test::serial;

    async fn create_authenticated_user(
        app: &impl actix_web::dev::Service<
            actix_http::Request,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        >,
    ) -> String {
        let user = UserCreate {
            email: "forum_integration@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Forum Integration User".to_string(),
            role: UserRole::Admin,
        };

        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&user)
            .to_request();
        let resp = test::call_service(app, register_req).await;
        assert!(resp.status().is_success());

        let body: serde_json::Value = test::read_body_json(resp).await;
        body["token"].as_str().unwrap().to_string()
    }

    #[actix_rt::test]
    #[serial]
    async fn test_forum_endpoints() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        let token = create_authenticated_user(&app).await;

        // Test create thread
        let thread_req = CreateThreadRequest {
            title: "Integration Test Thread".to_string(),
            content: "Integration Test Content".to_string(),
            tag_ids: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/threads")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&thread_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let thread: serde_json::Value = test::read_body_json(resp).await;
        let thread_id = thread["id"].as_str().unwrap();

        // Test get threads
        let req = test::TestRequest::get()
            .uri("/api/forum/threads")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let threads: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(threads.len(), 1);

        // Test create reply
        let reply_req = CreateReplyRequest {
            content: "Integration Test Reply".to_string(),
        };

        let req = test::TestRequest::post()
            .uri(&format!("/api/forum/threads/{}/replies", thread_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&reply_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test get replies
        let req = test::TestRequest::get()
            .uri(&format!("/api/forum/threads/{}/replies", thread_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let replies: Vec<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(replies.len(), 1);
    }

    #[actix_rt::test]
    #[serial]
    async fn test_forum_unauthorized_access() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        // Try to create thread without authentication
        let thread_req = CreateThreadRequest {
            title: "Unauthorized Thread".to_string(),
            content: "Unauthorized Content".to_string(),
            tag_ids: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/threads")
            .set_json(&thread_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        // Try to create reply without authentication
        let reply_req = CreateReplyRequest {
            content: "Unauthorized Reply".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/threads/123/replies")
            .set_json(&reply_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_rt::test]
    #[serial]
    async fn test_tag_endpoints() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        let token = create_authenticated_user(&app).await;

        // Test create tag
        let tag_req = CreateTagRequest {
            name: "Integration Tag".to_string(),
            description: Some("Test tag".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/tags")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&tag_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let created_tag: ForumTag = test::read_body_json(resp).await;
        assert_eq!(created_tag.name, "Integration Tag");

        // Test create thread with tag
        let thread_req = CreateThreadRequest {
            title: "Tagged Thread".to_string(),
            content: "Test Content".to_string(),
            tag_ids: Some(vec![created_tag.id]),
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/threads")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&thread_req)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    #[serial]
    async fn test_search_endpoint() {
        let pool = setup_test_db().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await;

        let token = create_authenticated_user(&app).await;

        // Create test thread
        let thread_req = CreateThreadRequest {
            title: "Searchable Thread".to_string(),
            content: "Unique content for search".to_string(),
            tag_ids: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/forum/threads")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&thread_req)
            .to_request();

        let insert = test::call_service(&app, req).await;
        assert_eq!(insert.status(), 200);

        // Test search
        let req = test::TestRequest::get()
            .uri("/api/forum/threads/search?query=Unique")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("Hidden output");
        // assert_eq!(resp.status(), 200);
        // std::env::set_var("RUST_LOG", "sqlx=debug");
        // env_logger::init_from_env("RUST_LOG");
        let search_results: Vec<Thread> = test::read_body_json(resp).await;
        println!("{:#?}", search_results);
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].title, "Searchable Thread");
    }
}
