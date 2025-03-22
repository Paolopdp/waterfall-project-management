use crate::errors::ServiceError;
use crate::models::auth::{AuthResponse, Claims, LoginCredentials};
use crate::models::user::{User, UserCreate};
use crate::services::user_service::UserService;
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::env;

pub struct AuthService;

impl AuthService {
    pub async fn register(
        user_create: UserCreate,
        pool: &PgPool,
    ) -> Result<AuthResponse, ServiceError> {
        let user = UserService::create(user_create, pool).await?;
        Self::generate_token(&user)
    }

    pub async fn login(
        credentials: LoginCredentials,
        pool: &PgPool,
    ) -> Result<AuthResponse, ServiceError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, full_name, role as "role: _", created_at, updated_at
            FROM users WHERE email = $1
            "#,
            credentials.email
        )
        .fetch_optional(pool)
        .await?
        .ok_or(ServiceError::InvalidCredentials)?;

        if !verify(credentials.password.as_bytes(), &user.password_hash)? {
            return Err(ServiceError::InvalidCredentials);
        }

        Self::generate_token(&user)
    }

    fn generate_token(user: &User) -> Result<AuthResponse, ServiceError> {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: serde_json::to_string(&user.role)
                .map_err(|_e| ServiceError::InternalServerError)?,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )?;

        Ok(AuthResponse {
            token,
            token_type: "Bearer".to_string(),
            user_id: user.id,
        })
    }
}
