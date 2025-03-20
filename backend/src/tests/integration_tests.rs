#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::routes;
    use crate::models::auth::LoginCredentials;
    use crate::models::user::{UserCreate, UserRole};

    async fn setup_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let pool = setup_test_db().await;
        test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes::config),
        )
        .await
    }

    #[actix_rt::test]
    async fn test_protected_endpoints() {
        let app = setup_test_app().await;

        // Try accessing protected endpoint without token
        let req = test::TestRequest::get().uri("/api/projects").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);

        // Register and login to get token
        let user = UserCreate {
            email: "test_integration@example.com".to_string(),
            password: "password123".to_string(),
            full_name: "Test Integration".to_string(),
            role: UserRole::Developer,
        };

        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&user)
            .to_request();
        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status(), 200);

        // Access protected endpoint with token
        let auth_resp: AuthResponse = test::read_body_json(register_resp).await;
        let req = test::TestRequest::get()
            .uri("/api/projects")
            .insert_header(("Authorization", format!("Bearer {}", auth_resp.token)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
