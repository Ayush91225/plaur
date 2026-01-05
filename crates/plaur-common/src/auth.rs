use crate::error::{PlaurdError, Result};
use crate::models::user::{JwtClaims, UserRole};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn generate_token(&self, user_id: Uuid, email: &str, role: UserRole) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(24);
        
        let claims = JwtClaims {
            sub: user_id,
            email: email.to_string(),
            role,
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| PlaurdError::Internal {
                message: "Failed to generate token".to_string(),
            })
    }

    pub fn verify_token(&self, token: &str) -> Result<JwtClaims> {
        decode::<JwtClaims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| PlaurdError::Authentication {
                message: "Invalid token".to_string(),
            })
    }

    pub fn hash_password(password: &str) -> Result<String> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|_| PlaurdError::Internal {
                message: "Failed to hash password".to_string(),
            })
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        bcrypt::verify(password, hash)
            .map_err(|_| PlaurdError::Internal {
                message: "Failed to verify password".to_string(),
            })
    }
}