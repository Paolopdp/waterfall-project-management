#[cfg(test)]
mod tests {
    use crate::models::forum::{
        CreateReplyRequest, CreateThreadRequest, ForumTag, ThreadSearchParams,
    };
    use crate::models::user::{UserCreate, UserRole};
    use crate::services::forum_service::ForumService;
    use crate::services::user_service::UserService;
    use crate::tests::test_helpers::{cleanup_test_db, setup_test_db};
    use chrono::{Duration, Utc};
    use serial_test::serial;
    use uuid::Uuid;

    async fn create_test_user(pool: &sqlx::PgPool) -> uuid::Uuid {
        let user = UserCreate {
            email: "forum_test@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Forum Test User".to_string(),
            role: UserRole::Developer,
        };
        let created_user = UserService::create(user, pool).await.unwrap();
        created_user.id
    }

    #[actix_rt::test]
    #[serial]
    async fn test_thread_crud_operations() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Test Create Thread
        let thread_req = CreateThreadRequest {
            title: "Test Thread".to_string(),
            content: "Test Content".to_string(),
            tag_ids: None,
        };

        let created_thread = forum_service
            .create_thread(user_id, thread_req, &pool)
            .await
            .unwrap();
        assert_eq!(created_thread.title, "Test Thread");
        assert_eq!(created_thread.content, "Test Content");
        assert_eq!(created_thread.author_id, user_id);

        // Test Get Threads
        let threads = forum_service.get_threads(&pool).await.unwrap();
        assert_eq!(threads.len(), 1);
        assert_eq!(threads[0].id, created_thread.id);

        // Test Create Reply
        let reply_req = CreateReplyRequest {
            content: "Test Reply".to_string(),
        };

        let created_reply = forum_service
            .create_reply(user_id, created_thread.id, reply_req, &pool)
            .await
            .unwrap();
        assert_eq!(created_reply.content, "Test Reply");
        assert_eq!(created_reply.author_id, user_id);
        assert_eq!(created_reply.thread_id, created_thread.id);

