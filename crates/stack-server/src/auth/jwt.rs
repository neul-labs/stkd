//! JWT token management.

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiError, ApiResult};

/// JWT claims.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time
    pub exp: i64,
    /// Issued at
    pub iat: i64,
    /// Session ID
    pub sid: String,
}

impl Claims {
    /// Create new claims.
    pub fn new(user_id: Uuid, session_id: Uuid, expiry_days: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.to_string(),
            exp: (now + Duration::days(expiry_days)).timestamp(),
            iat: now.timestamp(),
            sid: session_id.to_string(),
        }
    }

    /// Get the user ID.
    pub fn user_id(&self) -> ApiResult<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))
    }

    /// Get the session ID.
    pub fn session_id(&self) -> ApiResult<Uuid> {
        Uuid::parse_str(&self.sid)
            .map_err(|_| ApiError::Unauthorized("Invalid session ID in token".to_string()))
    }
}

/// JWT token manager.
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiry_days: i64,
}

impl JwtManager {
    /// Create a new JWT manager.
    pub fn new(secret: &str, expiry_days: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiry_days,
        }
    }

    /// Create a new token.
    pub fn create_token(&self, user_id: Uuid, session_id: Uuid) -> ApiResult<String> {
        let claims = Claims::new(user_id, session_id, self.expiry_days);
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ApiError::Internal(format!("Failed to create token: {}", e)))
    }

    /// Verify and decode a token.
    pub fn verify_token(&self, token: &str) -> ApiResult<Claims> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| ApiError::Unauthorized(format!("Invalid token: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_roundtrip() {
        let manager = JwtManager::new("test-secret", 7);
        let user_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        let token = manager.create_token(user_id, session_id).unwrap();
        let claims = manager.verify_token(&token).unwrap();

        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.session_id().unwrap(), session_id);
    }
}
