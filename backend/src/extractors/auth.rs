use actix_web::{dev::Payload, error::ErrorUnauthorized, http::header, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;

use crate::models::{auth::Claims, user::UserRole};

pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
    pub role: UserRole,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Extract authorization header
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header.to_str().unwrap_or(""),
            None => return ready(Err(ErrorUnauthorized("No authorization header"))),
        };

        if !auth_header.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Invalid authorization header")));
        }

        let token = &auth_header["Bearer ".len()..];
        let jwt_secret = match env::var("JWT_SECRET") {
            Ok(secret) => secret,
            Err(_) => return ready(Err(ErrorUnauthorized("JWT_SECRET not configured"))),
        };

        // Decode and validate JWT
        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid token"))),
        };

        let claims = token_data.claims;
        let role: UserRole = serde_json::from_str(&claims.role).unwrap_or(UserRole::Developer);

        ready(Ok(AuthenticatedUser {
            user_id: claims.sub,
            email: claims.email,
            role,
        }))
    }
}