        // Test Get Thread Replies
        let replies = forum_service
            .get_thread_replies(created_thread.id, &pool)
            .await
            .unwrap();
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].id, created_reply.id);

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    #[serial]
    async fn test_thread_validation() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Test empty title
        let invalid_thread = CreateThreadRequest {
            title: "".to_string(),
            content: "Test Content".to_string(),
            tag_ids: None,
        };
        let result = forum_service
            .create_thread(user_id, invalid_thread, &pool)
            .await;
        assert!(result.is_err());

        // Test too long title
        let invalid_thread = CreateThreadRequest {
            title: "a".repeat(101),
            content: "Test Content".to_string(),
            tag_ids: None,
        };
        let result = forum_service
            .create_thread(user_id, invalid_thread, &pool)
            .await;
        assert!(result.is_err());

        // Test empty content
        let invalid_thread = CreateThreadRequest {
            title: "Test Title".to_string(),
            content: "".to_string(),
            tag_ids: None,
        };
        let result = forum_service
            .create_thread(user_id, invalid_thread, &pool)
            .await;
        assert!(result.is_err());

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    #[serial]
    async fn test_reply_validation() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create a thread first
        let thread_req = CreateThreadRequest {
            title: "Test Thread".to_string(),
            content: "Test Content".to_string(),
            tag_ids: None,
        };
        let thread = forum_service
            .create_thread(user_id, thread_req, &pool)
            .await
            .unwrap();

        // Test empty reply
        let invalid_reply = CreateReplyRequest {
            content: "".to_string(),
        };
        let result = forum_service
            .create_reply(user_id, thread.id, invalid_reply, &pool)
            .await;
        assert!(result.is_err());

        // Test reply to non-existent thread
        let reply = CreateReplyRequest {
            content: "Test Reply".to_string(),
        };
        let result = forum_service
            .create_reply(user_id, uuid::Uuid::new_v4(), reply, &pool)
            .await;
        assert!(result.is_err());

        cleanup_test_db(&pool).await;
    }

    #[actix_rt::test]
    #[serial]
    async fn test_tag_operations() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());

        // Test creating tags
        let tag1 = forum_service
            .create_tag("Bug".to_string(), Some("Bug reports".to_string()), &pool)
            .await
            .unwrap();
        let tag2 = forum_service
            .create_tag(
                "Feature".to_string(),
                Some("Feature requests".to_string()),
                &pool,
            )
            .await
            .unwrap();

        assert_eq!(tag1.name, "Bug");
        assert_eq!(tag2.name, "Feature");

        // Test creating thread with tags
        let user_id = create_test_user(&pool).await;
        let thread_req = CreateThreadRequest {
            title: "Test Thread with Tags".to_string(),
            content: "Test Content".to_string(),
            tag_ids: Some(vec![tag1.id, tag2.id]),
        };

        let created_thread = forum_service
            .create_thread(user_id, thread_req, &pool)
            .await
            .unwrap();

        // Test retrieving thread with tags
        let thread_with_tags = forum_service
            .get_thread_with_relations(created_thread.id, &pool)
            .await
            .unwrap();

        assert_eq!(thread_with_tags.tags.unwrap().len(), 2);
    }

    #[actix_rt::test]
    #[serial]
    async fn test_thread_search() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create test tags
        let bug_tag = forum_service
            .create_tag("Bug".to_string(), None, &pool)
            .await
            .unwrap();

        // Create multiple threads
        let threads = vec![
            (
                "Critical Bug",
                "System crashes on startup",
                vec![bug_tag.id],
            ),
            ("Feature Discussion", "New feature ideas", vec![]),
            ("Another Bug", "Minor UI glitch", vec![bug_tag.id]),
        ];

        for (title, content, tags) in threads {
            let thread_req = CreateThreadRequest {
                title: title.to_string(),
                content: content.to_string(),
                tag_ids: Some(tags),
            };
            forum_service
                .create_thread(user_id, thread_req, &pool)
                .await
                .unwrap();
        }

        // Test search by text
        let search_params = ThreadSearchParams {
            query: Some("bug".to_string()),
            tags: None,
            author_id: None,
            from_date: None,
            to_date: None,
            limit: Some(10),
            offset: Some(0),
        };
        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|t| t.title == "Critical Bug"));
        assert!(results.iter().any(|t| t.title == "Another Bug"));

        // Test search by tag
        let search_params = ThreadSearchParams {
            query: None,
            tags: Some(vec![bug_tag.id]),
            author_id: None,
            from_date: None,
            to_date: None,
            limit: Some(10),
            offset: Some(0),
        };

        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);

        // Test search by author
        let search_params = ThreadSearchParams {
            query: None,
            tags: None,
            author_id: Some(user_id),
            from_date: None,
            to_date: None,
            limit: Some(10),
            offset: Some(0),
        };

        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);

        // Test search with date range
        let search_params = ThreadSearchParams {
            query: None,
            tags: None,
            author_id: None,
            from_date: Some(Utc::now() - Duration::hours(1)),
            to_date: Some(Utc::now() + Duration::hours(1)),
            limit: Some(10),
            offset: Some(0),
        };

        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);
    }

    #[actix_rt::test]
    #[serial]
    async fn test_pagination() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());
        let user_id = create_test_user(&pool).await;

        // Create 5 test threads
        for i in 1..=5 {
            let thread_req = CreateThreadRequest {
                title: format!("Thread {}", i),
                content: format!("Content {}", i),
                tag_ids: None,
            };
            forum_service
                .create_thread(user_id, thread_req, &pool)
                .await
                .unwrap();
        }

        // Test first page (limit 2)
        let search_params = ThreadSearchParams {
            query: None,
            tags: None,
            author_id: None,
            from_date: None,
            to_date: None,
            limit: Some(2),
            offset: Some(0),
        };

        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Thread 5"); // Most recent first
        assert_eq!(results[1].title, "Thread 4");

        // Test second page
        let search_params = ThreadSearchParams {
            query: None,
            tags: None,
            author_id: None,
            from_date: None,
            to_date: None,
            limit: Some(2),
            offset: Some(2),
        };

        let results = forum_service
            .search_threads(search_params, &pool)
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].title, "Thread 3");
        assert_eq!(results[1].title, "Thread 2");
    }

    #[actix_rt::test]
    #[serial]
    async fn test_tag_validation() {
        let pool = setup_test_db().await;
        let forum_service = ForumService::new(pool.clone());

        // Test duplicate tag name
        let tag_name = "Test Tag".to_string();
        let _ = forum_service
            .create_tag(tag_name.clone(), None, &pool)
            .await
            .unwrap();

        let result = forum_service.create_tag(tag_name, None, &pool).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("duplicate"));
    }
}
